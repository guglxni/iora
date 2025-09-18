use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use regex::Regex;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use std::collections::HashMap;

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

#[derive(Debug)]
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
            base_url: "https://api.moonshot.cn".to_string(),
            model: "moonshot-v1-8k".to_string(),
            max_tokens: Some(2048),
            temperature: Some(0.7),
        }
    }

    pub fn kimi(api_key: String) -> Self {
        Self {
            provider: LlmProvider::Kimi,
            api_key,
            base_url: "https://api.moonshot.cn".to_string(), // Kimi uses Moonshot's API
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub insight: String,
    pub processed_price: f64,
    pub confidence: f32,
    pub recommendation: String,
    pub raw_data: super::fetcher::RawData,
}

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Debug, Deserialize)]
struct ContentResponse {
    parts: Vec<Part>,
}

#[derive(Debug)]
pub enum AnalyzerError {
    ApiError(String),
    ParseError(String),
    RateLimitError,
    InvalidResponse(String),
}

#[derive(Debug)]
struct RateLimitInfo {
    requests_remaining: Option<u32>,
    requests_reset_time: Option<Instant>,
    tokens_remaining: Option<u32>,
    tokens_reset_time: Option<Instant>,
}

#[derive(Debug)]
struct GeminiRateLimitHandler {
    rate_limit_info: std::sync::Mutex<Option<RateLimitInfo>>,
    last_request_time: std::sync::Mutex<Option<Instant>>,
}

impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzerError::ApiError(msg) => write!(f, "API Error: {}", msg),
            AnalyzerError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            AnalyzerError::RateLimitError => write!(f, "Rate Limit Exceeded"),
            AnalyzerError::InvalidResponse(msg) => write!(f, "Invalid Response: {}", msg),
        }
    }
}

impl Error for AnalyzerError {}

impl GeminiRateLimitHandler {
    fn new() -> Self {
        Self {
            rate_limit_info: std::sync::Mutex::new(None),
            last_request_time: std::sync::Mutex::new(None),
        }
    }

    fn update_rate_limits(&self, response: &reqwest::Response) {
        let mut info = self.rate_limit_info.lock().unwrap();

        // Debug: print all rate limit related headers
        println!("🔍 Rate limit headers received:");
        for (key, value) in response.headers() {
            if key.as_str().to_lowercase().contains("rate") || key.as_str().to_lowercase().contains("limit") {
                println!("   {}: {:?}", key, value);
            }
        }

        let new_info = RateLimitInfo {
            requests_remaining: response.headers()
                .get("x-ratelimit-requests-remaining")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok()),
            requests_reset_time: response.headers()
                .get("x-ratelimit-requests-reset")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .map(|ts: u64| Instant::now() + Duration::from_secs(ts)),
            tokens_remaining: response.headers()
                .get("x-ratelimit-tokens-remaining")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok()),
            tokens_reset_time: response.headers()
                .get("x-ratelimit-tokens-reset")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok())
                .map(|ts: u64| Instant::now() + Duration::from_secs(ts)),
        };

        println!("🔍 Parsed rate limit info: {:?}", new_info);
        *info = Some(new_info);
    }

    async fn wait_for_rate_limit_reset(&self) -> Result<(), AnalyzerError> {
        let info = self.rate_limit_info.lock().unwrap();

        println!("🔍 Rate limit info: {:?}", *info);

        if let Some(ref info) = *info {
            let now = Instant::now();

            // Check if we need to wait for requests reset
            if let Some(requests_reset) = info.requests_reset_time {
                if now < requests_reset {
                    let wait_duration = requests_reset - now;
                    println!("⏳ Rate limit: Waiting {}s for requests reset...", wait_duration.as_secs());
                    sleep(wait_duration).await;
                    return Ok(());
                }
            }

            // Check if we need to wait for tokens reset
            if let Some(tokens_reset) = info.tokens_reset_time {
                if now < tokens_reset {
                    let wait_duration = tokens_reset - now;
                    println!("⏳ Rate limit: Waiting {}s for tokens reset...", wait_duration.as_secs());
                    sleep(wait_duration).await;
                    return Ok(());
                }
            }
        } else {
            println!("ℹ️  No rate limit info available, assuming we can retry soon...");
            // Wait a short time if we don't have rate limit info
            sleep(Duration::from_secs(5)).await;
        }

        Ok(())
    }

    fn can_make_request(&self) -> bool {
        let info = self.rate_limit_info.lock().unwrap();

        if let Some(ref info) = *info {
            // Check if we have remaining requests or tokens
            let has_requests = info.requests_remaining.map(|r| r > 0).unwrap_or(true);
            let has_tokens = info.tokens_remaining.map(|t| t > 0).unwrap_or(true);

            has_requests && has_tokens
        } else {
            // No rate limit info yet, assume we can make requests
            true
        }
    }
}

pub struct Analyzer {
    client: Client,
    llm_config: LlmConfig,
    rate_limit_handler: GeminiRateLimitHandler,
}

impl Analyzer {
    pub fn new(llm_config: LlmConfig) -> Self {
        Self {
            client: Client::new(),
            llm_config,
            rate_limit_handler: GeminiRateLimitHandler::new(),
        }
    }

    // Convenience constructor for Gemini (backward compatibility)
    pub fn new_gemini(api_key: String) -> Self {
        Self::new(LlmConfig::gemini(api_key))
    }

    // Convenience constructors for other providers
    pub fn new_openai(api_key: String) -> Self {
        Self::new(LlmConfig::openai(api_key))
    }

    pub fn new_moonshot(api_key: String) -> Self {
        Self::new(LlmConfig::moonshot(api_key))
    }

    pub fn new_kimi(api_key: String) -> Self {
        Self::new(LlmConfig::kimi(api_key))
    }

    pub async fn analyze(
        &self,
        augmented_data: &super::rag::AugmentedData,
    ) -> Result<Analysis, Box<dyn Error>> {
        println!("🤖 Starting {} analysis...", self.llm_config.provider);

        // Check if we can make a request based on rate limits
        if !self.rate_limit_handler.can_make_request() {
            println!("⏳ Waiting for rate limit reset before making request...");
            self.rate_limit_handler.wait_for_rate_limit_reset().await?;
        }

        let prompt = self.build_analysis_prompt(augmented_data);

        match self.llm_config.provider {
            LlmProvider::Gemini => self.analyze_gemini(&prompt, augmented_data).await,
            LlmProvider::OpenAI | LlmProvider::Moonshot | LlmProvider::Kimi | LlmProvider::DeepSeek | LlmProvider::Together | LlmProvider::Custom(_) => {
                self.analyze_openai_compatible(&prompt, augmented_data).await
            }
        }
    }

    async fn analyze_gemini(&self, prompt: &str, augmented_data: &super::rag::AugmentedData) -> Result<Analysis, Box<dyn Error>> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt.to_string() }],
            }],
        };

        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            self.llm_config.base_url.trim_end_matches('/'),
            self.llm_config.model,
            self.llm_config.api_key
        );

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| AnalyzerError::ApiError(format!("Request failed: {}", e)))?;

        // Update rate limit information from response headers
        self.rate_limit_handler.update_rate_limits(&response);

        // Handle rate limiting with intelligent retry
        if response.status() == 429 {
            println!("🔄 Rate limit hit (429). Waiting for reset and will retry on next call...");
            self.rate_limit_handler.wait_for_rate_limit_reset().await?;
            return Err(Box::new(AnalyzerError::RateLimitError));
        }

        if !response.status().is_success() {
            let status = response.status();
            let status_text = response.text().await.unwrap_or_default();
            return Err(Box::new(AnalyzerError::ApiError(format!(
                "HTTP {}: {}", status, status_text
            ))));
        }

        let gemini_response: GeminiResponse = response.json().await
            .map_err(|e| AnalyzerError::ParseError(format!("Failed to parse response: {}", e)))?;

        if gemini_response.candidates.is_empty() {
            return Err(Box::new(AnalyzerError::InvalidResponse(
                "No candidates in response".to_string()
            )));
        }

        let candidate = &gemini_response.candidates[0];
        if candidate.content.parts.is_empty() {
            return Err(Box::new(AnalyzerError::InvalidResponse(
                "No content parts in response".to_string()
            )));
        }

        let analysis_text = &candidate.content.parts[0].text;
        let mut analysis = self.parse_gemini_response(analysis_text, augmented_data.raw_data.clone())?;
        analysis.raw_data = augmented_data.raw_data.clone();
        Ok(analysis)
    }

    async fn analyze_openai_compatible(&self, prompt: &str, augmented_data: &super::rag::AugmentedData) -> Result<Analysis, Box<dyn Error>> {
        #[derive(Serialize)]
        struct OpenAIRequest {
            model: String,
            messages: Vec<HashMap<String, String>>,
            max_tokens: Option<u32>,
            temperature: Option<f32>,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<OpenAIChoice>,
        }

        #[derive(Deserialize)]
        struct OpenAIChoice {
            message: OpenAIMessage,
        }

        #[derive(Deserialize)]
        struct OpenAIMessage {
            content: String,
        }

        let mut messages = Vec::new();
        let mut system_message = HashMap::new();
        system_message.insert("role".to_string(), "system".to_string());
        system_message.insert("content".to_string(), "You are a cryptocurrency analyst. Analyze the given data and provide insights in the exact format requested.".to_string());
        messages.push(system_message);

        let mut user_message = HashMap::new();
        user_message.insert("role".to_string(), "user".to_string());
        user_message.insert("content".to_string(), prompt.to_string());
        messages.push(user_message);

        let request = OpenAIRequest {
            model: self.llm_config.model.clone(),
            messages,
            max_tokens: self.llm_config.max_tokens,
            temperature: self.llm_config.temperature,
        };

        let url = format!(
            "{}/v1/chat/completions",
            self.llm_config.base_url.trim_end_matches('/')
        );

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.llm_config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AnalyzerError::ApiError(format!("Request failed: {}", e)))?;

        // Update rate limit information from response headers
        self.rate_limit_handler.update_rate_limits(&response);

        // Handle rate limiting with intelligent retry
        if response.status() == 429 {
            println!("🔄 Rate limit hit (429). Waiting for reset and will retry on next call...");
            self.rate_limit_handler.wait_for_rate_limit_reset().await?;
            return Err(Box::new(AnalyzerError::RateLimitError));
        }

        if !response.status().is_success() {
            let status = response.status();
            let status_text = response.text().await.unwrap_or_default();
            return Err(Box::new(AnalyzerError::ApiError(format!(
                "HTTP {}: {}", status, status_text
            ))));
        }

        let openai_response: OpenAIResponse = response.json().await
            .map_err(|e| AnalyzerError::ParseError(format!("Failed to parse response: {}", e)))?;

        if openai_response.choices.is_empty() {
            return Err(Box::new(AnalyzerError::InvalidResponse(
                "No choices in response".to_string()
            )));
        }

        let analysis_text = &openai_response.choices[0].message.content;
        let mut analysis = self.parse_gemini_response(analysis_text, augmented_data.raw_data.clone())?;
        analysis.raw_data = augmented_data.raw_data.clone();
        Ok(analysis)
    }


    fn build_analysis_prompt(&self, augmented_data: &super::rag::AugmentedData) -> String {
        let context_summary = if augmented_data.context.len() > 3 {
            augmented_data.context[..3].join(". ") + "..."
        } else {
            augmented_data.context.join(". ")
        };

        format!(
            "You are a cryptocurrency analyst. Analyze this data and provide insights in EXACTLY this format:

SYMBOL: {}
CURRENT_PRICE: ${:.2}
CONTEXT: {}

Provide your analysis in this exact format:
INSIGHT: [Your detailed analysis here, max 200 words]
CONFIDENCE: [0.0-1.0, based on data quality and market conditions]
RECOMMENDATION: [BUY/SELL/HOLD - one word only]
PROCESSED_PRICE: [adjusted price prediction based on your analysis, as a number only]

Be concise but informative. Base your analysis on the provided data and context.",
            augmented_data.raw_data.symbol,
            augmented_data.raw_data.price_usd,
            context_summary
        )
    }

    fn parse_gemini_response(&self, response_text: &str, raw_data: super::fetcher::RawData) -> Result<Analysis, Box<dyn Error>> {
        let response = response_text.trim();

        // Extract insight (simple approach)
        let insight_start = response.find("INSIGHT:").unwrap_or(0) + 8;
        let insight_end = response.find("CONFIDENCE:").unwrap_or(response.len());
        let insight = response[insight_start..insight_end].trim().to_string();
        let insight = if insight.is_empty() { "Analysis completed".to_string() } else { insight };

        // Extract confidence
        let confidence_re = Regex::new(r"CONFIDENCE:\s*([0-9]*\.?[0-9]+)")?;
        let confidence: f32 = confidence_re.captures(response)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0.7);

        // Extract recommendation
        let recommendation_re = Regex::new(r"RECOMMENDATION:\s*(BUY|SELL|HOLD)")?;
        let recommendation = recommendation_re.captures(response)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "HOLD".to_string());

        // Extract processed price
        let processed_price_re = Regex::new(r"PROCESSED_PRICE:\s*([0-9]*\.?[0-9]+)")?;
        let processed_price: f64 = processed_price_re.captures(response)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(raw_data.price_usd);

        // Validate confidence range
        let confidence = confidence.max(0.0).min(1.0);

        Ok(Analysis {
            raw_data: raw_data.clone(),
            insight: insight.chars().take(500).collect(), // Limit insight length
            processed_price,
            confidence,
            recommendation,
        })
    }
}
