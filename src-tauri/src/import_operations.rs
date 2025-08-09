use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::time::Instant;
use tauri::Emitter;
use zip::ZipArchive;

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
        "import-progress",
        ProgressPayload {
            percent,
            eta_ms,
            phase,
            processed,
            total,
        },
    );
}

#[derive(Serialize, Clone)]
pub struct DetectedContents {
    desktop: bool,
    documents: bool,
    pictures: bool,
    downloads: bool,
    chrome: bool,
    edge: bool,
    firefox: bool,
    outlook_signatures: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SelectedItems {
    desktop: bool,
    documents: bool,
    pictures: bool,
    downloads: bool,
    chrome: bool,
    edge: bool,
    firefox: bool,
    outlook_signatures: bool,
}

#[tauri::command]
pub async fn detect_package_contents(
    package_path: String,
    password: String,
) -> Result<DetectedContents, String> {
    println!("=== DETECTION START ===");
    println!("Datei: {}", &package_path);
    println!("Passwort-Länge: {}", password.len());
    
    // Grundlegende Validierung
    let package_path = PathBuf::from(&package_path);
    if !package_path.exists() { 
        println!("❌ Datei existiert nicht");
        return Err("Paketdatei nicht gefunden".to_string()); 
    }

    // Datei lesen
    println!("📁 Lese Datei...");
    let data = match fs::read(&package_path) {
        Ok(data) => {
            println!("✅ Datei gelesen: {} bytes", data.len());
            data
        },
        Err(e) => {
            println!("❌ Fehler beim Lesen: {}", e);
            return Err(format!("Konnte Paketdatei nicht lesen: {}", e));
        }
    };
    
    if data.len() < 12 { 
        println!("❌ Datei zu klein: {} bytes", data.len());
        return Err("Datei ist beschädigt".to_string()); 
    }
    
    // Entschlüsselung
    println!("🔐 Starte Entschlüsselung... (Datei: {:.1} MB)", data.len() as f64 / 1024.0 / 1024.0);
    let (nonce_bytes, cipher_bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let mut key = [0u8; 32];
    for (i, &b) in password.as_bytes().iter().enumerate() { 
        if i >= 32 { break; } 
        key[i] = b; 
    }
    
    let cipher = match Aes256Gcm::new_from_slice(&key) {
        Ok(cipher) => {
            println!("✅ Cipher erstellt");
            cipher
        },
        Err(e) => {
            println!("❌ Cipher-Fehler: {}", e);
            return Err(format!("Cipher-Fehler: {}", e));
        }
    };
    
    // Bei großen Dateien (>100MB) einen schnelleren Test machen
    if cipher_bytes.len() > 100 * 1024 * 1024 {
        println!("⚠️ Große Datei erkannt ({:.1} MB) - verwende schnelle Validierung", cipher_bytes.len() as f64 / 1024.0 / 1024.0);
        
        // Teste nur die ersten 1KB zur Passwort-Validierung
        let test_data = &cipher_bytes[..std::cmp::min(1024, cipher_bytes.len())];
        match cipher.decrypt(nonce, test_data) {
            Ok(_) => {
                println!("✅ Passwort validiert - überspringe vollständige Entschlüsselung für Detection");
                // Für große Dateien: Setze alle Inhalte auf true
                return Ok(DetectedContents {
                    desktop: true,
                    documents: true,
                    pictures: true,
                    downloads: true,
                    chrome: true,
                    edge: true,
                    firefox: true,
                    outlook_signatures: true,
                });
            },
            Err(e) => {
                println!("❌ Passwort-Validierung fehlgeschlagen: {}", e);
                return Err("Entschlüsselung fehlgeschlagen (falsches Passwort?)".to_string());
            }
        }
    }
    
    let decrypted = match cipher.decrypt(nonce, cipher_bytes) {
        Ok(data) => {
            println!("✅ Entschlüsselung erfolgreich: {} bytes", data.len());
            data
        },
        Err(e) => {
            println!("❌ Entschlüsselung fehlgeschlagen: {}", e);
            return Err("Entschlüsselung fehlgeschlagen (falsches Passwort?)".to_string());
        }
    };

    // ZIP verarbeiten
    println!("📦 Öffne ZIP...");
    let reader = std::io::Cursor::new(decrypted);
    let mut archive = match ZipArchive::new(reader) {
        Ok(archive) => {
            println!("✅ ZIP geöffnet: {} Einträge", archive.len());
            archive
        },
        Err(e) => {
            println!("❌ ZIP-Fehler: {}", e);
            return Err(format!("ZIP fehlerhaft: {}", e));
        }
    };

    // Einfache Detection - nur Dateinamen sammeln
    println!("🔍 Sammle Dateinamen...");
    let mut file_names = Vec::new();
    let max_check = std::cmp::min(archive.len(), 50); // Nur erste 50 Einträge
    
    for i in 0..max_check {
        match archive.by_index(i) {
            Ok(file) => {
                let name = file.name().to_string();
                file_names.push(name);
                if i < 10 {
                    println!("  Eintrag {}: {}", i, file.name());
                }
            }
            Err(e) => {
                println!("  Fehler bei Eintrag {}: {}", i, e);
                break; // Bei ersten Fehler aufhören
            }
        }
    }
    
    println!("📋 Gesamt {} Dateinamen gesammelt", file_names.len());

    // Pattern-Matching auf gesammelten Namen
    let mut detected = DetectedContents {
        desktop: false,
        documents: false,
        pictures: false,
        downloads: false,
        chrome: false,
        edge: false,
        firefox: false,
        outlook_signatures: false,
    };

    for name in &file_names {
        if name.contains("Desktop/") { detected.desktop = true; }
        if name.contains("Documents/") || name.contains("Dokumente/") { detected.documents = true; }
        if name.contains("Pictures/") || name.contains("Bilder/") { detected.pictures = true; }
        if name.contains("Downloads/") { detected.downloads = true; }
        if name.contains("Chrome/User Data/") { detected.chrome = true; }
        if name.contains("Edge/User Data/") { detected.edge = true; }
        if name.contains("Mozilla/Firefox/") { detected.firefox = true; }
        if name.contains("Microsoft/Signatures/") { detected.outlook_signatures = true; }
    }
    
    println!("✅ Detection abgeschlossen:");
    println!("  Desktop: {}, Documents: {}, Pictures: {}, Downloads: {}", 
             detected.desktop, detected.documents, detected.pictures, detected.downloads);
    println!("  Chrome: {}, Edge: {}, Firefox: {}, Outlook: {}", 
             detected.chrome, detected.edge, detected.firefox, detected.outlook_signatures);
    println!("=== DETECTION END ===");
    
    Ok(detected)
}

#[tauri::command]
pub async fn start_import_command(
    app: tauri::AppHandle,
    package_path: String,
    password: String,
    selected_user: String,
    selected_items: SelectedItems,
) -> Result<String, String> {
    println!("Import gestartet mit Auswahl: {:?}", selected_items);
    
    let package_path = PathBuf::from(package_path);
    if !package_path.exists() {
        return Err("Paketdatei nicht gefunden".to_string());
    }

    let start = Instant::now();
    emit_progress(&app, start, 0, 100, "Entschlüsseln");

    let data =
        fs::read(&package_path).map_err(|e| format!("Konnte Paketdatei nicht lesen: {}", e))?;
    if data.len() < 12 {
        return Err("Datei ist beschädigt".to_string());
    }
    let (nonce_bytes, cipher_bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let mut key = [0u8; 32];
    for (i, &b) in password.as_bytes().iter().enumerate() {
        if i >= 32 {
            break;
        }
        key[i] = b;
    }
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Cipher-Fehler: {}", e))?;
    let decrypted = cipher
        .decrypt(nonce, cipher_bytes)
        .map_err(|e| format!("Entschlüsselung fehlgeschlagen: {}", e))?;
    emit_progress(&app, start, 10, 100, "Entpacken");

    let temp_dir = std::env::temp_dir().join("niloshift_import");
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    let zip_path = temp_dir.join("data.zip");
    fs::write(&zip_path, &decrypted)
        .map_err(|e| format!("Konnte temporäre ZIP schreiben: {}", e))?;

    let file = fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("ZIP fehlerhaft: {}", e))?;

    let extract_dir = temp_dir.join("extract");
    fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;

    let total = archive.len() as u64;
    let mut processed = 0u64;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        // Stelle sicher, dass AppData-Struktur korrekt wiederhergestellt wird
        let name = sanitize_zip_path(file.name());
        let outpath = extract_dir.join(name);
        if file.name().ends_with('/') {
            let _ = fs::create_dir_all(&outpath);
        } else {
            if let Some(p) = outpath.parent() {
                let _ = fs::create_dir_all(p);
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
        processed += 1;
        emit_progress(&app, start, processed.min(total / 2) + 10, 100, "Entpacken");
    }

    emit_progress(&app, start, 60, 100, "Wiederherstellen");

    let user_base = PathBuf::from(r"C:\\Users").join(&selected_user);
    let entries = match fs::read_dir(&extract_dir) {
        Ok(e) => e,
        Err(e) => return Err(e.to_string()),
    };
    
    // Filtere die Einträge basierend auf der Auswahl
    let mut filtered_entries = Vec::new();
    for entry_res in entries {
        if let Ok(entry) = entry_res {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            let should_import = match name.as_str() {
                "desktop" => selected_items.desktop,
                "documents" => selected_items.documents,
                "pictures" => selected_items.pictures,
                "downloads" => selected_items.downloads,
                name if name.starts_with("appdata") => {
                    // Prüfe, ob Browser-Daten oder Outlook-Signaturen in diesem AppData-Ordner sind
                    if !selected_items.chrome && !selected_items.edge && !selected_items.firefox && !selected_items.outlook_signatures {
                        false
                    } else {
                        // Prüfe spezifische Browser/Outlook-Unterordner
                        let appdata_path = entry.path();
                        should_import_appdata(&appdata_path, &selected_items)
                    }
                }
                _ => false, // Unbekannte Ordner werden ignoriert
            };
            
            if should_import {
                filtered_entries.push(entry);
            }
        }
    }
    
    let count = filtered_entries.len() as u64;
    println!("Gefilterte Einträge: {}", count);
    for entry in &filtered_entries {
        println!("  -> {}", entry.file_name().to_string_lossy());
    }
    
    if count == 0 {
        return Err("Keine Inhalte zum Importieren ausgewählt".to_string());
    }
    
    let mut done = 0u64;
    for entry in filtered_entries {
        let src = entry.path();
        let dst = user_base.join(entry.file_name());
        
        // Spezielle Behandlung für AppData-Ordner
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.starts_with("appdata") {
            // Selektives Kopieren der Browser-Daten
            copy_appdata_selective(&src, &dst, &selected_items)?;
        } else {
            copy_directory_merge(&src, &dst)?;
        }
        
        done += 1;
        emit_progress(
            &app,
            start,
            60 + ((done as f32 / count.max(1) as f32) * 40.0) as u64,
            100,
            "Wiederherstellen",
        );
    }

    let _ = fs::remove_dir_all(&temp_dir);
    emit_progress(&app, start, 100, 100, "Fertig");
    Ok("Import abgeschlossen".to_string())
}

// (keine separate extract_zip-Funktion nötig)

fn sanitize_zip_path(name: &str) -> PathBuf {
    let mut pb = PathBuf::new();
    for part in Path::new(name).components() {
        use std::path::Component::*;
        match part {
            Normal(p) => pb.push(p),
            CurDir | ParentDir | RootDir | Prefix(_) => {}
        }
    }
    pb
}

fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

fn should_import_appdata(appdata_path: &Path, selected_items: &SelectedItems) -> bool {
    // Prüfe, ob dieser AppData-Ordner Browser-Daten oder Outlook-Signaturen enthält, die importiert werden sollen
    if !selected_items.chrome && !selected_items.edge && !selected_items.firefox && !selected_items.outlook_signatures {
        return false;
    }
    
    println!("Prüfe AppData-Ordner: {}", appdata_path.display());
    
    // Prüfe die Unterordner - verschiedene Pfade je nach Export-Struktur
    let check_dirs = vec![
        // Browser-Pfade
        ("Local/Google/Chrome", selected_items.chrome),
        ("Local/Microsoft/Edge", selected_items.edge),
        ("Roaming/Mozilla/Firefox", selected_items.firefox),
        // Outlook-Signaturen Pfade
        ("Roaming/Microsoft/Signatures", selected_items.outlook_signatures),
        // Alternative Pfade falls direkt exportiert
        ("Google/Chrome", selected_items.chrome),
        ("Microsoft/Edge", selected_items.edge),
        ("Mozilla/Firefox", selected_items.firefox),
        ("Microsoft/Signatures", selected_items.outlook_signatures),
    ];
    
    for (app_path, should_import) in check_dirs {
        if should_import {
            let app_full_path = appdata_path.join(app_path);
            println!("  Prüfe Pfad: {} -> Existiert: {}", app_full_path.display(), app_full_path.exists());
            if app_full_path.exists() {
                return true;
            }
        }
    }
    
    // Zusätzliche Prüfung: Schaue direkt nach Microsoft-Ordner wenn Outlook ausgewählt
    if selected_items.outlook_signatures {
        let microsoft_path = appdata_path.join("Microsoft");
        let signatures_path = microsoft_path.join("Signatures");
        println!("  Prüfe direkten Microsoft-Pfad: {} -> Existiert: {}", signatures_path.display(), signatures_path.exists());
        if signatures_path.exists() {
            return true;
        }
        
        // Prüfe auch Roaming-Struktur
        let roaming_signatures = appdata_path.join("Roaming").join("Microsoft").join("Signatures");
        println!("  Prüfe Roaming-Signaturen-Pfad: {} -> Existiert: {}", roaming_signatures.display(), roaming_signatures.exists());
        if roaming_signatures.exists() {
            return true;
        }
    }
    
    false
}

fn copy_appdata_selective(src: &Path, dst: &Path, selected_items: &SelectedItems) -> Result<(), String> {
    // Erstelle das Zielverzeichnis falls es nicht existiert
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    }
    
    // Liste der Browser/App-Pfade und ob sie importiert werden sollen
    let app_configs = vec![
        ("Google", selected_items.chrome),
        ("Microsoft", selected_items.edge || selected_items.outlook_signatures), 
        ("Mozilla", selected_items.firefox),
    ];
    
    // Durchlaufe alle Einträge im AppData-Ordner
    let entries = fs::read_dir(src).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        // Prüfe, ob dieser Ordner importiert werden soll
        let should_copy = app_configs.iter().any(|(app_name, should_import)| {
            *should_import && name.starts_with(app_name)
        }) || (selected_items.outlook_signatures && name == "Microsoft") ||
             (selected_items.outlook_signatures && name == "Roaming" && 
              src.join(&name).join("Microsoft").join("Signatures").exists());
        
        if should_copy {
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            // Spezielle Behandlung für Microsoft/Outlook-Signaturen
            if selected_items.outlook_signatures && (name == "Microsoft" || name == "Roaming") {
                copy_appdata_selective_recursive(&src_path, &dst_path, selected_items)?;
            } else {
                copy_directory_merge(&src_path, &dst_path)?;
            }
        }
    }
    
    Ok(())
}

fn copy_appdata_selective_recursive(src: &Path, dst: &Path, selected_items: &SelectedItems) -> Result<(), String> {
    if !src.exists() {
        return Ok(());
    }
    
    // Erstelle Zielverzeichnis
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    }
    
    // Prüfe ob wir in einem Microsoft-Ordner sind und nur Signatures kopieren sollen
    let src_name = src.file_name().unwrap_or_default().to_string_lossy();
    if src_name == "Microsoft" && selected_items.outlook_signatures {
        // Kopiere nur den Signatures-Ordner
        let signatures_src = src.join("Signatures");
        if signatures_src.exists() {
            let signatures_dst = dst.join("Signatures");
            println!("Kopiere Outlook-Signaturen: {} -> {}", signatures_src.display(), signatures_dst.display());
            copy_directory_merge(&signatures_src, &signatures_dst)?;
        }
        return Ok(());
    }
    
    // Für Roaming-Ordner: Rekursiv nach Microsoft/Signatures suchen
    if src_name == "Roaming" && selected_items.outlook_signatures {
        let microsoft_src = src.join("Microsoft");
        if microsoft_src.exists() {
            let microsoft_dst = dst.join("Microsoft");
            copy_appdata_selective_recursive(&microsoft_src, &microsoft_dst, selected_items)?;
        }
        return Ok(());
    }
    
    // Fallback: Normale Kopie
    copy_directory_merge(src, dst)
}

fn copy_directory_merge(src: &Path, dst: &Path) -> Result<(), String> {
    if is_symlink(src) {
        return Ok(());
    }
    if src.is_dir() {
        let _ = fs::create_dir_all(dst);
        let entries = match fs::read_dir(src) {
            Ok(it) => it,
            Err(_) => return Ok(()),
        };
        for e in entries {
            let e = match e {
                Ok(x) => x,
                Err(_) => continue,
            };
            let s = e.path();
            let d = dst.join(e.file_name());
            copy_directory_merge(&s, &d)?;
        }
    } else {
        if let Some(parent) = dst.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::copy(src, dst);
    }
    Ok(())
}