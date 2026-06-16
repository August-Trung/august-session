// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod tray;

use tauri::GlobalShortcutManager;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
