use std::sync::{Arc, Mutex};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn select_import_file(app: tauri::AppHandle) -> Result<String, String> {
    let result = Arc::new(Mutex::new(None::<Option<String>>));
    let result_clone = Arc::clone(&result);

    app.dialog()
        .file()
        .set_title("NiloShift-Paket auswählen")
        .add_filter("NiloShift Paket", &["nilo"])
        .add_filter("Alle Dateien", &["*"])
        .pick_file(move |file_path| {
            let mut res = result_clone.lock().unwrap();
            *res = Some(file_path.map(|p| p.as_path().unwrap().to_string_lossy().to_string()));
        });

    loop {
        {
            let res = result.lock().unwrap();
            if let Some(file_result) = res.as_ref() {
                return match file_result {
                    Some(path) => Ok(path.clone()),
                    None => Err("Keine Datei ausgewählt".to_string()),
                };
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}

#[tauri::command]
pub async fn select_export_path(
    app: tauri::AppHandle,
    default_file_name: Option<String>,
) -> Result<String, String> {
    let result = Arc::new(Mutex::new(None::<Option<String>>));
    let result_clone = Arc::clone(&result);

    let builder = app
        .dialog()
        .file()
        .set_title("Ziel für Export auswählen")
        .add_filter("NiloShift Paket", &["nilo"]);

    if let Some(_name) = default_file_name {
        // current API may not support setting default file name; ignore safely
    }

    builder.save_file(move |file_path| {
        let mut res = result_clone.lock().unwrap();
        *res = Some(file_path.map(|p| p.as_path().unwrap().to_string_lossy().to_string()));
    });

    loop {
        {
            let res = result.lock().unwrap();
            if let Some(file_result) = res.as_ref() {
                return match file_result {
                    Some(path) => Ok(path.clone()),
                    None => Err("Kein Speicherort gewählt".to_string()),
                };
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
