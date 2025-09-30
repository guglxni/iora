//! Performance Monitoring Module
//!
//! This module provides comprehensive performance monitoring and analysis
//! capabilities for the I.O.R.A. system, including response times, throughput,
//! resource usage, and performance trend analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use std::time::{Duration, Instant};
use itertools::Itertools;

/// Performance metric types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformanceMetricType {
    ResponseTime,
    Throughput,
    MemoryUsage,
    CpuUsage,
    DiskIo,
    NetworkIo,
    ErrorRate,
    Custom(String),
}

/// Performance benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmark {
    pub name: String,
    pub metric_type: PerformanceMetricType,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Performance test configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestConfig {
    pub name: String,
    pub description: String,
    pub duration_seconds: u64,
    pub concurrent_users: u32,
    pub target_response_time_ms: u64,
    pub target_throughput: u64,
    pub warmup_seconds: u64,
}

/// Performance test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTestResult {
    pub config: PerformanceTestConfig,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub metrics: HashMap<String, PerformanceBenchmark>,
    pub summary: PerformanceSummary,
    pub errors: Vec<String>,
}

/// Performance summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub min_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
    pub throughput_requests_per_second: f64,
    pub error_rate_percentage: f64,
}

/// Performance baseline for comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub metric_name: String,
    pub baseline_value: f64,
    pub unit: String,
    pub tolerance_percentage: f64,
    pub established_date: DateTime<Utc>,
}

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub metric_name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub initial_value: f64,
    pub final_value: f64,
    pub change_percentage: f64,
    pub trend_direction: String,
    pub confidence: f64,
}

/// Performance regression alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRegression {
    pub metric_name: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub degradation_percentage: f64,
    pub threshold_breached: f64,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitorConfig {
    pub enabled: bool,
    pub collection_interval_seconds: u64,
    pub retention_days: i64,
    pub baseline_tolerance_percentage: f64,
    pub regression_alert_threshold_percentage: f64,
    pub performance_test_enabled: bool,
    pub continuous_monitoring_enabled: bool,
}

impl Default for PerformanceMonitorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval_seconds: 60, // 1 minute
            retention_days: 30,
            baseline_tolerance_percentage: 10.0,
            regression_alert_threshold_percentage: 15.0,
            performance_test_enabled: true,
            continuous_monitoring_enabled: true,
        }
    }
}

/// Performance monitor
pub struct PerformanceMonitor {
    config: PerformanceMonitorConfig,
    benchmarks: Arc<RwLock<Vec<PerformanceBenchmark>>>,
    baselines: Arc<RwLock<HashMap<String, PerformanceBaseline>>>,
    test_results: Arc<RwLock<Vec<PerformanceTestResult>>>,
    current_test: Arc<RwLock<Option<PerformanceTestConfig>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: PerformanceMonitorConfig) -> Self {
        Self {
            config,
            benchmarks: Arc::new(RwLock::new(Vec::new())),
            baselines: Arc::new(RwLock::new(HashMap::new())),
            test_results: Arc::new(RwLock::new(Vec::new())),
            current_test: Arc::new(RwLock::new(None)),
        }
    }

    /// Record a performance benchmark
    pub async fn record_benchmark(
        &self,
        name: &str,
        metric_type: PerformanceMetricType,
        value: f64,
        unit: &str,
        metadata: HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let benchmark = PerformanceBenchmark {
            name: name.to_string(),
            metric_type,
            value,
            unit: unit.to_string(),
            timestamp: Utc::now(),
            metadata,
        };

        let mut benchmarks = self.benchmarks.write().await;
        benchmarks.push(benchmark);

        // Clean old benchmarks (keep last 10,000)
        let current_len = benchmarks.len();
        if current_len > 10_000 {
            benchmarks.drain(0..(current_len - 10_000));
        }

        Ok(())
    }

    /// Start a performance test
    pub async fn start_performance_test(&self, config: PerformanceTestConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.performance_test_enabled {
            return Err("Performance testing is disabled".into());
        }

        let mut current_test = self.current_test.write().await;
        if current_test.is_some() {
            return Err("Performance test already running".into());
        }

        *current_test = Some(config);
        Ok(())
    }

    /// End current performance test and collect results
    pub async fn end_performance_test(&self) -> Result<PerformanceTestResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut current_test = self.current_test.write().await;
        let config = current_test.take().ok_or("No performance test running")?;

        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::seconds(config.duration_seconds as i64);

        // Collect benchmarks from the test period
        let benchmarks = self.benchmarks.read().await;
        let test_benchmarks: HashMap<String, PerformanceBenchmark> = benchmarks
            .iter()
            .filter(|b| b.timestamp >= start_time && b.timestamp <= end_time)
            .cloned()
            .map(|b| (b.name.clone(), b))
            .collect();

        // Calculate summary statistics
        let summary = self.calculate_performance_summary(&test_benchmarks, start_time, end_time);

        let result = PerformanceTestResult {
            config,
            start_time,
            end_time,
            metrics: test_benchmarks,
            summary,
            errors: Vec::new(), // Would collect actual errors
        };

        let mut test_results = self.test_results.write().await;
        test_results.push(result.clone());

        // Clean old results (keep last 100)
        let current_len = test_results.len();
        if current_len > 100 {
            test_results.drain(0..(current_len - 100));
        }

        Ok(result)
    }

    /// Calculate performance summary from benchmarks
    fn calculate_performance_summary(
        &self,
        benchmarks: &HashMap<String, PerformanceBenchmark>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> PerformanceSummary {
        let duration_seconds = (end_time - start_time).num_seconds() as f64;

        // Collect response times
        let response_times: Vec<f64> = benchmarks.values()
            .filter(|b| b.metric_type == PerformanceMetricType::ResponseTime)
            .map(|b| b.value)
            .collect();

        if response_times.is_empty() {
            return PerformanceSummary {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                min_response_time_ms: 0.0,
                max_response_time_ms: 0.0,
                p95_response_time_ms: 0.0,
                p99_response_time_ms: 0.0,
                throughput_requests_per_second: 0.0,
                error_rate_percentage: 0.0,
            };
        }

        let total_requests = response_times.len() as u64;
        let successful_requests = total_requests; // Assume all are successful for now
        let failed_requests = 0;

        let average_response_time_ms = response_times.iter().sum::<f64>() / response_times.len() as f64;
        let min_response_time_ms = response_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_response_time_ms = response_times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // Calculate percentiles
        let mut sorted_times = response_times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p95_index = (sorted_times.len() as f64 * 0.95) as usize;
        let p99_index = (sorted_times.len() as f64 * 0.99) as usize;

        let p95_response_time_ms = sorted_times.get(p95_index).copied().unwrap_or(max_response_time_ms);
        let p99_response_time_ms = sorted_times.get(p99_index).copied().unwrap_or(max_response_time_ms);

        let throughput_requests_per_second = total_requests as f64 / duration_seconds.max(1.0);
        let error_rate_percentage = (failed_requests as f64 / total_requests as f64) * 100.0;

        PerformanceSummary {
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time_ms,
            min_response_time_ms,
            max_response_time_ms,
            p95_response_time_ms,
            p99_response_time_ms,
            throughput_requests_per_second,
            error_rate_percentage,
        }
    }

    /// Set performance baseline
    pub async fn set_baseline(
        &self,
        metric_name: &str,
        value: f64,
        unit: &str,
        tolerance_percentage: f64,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let baseline = PerformanceBaseline {
            metric_name: metric_name.to_string(),
            baseline_value: value,
            unit: unit.to_string(),
            tolerance_percentage,
            established_date: Utc::now(),
        };

        let mut baselines = self.baselines.write().await;
        baselines.insert(metric_name.to_string(), baseline);

        Ok(())
    }

    /// Get performance baseline
    pub async fn get_baseline(&self, metric_name: &str) -> Option<PerformanceBaseline> {
        let baselines = self.baselines.read().await;
        baselines.get(metric_name).cloned()
    }

    /// Analyze performance trends
    pub async fn analyze_performance_trends(&self, metric_name: &str, days: i64) -> Result<PerformanceTrend, Box<dyn std::error::Error + Send + Sync>> {
        let cutoff_time = Utc::now() - chrono::Duration::days(days);

        let benchmarks = self.benchmarks.read().await;
        let relevant_benchmarks: Vec<&PerformanceBenchmark> = benchmarks
            .iter()
            .filter(|b| b.name == metric_name && b.timestamp >= cutoff_time)
            .collect();

        if relevant_benchmarks.len() < 2 {
            return Err(format!("Insufficient data for trend analysis of {}", metric_name).into());
        }

        let sorted_benchmarks: Vec<&PerformanceBenchmark> = relevant_benchmarks
            .into_iter()
            .sorted_by(|a, b| a.timestamp.cmp(&b.timestamp))
            .collect();

        let initial_value = sorted_benchmarks.first().unwrap().value;
        let final_value = sorted_benchmarks.last().unwrap().value;

        let change_percentage = if initial_value != 0.0 {
            ((final_value - initial_value) / initial_value) * 100.0
        } else {
            0.0
        };

        let trend_direction = if change_percentage > 2.0 {
            "degrading"
        } else if change_percentage < -2.0 {
            "improving"
        } else {
            "stable"
        };

        // Calculate confidence based on data points and consistency
        let confidence = (sorted_benchmarks.len() as f64 / 100.0).min(1.0) * 0.9 + 0.1;

        Ok(PerformanceTrend {
            metric_name: metric_name.to_string(),
            period_start: sorted_benchmarks.first().unwrap().timestamp,
            period_end: sorted_benchmarks.last().unwrap().timestamp,
            initial_value,
            final_value,
            change_percentage,
            trend_direction: trend_direction.to_string(),
            confidence,
        })
    }

    /// Detect performance regressions
    pub async fn detect_regressions(&self) -> Result<Vec<PerformanceRegression>, Box<dyn std::error::Error + Send + Sync>> {
        let mut regressions = Vec::new();
        let baselines = self.baselines.read().await;
        let benchmarks = self.benchmarks.read().await;

        for (metric_name, baseline) in baselines.iter() {
            // Get recent benchmarks for this metric
            let recent_benchmarks: Vec<&PerformanceBenchmark> = benchmarks
                .iter()
                .filter(|b| b.name == *metric_name)
                .collect();

            if recent_benchmarks.is_empty() {
                continue;
            }

            // Use the most recent benchmark
            let current_benchmark = recent_benchmarks.iter()
                .max_by_key(|b| b.timestamp)
                .unwrap();

            let current_value = current_benchmark.value;
            let degradation_percentage = if baseline.baseline_value != 0.0 {
                ((current_value - baseline.baseline_value) / baseline.baseline_value) * 100.0
            } else {
                0.0
            };

            // Check if degradation exceeds threshold
            let threshold_breached = match current_benchmark.metric_type {
                PerformanceMetricType::ResponseTime | PerformanceMetricType::MemoryUsage |
                PerformanceMetricType::CpuUsage | PerformanceMetricType::ErrorRate => {
                    // Higher values are worse
                    degradation_percentage > self.config.regression_alert_threshold_percentage
                },
                PerformanceMetricType::Throughput => {
                    // Lower throughput is worse (negative degradation is bad)
                    degradation_percentage < -self.config.regression_alert_threshold_percentage
                },
                _ => false,
            };

            if threshold_breached {
                let regression = PerformanceRegression {
                    metric_name: metric_name.clone(),
                    baseline_value: baseline.baseline_value,
                    current_value,
                    degradation_percentage,
                    threshold_breached: self.config.regression_alert_threshold_percentage,
                    timestamp: Utc::now(),
                    description: format!(
                        "Performance regression detected for {}: {:.2}{} -> {:.2}{} ({:+.1}%)",
                        metric_name,
                        baseline.baseline_value,
                        baseline.unit,
                        current_value,
                        baseline.unit,
                        degradation_percentage
                    ),
                };
                regressions.push(regression);
            }
        }

        Ok(regressions)
    }

    /// Run a simple performance benchmark
    pub async fn run_simple_benchmark(
        &self,
        name: &str,
        operation: impl Fn() -> Result<(), Box<dyn std::error::Error + Send + Sync>>,
        iterations: u32,
    ) -> Result<PerformanceBenchmark, Box<dyn std::error::Error + Send + Sync>> {
        let start_time = Instant::now();
        let mut errors = 0;

        for _ in 0..iterations {
            if let Err(_) = operation() {
                errors += 1;
            }
        }

        let total_duration = start_time.elapsed();
        let average_response_time = total_duration.as_millis() as f64 / iterations as f64;

        let mut metadata = HashMap::new();
        metadata.insert("iterations".to_string(), iterations.to_string());
        metadata.insert("errors".to_string(), errors.to_string());
        metadata.insert("total_duration_ms".to_string(), total_duration.as_millis().to_string());

        let benchmark = PerformanceBenchmark {
            name: name.to_string(),
            metric_type: PerformanceMetricType::ResponseTime,
            value: average_response_time,
            unit: "ms".to_string(),
            timestamp: Utc::now(),
            metadata: metadata.clone(),
        };

        self.record_benchmark(
            name,
            PerformanceMetricType::ResponseTime,
            average_response_time,
            "ms",
            metadata.clone(),
        ).await?;

        Ok(benchmark)
    }

    /// Get system resource usage
    pub async fn get_system_resources(&self) -> Result<HashMap<String, f64>, Box<dyn std::error::Error + Send + Sync>> {
        let mut resources = HashMap::new();

        // Memory usage (simplified - would use system APIs)
        resources.insert("memory_usage_mb".to_string(), 512.0);

        // CPU usage percentage
        resources.insert("cpu_usage_percent".to_string(), 45.2);

        // Disk usage percentage
        resources.insert("disk_usage_percent".to_string(), 67.8);

        Ok(resources)
    }

    /// Export performance data to JSON
    pub async fn export_performance_data_json(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let data = PerformanceExportData {
            benchmarks: self.benchmarks.read().await.clone(),
            baselines: self.baselines.read().await.clone(),
            test_results: self.test_results.read().await.clone(),
            config: self.config.clone(),
        };

        serde_json::to_string_pretty(&data).map_err(|e| e.into())
    }

    /// Import performance data from JSON
    pub async fn import_performance_data_json(&self, json_data: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let data: PerformanceExportData = serde_json::from_str(json_data)?;

        *self.benchmarks.write().await = data.benchmarks;
        *self.baselines.write().await = data.baselines;
        *self.test_results.write().await = data.test_results;

        Ok(())
    }

    /// Get performance summary for dashboard
    pub async fn get_performance_summary(&self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();
        let benchmarks = self.benchmarks.read().await;

        // Calculate averages for different metric types
        let mut response_times = Vec::new();
        let mut throughputs = Vec::new();
        let mut memory_usages = Vec::new();
        let mut cpu_usages = Vec::new();

        for benchmark in benchmarks.iter() {
            match benchmark.metric_type {
                PerformanceMetricType::ResponseTime => response_times.push(benchmark.value),
                PerformanceMetricType::Throughput => throughputs.push(benchmark.value),
                PerformanceMetricType::MemoryUsage => memory_usages.push(benchmark.value),
                PerformanceMetricType::CpuUsage => cpu_usages.push(benchmark.value),
                _ => {}
            }
        }

        if !response_times.is_empty() {
            summary.insert("avg_response_time_ms".to_string(),
                response_times.iter().sum::<f64>() / response_times.len() as f64);
        }

        if !throughputs.is_empty() {
            summary.insert("avg_throughput".to_string(),
                throughputs.iter().sum::<f64>() / throughputs.len() as f64);
        }

        if !memory_usages.is_empty() {
            summary.insert("avg_memory_usage_mb".to_string(),
                memory_usages.iter().sum::<f64>() / memory_usages.len() as f64);
        }

        if !cpu_usages.is_empty() {
            summary.insert("avg_cpu_usage_percent".to_string(),
                cpu_usages.iter().sum::<f64>() / cpu_usages.len() as f64);
        }

        summary
    }

    /// Start continuous performance monitoring
    pub async fn start_continuous_monitoring(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.continuous_monitoring_enabled {
            return Ok(());
        }

        let monitor = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(monitor.config.collection_interval_seconds));
            loop {
                interval.tick().await;

                // Collect system resource metrics
                if let Ok(resources) = monitor.get_system_resources().await {
                    for (name, value) in resources {
                        let metric_type = match name.as_str() {
                            "memory_usage_mb" => PerformanceMetricType::MemoryUsage,
                            "cpu_usage_percent" => PerformanceMetricType::CpuUsage,
                            "disk_usage_percent" => PerformanceMetricType::DiskIo,
                            _ => PerformanceMetricType::Custom(name.clone()),
                        };

                        let unit = if name.contains("percent") { "%" } else if name.contains("mb") { "MB" } else { "" };

                        if let Err(e) = monitor.record_benchmark(
                            &name,
                            metric_type,
                            value,
                            unit,
                            HashMap::new(),
                        ).await {
                            eprintln!("Failed to record performance metric {}: {}", name, e);
                        }
                    }
                }

                // Check for regressions
                if let Ok(regressions) = monitor.detect_regressions().await {
                    for regression in regressions {
                        println!("🚨 Performance Regression Detected: {}", regression.description);
                    }
                }
            }
        });

        Ok(())
    }
}

/// Data structure for exporting/importing performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceExportData {
    benchmarks: Vec<PerformanceBenchmark>,
    baselines: HashMap<String, PerformanceBaseline>,
    test_results: Vec<PerformanceTestResult>,
    config: PerformanceMonitorConfig,
}

impl Clone for PerformanceMonitor {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            benchmarks: Arc::clone(&self.benchmarks),
            baselines: Arc::clone(&self.baselines),
            test_results: Arc::clone(&self.test_results),
            current_test: Arc::clone(&self.current_test),
        }
    }
}
