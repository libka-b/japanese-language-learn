use std::collections::HashMap;
use crate::manager::{Manager, Entry, EntryCounter, Stats};
use tauri::AppHandle;

const HIRAGANA_PATH: &str = "resources/hiragana.json";
const HIRAGANA_STATS: &str = "hiragana-stats.json";

const HIRAGANA_PLUS_PATH: &str = "resources/hiragana-plus.json";
const HIRAGANA_PLUS_STATS: &str = "hiragana-plus-stats.json";

const KATAKANA_PATH: &str = "resources/katakana.json";
const KATAKANA_STATS: &str = "katakana-stats.json";

const KATAKANA_PLUS_PATH: &str = "resources/katakana-plus.json";
const KATAKANA_PLUS_STATS: &str = "katakana-plus-stats.json";

pub struct Router {
    manager_map: HashMap<String, Manager>,
}

impl Router {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "hiragana".to_string(),
            Manager::new(HIRAGANA_PATH.to_string(), HIRAGANA_STATS.to_string()),
        );
        map.insert(
            "hiragana-plus".to_string(),
            Manager::new(HIRAGANA_PLUS_PATH.to_string(), HIRAGANA_PLUS_STATS.to_string()),
        );
        map.insert(
            "katakana".to_string(),
            Manager::new(KATAKANA_PATH.to_string(), KATAKANA_STATS.to_string()),
        );
        map.insert(
            "katakana-plus".to_string(),
            Manager::new(KATAKANA_PLUS_PATH.to_string(), KATAKANA_PLUS_STATS.to_string()),
        );

        Self { manager_map: map }
    }

    pub fn get_next(&mut self, handle: AppHandle, name: &str) -> Option<EntryCounter> {
        self.manager_map.get_mut(name).unwrap().get_next(handle)
    }

    pub fn get_stats(&mut self, handle: AppHandle) -> HashMap<String, Stats>  {
        self.manager_map.iter_mut().map(|(name, manager)| {
            (name.clone(), manager.get_stats(handle.clone()))
        }).collect()
    }

    pub fn add_correct(&mut self, handle: AppHandle, name: &str, entry: Entry) {
        self.manager_map.get_mut(name).unwrap().add_correct(handle, entry);
    }

    pub fn add_incorrect(&mut self, handle: AppHandle, name: &str, entry: Entry) {
        self.manager_map.get_mut(name).unwrap().add_incorrect(handle, entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        for manager in self.manager_map.values_mut() {
            manager.save_stats(handle.clone());
        }
    }
}
