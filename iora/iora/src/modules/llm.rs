#[derive(Debug, Clone, PartialEq)]
pub enum LlmProvider {
    Gemini,
    OpenAI,
    Moonshot,
    Kimi,
    DeepSeek,
    Together,
    Custom(String),
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmProvider::Gemini => write!(f, "Gemini"),
            LlmProvider::OpenAI => write!(f, "OpenAI"),
            LlmProvider::Moonshot => write!(f, "Moonshot"),
            LlmProvider::Kimi => write!(f, "Kimi"),
            LlmProvider::DeepSeek => write!(f, "DeepSeek"),
            LlmProvider::Together => write!(f, "Together"),
            LlmProvider::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

impl LlmConfig {
    pub fn gemini(api_key: String) -> Self {
        Self {
            provider: LlmProvider::Gemini,
            api_key,
            base_url: "https://generativelanguage.googleapis.com".to_string(),
            model: "gemini-1.5-flash".to_string(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }

    pub fn openai(api_key: String) -> Self {
        Self {
            provider: LlmProvider::OpenAI,
            api_key,
            base_url: "https://api.openai.com".to_string(),
            model: "gpt-4".to_string(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }

    pub fn moonshot(api_key: String) -> Self {
        Self {
            provider: LlmProvider::Moonshot,
            api_key,
            base_url: "https://api.moonshot.ai".to_string(),
            model: "moonshot-v1-8k".to_string(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }

    pub fn kimi(api_key: String) -> Self {
        Self {
            provider: LlmProvider::Kimi,
            api_key,
            base_url: "https://api.moonshot.ai".to_string(), // Kimi uses Moonshot's API
            model: "kimi-latest".to_string(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }

    pub fn custom(provider: LlmProvider, api_key: String, base_url: String, model: String) -> Self {
        Self {
            provider,
            api_key,
            base_url,
            model,
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }
}