use aes_gcm::{
   aead::{Aead, KeyInit, OsRng},
   Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tauri::Emitter;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
   pub desktop: bool,
   pub documents: bool,
   pub pictures: bool,
   pub videos: bool,
   pub music: bool,
   pub downloads: bool,
   pub chrome: bool,
   pub edge: bool,
   pub firefox: bool,
}

#[derive(Serialize, Clone)]
struct ProgressPayload {
   percent: f32,
   eta_ms: u64,
   phase: &'static str,
   processed: u64,
   total: u64,
}

fn emit_progress(
   app: &tauri::AppHandle,
   start: Instant,
   processed: u64,
   total: u64,
   phase: &'static str,
) {
   let percent = if total == 0 {
       0.0
   } else {
       (processed as f32 / total as f32) * 100.0
   };
   let elapsed = start.elapsed().as_millis() as u64;
   let eta_ms = if processed == 0 {
       0
   } else {
       ((elapsed as f64) * ((total as f64 / processed as f64) - 1.0)).max(0.0) as u64
   };
   let _ = app.emit(
       "export-progress",
       ProgressPayload {
           percent,
           eta_ms,
           phase,
           processed,
           total,
       },
   );
}

#[tauri::command]
pub async fn start_export_command(
   app: tauri::AppHandle,
   options: ExportOptions,
   password: String,
   export_path: Option<String>,
   selected_user: String,
) -> Result<String, String> {
   let output_path = match export_path {
       Some(path) => PathBuf::from(path),
       None => {
           let desktop = PathBuf::from(r"C:\\Users")
               .join(&selected_user)
               .join("Desktop");
           desktop.join("NiloShift_Export.nilo")
       }
   };

   let temp_dir = std::env::temp_dir().join("niloshift_export");
   if temp_dir.exists() {
       let _ = fs::remove_dir_all(&temp_dir);
   }
   fs::create_dir_all(&temp_dir)
       .map_err(|e| format!("Fehler beim Erstellen des temporären Ordners: {}", e))?;

   let user_base = PathBuf::from(r"C:\\Users").join(&selected_user);

   // Sofortigen Status senden, damit die UI nicht im Idle bleibt
   let prep_start = Instant::now();
   emit_progress(&app, prep_start, 0, 100, "Vorbereiten");

   // Vorab: Anzahl Dateien zählen, um ETA/Progress zu berechnen (nur ausgewählte Quellen)
   let mut total_files: u64 = 0;
   if options.desktop {
       total_files += count_dir_files(&user_base.join("Desktop"));
   }
   if options.documents {
       total_files += count_dir_files(&user_base.join("Documents"));
   }
   if options.pictures {
       total_files += count_dir_files(&user_base.join("Pictures"));
   }
   if options.downloads {
       total_files += count_dir_files(&user_base.join("Downloads"));
   }
   // Browser (groß) zählen
   if options.chrome {
       total_files += count_dir_files(&user_base.join("AppData/Local/Google/Chrome/User Data"));
   }
   if options.edge {
       total_files += count_dir_files(&user_base.join("AppData/Local/Microsoft/Edge/User Data"));
   }
   if options.firefox {
       total_files += count_dir_files(&user_base.join("AppData/Roaming/Mozilla/Firefox"));
   }

   // Wir zählen Dateien für Copy und Zip -> Faktor 2
   let total_ops = total_files.saturating_mul(2).saturating_add(1); // +1 für Encrypt
   let mut processed: u64 = 0;
   let start = Instant::now();

   // 1) Daten sammeln (Copy)
   emit_progress(&app, start, processed, total_ops, "Sammeln");
   if options.desktop {
       copy_directory_with_progress(
           &user_base.join("Desktop"),
           &temp_dir.join("Desktop"),
           &app,
           start,
           &mut processed,
           total_ops,
       )?;
   }
   if options.documents {
       copy_directory_with_progress(
           &user_base.join("Documents"),
           &temp_dir.join("Documents"),
           &app,
           start,
           &mut processed,
           total_ops,
       )?;
   }
   if options.pictures {
       copy_directory_with_progress(
           &user_base.join("Pictures"),
           &temp_dir.join("Pictures"),
           &app,
           start,
           &mut processed,
           total_ops,
       )?;
   }
   if options.downloads {
       copy_directory_with_progress(
           &user_base.join("Downloads"),
           &temp_dir.join("Downloads"),
           &app,
           start,
           &mut processed,
           total_ops,
       )?;
   }

   let appdata = user_base.join("AppData");
   if options.chrome {
       let p = appdata.join("Local/Google/Chrome/User Data");
       if p.exists() {
           // Bewahre die relative Struktur bei
           copy_directory_with_progress(
               &p,
               &temp_dir.join("AppData/Local/Google/Chrome/User Data"),
               &app,
               start,
               &mut processed,
               total_ops,
           )?;
       }
   }
   if options.edge {
       let p = appdata.join("Local/Microsoft/Edge/User Data");
       if p.exists() {
           copy_directory_with_progress(
               &p,
               &temp_dir.join("AppData/Local/Microsoft/Edge/User Data"),
               &app,
               start,
               &mut processed,
               total_ops,
           )?;
       }
   }
   if options.firefox {
       let p = appdata.join("Roaming/Mozilla/Firefox");
       if p.exists() {
           copy_directory_with_progress(
               &p,
               &temp_dir.join("AppData/Roaming/Mozilla/Firefox"),
               &app,
               start,
               &mut processed,
               total_ops,
           )?;
       }
   }

   // 2) ZIPen
   // ZIP-Datei außerhalb des Quellordners erstellen, damit sie nicht ins Archiv gerät
   let zip_path = std::env::temp_dir().join("niloshift_data.zip");
   emit_progress(&app, start, processed, total_ops, "Packen");
   create_zip_archive_with_progress(&temp_dir, &zip_path, &app, start, &mut processed, total_ops)
       .await?;

   // 3) Verschlüsseln (kleiner Schritt)
   emit_progress(&app, start, processed, total_ops, "Verschlüsseln");
   encrypt_file(&zip_path, &output_path, &password).await?;
   processed = total_ops; // Fertig
   emit_progress(&app, start, processed, total_ops, "Fertig");

   let _ = fs::remove_dir_all(&temp_dir);

   Ok(format!(
       "Export erfolgreich nach {} erstellt",
       output_path.display()
   ))
}

fn is_symlink(path: &Path) -> bool {
   fs::symlink_metadata(path)
       .map(|m| m.file_type().is_symlink())
       .unwrap_or(false)
}

fn count_dir_files(dir: &Path) -> u64 {
   if !dir.exists() || is_symlink(dir) {
       return 0;
   }
   let mut cnt = 0u64;
   let it = match fs::read_dir(dir) {
       Ok(i) => i,
       Err(_) => return 0,
   };
   for e in it {
       if let Ok(e) = e {
           let p = e.path();
           if is_symlink(&p) {
               continue;
           }
           if p.is_dir() {
               cnt += count_dir_files(&p);
           } else {
               cnt += 1;
           }
       }
   }
   cnt
}

fn copy_directory_with_progress(
   src: &Path,
   dst: &Path,
   app: &tauri::AppHandle,
   start: Instant,
   processed: &mut u64,
   total: u64,
) -> Result<(), String> {
   if !src.exists() || is_symlink(src) {
       return Ok(());
   }
   if let Err(e) = fs::create_dir_all(dst) {
       eprintln!("Warnung: {}", e);
   }
   let it = match fs::read_dir(src) {
       Ok(i) => i,
       Err(_) => return Ok(()),
   };
   for e in it {
       let e = match e {
           Ok(x) => x,
           Err(_) => continue,
       };
       let sp = e.path();
       let dp = dst.join(e.file_name());
       if is_symlink(&sp) {
           continue;
       }
       let name_lower = e.file_name().to_string_lossy().to_ascii_lowercase();
       if name_lower == "node_modules" || name_lower.starts_with("cache") || name_lower == "temp" || name_lower == "tmp" {
           continue;
       }
       if sp.is_dir() {
           let _ = copy_directory_with_progress(&sp, &dp, app, start, processed, total);
       } else {
           let _ = fs::copy(&sp, &dp);
           *processed = processed.saturating_add(1);
           emit_progress(app, start, *processed, total, "Sammeln");
       }
   }
   Ok(())
}

async fn create_zip_archive_with_progress(
   source_dir: &Path,
   zip_path: &Path,
   app: &tauri::AppHandle,
   start: Instant,
   processed: &mut u64,
   total: u64,
) -> Result<(), String> {
   let file = fs::File::create(zip_path)
       .map_err(|e| format!("Fehler beim Erstellen der ZIP-Datei: {}", e))?;
   let mut zip = ZipWriter::new(file);
   // Performance: keine Kompression, nur Archivierung (schneller, kleinere CPU-Last)
   let options = FileOptions::default()
       .compression_method(CompressionMethod::Stored)
       .unix_permissions(0o755);
   add_dir_to_zip_progress(
       &mut zip, source_dir, "", &options, app, start, processed, total,
   )?;
   zip.finish()
       .map_err(|e| format!("Fehler beim Abschließen der ZIP-Datei: {}", e))?;
   Ok(())
}

fn add_dir_to_zip_progress<W: Write + io::Seek>(
   zip: &mut ZipWriter<W>,
   src_dir: &Path,
   prefix: &str,
   options: &FileOptions,
   app: &tauri::AppHandle,
   start: Instant,
   processed: &mut u64,
   total: u64,
) -> Result<(), String> {
   let entries = match fs::read_dir(src_dir) {
       Ok(i) => i,
       Err(_) => return Ok(()),
   };
   for entry in entries {
       let entry = match entry {
           Ok(e) => e,
           Err(_) => continue,
       };
       let path = entry.path();
       if is_symlink(&path) {
           continue;
       }
       // Große oder problematische Ordner überspringen, um Hänger zu vermeiden
       let fname = entry.file_name().to_string_lossy().to_ascii_lowercase();
       if fname == "node_modules" || fname.starts_with("cache") || fname == "temp" || fname == "tmp" {
           continue;
       }
       let name = entry.file_name();
       let zip_path = if prefix.is_empty() {
           name.to_string_lossy().to_string()
       } else {
           format!("{}/{}", prefix, name.to_string_lossy())
       };
       if path.is_dir() {
           zip.add_directory(&zip_path, *options)
               .map_err(|e| format!("Fehler Ordner {}: {}", zip_path, e))?;
           add_dir_to_zip_progress(zip, &path, &zip_path, options, app, start, processed, total)?;
       } else {
           zip.start_file(&zip_path, *options)
               .map_err(|e| format!("Fehler Datei {}: {}", zip_path, e))?;
           let content = match fs::read(&path) {
               Ok(c) => c,
               Err(_) => {
                   continue;
               }
           };
           zip.write_all(&content)
               .map_err(|e| format!("Fehler Schreiben {}: {}", zip_path, e))?;
           *processed = processed.saturating_add(1);
           emit_progress(app, start, *processed, total, "Packen");
       }
   }
   Ok(())
}

async fn encrypt_file(zip_path: &Path, output_path: &Path, password: &str) -> Result<(), String> {
   let zip_data =
       fs::read(zip_path).map_err(|e| format!("Fehler beim Lesen der ZIP-Datei: {}", e))?;
   let mut key = [0u8; 32];
   let password_bytes = password.as_bytes();
   for (i, &byte) in password_bytes.iter().enumerate() {
       if i >= 32 {
           break;
       }
       key[i] = byte;
   }
   let mut nonce_bytes = [0u8; 12];
   OsRng.fill_bytes(&mut nonce_bytes);
   let nonce = Nonce::from_slice(&nonce_bytes);
   let cipher = Aes256Gcm::new_from_slice(&key)
       .map_err(|e| format!("Fehler beim Erstellen des Ciphers: {}", e))?;
   let encrypted_data = cipher
       .encrypt(nonce, zip_data.as_ref())
       .map_err(|e| format!("Fehler beim Verschlüsseln: {}", e))?;
   let mut output_data = Vec::new();
   output_data.extend_from_slice(&nonce_bytes);
   output_data.extend_from_slice(&encrypted_data);
   fs::write(output_path, output_data)
       .map_err(|e| format!("Fehler beim Schreiben der verschlüsselten Datei: {}", e))?;
   let _ = fs::remove_file(zip_path);
   Ok(())
}