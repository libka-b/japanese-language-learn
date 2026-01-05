use crate::agent::{ApiKeyError, LessonData, Translation, generate_lesson, validate_translation};
use crate::manager::{CharacterEntry, CharacterEntryTable, EntryCounter, JsonCompatibleStats, VocabularyEntry};
use crate::AppState;
use std::collections::HashMap;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn get_character_table(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
) -> CharacterEntryTable {
    app_state.manager.lock().unwrap().get_character_table(handle, lesson_name)
}

#[tauri::command]
pub fn next_character_lesson_entry(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
) -> Option<EntryCounter<CharacterEntry>> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_next_character_entry(handle, lesson_name)
}

#[tauri::command]
pub fn next_vocabulary_lesson_entry(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
) -> Option<EntryCounter<VocabularyEntry>> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_next_vocabulary_entry(handle, lesson_name)
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
pub fn add_correct_character_entry(handle: AppHandle, app_state: State<AppState>, lesson_name: &str, entry: CharacterEntry) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_correct_character_entry(handle, lesson_name, entry);
}

#[tauri::command]
pub fn add_correct_vocabulary_entry(handle: AppHandle, app_state: State<AppState>, lesson_name: &str, entry: VocabularyEntry) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_correct_vocabulary_entry(handle, lesson_name, entry);
}


#[tauri::command]
pub fn add_incorrect_character_entry(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
    entry: CharacterEntry,
) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_incorrect_character_entry(handle, lesson_name, entry);
}

#[tauri::command]
pub fn add_incorrect_vocabulary_entry(
    handle: AppHandle,
    app_state: State<AppState>,
    lesson_name: &str,
    entry: VocabularyEntry,
) {
    app_state
        .manager
        .lock()
        .unwrap()
        .add_incorrect_vocabulary_entry(handle, lesson_name, entry);
}

#[tauri::command]
pub fn get_character_entry_stats(
    handle: AppHandle,
    app_state: State<AppState>,
) -> HashMap<String, JsonCompatibleStats<CharacterEntry>> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_character_entry_stats(handle)
        .into_iter()
        .map(|(name, stats)| (name, JsonCompatibleStats::from_stats(stats)))
        .collect()
}

#[tauri::command]
pub fn get_vocabulary_entry_stats(
    handle: AppHandle,
    app_state: State<AppState>,
) -> HashMap<String, JsonCompatibleStats<VocabularyEntry>> {
    app_state
        .manager
        .lock()
        .unwrap()
        .get_vocabulary_entry_stats(handle)
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
