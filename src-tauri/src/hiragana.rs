use std::sync::{OnceLock};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::path::BaseDirectory;
use rand::prelude::IndexedRandom;
use tauri::Manager;
use crate::counter::{COUNTER};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    japanese: String,
    english: String,
}

static HIRAGANA: OnceLock<Vec<Entry>> = OnceLock::new();

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

    let hiragana = HIRAGANA.get_or_init(|| {
        let resource_path = handle.path()
            .resolve("resources/hiragana.json", BaseDirectory::Resource)
            .expect("Failed to resolve `resources/hiragana.json`.");

        let json_data = fs::read_to_string(resource_path).expect("Failed to read hiragana.json");
        serde_json::from_str(&json_data).expect("Failed to parse JSON")
    });

    if let Some(c) = counter.as_mut() {
        c.current += 1;
    }

    Some(hiragana.choose(&mut rand::rng()).unwrap().clone())
}
