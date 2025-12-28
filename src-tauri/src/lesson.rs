use crate::agent::{ApiKeyError, LessonData, Translation, generate_lesson, validate_translation};
use crate::manager::{Entry, EntryCounter, JsonCompatibleStats};
use crate::AppState;
use std::collections::HashMap;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn next_lesson_entry(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
) -> Option<EntryCounter> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_next(handle, lesson_name)
}

#[tauri::command]
pub fn generate_agentic_lesson(handle: AppHandle, app_state: State<AppState>) -> Result<LessonData, ApiKeyError> {
    let api_key = app_state.get_api_key(handle)?;

    Ok(generate_lesson(api_key).unwrap())
}

#[tauri::command]
pub fn validate_translation_lesson(
    handle: AppHandle,
    app_state: State<AppState>,
    original: String,
    translation: String,
) -> Result<Translation, ApiKeyError> {
    let api_key = app_state.get_api_key(handle)?;

    Ok(validate_translation(original, translation, api_key).unwrap())
}

#[tauri::command]
pub fn set_api_key(app_state: State<AppState>, key: String) {
    app_state.set_api_key(key)
}

#[tauri::command]
pub fn add_correct(handle: AppHandle, app_state: State<AppState>, lesson_name: &str, entry: Entry) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_correct(handle, lesson_name, entry);
}

#[tauri::command]
pub fn add_incorrect(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
    entry: Entry,
) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_incorrect(handle, lesson_name, entry);
}

#[tauri::command]
pub fn get_stats(
    handle: AppHandle,
    app_state: State<AppState>,
) -> HashMap<String, JsonCompatibleStats> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_stats(handle)
        .into_iter()
        .map(|(name, stats)| (name, JsonCompatibleStats::from_stats(stats)))
        .collect()
}

#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle, app_state: State<AppState>) {
    app_state.manager.lock().unwrap().save_stats(app_handle.clone());
    match app_state.api_key.read().unwrap().as_ref() {
        Some(key) => key.save_key(app_handle).unwrap(),
        None => {},
    }

    std::process::exit(0);
}
