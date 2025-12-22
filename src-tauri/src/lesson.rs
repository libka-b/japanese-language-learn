use crate::agent::{LessonData, Translation, generate_lesson, validate_translation};
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
pub fn generate_agentic_lesson() -> LessonData {
    generate_lesson().unwrap()
}

#[tauri::command]
pub fn validate_translation_lesson(original: String, translation: String) -> Translation {
    validate_translation(original, translation).unwrap()
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
    app_state.manager.lock().unwrap().save_stats(app_handle);

    std::process::exit(0);
}
