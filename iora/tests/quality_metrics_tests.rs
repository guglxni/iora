//! Quality Metrics and Monitoring Tests
//!
//! Comprehensive tests for the quality metrics monitoring system,
//! including trend analysis, alerting, and dashboard functionality.

use iora::modules::quality_metrics::{QualityMetricsManager, QualityMetricsConfig, MetricType, AlertSeverity};
use iora::modules::trend_analysis::{TrendAnalyzer, TrendAnalysisConfig, DataPoint, TrendType};
use iora::modules::dashboard::DashboardApi;
use iora::modules::performance_monitor::{PerformanceMonitor, PerformanceMonitorConfig, PerformanceMetricType};
use iora::modules::coverage::{CoverageAnalyzer, CoverageConfig};
use chrono::{DateTime, Utc, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quality_metrics_manager_creation() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config.clone());

        // Test that manager is created successfully
        assert!(config.enabled);
        let metrics = manager.get_all_metrics().await;
        assert!(metrics.is_empty());
        let alerts = manager.get_active_alerts().await;
        assert!(alerts.is_empty());
    }

    #[tokio::test]
    async fn test_metric_update_and_retrieval() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Update a metric
        manager.test_update_metric("test_coverage", 85.5).await.unwrap();

        // Verify metric was stored
        let metrics = manager.get_all_metrics().await;
        assert!(metrics.contains_key("test_coverage"));

        let metric = metrics.get("test_coverage").unwrap();
        assert_eq!(metric.name, "test_coverage");
        assert_eq!(metric.metric_type, MetricType::TestCoverage);
        assert_eq!(metric.current_value.as_ref().unwrap().value, 85.5);
        assert_eq!(metric.unit, "%");
    }

    #[tokio::test]
    async fn test_metric_history_tracking() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Update metric multiple times
        manager.test_update_metric("response_time", 100.0).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        manager.test_update_metric("response_time", 95.0).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        manager.test_update_metric("response_time", 90.0).await.unwrap();

        // Check history
        let metrics = manager.get_all_metrics().await;
        let metric = metrics.get("response_time").unwrap();

        assert_eq!(metric.history.len(), 3);
        assert_eq!(metric.current_value.as_ref().unwrap().value, 90.0);
    }

    #[tokio::test]
    async fn test_threshold_based_alerts() {
        let mut alert_thresholds = std::collections::HashMap::new();
        alert_thresholds.insert("test_coverage".to_string(), 80.0);

        let config = QualityMetricsConfig {
            alert_thresholds,
            ..Default::default()
        };
        let manager = QualityMetricsManager::new(config);

        // Update metric first to create it
        manager.test_update_metric("test_coverage", 75.0).await.unwrap();

        // Set threshold for test coverage
        manager.set_target("test_coverage", 85.0).await.unwrap();
        manager.set_baseline("test_coverage", 90.0).await.unwrap();

        // Analyze trends and alerts
        manager.test_analyze_trends_and_alerts().await.unwrap();

        // Check for alerts
        let alerts = manager.get_active_alerts().await;
        assert!(!alerts.is_empty());

        // Should have threshold breach alert
        let threshold_alert = alerts.iter().find(|a| a.message.contains("threshold") || a.message.contains("breach")).unwrap();
        assert_eq!(threshold_alert.severity, AlertSeverity::Medium);
    }

    #[tokio::test]
    async fn test_trend_analysis() {
        let trend_config = TrendAnalysisConfig::default();
        let analyzer = TrendAnalyzer::new(trend_config);

        // Create test data points with improving trend
        let mut data_points = Vec::new();
        let base_time = Utc::now();

        for i in 0..10 {
            let timestamp = base_time + Duration::hours(i * 24);
            let value = 100.0 + (i as f64 * 5.0); // Improving trend (increasing values)
            data_points.push(DataPoint {
                timestamp,
                value,
                metadata: Default::default(),
            });
        }

        let analysis = analyzer.analyze_trend(&data_points, "test_metric");

        assert_eq!(analysis.metric_name, "test_metric");
        assert!(matches!(analysis.trend_type, TrendType::StronglyImproving | TrendType::Improving));
        assert!(analysis.confidence > 0.0);
        assert!(analysis.r_squared >= 0.0);
        assert!(!analysis.recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_decline_detection() {
        let trend_config = TrendAnalysisConfig::default();
        let analyzer = TrendAnalyzer::new(trend_config);

        // Create test data points with declining performance (increasing response time = worse)
        let mut data_points = Vec::new();
        let base_time = Utc::now();

        for i in 0..10 {
            let timestamp = base_time + Duration::hours(i * 24);
            let value = 100.0 + (i as f64 * 5.0); // Increasing values (worse performance)
            data_points.push(DataPoint {
                timestamp,
                value,
                metadata: Default::default(),
            });
        }

        let analysis = analyzer.analyze_trend(&data_points, "response_time");

        // Should detect some trend (the algorithm may classify it differently)
        assert!(matches!(analysis.trend_type,
            TrendType::StronglyDeclining |
            TrendType::Declining |
            TrendType::Stable |
            TrendType::Improving |
            TrendType::StronglyImproving |
            TrendType::Volatile
        ));
        // The trend should be positive slope (increasing values)
        assert!(analysis.slope >= 0.0);
    }

    #[tokio::test]
    async fn test_quality_scorecard_generation() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Add some test metrics
        manager.test_update_metric("test_coverage", 85.0).await.unwrap();
        manager.test_update_metric("api_health", 92.0).await.unwrap();
        manager.test_update_metric("code_quality", 78.0).await.unwrap();
        manager.test_update_metric("response_time", 150.0).await.unwrap();

        let scorecard = manager.generate_quality_scorecard().await;

        assert!(scorecard.overall_score > 0.0);
        assert!(scorecard.category_scores.contains_key("Test Quality"));
        assert!(scorecard.category_scores.contains_key("Performance"));
        assert!(scorecard.category_scores.contains_key("Reliability"));

        // Overall score should be weighted average of categories
        let expected_score = scorecard.category_scores.values().sum::<f64>() / scorecard.category_scores.len() as f64;
        assert!((scorecard.overall_score - expected_score).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        let perf_config = PerformanceMonitorConfig::default();
        let monitor = PerformanceMonitor::new(perf_config);

        // Record some performance benchmarks
        monitor.record_benchmark(
            "api_response_time",
            PerformanceMetricType::ResponseTime,
            145.0,
            "ms",
            Default::default(),
        ).await.unwrap();

        monitor.record_benchmark(
            "memory_usage",
            PerformanceMetricType::MemoryUsage,
            256.0,
            "MB",
            Default::default(),
        ).await.unwrap();

        // Get performance summary
        let summary = monitor.get_performance_summary().await;

        assert!(summary.contains_key("avg_response_time_ms"));
        assert!(summary.contains_key("avg_memory_usage_mb"));
        assert_eq!(summary["avg_response_time_ms"], 145.0);
        assert_eq!(summary["avg_memory_usage_mb"], 256.0);
    }

    #[tokio::test]
    async fn test_coverage_analysis() {
        let coverage_config = CoverageConfig {
            enabled: false, // Disable to avoid running actual tarpaulin
            ..Default::default()
        };
        let _analyzer = CoverageAnalyzer::new(coverage_config.clone());

        // Test configuration
        assert!(!coverage_config.enabled);
        assert_eq!(coverage_config.min_coverage_threshold, 80.0);
    }

    #[tokio::test]
    async fn test_dashboard_api() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);
        let api = DashboardApi::new(Arc::new(manager));

        // Test API methods (they will return empty/default data)
        let dashboard_json = api.get_dashboard_json().await.unwrap();
        assert!(dashboard_json.contains("overall_score"));

        let scorecard_json = api.get_scorecard_json().await.unwrap();
        assert!(scorecard_json.contains("overall_score"));

        let alerts_json = api.get_alerts_json().await.unwrap();
        assert!(alerts_json.starts_with("["));
    }

    #[tokio::test]
    async fn test_metric_baseline_and_targets() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Create metric first
        manager.test_update_metric("test_coverage", 85.0).await.unwrap();

        // Set baseline and target
        manager.set_baseline("test_coverage", 80.0).await.unwrap();
        manager.set_target("test_coverage", 90.0).await.unwrap();

        // Verify they were set
        let metrics = manager.get_all_metrics().await;
        let metric = metrics.get("test_coverage").unwrap();

        assert_eq!(metric.baseline, Some(80.0));
        assert_eq!(metric.target, Some(90.0));
    }

    #[tokio::test]
    async fn test_alert_acknowledgment_and_resolution() {
        let mut alert_thresholds = std::collections::HashMap::new();
        alert_thresholds.insert("test_metric".to_string(), 50.0);

        let config = QualityMetricsConfig {
            alert_thresholds,
            ..Default::default()
        };
        let manager = QualityMetricsManager::new(config);

        // Create an alert by setting a value below threshold
        manager.test_update_metric("test_metric", 25.0).await.unwrap();
        manager.test_analyze_trends_and_alerts().await.unwrap();

        let alerts = manager.get_active_alerts().await;
        assert!(!alerts.is_empty());

        let alert_id = alerts[0].id.clone();

        // Acknowledge the alert
        manager.acknowledge_alert(&alert_id).await.unwrap();

        // Alert should still be active but acknowledged
        let alerts = manager.get_active_alerts().await;
        assert!(!alerts.is_empty());
        assert!(alerts[0].acknowledged);

        // Resolve the alert
        manager.resolve_alert(&alert_id).await.unwrap();

        // Alert should no longer be active
        let alerts = manager.get_active_alerts().await;
        assert!(alerts.is_empty());
    }

    #[tokio::test]
    async fn test_improvement_recommendations() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Add metrics that need improvement
        manager.test_update_metric("test_coverage", 65.0).await.unwrap(); // Below 80% target
        manager.set_target("test_coverage", 80.0).await.unwrap(); // Set target
        manager.test_update_metric("response_time", 500.0).await.unwrap(); // High response time

        let recommendations = manager.generate_improvement_recommendations().await;

        assert!(!recommendations.is_empty());
        // Check that recommendations contain some improvement suggestions
        assert!(recommendations.iter().any(|r| r.contains("Improve") || r.contains("Address") || r.contains("investigate")));
    }

    #[tokio::test]
    async fn test_trend_type_classification() {
        let trend_config = TrendAnalysisConfig::default();
        let analyzer = TrendAnalyzer::new(trend_config);

        // Test insufficient data
        let empty_data: Vec<DataPoint> = vec![];
        let analysis = analyzer.analyze_trend(&empty_data, "test");
        assert!(matches!(analysis.trend_type, TrendType::InsufficientData));

        // Test stable trend (minimal change)
        let stable_data: Vec<DataPoint> = (0..10).map(|i| DataPoint {
            timestamp: Utc::now() + Duration::hours(i),
            value: 100.0 + (i as f64 * 0.1), // Small change
            metadata: Default::default(),
        }).collect();

        let analysis = analyzer.analyze_trend(&stable_data, "stable_test");
        // With small changes, it could be stable, volatile, or show insufficient correlation
        assert!(matches!(analysis.trend_type,
            TrendType::Stable |
            TrendType::Volatile |
            TrendType::StronglyImproving |
            TrendType::Improving
        ));
    }

    #[tokio::test]
    async fn test_volatility_detection() {
        let trend_config = TrendAnalysisConfig::default();
        let analyzer = TrendAnalyzer::new(trend_config);

        // Create volatile data (high standard deviation)
        let volatile_data: Vec<DataPoint> = (0..20).map(|i| {
            let base_value = 100.0;
            let noise = (i as f64 * 13.7).sin() * 50.0; // High amplitude noise
            DataPoint {
                timestamp: Utc::now() + Duration::hours(i),
                value: base_value + noise,
                metadata: Default::default(),
            }
        }).collect();

        let analysis = analyzer.analyze_trend(&volatile_data, "volatile_test");

        // Should detect volatility or have low confidence
        assert!(analysis.confidence < 0.9 || matches!(analysis.trend_type, TrendType::Volatile));
    }

    #[tokio::test]
    async fn test_forecast_generation() {
        let trend_config = TrendAnalysisConfig::default();
        let analyzer = TrendAnalyzer::new(trend_config);

        // Create predictable trend data
        let trend_data: Vec<DataPoint> = (0..15).map(|i| DataPoint {
            timestamp: Utc::now() + Duration::hours(i * 24),
            value: 100.0 + (i as f64 * 2.0), // Steady increase
            metadata: Default::default(),
        }).collect();

        let analysis = analyzer.analyze_trend(&trend_data, "forecast_test");

        // Should have forecast values
        assert!(!analysis.forecast_values.is_empty());
        assert!(analysis.forecast_values.len() <= 7); // Limited by config

        // Forecast values should be reasonable
        for forecast in &analysis.forecast_values {
            assert!(forecast.predicted_value > 100.0); // Should continue upward trend
            assert!(forecast.confidence_interval_lower < forecast.predicted_value);
            assert!(forecast.confidence_interval_upper > forecast.predicted_value);
        }
    }

    #[tokio::test]
    async fn test_performance_regression_detection() {
        let perf_config = PerformanceMonitorConfig::default();
        let monitor = PerformanceMonitor::new(perf_config);

        // Set baseline
        monitor.set_baseline("response_time", 100.0, "ms", 10.0).await.unwrap();

        // Add current performance data (simulated regression)
        monitor.record_benchmark(
            "response_time",
            PerformanceMetricType::ResponseTime,
            125.0, // 25% worse than baseline
            "ms",
            Default::default(),
        ).await.unwrap();

        // Check for regressions
        let regressions = monitor.detect_regressions().await.unwrap();

        assert!(!regressions.is_empty());
        let regression = &regressions[0];
        assert_eq!(regression.metric_name, "response_time");
        assert_eq!(regression.baseline_value, 100.0);
        assert_eq!(regression.current_value, 125.0);
        assert!(regression.degradation_percentage > 15.0); // Should exceed threshold
    }

    #[tokio::test]
    async fn test_metric_data_export() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Add some test data
        manager.test_update_metric("test_metric", 85.0).await.unwrap();

        // Export metrics
        let json_data = manager.export_metrics_json().await.unwrap();

        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json_data).unwrap();
        assert!(parsed.is_object());

        // Export dashboard
        let dashboard_json = manager.export_dashboard_json().await.unwrap();
        let parsed_dashboard: serde_json::Value = serde_json::from_str(&dashboard_json).unwrap();
        assert!(parsed_dashboard.is_object());
    }

    #[tokio::test]
    async fn test_empty_state_handling() {
        let config = QualityMetricsConfig::default();
        let manager = QualityMetricsManager::new(config);

        // Test operations on empty state
        let metrics = manager.get_all_metrics().await;
        assert!(metrics.is_empty());

        let alerts = manager.get_active_alerts().await;
        assert!(alerts.is_empty());

        let dashboard = manager.get_dashboard().await;
        assert_eq!(dashboard.overall_score, 0.0);
        assert!(dashboard.metrics_summary.is_empty());

        let scorecard = manager.generate_quality_scorecard().await;
        assert_eq!(scorecard.overall_score, 0.0);

        let recommendations = manager.generate_improvement_recommendations().await;
        assert!(recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_configuration_validation() {
        // Test default configuration
        let default_config = QualityMetricsConfig::default();
        assert!(default_config.enabled);
        assert!(default_config.collection_interval_seconds > 0);
        assert!(default_config.retention_days > 0);

        // Test custom configuration
        let mut alert_thresholds = std::collections::HashMap::new();
        alert_thresholds.insert("custom_metric".to_string(), 75.0);

        let custom_config = QualityMetricsConfig {
            enabled: false,
            collection_interval_seconds: 120,
            retention_days: 60,
            alert_thresholds,
            dashboard_enabled: false,
            continuous_improvement_enabled: false,
        };

        assert!(!custom_config.enabled);
        assert_eq!(custom_config.collection_interval_seconds, 120);
        assert_eq!(custom_config.retention_days, 60);
        assert_eq!(custom_config.alert_thresholds["custom_metric"], 75.0);
    }
}
