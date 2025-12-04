use crate::{manager::Config, AppState};
use tauri::State;

#[tauri::command]
pub fn get_config(app_state: State<AppState>) -> Config {
    app_state.config.clone()
}
