use serde::{Deserialize, Serialize, de::DeserializeOwned};
use crate::manager::Counter;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CharacterEntry {
    pub japanese: String,
    pub english: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VocabularyEntry {
    pub japanese: String,
    pub pronunciation: String,
    pub english: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCounter<T> {
    pub entry: T,
    pub counter: Counter,
}

#[derive(Debug, Clone)]
pub struct Stats<T> {
    pub total: u32,
    pub incorrect: u32,
    pub wrong: HashMap<T, u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct EntryCount<T: DeserializeOwned + Serialize> {
    pub entry: T,
    pub count: u32,
}
