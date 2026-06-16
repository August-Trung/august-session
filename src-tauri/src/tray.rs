use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "Open August Session");
    let pause = CustomMenuItem::new("pause".to_string(), "Pause Workspace (Ctrl+Shift+P)");
    let quit = CustomMenuItem::new("quit".to_string(), "Exit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(pause)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "show" => {
                    if let Some(window) = app.get_window("main") {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                "pause" => {
                    trigger_pause_overlay(app);
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        SystemTrayEvent::DoubleClick { .. } => {
            if let Some(window) = app.get_window("main") {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
        }
        _ => {}
    }
}

pub fn trigger_pause_overlay(app: &AppHandle) {
    if let Some(window) = app.get_window("pause_overlay") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let _window = tauri::WindowBuilder::new(
            app,
            "pause_overlay",
            tauri::WindowUrl::App("index.html#/pause".into()),
        )
        .title("What should you remember?")
        .inner_size(500.0, 300.0)
        .resizable(false)
        .decorations(false) // Frameless
        .always_on_top(true)
        .center()
        .build()
        .expect("Failed to build pause overlay window");
    }
}
