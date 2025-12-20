use crate::agent::request::{Content, GeminiRequest, Part, SystemInstruction, GenerationConfig, ResponseSchema};
use crate::agent::response::GeminiResponse;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct LessonData {
    pub japanese_text: String,
    pub difficulty: String,
}

pub fn query_gemini() -> Result<LessonData, Box<dyn std::error::Error>> {
    let request = GeminiRequest {
        contents: vec![
            Content {
                parts: vec![
                    Part {
                        text: "Generate simple text in hiragana for user to translate.".to_string(),
                    },
                ],
            },
        ],
        system_instruction: SystemInstruction {
            parts: vec![
                Part {
                    text: r#"You are an AI agent called from application meant for learning Japanese. 
                    Your task is to generate Japanese text for users to translate and then, 
                    given both the original text and translation, evaluate the translation and 
                    provide pointers, explain mistakes and suggest ways to improve. 
                    Generate text without any control characters or diacritics so that it is easy for 
                    users to read."#.to_string(),
                },
            ],
        },
        generation_config: GenerationConfig {
            response_mime_type: "application/json".to_string(),
            response_schema: ResponseSchema {
                schema_type: "object".to_string(),
                properties: serde_json::json!({
                    "japanese_text": {
                        "type": "string",
                        "description": "Japanese text in hiragana"
                    },
                    "difficulty": {
                        "type": "string",
                        "description": "Difficulty level"
                    }
                }).as_object().unwrap().clone(),
                required: vec!["japanese_text".to_string()],
            },
        },
    };

    let body = serde_json::to_string(&request).map_err(|e| e.to_string())?;

    let response = reqwest::blocking::Client::new()
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent")
        .header("Content-Type", "application/json")
        .header("x-goog-api-key", env::var("GEMINI_API_KEY").unwrap())
        .body(body)
        .send()?;

    let status = response.status();
    let response_text = response.text()?;

    if !status.is_success() {
        return Err(format!("API Error {}: {}", status, response_text).into());
    }

    let gemini_response: GeminiResponse = serde_json::from_str(&response_text)?;

    let data = gemini_response.candidates[0].content.parts[0].text.clone();

    Ok(serde_json::from_str(&data)?)
}
