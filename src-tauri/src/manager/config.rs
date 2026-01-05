use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LessonType {
    AgenticExercise,
    CharacterExercise(LessonGroup),
    VocabularyExercise(LessonGroup),
    CharacterLearning(CharacterLearningLesson),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterLearningLessonConfig {
    pub name: String,
    pub character_path: String,
    pub character_table_path: String,
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
pub struct CharacterLearningLesson {
    pub name: String,
    pub lesson_map: HashMap<String, CharacterLearningLessonConfig>,
    pub lesson_order: Vec<String>,
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
