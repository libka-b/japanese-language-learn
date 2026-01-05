use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use crate::manager::utils::load_csv_entries;
use crate::manager::{CharacterEntry, CharacterEntryTable};
use crate::manager::model::CharacterEntryRow;
use crate::manager::config::CharacterLearningLessonConfig;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct EntryRow {
    pub col1: String,
    pub col2: String,
    pub col3: String,
    pub col4: String,
    pub col5: String,
}

pub struct CharacterLearningManager {
    character_learning_lesson_config: CharacterLearningLessonConfig,
    character_entry_table: Option<CharacterEntryTable>,
}

impl CharacterLearningManager {
    pub fn new(character_learning_lesson_config: CharacterLearningLessonConfig) -> Self {
        Self {
            character_learning_lesson_config,
            character_entry_table: None,
        }
    }

    pub fn get_character_entry_table(&mut self, handle: AppHandle) -> CharacterEntryTable {
        if self.character_entry_table.is_none() {
            self.load_character_entry_table(handle);
        };

        self.character_entry_table
            .clone()
            .expect("Something went wrong and the character table is not loaded properly.")
    }

    fn load_character_entry_table(&mut self, handle: AppHandle) {
        let character_map = self.load_character_map(handle.clone());

        let character_rows = load_csv_entries::<EntryRow>(
            &self.character_learning_lesson_config.character_table_path,
            handle,
        );

        let character_entry_rows: Vec<CharacterEntryRow> = character_rows.into_iter()
            .map(|row| {
                CharacterEntryRow {
                    col1: character_map.get(&row.col1).cloned(),
                    col2: character_map.get(&row.col2).cloned(),
                    col3: character_map.get(&row.col3).cloned(),
                    col4: character_map.get(&row.col4).cloned(),
                    col5: character_map.get(&row.col5).cloned(),
                }
            })
            .collect();

        self.character_entry_table = Some(CharacterEntryTable { rows: character_entry_rows });
    }

    fn load_character_map(&self, handle: AppHandle) -> HashMap<String, CharacterEntry> {
        let character_entries = load_csv_entries::<CharacterEntry>(
            &self.character_learning_lesson_config.character_path,
            handle,
        );

        character_entries.into_iter()
            .map(|entry| (entry.japanese.clone(), entry))
            .collect()
    }
}
