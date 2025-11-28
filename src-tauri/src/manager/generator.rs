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
            let next = diff.iter().next()?.clone();
            self.returned.insert(next.clone());
            return Some(next)
        }

        let diff = self.entries.difference(&self.returned).cloned().collect::<HashSet<_>>();
        if !diff.is_empty() {
            self.counter.incr();
            let next = diff.iter().next()?.clone();
            self.returned.insert(next.clone());
            return Some(next)
        }

        self.reset();
        self.next()
    }

    fn reset(&mut self) {
        self.returned.clear();
    }
}
