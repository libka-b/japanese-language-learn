mod agent;
mod config;
mod lesson;
mod manager;

use agent::{ApiKey, ApiKeyError};
use config::get_config;
use lesson::{
    add_correct,
    add_incorrect,
    exit_app,
    get_stats,
    next_lesson_entry,
    generate_agentic_lesson,
    validate_translation_lesson,
    set_api_key,
};
use manager::{Config, Router};
use std::sync::{Mutex, RwLock};
use tauri::Manager;

pub struct AppState {
    config: Config,
    manager: Mutex<Router>,
    api_key: RwLock<Option<ApiKey>>,
}

impl AppState {
    pub fn get_api_key(&self, handle: tauri::AppHandle) -> Result<ApiKey, ApiKeyError> {
        {
            let read_guard = self.api_key.read().unwrap();
            if let Some(key) = read_guard.as_ref() {
                return Ok(key.clone());
            }
        }

        let mut write_guard = self.api_key.write().unwrap();

        if let Some(key) = write_guard.as_mut() {
            return Ok(key.clone());
        }

        match ApiKey::load_key(handle) {
            Ok(key) => {
                *write_guard = Some(key.clone());
                Ok(key)
            }
            Err(_) => Err(ApiKeyError::ApiKeyNotSet),
        }
    }

    pub fn set_api_key(&self, key: String) {
        let mut write_guard = self.api_key.write().unwrap();
        *write_guard = Some(ApiKey::new(key));
    }
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
                api_key: RwLock::new(None),
            };
            app.manage(app_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_agentic_lesson,
            validate_translation_lesson,
            exit_app,
            next_lesson_entry,
            add_correct,
            add_incorrect,
            get_stats,
            get_config,
            set_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
