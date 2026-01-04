use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LessonType {
    Agentic,
    Character(LessonGroup),
    Vocabulary(LessonGroup),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonConfig {
    pub name: String,
    pub path: String,
}

impl LessonConfig {
    pub fn stats_path(&self) -> String {
        format!("{}-stats.json", self.name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonGroup {
    pub name: String,
    pub lesson_map: HashMap<String, LessonConfig>,
    pub lesson_order: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub group_order: Vec<String>,
    pub group_map: HashMap<String, LessonType>,
}
