use crate::manager::{Config, Entry, EntryCounter, Manager, Stats};
use std::collections::HashMap;
use tauri::AppHandle;

pub struct Router {
    manager_map: HashMap<String, Manager>,
}

impl Router {
    pub fn new(config: Config) -> Result<Self, String> {
        let mut map = HashMap::new();
        for lesson_group in config.group_map.values() {
            for lesson_config in lesson_group.lesson_map.values() {
                if map.contains_key(&lesson_config.name) {
                    return Err(format!(
                        "Lesson name `{}` already exists.",
                        lesson_config.name
                    ));
                }
                map.insert(
                    lesson_config.name.to_string(),
                    Manager::new(lesson_config.path.to_string(), lesson_config.stats_path()),
                );
            }
        }

        Ok(Self { manager_map: map })
    }

    pub fn get_next(&mut self, handle: AppHandle, name: &str) -> Option<EntryCounter> {
        self.manager_map.get_mut(name).unwrap().get_next(handle)
    }

    pub fn get_stats(&mut self, handle: AppHandle) -> HashMap<String, Stats> {
        self.manager_map
            .iter_mut()
            .map(|(name, manager)| (name.clone(), manager.get_stats(handle.clone())))
            .collect()
    }

    pub fn add_correct(&mut self, handle: AppHandle, name: &str, entry: Entry) {
        self.manager_map
            .get_mut(name)
            .unwrap()
            .add_correct(handle, entry);
    }

    pub fn add_incorrect(&mut self, handle: AppHandle, name: &str, entry: Entry) {
        self.manager_map
            .get_mut(name)
            .unwrap()
            .add_incorrect(handle, entry);
    }

    pub fn save_stats(&mut self, handle: AppHandle) {
        for manager in self.manager_map.values_mut() {
            manager.save_stats(handle.clone());
        }
    }
}
