use anyhow::*;
use reqwest::Client;
use serde_json::json;
use crate::modules::llm::{LlmOutput, prompt};

pub async fn call(prompt_text: &str) -> Result<LlmOutput> {
    let api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable not set")?;
    let base = std::env::var("GEMINI_BASE")
        .unwrap_or_else(|_| "https://generativelanguage.googleapis.com".into());
    let model = std::env::var("GEMINI_MODEL")
        .unwrap_or_else(|_| "gemini-1.5-flash".into());
    let url = format!("{base}/v1beta/models/{model}:generateContent?key={api_key}");

    let cli = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let body = json!({
      "contents": [{
        "parts": [{
          "text": format!("{}\n\n{}", prompt::SYSTEM_JSON_SCHEMA, prompt_text)
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
        .ok_or_else(|| anyhow!("unexpected Gemini response: {:?}", res))?;

    parse_structured_json(text)
}

fn parse_structured_json(text: &str) -> Result<LlmOutput> {
    // Strict JSON enforcement - reject prose responses
    let trimmed = text.trim();

    // Must start and end with braces (JSON object)
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(anyhow!("Gemini returned invalid format (not JSON object): {}", trimmed));
    }

    // Parse JSON strictly
    let v: serde_json::Value = serde_json::from_str(trimmed)
        .with_context(|| format!("Gemini returned invalid JSON: {}", trimmed))?;

    // Validate required structure
    let summary = v["summary"].as_str()
        .ok_or_else(|| anyhow!("Gemini JSON missing required 'summary' field"))?;
    let signals = v["signals"].as_array()
        .ok_or_else(|| anyhow!("Gemini JSON missing required 'signals' array"))?;
    let confidence = v["confidence"].as_f64()
        .ok_or_else(|| anyhow!("Gemini JSON missing required 'confidence' number"))?;
    let sources = v["sources"].as_array()
        .ok_or_else(|| anyhow!("Gemini JSON missing required 'sources' array"))?;

    // Validate confidence range
    if confidence < 0.0 || confidence > 1.0 {
        return Err(anyhow!("Gemini confidence {} out of range [0,1]", confidence));
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
