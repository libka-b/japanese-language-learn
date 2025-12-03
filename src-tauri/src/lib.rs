mod manager;
mod lesson;
mod config;

use std::sync::Mutex;
use lesson::{next_lesson_entry, add_correct, add_incorrect, get_stats, exit_app};
use manager::{Router, Config};
use tauri::Manager;
use config::get_config;

pub struct AppState {
  config: Config,
  manager: Mutex<Router>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .setup(|app| {
      let app_handle = app.handle();
      let config_path = app_handle.path()
        .resolve("resources/config.json", tauri::path::BaseDirectory::Resource)
        .expect("Unable to resolve config path");

      let json = std::fs::read_to_string(config_path)
        .expect("Unable to read config file");

      let config: Config = serde_json::from_str(&json)
        .expect("Unable to parse config.");

      let app_state = AppState {
        config: config.clone(),
        manager: Mutex::new(Router::new(config).unwrap_or_else(|err| {
          panic!("Unable to instantiate router. Error: {}", err);
        })),
      };
      app.manage(app_state);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      exit_app,
      next_lesson_entry,
      add_correct,
      add_incorrect,
      get_stats,
      get_config,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
