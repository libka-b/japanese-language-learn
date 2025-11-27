use std::collections::HashSet;
use crate::manager::{Entry, Counter};

pub struct Generator {
    entries: HashSet<Entry>,
    wrong: HashSet<Entry>,
    returned: HashSet<Entry>,
    counter: Counter,
}

impl Generator {
    pub fn new(entries: HashSet<Entry>, wrong: HashSet<Entry>, counter: Counter) -> Result<Self, String> {
        if entries.is_empty() {
            Err("Provided empty `entries` collection!".to_string())
        } else {
            Ok(Self { entries, wrong, returned: HashSet::new(), counter })
        }
    }

    pub fn next(&mut self) -> Option<Entry> {
        if !self.counter.should_continue() {
            return None;
        }

        let diff = self.wrong.difference(&self.returned).cloned().collect::<HashSet<_>>();
        if !diff.is_empty() {
            self.counter.incr();
            self.returned.insert(diff.iter().next()?.clone());
            return Some(diff.iter().next()?.clone())
        }

        let diff = self.entries.difference(&self.returned).cloned().collect::<HashSet<_>>();
        if !diff.is_empty() {
            self.counter.incr();
            self.returned.insert(diff.iter().next()?.clone());
            return Some(diff.iter().next()?.clone())
        }

        self.reset();
        self.next()
    }

    fn reset(&mut self) {
        self.returned.clear();
    }
}
