use tauri::State;
use crate::{AppState, manager::Config};

#[tauri::command]
pub fn get_config(app_state: State<AppState>) -> Config {
    app_state.config.clone()
}
