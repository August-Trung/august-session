// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture;
mod db;
mod tray;
mod window_manager;

use tauri::GlobalShortcutManager;

#[tauri::command]
fn test_screenshot(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;

    let screenshots_dir = app_data_dir.join("screenshots");
    let test_filename = format!("test_{}.webp", chrono::Utc::now().timestamp());
    let test_path = screenshots_dir.join(&test_filename);

    capture::capture_screenshot(&test_path)?;
    Ok(test_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn test_enumerate() -> Result<Vec<window_manager::WindowInfo>, String> {
    Ok(window_manager::enumerate_windows())
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::create_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .setup(|app| {
            let app_handle = app.handle();
            let app_data_dir = app_handle
                .path_resolver()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // M1-T2: SQLite database setup
            db::init_db(&app_data_dir).expect("Failed to initialize SQLite database");

            // M1-T3: Create screenshots directory
            let screenshots_dir = app_data_dir.join("screenshots");
            if !screenshots_dir.exists() {
                std::fs::create_dir_all(&screenshots_dir)
                    .expect("Failed to create screenshots directory");
            }

            // M1-T4: Register global hotkey Ctrl+Shift+P
            let handle_clone = app_handle.clone();
            app.global_shortcut_manager()
                .register("Ctrl+Shift+P", move || {
                    tray::trigger_pause_overlay(&handle_clone);
                })
                .expect("Failed to register global hotkey");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![test_screenshot, test_enumerate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
