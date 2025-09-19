//! Quality Trend Analysis Module
//!
//! This module provides advanced trend analysis capabilities for quality metrics,
//! including statistical analysis, forecasting, and quality improvement recommendations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use std::cmp::Ordering;

/// Trend analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysisConfig {
    pub enabled: bool,
    pub analysis_window_days: i64,
    pub minimum_data_points: usize,
    pub confidence_threshold: f64,
    pub forecast_days: i64,
    pub seasonal_analysis: bool,
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_window_days: 30,
            minimum_data_points: 10,
            confidence_threshold: 0.8,
            forecast_days: 7,
            seasonal_analysis: false,
        }
    }
}

/// Statistical trend analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub metric_name: String,
    pub trend_type: TrendType,
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub confidence: f64,
    pub forecast_values: Vec<ForecastPoint>,
    pub seasonality_detected: bool,
    pub analysis_period_days: i64,
    pub recommendations: Vec<String>,
}

/// Forecast data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    pub timestamp: DateTime<Utc>,
    pub predicted_value: f64,
    pub confidence_interval_lower: f64,
    pub confidence_interval_upper: f64,
}

/// Trend types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrendType {
    StronglyImproving,
    Improving,
    Stable,
    Declining,
    StronglyDeclining,
    Volatile,
    InsufficientData,
}

/// Quality improvement recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRecommendation {
    pub priority: RecommendationPriority,
    pub category: String,
    pub description: String,
    pub expected_impact: f64,
    pub implementation_effort: EffortLevel,
    pub timeframe: String,
    pub prerequisites: Vec<String>,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Implementation effort levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Quality scorecard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScorecard {
    pub overall_score: f64,
    pub category_scores: HashMap<String, f64>,
    pub trend_summary: HashMap<String, TrendType>,
    pub critical_issues: Vec<String>,
    pub top_recommendations: Vec<QualityRecommendation>,
    pub generated_at: DateTime<Utc>,
}

/// Time series data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub metadata: HashMap<String, String>,
}

/// Trend analyzer
pub struct TrendAnalyzer {
    config: TrendAnalysisConfig,
}

impl TrendAnalyzer {
    /// Create a new trend analyzer
    pub fn new(config: TrendAnalysisConfig) -> Self {
        Self { config }
    }

    /// Analyze trend for a time series
    pub fn analyze_trend(&self, data_points: &[DataPoint], metric_name: &str) -> TrendAnalysis {
        if data_points.len() < self.config.minimum_data_points {
            return TrendAnalysis {
                metric_name: metric_name.to_string(),
                trend_type: TrendType::InsufficientData,
                slope: 0.0,
                intercept: 0.0,
                r_squared: 0.0,
                confidence: 0.0,
                forecast_values: Vec::new(),
                seasonality_detected: false,
                analysis_period_days: 0,
                recommendations: vec!["Collect more data points for meaningful analysis".to_string()],
            };
        }

        // Sort data points by timestamp
        let mut sorted_points = data_points.to_vec();
        sorted_points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Calculate linear regression
        let regression = self.linear_regression(&sorted_points);

        // Determine trend type
        let trend_type = self.classify_trend(&regression, &sorted_points);

        // Generate forecast
        let forecast_values = self.generate_forecast(&regression, &sorted_points);

        // Check for seasonality
        let seasonality_detected = self.config.seasonal_analysis && self.detect_seasonality(&sorted_points);

        // Calculate analysis period
        let analysis_period_days = if let (Some(first), Some(last)) = (sorted_points.first(), sorted_points.last()) {
            (last.timestamp - first.timestamp).num_days()
        } else {
            0
        };

        // Generate recommendations
        let recommendations = self.generate_recommendations(&trend_type, metric_name, regression.r_squared);

        TrendAnalysis {
            metric_name: metric_name.to_string(),
            trend_type,
            slope: regression.slope,
            intercept: regression.intercept,
            r_squared: regression.r_squared,
            confidence: regression.confidence,
            forecast_values,
            seasonality_detected,
            analysis_period_days,
            recommendations,
        }
    }

    /// Perform linear regression on data points
    fn linear_regression(&self, points: &[DataPoint]) -> RegressionResult {
        let n = points.len() as f64;

        if n < 2.0 {
            return RegressionResult {
                slope: 0.0,
                intercept: 0.0,
                r_squared: 0.0,
                confidence: 0.0,
            };
        }

        // Convert timestamps to days since first point for numerical stability
        let first_timestamp = points[0].timestamp;
        let x_values: Vec<f64> = points.iter()
            .map(|p| (p.timestamp - first_timestamp).num_seconds() as f64 / 86400.0)
            .collect();
        let y_values: Vec<f64> = points.iter().map(|p| p.value).collect();

        // Calculate means
        let x_mean = x_values.iter().sum::<f64>() / n;
        let y_mean = y_values.iter().sum::<f64>() / n;

        // Calculate slope and intercept
        let numerator: f64 = x_values.iter().zip(y_values.iter())
            .map(|(x, y)| (x - x_mean) * (y - y_mean))
            .sum();
        let denominator: f64 = x_values.iter()
            .map(|x| (x - x_mean).powi(2))
            .sum();

        let slope = if denominator != 0.0 { numerator / denominator } else { 0.0 };
        let intercept = y_mean - slope * x_mean;

        // Calculate R-squared
        let y_predicted: Vec<f64> = x_values.iter()
            .map(|x| slope * x + intercept)
            .collect();

        let ss_res: f64 = y_values.iter().zip(y_predicted.iter())
            .map(|(y, y_pred)| (y - y_pred).powi(2))
            .sum();
        let ss_tot: f64 = y_values.iter()
            .map(|y| (y - y_mean).powi(2))
            .sum();

        let r_squared = if ss_tot != 0.0 { 1.0 - (ss_res / ss_tot) } else { 0.0 };

        // Calculate confidence (simplified)
        let confidence = if r_squared > 0.7 {
            0.9
        } else if r_squared > 0.5 {
            0.7
        } else if r_squared > 0.3 {
            0.5
        } else {
            0.3
        };

        RegressionResult {
            slope,
            intercept,
            r_squared,
            confidence,
        }
    }

    /// Classify trend based on regression and data characteristics
    fn classify_trend(&self, regression: &RegressionResult, points: &[DataPoint]) -> TrendType {
        let slope_threshold = 0.01; // Adjust based on metric scale
        let r_squared_threshold = 0.5;

        // Check if trend is statistically significant
        if regression.r_squared < r_squared_threshold {
            // Check for volatility
            let volatility = self.calculate_volatility(points);
            if volatility > 0.2 {
                return TrendType::Volatile;
            } else {
                return TrendType::Stable;
            }
        }

        // Classify based on slope
        match regression.slope.abs() {
            s if s > slope_threshold * 3.0 => {
                if regression.slope > 0.0 {
                    TrendType::StronglyImproving
                } else {
                    TrendType::StronglyDeclining
                }
            },
            s if s > slope_threshold => {
                if regression.slope > 0.0 {
                    TrendType::Improving
                } else {
                    TrendType::Declining
                }
            },
            _ => TrendType::Stable,
        }
    }

    /// Calculate volatility (coefficient of variation)
    fn calculate_volatility(&self, points: &[DataPoint]) -> f64 {
        if points.is_empty() {
            return 0.0;
        }

        let values: Vec<f64> = points.iter().map(|p| p.value).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        if mean != 0.0 {
            std_dev / mean
        } else {
            0.0
        }
    }

    /// Generate forecast values
    fn generate_forecast(&self, regression: &RegressionResult, points: &[DataPoint]) -> Vec<ForecastPoint> {
        if points.is_empty() {
            return Vec::new();
        }

        let last_timestamp = points.last().unwrap().timestamp;
        let forecast_interval = Duration::hours(24 / (self.config.forecast_days as i64 / 7)); // Daily intervals

        let mut forecast_values = Vec::new();

        for i in 1..=self.config.forecast_days {
            let forecast_timestamp = last_timestamp + forecast_interval * i as i32;
            let days_since_first = (forecast_timestamp - points[0].timestamp).num_seconds() as f64 / 86400.0;

            let predicted_value = regression.slope * days_since_first + regression.intercept;

            // Simple confidence interval calculation
            let confidence_range = predicted_value * 0.1; // 10% confidence interval

            forecast_values.push(ForecastPoint {
                timestamp: forecast_timestamp,
                predicted_value,
                confidence_interval_lower: predicted_value - confidence_range,
                confidence_interval_upper: predicted_value + confidence_range,
            });
        }

        forecast_values
    }

    /// Detect seasonality in data
    fn detect_seasonality(&self, points: &[DataPoint]) -> bool {
        if points.len() < 14 { // Need at least 2 weeks of data
            return false;
        }

        // Simple autocorrelation-based seasonality detection
        // This is a basic implementation - production systems would use more sophisticated methods
        let values: Vec<f64> = points.iter().map(|p| p.value).collect();

        // Check for weekly pattern (lag 7)
        if values.len() >= 14 {
            let correlation = self.autocorrelation(&values, 7);
            if correlation.abs() > 0.6 {
                return true;
            }
        }

        false
    }

    /// Calculate autocorrelation at given lag
    fn autocorrelation(&self, values: &[f64], lag: usize) -> f64 {
        if values.len() <= lag {
            return 0.0;
        }

        let n = values.len() - lag;
        let mean = values.iter().sum::<f64>() / values.len() as f64;

        let numerator: f64 = (0..n)
            .map(|i| (values[i] - mean) * (values[i + lag] - mean))
            .sum();

        let denominator: f64 = (0..values.len())
            .map(|i| (values[i] - mean).powi(2))
            .sum();

        if denominator != 0.0 {
            numerator / denominator
        } else {
            0.0
        }
    }

    /// Generate recommendations based on trend analysis
    fn generate_recommendations(&self, trend_type: &TrendType, metric_name: &str, r_squared: f64) -> Vec<String> {
        let mut recommendations = Vec::new();

        match trend_type {
            TrendType::StronglyDeclining | TrendType::Declining => {
                recommendations.push(format!("Address declining trend in {}", metric_name));
                if r_squared > 0.7 {
                    recommendations.push("Trend is statistically significant - investigate root causes".to_string());
                }
            },
            TrendType::Stable => {
                recommendations.push(format!("{} is stable - monitor for changes", metric_name));
            },
            TrendType::Improving | TrendType::StronglyImproving => {
                recommendations.push(format!("Continue positive trend in {}", metric_name));
            },
            TrendType::Volatile => {
                recommendations.push(format!("Reduce volatility in {} measurements", metric_name));
                recommendations.push("Implement more stable measurement processes".to_string());
            },
            TrendType::InsufficientData => {
                recommendations.push(format!("Collect more data points for {} analysis", metric_name));
            },
        }

        recommendations
    }

    /// Generate quality scorecard
    pub fn generate_scorecard(
        &self,
        metric_analyses: HashMap<String, TrendAnalysis>,
        current_values: HashMap<String, f64>,
    ) -> QualityScorecard {
        let mut category_scores = HashMap::new();
        let mut trend_summary = HashMap::new();
        let mut critical_issues = Vec::new();

        // Categorize metrics and calculate scores
        let mut test_quality_score = 0.0;
        let mut performance_score = 0.0;
        let mut reliability_score = 0.0;
        let mut test_count = 0;
        let mut perf_count = 0;
        let mut reliability_count = 0;

        for (metric_name, analysis) in &metric_analyses {
            trend_summary.insert(metric_name.clone(), analysis.trend_type.clone());

            // Categorize and score
            if metric_name.contains("test") || metric_name.contains("coverage") {
                test_quality_score += self.trend_to_score(&analysis.trend_type);
                test_count += 1;
            } else if metric_name.contains("response") || metric_name.contains("throughput") ||
                      metric_name.contains("memory") || metric_name.contains("cpu") {
                performance_score += self.trend_to_score(&analysis.trend_type);
                perf_count += 1;
            } else if metric_name.contains("error") || metric_name.contains("health") ||
                      metric_name.contains("api") {
                reliability_score += self.trend_to_score(&analysis.trend_type);
                reliability_count += 1;
            }

            // Check for critical issues
            if matches!(analysis.trend_type, TrendType::StronglyDeclining) {
                critical_issues.push(format!("Critical decline in {}", metric_name));
            }
        }

        // Normalize category scores
        if test_count > 0 {
            category_scores.insert("Test Quality".to_string(), test_quality_score / test_count as f64);
        }
        if perf_count > 0 {
            category_scores.insert("Performance".to_string(), performance_score / perf_count as f64);
        }
        if reliability_count > 0 {
            category_scores.insert("Reliability".to_string(), reliability_score / reliability_count as f64);
        }

        // Calculate overall score
        let overall_score = category_scores.values().sum::<f64>() / category_scores.len().max(1) as f64;

        // Generate top recommendations
        let top_recommendations = self.generate_top_recommendations(&metric_analyses);

        QualityScorecard {
            overall_score,
            category_scores,
            trend_summary,
            critical_issues,
            top_recommendations,
            generated_at: Utc::now(),
        }
    }

    /// Convert trend type to numerical score
    fn trend_to_score(&self, trend_type: &TrendType) -> f64 {
        match trend_type {
            TrendType::StronglyImproving => 95.0,
            TrendType::Improving => 80.0,
            TrendType::Stable => 70.0,
            TrendType::Declining => 50.0,
            TrendType::StronglyDeclining => 30.0,
            TrendType::Volatile => 40.0,
            TrendType::InsufficientData => 60.0,
        }
    }

    /// Generate top recommendations based on analysis
    fn generate_top_recommendations(&self, analyses: &HashMap<String, TrendAnalysis>) -> Vec<QualityRecommendation> {
        let mut recommendations = Vec::new();

        // Find declining metrics
        for (metric_name, analysis) in analyses {
            if matches!(analysis.trend_type, TrendType::StronglyDeclining | TrendType::Declining) {
                let recommendation = QualityRecommendation {
                    priority: RecommendationPriority::High,
                    category: "Quality Improvement".to_string(),
                    description: format!("Address declining trend in {}", metric_name),
                    expected_impact: 15.0,
                    implementation_effort: EffortLevel::Medium,
                    timeframe: "2-4 weeks".to_string(),
                    prerequisites: vec!["Root cause analysis".to_string()],
                };
                recommendations.push(recommendation);
            }
        }

        // Add general recommendations
        if analyses.len() < 5 {
            recommendations.push(QualityRecommendation {
                priority: RecommendationPriority::Medium,
                category: "Data Collection".to_string(),
                description: "Increase metric collection frequency".to_string(),
                expected_impact: 10.0,
                implementation_effort: EffortLevel::Low,
                timeframe: "1 week".to_string(),
                prerequisites: Vec::new(),
            });
        }

        // Sort by priority and impact
        recommendations.sort_by(|a, b| {
            match (a.priority.clone(), b.priority.clone()) {
                (RecommendationPriority::Critical, _) => Ordering::Less,
                (_, RecommendationPriority::Critical) => Ordering::Greater,
                (RecommendationPriority::High, _) => Ordering::Less,
                (_, RecommendationPriority::High) => Ordering::Greater,
                _ => b.expected_impact.partial_cmp(&a.expected_impact).unwrap_or(Ordering::Equal),
            }
        });

        recommendations.into_iter().take(5).collect()
    }
}

/// Linear regression result
struct RegressionResult {
    slope: f64,
    intercept: f64,
    r_squared: f64,
    confidence: f64,
}
