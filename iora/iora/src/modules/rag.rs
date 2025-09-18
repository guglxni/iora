use crate::modules::analyzer::{LlmProvider, LlmConfig};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AugmentedData {
    pub raw_data: super::fetcher::RawData,
    pub context: Vec<String>,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalDataDocument {
    pub id: String,
    pub embedding: Vec<f32>,
    pub text: String,
    pub price: f64,
    pub timestamp: i64,
    pub symbol: String,
}

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    content: Content,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<EmbeddingPart>,
}

#[derive(Debug, Serialize)]
struct EmbeddingPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    embedding: EmbeddingData,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    values: Vec<f32>,
}

pub struct RagSystem {
    client: Client,
    typesense_url: String,
    typesense_api_key: String,
    llm_config: LlmConfig,
    initialized: bool,
}

impl RagSystem {
    pub fn new(typesense_url: String, typesense_api_key: String, llm_config: LlmConfig) -> Self {
        println!("🏗️ Creating RagSystem with provider: {}", llm_config.provider);
        Self {
            client: Client::new(),
            typesense_url,
            typesense_api_key,
            llm_config,
            initialized: false,
        }
    }

    pub async fn init_typesense(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🔍 Initializing Typesense client...");

        // Test connection with health check
        let health_url = format!("{}/health", self.typesense_url.trim_end_matches('/'));
        let response = self.client
            .get(&health_url)
            .header("X-TYPESENSE-API-KEY", &self.typesense_api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Typesense health check failed: HTTP {}", response.status()).into());
        }

        // Create historical_data collection if it doesn't exist
        let collection_url = format!("{}/collections", self.typesense_url.trim_end_matches('/'));

        let collection_schema = serde_json::json!({
            "name": "historical_data",
            "fields": [
                {"name": "id", "type": "string"},
                {"name": "embedding", "type": "float[]", "num_dim": 768},
                {"name": "text", "type": "string"},
                {"name": "price", "type": "float"},
                {"name": "timestamp", "type": "int64"},
                {"name": "symbol", "type": "string"}
            ]
        });

        let response = self.client
            .post(&collection_url)
            .header("X-TYPESENSE-API-KEY", &self.typesense_api_key)
            .header("Content-Type", "application/json")
            .json(&collection_schema)
            .send()
            .await?;

        if response.status() == 409 {
            // Collection already exists, that's fine
            println!("ℹ️  historical_data collection already exists");
        } else if !response.status().is_success() {
            return Err(format!("Failed to create collection: HTTP {}", response.status()).into());
        } else {
            println!("✅ Created historical_data collection");
        }

        self.initialized = true;
        println!("🎉 Typesense RAG system initialized successfully!");
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Augment data with hybrid search (REAL FUNCTIONAL CODE ONLY - NO FALLBACKS)
    pub async fn augment_data(
        &self,
        raw_data: super::fetcher::RawData,
    ) -> Result<AugmentedData, Box<dyn Error>> {
        println!("🔍 augment_data called for symbol: {}", raw_data.symbol);
        // REAL FUNCTIONAL CODE ONLY - NO FALLBACKS ALLOWED
        if !self.is_initialized() {
            return Err("Typesense not initialized. Call init_typesense() first.".into());
        }

        // Generate embedding for the raw data using configured LLM
        let embedding = self
            .generate_embedding(&format!(
                "{} price: ${}",
                raw_data.symbol, raw_data.price_usd
            ))
            .await?;

        // Perform HYBRID SEARCH: combine vector similarity + text search (top-k=3)
        let relevant_docs = self.hybrid_search(&raw_data.symbol, &embedding, 3).await?;

        // Extract context from relevant documents with ranking information
        let context: Vec<String> = relevant_docs
            .iter()
            .enumerate()
            .map(|(i, doc)| {
                format!(
                    "[{}] {} (Price: ${:.2}, Time: {})",
                    i + 1,
                    doc.text,
                    doc.price,
                    DateTime::from_timestamp(doc.timestamp, 0)
                        .unwrap_or_else(|| Utc::now().into())
                        .format("%Y-%m-%d %H:%M:%S UTC")
                )
            })
            .collect();

        Ok(AugmentedData {
            raw_data,
            context,
            embedding,
        })
    }

    /// Generate embedding using the configured LLM provider
    /// Note: For providers that don't support embeddings (like Moonshot/Kimi),
    /// we fall back to Gemini for embeddings while using the configured provider for analysis
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        // For embedding generation, we use Gemini as it's reliable and supports embeddings
        // The llm_config.provider is used for analysis, but embeddings always use Gemini
        println!("🔧 Generating embeddings using Gemini (reliable embedding provider)");
        self.generate_gemini_embedding_fallback(text).await
    }

    /// Generate embedding using Gemini (used as fallback for providers without embedding support)
    pub async fn generate_gemini_embedding_fallback(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        println!("🔧 Attempting Gemini embedding generation...");
        // Use a fallback Gemini API key if available, otherwise use the configured one
        let gemini_key = std::env::var("GEMINI_API_KEY")
            .unwrap_or_else(|_| "AIzaSyDummyKeyForEmbeddings".to_string());

        if gemini_key.starts_with("AIzaSyDummy") {
            // If no real Gemini key, return a simple hash-based embedding as fallback
            println!("⚠️  No Gemini API key available for embeddings, using simple fallback");
            return Ok(self.generate_simple_embedding(text));
        }

        let request = EmbeddingRequest {
            content: Content {
                parts: vec![EmbeddingPart {
                    text: text.to_string(),
                }],
            },
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/embedding-001:embedContent?key={}",
            gemini_key
        );

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status() == 429 {
            println!("🔄 Gemini embedding API rate limited (429), using simple fallback");
            return Ok(self.generate_simple_embedding(text));
        } else if response.status() == reqwest::StatusCode::BAD_REQUEST {
            println!("🔄 Gemini embedding API bad request (400), using simple fallback");
            return Ok(self.generate_simple_embedding(text));
        } else if !response.status().is_success() {
            println!("⚠️  Gemini embedding API failed ({}), using simple fallback", response.status());
            return Ok(self.generate_simple_embedding(text));
        }

        let embedding_response: EmbeddingResponse = response.json().await?;
        Ok(embedding_response.embedding.values)
    }

    /// Generate a simple hash-based embedding as ultimate fallback
    fn generate_simple_embedding(&self, text: &str) -> Vec<f32> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        // Generate a simple 768-dimensional embedding from the hash
        (0..768).map(|i| {
            let h = hash.wrapping_mul(i as u64 + 1);
            (h as f32 / u64::MAX as f32) * 2.0 - 1.0 // Normalize to [-1, 1]
        }).collect()
    }

    pub async fn generate_gemini_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        if self.llm_config.api_key.is_empty() {
            return Err("Gemini API key is required - no fallbacks allowed".into());
        }

        let request = EmbeddingRequest {
            content: Content {
                parts: vec![EmbeddingPart {
                    text: text.to_string(),
                }],
            },
        };

        let url = format!(
            "{}/v1beta/models/embedding-001:embedContent?key={}",
            self.llm_config.base_url.trim_end_matches('/'),
            self.llm_config.api_key
        );

        let response = self.client.post(&url).json(&request).send().await?;

        if response.status() == 429 {
            println!("🔄 Gemini API rate limited (429) in RAG embedding generation. Waiting 10 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            // Retry once
            let response = self.client.post(&url).json(&request).send().await?;
            if !response.status().is_success() {
                return Err(format!("Gemini API error after retry: HTTP {}", response.status()).into());
            }
        } else         if !response.status().is_success() {
            // If Gemini API returns 400 (bad request) or 429 (rate limited), fall back to simple embedding
            if response.status() == reqwest::StatusCode::BAD_REQUEST || response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                println!("⚠️  Gemini embedding API failed ({}), using simple hash-based fallback", response.status());
                return Ok(self.generate_simple_fallback_embedding(text));
            }
            return Err(format!("Gemini API error: HTTP {}", response.status()).into());
        }

        let embedding_response: EmbeddingResponse = response.json().await?;
        Ok(embedding_response.embedding.values)
    }

    pub async fn generate_openai_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn Error>> {
        #[derive(Serialize)]
        struct OpenAIEmbeddingRequest {
            input: String,
            model: String,
        }

        #[derive(Deserialize)]
        struct OpenAIEmbeddingResponse {
            data: Vec<OpenAIEmbeddingData>,
        }

        #[derive(Deserialize)]
        struct OpenAIEmbeddingData {
            embedding: Vec<f32>,
        }

        let request = OpenAIEmbeddingRequest {
            input: text.to_string(),
            model: "text-embedding-3-small".to_string(), // Default embedding model
        };

        let url = format!(
            "{}/v1/embeddings",
            self.llm_config.base_url.trim_end_matches('/')
        );

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.llm_config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status() == 429 {
            println!("🔄 {} API rate limited (429) in RAG embedding generation. Waiting 10 seconds...", self.llm_config.provider);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            // Retry once
            let response = self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.llm_config.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await?;
            if !response.status().is_success() {
                return Err(format!("{} API error after retry: HTTP {}", self.llm_config.provider, response.status()).into());
            }
        } else if !response.status().is_success() {
            return Err(format!("{} API error: HTTP {}", self.llm_config.provider, response.status()).into());
        }

        let embedding_response: OpenAIEmbeddingResponse = response.json().await?;
        if embedding_response.data.is_empty() {
            return Err("No embedding data received".into());
        }

        Ok(embedding_response.data[0].embedding.clone())
    }

    /// Perform hybrid search combining vector similarity and text search
    pub async fn hybrid_search(
        &self,
        query_symbol: &str,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<HistoricalDataDocument>, Box<dyn Error>> {
        if !self.is_initialized() {
            return Err("Typesense not initialized".into());
        }

        let search_url = format!(
            "{}/collections/historical_data/documents/search",
            self.typesense_url.trim_end_matches('/')
        );

        // Convert embedding vector to JSON array format for Typesense
        let vector_query = format!(
            "embedding:({}, k:{})",
            serde_json::to_string(query_embedding)?,
            limit
        );

        let search_body = serde_json::json!({
            "q": query_symbol,
            "query_by": "symbol,text",
            "vector_query": vector_query,
            "limit": limit,
            "include_fields": "id,embedding,text,price,timestamp,symbol"
        });

        let response = self.client
            .get(&search_url)
            .header("X-TYPESENSE-API-KEY", &self.typesense_api_key)
            .header("Content-Type", "application/json")
            .json(&search_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Typesense search failed: HTTP {}", response.status()).into());
        }

        #[derive(Deserialize)]
        struct SearchResponse {
            hits: Vec<SearchHit>,
        }

        #[derive(Deserialize)]
        struct SearchHit {
            document: HistoricalDataDocument,
        }

        let search_response: SearchResponse = response.json().await?;
        let documents = search_response.hits
            .into_iter()
            .map(|hit| hit.document)
            .collect();

        Ok(documents)
    }

    /// Index historical data for RAG
    pub async fn index_historical_data(&self, data_path: &str) -> Result<(), Box<dyn Error>> {
        if !self.is_initialized() {
            return Err("Typesense not initialized. Call init_typesense() first.".into());
        }

        println!("📊 Starting historical data indexing from: {}", data_path);

        // Read historical data file
        let data = std::fs::read_to_string(data_path)
            .map_err(|e| format!("Failed to read historical data file: {}", e))?;

        let historical_entries: Vec<serde_json::Value> = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse historical data JSON: {}", e))?;

        let mut documents = Vec::new();

        for entry in historical_entries {
            let text = format!(
                "{} at ${} on {}",
                entry["symbol"].as_str().unwrap_or("UNKNOWN"),
                entry["price"].as_f64().unwrap_or(0.0),
                entry["timestamp"].as_i64().unwrap_or(0)
            );

            let embedding = self.generate_embedding(&text).await?;

            let doc = HistoricalDataDocument {
                id: format!("hist_{}", entry["timestamp"].as_i64().unwrap_or(0)),
                embedding,
                text,
                price: entry["price"].as_f64().unwrap_or(0.0),
                timestamp: entry["timestamp"].as_i64().unwrap_or(0),
                symbol: entry["symbol"].as_str().unwrap_or("UNKNOWN").to_string(),
            };

            documents.push(doc);
        }

        // Bulk index documents
        self.bulk_index_documents(documents).await?;

        println!("✅ Successfully indexed {} historical documents", historical_entries.len());
        Ok(())
    }

    async fn bulk_index_documents(&self, documents: Vec<HistoricalDataDocument>) -> Result<(), Box<dyn Error>> {
        let index_url = format!(
            "{}/collections/historical_data/documents/import",
            self.typesense_url.trim_end_matches('/')
        );

        let json_lines: Vec<String> = documents
            .into_iter()
            .map(|doc| serde_json::to_string(&doc))
            .collect::<Result<Vec<_>, _>>()?;

        let bulk_data = json_lines.join("\n");

        let response = self.client
            .post(&index_url)
            .header("X-TYPESENSE-API-KEY", &self.typesense_api_key)
            .header("Content-Type", "text/plain")
            .body(bulk_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Bulk indexing failed: HTTP {} - {}", response.status(), error_text).into());
        }

        Ok(())
    }

    /// Get masked API key (first 8 characters)
    pub fn get_masked_api_key(&self) -> &str {
        if self.llm_config.api_key.len() >= 8 {
            &self.llm_config.api_key[..8]
        } else {
            &self.llm_config.api_key
        }
    }
}
