mod stats;
mod menu;
mod hiragana;
mod counter;
mod entry_generator;

pub use menu::{exit_app};
pub use hiragana::{next_hiragana_entry, add_correct, add_incorrect, get_stats};
pub use counter::{set_counter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![
      exit_app,
      next_hiragana_entry,
      set_counter,
      add_correct,
      add_incorrect,
      get_stats,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
