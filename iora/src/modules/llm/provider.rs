use anyhow::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LlmProvider {
    Gemini,
    Mistral,
    AimlApi
}

impl LlmProvider {
    pub fn parse(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().as_str() {
            "gemini" => Self::Gemini,
            "mistral" => Self::Mistral,
            "aimlapi" => Self::AimlApi,
            _ => bail!("unsupported provider: {s}"),
        })
    }
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gemini => write!(f, "gemini"),
            Self::Mistral => write!(f, "mistral"),
            Self::AimlApi => write!(f, "aimlapi"),
        }
    }
}
