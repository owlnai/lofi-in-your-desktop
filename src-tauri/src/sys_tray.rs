use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem,SystemTray};

pub fn build_tray() -> SystemTray {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

    SystemTray::new().with_menu(tray_menu)
}