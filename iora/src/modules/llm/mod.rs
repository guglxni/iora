use anyhow::*;
use serde::{Deserialize, Serialize};
use provider::LlmProvider;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LlmOutput {
    pub summary: String,
    pub signals: Vec<String>,
    pub confidence: f32,
    pub sources: Vec<String>,
}

pub async fn run_llm(provider: LlmProvider, prompt: &str) -> Result<LlmOutput> {
    match provider {
        LlmProvider::Mistral => clients::mistral::call(prompt).await,
        LlmProvider::AimlApi => clients::aimlapi::call(prompt).await,
        LlmProvider::Gemini  => clients::gemini::call(prompt).await,
    }
}

pub mod clients;
pub mod provider;
pub mod prompt;
