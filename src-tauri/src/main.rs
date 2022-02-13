#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod punch;
mod system_tray;

fn main() {
  use tauri::Manager;
  tauri::Builder::default()
    .system_tray(system_tray::create())
    .on_system_tray_event(system_tray::event_handler)
    .setup(|app| {
      // app.listen_global("start-task", |event| {});
      let p = tauri::api::process::current_binary(&app.env()).unwrap();
      println!("App path: {}", p.to_str().unwrap());
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
