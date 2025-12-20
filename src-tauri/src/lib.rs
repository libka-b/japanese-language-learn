mod agent;
mod config;
mod lesson;
mod manager;

use config::get_config;
use lesson::{add_correct, add_incorrect, exit_app, get_stats, next_lesson_entry, generate_agentic_lesson};
use manager::{Config, Router};
use std::sync::Mutex;
use tauri::Manager;

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
            let config_path = app_handle
                .path()
                .resolve(
                    "resources/config.yaml",
                    tauri::path::BaseDirectory::Resource,
                )
                .expect("Unable to resolve config path");

            let content = std::fs::read_to_string(config_path).expect("Unable to read config file");

            let config: Config = serde_yaml::from_str(&content).expect("Unable to parse config.");

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
            generate_agentic_lesson,
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
