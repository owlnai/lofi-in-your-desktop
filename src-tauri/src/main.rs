#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sys_tray::build_tray;
use tauri::{Manager, SystemTrayEvent};
use window_shadows::set_shadow;

mod sys_tray;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      set_shadow(&window, true).expect("Unsupported platform!");
      window.hide().unwrap();
      Ok(())
    })
    .system_tray(build_tray())
    .on_system_tray_event(
    |app, event| match event {
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        app.get_window("main").unwrap().show().unwrap();
      },
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          "quit" => {
            let window = app.get_window("main").unwrap();
            window.close().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}