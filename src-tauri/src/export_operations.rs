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

fn emit_progress_percent(
    app: &tauri::AppHandle,
    start: Instant,
    percent: f32,
    phase: &'static str,
) {
    let elapsed = start.elapsed().as_millis() as u64;
    let _ = app.emit(
        "export-progress",
        ProgressPayload {
            percent,
            eta_ms: elapsed.saturating_div(2), // grobe Heuristik
            phase,
            processed: 0,
            total: 0,
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

    // 3) Verschlüsseln (kleiner Schritt, aber kann bei großen Dateien dauern)
    encrypt_file(
        &zip_path,
        &output_path,
        &password,
        &app,
        start,
        &mut processed,
        total_ops,
    )
    .await?;
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

#[cfg(target_os = "windows")]
fn is_reparse_point(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    fs::symlink_metadata(path)
        .map(|m| (m.file_attributes() & 0x400) != 0)
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
fn is_reparse_point(_path: &Path) -> bool { false }

fn should_skip_name(lower: &str) -> bool {
    if lower == "node_modules" || lower.starts_with("cache") || lower == "temp" || lower == "tmp" {
        return true;
    }
    // Windows typische Junction-Schleifen in Benutzerprofilen
    let l = lower;
    l == "application data" || l == "local settings" || l == "recent" || l == "sendto" || l == "start menu" || l == "templates" || l == "cookies" || l == "nethood" || l == "printhood"
}

fn count_dir_files(dir: &Path) -> u64 {
    if !dir.exists() || is_symlink(dir) || is_reparse_point(dir) {
        return 0;
    }
    let mut cnt = 0u64;
    let mut stack: Vec<PathBuf> = Vec::new();
    stack.push(dir.to_path_buf());
    while let Some(current) = stack.pop() {
        let it = match fs::read_dir(&current) { Ok(i) => i, Err(_) => continue };
        for e in it {
            let e = match e { Ok(x) => x, Err(_) => continue };
            let p = e.path();
            let name_lower = e.file_name().to_string_lossy().to_ascii_lowercase();
            if is_symlink(&p) || is_reparse_point(&p) || should_skip_name(&name_lower) { continue; }
            if p.is_dir() {
                stack.push(p);
            } else {
                cnt = cnt.saturating_add(1);
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
    let mut stack: Vec<(PathBuf, PathBuf)> = Vec::new();
    stack.push((src.to_path_buf(), dst.to_path_buf()));
    while let Some((current_src, current_dst)) = stack.pop() {
        if is_symlink(&current_src) {
            continue;
        }
        if let Err(e) = fs::create_dir_all(&current_dst) {
            eprintln!("Warnung: {}", e);
        }
        let it = match fs::read_dir(&current_src) { Ok(i) => i, Err(_) => continue };
        for entry in it {
            let entry = match entry { Ok(x) => x, Err(_) => continue };
            let sp = entry.path();
            let dp = current_dst.join(entry.file_name());
            if is_symlink(&sp) || is_reparse_point(&sp) { continue; }
            let name_lower = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if should_skip_name(&name_lower) {
                continue;
            }
            if sp.is_dir() {
                stack.push((sp, dp));
            } else {
                let _ = fs::copy(&sp, &dp);
                *processed = processed.saturating_add(1);
                emit_progress(app, start, *processed, total, "Sammeln");
            }
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
    let mut stack: Vec<(PathBuf, String)> = Vec::new();
    stack.push((src_dir.to_path_buf(), prefix.to_string()));
    while let Some((current_dir, current_prefix)) = stack.pop() {
        let entries = match fs::read_dir(&current_dir) { Ok(i) => i, Err(_) => continue };
        for entry in entries {
            let entry = match entry { Ok(e) => e, Err(_) => continue };
            let path = entry.path();
            if is_symlink(&path) || is_reparse_point(&path) { continue; }
            let fname = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if should_skip_name(&fname) { continue; }
            let name = entry.file_name();
            let zip_path = if current_prefix.is_empty() {
                name.to_string_lossy().to_string()
            } else {
                format!("{}/{}", current_prefix, name.to_string_lossy())
            };
            if path.is_dir() {
                zip.add_directory(&zip_path, *options)
                    .map_err(|e| format!("Fehler Ordner {}: {}", zip_path, e))?;
                stack.push((path, zip_path));
            } else {
                zip.start_file(&zip_path, *options)
                    .map_err(|e| format!("Fehler Datei {}: {}", zip_path, e))?;
                let mut infile = match fs::File::open(&path) { Ok(f) => f, Err(_) => continue };
                let mut buffer = [0u8; 1024 * 1024];
                loop {
                    match std::io::Read::read(&mut infile, &mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            zip.write_all(&buffer[..n])
                                .map_err(|e| format!("Fehler Schreiben {}: {}", zip_path, e))?;
                        }
                        Err(_) => break,
                    }
                }
                *processed = processed.saturating_add(1);
                emit_progress(app, start, *processed, total, "Packen");
            }
        }
    }
    Ok(())
}

async fn encrypt_file(
    zip_path: &Path,
    output_path: &Path,
    password: &str,
    app: &tauri::AppHandle,
    start: Instant,
    processed: &mut u64,
    total_ops: u64,
) -> Result<(), String> {
    // Datei größenbasiert einlesen und Fortschritt melden (max. 99%)
    let meta = fs::metadata(zip_path)
        .map_err(|e| format!("Fehler beim Lesen der ZIP-Metadaten: {}", e))?;
    let total_size = meta.len();
    let mut file = fs::File::open(zip_path)
        .map_err(|e| format!("Fehler beim Öffnen der ZIP-Datei: {}", e))?;
    let mut zip_data: Vec<u8> = Vec::with_capacity(total_size as usize);
    let mut buffer = [0u8; 1024 * 1024];
    let mut read_bytes: u64 = 0;
    let base_percent = if total_ops == 0 { 0.0 } else { (*processed as f32 / total_ops as f32) * 100.0 };
    loop {
        match std::io::Read::read(&mut file, &mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                zip_data.extend_from_slice(&buffer[..n]);
                read_bytes += n as u64;
                let frac = if total_size == 0 { 1.0 } else { read_bytes as f32 / total_size as f32 };
                let p = base_percent + (99.0 - base_percent).max(0.0) * frac;
                emit_progress_percent(app, start, p.min(99.0), "Verschlüsseln");
            }
            Err(e) => return Err(format!("Fehler beim Lesen der ZIP-Datei: {}", e)),
        }
    }
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
