use tauri::{AppHandle, State};
use crate::AppState;
use crate::manager::{Entry, JsonCompatibleStats};

#[tauri::command]
pub fn next_hiragana_entry(handle: AppHandle, app_state: State<AppState>) -> Option<Entry> {
    app_state.manager.lock().unwrap().get_next(handle)
}

#[tauri::command]
pub fn add_correct(handle: AppHandle, app_state: State<AppState>, entry: Entry) {
    app_state.manager.lock().unwrap().add_correct(handle, entry);
}

#[tauri::command]
pub fn add_incorrect(handle: AppHandle, app_state: State<AppState>, entry: Entry) {
    app_state.manager.lock().unwrap().add_incorrect(handle, entry);
}

#[tauri::command]
pub fn get_stats(handle: AppHandle, app_state: State<AppState>) -> JsonCompatibleStats {
    let stats = app_state.manager.lock().unwrap().get_stats(handle);
    JsonCompatibleStats::from_stats(stats)
}

#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle, app_state: State<AppState>) {
    app_state.manager.lock().unwrap().save_stats(app_handle);

    std::process::exit(0);
}
