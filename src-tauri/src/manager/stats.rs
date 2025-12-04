use crate::manager::Entry;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use tauri::path::BaseDirectory;
use tauri::Manager;

#[derive(Debug, Clone)]
pub struct Stats {
    pub total: u32,
    pub incorrect: u32,
    pub wrong: HashMap<Entry, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCount {
    pub entry: Entry,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCompatibleStats {
    pub total: u32,
    pub incorrect: u32,
    pub wrong: Vec<EntryCount>,
}

impl JsonCompatibleStats {
    pub fn from_stats(stats: Stats) -> Self {
        Self {
            total: stats.total,
            incorrect: stats.incorrect,
            wrong: stats
                .wrong
                .iter()
                .filter(|(_, count)| **count > 0)
                .map(|(entry, count)| EntryCount {
                    entry: entry.clone(),
                    count: *count,
                })
                .collect(),
        }
    }

    pub fn to_stats(&self) -> Stats {
        Stats {
            total: self.total,
            incorrect: self.incorrect,
            wrong: self
                .wrong
                .iter()
                .map(|ec| (ec.entry.clone(), ec.count))
                .collect(),
        }
    }

    pub fn save_to_file(
        &self,
        path: String,
        handle: tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        let path = handle.path().resolve(path, BaseDirectory::AppData)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if fs::write(&path, json).is_err() {
            return Err(format!("Unable to write to file {}.", path.display()).into());
        };
        Ok(())
    }

    pub fn load_from_file(
        path: PathBuf,
        handle: tauri::AppHandle,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = handle.path().resolve(path, BaseDirectory::AppData)?;
        let json = fs::read_to_string(path)?;
        let stats: Self = serde_json::from_str(&json)?;
        Ok(stats)
    }
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total: 0,
            incorrect: 0,
            wrong: HashMap::new(),
        }
    }

    pub fn add_correct(&mut self, entry: Entry) {
        self.wrong.entry(entry).and_modify(|value| {
            if *value > 0 {
                *value -= 1;
            }
        });
        self.total += 1;
    }

    pub fn add_incorrect(&mut self, entry: Entry) {
        self.total += 1;
        self.incorrect += 1;
        *self.wrong.entry(entry).or_insert(0) += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_correct() {
        let mut stats = Stats::new();
        let entry = Entry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };

        stats.add_correct(entry);
        assert_eq!(stats.total, 1);
        assert_eq!(stats.incorrect, 0);
        assert_eq!(stats.wrong.len(), 0);
    }

    #[test]
    fn test_add_incorrect() {
        let mut stats = Stats::new();
        let entry = Entry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };

        stats.add_incorrect(entry.clone());
        assert_eq!(stats.total, 1);
        assert_eq!(stats.incorrect, 1);
        assert_eq!(stats.wrong.len(), 1);
        assert_eq!(stats.wrong.iter().next(), Some((&entry, &1)));
    }
}
