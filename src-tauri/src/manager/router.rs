use crate::manager::{CharacterEntry, Config, EntryCounter, Manager, Stats, VocabularyEntry};
use crate::manager::config::LessonType;
use std::collections::HashMap;
use tauri::AppHandle;

pub struct Router {
    character_manager_map: HashMap<String, Manager<CharacterEntry>>,
    vocabulary_manager_map: HashMap<String, Manager<VocabularyEntry>>,
}

impl Router {
    pub fn new(config: Config) -> Result<Self, String> {
        let mut character_manager_map = HashMap::new();
        let mut vocabulary_manager_map = HashMap::new();

        for lesson_group in config.group_map.values() {
            match lesson_group {
                LessonType::Character(group) => {
                    for lesson_config in group.lesson_map.values() {
                        if character_manager_map.contains_key(&lesson_config.name) {
                            return Err(format!(
                                "Lesson name `{}` already exists.",
                                lesson_config.name
                            ));
                        }
                        character_manager_map.insert(
                            lesson_config.name.to_string(),
                            Manager::new(lesson_config.path.to_string(), lesson_config.stats_path()),
                        );
                   }
                },
                LessonType::Vocabulary(group) => {
                    for lesson_config in group.lesson_map.values() {
                        if vocabulary_manager_map.contains_key(&lesson_config.name) {
                            return Err(format!(
                                "Lesson name `{}` already exists.",
                                lesson_config.name
                            ));
                        }
                        vocabulary_manager_map.insert(
                            lesson_config.name.to_string(),
                            Manager::new(lesson_config.path.to_string(), lesson_config.stats_path()),
                        );
                   }
                },
                LessonType::Agentic => {},
            }
        }

        Ok(Self { character_manager_map, vocabulary_manager_map })
    }

    pub fn get_next_character_entry(&mut self, handle: AppHandle, name: &str) -> Option<EntryCounter<CharacterEntry>> {
        self.character_manager_map.get_mut(name).unwrap().get_next(handle)
    }

    pub fn get_next_vocabulary_entry(&mut self, handle: AppHandle, name: &str) -> Option<EntryCounter<VocabularyEntry>> {
        self.vocabulary_manager_map.get_mut(name).unwrap().get_next(handle)
    }

    pub fn get_character_entry_stats(&mut self, handle: AppHandle) -> HashMap<String, Stats<CharacterEntry>> {
        self.character_manager_map
            .iter_mut()
            .map(|(name, manager)| (name.clone(), manager.get_stats(handle.clone())))
            .collect()
    }

    pub fn get_vocabulary_entry_stats(&mut self, handle: AppHandle) -> HashMap<String, Stats<VocabularyEntry>> {
        self.vocabulary_manager_map
            .iter_mut()
            .map(|(name, manager)| (name.clone(), manager.get_stats(handle.clone())))
            .collect()
    }

    pub fn add_correct_character_entry(&mut self, handle: AppHandle, name: &str, entry: CharacterEntry) {
        self.character_manager_map
            .get_mut(name)
            .unwrap()
            .add_correct(handle, entry);
    }

    pub fn add_correct_vocabulary_entry(&mut self, handle: AppHandle, name: &str, entry: VocabularyEntry) {
        self.vocabulary_manager_map
            .get_mut(name)
            .unwrap()
            .add_correct(handle, entry);
    }

    pub fn add_incorrect_character_entry(&mut self, handle: AppHandle, name: &str, entry: CharacterEntry) {
        self.character_manager_map
            .get_mut(name)
            .unwrap()
            .add_incorrect(handle, entry);
    }

    pub fn add_incorrect_vocabulary_entry(&mut self, handle: AppHandle, name: &str, entry: VocabularyEntry) {
        self.vocabulary_manager_map
            .get_mut(name)
            .unwrap()
            .add_incorrect(handle, entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        for manager in self.character_manager_map.values_mut() {
            manager.save_stats(handle.clone());
        }

        for manager in self.vocabulary_manager_map.values_mut() {
            manager.save_stats(handle.clone());
        }
    }
}
