use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};

pub trait GeminiSchema {
    fn get_gemini_schema() -> Map<String, Value>;
    fn get_gemini_required() -> Vec<String>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LessonData {
    pub japanese_text: String,
    pub difficulty: String,
}

impl GeminiSchema for LessonData {
    fn get_gemini_schema() -> Map<String, Value> {
        serde_json::json!({
            "japanese_text": {
                "type": "string",
                "description": "Japanese text in hiragana"
            },
            "difficulty": {
                "type": "string",
                "description": "Difficulty level"
            }
        }).as_object().unwrap().clone()
    }

    fn get_gemini_required() -> Vec<String> {
        vec![
            "japanese_text".to_string(),
            "difficulty".to_string(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub original_text: String,
    pub translation: String,
    pub correction: String,
    pub mistakes: String,
    pub suggestions: String,
    pub mark: f32,
}

impl GeminiSchema for Translation {
    fn get_gemini_schema() -> Map<String, Value> {
        serde_json::json!({
            "original_text": {
                "type": "string",
                "description": "Original AI generated text."
            },
            "translation": {
                "type": "string",
                "description": "User provided translation."
            },
            "correction": {
                "type": "string",
                "description": "Corrected translation (if necessary)."
            },
            "mistakes": {
                "type": "string",
                "description": "Mistakes made in translating the original text."
            },
            "suggestions": {
                "type": "string",
                "description": "Suggestions for improving the translation."
            },
            "mark": {
                "type": "number",
                "description": "Mark 1.0 - 5.0, where 1.0 is the best, evaluating the translation provided by user.",
            },
        }).as_object().unwrap().clone()
    }

    fn get_gemini_required() -> Vec<String> {
        vec!(
            "original_text".to_string(),
            "translation".to_string(),
            "correction".to_string(),
            "mistakes".to_string(),
            "suggestions".to_string(),
            "mark".to_string(),
        )
    }
}
