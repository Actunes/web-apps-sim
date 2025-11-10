#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_opener::init;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin-opener::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}