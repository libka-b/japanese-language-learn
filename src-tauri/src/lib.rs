mod manager;
mod hiragana;

use std::sync::Mutex;
use hiragana::{next_hiragana_entry, add_correct, add_incorrect, get_stats, exit_app};
use manager::Manager;

pub struct AppState {
  manager: Mutex<Manager>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .manage(AppState { manager: Mutex::new(Manager::new()) })
    .invoke_handler(tauri::generate_handler![
      exit_app,
      next_hiragana_entry,
      add_correct,
      add_incorrect,
      get_stats,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
