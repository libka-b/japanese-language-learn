use std::sync::Mutex;

pub struct Counter {
    pub current: i32,
    pub stop_at: i32,
}

pub static COUNTER: Mutex<Option<Counter>> = Mutex::new(None);

#[tauri::command]
pub fn set_counter(stop_at: i32) {
    let mut counter = COUNTER.lock().unwrap();
    *counter = Some(Counter { current: 0, stop_at });
}
