use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
pub struct Config {
    pub lesson_order: Vec<String>,
    pub lesson_map: HashMap<String, LessonConfig>,
}
