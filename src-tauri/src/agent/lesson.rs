use rand::seq::IndexedRandom;
use rand::rng;
use crate::agent::ApiKey;
use crate::agent::gemini::query_gemini;
use crate::agent::types::{LessonData, Translation};

const SYSTEM_INSTRUCTION: &str = r#"You are an AI agent called from application meant for learning Japanese. 
Your task is to generate Japanese text for users to translate and then, 
given both the original text and translation, evaluate the translation and 
provide pointers, explain mistakes and suggest ways to improve. 
Generate text without any control characters or diacritics so that it is easy for 
users to read."#;

const TOPICS: &[&str] = &[
    "commuting",
    "shopping",
    "restaurant",
    "weather",
    "workplace",
    "doctor",
    "hobby",
    "traveling",
    "friends",
    "technology",
];

pub fn generate_lesson(api_key: ApiKey) -> Result<LessonData, Box<dyn std::error::Error>> {
    let topic = TOPICS.choose(&mut rng()).unwrap();
    query_gemini(
        api_key,
        format!("Generate simple text using only hiragana for user to translate. The text should be about ${}", topic),
        SYSTEM_INSTRUCTION.to_string(),
    )
}

pub fn validate_translation(original: String, translation: String, api_key: ApiKey) -> Result<Translation, Box<dyn std::error::Error>> {
    query_gemini(
        api_key,
        format!(
            "This is the original generated japanese text: '{original}'. 
            This is the user provided english translation: '{translation}'. 
            Validate the translation and if it's wrong, provide correct 
            translation of the original text and pointers for the user to improve.",
            original = original,
            translation = translation,
        ),
        SYSTEM_INSTRUCTION.to_string(),
    )
}
