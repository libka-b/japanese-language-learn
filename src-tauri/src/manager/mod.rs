mod character_learning_manager;
mod config;
mod counter;
mod generator;
mod model;
mod router;
mod stats;
mod utils;

#[allow(clippy::module_inception)]
mod manager;

pub use config::Config;
use counter::Counter;
use generator::Generator;
pub use manager::Manager;
pub use model::{
    CharacterEntry, CharacterEntryTable, EntryCount, EntryCounter, Stats, VocabularyEntry,
};
pub use router::Router;
pub use stats::JsonCompatibleStats;
