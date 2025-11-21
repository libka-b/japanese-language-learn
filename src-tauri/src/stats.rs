use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::entry_generator::Entry;


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
pub struct FrontendCompatibleStats {
    pub total: u32,
    pub incorrect: u32,
    pub wrong: Vec<EntryCount>,
}

impl FrontendCompatibleStats {
    pub fn from_stats(stats: Stats) -> Self {
        Self {
            total: stats.total,
            incorrect: stats.incorrect,
            wrong: stats.wrong.iter()
                .map(|(entry, count)| EntryCount { entry: entry.clone(), count: *count })
                .collect(),
        }
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

    pub fn add_correct(&mut self) {
        self.total += 1;
    }

    pub fn add_incorrect(&mut self, entry: Entry) {
        self.total += 1;
        self.incorrect += 1;
        *self.wrong.entry(entry).or_insert(0) += 1
    }
}
