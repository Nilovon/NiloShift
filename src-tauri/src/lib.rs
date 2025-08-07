mod export_operations;
mod file_operations;
mod import_operations;
mod system_operations;

#[cfg(target_os = "windows")]
fn ensure_elevated() {
    if !is_elevated::is_elevated() {
        let exe = std::env::current_exe().unwrap();
        let _ = runas::Command::new(exe).gui(true).status();
        std::process::exit(0);
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Elevation nur in Release-Builds anfordern (Dev würde sonst beendet werden)
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    ensure_elevated();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_operations::select_import_file,
            file_operations::select_export_path,
            export_operations::start_export_command,
            import_operations::start_import_command,
            import_operations::detect_package_contents,
            system_operations::list_windows_profiles,
            system_operations::detect_browsers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
