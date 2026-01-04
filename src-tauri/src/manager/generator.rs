use serde::de::DeserializeOwned;
use std::{collections::HashSet, hash::Hash};
use crate::manager::{Counter, EntryCounter};

pub struct Generator<T: DeserializeOwned + Clone + PartialEq + Eq + Hash> {
    entries: HashSet<T>,
    wrong: HashSet<T>,
    returned: HashSet<T>,
    counter: Counter,
}

impl <T: DeserializeOwned + Clone + PartialEq + Eq + Hash> Generator<T> {
    pub fn new(
        entries: HashSet<T>,
        wrong: HashSet<T>,
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

    pub fn next(&mut self) -> Option<EntryCounter<T>> {
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
    use crate::manager::CharacterEntry;

    #[test]
    fn test_generator_empty_entries() {
        let generator = Generator::<CharacterEntry>::new(
            HashSet::new(),
            HashSet::new(),
            Counter::new(1)
        );
        assert!(generator.is_err());
    }

    #[test]
    fn test_generator_next_with_non_empty_wrong() {
        let wrong_entry: CharacterEntry = CharacterEntry {
            japanese: "a".to_string(),
            english: "a".to_string(),
        };
        let other_entry: CharacterEntry = CharacterEntry {
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
        let entry: CharacterEntry = CharacterEntry {
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
        let entry: CharacterEntry = CharacterEntry {
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
