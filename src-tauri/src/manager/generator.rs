use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::manager::{Counter, Entry};

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCounter {
    entry: Entry,
    counter: Counter,
}

pub struct Generator {
    entries: HashSet<Entry>,
    wrong: HashSet<Entry>,
    returned: HashSet<Entry>,
    counter: Counter,
}

impl Generator {
    pub fn new(
        entries: HashSet<Entry>,
        wrong: HashSet<Entry>,
        counter: Counter,
    ) -> Result<Self, String> {
        if entries.is_empty() {
            Err("Provided empty `entries` collection!".to_string())
        } else {
            Ok(Self {
                entries,
                wrong,
                returned: HashSet::new(),
                counter,
            })
        }
    }

    pub fn next(&mut self) -> Option<EntryCounter> {
        if !self.counter.should_continue() {
            return None;
        }

        let diff = self
            .wrong
            .difference(&self.returned)
            .cloned()
            .collect::<HashSet<_>>();
        if !diff.is_empty() {
            self.counter.incr();
            let next = diff.iter().next()?.clone();
            self.returned.insert(next.clone());
            return Some(EntryCounter {
                entry: next,
                counter: self.counter.clone(),
            });
        }

        let diff = self
            .entries
            .difference(&self.returned)
            .cloned()
            .collect::<HashSet<_>>();
        if !diff.is_empty() {
            self.counter.incr();
            let next = diff.iter().next()?.clone();
            self.returned.insert(next.clone());
            return Some(EntryCounter {
                entry: next,
                counter: self.counter.clone(),
            });
        }

        self.reset();
        self.next()
    }

    fn reset(&mut self) {
        self.returned.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_empty_entries() {
        let generator = Generator::new(HashSet::new(), HashSet::new(), Counter::new(1));
        assert!(generator.is_err());
    }

    #[test]
    fn test_generator_next_with_non_empty_wrong() {
        let wrong_entry: Entry = Entry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };
        let other_entry: Entry = Entry {
            japanese: "b".to_string(),
            english: "b".to_string(),
        };

        let entries = HashSet::from([wrong_entry.clone(), other_entry.clone()]);
        let wrong = HashSet::from([wrong_entry.clone()]);
        let mut generator = Generator::new(entries, wrong, Counter::new(1));
        assert!(generator.is_ok());

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().entry, wrong_entry);

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_none());
    }

    #[test]
    fn test_generator_next_with_empty_wrong() {
        let entry: Entry = Entry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };

        let entries = HashSet::from([entry.clone()]);
        let mut generator = Generator::new(entries, HashSet::new(), Counter::new(1));
        assert!(generator.is_ok());

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().entry, entry);

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_none());
    }

    #[test]
    fn test_generator_next_resets_after_exhausting_entries() {
        let entry: Entry = Entry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };

        let entries = HashSet::from([entry.clone()]);
        let mut generator = Generator::new(entries, HashSet::new(), Counter::new(2));
        assert!(generator.is_ok());

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().entry, entry);

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_some());
        assert_eq!(next.unwrap().entry, entry);

        let next = generator.as_mut().unwrap().next();
        assert!(next.is_none());
    }
}
