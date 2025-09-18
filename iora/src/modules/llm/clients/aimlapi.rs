use anyhow::*;
use reqwest::Client;
use serde_json::json;
use crate::modules::llm::{LlmOutput, prompt};

pub async fn call(prompt_text: &str) -> Result<LlmOutput> {
    let api_key = std::env::var("AIMLAPI_API_KEY")
        .context("AIMLAPI_API_KEY environment variable not set")?;
    let base = std::env::var("AIMLAPI_BASE")
        .unwrap_or_else(|_| "https://api.aimlapi.com".into());
    let model = std::env::var("AIMLAPI_MODEL")
        .unwrap_or_else(|_| "llama-3.1-70b-instruct".into());
    let url = format!("{base}/chat/completions");

    let cli = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    let body = json!({
      "model": model,
      "messages":[
        {"role":"system","content": prompt::SYSTEM_JSON_SCHEMA},
        {"role":"user","content": prompt_text}
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
        .ok_or_else(|| anyhow!("unexpected AIML API response: {:?}", res))?;

    parse_structured_json(text)
}

fn parse_structured_json(text: &str) -> Result<LlmOutput> {
    // Expect the model to return strict JSON per SYSTEM_JSON_SCHEMA
    let v: serde_json::Value = serde_json::from_str(text)
        .with_context(|| format!("AIML API did not return JSON: {}", text))?;

    Ok(LlmOutput{
        summary: v["summary"].as_str().unwrap_or_default().to_string(),
        signals: v["signals"].as_array().unwrap_or(&vec![]).iter()
                  .filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
        confidence: v["confidence"].as_f64().unwrap_or(0.5) as f32,
        sources: v["sources"].as_array().unwrap_or(&vec![]).iter()
                  .filter_map(|x| x.as_str().map(|s| s.to_string())).collect(),
    })
}
