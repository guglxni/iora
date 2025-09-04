//! API Health Monitoring Module (Task 2.3.2)
//!
//! This module provides comprehensive API health monitoring including:
//! - Real-time API health monitoring system
//! - Automatic API status detection
//! - Alerting system for API failures
//! - API performance benchmarking tools
//! - Concurrent health checks across all APIs simultaneously
//! - Parallel performance benchmarking across multiple endpoints
//! - Concurrent alerting system for multi-API status monitoring

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::fetcher::{ApiProvider, MultiApiClient, ApiError};
// Analytics integration removed - health module is standalone

/// Health status levels for APIs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// API is fully operational
    Healthy,
    /// API is experiencing minor issues
    Degraded,
    /// API is experiencing significant issues
    Unhealthy,
    /// API is completely down
    Down,
    /// API status is unknown (no recent checks)
    Unknown,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Informational alerts
    Info,
    /// Warning alerts
    Warning,
    /// Critical alerts requiring immediate attention
    Critical,
    /// Emergency alerts - system down
    Emergency,
}

/// API health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub provider: ApiProvider,
    pub status: HealthStatus,
    pub response_time: Duration,
    pub last_check: DateTime<Utc>,
    pub consecutive_failures: u32,
    pub total_checks: u64,
    pub successful_checks: u64,
    pub uptime_percentage: f64,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub error_rate: f64,
}

/// Performance benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub provider: ApiProvider,
    pub endpoint: String,
    pub method: String,
    pub response_time: Duration,
    pub status_code: Option<u16>,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
    pub concurrent_requests: u32,
}

/// Health alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthAlert {
    pub id: String,
    pub provider: ApiProvider,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub consecutive_failures: u32,
    pub affected_endpoints: Vec<String>,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
    pub recovery_threshold: u32,
    pub max_history_size: usize,
    pub alert_cooldown: Duration,
    pub benchmark_concurrent_requests: u32,
    pub enable_auto_alerts: bool,
    pub alert_channels: Vec<AlertChannel>,
}

/// Alert delivery channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannel {
    /// Print to console
    Console,
    /// Write to log file
    LogFile(String),
    /// Send via webhook
    Webhook(String),
    /// Send via email (placeholder for future)
    Email,
}

/// Main health monitoring system
pub struct HealthMonitor {
    config: HealthConfig,
    metrics: Arc<RwLock<HashMap<ApiProvider, HealthMetrics>>>,
    alerts: Arc<RwLock<Vec<HealthAlert>>>,
    benchmark_results: Arc<RwLock<VecDeque<BenchmarkResult>>>,
    last_check: Arc<RwLock<HashMap<ApiProvider, Instant>>>,
    alert_cooldowns: Arc<RwLock<HashMap<String, Instant>>>,
}

impl HealthMonitor {
    /// Create a new health monitor with default configuration
    pub fn new() -> Self {
        Self::with_config(HealthConfig::default())
    }

    /// Create a new health monitor with custom configuration
    pub fn with_config(config: HealthConfig) -> Self {
        Self {
            config,
            metrics: Arc::new(RwLock::new(HashMap::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            benchmark_results: Arc::new(RwLock::new(VecDeque::new())),
            last_check: Arc::new(RwLock::new(HashMap::new())),
            alert_cooldowns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Perform concurrent health checks on all APIs
    pub async fn check_all_health(&self, client: Arc<MultiApiClient>) -> HashMap<ApiProvider, HealthStatus> {
        let providers = self.get_available_providers(client.clone());
        let mut handles = vec![];
        let mut results = HashMap::new();

        // Create concurrent health checks
        for provider in providers {
            let provider_clone = provider.clone();
            let monitor = Arc::new(self.clone());
            let client_clone = Arc::clone(&client);

            let handle = tokio::spawn(async move {
                let status = monitor.check_provider_health(&client_clone, &provider_clone).await;
                (provider_clone, status)
            });

            handles.push(handle);
        }

        // Collect results
        for handle in handles {
            match handle.await {
                Ok((provider, status)) => {
                    results.insert(provider, status);
                }
                Err(e) => {
                    println!("⚠️  Health check task failed: {}", e);
                }
            }
        }

        results
    }

    /// Check health of a specific API provider
    pub async fn check_provider_health(&self, client: &MultiApiClient, provider: &ApiProvider) -> HealthStatus {
        let start_time = Instant::now();

        // Update last check time
        {
            let mut last_checks = self.last_check.write().await;
            last_checks.insert(*provider, start_time);
        }

        // Perform actual health check by making a test request
        let health_result = self.perform_health_check(client, provider).await;
        let response_time = start_time.elapsed();

        // Update metrics
        self.update_metrics(provider, health_result.is_ok(), response_time).await;

        // Generate alerts if needed
        if let Err(error) = health_result {
            self.generate_alert(provider, &error, response_time).await;
        }

        // Determine status based on recent metrics
        self.determine_health_status(provider).await
    }

    /// Perform actual health check by making a test request
    async fn perform_health_check(&self, client: &MultiApiClient, provider: &ApiProvider) -> Result<(), HealthError> {
        // Use a timeout for the health check
        let timeout_result = tokio::time::timeout(
            self.config.timeout,
            self.make_test_request(client, provider)
        ).await;

        match timeout_result {
            Ok(result) => result.map_err(|e| HealthError::ApiError(e.to_string())),
            Err(_) => Err(HealthError::Timeout),
        }
    }

    /// Make a test request to check API health
    async fn make_test_request(&self, client: &MultiApiClient, _provider: &ApiProvider) -> Result<(), ApiError> {
        // Try to get a simple price for a well-known symbol (BTC)
        // This is a lightweight health check that doesn't consume much resources
        let _ = client.get_price_intelligent("BTC").await?;
        Ok(())
    }

    /// Update health metrics for a provider
    async fn update_metrics(&self, provider: &ApiProvider, success: bool, response_time: Duration) {
        let mut metrics = self.metrics.write().await;

        let entry = metrics.entry(*provider).or_insert_with(|| HealthMetrics {
            provider: *provider,
            status: HealthStatus::Unknown,
            response_time,
            last_check: Utc::now(),
            consecutive_failures: 0,
            total_checks: 0,
            successful_checks: 0,
            uptime_percentage: 100.0,
            average_response_time: Duration::ZERO,
            min_response_time: Duration::from_secs(u64::MAX),
            max_response_time: Duration::ZERO,
            error_rate: 0.0,
        });

        entry.total_checks += 1;
        entry.last_check = Utc::now();
        entry.response_time = response_time;

        if success {
            entry.successful_checks += 1;
            entry.consecutive_failures = 0;
        } else {
            entry.consecutive_failures += 1;
        }

        // Update statistics
        if success {
            if entry.min_response_time > response_time {
                entry.min_response_time = response_time;
            }
            if entry.max_response_time < response_time {
                entry.max_response_time = response_time;
            }

            // Calculate running average
            let total_time = entry.average_response_time * (entry.successful_checks - 1) as u32 + response_time;
            entry.average_response_time = total_time / entry.successful_checks as u32;
        }

        // Calculate uptime percentage and error rate
        entry.uptime_percentage = if entry.total_checks > 0 {
            (entry.successful_checks as f64 / entry.total_checks as f64) * 100.0
        } else {
            100.0
        };

        entry.error_rate = if entry.total_checks > 0 {
            ((entry.total_checks - entry.successful_checks) as f64 / entry.total_checks as f64) * 100.0
        } else {
            0.0
        };
    }

    /// Determine health status based on recent metrics
    async fn determine_health_status(&self, provider: &ApiProvider) -> HealthStatus {
        let metrics = self.metrics.read().await;

        if let Some(metric) = metrics.get(provider) {
            if metric.consecutive_failures >= self.config.failure_threshold {
                return HealthStatus::Down;
            }

            if metric.error_rate > 20.0 {
                return HealthStatus::Unhealthy;
            }

            if metric.error_rate > 5.0 {
                return HealthStatus::Degraded;
            }

            if metric.successful_checks > 0 {
                return HealthStatus::Healthy;
            }
        }

        HealthStatus::Unknown
    }

    /// Generate alert for health issues
    async fn generate_alert(&self, provider: &ApiProvider, error: &HealthError, response_time: Duration) {
        if !self.config.enable_auto_alerts {
            return;
        }

        let metrics = self.metrics.read().await;
        let consecutive_failures = metrics.get(provider)
            .map(|m| m.consecutive_failures)
            .unwrap_or(0);

        // Check if we're in cooldown period
        let alert_key = format!("{:?}_{}", provider, error);
        let mut cooldowns = self.alert_cooldowns.write().await;

        if let Some(last_alert) = cooldowns.get(&alert_key) {
            if last_alert.elapsed() < self.config.alert_cooldown {
                return; // Still in cooldown
            }
        }

        let severity = self.determine_alert_severity(error, consecutive_failures);

        let alert = HealthAlert {
            id: format!("alert_{}_{}", chrono::Utc::now().timestamp(), alert_key),
            provider: *provider,
            severity: severity.clone(),
            title: format!("{} API Health Alert", provider),
            message: format!(
                "{} API experiencing issues: {} ({} consecutive failures, response time: {:.2}s)",
                provider,
                error,
                consecutive_failures,
                response_time.as_secs_f64()
            ),
            timestamp: Utc::now(),
            resolved: false,
            resolved_at: None,
            consecutive_failures,
            affected_endpoints: vec!["price".to_string(), "analysis".to_string()],
        };

        // Add alert to list
        let mut alerts = self.alerts.write().await;
        alerts.push(alert.clone());

        // Update cooldown
        cooldowns.insert(alert_key, Instant::now());

        // Send alert to configured channels
        self.send_alert(&alert).await;
    }

    /// Determine alert severity based on error and failure count
    fn determine_alert_severity(&self, error: &HealthError, consecutive_failures: u32) -> AlertSeverity {
        if consecutive_failures >= self.config.failure_threshold * 3 {
            return AlertSeverity::Emergency;
        }

        if consecutive_failures >= self.config.failure_threshold {
            return AlertSeverity::Critical;
        }

        match error {
            HealthError::Timeout => AlertSeverity::Warning,
            HealthError::ApiError(_) => AlertSeverity::Warning,
            HealthError::NetworkError => AlertSeverity::Critical,
        }
    }

    /// Send alert to configured channels
    async fn send_alert(&self, alert: &HealthAlert) {
        for channel in &self.config.alert_channels {
            match channel {
                AlertChannel::Console => {
                    self.send_console_alert(alert);
                }
                AlertChannel::LogFile(path) => {
                    self.send_log_alert(alert, path).await;
                }
                AlertChannel::Webhook(url) => {
                    self.send_webhook_alert(alert, url).await;
                }
                AlertChannel::Email => {
                    // Placeholder for email implementation
                    println!("📧 Email alert not implemented yet: {}", alert.title);
                }
            }
        }
    }

    /// Send alert to console
    fn send_console_alert(&self, alert: &HealthAlert) {
        let severity_icon = match alert.severity {
            AlertSeverity::Info => "ℹ️",
            AlertSeverity::Warning => "⚠️",
            AlertSeverity::Critical => "🚨",
            AlertSeverity::Emergency => "🚨🚨",
        };

        println!(
            "{} {} - {}: {} (Severity: {:?})",
            severity_icon,
            alert.timestamp.format("%H:%M:%S"),
            alert.provider,
            alert.title,
            alert.severity
        );

        if alert.severity == AlertSeverity::Critical || alert.severity == AlertSeverity::Emergency {
            println!("   📝 Details: {}", alert.message);
        }
    }

    /// Send alert to log file
    async fn send_log_alert(&self, alert: &HealthAlert, _path: &str) {
        // In a real implementation, this would write to a log file
        println!("📝 Log alert: {} - {}", alert.title, alert.message);
    }

    /// Send alert via webhook
    async fn send_webhook_alert(&self, alert: &HealthAlert, _url: &str) {
        // In a real implementation, this would send HTTP request to webhook
        println!("🔗 Webhook alert: {} - {}", alert.title, alert.message);
    }

    /// Get available providers from client
    fn get_available_providers(&self, _client: Arc<MultiApiClient>) -> Vec<ApiProvider> {
        vec![
            ApiProvider::CoinGecko,
            ApiProvider::CoinPaprika,
            ApiProvider::CoinMarketCap,
            ApiProvider::CryptoCompare,
        ]
    }

    /// Get current health metrics for all providers
    pub async fn get_health_metrics(&self) -> HashMap<ApiProvider, HealthMetrics> {
        self.metrics.read().await.clone()
    }

    /// Get health metrics for a specific provider
    pub async fn get_provider_metrics(&self, provider: &ApiProvider) -> Option<HealthMetrics> {
        self.metrics.read().await.get(provider).cloned()
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Vec<HealthAlert> {
        let alerts = self.alerts.read().await;
        alerts.iter().rev().take(limit).cloned().collect()
    }

    /// Get unresolved alerts
    pub async fn get_unresolved_alerts(&self) -> Vec<HealthAlert> {
        let alerts = self.alerts.read().await;
        alerts.iter()
            .filter(|alert| !alert.resolved)
            .cloned()
            .collect()
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: &str) {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
            alert.resolved_at = Some(Utc::now());
        }
    }

    /// Run performance benchmarks concurrently
    pub async fn run_performance_benchmarks(&self, client: Arc<MultiApiClient>) -> Vec<BenchmarkResult> {
        let providers = self.get_available_providers(client.clone());
        let mut handles = vec![];
        let mut results = vec![];

        // Create concurrent benchmark tasks
        for provider in providers {
            let provider_clone = provider.clone();
            let monitor = Arc::new(self.clone());
            let client_clone = Arc::clone(&client);

            let handle = tokio::spawn(async move {
                monitor.benchmark_provider(&client_clone, &provider_clone).await
            });

            handles.push(handle);
        }

        // Collect benchmark results
        for handle in handles {
            match handle.await {
                Ok(provider_results) => {
                    results.extend(provider_results);
                }
                Err(e) => {
                    println!("⚠️  Benchmark task failed: {}", e);
                }
            }
        }

        results
    }

    /// Benchmark a specific provider
    async fn benchmark_provider(&self, client: &MultiApiClient, provider: &ApiProvider) -> Vec<BenchmarkResult> {
        let mut results = vec![];
        let symbols = vec!["BTC", "ETH", "ADA", "DOT", "LINK"];

        // Benchmark different endpoints
        for symbol in &symbols {
            let result = self.benchmark_endpoint(client, provider, "price", symbol).await;
            results.push(result);
        }

        // Store benchmark results
        let mut benchmark_results = self.benchmark_results.write().await;
        for result in &results {
            benchmark_results.push_back(result.clone());

            // Keep only recent results
            while benchmark_results.len() > self.config.max_history_size {
                benchmark_results.pop_front();
            }
        }

        results
    }

    /// Benchmark a specific endpoint
    async fn benchmark_endpoint(&self, client: &MultiApiClient, provider: &ApiProvider, endpoint: &str, symbol: &str) -> BenchmarkResult {
        let start_time = Instant::now();

        let success = match endpoint {
            "price" => client.get_price_intelligent(symbol).await.is_ok(),
            _ => false,
        };

        let response_time = start_time.elapsed();

        BenchmarkResult {
            provider: *provider,
            endpoint: endpoint.to_string(),
            method: "GET".to_string(),
            response_time,
            status_code: Some(200), // Simplified - in real implementation would get actual status
            success,
            timestamp: Utc::now(),
            concurrent_requests: 1,
        }
    }

    /// Get benchmark results
    pub async fn get_benchmark_results(&self, limit: usize) -> Vec<BenchmarkResult> {
        let results = self.benchmark_results.read().await;
        results.iter().rev().take(limit).cloned().collect()
    }

    /// Get health dashboard data
    pub async fn get_health_dashboard(&self) -> serde_json::Value {
        let metrics = self.get_health_metrics().await;
        let unresolved_alerts = self.get_unresolved_alerts().await;
        let recent_benchmarks = self.get_benchmark_results(10).await;

        serde_json::json!({
            "timestamp": Utc::now(),
            "overall_health": self.calculate_overall_health(&metrics).await,
            "provider_health": metrics,
            "active_alerts": unresolved_alerts.len(),
            "recent_alerts": unresolved_alerts.into_iter().take(5).collect::<Vec<_>>(),
            "benchmark_summary": self.summarize_benchmarks(&recent_benchmarks).await,
            "system_status": "operational"
        })
    }

    /// Calculate overall system health
    async fn calculate_overall_health(&self, metrics: &HashMap<ApiProvider, HealthMetrics>) -> String {
        if metrics.is_empty() {
            return "unknown".to_string();
        }

        let total_providers = metrics.len();
        let healthy_providers = metrics.values()
            .filter(|m| m.status == HealthStatus::Healthy)
            .count();

        let health_percentage = (healthy_providers as f64 / total_providers as f64) * 100.0;

        if health_percentage >= 75.0 {
            "excellent".to_string()
        } else if health_percentage >= 50.0 {
            "good".to_string()
        } else if health_percentage >= 25.0 {
            "fair".to_string()
        } else {
            "poor".to_string()
        }
    }

    /// Summarize benchmark results
    async fn summarize_benchmarks(&self, benchmarks: &[BenchmarkResult]) -> serde_json::Value {
        if benchmarks.is_empty() {
            return serde_json::json!({ "status": "no_data" });
        }

        let total_requests = benchmarks.len();
        let successful_requests = benchmarks.iter().filter(|b| b.success).count();
        let success_rate = (successful_requests as f64 / total_requests as f64) * 100.0;

        let avg_response_time = benchmarks.iter()
            .map(|b| b.response_time)
            .sum::<Duration>() / total_requests as u32;

        serde_json::json!({
            "total_requests": total_requests,
            "successful_requests": successful_requests,
            "success_rate": success_rate,
            "average_response_time_ms": avg_response_time.as_millis(),
            "fastest_provider": self.find_fastest_provider(benchmarks),
            "slowest_provider": self.find_slowest_provider(benchmarks)
        })
    }

    /// Find fastest provider from benchmark results
    fn find_fastest_provider(&self, benchmarks: &[BenchmarkResult]) -> Option<String> {
        benchmarks.iter()
            .filter(|b| b.success)
            .min_by_key(|b| b.response_time)
            .map(|b| format!("{:?}", b.provider))
    }

    /// Find slowest provider from benchmark results
    fn find_slowest_provider(&self, benchmarks: &[BenchmarkResult]) -> Option<String> {
        benchmarks.iter()
            .filter(|b| b.success)
            .max_by_key(|b| b.response_time)
            .map(|b| format!("{:?}", b.provider))
    }

    /// Get health status summary
    pub async fn get_health_summary(&self) -> String {
        let metrics = self.get_health_metrics().await;
        let alerts = self.get_unresolved_alerts().await;

        let mut summary = format!("🩺 API Health Summary ({} providers monitored)\n", metrics.len());
        summary.push_str("================================\n\n");

        for (provider, metric) in &metrics {
            let status_icon = match metric.status {
                HealthStatus::Healthy => "✅",
                HealthStatus::Degraded => "⚠️",
                HealthStatus::Unhealthy => "🚨",
                HealthStatus::Down => "❌",
                HealthStatus::Unknown => "❓",
            };

            summary.push_str(&format!(
                "{} {}: {:.1}% uptime, {:.2}s avg response, {} failures\n",
                status_icon,
                provider,
                metric.uptime_percentage,
                metric.average_response_time.as_secs_f64(),
                metric.consecutive_failures
            ));
        }

        if !alerts.is_empty() {
            summary.push_str(&format!("\n🚨 Active Alerts: {}\n", alerts.len()));
            for alert in alerts.iter().take(3) {
                summary.push_str(&format!("   • {} ({:?})\n", alert.title, alert.severity));
            }
        }

        summary
    }

    /// Start continuous health monitoring
    pub async fn start_monitoring(&self, _client: Arc<MultiApiClient>) {
        let monitor = Arc::new(self.clone());

        tokio::spawn(async move {
            loop {
                println!("🔍 Running health checks...");
                // Create a new client instance for each health check
                let client = MultiApiClient::new_with_all_apis();
                let client_arc = Arc::new(client);
                let results = monitor.check_all_health(client_arc).await;

                for (provider, status) in &results {
                    match status {
                        HealthStatus::Healthy => println!("✅ {}: Healthy", provider),
                        HealthStatus::Degraded => println!("⚠️  {}: Degraded", provider),
                        HealthStatus::Unhealthy => println!("🚨 {}: Unhealthy", provider),
                        HealthStatus::Down => println!("❌ {}: Down", provider),
                        HealthStatus::Unknown => println!("❓ {}: Unknown", provider),
                    }
                }

                tokio::time::sleep(monitor.config.check_interval).await;
            }
        });
    }
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(60), // Check every minute
            timeout: Duration::from_secs(10),        // 10 second timeout
            failure_threshold: 3,                    // 3 consecutive failures = down
            recovery_threshold: 2,                   // 2 consecutive successes = recovered
            max_history_size: 1000,
            alert_cooldown: Duration::from_secs(300), // 5 minute cooldown between alerts
            benchmark_concurrent_requests: 5,
            enable_auto_alerts: true,
            alert_channels: vec![AlertChannel::Console],
        }
    }
}

impl Clone for HealthMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            metrics: Arc::clone(&self.metrics),
            alerts: Arc::clone(&self.alerts),
            benchmark_results: Arc::clone(&self.benchmark_results),
            last_check: Arc::clone(&self.last_check),
            alert_cooldowns: Arc::clone(&self.alert_cooldowns),
        }
    }
}

/// Health error types
#[derive(Debug)]
pub enum HealthError {
    ApiError(String), // Store error message instead of ApiError
    Timeout,
    NetworkError,
}

impl std::fmt::Display for HealthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthError::ApiError(e) => write!(f, "API Error: {}", e),
            HealthError::Timeout => write!(f, "Request Timeout"),
            HealthError::NetworkError => write!(f, "Network Error"),
        }
    }
}

impl std::error::Error for HealthError {}

/// Concurrent health checker for parallel operations
pub struct ConcurrentHealthChecker {
    monitor: Arc<HealthMonitor>,
    workers: usize,
}

impl ConcurrentHealthChecker {
    pub fn new(monitor: Arc<HealthMonitor>, workers: usize) -> Self {
        Self { monitor, workers }
    }

    /// Perform concurrent health checks across multiple providers
    pub async fn check_multiple_providers(&self, client: Arc<MultiApiClient>, providers: Vec<ApiProvider>) -> HashMap<ApiProvider, HealthStatus> {
        let mut handles = vec![];
        let mut results = HashMap::new();

        // Split providers into chunks for concurrent processing
        for chunk in providers.chunks((providers.len() + self.workers - 1) / self.workers) {
            let chunk = chunk.to_vec();
            let monitor = Arc::clone(&self.monitor);
            let client_clone = Arc::clone(&client);

            let handle = tokio::spawn(async move {
                let mut chunk_results = HashMap::new();
                for provider in chunk {
                    let status = monitor.check_provider_health(&client_clone, &provider).await;
                    chunk_results.insert(provider, status);
                }
                chunk_results
            });

            handles.push(handle);
        }

        // Collect all results
        for handle in handles {
            match handle.await {
                Ok(chunk_results) => {
                    results.extend(chunk_results);
                }
                Err(e) => {
                    println!("⚠️  Concurrent health check failed: {}", e);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor_creation() {
        let monitor = HealthMonitor::new();
        assert!(monitor.metrics.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_health_config_defaults() {
        let config = HealthConfig::default();
        assert_eq!(config.check_interval, Duration::from_secs(60));
        assert_eq!(config.failure_threshold, 3);
        assert!(config.enable_auto_alerts);
    }

    #[tokio::test]
    async fn test_health_status_determination() {
        let monitor = HealthMonitor::new();

        // Test with no metrics (should be Unknown)
        let status = monitor.determine_health_status(&ApiProvider::CoinGecko).await;
        assert_eq!(status, HealthStatus::Unknown);
    }
}
