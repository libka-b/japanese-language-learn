use tauri::State;
use crate::AppState;

#[tauri::command]
pub fn get_config(app_state: State<AppState>) -> Vec<String> {
    app_state.config.lesson_order.clone()
}
