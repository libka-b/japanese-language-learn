use serde::de::DeserializeOwned;
use crate::agent::request::{Content, GeminiRequest, Part, SystemInstruction, GenerationConfig, ResponseSchema};
use crate::agent::response::GeminiResponse;
use std::env;
use crate::agent::types::GeminiSchema;

pub fn query_gemini<T: GeminiSchema + DeserializeOwned>(
    prompt: String,
    system_instruction: String,
) -> Result<T, Box<dyn std::error::Error>> {
    let request = GeminiRequest {
        contents: vec![
            Content {
                parts: vec![
                    Part {
                        text: prompt,
                    },
                ],
            },
        ],
        system_instruction: SystemInstruction {
            parts: vec![
                Part {
                    text: system_instruction,
                },
            ],
        },
        generation_config: GenerationConfig {
            response_mime_type: "application/json".to_string(),
            response_schema: ResponseSchema {
                schema_type: "object".to_string(),
                properties: T::get_gemini_schema(),
                required: T::get_gemini_required(),
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
