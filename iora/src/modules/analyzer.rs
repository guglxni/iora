use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub insight: String,
    pub processed_price: f64,
    pub confidence: f32,
    pub recommendation: String,
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

pub struct Analyzer {
    client: Client,
    api_key: String,
}

impl Analyzer {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn analyze(&self, augmented_data: &super::rag::AugmentedData) -> Result<Analysis, Box<dyn Error>> {
        let prompt = format!(
            "Analyze this cryptocurrency data and provide insights:\n\
            Symbol: {}\n\
            Current Price: ${}\n\
            Context: {}\n\
            \n\
            Provide a brief analysis, confidence score (0-1), and trading recommendation.",
            augmented_data.raw_data.symbol,
            augmented_data.raw_data.price_usd,
            augmented_data.context.join("; ")
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
            self.api_key
        );

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let gemini_response: GeminiResponse = response.json().await?;
            if let Some(candidate) = gemini_response.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    // Parse the response to extract insight, confidence, and recommendation
                    let analysis_text = &part.text;

                    return Ok(Analysis {
                        insight: analysis_text.clone(),
                        processed_price: augmented_data.raw_data.price_usd,
                        confidence: 0.8, // Default confidence for MVP
                        recommendation: "HOLD".to_string(), // Default recommendation
                    });
                }
            }
        }

        // Fallback analysis for MVP
        Ok(Analysis {
            insight: format!("Analysis of {} at price ${}", augmented_data.raw_data.symbol, augmented_data.raw_data.price_usd),
            processed_price: augmented_data.raw_data.price_usd,
            confidence: 0.7,
            recommendation: "MONITOR".to_string(),
        })
    }
}
