use std::collections::HashSet;
use std::sync::{OnceLock, Mutex};
use std::fs;
use tauri::path::BaseDirectory;
use tauri::Manager;
use crate::counter::{COUNTER};
use crate::entry_generator::{Entry, EntryGenerator};
use crate::stats::{FrontendCompatibleStats, Stats};

static HIRAGANA_GEN: OnceLock<Mutex<EntryGenerator>> = OnceLock::new();
static STATS: OnceLock<Mutex<Stats>> = OnceLock::new();

#[tauri::command]
pub fn next_hiragana_entry(handle: tauri::AppHandle) -> Option<Entry> {
    let mut counter = COUNTER.lock().unwrap();
    let should_continue = match counter.as_ref() {
        Some(c) => c.current < c.stop_at,
        None => false
    };

    if !should_continue {
        return None;
    }

    let hiragana = HIRAGANA_GEN.get_or_init(|| {
        let resource_path = handle.path()
            .resolve("resources/hiragana.json", BaseDirectory::Resource)
            .expect("Failed to resolve `resources/hiragana.json`.");

        let json_data = fs::read_to_string(resource_path).expect("Failed to read hiragana.json");
        let entries: HashSet<Entry> = serde_json::from_str(&json_data).expect("Failed to parse JSON");
        let generator = EntryGenerator::new(entries).expect("Failed to create EntryGenerator");
        Mutex::new(generator)
    });

    if let Some(c) = counter.as_mut() {
        c.current += 1;
    }

    let entry = hiragana.lock().unwrap().next();

    Some(entry)
}

#[tauri::command]
pub fn add_correct() {
    let stats = STATS.get_or_init(|| {
        Mutex::new(Stats::new())
    });

    stats.lock().unwrap().add_correct();
}

#[tauri::command]
pub fn add_incorrect(entry: Entry) {
    let stats = STATS.get_or_init(|| {
        Mutex::new(Stats::new())
    });

    stats.lock().unwrap().add_incorrect(entry);
}

#[tauri::command]
pub fn get_stats() -> FrontendCompatibleStats {
    let stats = STATS.get_or_init(|| {
        Mutex::new(Stats::new())
    });

    FrontendCompatibleStats::from_stats(stats.lock().unwrap().clone())
}
