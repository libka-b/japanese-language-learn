use crate::manager::{Counter, EntryCounter, Generator, JsonCompatibleStats, Stats};
use csv::Reader;
use std::collections::HashSet;
use tauri::{path::BaseDirectory, AppHandle, Manager as TauriManager};
use serde::{Serialize, de::DeserializeOwned};
use std::hash::Hash;

pub struct Manager<T: DeserializeOwned + Serialize + Clone + PartialEq + Eq + Hash> {
    resource_path: String,
    stats_path: String,
    stats: Option<Stats<T>>,
    entry_generator: Option<Generator<T>>,
}

impl <T: DeserializeOwned + Serialize + Clone + PartialEq + Eq + Hash> Manager<T> {
    pub fn new(resource_path: String, stats_path: String) -> Self {
        Self {
            resource_path,
            stats_path,
            stats: None,
            entry_generator: None,
        }
    }

    pub fn get_next(&mut self, handle: AppHandle) -> Option<EntryCounter<T>> {
        if self.entry_generator.is_none() {
            self.entry_generator = Some(
                self.load_generator(handle)
                    .expect("Unable to load generator."),
            )
        }

        match self.entry_generator.as_mut().unwrap().next() {
            Some(entry) => Some(entry),
            None => {
                self.entry_generator = None;
                None
            }
        }
    }

    pub fn get_stats(&mut self, handle: AppHandle) -> Stats<T> {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_ref().unwrap().clone()
    }

    pub fn add_correct(&mut self, handle: AppHandle, entry: T) {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_mut().unwrap().add_correct(entry);
    }

    pub fn add_incorrect(&mut self, handle: AppHandle, entry: T) {
        if self.stats.is_none() {
            self.stats = Some(self.load_stats(handle));
        }

        self.stats.as_mut().unwrap().add_incorrect(entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        if self.stats.is_some() {
            let json_stats = JsonCompatibleStats::from_stats(self.stats.as_ref().unwrap().clone());
            let _ = json_stats.save_to_file(self.stats_path.to_string(), handle);
        }
    }

    fn load_generator(&mut self, handle: AppHandle) -> Result<Generator<T>, String> {
        let entries = self.load_entries(handle.clone());
        let stats = self.get_stats(handle);
        let wrong: HashSet<T> = stats.wrong.keys().cloned().collect();
        let entries_len = entries.len() as u32;

        let stop_at = compute_stop_at(entries_len, stats);
        let counter = Counter::new(stop_at);
        Generator::new(entries, wrong, counter)
    }

    fn load_stats(&self, handle: AppHandle) -> Stats<T> {
        let stats_path = handle
            .path()
            .resolve(&self.stats_path, BaseDirectory::AppData)
            .unwrap_or_else(|_| panic!("Unable to resolve stats path: `{}`.", self.stats_path));

        let json_stats = JsonCompatibleStats::load_from_file(stats_path, handle)
            .unwrap_or_else(|_| JsonCompatibleStats::from_stats(Stats::new()));

        json_stats.to_stats()
    }

    fn load_entries(&self, handle: AppHandle) -> HashSet<T> {
        let resource_path = handle
            .path()
            .resolve(&self.resource_path, BaseDirectory::Resource)
            .unwrap_or_else(|_| {
                panic!("Unable to resolve resource path `{}`.", self.resource_path)
            });

        let mut reader = Reader::from_path(resource_path).expect("Unable tp read CSV file");

        let records: Vec<T> = reader.deserialize().filter_map(Result::ok).collect();

        records.into_iter().collect()
    }
}

// stop_at is computed as half of the total entries + % of mistakes done * total entries
// that means, the worst case scenario (100% mistakes) it will end up being 1.5 times the total entries
fn compute_stop_at<T: DeserializeOwned + Clone + PartialEq + Eq + Hash>(entries_len: u32, stats: Stats<T>) -> u32 {
    let base = entries_len as f32 / 2.0;
    let extra = match stats.total {
        0 => 0.0,
        _ => (stats.incorrect as f32 / stats.total as f32) * entries_len as f32,
    };
    (base + extra) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::manager::CharacterEntry;

    #[test]
    fn test_compute_stop_at_empty_stats() {
        let stats = Stats::<CharacterEntry> {
            total: 0,
            incorrect: 0,
            wrong: HashMap::new(),
        };
        let entries_len = 100;
        let stop_at = compute_stop_at(entries_len, stats);
        assert_eq!(stop_at, 50);
    }

    #[test]
    fn test_compute_stop_at_some_stats() {
        let stats = Stats::<CharacterEntry> {
            total: 10,
            incorrect: 5,
            wrong: HashMap::new(),
        };
        let entries_len = 100;
        let stop_at = compute_stop_at(entries_len, stats);
        assert_eq!(stop_at, 100);
    }
}
