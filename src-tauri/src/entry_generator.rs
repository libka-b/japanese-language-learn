use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    japanese: String,
    english: String,
}

pub struct EntryGenerator {
    entries: HashSet<Entry>,
    returned: HashSet<Entry>,
}

impl EntryGenerator {
    pub fn new(entries: HashSet<Entry>) -> Result<Self, String> {
        if entries.is_empty() {
            Err("No entries provided".to_string())
        } else {
            Ok(Self {entries, returned: HashSet::new()})
        }
    }

    pub fn next(&mut self) -> Entry {
        let diff = self.entries.difference(&self.returned).cloned().collect::<HashSet<_>>();
        if diff.is_empty() {
            self.reset();
            return self.next()
        }

        let next = diff.iter().next().unwrap().clone();
        self.returned.insert(next.clone());
        next
    }

    fn reset(&mut self) {
        self.returned.clear();
    }
}
