use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawData {
    pub price: f64,
    pub timestamp: u64,
    pub symbol: String,
}

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_crypto_price(&self, symbol: &str) -> Result<RawData, Box<dyn Error>> {
        let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", symbol);

        let response = self.client.get(&url).send().await?;
        let data: serde_json::Value = response.json().await?;

        if let Some(price_data) = data.get(symbol) {
            if let Some(price) = price_data.get("usd") {
                return Ok(RawData {
                    price: price.as_f64().unwrap_or(0.0),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs(),
                    symbol: symbol.to_string(),
                });
            }
        }

        Err("Failed to fetch price data".into())
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self::new()
    }
}
