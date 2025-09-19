//! Multi-API Crypto Data Fetching Module with RAG Intelligence
//!
//! This module provides a unified interface for fetching cryptocurrency data from multiple
//! APIs simultaneously, with intelligent routing, BYOK support, and performance optimization.

use crate::modules::analytics::{AnalyticsConfig, AnalyticsManager};
use crate::modules::cache::{CacheConfig, CacheWarmer, IntelligentCache};
use crate::modules::health::{HealthConfig, HealthMonitor};
use crate::modules::historical::{HistoricalDataManager, TimeSeriesConfig, TimeSeriesPoint};
use crate::modules::processor::{DataProcessor, NormalizedData, ProcessingConfig};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Core data structures for cryptocurrency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    pub symbol: String,
    pub name: String,
    pub price_usd: f64,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub source: ApiProvider,
}

/// Comprehensive price analysis from multiple APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAnalysis {
    pub symbol: String,
    pub average_price: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub price_spread: f64,
    pub consensus_price: f64,
    pub api_count: u32,
    pub fastest_response_time: Duration,
    pub sources: Vec<ApiProvider>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Individual source data for multi-source analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceData {
    pub provider: ApiProvider,
    pub price_usd: f64,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub response_time: Duration,
}

/// Comprehensive multi-source price analysis with detailed breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSourceAnalysis {
    pub symbol: String,
    pub consensus_price: f64,
    pub average_price: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub price_spread: f64,
    pub sources_used: usize,
    pub total_sources: usize,
    pub fastest_response_time: Duration,
    pub confidence_score: f64,
    pub source_breakdown: Vec<SourceData>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price_usd: f64,
    pub volume_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub source: ApiProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalData {
    pub symbol: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
    pub source: ApiProvider,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMarketData {
    pub total_market_cap_usd: f64,
    pub total_volume_24h_usd: f64,
    pub market_cap_change_percentage_24h: f64,
    pub active_cryptocurrencies: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub source: ApiProvider,
}

/// Enumeration of supported API providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiProvider {
    CoinPaprika,   // Free, no API key required
    CoinGecko,     // Free tier + paid options
    CoinMarketCap, // Paid API with comprehensive data
    CryptoCompare, // Real-time data with paid tiers
}

/// Context information for intelligent API selection
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub data_type: DataType,
    pub priority: Priority,
    pub max_budget: Option<f64>,
    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    RealTimePrice,
    HistoricalData,
    GlobalMarket,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Speed,
    Cost,
    Reliability,
    Balanced,
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            data_type: DataType::RealTimePrice,
            priority: Priority::Balanced,
            max_budget: None,
            timeout: Duration::from_secs(30),
        }
    }
}

impl std::fmt::Display for ApiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiProvider::CoinPaprika => write!(f, "CoinPaprika"),
            ApiProvider::CoinGecko => write!(f, "CoinGecko"),
            ApiProvider::CoinMarketCap => write!(f, "CoinMarketCap"),
            ApiProvider::CryptoCompare => write!(f, "CryptoCompare"),
        }
    }
}

/// API-specific configuration for BYOK (Bring Your Own Key) support
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub provider: ApiProvider,
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub rate_limit: u32, // requests per minute
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self::coinpaprika_default()
    }
}

impl ApiConfig {
    pub fn coinpaprika_default() -> Self {
        Self {
            provider: ApiProvider::CoinPaprika,
            enabled: true,
            api_key: None, // No API key required
            base_url: "https://api.coinpaprika.com/v1".to_string(),
            rate_limit: 1000, // Very generous free tier
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }

    pub fn coingecko_default() -> Self {
        Self {
            provider: ApiProvider::CoinGecko,
            enabled: std::env::var("COINGECKO_API_KEY").is_ok(),
            api_key: std::env::var("COINGECKO_API_KEY").ok(),
            base_url: "https://api.coingecko.com/api/v3".to_string(),
            rate_limit: 30, // Free tier limit
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }

    pub fn coinmarketcap_default() -> Self {
        Self {
            provider: ApiProvider::CoinMarketCap,
            enabled: std::env::var("COINMARKETCAP_API_KEY").is_ok(),
            api_key: std::env::var("COINMARKETCAP_API_KEY").ok(),
            base_url: "https://pro-api.coinmarketcap.com/v1".to_string(),
            rate_limit: 10000, // Paid tier limit
            timeout_seconds: 30,
            retry_attempts: 3,
        }
    }

    pub fn cryptocompare_default() -> Self {
        Self {
            provider: ApiProvider::CryptoCompare,
            enabled: std::env::var("CRYPTOCOMPARE_API_KEY").is_ok(),
            api_key: std::env::var("CRYPTOCOMPARE_API_KEY").ok(),
            base_url: "https://min-api.cryptocompare.com/data".to_string(),
            rate_limit: 1000, // Paid tier limit
            timeout_seconds: 15,
            retry_attempts: 3,
        }
    }

    pub fn is_configured(&self) -> bool {
        match self.provider {
            ApiProvider::CoinPaprika => true, // No key required
            _ => self.api_key.is_some(),
        }
    }
}

/// Performance metrics for RAG learning and intelligent routing
#[derive(Debug, Clone)]
pub struct ApiMetrics {
    pub provider: ApiProvider,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: Duration,
    pub last_request_time: Option<Instant>,
    pub consecutive_failures: u32,
    pub circuit_breaker_tripped: bool,
    pub cost_per_request: f64, // For paid APIs
}

impl ApiMetrics {
    pub fn new(provider: ApiProvider) -> Self {
        let cost_per_request = match provider {
            ApiProvider::CoinPaprika => 0.0,
            ApiProvider::CoinGecko => 0.0,         // Free tier
            ApiProvider::CoinMarketCap => 0.0001,  // Example paid rate
            ApiProvider::CryptoCompare => 0.00005, // Example paid rate
        };

        Self {
            provider,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: Duration::from_millis(500),
            last_request_time: None,
            consecutive_failures: 0,
            circuit_breaker_tripped: false,
            cost_per_request,
        }
    }

    pub fn record_success(&mut self, response_time: Duration) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.consecutive_failures = 0;
        self.last_request_time = Some(Instant::now());

        // Update rolling average response time
        let current_avg = self.average_response_time.as_millis() as f64;
        let new_time = response_time.as_millis() as f64;
        let new_avg = (current_avg * 0.9) + (new_time * 0.1); // Weighted average
        self.average_response_time = Duration::from_millis(new_avg as u64);
    }

    pub fn record_failure(&mut self) {
        self.total_requests += 1;
        self.failed_requests += 1;
        self.consecutive_failures += 1;
        self.last_request_time = Some(Instant::now());

        // Trip circuit breaker after 5 consecutive failures
        if self.consecutive_failures >= 5 {
            self.circuit_breaker_tripped = true;
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }

    pub fn is_healthy(&self) -> bool {
        !self.circuit_breaker_tripped && self.consecutive_failures < 3
        // Note: After circuit breaker reset, we give the API another chance
        // regardless of historical success rate
    }

    pub fn reset_circuit_breaker(&mut self) {
        self.circuit_breaker_tripped = false;
        self.consecutive_failures = 0;
    }
}

/// Unified trait that all crypto APIs must implement
#[async_trait::async_trait]
pub trait CryptoApi: Send + Sync {
    /// Get the API provider identifier
    fn provider(&self) -> ApiProvider;

    /// Get current price for a cryptocurrency
    async fn get_price(&self, symbol: &str) -> Result<PriceData, ApiError>;

    /// Get historical price data
    async fn get_historical_data(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError>;

    /// Get global market statistics
    async fn get_global_market_data(&self) -> Result<GlobalMarketData, ApiError>;

    /// Check if the API is currently available and configured
    async fn is_available(&self) -> bool;

    /// Get the API's rate limit status
    fn rate_limit(&self) -> u32;

    /// Get the API's current configuration
    fn config(&self) -> &ApiConfig;
}

/// Comprehensive error types for API operations
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API rate limit exceeded for {0}")]
    RateLimitExceeded(ApiProvider),

    #[error("Invalid API key for {0}")]
    InvalidApiKey(ApiProvider),

    #[error("API endpoint not found: {0}")]
    NotFound(String),

    #[error("API returned error: {0}")]
    ApiError(String),

    #[error("Timeout exceeded for {0}")]
    Timeout(ApiProvider),

    #[error("Circuit breaker tripped for {0}")]
    CircuitBreaker(ApiProvider),

    #[error("Unsupported operation for {0}")]
    UnsupportedOperation(ApiProvider),

    #[error("Network connectivity error: {0}")]
    NetworkError(String),

    #[error("Rate limit exceeded for {0}")]
    RateLimit(ApiProvider),

    #[error("Unauthorized access to {0}")]
    Unauthorized(ApiProvider),

    #[error("Forbidden access to {0}")]
    Forbidden(ApiProvider),

    #[error("Server error from {0}")]
    ServerError(ApiProvider),

    #[error("Parse error for {0}")]
    ParseError(ApiProvider),

    #[error("Circuit breaker is open for {0}")]
    CircuitBreakerOpen(ApiProvider),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Intelligent multi-API client for concurrent data fetching
pub struct MultiApiClient {
    pub(crate) apis: HashMap<ApiProvider, Box<dyn CryptoApi>>,
    metrics: tokio::sync::RwLock<HashMap<ApiProvider, ApiMetrics>>,
    router: ApiRouter,
    resilience_manager: ResilienceManager,
    cache: Option<Arc<IntelligentCache>>,
    cache_warmer: Option<Arc<CacheWarmer>>,
    processor: Option<Arc<DataProcessor>>,
    historical_manager: Option<Arc<HistoricalDataManager>>,
    pub(crate) analytics_manager: Option<Arc<AnalyticsManager>>,
    health_monitor: Option<Arc<HealthMonitor>>,
}

/// Intelligent API router for RAG-powered selection
#[derive(Debug, Clone)]
pub struct ApiRouter {
    routing_strategy: RoutingStrategy,
    cost_optimization: bool,
    real_time_priority: bool,
    fallback_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingStrategy {
    /// Select fastest API based on recent performance
    Fastest,
    /// Select cheapest API based on cost metrics
    Cheapest,
    /// Select most reliable API based on success rate
    MostReliable,
    /// Use all available APIs simultaneously (race condition)
    RaceCondition,
    /// Load balance across all healthy APIs
    LoadBalanced,
    /// Context-aware selection based on data type and requirements
    ContextAware,
}

impl ApiRouter {
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            routing_strategy: strategy,
            cost_optimization: true,
            real_time_priority: true,
            fallback_enabled: true,
        }
    }

    pub fn with_cost_optimization(mut self, enabled: bool) -> Self {
        self.cost_optimization = enabled;
        self
    }

    pub fn with_real_time_priority(mut self, enabled: bool) -> Self {
        self.real_time_priority = enabled;
        self
    }

    pub fn with_fallback(mut self, enabled: bool) -> Self {
        self.fallback_enabled = enabled;
        self
    }

    /// Select the best API based on current routing strategy and metrics
    pub async fn select_api(
        &self,
        available_apis: &HashMap<ApiProvider, Box<dyn CryptoApi>>,
        metrics: &HashMap<ApiProvider, ApiMetrics>,
        context: &RequestContext,
    ) -> Option<ApiProvider> {
        let healthy_apis: Vec<ApiProvider> = available_apis
            .keys()
            .filter(|provider| {
                metrics
                    .get(provider)
                    .map(|m| m.is_healthy())
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        if healthy_apis.is_empty() {
            return None;
        }

        match self.routing_strategy {
            RoutingStrategy::Fastest => self.select_fastest_api(&healthy_apis, metrics),
            RoutingStrategy::Cheapest => self.select_cheapest_api(&healthy_apis, metrics, context),
            RoutingStrategy::MostReliable => self.select_most_reliable_api(&healthy_apis, metrics),
            RoutingStrategy::RaceCondition => None, // Race condition uses all APIs
            RoutingStrategy::LoadBalanced => self.select_load_balanced_api(&healthy_apis, metrics),
            RoutingStrategy::ContextAware => {
                self.select_context_aware_api(&healthy_apis, metrics, context)
            }
        }
    }

    fn select_fastest_api(
        &self,
        apis: &[ApiProvider],
        metrics: &HashMap<ApiProvider, ApiMetrics>,
    ) -> Option<ApiProvider> {
        apis.iter()
            .min_by(|a, b| {
                let time_a = metrics
                    .get(a)
                    .map(|m| m.average_response_time)
                    .unwrap_or(Duration::from_secs(10));
                let time_b = metrics
                    .get(b)
                    .map(|m| m.average_response_time)
                    .unwrap_or(Duration::from_secs(10));
                time_a.cmp(&time_b)
            })
            .cloned()
    }

    fn select_cheapest_api(
        &self,
        apis: &[ApiProvider],
        metrics: &HashMap<ApiProvider, ApiMetrics>,
        _context: &RequestContext,
    ) -> Option<ApiProvider> {
        if !self.cost_optimization {
            return apis.first().cloned();
        }

        apis.iter()
            .min_by(|a, b| {
                let cost_a = metrics.get(a).map(|m| m.cost_per_request).unwrap_or(0.0);
                let cost_b = metrics.get(b).map(|m| m.cost_per_request).unwrap_or(0.0);
                cost_a
                    .partial_cmp(&cost_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    fn select_most_reliable_api(
        &self,
        apis: &[ApiProvider],
        metrics: &HashMap<ApiProvider, ApiMetrics>,
    ) -> Option<ApiProvider> {
        apis.iter()
            .max_by(|a, b| {
                let rate_a = metrics.get(a).map(|m| m.success_rate()).unwrap_or(0.0);
                let rate_b = metrics.get(b).map(|m| m.success_rate()).unwrap_or(0.0);
                rate_a
                    .partial_cmp(&rate_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    fn select_load_balanced_api(
        &self,
        apis: &[ApiProvider],
        metrics: &HashMap<ApiProvider, ApiMetrics>,
    ) -> Option<ApiProvider> {
        // Simple round-robin based on request count
        apis.iter()
            .min_by(|a, b| {
                let count_a = metrics.get(a).map(|m| m.total_requests).unwrap_or(0);
                let count_b = metrics.get(b).map(|m| m.total_requests).unwrap_or(0);
                count_a.cmp(&count_b)
            })
            .cloned()
    }

    fn select_context_aware_api(
        &self,
        apis: &[ApiProvider],
        metrics: &HashMap<ApiProvider, ApiMetrics>,
        context: &RequestContext,
    ) -> Option<ApiProvider> {
        match context.data_type {
            DataType::RealTimePrice => {
                // For real-time data, prioritize speed and low latency
                if self.real_time_priority {
                    self.select_fastest_api(apis, metrics)
                } else {
                    self.select_most_reliable_api(apis, metrics)
                }
            }
            DataType::HistoricalData => {
                // For historical data, prioritize reliability and cost
                if self.cost_optimization {
                    self.select_cheapest_api(apis, metrics, context)
                } else {
                    self.select_most_reliable_api(apis, metrics)
                }
            }
            DataType::GlobalMarket => {
                // For global data, prioritize comprehensive sources
                if let Some(prioritized) = apis.iter().find(|provider| {
                    matches!(
                        provider,
                        ApiProvider::CoinGecko | ApiProvider::CoinMarketCap
                    )
                }) {
                    Some(*prioritized)
                } else {
                    self.select_most_reliable_api(apis, metrics)
                }
            }
        }
    }
}

impl MultiApiClient {
    pub fn new() -> Self {
        Self {
            apis: HashMap::new(),
            metrics: tokio::sync::RwLock::new(HashMap::new()),
            router: ApiRouter::new(RoutingStrategy::ContextAware),
            resilience_manager: ResilienceManager::default(),
            cache: None,
            cache_warmer: None,
            processor: None,
            historical_manager: None,
            analytics_manager: None,
            health_monitor: None,
        }
    }

    pub fn with_resilience_config(mut self, config: ResilienceConfig) -> Self {
        self.resilience_manager = ResilienceManager::new(config);
        self
    }

    pub fn with_routing_strategy(mut self, strategy: RoutingStrategy) -> Self {
        self.router.routing_strategy = strategy;
        self
    }

    /// Enable caching with default configuration
    pub fn with_caching(mut self) -> Self {
        let cache = Arc::new(IntelligentCache::default());
        let cache_warmer = Arc::new(CacheWarmer::new(Arc::clone(&cache)));
        self.cache = Some(cache);
        self.cache_warmer = Some(cache_warmer);
        self
    }

    /// Enable caching with custom configuration
    pub fn with_cache_config(mut self, config: CacheConfig) -> Self {
        let cache = Arc::new(IntelligentCache::new(config));
        let cache_warmer = Arc::new(CacheWarmer::new(Arc::clone(&cache)));
        self.cache = Some(cache);
        self.cache_warmer = Some(cache_warmer);
        self
    }

    /// Enable caching with existing cache instance
    pub fn with_cache(mut self, cache: Arc<IntelligentCache>) -> Self {
        let cache_warmer = Arc::new(CacheWarmer::new(Arc::clone(&cache)));
        self.cache = Some(cache);
        self.cache_warmer = Some(cache_warmer);
        self
    }

    /// Enable analytics with default configuration
    pub fn with_analytics(mut self) -> Self {
        let analytics = Arc::new(AnalyticsManager::default());
        self.analytics_manager = Some(analytics);
        self
    }

    /// Enable analytics with custom configuration
    pub fn with_analytics_config(mut self, config: AnalyticsConfig) -> Self {
        let analytics = Arc::new(AnalyticsManager::new(config));
        self.analytics_manager = Some(analytics);
        self
    }

    /// Enable analytics with existing analytics manager
    pub fn with_analytics_manager(mut self, analytics: Arc<AnalyticsManager>) -> Self {
        self.analytics_manager = Some(analytics);
        self
    }

    /// Enable health monitoring with default configuration
    pub fn with_health_monitoring(mut self) -> Self {
        let health_monitor = Arc::new(HealthMonitor::new());
        self.health_monitor = Some(health_monitor);
        self
    }

    /// Enable health monitoring with custom configuration
    pub fn with_health_config(mut self, config: HealthConfig) -> Self {
        let health_monitor = Arc::new(HealthMonitor::with_config(config));
        self.health_monitor = Some(health_monitor);
        self
    }

    /// Enable health monitoring with existing health monitor
    pub fn with_health_monitor(mut self, health_monitor: Arc<HealthMonitor>) -> Self {
        self.health_monitor = Some(health_monitor);
        self
    }

    /// Enable data processing with default configuration
    pub fn with_processing(mut self) -> Self {
        self.processor = Some(Arc::new(DataProcessor::default()));
        self
    }

    /// Enable data processing with custom configuration
    pub fn with_processing_config(mut self, config: ProcessingConfig) -> Self {
        self.processor = Some(Arc::new(DataProcessor::new_with_default_client(config)));
        self
    }

    /// Enable data processing with existing processor instance
    pub fn with_processor(mut self, processor: Arc<DataProcessor>) -> Self {
        self.processor = Some(processor);
        self
    }

    /// Enable historical data management with default configuration
    pub fn with_historical(mut self) -> Self {
        self.historical_manager = Some(Arc::new(HistoricalDataManager::default()));
        self
    }

    /// Enable historical data management with custom configuration
    pub fn with_historical_config(mut self, config: TimeSeriesConfig) -> Self {
        self.historical_manager = Some(Arc::new(HistoricalDataManager::new(config)));
        self
    }

    /// Enable historical data management with existing manager instance
    pub fn with_historical_manager(mut self, manager: Arc<HistoricalDataManager>) -> Self {
        self.historical_manager = Some(manager);
        self
    }

    /// Create a new MultiApiClient with all available API implementations
    pub fn new_with_all_apis() -> Self {
        let mut client = Self::new();

        // Add CoinPaprika (free, always available)
        client.add_api(Box::new(CoinPaprikaApi::new()));

        // Add CoinGecko if API key is available
        if let Ok(coingecko_key) = std::env::var("COINGECKO_API_KEY") {
            if !coingecko_key.is_empty() {
                client.add_api(Box::new(CoinGeckoApi::new()));
            }
        }

        // Add CoinMarketCap if API key is available
        if let Ok(cmc_key) = std::env::var("COINMARKETCAP_API_KEY") {
            if !cmc_key.is_empty() {
                client.add_api(Box::new(CoinMarketCapApi::new()));
            }
        }

        // Add CryptoCompare if API key is available
        if let Ok(cc_key) = std::env::var("CRYPTOCOMPARE_API_KEY") {
            if !cc_key.is_empty() {
                client.add_api(Box::new(CryptoCompareApi::new()));
            }
        }

        client
    }

    /// Add an API implementation to the client
    pub fn add_api(&mut self, api: Box<dyn CryptoApi>) {
        let provider = api.provider();
        self.apis.insert(provider, api);
        // Initialize metrics for this API
        futures::executor::block_on(async {
            let mut metrics = self.metrics.write().await;
            metrics.insert(provider, ApiMetrics::new(provider));
        });
    }

    /// Add specific API implementations
    pub fn add_coinpaprika(&mut self) {
        self.add_api(Box::new(CoinPaprikaApi::new()));
    }

    pub fn add_coingecko(&mut self) {
        self.add_api(Box::new(CoinGeckoApi::new()));
    }

    pub fn add_coinmarketcap(&mut self) {
        self.add_api(Box::new(CoinMarketCapApi::new()));
    }

    pub fn add_cryptocompare(&mut self) {
        self.add_api(Box::new(CryptoCompareApi::new()));
    }

    /// Get price with intelligent routing based on current strategy
    pub async fn get_price_intelligent(&self, symbol: &str) -> Result<PriceData, ApiError> {
        self.get_price_with_context(symbol, &RequestContext::default())
            .await
    }

    /// Get price with intelligent routing and custom context
    pub async fn get_price_with_context(
        &self,
        symbol: &str,
        context: &RequestContext,
    ) -> Result<PriceData, ApiError> {
        // Check cache first if enabled
        if let Some(cache) = &self.cache {
            let cache_key =
                cache.generate_cache_key(&ApiProvider::CoinGecko, "price", Some(symbol));
            if let Some(cached_data) = cache.get(&cache_key).await {
                // Convert RawData back to PriceData
                let price_data = PriceData {
                    symbol: cached_data.symbol,
                    price_usd: cached_data.price_usd,
                    volume_24h: cached_data.volume_24h,
                    market_cap: cached_data.market_cap,
                    price_change_24h: cached_data.price_change_24h,
                    last_updated: cached_data.last_updated,
                    source: cached_data.source,
                };
                return Ok(price_data);
            }
        }

        let metrics = self.metrics.read().await;

        let result = match self.router.routing_strategy {
            RoutingStrategy::RaceCondition => {
                self.execute_race_condition_price_with_resilience(symbol, context)
                    .await
            }
            _ => {
                if let Some(selected_api) =
                    self.router.select_api(&self.apis, &metrics, context).await
                {
                    self.get_price_with_resilience(&selected_api, symbol).await
                } else {
                    Err(ApiError::Unknown("No suitable API available".to_string()))
                }
            }
        };

        // Cache the result if caching is enabled and successful
        if let (Some(cache), Ok(ref price_data)) = (&self.cache, &result) {
            let raw_data = RawData {
                symbol: price_data.symbol.clone(),
                name: price_data.symbol.clone(), // Using symbol as name for simplicity
                price_usd: price_data.price_usd,
                volume_24h: price_data.volume_24h,
                market_cap: price_data.market_cap,
                price_change_24h: price_data.price_change_24h,
                last_updated: price_data.last_updated,
                source: price_data.source,
            };
            let _ = cache
                .put(&price_data.source, "price", Some(symbol), raw_data)
                .await;
        }

        result
    }

    /// Get price with resilience (retry, circuit breaker, timeout)
    async fn get_price_with_resilience(
        &self,
        provider: &ApiProvider,
        symbol: &str,
    ) -> Result<PriceData, ApiError> {
        let api = self
            .apis
            .get(provider)
            .ok_or_else(|| ApiError::UnsupportedOperation(*provider))?;
        let symbol_clone = symbol.to_string();

        self.resilience_manager
            .execute_with_resilience(provider, move || {
                let api = api.as_ref();
                let symbol = symbol_clone.clone();
                async move { api.get_price(&symbol).await }
            })
            .await
    }

    /// Helper method to call price API without closure lifetime issues
    async fn call_single_api_price(
        &self,
        provider: &ApiProvider,
        symbol: &str,
    ) -> Result<PriceData, ApiError> {
        let api = self
            .apis
            .get(provider)
            .ok_or_else(|| ApiError::UnsupportedOperation(*provider))?;

        let start = Instant::now();
        let result = api.get_price(symbol).await;
        let duration = start.elapsed();

        self.record_api_call(*provider, result.is_ok(), duration)
            .await;

        result
    }

    /// Execute race condition: all APIs simultaneously, return fastest result
    /// Execute race condition with resilience: all APIs simultaneously, return fastest result
    async fn execute_race_condition_price_with_resilience(
        &self,
        symbol: &str,
        _context: &RequestContext,
    ) -> Result<PriceData, ApiError> {
        use futures::future::select_ok;

        let tasks: Vec<_> = self
            .apis
            .values()
            .filter(|api| self.is_api_available(api.provider()))
            .map(|api| {
                let provider = api.provider();
                let symbol_clone = symbol.to_string();
                let resilience_manager = self.resilience_manager.clone();

                Box::pin(async move {
                    // Use resilience for each individual API call
                    let result = resilience_manager
                        .execute_with_resilience(&provider, move || {
                            let api = api.as_ref();
                            let symbol = symbol_clone.clone();
                            async move { api.get_price(&symbol).await }
                        })
                        .await;

                    result
                })
                    as std::pin::Pin<
                        Box<dyn std::future::Future<Output = Result<PriceData, ApiError>> + Send>,
                    >
            })
            .collect();

        if tasks.is_empty() {
            return Err(ApiError::Unknown(
                "No available APIs for race condition".to_string(),
            ));
        }

        // Return the first successful result
        let (result, _) = select_ok(tasks)
            .await
            .map_err(|_| ApiError::Unknown("All APIs failed in race condition".to_string()))?;

        Ok(result)
    }

    /// Get price from all APIs and return consensus result
    pub async fn get_price_consensus(&self, symbol: &str) -> Result<PriceData, ApiError> {
        let results = self.execute_parallel_price_requests(symbol).await;

        if results.is_empty() {
            return Err(ApiError::Unknown("No API responses received".to_string()));
        }

        // Calculate consensus price
        let prices: Vec<&PriceData> = results
            .iter()
            .filter_map(|(price_data, _, _)| price_data.as_ref())
            .collect();

        if prices.is_empty() {
            return Err(ApiError::Unknown("All API requests failed".to_string()));
        }

        let consensus_price = utils::calculate_consensus_price(&prices)
            .ok_or_else(|| ApiError::Unknown("Failed to calculate consensus price".to_string()))?;

        // Return the result from the fastest successful API, but with consensus price
        let (fastest_price, _, _) = results
            .into_iter()
            .find(|(price_data, _, _)| price_data.is_some())
            .and_then(|(price_data, duration, provider)| {
                price_data.map(|p| (p, duration, provider))
            })
            .ok_or_else(|| ApiError::Unknown("No successful API responses".to_string()))?;

        Ok(PriceData {
            symbol: fastest_price.symbol,
            price_usd: consensus_price,
            volume_24h: fastest_price.volume_24h,
            market_cap: fastest_price.market_cap,
            price_change_24h: fastest_price.price_change_24h,
            last_updated: fastest_price.last_updated,
            source: fastest_price.source, // Keep original source attribution
        })
    }

    /// Get comprehensive price analysis from all 4 sources simultaneously
    pub async fn get_multi_source_price_analysis(&self, symbol: &str) -> Result<MultiSourceAnalysis, ApiError> {
        let results = self.execute_parallel_price_requests(symbol).await;

        if results.is_empty() {
            return Err(ApiError::Unknown("No API responses received for analysis".to_string()));
        }

        let successful_results: Vec<&PriceData> = results
            .iter()
            .filter_map(|(price_data, _, _)| price_data.as_ref())
            .collect();

        if successful_results.is_empty() {
            return Err(ApiError::Unknown("All API requests failed".to_string()));
        }

        // Calculate statistics
        let prices: Vec<f64> = successful_results.iter().map(|p| p.price_usd).collect();
        let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
        let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let price_spread = max_price - min_price;

        // Find fastest response
        let fastest_response = results
            .iter()
            .filter_map(|(price_data, duration, _)| price_data.as_ref().map(|_| duration))
            .min()
            .copied()
            .unwrap_or(Duration::from_secs(30));

        // Calculate consensus price
        let consensus_price = calculate_consensus_price(&successful_results)
            .unwrap_or(avg_price);

        // Create source breakdown
        let mut source_breakdown = Vec::new();
        for (price_data, duration, provider) in &results {
            if let Some(price) = price_data {
                source_breakdown.push(SourceData {
                    provider: *provider,
                    price_usd: price.price_usd,
                    volume_24h: price.volume_24h,
                    market_cap: price.market_cap,
                    price_change_24h: price.price_change_24h,
                    response_time: *duration,
                });
            }
        }

        // Calculate confidence based on price agreement
        let price_variance = if prices.len() > 1 {
            let variance = prices.iter().map(|p| (p - avg_price).powi(2)).sum::<f64>() / prices.len() as f64;
            variance.sqrt() / avg_price // Coefficient of variation
        } else {
            0.0
        };

        let confidence_score = (1.0 - price_variance.min(0.5) * 2.0).max(0.1); // Convert to 0.1-1.0 scale

        Ok(MultiSourceAnalysis {
            symbol: symbol.to_string(),
            consensus_price,
            average_price: avg_price,
            min_price,
            max_price,
            price_spread,
            sources_used: successful_results.len(),
            total_sources: results.len(),
            fastest_response_time: fastest_response,
            confidence_score,
            source_breakdown,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Execute parallel price requests to all available APIs
    async fn execute_parallel_price_requests(
        &self,
        symbol: &str,
    ) -> Vec<(Option<PriceData>, Duration, ApiProvider)> {
        let mut results = Vec::new();

        for (provider, api) in &self.apis {
            if self.is_api_available(*provider) {
                let start = Instant::now();
                let result = api.get_price(symbol).await;
                let duration = start.elapsed();

                let success = result.is_ok();
                self.record_api_call(*provider, success, duration).await;

                results.push((result.ok(), duration, *provider));
            }
        }

        results
    }

    /// Get historical data with intelligent routing
    pub async fn get_historical_data_intelligent(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        let context = RequestContext {
            data_type: DataType::HistoricalData,
            priority: Priority::Reliability,
            max_budget: None,
            timeout: Duration::from_secs(60),
        };

        let metrics = self.metrics.read().await;

        if let Some(selected_api) = self.router.select_api(&self.apis, &metrics, &context).await {
            self.call_single_api_historical(&selected_api, symbol, days)
                .await
        } else {
            Err(ApiError::Unknown(
                "No suitable API for historical data".to_string(),
            ))
        }
    }

    /// Helper method to call historical data API without closure lifetime issues
    async fn call_single_api_historical(
        &self,
        provider: &ApiProvider,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        let api = self
            .apis
            .get(provider)
            .ok_or_else(|| ApiError::UnsupportedOperation(*provider))?;

        let start = Instant::now();
        let result = api.get_historical_data(symbol, days).await;
        let duration = start.elapsed();

        self.record_api_call(*provider, result.is_ok(), duration)
            .await;

        result
    }

    /// Get global market data with intelligent routing
    pub async fn get_global_market_data_intelligent(&self) -> Result<GlobalMarketData, ApiError> {
        let context = RequestContext {
            data_type: DataType::GlobalMarket,
            priority: Priority::Reliability,
            max_budget: None,
            timeout: Duration::from_secs(30),
        };

        let metrics = self.metrics.read().await;

        if let Some(selected_api) = self.router.select_api(&self.apis, &metrics, &context).await {
            self.call_single_api_global(&selected_api).await
        } else {
            Err(ApiError::Unknown(
                "No suitable API for global market data".to_string(),
            ))
        }
    }

    /// Helper method to call global market data API without closure lifetime issues
    async fn call_single_api_global(
        &self,
        provider: &ApiProvider,
    ) -> Result<GlobalMarketData, ApiError> {
        let api = self
            .apis
            .get(provider)
            .ok_or_else(|| ApiError::UnsupportedOperation(*provider))?;

        let start = Instant::now();
        let result = api.get_global_market_data().await;
        let duration = start.elapsed();

        self.record_api_call(*provider, result.is_ok(), duration)
            .await;

        result
    }

    /// Get comprehensive price analysis from multiple APIs
    pub async fn get_price_analysis(&self, symbol: &str) -> Result<PriceAnalysis, ApiError> {
        let results = self.execute_parallel_price_requests(symbol).await;

        if results.is_empty() {
            return Err(ApiError::Unknown(
                "No API responses received for analysis".to_string(),
            ));
        }

        let successful_results: Vec<&PriceData> = results
            .iter()
            .filter_map(|(price_data, _, _)| price_data.as_ref())
            .collect();

        if successful_results.is_empty() {
            return Err(ApiError::Unknown("All API requests failed".to_string()));
        }

        // Calculate statistics
        let prices: Vec<f64> = successful_results.iter().map(|p| p.price_usd).collect();
        let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;
        let min_price = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_price = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let price_spread = max_price - min_price;

        // Find fastest response
        let fastest_response = results
            .iter()
            .filter_map(|(price_data, duration, _)| price_data.as_ref().map(|_| duration))
            .min()
            .copied()
            .unwrap_or(Duration::from_secs(30));

        Ok(PriceAnalysis {
            symbol: symbol.to_uppercase(),
            average_price: avg_price,
            min_price,
            max_price,
            price_spread,
            consensus_price: utils::calculate_consensus_price(&successful_results)
                .unwrap_or(avg_price),
            api_count: successful_results.len() as u32,
            fastest_response_time: fastest_response,
            sources: successful_results.iter().map(|p| p.source).collect(),
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get current metrics for all APIs
    pub async fn get_metrics(&self) -> HashMap<ApiProvider, ApiMetrics> {
        self.metrics.read().await.clone()
    }

    /// Get resilience metrics for all providers
    pub fn get_resilience_metrics(&self) -> HashMap<ApiProvider, ResilienceMetrics> {
        self.resilience_manager.get_all_metrics()
    }

    /// Get resilience status for a specific provider
    pub fn get_provider_resilience_status(&self, provider: &ApiProvider) -> ResilienceStatus {
        self.resilience_manager.get_provider_status(provider)
    }

    /// Get total requests for a provider (for display purposes)
    pub fn get_provider_total_requests(&self, provider: &ApiProvider) -> Option<u64> {
        self.resilience_manager.get_provider_total_requests(provider)
    }

    /// Get resilience status for all providers
    pub fn get_all_resilience_status(&self) -> HashMap<ApiProvider, ResilienceStatus> {
        let mut status = HashMap::new();
        for provider in &[
            ApiProvider::CoinPaprika,
            ApiProvider::CoinGecko,
            ApiProvider::CoinMarketCap,
            ApiProvider::CryptoCompare,
        ] {
            status.insert(*provider, self.get_provider_resilience_status(provider));
        }
        status
    }

    /// Reset circuit breaker for a provider
    pub fn reset_circuit_breaker(&self, provider: &ApiProvider) {
        self.resilience_manager.reset_circuit_breaker(provider);
        println!("🔄 Circuit breaker reset for {}", provider);
    }

    /// Get resilience configuration
    pub fn get_resilience_config(&self) -> &ResilienceConfig {
        &self.resilience_manager.config
    }

    /// Check if API is available and healthy
    fn is_api_available(&self, provider: ApiProvider) -> bool {
        self.apis.contains_key(&provider)
            && futures::executor::block_on(async {
                let metrics = self.metrics.read().await;
                metrics
                    .get(&provider)
                    .map(|m| m.is_healthy())
                    .unwrap_or(true) // Allow untested APIs to be tried
            })
    }

    /// Call single API with metrics tracking
    async fn call_single_api<F, Fut, T>(
        &self,
        provider: &ApiProvider,
        _symbol: &str,
        operation: F,
    ) -> Result<T, ApiError>
    where
        F: Fn(&Box<dyn CryptoApi>) -> Fut,
        Fut: std::future::Future<Output = Result<T, ApiError>>,
    {
        let api = self
            .apis
            .get(provider)
            .ok_or_else(|| ApiError::UnsupportedOperation(*provider))?;

        let start = Instant::now();
        let result = operation(api).await;
        let duration = start.elapsed();

        self.record_api_call(*provider, result.is_ok(), duration)
            .await;

        result
    }

    /// Record API call metrics for RAG learning
    async fn record_api_call(&self, provider: ApiProvider, success: bool, duration: Duration) {
        // Record existing metrics
        let mut metrics = self.metrics.write().await;
        if let Some(metric) = metrics.get_mut(&provider) {
            if success {
                metric.record_success(duration);
            } else {
                metric.record_failure();
            }
        }

        // Record analytics data
        if let Some(analytics) = &self.analytics_manager {
            // Estimate cost based on provider (simplified for demo)
            let estimated_cost = match provider {
                ApiProvider::CoinGecko => 0.001,     // Free tier
                ApiProvider::CoinPaprika => 0.0,     // Free
                ApiProvider::CoinMarketCap => 0.01,  // Paid tier
                ApiProvider::CryptoCompare => 0.005, // Paid tier
            };

            if success {
                analytics
                    .record_successful_request(&provider, duration, estimated_cost)
                    .await;
            } else {
                // For failed requests, we don't have specific error details here
                // In a real implementation, we'd pass the actual error type
                analytics
                    .record_failed_request(&provider, duration, "unknown_error", estimated_cost)
                    .await;
            }

            // Update analytics metrics periodically
            analytics.update_metrics().await;
        }
    }

    // ===== CACHE MANAGEMENT METHODS =====

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> Option<crate::modules::cache::CacheStats> {
        self.cache.as_ref().map(|c| c.get_stats())
    }

    /// Get cache hit rate
    pub fn get_cache_hit_rate(&self) -> Option<f64> {
        self.cache.as_ref().map(|c| c.get_hit_rate())
    }

    // ===== ANALYTICS METHODS =====

    /// Get analytics manager reference
    pub fn get_analytics_manager(&self) -> Option<&Arc<AnalyticsManager>> {
        self.analytics_manager.as_ref()
    }

    /// Get usage metrics for all providers
    pub async fn get_analytics_usage_metrics(
        &self,
    ) -> Option<HashMap<ApiProvider, crate::modules::analytics::ApiUsageMetrics>> {
        self.analytics_manager
            .as_ref()?
            .get_usage_metrics()
            .await
            .into()
    }

    /// Get performance metrics dashboard
    pub async fn get_analytics_performance_metrics(
        &self,
    ) -> Option<crate::modules::analytics::PerformanceMetrics> {
        self.analytics_manager
            .as_ref()?
            .calculate_performance_metrics()
            .await
            .into()
    }

    /// Get cost analysis for current API usage
    pub async fn get_cost_analysis(
        &self,
    ) -> Option<HashMap<String, crate::modules::analytics::CostAnalysis>> {
        Some(self.analytics_manager.as_ref()?.analyze_costs(self).await)
    }

    /// Get optimization recommendations
    pub async fn get_optimization_recommendations(
        &self,
    ) -> Option<Vec<crate::modules::analytics::OptimizationRecommendation>> {
        self.analytics_manager
            .as_ref()?
            .generate_recommendations()
            .await
            .into()
    }

    /// Get analytics dashboard data
    pub async fn get_analytics_dashboard(&self) -> Option<serde_json::Value> {
        self.analytics_manager
            .as_ref()?
            .get_dashboard_data()
            .await
            .into()
    }

    /// Export analytics data for external analysis
    pub async fn export_analytics_data(&self) -> Option<serde_json::Value> {
        self.analytics_manager.as_ref()?.export_data().await.into()
    }

    // ===== HEALTH MONITORING METHODS =====

    /// Get health monitor reference
    pub fn get_health_monitor(&self) -> Option<&Arc<HealthMonitor>> {
        self.health_monitor.as_ref()
    }

    /// Check health of all APIs concurrently
    pub async fn check_all_api_health(
        &self,
    ) -> Option<std::collections::HashMap<ApiProvider, crate::modules::health::HealthStatus>> {
        // Create a new client instance for health checks
        let client = MultiApiClient::new_with_all_apis();
        let client_arc = Arc::new(client);
        Some(
            self.health_monitor
                .as_ref()?
                .check_all_health(client_arc)
                .await,
        )
    }

    /// Check health of a specific API provider
    pub async fn check_provider_health(
        &self,
        provider: &ApiProvider,
    ) -> Option<crate::modules::health::HealthStatus> {
        Some(
            self.health_monitor
                .as_ref()?
                .check_provider_health(self, provider)
                .await,
        )
    }

    /// Get health metrics for all providers
    pub async fn get_health_metrics(
        &self,
    ) -> Option<std::collections::HashMap<ApiProvider, crate::modules::health::HealthMetrics>> {
        Some(self.health_monitor.as_ref()?.get_health_metrics().await)
    }

    /// Get health dashboard data
    pub async fn get_health_dashboard(&self) -> Option<serde_json::Value> {
        self.health_monitor
            .as_ref()?
            .get_health_dashboard()
            .await
            .into()
    }

    /// Get recent health alerts
    pub async fn get_health_alerts(
        &self,
        limit: usize,
    ) -> Option<Vec<crate::modules::health::HealthAlert>> {
        Some(self.health_monitor.as_ref()?.get_recent_alerts(limit).await)
    }

    /// Get unresolved health alerts
    pub async fn get_unresolved_health_alerts(
        &self,
    ) -> Option<Vec<crate::modules::health::HealthAlert>> {
        Some(self.health_monitor.as_ref()?.get_unresolved_alerts().await)
    }

    /// Run performance benchmarks
    pub async fn run_performance_benchmarks(
        &self,
    ) -> Option<Vec<crate::modules::health::BenchmarkResult>> {
        // Create a new client instance for benchmarks
        let client = MultiApiClient::new_with_all_apis();
        let client_arc = Arc::new(client);
        Some(
            self.health_monitor
                .as_ref()?
                .run_performance_benchmarks(client_arc)
                .await,
        )
    }

    /// Get benchmark results
    pub async fn get_benchmark_results(
        &self,
        limit: usize,
    ) -> Option<Vec<crate::modules::health::BenchmarkResult>> {
        Some(
            self.health_monitor
                .as_ref()?
                .get_benchmark_results(limit)
                .await,
        )
    }

    /// Get health status summary
    pub async fn get_health_summary(&self) -> Option<String> {
        Some(self.health_monitor.as_ref()?.get_health_summary().await)
    }

    /// Start continuous health monitoring
    pub fn start_continuous_health_monitoring(&self) -> Option<()> {
        if let Some(monitor) = &self.health_monitor {
            let monitor_clone = Arc::clone(monitor);
            // Create a new client instance for monitoring
            let client = MultiApiClient::new_with_all_apis();
            let client_clone = Arc::new(client);
            tokio::spawn(async move {
                monitor_clone.start_monitoring(client_clone).await;
            });
        }
        Some(())
    }

    /// Invalidate cache for a specific provider
    pub async fn invalidate_provider_cache(&self, provider: &ApiProvider) {
        if let Some(cache) = &self.cache {
            cache.invalidate_provider(provider).await;
        }
    }

    /// Invalidate all expired cache entries
    pub async fn invalidate_expired_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.invalidate_expired().await;
        }
    }

    /// Clear entire cache
    pub async fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear().await;
        }
    }

    /// Get popular cache keys for warming
    pub fn get_popular_cache_keys(&self, limit: usize) -> Option<Vec<String>> {
        self.cache.as_ref().map(|c| c.get_popular_keys(limit))
    }

    /// Warm cache with popular symbols
    pub async fn warm_cache_with_popular_symbols(&self, symbols: Vec<String>) {
        if let Some(cache_warmer) = &self.cache_warmer {
            cache_warmer
                .warm_popular_symbols(symbols, |_key| async move {
                    // This would be replaced with actual API calls in a real implementation
                    None // Placeholder - would fetch from APIs
                })
                .await;
        }
    }

    /// Warm cache with global market data
    pub async fn warm_cache_with_global_data(&self) {
        if let Some(cache_warmer) = &self.cache_warmer {
            cache_warmer
                .warm_global_data(|| async move {
                    // This would be replaced with actual global market data fetch
                    None // Placeholder - would fetch from APIs
                })
                .await;
        }
    }

    /// Get cache information (current size, max size, utilization)
    pub fn get_cache_info(&self) -> Option<(usize, usize, f64)> {
        self.cache.as_ref().map(|c| c.get_cache_info())
    }

    /// Check if caching is enabled
    pub fn is_caching_enabled(&self) -> bool {
        self.cache.is_some()
    }

    /// Get cache health status
    pub fn get_cache_health(&self) -> Option<bool> {
        self.cache.as_ref().map(|c| c.health_check())
    }

    /// Populate cache from multiple APIs concurrently
    pub async fn populate_cache_from_apis(
        &self,
        providers: Vec<ApiProvider>,
        data_types: Vec<String>,
        symbols: Vec<String>,
    ) -> Result<(), ApiError> {
        if let Some(cache) = &self.cache {
            cache
                .populate_from_multiple_apis(
                    providers,
                    data_types,
                    symbols,
                    |provider, _data_type, _symbol| async move {
                        // This would be replaced with actual API calls
                        Err(ApiError::UnsupportedOperation(provider)) // Placeholder
                    },
                )
                .await?;
        }
        Ok(())
    }

    // ===== DATA PROCESSING METHODS =====

    /// Get normalized price data with processing pipeline
    pub async fn get_normalized_price(&self, symbol: &str) -> Result<NormalizedData, ApiError> {
        if let Some(processor) = &self.processor {
            // Collect responses from all APIs
            let mut responses = vec![];

            for (provider, _api) in &self.apis {
                let response = self.get_price_with_resilience(provider, symbol).await;
                // Convert PriceData to RawData for processing
                let raw_response = match response {
                    Ok(price_data) => Ok(RawData {
                        symbol: price_data.symbol.clone(),
                        name: price_data.symbol.clone(), // Use symbol as name for now
                        price_usd: price_data.price_usd,
                        volume_24h: price_data.volume_24h,
                        market_cap: price_data.market_cap,
                        price_change_24h: price_data.price_change_24h,
                        last_updated: price_data.last_updated,
                        source: price_data.source,
                    }),
                    Err(e) => Err(e),
                };
                responses.push((*provider, raw_response));
            }

            // Process responses concurrently
            processor
                .process_concurrent_responses(responses, symbol)
                .await
        } else {
            Err(ApiError::Unknown("Data processor not enabled".to_string()))
        }
    }

    /// Get processing statistics
    pub async fn get_processing_stats(&self) -> Option<crate::modules::processor::ProcessingStats> {
        self.processor.as_ref().map(|p| {
            // This would ideally be async, but we'll simulate for now
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async { p.get_processing_stats().await })
            })
        })
    }

    /// Check if processing is enabled
    pub fn is_processing_enabled(&self) -> bool {
        self.processor.is_some()
    }

    /// Get processor configuration
    pub fn get_processing_config(&self) -> Option<ProcessingConfig> {
        self.processor.as_ref().map(|p| p.get_config().clone())
    }

    /// Process historical data with normalization
    pub async fn get_normalized_historical(
        &self,
        symbol: &str,
        _limit: usize,
    ) -> Result<Vec<NormalizedData>, ApiError> {
        // For now, return price data as historical data (simplified implementation)
        match self.get_normalized_price(symbol).await {
            Ok(data) => Ok(vec![data]),
            Err(e) => Err(e),
        }
    }

    // ===== HISTORICAL DATA MANAGEMENT METHODS =====

    /// Fetch and store historical data for a symbol
    pub async fn fetch_historical_data(
        &self,
        symbol: &str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        interval: &str,
    ) -> Result<(), ApiError> {
        if let Some(manager) = &self.historical_manager {
            manager
                .fetch_and_store_historical(self, symbol, start_date, end_date, interval)
                .await
        } else {
            Err(ApiError::Unknown(
                "Historical data manager not enabled".to_string(),
            ))
        }
    }

    /// Query historical data
    pub async fn query_historical_data(
        &self,
        symbol: &str,
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<usize>,
    ) -> Result<Vec<TimeSeriesPoint>, ApiError> {
        if let Some(manager) = &self.historical_manager {
            manager
                .query_historical_data(symbol, start_date, end_date, limit)
                .await
        } else {
            Err(ApiError::Unknown(
                "Historical data manager not enabled".to_string(),
            ))
        }
    }

    /// Get historical data metadata
    pub async fn get_historical_metadata(
        &self,
        symbol: &str,
    ) -> Option<crate::modules::historical::HistoricalMetadata> {
        if let Some(manager) = &self.historical_manager {
            manager.get_metadata(symbol).await
        } else {
            None
        }
    }

    /// Get historical data storage statistics
    pub async fn get_historical_stats(&self) -> Option<crate::modules::historical::StorageStats> {
        if let Some(manager) = &self.historical_manager {
            Some(manager.get_storage_stats().await)
        } else {
            None
        }
    }

    /// Optimize historical data for RAG training
    pub async fn optimize_historical_for_rag(&self, symbol: &str) -> Result<Vec<String>, ApiError> {
        if let Some(manager) = &self.historical_manager {
            manager.optimize_for_rag(symbol).await
        } else {
            Err(ApiError::Unknown(
                "Historical data manager not enabled".to_string(),
            ))
        }
    }

    /// Check if historical data management is enabled
    pub fn is_historical_enabled(&self) -> bool {
        self.historical_manager.is_some()
    }

    /// Get historical data manager configuration
    pub fn get_historical_config(&self) -> Option<TimeSeriesConfig> {
        self.historical_manager.as_ref().map(|_m| {
            // This is a simplified version - in practice you'd need to expose the config
            TimeSeriesConfig {
                compression_enabled: true,
                compression_threshold: 1000,
                deduplication_enabled: true,
                gap_filling_enabled: true,
                validation_enabled: true,
                storage_path: std::path::PathBuf::from("./data/historical"),
                max_memory_cache: 10000,
                prefetch_window: chrono::Duration::days(7),
            }
        })
    }
}

/// Utility functions for data processing
pub mod utils {
    use super::*;

    /// Calculate consensus price from multiple API results
    pub fn calculate_consensus_price(prices: &[&PriceData]) -> Option<f64> {
        if prices.is_empty() {
            return None;
        }

        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for price in prices {
            let weight = match price.source {
                ApiProvider::CoinPaprika => 1.0,
                ApiProvider::CoinGecko => 1.2,
                ApiProvider::CoinMarketCap => 1.5,
                ApiProvider::CryptoCompare => 1.3,
            };

            weighted_sum += price.price_usd * weight;
            total_weight += weight;
        }

        Some(weighted_sum / total_weight)
    }

    /// Validate API response data quality
    pub fn validate_price_data(price: &PriceData) -> bool {
        price.price_usd > 0.0
            && price.symbol.len() >= 2
            && price.symbol.len() <= 10
            && price.last_updated.timestamp() > 0
    }

    /// Normalize cryptocurrency symbols across APIs
    pub fn normalize_symbol(symbol: &str) -> String {
        symbol
            .to_uppercase()
            .replace("BTC", "bitcoin")
            .replace("ETH", "ethereum")
            .replace("USDT", "tether")
            .replace("BNB", "binance-coin")
            .replace("ADA", "cardano")
            .replace("SOL", "solana")
            .replace("DOT", "polkadot")
            .replace("DOGE", "dogecoin")
    }
}

// Export key types for external use
pub use utils::*;

/// ============================================================================
/// BYOK CONFIGURATION SYSTEM
/// ============================================================================
use tokio::sync::RwLock;

/// BYOK Configuration Manager
#[derive(Debug, Clone)]
pub struct ByokConfigManager {
    api_configs: Arc<RwLock<HashMap<ApiProvider, ApiConfig>>>,
    validation_rules: HashMap<ApiProvider, ValidationRule>,
    hot_reload_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub required_length: Option<usize>,
    pub pattern: Option<String>,
    pub prefix: Option<String>,
    pub custom_validator: Option<String>, // Could be extended with custom validation functions
}

impl ByokConfigManager {
    pub fn new() -> Self {
        let mut validation_rules = HashMap::new();

        // Define validation rules for each API provider
        validation_rules.insert(
            ApiProvider::CoinGecko,
            ValidationRule {
                required_length: Some(20), // CoinGecko API keys are typically 20+ characters
                pattern: Some(r"^[A-Za-z0-9_-]{20,}$".to_string()),
                prefix: None, // Not all keys start with CG-
                custom_validator: None,
            },
        );

        validation_rules.insert(
            ApiProvider::CoinMarketCap,
            ValidationRule {
                required_length: Some(32), // CMC API keys are typically 32+ characters
                pattern: Some(r"^[a-f0-9-]{32,}$".to_string()), // CMC keys can contain dashes
                prefix: None,
                custom_validator: None,
            },
        );

        validation_rules.insert(
            ApiProvider::CryptoCompare,
            ValidationRule {
                required_length: Some(32), // CryptoCompare keys are typically 32+ characters
                pattern: Some(r"^[A-Za-z0-9_-]{32,}$".to_string()),
                prefix: None,
                custom_validator: None,
            },
        );

        // CoinPaprika doesn't require validation (no API key)
        validation_rules.insert(
            ApiProvider::CoinPaprika,
            ValidationRule {
                required_length: None,
                pattern: None,
                prefix: None,
                custom_validator: None,
            },
        );

        Self {
            api_configs: Arc::new(RwLock::new(HashMap::new())),
            validation_rules,
            hot_reload_enabled: true,
        }
    }

    /// Load configuration from environment variables
    pub async fn load_from_env(&self) -> Result<(), ConfigError> {
        let mut configs = self.api_configs.write().await;

        // Load each API configuration
        configs.insert(ApiProvider::CoinPaprika, ApiConfig::coinpaprika_default());
        configs.insert(ApiProvider::CoinGecko, ApiConfig::coingecko_default());
        configs.insert(
            ApiProvider::CoinMarketCap,
            ApiConfig::coinmarketcap_default(),
        );
        configs.insert(
            ApiProvider::CryptoCompare,
            ApiConfig::cryptocompare_default(),
        );

        Ok(())
    }

    /// Validate API key for a specific provider
    pub fn validate_api_key(
        &self,
        provider: ApiProvider,
        api_key: &str,
    ) -> Result<(), ConfigError> {
        let rule = self
            .validation_rules
            .get(&provider)
            .ok_or_else(|| ConfigError::UnknownProvider(provider.to_string()))?;

        // CoinPaprika doesn't require validation
        if provider == ApiProvider::CoinPaprika {
            return Ok(());
        }

        // Check if API key is provided
        if api_key.trim().is_empty() {
            return Err(ConfigError::MissingApiKey(provider.to_string()));
        }

        // Check required length
        if let Some(min_len) = rule.required_length {
            if api_key.len() < min_len {
                return Err(ConfigError::InvalidKeyLength {
                    provider: provider.to_string(),
                    expected: min_len,
                    actual: api_key.len(),
                });
            }
        }

        // Check pattern
        if let Some(pattern) = &rule.pattern {
            let regex = regex::Regex::new(pattern)
                .map_err(|_| ConfigError::InvalidPattern(pattern.clone()))?;
            if !regex.is_match(api_key) {
                return Err(ConfigError::InvalidKeyFormat(provider.to_string()));
            }
        }

        // Check prefix
        if let Some(prefix) = &rule.prefix {
            if !api_key.starts_with(prefix) {
                return Err(ConfigError::InvalidKeyPrefix {
                    provider: provider.to_string(),
                    expected: prefix.clone(),
                });
            }
        }

        Ok(())
    }

    /// Get validated API configuration for a provider
    pub async fn get_validated_config(
        &self,
        provider: ApiProvider,
    ) -> Result<ApiConfig, ConfigError> {
        let configs = self.api_configs.read().await;
        let config = configs
            .get(&provider)
            .ok_or_else(|| ConfigError::UnknownProvider(provider.to_string()))?
            .clone();

        // Validate API key if required
        if let Some(ref api_key) = config.api_key {
            self.validate_api_key(provider, api_key)?;
        } else if provider != ApiProvider::CoinPaprika {
            return Err(ConfigError::MissingApiKey(provider.to_string()));
        }

        Ok(config)
    }

    /// Update API key for a provider
    pub async fn update_api_key(
        &self,
        provider: ApiProvider,
        api_key: String,
    ) -> Result<(), ConfigError> {
        // Validate the new key
        self.validate_api_key(provider, &api_key)?;

        // Set the environment variable for persistence
        let env_var = match provider {
            ApiProvider::CoinGecko => "COINGECKO_API_KEY",
            ApiProvider::CoinMarketCap => "COINMARKETCAP_API_KEY",
            ApiProvider::CryptoCompare => "CRYPTOCOMPARE_API_KEY",
            ApiProvider::CoinPaprika => return Ok(()), // No key needed
        };

        std::env::set_var(env_var, &api_key);

        let mut configs = self.api_configs.write().await;
        if let Some(config) = configs.get_mut(&provider) {
            config.api_key = Some(api_key);
            config.enabled = true;
        }

        Ok(())
    }

    /// Get configuration status for all providers
    pub async fn get_config_status(&self) -> HashMap<ApiProvider, ConfigStatus> {
        let mut status = HashMap::new();

        for &provider in &[
            ApiProvider::CoinPaprika,
            ApiProvider::CoinGecko,
            ApiProvider::CoinMarketCap,
            ApiProvider::CryptoCompare,
        ] {
            let config_result = self.get_validated_config(provider).await;
            let config_status = match config_result {
                Ok(config) => {
                    if config.is_configured() {
                        ConfigStatus::Configured
                    } else {
                        ConfigStatus::NotConfigured
                    }
                }
                Err(ConfigError::MissingApiKey(_)) => ConfigStatus::NotConfigured,
                Err(_) => ConfigStatus::Invalid,
            };

            status.insert(provider, config_status);
        }

        status
    }

    /// Export configuration to .env format
    pub async fn export_to_env_format(&self) -> String {
        let configs = self.api_configs.read().await;
        let mut env_content = String::new();

        env_content.push_str("# I.O.R.A. Environment Configuration\n");
        env_content.push_str("# Update these values with your actual credentials\n\n");

        // Gemini AI Key (existing)
        env_content
            .push_str("# Gemini AI API Key (get from: https://makersuite.google.com/app/apikey)\n");
        env_content.push_str(&format!(
            "GEMINI_API_KEY={}\n\n",
            std::env::var("GEMINI_API_KEY").unwrap_or("your_gemini_api_key_here".to_string())
        ));

        // Solana Configuration (existing)
        env_content.push_str("# Solana Configuration (pre-configured)\n");
        env_content.push_str(&format!(
            "SOLANA_RPC_URL={}\n",
            std::env::var("SOLANA_RPC_URL")
                .unwrap_or("https://api.mainnet-beta.solana.com".to_string())
        ));
        env_content.push_str(&format!(
            "SOLANA_WALLET_PATH={}\n\n",
            std::env::var("SOLANA_WALLET_PATH")
                .unwrap_or("./wallets/mainnet-wallet.json".to_string())
        ));

        // Typesense Configuration (existing)
        env_content.push_str("# Self-hosted Typesense Configuration\n");
        env_content.push_str(&format!(
            "TYPESENSE_API_KEY={}\n",
            std::env::var("TYPESENSE_API_KEY").unwrap_or("iora_dev_typesense_key_2024".to_string())
        ));
        env_content.push_str(&format!(
            "TYPESENSE_URL={}\n\n",
            std::env::var("TYPESENSE_URL")
                .unwrap_or("https://typesense.your-domain.com".to_string())
        ));

        // Crypto API Keys
        env_content.push_str("# Crypto API Keys for Multi-API Data Fetching (Task 2.1.2)\n");

        for (provider, _config) in configs.iter() {
            if *provider != ApiProvider::CoinPaprika {
                // Skip CoinPaprika as it doesn't need a key
                let env_var_name = match provider {
                    ApiProvider::CoinGecko => "COINGECKO_API_KEY",
                    ApiProvider::CoinMarketCap => "COINMARKETCAP_API_KEY",
                    ApiProvider::CryptoCompare => "CRYPTOCOMPARE_API_KEY",
                    _ => continue,
                };

                let _env_var_value =
                    std::env::var(env_var_name).unwrap_or("your_api_key_here".to_string());
                let comment = match provider {
                    ApiProvider::CoinGecko => "# Get from: https://www.coingecko.com/en/api",
                    ApiProvider::CoinMarketCap => "# Get from: https://coinmarketcap.com/api/",
                    ApiProvider::CryptoCompare => "# Get from: https://min-api.cryptocompare.com/",
                    _ => "",
                };

                env_content.push_str(&format!("{} {}\n", comment, env_var_name));
                env_content.push_str(&format!("{}=your_api_key_here\n\n", env_var_name));
            }
        }

        env_content
    }

    /// Enable or disable hot reloading
    pub fn set_hot_reload(&mut self, enabled: bool) {
        self.hot_reload_enabled = enabled;
    }

    /// Start hot reloading for configuration changes
    pub async fn start_hot_reload(&self) -> Result<(), ConfigError> {
        use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
        use std::path::Path;
        use std::sync::mpsc::channel;

        if !self.hot_reload_enabled {
            return Ok(());
        }

        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default())
            .map_err(|e| ConfigError::NotifyError(e.to_string()))?;

        // Watch the .env file
        let env_path = Path::new(".env");
        if env_path.exists() {
            watcher
                .watch(env_path, RecursiveMode::NonRecursive)
                .map_err(|e| ConfigError::NotifyError(e.to_string()))?;

            println!("🔄 Hot reloading enabled for .env file");
            println!("💡 Configuration will automatically reload on file changes");
        } else {
            println!("⚠️  .env file not found - hot reloading disabled");
            return Ok(());
        }

        // Clone self for the async task
        let config_manager = std::sync::Arc::new(self.clone());

        tokio::spawn(async move {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        if let Ok(Event {
                            kind: EventKind::Modify(_),
                            ..
                        }) = event
                        {
                            println!("\n🔄 Configuration file changed - reloading...");

                            // Reload configuration
                            if let Err(e) = config_manager.load_from_env().await {
                                eprintln!("❌ Failed to reload configuration: {}", e);
                            } else {
                                println!("✅ Configuration reloaded successfully");

                                // Show updated status
                                let status = config_manager.get_config_status().await;
                                println!("\n📊 Updated Configuration Status:");
                                for (provider, config_status) in status {
                                    let status_icon = match config_status {
                                        ConfigStatus::Configured => "✅",
                                        ConfigStatus::NotConfigured => "❌",
                                        ConfigStatus::Invalid => "⚠️ ",
                                    };
                                    println!("{} {}", status_icon, provider);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ File watching error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// Secure API key storage using environment encryption
    pub fn secure_store_api_key(provider: ApiProvider, api_key: &str) -> Result<(), ConfigError> {
        use std::env;

        // Basic encryption using base64 (in production, use proper encryption)
        let encrypted = base64::engine::general_purpose::STANDARD.encode(api_key.as_bytes());

        // Store in environment variable with secure naming
        let env_var = match provider {
            ApiProvider::CoinGecko => "CG_ENCRYPTED_KEY",
            ApiProvider::CoinMarketCap => "CMC_ENCRYPTED_KEY",
            ApiProvider::CryptoCompare => "CC_ENCRYPTED_KEY",
            ApiProvider::CoinPaprika => return Ok(()), // No key needed
        };

        env::set_var(env_var, encrypted);
        println!("🔐 API key securely stored for {}", provider);

        Ok(())
    }

    /// Secure API key retrieval with decryption
    pub fn secure_retrieve_api_key(provider: ApiProvider) -> Result<String, ConfigError> {
        use std::env;

        let env_var = match provider {
            ApiProvider::CoinGecko => "CG_ENCRYPTED_KEY",
            ApiProvider::CoinMarketCap => "CMC_ENCRYPTED_KEY",
            ApiProvider::CryptoCompare => "CC_ENCRYPTED_KEY",
            ApiProvider::CoinPaprika => {
                return Err(ConfigError::MissingApiKey("CoinPaprika".to_string()))
            }
        };

        let encrypted =
            env::var(env_var).map_err(|_| ConfigError::MissingApiKey(provider.to_string()))?;

        // Basic decryption (in production, use proper decryption)
        let decrypted = base64::engine::general_purpose::STANDARD
            .decode(&encrypted)
            .map_err(|_| ConfigError::InvalidKeyFormat(provider.to_string()))?;

        String::from_utf8(decrypted)
            .map_err(|_| ConfigError::InvalidKeyFormat(provider.to_string()))
    }
}

/// Configuration status for API providers
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigStatus {
    Configured,    // API key is present and valid
    NotConfigured, // API key is missing
    Invalid,       // API key is present but invalid
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Unknown API provider: {0}")]
    UnknownProvider(String),

    #[error("Missing API key for {0}")]
    MissingApiKey(String),

    #[error("Invalid API key length for {provider}: expected {expected}, got {actual}")]
    InvalidKeyLength {
        provider: String,
        expected: usize,
        actual: usize,
    },

    #[error("Invalid API key format for {0}")]
    InvalidKeyFormat(String),

    #[error("Invalid API key prefix for {provider}: expected {expected}")]
    InvalidKeyPrefix { provider: String, expected: String },

    #[error("Invalid validation pattern: {0}")]
    InvalidPattern(String),

    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File watching error: {0}")]
    NotifyError(String),

    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
/// ============================================================================
/// ENHANCED ERROR HANDLING & RESILIENCE SYSTEM
/// ============================================================================
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,   // Normal operation
    Open,     // Circuit is open, failing fast
    HalfOpen, // Testing if service has recovered
}

/// Enhanced API metrics with resilience tracking
#[derive(Debug)]
pub struct ResilienceMetrics {
    pub consecutive_failures: AtomicU32,
    pub last_failure_time: AtomicU64,
    pub circuit_state: std::sync::RwLock<CircuitState>,
    pub total_requests: AtomicU64,
    pub successful_requests: AtomicU64,
    pub failed_requests: AtomicU64,
    pub timeout_count: AtomicU32,
    pub rate_limit_count: AtomicU32,
}

impl Clone for ResilienceMetrics {
    fn clone(&self) -> Self {
        // Create a new instance with the same values
        let new_metrics = ResilienceMetrics::new();
        new_metrics.consecutive_failures.store(
            self.consecutive_failures.load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        new_metrics.last_failure_time.store(
            self.last_failure_time.load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        // Copy circuit state
        *new_metrics.circuit_state.write().unwrap() = self.circuit_state.read().unwrap().clone();
        new_metrics
            .total_requests
            .store(self.total_requests.load(Ordering::SeqCst), Ordering::SeqCst);
        new_metrics.successful_requests.store(
            self.successful_requests.load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        new_metrics.failed_requests.store(
            self.failed_requests.load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        new_metrics
            .timeout_count
            .store(self.timeout_count.load(Ordering::SeqCst), Ordering::SeqCst);
        new_metrics.rate_limit_count.store(
            self.rate_limit_count.load(Ordering::SeqCst),
            Ordering::SeqCst,
        );
        new_metrics
    }
}

impl ResilienceMetrics {
    pub fn new() -> Self {
        Self {
            consecutive_failures: AtomicU32::new(0),
            last_failure_time: AtomicU64::new(0),
            circuit_state: std::sync::RwLock::new(CircuitState::Closed),
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            timeout_count: AtomicU32::new(0),
            rate_limit_count: AtomicU32::new(0),
        }
    }

    pub fn record_success(&self) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);
        self.successful_requests.fetch_add(1, Ordering::SeqCst);
        self.consecutive_failures.store(0, Ordering::SeqCst);

        // Reset circuit breaker if it's half-open
        if let Ok(mut state) = self.circuit_state.write() {
            if *state == CircuitState::HalfOpen {
                *state = CircuitState::Closed;
            }
        }
    }

    pub fn record_failure(&self, error_type: &ErrorType) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);
        self.failed_requests.fetch_add(1, Ordering::SeqCst);
        self.consecutive_failures.fetch_add(1, Ordering::SeqCst);
        self.last_failure_time.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            Ordering::SeqCst,
        );

        // Update specific error counters
        match error_type {
            ErrorType::Timeout => {
                self.timeout_count.fetch_add(1, Ordering::SeqCst);
            }
            ErrorType::RateLimit => {
                self.rate_limit_count.fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        }

        // Check if circuit breaker should open
        let consecutive_failures = self.consecutive_failures.load(Ordering::SeqCst);
        if consecutive_failures >= 5 {
            // Circuit breaker threshold
            if let Ok(mut state) = self.circuit_state.write() {
                *state = CircuitState::Open;
            }
        }
    }

    pub fn is_circuit_open(&self) -> bool {
        if let Ok(state) = self.circuit_state.read() {
            *state == CircuitState::Open
        } else {
            false
        }
    }

    pub fn should_attempt_recovery(&self) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let last_failure = self.last_failure_time.load(Ordering::SeqCst);

        // Attempt recovery after 60 seconds
        if let Ok(mut state) = self.circuit_state.write() {
            if *state == CircuitState::Open && (current_time - last_failure) > 60 {
                *state = CircuitState::HalfOpen;
                return true;
            }
        }

        false
    }

    pub fn get_success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::SeqCst);
        let successful = self.successful_requests.load(Ordering::SeqCst);

        if total == 0 {
            0.0
        } else {
            (successful as f64) / (total as f64)
        }
    }
}

/// Comprehensive error classification system
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    // Network errors
    NetworkError,
    Timeout,
    ConnectionFailed,
    DnsResolutionFailed,

    // API errors
    RateLimit,
    Unauthorized,
    Forbidden,
    NotFound,
    ServerError,
    BadRequest,

    // Data errors
    ParseError,
    ValidationError,
    DataNotAvailable,

    // System errors
    ConfigurationError,
    InternalError,

    // Unknown errors
    Unknown,
}

impl ErrorType {
    pub fn from_status_code(status: u16) -> Self {
        match status {
            400 => ErrorType::BadRequest,
            401 => ErrorType::Unauthorized,
            403 => ErrorType::Forbidden,
            404 => ErrorType::NotFound,
            429 => ErrorType::RateLimit,
            500..=599 => ErrorType::ServerError,
            _ => ErrorType::Unknown,
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ErrorType::NetworkError
                | ErrorType::Timeout
                | ErrorType::ConnectionFailed
                | ErrorType::ServerError
                | ErrorType::RateLimit
        )
    }

    pub fn is_circuit_breaker_error(&self) -> bool {
        matches!(
            self,
            ErrorType::ServerError
                | ErrorType::NetworkError
                | ErrorType::Timeout
                | ErrorType::ConnectionFailed
        )
    }
}

/// Resilience configuration
#[derive(Debug, Clone)]
pub struct ResilienceConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub timeout_seconds: u64,
    pub circuit_breaker_threshold: u32,
    pub recovery_timeout_seconds: u64,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 10000,
            timeout_seconds: 30,
            circuit_breaker_threshold: 5,
            recovery_timeout_seconds: 60,
        }
    }
}

/// Enhanced resilience manager
#[derive(Debug, Clone)]
pub struct ResilienceManager {
    config: ResilienceConfig,
    metrics: Arc<std::sync::RwLock<HashMap<ApiProvider, ResilienceMetrics>>>,
}

impl ResilienceManager {
    pub fn new(config: ResilienceConfig) -> Self {
        Self {
            config,
            metrics: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn default() -> Self {
        Self::new(ResilienceConfig::default())
    }

    /// Get or create metrics for a provider
    fn get_metrics(&self, provider: &ApiProvider) -> ResilienceMetrics {
        let metrics_map = self.metrics.read().unwrap();
        if let Some(metrics) = metrics_map.get(provider) {
            metrics.clone()
        } else {
            drop(metrics_map);
            let mut metrics_map = self.metrics.write().unwrap();
            metrics_map
                .entry(*provider)
                .or_insert_with(ResilienceMetrics::new)
                .clone()
        }
    }

    /// Execute operation with exponential backoff and circuit breaker
    pub async fn execute_with_resilience<F, Fut, T>(
        &self,
        provider: &ApiProvider,
        operation: F,
    ) -> Result<T, ApiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, ApiError>>,
    {
        let metrics = self.get_metrics(provider);

        // Check circuit breaker
        if metrics.is_circuit_open() {
            if !metrics.should_attempt_recovery() {
                return Err(ApiError::CircuitBreakerOpen(*provider));
            }
        }

        // Create retry strategy with exponential backoff and jitter
        let retry_strategy = ExponentialBackoff::from_millis(self.config.base_delay_ms)
            .max_delay(std::time::Duration::from_millis(self.config.max_delay_ms))
            .map(jitter)
            .take(self.config.max_retries as usize);

        // Execute with retry
        let result = Retry::spawn(retry_strategy, || async {
            // Add timeout
            let operation_future = operation();
            let timeout_duration = std::time::Duration::from_secs(self.config.timeout_seconds);

            match tokio::time::timeout(timeout_duration, operation_future).await {
                Ok(result) => result,
                Err(_) => Err(ApiError::Timeout(*provider)),
            }
        })
        .await;

        // Record metrics
        match &result {
            Ok(_) => metrics.record_success(),
            Err(e) => {
                let error_type = self.classify_error(e);
                metrics.record_failure(&error_type);
            }
        }

        result
    }

    /// Classify error type for resilience handling
    fn classify_error(&self, error: &ApiError) -> ErrorType {
        match error {
            ApiError::Timeout(_) => ErrorType::Timeout,
            ApiError::RateLimit(_) => ErrorType::RateLimit,
            ApiError::Unauthorized(_) => ErrorType::Unauthorized,
            ApiError::Forbidden(_) => ErrorType::Forbidden,
            ApiError::NotFound(_) => ErrorType::NotFound,
            ApiError::ServerError(_) => ErrorType::ServerError,
            ApiError::NetworkError(_) => ErrorType::NetworkError,
            ApiError::ParseError(_) => ErrorType::ParseError,
            ApiError::UnsupportedOperation(_) => ErrorType::BadRequest,
            _ => ErrorType::Unknown,
        }
    }

    /// Get resilience metrics for all providers
    pub fn get_all_metrics(&self) -> HashMap<ApiProvider, ResilienceMetrics> {
        self.metrics.read().unwrap().clone()
    }

    /// Get resilience status for a provider
    pub fn get_provider_status(&self, provider: &ApiProvider) -> ResilienceStatus {
        let metrics = self.get_metrics(provider);
        let circuit_state = metrics.circuit_state.read().unwrap().clone();
        let success_rate = metrics.get_success_rate();
        let consecutive_failures = metrics.consecutive_failures.load(Ordering::SeqCst);

        // Consider healthy if circuit is closed and no consecutive failures
        // For systems with no requests yet, assume healthy until proven otherwise
        let total_requests = metrics.total_requests.load(Ordering::SeqCst);
        let is_healthy = if total_requests == 0 {
            circuit_state != CircuitState::Open
        } else {
            success_rate > 0.8 && circuit_state != CircuitState::Open
        };

        ResilienceStatus {
            provider: *provider,
            circuit_state: circuit_state.clone(),
            success_rate,
            consecutive_failures,
            is_healthy,
        }
    }

    /// Get total requests for a provider
    pub fn get_provider_total_requests(&self, provider: &ApiProvider) -> Option<u64> {
        let metrics = self.get_metrics(provider);
        Some(metrics.total_requests.load(Ordering::SeqCst))
    }

    /// Reset circuit breaker for a provider
    pub fn reset_circuit_breaker(&self, provider: &ApiProvider) {
        let metrics = self.get_metrics(provider);
        *metrics.circuit_state.write().unwrap() = CircuitState::Closed;
        metrics.consecutive_failures.store(0, Ordering::SeqCst);
    }

    /// Graceful degradation strategy
    pub async fn execute_with_graceful_degradation<F, Fut, T, Fallback>(
        &self,
        primary_operation: F,
        fallback_operation: Fallback,
        provider: &ApiProvider,
    ) -> Result<T, ApiError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, ApiError>>,
        Fallback: Fn() -> Result<T, ApiError>,
    {
        // Try primary operation with resilience
        match self
            .execute_with_resilience(provider, primary_operation)
            .await
        {
            Ok(result) => Ok(result),
            Err(_) => {
                // Primary failed, try fallback
                println!(
                    "⚠️  Primary operation failed for {}, falling back to degraded mode",
                    provider
                );
                fallback_operation()
            }
        }
    }
}

/// Resilience status for monitoring
#[derive(Debug, Clone)]
pub struct ResilienceStatus {
    pub provider: ApiProvider,
    pub circuit_state: CircuitState,
    pub success_rate: f64,
    pub consecutive_failures: u32,
    pub is_healthy: bool,
}

/// ============================================================================
/// INDIVIDUAL API IMPLEMENTATIONS
/// ============================================================================

/// CoinPaprika API Implementation
/// Free API, no authentication required
/// Base URL: https://api.coinpaprika.com/v1
pub struct CoinPaprikaApi {
    client: reqwest::Client,
    config: ApiConfig,
}

impl CoinPaprikaApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            config: ApiConfig::coinpaprika_default(),
        }
    }

    /// Convert symbol to CoinPaprika coin ID
    /// CoinPaprika uses format like "btc-bitcoin"
    fn symbol_to_coin_id(&self, symbol: &str) -> String {
        let normalized = utils::normalize_symbol(symbol);
        match normalized.as_str() {
            "bitcoin" => "btc-bitcoin".to_string(),
            "ethereum" => "eth-ethereum".to_string(),
            "tether" => "usdt-tether".to_string(),
            "binance-coin" => "bnb-binance-coin".to_string(),
            "cardano" => "ada-cardano".to_string(),
            "solana" => "sol-solana".to_string(),
            "polkadot" => "dot-polkadot".to_string(),
            "dogecoin" => "doge-dogecoin".to_string(),
            // For unknown symbols, try the format: {symbol}-{symbol}
            _ => format!("{}-{}", symbol.to_lowercase(), symbol.to_lowercase()),
        }
    }
}

#[async_trait::async_trait]
impl CryptoApi for CoinPaprikaApi {
    fn provider(&self) -> ApiProvider {
        ApiProvider::CoinPaprika
    }

    async fn get_price(&self, symbol: &str) -> Result<PriceData, ApiError> {
        let coin_id = self.symbol_to_coin_id(symbol);

        let url = format!("{}/tickers/{}", self.config.base_url, coin_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Err(ApiError::NotFound(format!("Coin not found: {}", symbol)));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        // Parse CoinPaprika response format
        let quotes = data["quotes"]["USD"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Invalid response format".to_string()))?;

        let price = quotes["price"]
            .as_f64()
            .ok_or_else(|| ApiError::ApiError("Price not found in response".to_string()))?;

        let volume_24h = quotes["volume_24h"].as_f64();
        let market_cap = quotes["market_cap"].as_f64();
        let percent_change_24h = quotes["percent_change_24h"].as_f64();

        Ok(PriceData {
            symbol: symbol.to_uppercase(),
            price_usd: price,
            volume_24h,
            market_cap,
            price_change_24h: percent_change_24h,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinPaprika,
        })
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        let coin_id = self.symbol_to_coin_id(symbol);

        // CoinPaprika historical data endpoint
        let url = format!(
            "{}/coins/{}/ohlcv/historical?start={}&limit={}",
            self.config.base_url,
            coin_id,
            (chrono::Utc::now() - chrono::Duration::days(days as i64)).timestamp(),
            days
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let mut historical_data = Vec::new();

        for item in data {
            if let Some(timestamp) = item[0].as_i64() {
                let open = item[1].as_f64().unwrap_or(0.0);
                let high = item[2].as_f64().unwrap_or(0.0);
                let low = item[3].as_f64().unwrap_or(0.0);
                let close = item[4].as_f64().unwrap_or(0.0);
                let volume = item[5].as_f64();

                historical_data.push(HistoricalData {
                    symbol: symbol.to_uppercase(),
                    timestamp: chrono::DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_else(|| chrono::Utc::now()),
                    open,
                    high,
                    low,
                    close,
                    volume,
                    source: ApiProvider::CoinPaprika,
                });
            }
        }

        Ok(historical_data)
    }

    async fn get_global_market_data(&self) -> Result<GlobalMarketData, ApiError> {
        let url = format!("{}/global", self.config.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let market_data = data
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Invalid global data format".to_string()))?;

        let total_market_cap = market_data["market_cap_usd"].as_f64().unwrap_or(0.0);
        let total_volume_24h = market_data["volume_24h_usd"].as_f64().unwrap_or(0.0);
        let market_cap_change_percentage_24h = market_data["market_cap_change_percentage_24h_usd"]
            .as_f64()
            .unwrap_or(0.0);
        let active_cryptocurrencies = market_data["active_cryptocurrencies"].as_u64().unwrap_or(0);

        Ok(GlobalMarketData {
            total_market_cap_usd: total_market_cap,
            total_volume_24h_usd: total_volume_24h,
            market_cap_change_percentage_24h: market_cap_change_percentage_24h,
            active_cryptocurrencies,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinPaprika,
        })
    }

    async fn is_available(&self) -> bool {
        // Simple ping to check availability
        let url = format!("{}/ping", self.config.base_url);
        self.client.get(&url).send().await.is_ok()
    }

    fn rate_limit(&self) -> u32 {
        self.config.rate_limit
    }

    fn config(&self) -> &ApiConfig {
        &self.config
    }
}

/// CoinGecko API Implementation
/// Supports both free and paid tiers
/// Base URL: https://api.coingecko.com/api/v3
pub struct CoinGeckoApi {
    client: reqwest::Client,
    config: ApiConfig,
}

impl CoinGeckoApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            config: ApiConfig::coingecko_default(),
        }
    }

    /// Convert symbol to CoinGecko coin ID
    fn symbol_to_coin_id(&self, symbol: &str) -> String {
        let normalized = utils::normalize_symbol(symbol);
        match normalized.as_str() {
            "bitcoin" => "bitcoin".to_string(),
            "ethereum" => "ethereum".to_string(),
            "tether" => "tether".to_string(),
            "binance-coin" => "binancecoin".to_string(),
            "cardano" => "cardano".to_string(),
            "solana" => "solana".to_string(),
            "polkadot" => "polkadot".to_string(),
            "dogecoin" => "dogecoin".to_string(),
            _ => symbol.to_lowercase(),
        }
    }
}

#[async_trait::async_trait]
impl CryptoApi for CoinGeckoApi {
    fn provider(&self) -> ApiProvider {
        ApiProvider::CoinGecko
    }

    async fn get_price(&self, symbol: &str) -> Result<PriceData, ApiError> {
        let coin_id = self.symbol_to_coin_id(symbol);

        let mut url = format!("{}/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
            self.config.base_url, coin_id);

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("&x_cg_demo_api_key={}", key));
        }

        let mut response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            // If API key is invalid (401), try again without API key (CoinGecko works with free tier)
            if response.status() == reqwest::StatusCode::UNAUTHORIZED && self.config.api_key.is_some() {
                println!("⚠️  CoinGecko API key invalid, trying without API key...");
                let fallback_url = format!("{}/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
                    self.config.base_url, coin_id);

                let fallback_response = self
                    .client
                    .get(&fallback_url)
                    .send()
                    .await
                    .map_err(|e| ApiError::Http(e))?;

                if !fallback_response.status().is_success() {
                    if fallback_response.status() == reqwest::StatusCode::NOT_FOUND {
                        return Err(ApiError::NotFound(format!("Coin not found: {}", symbol)));
                    }
                    return Err(ApiError::ApiError(format!(
                        "API returned status: {} (Provider: {:?})",
                        fallback_response.status(),
                        self.provider()
                    )));
                }
                // Use the fallback response
                response = fallback_response;
            } else {
                if response.status() == reqwest::StatusCode::NOT_FOUND {
                    return Err(ApiError::NotFound(format!("Coin not found: {}", symbol)));
                }
                if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    return Err(ApiError::RateLimitExceeded(ApiProvider::CoinGecko));
                }
                return Err(ApiError::ApiError(format!(
                    "API returned status: {} (Provider: {:?})",
                    response.status(),
                    self.provider()
                )));
            }
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let coin_data = data[coin_id]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Coin data not found in response".to_string()))?;

        let price = coin_data["usd"]
            .as_f64()
            .ok_or_else(|| ApiError::ApiError("USD price not found".to_string()))?;

        let price_change_24h = coin_data["usd_24h_change"].as_f64();
        let volume_24h = coin_data["usd_24h_vol"].as_f64();
        let market_cap = coin_data["usd_market_cap"].as_f64();

        Ok(PriceData {
            symbol: symbol.to_uppercase(),
            price_usd: price,
            volume_24h,
            market_cap,
            price_change_24h,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinGecko,
        })
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        let coin_id = self.symbol_to_coin_id(symbol);

        let mut url = format!(
            "{}/coins/{}/market_chart?vs_currency=usd&days={}",
            self.config.base_url, coin_id, days
        );

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("&x_cg_demo_api_key={}", key));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CoinGecko));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let prices = data["prices"]
            .as_array()
            .ok_or_else(|| ApiError::ApiError("Prices data not found".to_string()))?;

        let mut historical_data = Vec::new();

        for price_point in prices {
            if let Some(timestamp) = price_point[0].as_i64() {
                let price = price_point[1].as_f64().unwrap_or(0.0);

                historical_data.push(HistoricalData {
                    symbol: symbol.to_uppercase(),
                    timestamp: chrono::DateTime::from_timestamp(timestamp / 1000, 0)
                        .unwrap_or_else(|| chrono::Utc::now()),
                    open: price,
                    high: price,
                    low: price,
                    close: price,
                    volume: None, // CoinGecko basic plan doesn't include volume in historical
                    source: ApiProvider::CoinGecko,
                });
            }
        }

        Ok(historical_data)
    }

    async fn get_global_market_data(&self) -> Result<GlobalMarketData, ApiError> {
        let mut url = format!("{}/global", self.config.base_url);

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("?x_cg_pro_api_key={}", key));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CoinGecko));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let global_data = data["data"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Global data not found".to_string()))?;

        let total_market_cap = global_data["total_market_cap"]["usd"]
            .as_f64()
            .unwrap_or(0.0);
        let total_volume_24h = global_data["total_volume"]["usd"].as_f64().unwrap_or(0.0);
        let market_cap_change_percentage_24h = global_data["market_cap_change_percentage_24h_usd"]
            .as_f64()
            .unwrap_or(0.0);
        let active_cryptocurrencies = global_data["active_cryptocurrencies"].as_u64().unwrap_or(0);

        Ok(GlobalMarketData {
            total_market_cap_usd: total_market_cap,
            total_volume_24h_usd: total_volume_24h,
            market_cap_change_percentage_24h: market_cap_change_percentage_24h,
            active_cryptocurrencies,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinGecko,
        })
    }

    async fn is_available(&self) -> bool {
        let mut url = format!("{}/ping", self.config.base_url);

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("?x_cg_pro_api_key={}", key));
        }

        self.client.get(&url).send().await.is_ok()
    }

    fn rate_limit(&self) -> u32 {
        self.config.rate_limit
    }

    fn config(&self) -> &ApiConfig {
        &self.config
    }
}

/// CoinMarketCap API Implementation
/// Paid API with comprehensive data
/// Base URL: https://pro-api.coinmarketcap.com/v1
pub struct CoinMarketCapApi {
    client: reqwest::Client,
    config: ApiConfig,
}

impl CoinMarketCapApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            config: ApiConfig::coinmarketcap_default(),
        }
    }

    /// Convert symbol to CoinMarketCap ID
    fn symbol_to_cmc_id(&self, symbol: &str) -> Result<String, ApiError> {
        // For simplicity, we'll use symbols directly since CMC supports symbol-based queries
        // In production, you might want to maintain a mapping or use CMC's ID mapping endpoint
        Ok(symbol.to_uppercase())
    }
}

#[async_trait::async_trait]
impl CryptoApi for CoinMarketCapApi {
    fn provider(&self) -> ApiProvider {
        ApiProvider::CoinMarketCap
    }

    async fn get_price(&self, symbol: &str) -> Result<PriceData, ApiError> {
        if self.config.api_key.is_none() {
            return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
        }

        let url = format!(
            "{}/cryptocurrency/quotes/latest?symbol={}&convert=USD",
            self.config.base_url,
            symbol.to_uppercase()
        );

        let response = self
            .client
            .get(&url)
            .header(
                "X-CMC_PRO_API_KEY",
                self.config.api_key.as_ref().ok_or_else(|| {
                    ApiError::ApiError("CoinMarketCap API key not configured".to_string())
                })?,
            )
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
            }
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CoinMarketCap));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let data_obj = data["data"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Data not found in response".to_string()))?;

        let symbol_data = data_obj[&symbol.to_uppercase()]
            .as_object()
            .ok_or_else(|| ApiError::NotFound(format!("Symbol not found: {}", symbol)))?;

        let quote = symbol_data["quote"]["USD"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("USD quote not found".to_string()))?;

        let price = quote["price"]
            .as_f64()
            .ok_or_else(|| ApiError::ApiError("Price not found".to_string()))?;

        let volume_24h = quote["volume_24h"].as_f64();
        let market_cap = quote["market_cap"].as_f64();
        let percent_change_24h = quote["percent_change_24h"].as_f64();

        Ok(PriceData {
            symbol: symbol.to_uppercase(),
            price_usd: price,
            volume_24h,
            market_cap,
            price_change_24h: percent_change_24h,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinMarketCap,
        })
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        if self.config.api_key.is_none() {
            return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
        }

        let url = format!("{}/cryptocurrency/quotes/historical?symbol={}&convert=USD&time_start={}&time_end={}&interval=daily",
            self.config.base_url,
            symbol.to_uppercase(),
            (chrono::Utc::now() - chrono::Duration::days(days as i64)).timestamp(),
            chrono::Utc::now().timestamp()
        );

        let response = self
            .client
            .get(&url)
            .header(
                "X-CMC_PRO_API_KEY",
                self.config.api_key.as_ref().ok_or_else(|| {
                    ApiError::ApiError("CoinMarketCap API key not configured".to_string())
                })?,
            )
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
            }
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CoinMarketCap));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let quotes = data["data"]["quotes"]
            .as_array()
            .ok_or_else(|| ApiError::ApiError("Quotes data not found".to_string()))?;

        let mut historical_data = Vec::new();

        for quote in quotes {
            if let Some(timestamp_str) = quote["timestamp"].as_str() {
                if let Ok(timestamp) = chrono::DateTime::parse_from_rfc3339(timestamp_str) {
                    let price_data = quote["quote"]["USD"]
                        .as_object()
                        .ok_or_else(|| ApiError::ApiError("USD quote not found".to_string()))?;

                    let open = price_data["price"].as_f64().unwrap_or(0.0);
                    let high = price_data["price"].as_f64().unwrap_or(0.0);
                    let low = price_data["price"].as_f64().unwrap_or(0.0);
                    let close = price_data["price"].as_f64().unwrap_or(0.0);
                    let volume = price_data["volume_24h"].as_f64();

                    historical_data.push(HistoricalData {
                        symbol: symbol.to_uppercase(),
                        timestamp: timestamp.with_timezone(&chrono::Utc),
                        open,
                        high,
                        low,
                        close,
                        volume,
                        source: ApiProvider::CoinMarketCap,
                    });
                }
            }
        }

        Ok(historical_data)
    }

    async fn get_global_market_data(&self) -> Result<GlobalMarketData, ApiError> {
        if self.config.api_key.is_none() {
            return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
        }

        let url = format!("{}/global-metrics/quotes/latest", self.config.base_url);

        let response = self
            .client
            .get(&url)
            .header(
                "X-CMC_PRO_API_KEY",
                self.config.api_key.as_ref().ok_or_else(|| {
                    ApiError::ApiError("CoinMarketCap API key not configured".to_string())
                })?,
            )
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(ApiError::InvalidApiKey(ApiProvider::CoinMarketCap));
            }
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CoinMarketCap));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let metrics = data["data"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("Global metrics not found".to_string()))?;

        let quote = metrics["quote"]["USD"]
            .as_object()
            .ok_or_else(|| ApiError::ApiError("USD quote not found".to_string()))?;

        let total_market_cap = quote["total_market_cap"].as_f64().unwrap_or(0.0);
        let total_volume_24h = quote["total_volume_24h"].as_f64().unwrap_or(0.0);
        let market_cap_change_percentage_24h = quote
            ["total_market_cap_yesterday_percentage_change"]
            .as_f64()
            .unwrap_or(0.0);
        let active_cryptocurrencies = metrics["active_cryptocurrencies"].as_u64().unwrap_or(0);

        Ok(GlobalMarketData {
            total_market_cap_usd: total_market_cap,
            total_volume_24h_usd: total_volume_24h,
            market_cap_change_percentage_24h: market_cap_change_percentage_24h,
            active_cryptocurrencies,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CoinMarketCap,
        })
    }

    async fn is_available(&self) -> bool {
        if self.config.api_key.is_none() {
            return false;
        }

        let url = format!("{}/cryptocurrency/map", self.config.base_url);
        let result = self
            .client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", self.config.api_key.as_ref().unwrap())
            .send()
            .await;

        result.is_ok()
    }

    fn rate_limit(&self) -> u32 {
        self.config.rate_limit
    }

    fn config(&self) -> &ApiConfig {
        &self.config
    }
}

/// CryptoCompare API Implementation
/// Paid API with real-time data
/// Base URL: https://min-api.cryptocompare.com/data
pub struct CryptoCompareApi {
    client: reqwest::Client,
    config: ApiConfig,
}

impl CryptoCompareApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(15)) // Shorter timeout for real-time data
                .build()
                .expect("Failed to create HTTP client"),
            config: ApiConfig::cryptocompare_default(),
        }
    }
}

#[async_trait::async_trait]
impl CryptoApi for CryptoCompareApi {
    fn provider(&self) -> ApiProvider {
        ApiProvider::CryptoCompare
    }

    async fn get_price(&self, symbol: &str) -> Result<PriceData, ApiError> {
        let mut url = format!(
            "{}/price?fsym={}&tsyms=USD",
            self.config.base_url,
            symbol.to_uppercase()
        );

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("&api_key={}", key));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(ApiError::InvalidApiKey(ApiProvider::CryptoCompare));
            }
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CryptoCompare));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let price = data["USD"]
            .as_f64()
            .ok_or_else(|| ApiError::ApiError("USD price not found".to_string()))?;

        // CryptoCompare doesn't provide volume/market cap in basic price endpoint
        // Would need separate calls for that data
        Ok(PriceData {
            symbol: symbol.to_uppercase(),
            price_usd: price,
            volume_24h: None,
            market_cap: None,
            price_change_24h: None,
            last_updated: chrono::Utc::now(),
            source: ApiProvider::CryptoCompare,
        })
    }

    async fn get_historical_data(
        &self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<HistoricalData>, ApiError> {
        let mut url = format!(
            "{}/histoday?fsym={}&tsym=USD&limit={}&aggregate=1",
            self.config.base_url,
            symbol.to_uppercase(),
            days
        );

        // Add API key if available
        if let Some(key) = &self.config.api_key {
            url.push_str(&format!("&api_key={}", key));
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::Http(e))?;

        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::UNAUTHORIZED {
                return Err(ApiError::InvalidApiKey(ApiProvider::CryptoCompare));
            }
            if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(ApiError::RateLimitExceeded(ApiProvider::CryptoCompare));
            }
            return Err(ApiError::ApiError(format!(
                "API returned status: {} (Provider: {:?})",
                response.status(),
                self.provider()
            )));
        }

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ApiError::NetworkError(format!("Failed to parse JSON response: {}", e)))?;

        let data_array = data["Data"]["Data"]
            .as_array()
            .ok_or_else(|| ApiError::ApiError("Historical data not found".to_string()))?;

        let mut historical_data = Vec::new();

        for item in data_array {
            if let Some(timestamp) = item["time"].as_i64() {
                let open = item["open"].as_f64().unwrap_or(0.0);
                let high = item["high"].as_f64().unwrap_or(0.0);
                let low = item["low"].as_f64().unwrap_or(0.0);
                let close = item["close"].as_f64().unwrap_or(0.0);
                let volume = item["volumeto"].as_f64();

                historical_data.push(HistoricalData {
                    symbol: symbol.to_uppercase(),
                    timestamp: chrono::DateTime::from_timestamp(timestamp, 0)
                        .unwrap_or_else(|| chrono::Utc::now()),
                    open,
                    high,
                    low,
                    close,
                    volume,
                    source: ApiProvider::CryptoCompare,
                });
            }
        }

        Ok(historical_data)
    }

    async fn get_global_market_data(&self) -> Result<GlobalMarketData, ApiError> {
        // CryptoCompare doesn't have a direct global market endpoint
        // This is a limitation of their API
        Err(ApiError::UnsupportedOperation(ApiProvider::CryptoCompare))
    }

    async fn is_available(&self) -> bool {
        let url = format!("{}/price?fsym=BTC&tsyms=USD", self.config.base_url);
        self.client.get(&url).send().await.is_ok()
    }

    fn rate_limit(&self) -> u32 {
        self.config.rate_limit
    }

    fn config(&self) -> &ApiConfig {
        &self.config
    }
}
