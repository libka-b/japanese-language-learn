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
