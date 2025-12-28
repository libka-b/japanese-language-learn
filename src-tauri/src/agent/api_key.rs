use serde::{Deserialize, Serialize};
use tauri::Manager;
use thiserror::Error;

const PATH: &str = "api-key.conf";

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ApiKeyError {
    #[error("API key is not set")]
    ApiKeyNotSet,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiKey {
    pub key: String,
}

impl ApiKey {
    pub fn new(key: String) -> Self {
        ApiKey { key }
    }

    pub fn save_key(&self, handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        let path = handle.path().resolve(PATH, tauri::path::BaseDirectory::AppData)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if std::fs::write(&path, json).is_err() {
            return Err(format!("Unable to write to file {}.", path.display()).into());
        };
        Ok(())
    }

    pub fn load_key(handle: tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let path = handle.path().resolve(PATH, tauri::path::BaseDirectory::AppData)?;
        let json = std::fs::read_to_string(path)?;
        let api_key: Self = serde_json::from_str(&json)?;
        Ok(api_key)
    }
}
