//! Usage Analytics Module (Task 2.3.1)
//!
//! This module provides comprehensive API usage analytics including:
//! - Real-time usage tracking and reporting
//! - Cost analysis for different API combinations
//! - Performance metrics dashboard
//! - Usage optimization recommendations
//! - Concurrent analytics processing

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::modules::fetcher::{ApiProvider, MultiApiClient};

/// Analytics time window for rolling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeWindow {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

/// API usage metrics for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsageMetrics {
    pub provider: ApiProvider,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_response_time: Duration,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub total_cost: f64,
    pub cost_per_request: f64,
    pub last_updated: DateTime<Utc>,
    pub error_types: HashMap<String, u64>,
}

/// Performance metrics for dashboard display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub overall_success_rate: f64,
    pub average_response_time: Duration,
    pub total_requests_per_minute: f64,
    pub cost_per_request: f64,
    pub total_cost_per_hour: f64,
    pub most_used_provider: ApiProvider,
    pub least_reliable_provider: Option<ApiProvider>,
    pub fastest_provider: ApiProvider,
    pub timestamp: DateTime<Utc>,
}

/// Cost analysis for different API combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalysis {
    pub combination: Vec<ApiProvider>,
    pub total_cost: f64,
    pub cost_per_request: f64,
    pub cost_efficiency: f64, // Lower is better
    pub reliability_score: f64,
    pub performance_score: f64,
    pub overall_score: f64,
}

/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub recommendation_type: RecommendationType,
    pub description: String,
    pub expected_savings: f64,
    pub expected_improvement: f64,
    pub confidence_score: f64,
    pub implementation_priority: Priority,
}

/// Types of optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    SwitchProvider,
    UseCacheMore,
    ReduceFrequency,
    ChangeCombination,
    UpgradePlan,
    ImplementCircuitBreaker,
}

/// Priority levels for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize)]
pub struct AnalyticsConfig {
    pub max_history_size: usize,
    pub metrics_update_interval: Duration,
    pub cost_update_interval: Duration,
    pub enable_real_time_updates: bool,
    pub retention_period_days: u64,
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            max_history_size: 10000,
            metrics_update_interval: Duration::from_secs(30),
            cost_update_interval: Duration::from_secs(300), // 5 minutes
            enable_real_time_updates: true,
            retention_period_days: 30,
        }
    }
}

/// Main analytics manager
pub struct AnalyticsManager {
    config: AnalyticsConfig,
    usage_metrics: Arc<RwLock<HashMap<ApiProvider, ApiUsageMetrics>>>,
    performance_history: Arc<RwLock<VecDeque<PerformanceMetrics>>>,
    cost_analyses: Arc<RwLock<HashMap<String, CostAnalysis>>>,
    recommendations: Arc<RwLock<Vec<OptimizationRecommendation>>>,
    last_metrics_update: Arc<RwLock<Instant>>,
    last_cost_update: Arc<RwLock<Instant>>,
}

impl AnalyticsManager {
    /// Create a new analytics manager
    pub fn new(config: AnalyticsConfig) -> Self {
        Self {
            config,
            usage_metrics: Arc::new(RwLock::new(HashMap::new())),
            performance_history: Arc::new(RwLock::new(VecDeque::new())),
            cost_analyses: Arc::new(RwLock::new(HashMap::new())),
            recommendations: Arc::new(RwLock::new(Vec::new())),
            last_metrics_update: Arc::new(RwLock::new(Instant::now())),
            last_cost_update: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// Create with default configuration
    pub fn default_config() -> Self {
        Self::new(AnalyticsConfig::default())
    }

    /// Record a successful API request
    pub async fn record_successful_request(
        &self,
        provider: &ApiProvider,
        response_time: Duration,
        estimated_cost: f64,
    ) {
        let mut metrics = self.usage_metrics.write().await;

        let entry = metrics.entry(*provider).or_insert_with(|| ApiUsageMetrics {
            provider: *provider,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_response_time: Duration::ZERO,
            average_response_time: Duration::ZERO,
            min_response_time: Duration::from_secs(u64::MAX),
            max_response_time: Duration::ZERO,
            total_cost: 0.0,
            cost_per_request: 0.0,
            last_updated: Utc::now(),
            error_types: HashMap::new(),
        });

        entry.total_requests += 1;
        entry.successful_requests += 1;
        entry.total_response_time += response_time;
        entry.average_response_time = entry.total_response_time / entry.total_requests as u32;
        entry.min_response_time = entry.min_response_time.min(response_time);
        entry.max_response_time = entry.max_response_time.max(response_time);
        entry.total_cost += estimated_cost;
        entry.cost_per_request = if entry.total_requests > 0 {
            entry.total_cost / entry.total_requests as f64
        } else {
            0.0
        };
        entry.last_updated = Utc::now();
    }

    /// Record a failed API request
    pub async fn record_failed_request(
        &self,
        provider: &ApiProvider,
        response_time: Duration,
        error_type: &str,
        estimated_cost: f64,
    ) {
        let mut metrics = self.usage_metrics.write().await;

        let entry = metrics.entry(*provider).or_insert_with(|| ApiUsageMetrics {
            provider: *provider,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_response_time: Duration::ZERO,
            average_response_time: Duration::ZERO,
            min_response_time: Duration::from_secs(u64::MAX),
            max_response_time: Duration::ZERO,
            total_cost: 0.0,
            cost_per_request: 0.0,
            last_updated: Utc::now(),
            error_types: HashMap::new(),
        });

        entry.total_requests += 1;
        entry.failed_requests += 1;
        entry.total_response_time += response_time;
        entry.average_response_time = entry.total_response_time / entry.total_requests as u32;
        entry.min_response_time = entry.min_response_time.min(response_time);
        entry.max_response_time = entry.max_response_time.max(response_time);
        entry.total_cost += estimated_cost;
        entry.cost_per_request = if entry.total_requests > 0 {
            entry.total_cost / entry.total_requests as f64
        } else {
            0.0
        };
        *entry.error_types.entry(error_type.to_string()).or_insert(0) += 1;
        entry.last_updated = Utc::now();
    }

    /// Get current usage metrics for all providers
    pub async fn get_usage_metrics(&self) -> HashMap<ApiProvider, ApiUsageMetrics> {
        self.usage_metrics.read().await.clone()
    }

    /// Get usage metrics for a specific provider
    pub async fn get_provider_metrics(&self, provider: &ApiProvider) -> Option<ApiUsageMetrics> {
        self.usage_metrics.read().await.get(provider).cloned()
    }

    /// Calculate performance metrics
    pub async fn calculate_performance_metrics(&self) -> PerformanceMetrics {
        let metrics = self.usage_metrics.read().await;
        let now = Utc::now();

        if metrics.is_empty() {
            return PerformanceMetrics {
                overall_success_rate: 0.0,
                average_response_time: Duration::ZERO,
                total_requests_per_minute: 0.0,
                cost_per_request: 0.0,
                total_cost_per_hour: 0.0,
                most_used_provider: ApiProvider::CoinGecko,
                least_reliable_provider: None,
                fastest_provider: ApiProvider::CoinGecko,
                timestamp: now,
            };
        }

        let mut total_requests = 0u64;
        let mut total_successful = 0u64;
        let mut total_response_time = Duration::ZERO;
        let mut total_cost = 0.0;
        let mut fastest_provider = ApiProvider::CoinGecko;
        let mut min_avg_time = Duration::from_secs(u64::MAX);
        let mut most_used_provider = ApiProvider::CoinGecko;
        let mut max_requests = 0u64;
        let mut least_reliable_provider = None;
        let mut min_success_rate = 1.0;

        for (provider, metric) in metrics.iter() {
            total_requests += metric.total_requests;
            total_successful += metric.successful_requests;
            total_response_time += metric.average_response_time * metric.total_requests as u32;
            total_cost += metric.total_cost;

            // Find fastest provider
            if metric.average_response_time < min_avg_time {
                min_avg_time = metric.average_response_time;
                fastest_provider = *provider;
            }

            // Find most used provider
            if metric.total_requests > max_requests {
                max_requests = metric.total_requests;
                most_used_provider = *provider;
            }

            // Find least reliable provider
            if metric.total_requests > 0 {
                let success_rate = metric.successful_requests as f64 / metric.total_requests as f64;
                if success_rate < min_success_rate {
                    min_success_rate = success_rate;
                    least_reliable_provider = Some(*provider);
                }
            }
        }

        let overall_success_rate = if total_requests > 0 {
            total_successful as f64 / total_requests as f64
        } else {
            0.0
        };

        let average_response_time = if total_requests > 0 {
            total_response_time / total_requests as u32
        } else {
            Duration::ZERO
        };

        let total_requests_per_minute = total_requests as f64 / 60.0;
        let cost_per_request = if total_requests > 0 {
            total_cost / total_requests as f64
        } else {
            0.0
        };
        let total_cost_per_hour = total_cost * 2.0; // Rough estimate

        PerformanceMetrics {
            overall_success_rate,
            average_response_time,
            total_requests_per_minute,
            cost_per_request,
            total_cost_per_hour,
            most_used_provider,
            least_reliable_provider,
            fastest_provider,
            timestamp: now,
        }
    }

    /// Generate cost analysis for different API combinations
    pub async fn analyze_costs(&self, _client: &MultiApiClient) -> HashMap<String, CostAnalysis> {
        let metrics = self.usage_metrics.read().await;
        let mut analyses = HashMap::new();

        // Analyze individual providers
        for (provider, metric) in metrics.iter() {
            let combination_name = format!("{:?}", provider);
            let success_rate = if metric.total_requests > 0 {
                metric.successful_requests as f64 / metric.total_requests as f64
            } else {
                0.0
            };

            let analysis = CostAnalysis {
                combination: vec![*provider],
                total_cost: metric.total_cost,
                cost_per_request: if metric.total_requests > 0 {
                    metric.total_cost / metric.total_requests as f64
                } else {
                    0.0
                },
                cost_efficiency: metric.total_cost / (metric.total_requests as f64 + 1.0), // +1 to avoid division by zero
                reliability_score: success_rate,
                performance_score: 1.0 / (metric.average_response_time.as_millis() as f64 + 1.0),
                overall_score: success_rate
                    / (metric.average_response_time.as_millis() as f64 + 1.0),
            };

            analyses.insert(combination_name, analysis);
        }

        // Update the last cost update timestamp
        *self.last_cost_update.write().await = Instant::now();

        analyses
    }

    /// Get the time since last cost update
    pub async fn time_since_last_cost_update(&self) -> Duration {
        let last_update = *self.last_cost_update.read().await;
        last_update.elapsed()
    }

    /// Generate optimization recommendations
    pub async fn generate_recommendations(&self) -> Vec<OptimizationRecommendation> {
        let metrics = self.usage_metrics.read().await;
        let mut local_recommendations = Vec::new();

        if metrics.is_empty() {
            return local_recommendations;
        }

        // Find providers with high failure rates
        for (provider, metric) in metrics.iter() {
            if metric.total_requests > 10 {
                // Only consider providers with significant usage
                let failure_rate = metric.failed_requests as f64 / metric.total_requests as f64;

                if failure_rate > 0.3 {
                    // 30% failure rate
                    local_recommendations.push(OptimizationRecommendation {
                        recommendation_type: RecommendationType::SwitchProvider,
                        description: format!("{:?} has a {:.1}% failure rate. Consider switching to a more reliable provider.", provider, failure_rate * 100.0),
                        expected_savings: metric.total_cost * 0.2, // Estimate 20% cost savings
                        expected_improvement: 0.3, // 30% improvement in reliability
                        confidence_score: 0.8,
                        implementation_priority: Priority::High,
                    });
                }
            }
        }

        // Find expensive providers
        let mut provider_costs: Vec<_> = metrics.iter().collect();
        provider_costs.sort_by(|a, b| {
            b.1.cost_per_request
                .partial_cmp(&a.1.cost_per_request)
                .unwrap()
        });

        if let Some((expensive_provider, expensive_metric)) = provider_costs.first() {
            if expensive_metric.total_requests > 5 {
                local_recommendations.push(OptimizationRecommendation {
                    recommendation_type: RecommendationType::UpgradePlan,
                    description: format!("{:?} has high cost per request (${:.4}). Consider upgrading to a cheaper plan or switching providers.", expensive_provider, expensive_metric.total_cost / expensive_metric.total_requests as f64),
                    expected_savings: expensive_metric.total_cost * 0.5, // Estimate 50% savings
                    expected_improvement: 0.0, // Cost improvement
                    confidence_score: 0.7,
                    implementation_priority: Priority::Medium,
                });
            }
        }

        // Check for cache optimization opportunities
        let total_requests: u64 = metrics.values().map(|m| m.total_requests).sum();
        if total_requests > 100 {
            local_recommendations.push(OptimizationRecommendation {
                recommendation_type: RecommendationType::UseCacheMore,
                description: "High request volume detected. Consider increasing cache TTL and warming cache with popular symbols.".to_string(),
                expected_savings: 0.1 * metrics.values().map(|m| m.total_cost).sum::<f64>(), // 10% savings
                expected_improvement: 0.2, // 20% performance improvement
                confidence_score: 0.9,
                implementation_priority: Priority::Medium,
            });
        }

        local_recommendations.sort_by(|a, b| {
            // Sort by priority first, then by expected savings
            match (
                a.implementation_priority.clone(),
                b.implementation_priority.clone(),
            ) {
                (Priority::Critical, _) => std::cmp::Ordering::Less,
                (_, Priority::Critical) => std::cmp::Ordering::Greater,
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, _) => std::cmp::Ordering::Less,
                (_, Priority::Medium) => std::cmp::Ordering::Greater,
                (Priority::Low, Priority::Low) => {
                    b.expected_savings.partial_cmp(&a.expected_savings).unwrap()
                }
            }
        });

        local_recommendations
    }

    /// Get performance metrics history
    pub async fn get_performance_history(&self, limit: usize) -> Vec<PerformanceMetrics> {
        let history = self.performance_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Update metrics (should be called periodically)
    pub async fn update_metrics(&self) {
        let now = Instant::now();
        let mut last_update = self.last_metrics_update.write().await;

        if now.duration_since(*last_update) >= self.config.metrics_update_interval {
            let performance_metrics = self.calculate_performance_metrics().await;

            let mut history = self.performance_history.write().await;
            history.push_back(performance_metrics);

            // Keep only recent history
            while history.len() > self.config.max_history_size {
                history.pop_front();
            }

            *last_update = now;
        }
    }

    /// Get analytics dashboard data
    pub async fn get_dashboard_data(&self) -> serde_json::Value {
        let usage_metrics = self.get_usage_metrics().await;
        let performance = self.calculate_performance_metrics().await;
        let recommendations = self.generate_recommendations().await;
        let history = self.get_performance_history(10).await;

        serde_json::json!({
            "usage_metrics": usage_metrics,
            "current_performance": performance,
            "recommendations": recommendations,
            "performance_history": history,
            "timestamp": Utc::now()
        })
    }

    /// Export analytics data for external analysis
    pub async fn export_data(&self) -> serde_json::Value {
        let usage_metrics = self.get_usage_metrics().await;
        let cost_analyses = self.cost_analyses.read().await;
        let recommendations = self.generate_recommendations().await;

        serde_json::json!({
            "export_timestamp": Utc::now(),
            "usage_metrics": usage_metrics,
            "cost_analyses": *cost_analyses,
            "recommendations": recommendations,
            "config": self.config
        })
    }

    /// Get health status of analytics system
    pub async fn health_check(&self) -> bool {
        // Check if we can read metrics
        let _metrics = self.usage_metrics.read().await;
        let _history = self.performance_history.read().await;
        true
    }
}

/// Concurrent analytics processor for parallel processing
pub struct ConcurrentAnalyticsProcessor {
    analytics_manager: Arc<AnalyticsManager>,
    workers: usize,
}

impl ConcurrentAnalyticsProcessor {
    pub fn new(analytics_manager: Arc<AnalyticsManager>, workers: usize) -> Self {
        Self {
            analytics_manager,
            workers,
        }
    }

    /// Process analytics concurrently
    pub async fn process_concurrent(&self, data_sources: Vec<ApiProvider>) {
        let mut handles = vec![];

        for chunk in data_sources.chunks((data_sources.len() + self.workers - 1) / self.workers) {
            let chunk = chunk.to_vec();
            let analytics = Arc::clone(&self.analytics_manager);

            let handle = tokio::spawn(async move {
                for provider in chunk {
                    // Simulate processing time
                    tokio::time::sleep(Duration::from_millis(10)).await;

                    // In a real implementation, this would process actual metrics
                    // For now, this demonstrates the concurrent processing structure
                    let _metrics = analytics.get_provider_metrics(&provider).await;
                }
            });

            handles.push(handle);
        }

        // Wait for all concurrent tasks to complete
        for handle in handles {
            let _ = handle.await;
        }
    }
}

/// Integration with MultiApiClient for automatic tracking
impl MultiApiClient {
    // Removed duplicate with_analytics method - already defined in fetcher.rs

    /// Get analytics manager (if available)
    pub fn get_analytics(&self) -> Option<&Arc<AnalyticsManager>> {
        self.analytics_manager.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analytics_creation() {
        let analytics = AnalyticsManager::default();
        assert!(analytics.health_check().await);
    }

    #[tokio::test]
    async fn test_usage_tracking() {
        let analytics = AnalyticsManager::default();

        // Record some usage
        analytics
            .record_successful_request(&ApiProvider::CoinGecko, Duration::from_millis(150), 0.001)
            .await;

        analytics
            .record_failed_request(
                &ApiProvider::CoinPaprika,
                Duration::from_millis(200),
                "timeout",
                0.002,
            )
            .await;

        let metrics = analytics.get_usage_metrics().await;
        assert_eq!(metrics.len(), 2);

        let coingecko_metrics = metrics.get(&ApiProvider::CoinGecko).unwrap();
        assert_eq!(coingecko_metrics.total_requests, 1);
        assert_eq!(coingecko_metrics.successful_requests, 1);
        assert_eq!(coingecko_metrics.failed_requests, 0);
    }

    #[tokio::test]
    async fn test_performance_metrics_calculation() {
        let analytics = AnalyticsManager::default();

        // Add some test data
        analytics
            .record_successful_request(&ApiProvider::CoinGecko, Duration::from_millis(100), 0.001)
            .await;

        analytics
            .record_successful_request(&ApiProvider::CoinPaprika, Duration::from_millis(200), 0.002)
            .await;

        let performance = analytics.calculate_performance_metrics().await;
        assert!(performance.overall_success_rate > 0.0);
        assert!(performance.average_response_time > Duration::ZERO);
    }

    #[tokio::test]
    async fn test_recommendations_generation() {
        let analytics = AnalyticsManager::default();

        // Add data that should trigger recommendations
        for _ in 0..15 {
            analytics
                .record_failed_request(
                    &ApiProvider::CoinGecko,
                    Duration::from_millis(100),
                    "rate_limit",
                    0.001,
                )
                .await;
        }

        let recommendations = analytics.generate_recommendations().await;
        assert!(!recommendations.is_empty());
    }
}
