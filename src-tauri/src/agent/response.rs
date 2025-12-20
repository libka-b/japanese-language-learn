use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsePart {
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseContent {
    pub parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub content: ResponseContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}
