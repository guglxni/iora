use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::timeout;

use crate::modules::fetcher::{MultiApiClient, ApiError};
use crate::modules::cache::IntelligentCache;
use crate::modules::processor::DataProcessor;
use crate::modules::rag::RagSystem;
use crate::modules::historical::HistoricalDataManager;

/// Configuration for resilience testing
#[derive(Debug, Clone)]
pub struct ResilienceTestConfig {
    pub test_duration_seconds: u64,
    pub failure_injection_enabled: bool,
    pub circuit_breaker_enabled: bool,
    pub retry_attempts: u32,
    pub timeout_duration_seconds: u64,
    pub recovery_delay_ms: u64,
}

/// Types of failure scenarios to test
#[derive(Debug, Clone)]
pub enum FailureScenario {
    ApiTimeout,
    ApiFailure,
    NetworkFailure,
    RateLimitExceeded,
    ServiceUnavailable,
    PartialFailure,
    DataCorruption,
    ResourceExhaustion,
}

/// Circuit breaker states
#[derive(Debug, Clone, Copy)]
pub enum CircuitBreakerState {
    Closed,     // Normal operation
    Open,       // Failure threshold exceeded, requests blocked
    HalfOpen,   // Testing if service recovered
}

/// Circuit breaker implementation
#[derive(Debug)]
pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitBreakerState>>,
    failure_count: Arc<Mutex<u32>>,
    success_count: Arc<Mutex<u32>>,
    next_attempt_time: Arc<Mutex<Option<Instant>>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
    success_threshold: u32,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration, success_threshold: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitBreakerState::Closed)),
            failure_count: Arc::new(Mutex::new(0)),
            success_count: Arc::new(Mutex::new(0)),
            next_attempt_time: Arc::new(Mutex::new(None)),
            failure_threshold,
            recovery_timeout,
            success_threshold,
        }
    }

    pub async fn call<F, Fut, T>(&self, operation: F) -> Result<T, ResilienceError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, ResilienceError>>,
    {
        let current_state = self.state.lock().await.clone();

        match current_state {
            CircuitBreakerState::Open => {
                let next_attempt = self.next_attempt_time.lock().await;
                if let Some(attempt_time) = *next_attempt {
                    if Instant::now() < attempt_time {
                        return Err(ResilienceError::CircuitBreakerOpen);
                    }
                }
                // Move to half-open state
                *self.state.lock().await = CircuitBreakerState::HalfOpen;
            }
            CircuitBreakerState::HalfOpen => {
                // Allow request to test recovery
            }
            CircuitBreakerState::Closed => {
                // Normal operation
            }
        }

        // Execute the operation
        let result = operation().await;

        match result {
            Ok(value) => {
                self.record_success().await;
                Ok(value)
            }
            Err(error) => {
                self.record_failure().await;
                Err(error)
            }
        }
    }

    async fn record_success(&self) {
        // Get current state
        let current_state = {
            let state = self.state.lock().await;
            *state
        };

        match current_state {
            CircuitBreakerState::HalfOpen => {
                // Increment success count
                {
                    let mut success_count = self.success_count.lock().await;
                    *success_count += 1;
                }

                // Check if we should close the circuit
                let should_close = {
                    let success_count = self.success_count.lock().await;
                    *success_count >= self.success_threshold
                };

                if should_close {
                    // Recovery successful, close circuit and reset everything
                    {
                        let mut state = self.state.lock().await;
                        *state = CircuitBreakerState::Closed;
                    }
                    {
                        let mut failure_count = self.failure_count.lock().await;
                        *failure_count = 0;
                    }
                    {
                        let mut success_count = self.success_count.lock().await;
                        *success_count = 0;
                    }
                    {
                        let mut next_attempt = self.next_attempt_time.lock().await;
                        *next_attempt = None;
                    }
                }
            }
            CircuitBreakerState::Closed => {
                // Reset counters on success
                {
                    let mut failure_count = self.failure_count.lock().await;
                    *failure_count = 0;
                }
                {
                    let mut success_count = self.success_count.lock().await;
                    *success_count = 0;
                }
            }
            CircuitBreakerState::Open => {
                // Should not happen, but reset if it does
                {
                    let mut state = self.state.lock().await;
                    *state = CircuitBreakerState::Closed;
                }
            }
        }
    }

    async fn record_failure(&self) {
        // Increment failure count
        {
            let mut failure_count = self.failure_count.lock().await;
            *failure_count += 1;
        }

        // Check if we need to open the circuit
        let should_open = {
            let failure_count = self.failure_count.lock().await;
            *failure_count >= self.failure_threshold
        };

        if should_open {
            // Update state and related fields separately to avoid deadlocks
            {
                let mut state = self.state.lock().await;
                *state = CircuitBreakerState::Open;
            }
            {
                let mut next_attempt = self.next_attempt_time.lock().await;
                *next_attempt = Some(Instant::now() + self.recovery_timeout);
            }
            {
                let mut success_count = self.success_count.lock().await;
                *success_count = 0;
            }
        }
    }
}

/// Resilience testing results
#[derive(Debug)]
pub struct ResilienceTestResults {
    pub test_scenario: String,
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub timeout_operations: u64,
    pub circuit_breaker_trips: u64,
    pub recovery_time_ms: Option<u64>,
    pub error_distribution: HashMap<String, u64>,
    pub start_time: Instant,
    pub end_time: Instant,
}

/// Error types for resilience testing
#[derive(Debug, Clone)]
pub enum ResilienceError {
    ApiTimeout,
    ApiFailure(String),
    NetworkFailure,
    RateLimitExceeded,
    ServiceUnavailable,
    CircuitBreakerOpen,
    PartialFailure,
    DataCorruption,
    ResourceExhaustion,
    Unknown(String),
}

impl From<ApiError> for ResilienceError {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::Timeout(_) => ResilienceError::ApiTimeout,
            ApiError::RateLimit(_) => ResilienceError::RateLimitExceeded,
            _ => ResilienceError::ApiFailure(format!("API error: {:?}", error)),
        }
    }
}

impl std::fmt::Display for ResilienceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResilienceError::ApiTimeout => write!(f, "API request timed out"),
            ResilienceError::ApiFailure(msg) => write!(f, "API failure: {}", msg),
            ResilienceError::NetworkFailure => write!(f, "Network failure"),
            ResilienceError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ResilienceError::ServiceUnavailable => write!(f, "Service unavailable"),
            ResilienceError::CircuitBreakerOpen => write!(f, "Circuit breaker is open"),
            ResilienceError::PartialFailure => write!(f, "Partial failure occurred"),
            ResilienceError::DataCorruption => write!(f, "Data corruption detected"),
            ResilienceError::ResourceExhaustion => write!(f, "Resource exhaustion"),
            ResilienceError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ResilienceError {}

/// Core resilience testing engine
pub struct ResilienceTestingEngine {
    api_client: Arc<MultiApiClient>,
    cache: Arc<IntelligentCache>,
    processor: Arc<DataProcessor>,
    rag_system: Option<Arc<RagSystem>>,
    historical_manager: Arc<HistoricalDataManager>,
    config: ResilienceTestConfig,
    circuit_breaker: Option<CircuitBreaker>,
    results: Arc<Mutex<Vec<ResilienceTestResults>>>,
}

impl ResilienceTestingEngine {
    pub fn new(
        api_client: Arc<MultiApiClient>,
        cache: Arc<IntelligentCache>,
        processor: Arc<DataProcessor>,
        rag_system: Option<Arc<RagSystem>>,
        historical_manager: Arc<HistoricalDataManager>,
        config: ResilienceTestConfig,
    ) -> Self {
        let circuit_breaker = if config.circuit_breaker_enabled {
            Some(CircuitBreaker::new(
                5,  // failure threshold
                Duration::from_secs(30),  // recovery timeout
                3,  // success threshold
            ))
        } else {
            None
        };

        Self {
            api_client,
            cache,
            processor,
            rag_system,
            historical_manager,
            config,
            circuit_breaker,
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Run comprehensive resilience test suite
    pub async fn run_comprehensive_resilience_test(&self) -> Result<ResilienceTestResults, ResilienceError> {
        println!("🔄 Starting comprehensive resilience test...");

        let start_time = Instant::now();
        let mut total_operations = 0u64;
        let mut successful_operations = 0u64;
        let mut failed_operations = 0u64;
        let mut timeout_operations = 0u64;
        let mut circuit_breaker_trips = 0u64;
        let mut error_distribution = HashMap::new();

        println!("📋 Test scenarios: {}", 6); // Number of scenarios

        let scenarios = vec![
            ("api_timeout_test", FailureScenario::ApiTimeout),
            ("api_failure_test", FailureScenario::ApiFailure),
            ("network_failure_test", FailureScenario::NetworkFailure),
            ("rate_limit_test", FailureScenario::RateLimitExceeded),
            ("service_unavailable_test", FailureScenario::ServiceUnavailable),
            ("partial_failure_test", FailureScenario::PartialFailure),
        ];

        for (scenario_name, scenario) in scenarios {
            println!("🧪 Running scenario: {}", scenario_name);

            // Add timeout to each scenario to prevent hanging
            let scenario_start = Instant::now();
            let scenario_result = tokio::time::timeout(
                Duration::from_secs(self.config.test_duration_seconds / 2), // Half the total time per scenario
                self.run_failure_scenario(scenario)
            ).await;

            let scenario_duration = scenario_start.elapsed();
            println!("⏱️  Scenario {} completed in {:.2}s", scenario_name, scenario_duration.as_secs_f64());

            match scenario_result {
                Ok(result) => {
                    match result {
                        Ok((success, fail, timeout, trips, errors)) => {
                            total_operations += success + fail + timeout;
                            successful_operations += success;
                            failed_operations += fail;
                            timeout_operations += timeout;
                            circuit_breaker_trips += trips;

                            for (error_type, count) in errors {
                                *error_distribution.entry(error_type).or_insert(0) += count;
                            }
                        }
                        Err(error) => {
                            *error_distribution.entry(format!("scenario_error_{:?}", error)).or_insert(0) += 1;
                            failed_operations += 1;
                        }
                    }
                }
                Err(_) => {
                    // Scenario timed out
                    *error_distribution.entry("scenario_timeout".to_string()).or_insert(0) += 1;
                    failed_operations += 1;
                }
            }
        }

        let end_time = Instant::now();
        let total_duration = end_time.duration_since(start_time);

        println!("✅ Comprehensive resilience test completed in {:.2}s", total_duration.as_secs_f64());
        println!("📊 Final results: {} total operations", total_operations);

        Ok(ResilienceTestResults {
            test_scenario: "comprehensive_resilience_test".to_string(),
            total_operations,
            successful_operations,
            failed_operations,
            timeout_operations,
            circuit_breaker_trips,
            recovery_time_ms: None,
            error_distribution,
            start_time,
            end_time,
        })
    }

    /// Run specific failure scenario test
    async fn run_failure_scenario(&self, scenario: FailureScenario) -> Result<(u64, u64, u64, u64, HashMap<String, u64>), ResilienceError> {
        let mut successful = 0u64;
        let mut failed = 0u64;
        let timeout_count = 0u64;
        let mut circuit_trips = 0u64;
        let mut error_dist = HashMap::new();

        let operations = vec![
            "price_fetch".to_string(),
            "historical_data".to_string(),
            "cache_operation".to_string(),
            "search_query".to_string(),
        ];

        for operation in operations {
            let result = self.execute_operation_with_failure_injection(&operation, &scenario).await;

            match result {
                Ok(_) => successful += 1,
                Err(error) => {
                    failed += 1;
                    let error_key = match error {
                        ResilienceError::ApiTimeout => "api_timeout",
                        ResilienceError::CircuitBreakerOpen => "circuit_breaker_open",
                        ResilienceError::ApiFailure(_) => "api_failure",
                        ResilienceError::NetworkFailure => "network_failure",
                        ResilienceError::RateLimitExceeded => "rate_limit",
                        ResilienceError::ServiceUnavailable => "service_unavailable",
                        _ => "other_error",
                    };
                    *error_dist.entry(error_key.to_string()).or_insert(0) += 1;

                    if matches!(error, ResilienceError::CircuitBreakerOpen) {
                        circuit_trips += 1;
                    }
                }
            }
        }

        Ok((successful, failed, timeout_count, circuit_trips, error_dist))
    }

    /// Execute operation with failure injection
    async fn execute_operation_with_failure_injection(
        &self,
        operation: &str,
        scenario: &FailureScenario,
    ) -> Result<(), ResilienceError> {
        let operation_fn = || async {
            match operation {
                "price_fetch" => self.test_price_fetch_with_failure(scenario).await,
                "historical_data" => self.test_historical_data_with_failure(scenario).await,
                "cache_operation" => self.test_cache_operation_with_failure(scenario).await,
                "search_query" => self.test_search_query_with_failure(scenario).await,
                _ => Err(ResilienceError::Unknown("Unknown operation".to_string())),
            }
        };

        if let Some(circuit_breaker) = &self.circuit_breaker {
            circuit_breaker.call(operation_fn).await
        } else {
            operation_fn().await
        }
    }

    /// Test price fetch with failure injection
    async fn test_price_fetch_with_failure(&self, scenario: &FailureScenario) -> Result<(), ResilienceError> {
        println!("🧪 Testing price fetch with scenario: {:?}", scenario);

        // Inject failure based on scenario
        match scenario {
            FailureScenario::ApiTimeout => {
                // Force timeout by using very short timeout
                println!("⏰ Testing API timeout with 1ms timeout...");
                let start = Instant::now();
                let result = timeout(Duration::from_millis(1), self.api_client.get_price_intelligent("BTC")).await;
                let duration = start.elapsed();
                println!("⏱️  API timeout test took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(Ok(_)) => {
                        println!("✅ API call succeeded unexpectedly");
                        Ok(())
                    },
                    Ok(Err(e)) => {
                        println!("❌ API call failed: {:?}", e);
                        Err(ResilienceError::from(e))
                    },
                    Err(_) => {
                        println!("⏰ API call timed out as expected");
                        Err(ResilienceError::ApiTimeout)
                    },
                }
            }
            FailureScenario::ApiFailure => {
                // This will naturally fail if API is not available
                println!("🔥 Testing API failure with invalid symbol...");
                let start = Instant::now();
                let result = self.api_client.get_price_intelligent("INVALID_SYMBOL").await;
                let duration = start.elapsed();
                println!("⏱️  API failure test took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(_) => {
                        println!("✅ Invalid symbol call succeeded unexpectedly");
                        Ok(())
                    },
                    Err(e) => {
                        println!("❌ Invalid symbol call failed as expected: {:?}", e);
                        Err(ResilienceError::from(e))
                    }
                }
            }
            FailureScenario::NetworkFailure => {
                // Simulate network failure with a delay to make it realistic
                println!("🌐 Simulating network failure...");
                tokio::time::sleep(Duration::from_millis(100)).await;
                println!("❌ Network failure simulated");
                Err(ResilienceError::NetworkFailure)
            }
            FailureScenario::RateLimitExceeded => {
                // Simulate rate limit with realistic delay
                println!("🚦 Simulating rate limit exceeded...");
                tokio::time::sleep(Duration::from_millis(200)).await;
                println!("❌ Rate limit exceeded simulated");
                Err(ResilienceError::RateLimitExceeded)
            }
            FailureScenario::ServiceUnavailable => {
                // Simulate service unavailable with delay
                println!("🚫 Simulating service unavailable...");
                tokio::time::sleep(Duration::from_millis(150)).await;
                println!("❌ Service unavailable simulated");
                Err(ResilienceError::ServiceUnavailable)
            }
            _ => {
                // Normal operation - test real API call
                println!("✅ Testing normal price fetch operation...");
                let start = Instant::now();
                let result = timeout(Duration::from_secs(self.config.timeout_duration_seconds), self.api_client.get_price_intelligent("BTC")).await;
                let duration = start.elapsed();
                println!("⏱️  Normal API call took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(Ok(_)) => {
                        println!("✅ Normal API call succeeded");
                        Ok(())
                    },
                    Ok(Err(e)) => {
                        println!("❌ Normal API call failed: {:?}", e);
                        Err(ResilienceError::from(e))
                    },
                    Err(_) => {
                        println!("⏰ Normal API call timed out");
                        Err(ResilienceError::ApiTimeout)
                    },
                }
            }
        }
    }

    /// Test historical data fetch with failure injection
    async fn test_historical_data_with_failure(&self, scenario: &FailureScenario) -> Result<(), ResilienceError> {
        println!("📊 Testing historical data fetch with scenario: {:?}", scenario);

        match scenario {
            FailureScenario::ApiTimeout => {
                println!("⏰ Testing historical data timeout with 1ms timeout...");
                let start = Instant::now();
                let result = timeout(Duration::from_millis(1), self.api_client.get_historical_data_intelligent("BTC", 7)).await;
                let duration = start.elapsed();
                println!("⏱️  Historical data timeout test took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(Ok(_)) => {
                        println!("✅ Historical data call succeeded unexpectedly");
                        Ok(())
                    },
                    Ok(Err(e)) => {
                        println!("❌ Historical data call failed: {:?}", e);
                        Err(ResilienceError::from(e))
                    },
                    Err(_) => {
                        println!("⏰ Historical data call timed out as expected");
                        Err(ResilienceError::ApiTimeout)
                    },
                }
            }
            FailureScenario::ApiFailure => {
                println!("🔥 Testing historical data failure with invalid symbol...");
                let start = Instant::now();
                let result = self.api_client.get_historical_data_intelligent("INVALID_SYMBOL", 7).await;
                let duration = start.elapsed();
                println!("⏱️  Historical data failure test took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(_) => {
                        println!("✅ Invalid symbol historical call succeeded unexpectedly");
                        Ok(())
                    },
                    Err(e) => {
                        println!("❌ Invalid symbol historical call failed as expected: {:?}", e);
                        Err(ResilienceError::from(e))
                    }
                }
            }
            FailureScenario::NetworkFailure => {
                println!("🌐 Simulating network failure for historical data...");
                tokio::time::sleep(Duration::from_millis(120)).await;
                println!("❌ Network failure simulated for historical data");
                Err(ResilienceError::NetworkFailure)
            }
            FailureScenario::RateLimitExceeded => {
                println!("🚦 Simulating rate limit for historical data...");
                tokio::time::sleep(Duration::from_millis(180)).await;
                println!("❌ Rate limit exceeded simulated for historical data");
                Err(ResilienceError::RateLimitExceeded)
            }
            FailureScenario::ServiceUnavailable => {
                println!("🚫 Simulating service unavailable for historical data...");
                tokio::time::sleep(Duration::from_millis(160)).await;
                println!("❌ Service unavailable simulated for historical data");
                Err(ResilienceError::ServiceUnavailable)
            }
            _ => {
                println!("✅ Testing normal historical data fetch operation...");
                let start = Instant::now();
                let result = timeout(Duration::from_secs(self.config.timeout_duration_seconds), self.api_client.get_historical_data_intelligent("BTC", 7)).await;
                let duration = start.elapsed();
                println!("⏱️  Normal historical data call took {:.3}s", duration.as_secs_f64());

                match result {
                    Ok(Ok(_)) => {
                        println!("✅ Normal historical data call succeeded");
                        Ok(())
                    },
                    Ok(Err(e)) => {
                        println!("❌ Normal historical data call failed: {:?}", e);
                        Err(ResilienceError::from(e))
                    },
                    Err(_) => {
                        println!("⏰ Normal historical data call timed out");
                        Err(ResilienceError::ApiTimeout)
                    },
                }
            }
        }
    }

    /// Test cache operation with failure injection
    async fn test_cache_operation_with_failure(&self, scenario: &FailureScenario) -> Result<(), ResilienceError> {
        println!("💾 Testing cache operation with scenario: {:?}", scenario);

        match scenario {
            FailureScenario::ResourceExhaustion => {
                println!("💥 Simulating resource exhaustion in cache...");
                tokio::time::sleep(Duration::from_millis(300)).await;
                println!("❌ Resource exhaustion simulated in cache");
                Err(ResilienceError::ResourceExhaustion)
            }
            FailureScenario::DataCorruption => {
                println!("🔄 Simulating data corruption in cache...");
                tokio::time::sleep(Duration::from_millis(250)).await;
                println!("❌ Data corruption simulated in cache");
                Err(ResilienceError::DataCorruption)
            }
            _ => {
                // Normal cache operation
                println!("✅ Testing normal cache operation...");
                let start = Instant::now();
                let stats = self.cache.get_stats();
                let duration = start.elapsed();
                println!("⏱️  Cache stats retrieval took {:.3}s", duration.as_secs_f64());
                println!("📊 Cache stats: {:?}", stats);
                Ok(())
            }
        }
    }

    /// Test search query with failure injection
    async fn test_search_query_with_failure(&self, scenario: &FailureScenario) -> Result<(), ResilienceError> {
        println!("🔍 Testing search query with scenario: {:?}", scenario);

        if let Some(rag) = &self.rag_system {
            match scenario {
                FailureScenario::ApiTimeout => {
                    println!("⏰ Testing search timeout with 1ms timeout...");
                    let start = Instant::now();
                    let result = timeout(Duration::from_millis(1), rag.search_historical_data("bitcoin price", 5)).await;
                    let duration = start.elapsed();
                    println!("⏱️  Search timeout test took {:.3}s", duration.as_secs_f64());

                    match result {
                        Ok(Ok(_)) => {
                            println!("✅ Search call succeeded unexpectedly");
                            Ok(())
                        },
                        Ok(Err(e)) => {
                            println!("❌ Search call failed: {:?}", e);
                            Err(ResilienceError::ApiFailure(format!("Search failed: {:?}", e)))
                        },
                        Err(_) => {
                            println!("⏰ Search call timed out as expected");
                            Err(ResilienceError::ApiTimeout)
                        },
                    }
                }
                FailureScenario::ServiceUnavailable => {
                    println!("🚫 Simulating search service unavailable...");
                    tokio::time::sleep(Duration::from_millis(140)).await;
                    println!("❌ Search service unavailable simulated");
                    Err(ResilienceError::ServiceUnavailable)
                }
                _ => {
                    println!("✅ Testing normal search query operation...");
                    let start = Instant::now();
                    let result = timeout(Duration::from_secs(self.config.timeout_duration_seconds), rag.search_historical_data("bitcoin price", 5)).await;
                    let duration = start.elapsed();
                    println!("⏱️  Normal search query took {:.3}s", duration.as_secs_f64());

                    match result {
                        Ok(Ok(_)) => {
                            println!("✅ Normal search query succeeded");
                            Ok(())
                        },
                        Ok(Err(e)) => {
                            println!("❌ Normal search query failed: {:?}", e);
                            Err(ResilienceError::ApiFailure(format!("Search failed: {:?}", e)))
                        },
                        Err(_) => {
                            println!("⏰ Normal search query timed out");
                            Err(ResilienceError::ApiTimeout)
                        },
                    }
                }
            }
        } else {
            // RAG system not available
            println!("⚠️  RAG system not available for search testing");
            Err(ResilienceError::ServiceUnavailable)
        }
    }

    /// Export test results to JSON
    pub async fn export_results_to_json(&self, results: &ResilienceTestResults, filename: &str) -> Result<(), ResilienceError> {
        let json_data = serde_json::json!({
            "test_scenario": results.test_scenario,
            "total_operations": results.total_operations,
            "successful_operations": results.successful_operations,
            "failed_operations": results.failed_operations,
            "timeout_operations": results.timeout_operations,
            "circuit_breaker_trips": results.circuit_breaker_trips,
            "recovery_time_ms": results.recovery_time_ms,
            "error_distribution": results.error_distribution,
            "duration_ms": results.end_time.duration_since(results.start_time).as_millis(),
            "success_rate": if results.total_operations > 0 {
                (results.successful_operations as f64 / results.total_operations as f64) * 100.0
            } else {
                0.0
            }
        });

        tokio::fs::write(filename, serde_json::to_string_pretty(&json_data).unwrap())
            .await
            .map_err(|_| ResilienceError::Unknown("Failed to write results file".to_string()))?;

        Ok(())
    }
}
