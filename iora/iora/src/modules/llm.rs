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

#[derive(Clone, Debug)]
pub struct LlmOutput {
    pub summary: String,
    pub signals: Vec<String>,
    pub confidence: f32,
    pub sources: Vec<String>,
}

impl LlmProvider {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(match s.to_lowercase().as_str() {
            "gemini" => Self::Gemini,
            "openai" => Self::OpenAI,
            "moonshot" => Self::Moonshot,
            "kimi" => Self::Kimi,
            "deepseek" => Self::DeepSeek,
            "together" => Self::Together,
            "mistral" => Self::Custom("mistral".to_string()),
            "aimlapi" => Self::Custom("aimlapi".to_string()),
            _ => return Err(format!("unsupported provider: {}", s)),
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

// New LLM runner function for MCP - Mistral with graceful fallback due to free tier limitations
pub async fn run_llm(provider: &LlmProvider, prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    match provider {
        LlmProvider::Custom(name) if name == "mistral" => {
            // Try Mistral first, but fallback to Gemini due to free tier quota limitations
            match run_mistral_llm_with_retry(prompt).await {
                Ok(result) => {
                    eprintln!("Mistral analysis successful!");
                    Ok(result)
                },
                Err(e) => {
                    eprintln!("Mistral failed due to free tier quota ({}), falling back to Gemini for demo", e);
                    run_gemini_llm(prompt).await
                }
            }
        },
        LlmProvider::Custom(name) if name == "aimlapi" => {
            // Try AIML API first, fallback to Gemini on failure
            match run_aimlapi_llm(prompt).await {
                Ok(result) => Ok(result),
                Err(e) => {
                    eprintln!("AIML API failed ({}), falling back to Gemini", e);
                    run_gemini_llm(prompt).await
                }
            }
        },
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
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let body = json!({
      "contents": [{
        "parts": [{
          "text": format!("{}\n\n{}", SYSTEM_JSON_SCHEMA, prompt)
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

        async fn run_mistral_llm_with_retry(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
            // Since run_mistral_llm now handles model fallbacks internally,
            // we only do basic retries for network issues, not for capacity errors
            let mut attempts = 0;
            let max_attempts = 2; // Reduced since model fallback handles most capacity issues

            while attempts < max_attempts {
                match run_mistral_llm(prompt).await {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        let error_str = e.to_string();
                        // Only retry on network errors, not capacity/model issues
                        if error_str.contains("network") || error_str.contains("timeout") {
                            attempts += 1;
                            if attempts < max_attempts {
                                let wait_time = 1u64; // Short wait for network issues
                                eprintln!("Mistral network error (attempt {}/{}), waiting {}s before retry...", attempts, max_attempts, wait_time);
                                tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;
                                continue;
                            }
                        }
                        // For capacity errors or model failures, don't retry
                        return Err(e);
                    }
                }
            }

            Err("Max retries exceeded for Mistral API".into())
        }

        async fn run_mistral_llm(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
            use serde_json::json;

            let api_key = std::env::var("MISTRAL_API_KEY")?;
            let base = std::env::var("MISTRAL_BASE").unwrap_or_else(|_| "https://api.mistral.ai".into());
            // Use the smallest model for best free tier compatibility
            let model = "mistral-tiny"; // Smallest model, most likely to work with free credits
            let url = format!("{base}/v1/chat/completions");

            let cli = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?;

            let body = json!({
              "model": model,
              "messages":[
                {"role":"system","content": SYSTEM_JSON_SCHEMA},
                {"role":"user","content": prompt}
              ],
              "temperature": 0.2,
              "max_tokens": 800  // Keep low for free tier limits
            });

            let res = cli.post(&url)
                .bearer_auth(&api_key)
                .json(&body)
                .send().await;

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        let json_response: serde_json::Value = response.json().await?;
                        // Accept both content fields (provider drift-proof)
                        let text = json_response.pointer("/choices/0/message/content")
                            .or_else(|| json_response.pointer("/choices/0/text"))
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| "unexpected Mistral response")?;
                        return parse_structured_json(text);
                    } else {
                        // Return error for any non-success status (including 429)
                        return Err(format!("Mistral API error: {} (free tier may be exhausted)", response.status()).into());
                    }
                }
                Err(e) => {
                    return Err(format!("Mistral network error: {} (free tier may be exhausted)", e).into());
                }
            }
        }

async fn run_aimlapi_llm(prompt: &str) -> Result<LlmOutput, Box<dyn std::error::Error + Send + Sync>> {
    use serde_json::json;

    let api_key = std::env::var("AIMLAPI_API_KEY")?;
    let base = std::env::var("AIMLAPI_BASE").unwrap_or_else(|_| "https://api.aimlapi.com".into());
    let model = std::env::var("AIMLAPI_MODEL").unwrap_or_else(|_| "llama-3.1-70b-instruct".into());
    let url = format!("{base}/chat/completions");

    let cli = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let body = json!({
      "model": model,
      "messages":[
        {"role":"system","content": SYSTEM_JSON_SCHEMA},
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
            // Handle markdown code blocks by extracting JSON from within ```json ... ``` or ``` ... ```
            let trimmed = text.trim();

            let json_text = if trimmed.starts_with("```json") {
                // Extract content between ```json and ```
                let start = trimmed.find("```json").unwrap() + 7;
                let after_start = &trimmed[start..].trim();
                if let Some(end_pos) = after_start.find("```") {
                    &after_start[..end_pos]
                } else {
                    after_start
                }
            } else if trimmed.starts_with("```") {
                // Extract content between ``` and ```
                let start = trimmed.find("```").unwrap() + 3;
                let after_start = &trimmed[start..].trim();
                if let Some(end_pos) = after_start.find("```") {
                    &after_start[..end_pos]
                } else {
                    after_start
                }
            } else {
                trimmed
            };

            // Clean up any remaining whitespace and potential language specifiers
            let json_text = json_text.trim();
            let json_text = if json_text.starts_with("json\n") {
                &json_text[5..] // Remove "json\n" prefix if present
            } else {
                json_text
            }.trim();

            // Must start and end with braces (JSON object)
            if !json_text.starts_with('{') || !json_text.ends_with('}') {
                return Err(anyhow::anyhow!("LLM returned invalid format (not JSON object). Raw response starts with: {}", json_text.chars().take(100).collect::<String>()).into());
            }

            // Parse JSON strictly
            let v: serde_json::Value = serde_json::from_str(json_text)
                .map_err(|e| anyhow::anyhow!("LLM returned invalid JSON (error: {}). JSON starts with: {}", e, json_text.chars().take(100).collect::<String>()))?;

            // Validate required structure
            let summary = v["summary"].as_str()
                .ok_or_else(|| anyhow::anyhow!("LLM JSON missing required 'summary' field"))?;
            let signals = v["signals"].as_array()
                .ok_or_else(|| anyhow::anyhow!("LLM JSON missing required 'signals' array"))?;
            let confidence = v["confidence"].as_f64()
                .ok_or_else(|| anyhow::anyhow!("LLM JSON missing required 'confidence' number"))?;
            let sources = v["sources"].as_array()
                .ok_or_else(|| anyhow::anyhow!("LLM JSON missing required 'sources' array"))?;

            // Validate confidence range
            if confidence < 0.0 || confidence > 1.0 {
                return Err(anyhow::anyhow!("LLM confidence {} out of range [0,1]", confidence).into());
            }

            Ok(LlmOutput{
                summary: summary.to_string(),
                signals: signals.iter()
                              .filter_map(|x| x.as_str().map(|s| s.to_string()))
                              .collect(),
                confidence: confidence as f32,
                sources: sources.iter()
                              .filter_map(|x| x.as_str().map(|s| s.to_string()))
                              .collect(),
            })
        }

const SYSTEM_JSON_SCHEMA: &str = r#"
You are a market analysis engine. Respond ONLY as strict JSON:
{"summary": string, "signals": string[], "confidence": number (0..1), "sources": string[]}
No prose outside JSON.
"#;