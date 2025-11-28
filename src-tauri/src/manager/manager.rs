use std::collections::HashSet;
use tauri::{AppHandle, Manager as TauriManager, path::BaseDirectory};
use std::sync::Mutex;
use std::fs;
use crate::manager::{Generator, Stats, Entry, JsonCompatibleStats, Counter};

const HIRAGANA_PATH: &str = "resources/hiragana.json";
const HIRAGANA_STATS: &str = "stats.json";

pub struct Manager {
    stats: Mutex<Option<Stats>>,
    generator: Mutex<Option<Generator>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            stats: Mutex::new(None),
            generator: Mutex::new(None),
        }
    }

    pub fn get_next(&mut self, handle: AppHandle) -> Option<Entry> {
        let mut generator = self.generator.lock().unwrap();
        if generator.is_none() {
            drop(generator);

            let loaded_generator = self.load_generator(handle)
                .expect("Unable to load generator.");
            generator = self.generator.lock().unwrap();
            *generator = Some(loaded_generator);
        }

        generator.as_mut().unwrap().next()
    }

    pub fn get_stats(&mut self, handle: AppHandle) -> Stats {
        let mut stats = self.stats.lock().unwrap();
        if stats.is_none() {
            let loaded_stats = self.load_stats(handle);
            *stats = Some(loaded_stats);
        }

        stats.as_ref().unwrap().clone()
    }

    pub fn add_correct(&mut self, handle: AppHandle, entry: Entry) {
        let mut stats = self.stats.lock().unwrap();
        if stats.is_none() {
            let loaded_stats = self.load_stats(handle);
            *stats = Some(loaded_stats);
        }

        stats.as_mut().unwrap().add_correct(entry);
    }

    pub fn add_incorrect(&mut self, handle: AppHandle, entry: Entry) {
        let mut stats = self.stats.lock().unwrap();
        if stats.is_none() {
            let loaded_stats = self.load_stats(handle);
            *stats = Some(loaded_stats);
        }

        stats.as_mut().unwrap().add_incorrect(entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        let stats = self.stats.lock().unwrap().clone().unwrap();
        let json_stats = JsonCompatibleStats::from_stats(stats);
        let _ = json_stats.save_to_file(HIRAGANA_STATS.to_string(), handle);
    }

    fn load_generator(&mut self, handle: AppHandle) -> Result<Generator, String> {
        let entries = load_entries(handle.clone());
        let stats = self.get_stats(handle);
        let wrong: HashSet<Entry> = stats.wrong.keys().cloned().collect();
        let entries_len = entries.len();

        // stop_at is computed as half of the total entries + % of mistakes done * total entries
        // that means, the worst case scenario (100% mistakes) it will end up being 1.5 times the total entries
        let stop_at = (entries_len as f64 / 2.0) + (stats.incorrect as f64 / stats.total as f64) * 100.0 * entries_len as f64;
        let counter = Counter::new(stop_at as u32);
        Generator::new(entries, wrong, counter)
    }

    fn load_stats(&self, handle: AppHandle) -> Stats {
        let stats_path = handle.path()
            .resolve(HIRAGANA_STATS, BaseDirectory::AppData)
            .expect(&format!("Unable to resolve stats path: `{}`.", HIRAGANA_STATS));

        let json_stats = JsonCompatibleStats::load_from_file(stats_path, handle)
            .unwrap_or_else(|_| {
                JsonCompatibleStats::from_stats(Stats::new())
            });

        json_stats.to_stats()
    }
}

fn load_entries(handle: AppHandle) -> HashSet<Entry> {
    let resource_path = handle.path()
        .resolve(HIRAGANA_PATH, BaseDirectory::Resource)
        .expect(&format!("Unable to resolve resource path `{}`.", HIRAGANA_PATH));

    let json_data = fs::read_to_string(&resource_path)
        .expect(&format!("Unable to read data from `{}`.", resource_path.display()));

    serde_json::from_str(&json_data).expect(&format!("Unable to parse data from `{}`.", json_data))
}
