use tauri::{AppHandle, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub fn create() -> SystemTray {
  SystemTray::new().with_menu(
    SystemTrayMenu::new()
      .add_item(CustomMenuItem::new("hide", "Hide"))
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(CustomMenuItem::new("quit", "Quit")),
  )
}

pub fn event_handler(app: &AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::LeftClick { .. } => {
      let window = app.get_window("main").unwrap();
      window.hide().unwrap();
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
      let item_handle = app.tray_handle().get_item(&id);
      match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "hide" => {
          let window = app.get_window("main").unwrap();
          if window.is_visible().unwrap() {
            window.hide().unwrap();
            item_handle.set_title("Show").unwrap();
          } else {
            window.show().unwrap();
            item_handle.set_title("Hide").unwrap();
          }
        }
        _ => {}
      }
    }
    _ => {}
  }
}
