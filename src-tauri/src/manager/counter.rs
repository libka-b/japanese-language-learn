use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    current: u32,
    stop_at: u32,
}

impl Counter {
    pub fn new(stop_at: u32) -> Self {
        Self { current: 0, stop_at }
    }

    pub fn incr(&mut self) {
        self.current += 1;
    }

    pub fn should_continue(&self) -> bool {
        self.current < self.stop_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_incr() {
        let mut counter = Counter::new(1);
        counter.incr();
        assert_eq!(counter.current, 1);
        assert_eq!(counter.stop_at, 1);
    }

    #[test]
    fn test_counter_should_continue() {
        let mut counter = Counter::new(1);
        counter.incr();
        assert_eq!(counter.should_continue(), false);
    }
}
