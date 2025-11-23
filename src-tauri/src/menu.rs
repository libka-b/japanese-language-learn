use crate::hiragana::{STATS, create_stats};
use crate::stats::{JsonCompatibleStats};
use std::sync::Mutex;

#[tauri::command]
pub fn exit_app(app_handle: tauri::AppHandle) {
    let stats = STATS.get_or_init(|| {
        Mutex::new(create_stats(app_handle.clone()))
    });
    let json_stats = JsonCompatibleStats::from_stats(stats.lock().unwrap().clone());
    json_stats.save_to_file(app_handle)
        .unwrap_or_else(|err| {
            panic!("Failed to save stats: {}", err)
        });

    std::process::exit(0);
}
