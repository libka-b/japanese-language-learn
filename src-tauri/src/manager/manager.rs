use std::collections::HashSet;
use tauri::{AppHandle, Manager as TauriManager, path::BaseDirectory};
use std::fs;
use crate::manager::{Generator, Stats, Entry, JsonCompatibleStats, Counter, EntryCounter};

const HIRAGANA_PATH: &str = "resources/hiragana.json";
const HIRAGANA_STATS: &str = "stats.json";

pub struct Manager {
    stats: Option<Stats>,
    generator: Option<Generator>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            stats: None,
            generator: None,
        }
    }

    pub fn get_next(&mut self, handle: AppHandle) -> Option<EntryCounter> {
        if self.generator.is_none() {
            self.generator = Some(self.load_generator(handle).expect("Unable to load generator."))
        }

        match self.generator.as_mut().unwrap().next() {
            Some(entry) => Some(entry),
            None => {
                self.generator = None;
                None
            }
        }
    }

    pub fn get_stats(&mut self, handle: AppHandle) -> Stats {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_ref().unwrap().clone()
    }

    pub fn add_correct(&mut self, handle: AppHandle, entry: Entry) {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_mut().unwrap().add_correct(entry);
    }

    pub fn add_incorrect(&mut self, handle: AppHandle, entry: Entry) {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_mut().unwrap().add_incorrect(entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle.clone()));
        }

        let json_stats = JsonCompatibleStats::from_stats(self.stats.as_ref().unwrap().clone());
        let _ = json_stats.save_to_file(HIRAGANA_STATS.to_string(), handle);
    }

    fn load_generator(&mut self, handle: AppHandle) -> Result<Generator, String> {
        let entries = load_entries(handle.clone());
        let stats = self.get_stats(handle);
        let wrong: HashSet<Entry> = stats.wrong.keys().cloned().collect();
        let entries_len = entries.len() as u32;

        let stop_at = compute_stop_at(entries_len, stats);
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

// stop_at is computed as half of the total entries + % of mistakes done * total entries
// that means, the worst case scenario (100% mistakes) it will end up being 1.5 times the total entries
fn compute_stop_at(entries_len: u32, stats: Stats) -> u32 {
    let base = entries_len as f32 / 2.0;
    let extra = match stats.total {
        0 => 0.0,
        _ => (stats.incorrect as f32 / stats.total as f32) * 100.0 * entries_len as f32,
    };
    (base + extra) as u32
}

fn load_entries(handle: AppHandle) -> HashSet<Entry> {
    let resource_path = handle.path()
        .resolve(HIRAGANA_PATH, BaseDirectory::Resource)
        .expect(&format!("Unable to resolve resource path `{}`.", HIRAGANA_PATH));

    let json_data = fs::read_to_string(&resource_path)
        .expect(&format!("Unable to read data from `{}`.", resource_path.display()));

    serde_json::from_str(&json_data).expect(&format!("Unable to parse data from `{}`.", json_data))
}
