// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::api::shell::open;
use url::Url;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      
      let window = app.get_window("main").unwrap();
      
      let app_handle = app.handle(); 

      window.set_navigation_handler(move |url| {
        let url = Url::parse(&url).unwrap();

        if url.host_str() == Some("webeye.ivao.aero") {
          return true; 
        } else {
          open(&app_handle.shell_scope(), url.as_str(), None).unwrap();
          return false; 
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}