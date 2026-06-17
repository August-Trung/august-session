// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture;
mod db;
mod tray;
mod window_manager;

use std::sync::Mutex;
use tauri::{GlobalShortcutManager, Manager};

struct DbState(Mutex<rusqlite::Connection>);

#[tauri::command]
fn save_moment(
    state: tauri::State<DbState>,
    app_handle: tauri::AppHandle,
    words: String,
    close_everything: bool,
) -> Result<(), String> {
    // Hide overlay and main windows so they are not captured in the screenshot or closed
    if let Some(win) = app_handle.get_window("pause_overlay") {
        let _ = win.hide();
    }
    if let Some(win) = app_handle.get_window("main") {
        let _ = win.hide();
    }
    // Give the OS window manager time to repaint
    std::thread::sleep(std::time::Duration::from_millis(150));

    // 1. Enumerate visible windows in memory (with handles)
    let windows_with_handles = window_manager::enumerate_windows_with_handles();

    // 2. Generate a unique ID and screenshot file paths
    let id = uuid::Uuid::new_v4().to_string();
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;

    let screenshot_filename = format!("{}.webp", id);
    let screenshot_path = app_data_dir
        .join("screenshots")
        .join(&screenshot_filename);

    // 3. Capture screenshot
    capture::capture_screenshot(&screenshot_path)?;

    // 4. Map windows to DB struct (without handles) and serialize to JSON
    let db_windows: Vec<window_manager::WindowInfo> = windows_with_handles
        .iter()
        .map(|w| w.info.clone())
        .collect();

    let windows_json = serde_json::to_string(&db_windows)
        .map_err(|e| format!("Failed to serialize windows layout: {}", e))?;

    // 5. Save record to SQLite
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let created_at = chrono::Utc::now().to_rfc3339();
    db::save_moment(
        &conn,
        &id,
        &words,
        &windows_json,
        &screenshot_filename,
        &created_at,
    )
    .map_err(|e| e.to_string())?;

    // 6. Optionally close captured windows gracefully
    if close_everything {
        for w in windows_with_handles {
            window_manager::close_window(w.hwnd);
        }
    }

    // 7. Dismiss the overlay window
    if let Some(win) = app_handle.get_window("pause_overlay") {
        let _ = win.close();
    }

    // 8. Refresh main window if it is open
    if let Some(win) = app_handle.get_window("main") {
        let _ = win.emit("moment_saved", ());
    }

    Ok(())
}

#[tauri::command]
fn get_moments(state: tauri::State<DbState>) -> Result<Vec<db::MomentRecord>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::get_moments(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_moment(
    state: tauri::State<DbState>,
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    // 1. Delete from SQLite
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::delete_moment(&conn, &id).map_err(|e| e.to_string())?;

    // 2. Delete screenshot file from filesystem
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| "Failed to get app data directory".to_string())?;

    let screenshot_filename = format!("{}.webp", id);
    let screenshot_path = app_data_dir
        .join("screenshots")
        .join(&screenshot_filename);

    if screenshot_path.exists() {
        std::fs::remove_file(screenshot_path)
            .map_err(|e| format!("Failed to delete screenshot file: {}", e))?;
    }

    // 3. Notify UI
    if let Some(win) = app_handle.get_window("main") {
        let _ = win.emit("moment_deleted", ());
    }

    Ok(())
}

#[tauri::command]
fn restore_moment(state: tauri::State<DbState>, id: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let moments = db::get_moments(&conn).map_err(|e| e.to_string())?;

    if let Some(target) = moments.iter().find(|m| m.id == id) {
        let windows: Vec<window_manager::WindowInfo> = serde_json::from_str(&target.windows)
            .map_err(|e| format!("Failed to deserialize windows layout: {}", e))?;

        window_manager::restore_windows(windows);
        Ok(())
    } else {
        Err("Moment not found in database".to_string())
    }
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

            // Setup database
            let conn =
                db::init_db(&app_data_dir).expect("Failed to initialize SQLite database");
            app.manage(DbState(Mutex::new(conn)));

            // Create screenshots directory
            let screenshots_dir = app_data_dir.join("screenshots");
            if !screenshots_dir.exists() {
                std::fs::create_dir_all(&screenshots_dir)
                    .expect("Failed to create screenshots directory");
            }

            // Register global hotkey Ctrl+Shift+P
            let handle_clone = app_handle.clone();
            app.global_shortcut_manager()
                .register("Ctrl+Shift+P", move || {
                    tray::trigger_pause_overlay(&handle_clone);
                })
                .expect("Failed to register global hotkey");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_moment,
            get_moments,
            delete_moment,
            restore_moment
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
