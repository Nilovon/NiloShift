use std::fs;
use std::path::{Path, PathBuf};

#[tauri::command]
pub async fn list_windows_profiles() -> Result<Vec<String>, String> {
    #[cfg(not(target_os = "windows"))]
    {
        return Err("Nur auf Windows verfügbar".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let users_dir = PathBuf::from(r"C:\\Users");
        if !users_dir.exists() {
            return Err("C\\Users nicht gefunden".to_string());
        }
        let mut profiles: Vec<String> = Vec::new();
        let skip_names = [
            "All Users",
            "Default",
            "Default User",
            "Public",
            "DefaultAppPool",
            "WDAGUtilityAccount",
            "Administrator",
        ];
        for entry in fs::read_dir(&users_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let meta = entry.metadata().map_err(|e| e.to_string())?;
            if meta.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !skip_names.iter().any(|s| s.eq_ignore_ascii_case(&name)) {
                    profiles.push(name);
                }
            }
        }
        profiles.sort();
        Ok(profiles)
    }
}

#[derive(serde::Serialize)]
pub struct BrowserPresence {
    chrome: bool,
    edge: bool,
    firefox: bool,
}

#[tauri::command]
pub async fn detect_browsers(selected_user: String) -> Result<BrowserPresence, String> {
    #[cfg(not(target_os = "windows"))]
    {
        return Ok(BrowserPresence { chrome: true, edge: false, firefox: true });
    }

    #[cfg(target_os = "windows")]
    {
        let base = PathBuf::from(r"C:\\Users").join(&selected_user);
        let chrome = base.join("AppData/Local/Google/Chrome/User Data");
        let edge = base.join("AppData/Local/Microsoft/Edge/User Data");
        let firefox = base.join("AppData/Roaming/Mozilla/Firefox");
        Ok(BrowserPresence {
            chrome: path_exists_dir(&chrome),
            edge: path_exists_dir(&edge),
            firefox: path_exists_dir(&firefox),
        })
    }
}

fn path_exists_dir(p: &Path) -> bool {
    p.exists() && p.is_dir()
}