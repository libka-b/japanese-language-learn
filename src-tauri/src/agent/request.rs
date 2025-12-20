use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInstruction {
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub properties: Map<String, Value>,
    pub required: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerationConfig {
    pub response_mime_type: String,
    pub response_schema: ResponseSchema,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    pub system_instruction: SystemInstruction,
    pub generation_config: GenerationConfig,
}
