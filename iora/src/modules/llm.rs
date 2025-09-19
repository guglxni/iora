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

impl LlmProvider {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(match s.to_lowercase().as_str() {
            "gemini" => Self::Gemini,
            "mistral" => Self::Custom("mistral".to_string()),
            "aimlapi" => Self::Custom("aimlapi".to_string()),
            "openai" => Self::OpenAI,
            "moonshot" => Self::Moonshot,
            "kimi" => Self::Kimi,
            "deepseek" => Self::DeepSeek,
            "together" => Self::Together,
            _ => return Err(format!("unsupported provider: {s}")),
        })
    }
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

// New LLM output structure for MCP
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmOutput {
    pub summary: String,
    pub signals: Vec<String>,
    pub confidence: f32,
    pub sources: Vec<String>,
}

// New LLM runner function for MCP
pub async fn run_llm(provider: &LlmProvider, prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    match provider {
        LlmProvider::Custom(name) if name == "mistral" => run_mistral_llm(prompt).await,
        LlmProvider::Custom(name) if name == "aimlapi" => run_aimlapi_llm(prompt).await,
        _ => run_gemini_llm(prompt).await, // Default to Gemini for other providers
    }
}

async fn run_gemini_llm(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    use serde_json::json;

    let api_key = std::env::var("GEMINI_API_KEY")?;
    let base = std::env::var("GEMINI_BASE").unwrap_or_else(|_| "https://generativelanguage.googleapis.com".into());
    let model = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-flash".into());
    let url = format!("{base}/v1beta/models/{model}:generateContent?key={api_key}");

    let cli = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))  // Increase timeout for LLM calls
        .build()?;

    let body = json!({
      "contents": [{
        "parts": [{
          "text": format!("{}\n\n{}", crate::modules::llm::prompt::SYSTEM_JSON_SCHEMA, prompt)
        }]
      }],
      "generationConfig": {
        "temperature": 0.2,
        "maxOutputTokens": 2048
      }
    });

    let res = cli.post(&url)
        .json(&body)
        .send().await?
        .error_for_status()?
        .json::<serde_json::Value>().await?;

    let text = res.pointer("/candidates/0/content/parts/0/text")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "unexpected Gemini response")?;

    parse_structured_json(text)
}

async fn run_mistral_llm(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    use serde_json::json;

    let api_key = std::env::var("MISTRAL_API_KEY")?;
    let base = std::env::var("MISTRAL_BASE").unwrap_or_else(|_| "https://api.mistral.ai".into());
    let model = std::env::var("MISTRAL_MODEL").unwrap_or_else(|_| "mistral-large-latest".into());
    let url = format!("{base}/v1/chat/completions");

    let cli = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))  // Increase timeout for LLM calls
        .build()?;

    let body = json!({
      "model": model,
      "messages":[
        {"role":"system","content": crate::modules::llm::prompt::SYSTEM_JSON_SCHEMA},
        {"role":"user","content": prompt}
      ],
      "temperature": 0.2
    });

    let res = cli.post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send().await?
        .error_for_status()?
        .json::<serde_json::Value>().await?;

    // Accept both content fields (provider drift-proof)
    let text = res.pointer("/choices/0/message/content")
        .or_else(|| res.pointer("/choices/0/text"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "unexpected Mistral response")?;

    parse_structured_json(text)
}

async fn run_aimlapi_llm(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    use serde_json::json;

    let api_key = std::env::var("AIMLAPI_API_KEY")?;
    let base = std::env::var("AIMLAPI_BASE").unwrap_or_else(|_| "https://api.aimlapi.com".into());
    let model = std::env::var("AIMLAPI_MODEL").unwrap_or_else(|_| "llama-3.1-70b-instruct".into());
    let url = format!("{base}/chat/completions");

    let cli = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))  // Increase timeout for LLM calls
        .build()?;

    let body = json!({
      "model": model,
      "messages":[
        {"role":"system","content": crate::modules::llm::prompt::SYSTEM_JSON_SCHEMA},
        {"role":"user","content": prompt}
      ],
      "temperature": 0.2
    });

    let res = cli.post(&url)
        .bearer_auth(api_key)
        .json(&body)
        .send().await?
        .error_for_status()?
        .json::<serde_json::Value>().await?;

    // Accept both content fields (provider drift-proof)
    let text = res.pointer("/choices/0/message/content")
        .or_else(|| res.pointer("/choices/0/text"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "unexpected AIML API response")?;

    parse_structured_json(text)
}

fn parse_structured_json(text: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    // Expect the model to return strict JSON per SYSTEM_JSON_SCHEMA
    let v: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| format!("LLM did not return JSON: {} (error: {})", text, e))?;

    Ok(LlmOutput{
        summary: v["summary"].as_str().unwrap_or_default().to_string(),
        signals: v["signals"].as_array().unwrap_or(&vec![]).iter()
                  .filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
        confidence: v["confidence"].as_f64().unwrap_or(0.5) as f32,
        sources: v["sources"].as_array().unwrap_or(&vec![]).iter()
                  .filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
    })
}

// Module for MCP provider parsing
pub mod provider {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum LlmProvider {
        Gemini,
        Mistral,
        AimlApi
    }

    impl LlmProvider {
        pub fn parse(s: &str) -> Result<Self, String> {
            Ok(match s.to_lowercase().as_str() {
                "gemini" => Self::Gemini,
                "mistral" => Self::Mistral,
                "aimlapi" => Self::AimlApi,
                _ => return Err(format!("unsupported provider: {s}")),
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
}

// Module for MCP prompts
pub mod prompt {
    pub const SYSTEM_JSON_SCHEMA: &str = r#"
You are a market analysis engine. Respond ONLY as strict JSON:
{"summary": string, "signals": string[], "confidence": number (0..1), "sources": string[]}
No prose outside JSON.
"#;
}