use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use serde::Serialize;
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
}

#[tauri::command]
pub async fn detect_package_contents(
    package_path: String,
    password: String,
)
-> Result<DetectedContents, String> {
    let package_path = PathBuf::from(package_path);
    if !package_path.exists() { return Err("Paketdatei nicht gefunden".to_string()); }

    let data = fs::read(&package_path).map_err(|e| format!("Konnte Paketdatei nicht lesen: {}", e))?;
    if data.len() < 12 { return Err("Datei ist beschädigt".to_string()); }
    let (nonce_bytes, cipher_bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let mut key = [0u8; 32];
    for (i, &b) in password.as_bytes().iter().enumerate() { if i >= 32 { break; } key[i] = b; }
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("Cipher-Fehler: {}", e))?;
    let decrypted = cipher
        .decrypt(nonce, cipher_bytes)
        .map_err(|e| format!("Entschlüsselung fehlgeschlagen: {}", e))?;

    // ZIP im Speicher öffnen (ohne auf Disk zu schreiben)
    let reader = std::io::Cursor::new(decrypted);
    let mut archive = ZipArchive::new(reader).map_err(|e| format!("ZIP fehlerhaft: {}", e))?;

    let mut detected = DetectedContents {
        desktop: false,
        documents: false,
        pictures: false,
        downloads: false,
        chrome: false,
        edge: false,
        firefox: false,
    };

    for i in 0..archive.len() {
        let file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name();
        // Benutzerordner
        if name.starts_with("Desktop/") { detected.desktop = true; }
        if name.starts_with("Documents/") || name.starts_with("Dokumente/") { detected.documents = true; }
        if name.starts_with("Pictures/") || name.starts_with("Bilder/") { detected.pictures = true; }
        if name.starts_with("Downloads/") { detected.downloads = true; }
        // Browserpfade
        if name.starts_with("AppData/Local/Google/Chrome/User Data/") { detected.chrome = true; }
        if name.starts_with("AppData/Local/Microsoft/Edge/User Data/") { detected.edge = true; }
        if name.starts_with("AppData/Roaming/Mozilla/Firefox/") { detected.firefox = true; }
        // Frühausstieg, wenn alles gefunden
        if detected.desktop && detected.documents && detected.pictures && detected.downloads && detected.chrome && detected.edge && detected.firefox { break; }
    }
    Ok(detected)
}

#[tauri::command]
pub async fn start_import_command(
    app: tauri::AppHandle,
    package_path: String,
    password: String,
    selected_user: String,
) -> Result<String, String> {
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
    // Zähle nur Top-Level-Einträge für relative Fortschrittsberechnung
    let mut count = 0u64;
    for e in entries { if e.is_ok() { count += 1; } }
    let mut done = 0u64;
    let entries = fs::read_dir(&extract_dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let src = entry.path();
        let dst = user_base.join(entry.file_name());
        copy_directory_merge(&src, &dst)?;
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
