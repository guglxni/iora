//! Quality Metrics Dashboard Module
//!
//! This module provides web-based dashboard functionality for visualizing
//! quality metrics, trends, and alerts from the I.O.R.A. system.

use crate::modules::quality_metrics::{QualityMetricsManager, QualityDashboard, QualityAlert};
use crate::modules::trend_analysis::QualityScorecard;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use warp::Filter;

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub refresh_interval_seconds: u64,
    pub authentication_required: bool,
    pub api_key: Option<String>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            host: "127.0.0.1".to_string(),
            port: 8080,
            refresh_interval_seconds: 30,
            authentication_required: false,
            api_key: None,
        }
    }
}

/// Dashboard server
pub struct QualityDashboardServer {
    config: DashboardConfig,
    metrics_manager: Arc<QualityMetricsManager>,
    server_handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl QualityDashboardServer {
    /// Create a new dashboard server
    pub fn new(config: DashboardConfig, metrics_manager: Arc<QualityMetricsManager>) -> Self {
        Self {
            config,
            metrics_manager,
            server_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Start the dashboard server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enabled {
            return Ok(());
        }

        let metrics_manager = Arc::clone(&self.metrics_manager);
        let config = self.config.clone();

        let routes = self.create_routes(metrics_manager);

        let addr = format!("{}:{}", config.host, config.port);
        println!("🚀 Starting Quality Dashboard on http://{}", addr);

        let server = warp::serve(routes)
            .run(([127, 0, 0, 1], config.port));

        let handle = tokio::spawn(server);
        *self.server_handle.write().await = Some(handle);

        Ok(())
    }

    /// Stop the dashboard server
    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(handle) = self.server_handle.write().await.take() {
            handle.abort();
            println!("✅ Quality Dashboard stopped");
        }
        Ok(())
    }

    /// Create warp routes for the dashboard
    fn create_routes(&self, metrics_manager: Arc<QualityMetricsManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let metrics_manager_filter = warp::any().map(move || Arc::clone(&metrics_manager));

        let dashboard_route = warp::path::end()
            .and(metrics_manager_filter.clone())
            .and_then(Self::handle_dashboard);

        let api_route = warp::path("api")
            .and(warp::path("metrics"))
            .and(warp::path::end())
            .and(metrics_manager_filter.clone())
            .and_then(Self::handle_api_metrics);

        let api_scorecard = warp::path("api")
            .and(warp::path("scorecard"))
            .and(warp::path::end())
            .and(metrics_manager_filter.clone())
            .and_then(Self::handle_api_scorecard);

        let api_alerts = warp::path("api")
            .and(warp::path("alerts"))
            .and(warp::path::end())
            .and(metrics_manager_filter.clone())
            .and_then(Self::handle_api_alerts);

        let static_files = warp::path("static")
            .and(warp::fs::dir("./dashboard/static"));

        dashboard_route
            .or(api_route)
            .or(api_scorecard)
            .or(api_alerts)
            .or(static_files)
    }

    /// Handle dashboard page request
    async fn handle_dashboard(metrics_manager: Arc<QualityMetricsManager>) -> Result<impl warp::Reply, warp::Rejection> {
        let dashboard = metrics_manager.get_dashboard().await;
        let scorecard = metrics_manager.generate_quality_scorecard().await;

        let html = Self::generate_dashboard_html(&dashboard, &scorecard, 30); // Default 30 second refresh
        Ok(warp::reply::html(html))
    }

    /// Handle API metrics request
    async fn handle_api_metrics(metrics_manager: Arc<QualityMetricsManager>) -> Result<impl warp::Reply, warp::Rejection> {
        let dashboard = metrics_manager.get_dashboard().await;
        Ok(warp::reply::json(&dashboard))
    }

    /// Handle API scorecard request
    async fn handle_api_scorecard(metrics_manager: Arc<QualityMetricsManager>) -> Result<impl warp::Reply, warp::Rejection> {
        let scorecard = metrics_manager.generate_quality_scorecard().await;
        Ok(warp::reply::json(&scorecard))
    }

    /// Handle API alerts request
    async fn handle_api_alerts(metrics_manager: Arc<QualityMetricsManager>) -> Result<impl warp::Reply, warp::Rejection> {
        let alerts = metrics_manager.get_active_alerts().await;
        Ok(warp::reply::json(&alerts))
    }

    /// Generate dashboard HTML
    fn generate_dashboard_html(dashboard: &QualityDashboard, scorecard: &QualityScorecard, refresh_seconds: u64) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("    <title>I.O.R.A. Quality Dashboard</title>\n");
        html.push_str("    <style>\n");
        html.push_str(Self::get_css_styles());
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <header>\n");
        html.push_str("            <h1>🚀 I.O.R.A. Quality Dashboard</h1>\n");
        html.push_str(&format!("            <p class=\"last-updated\">Last updated: {}</p>\n",
            dashboard.last_updated.format("%Y-%m-%d %H:%M:%S UTC")));
        html.push_str("        </header>\n");

        // Overall score
        html.push_str("        <section class=\"overall-score\">\n");
        html.push_str(&format!("            <h2>Overall Quality Score</h2>\n"));
        html.push_str(&format!("            <div class=\"score-display {}\">{:.1}%</div>\n",
            Self::get_score_class(scorecard.overall_score),
            scorecard.overall_score));
        html.push_str("        </section>\n");

        // Category scores
        html.push_str("        <section class=\"category-scores\">\n");
        html.push_str("            <h2>Category Scores</h2>\n");
        html.push_str("            <div class=\"score-grid\">\n");

        for (category, score) in &scorecard.category_scores {
            html.push_str(&format!("                <div class=\"score-card {}\">\n", Self::get_score_class(*score)));
            html.push_str(&format!("                    <h3>{}</h3>\n", category));
            html.push_str(&format!("                    <div class=\"score\">{:.1}%</div>\n", score));
            html.push_str("                </div>\n");
        }

        html.push_str("            </div>\n");
        html.push_str("        </section>\n");

        // Active alerts
        if !dashboard.alerts.is_empty() {
            html.push_str("        <section class=\"alerts\">\n");
            html.push_str("            <h2>⚠️ Active Alerts</h2>\n");
            html.push_str("            <div class=\"alerts-list\">\n");

            for alert in &dashboard.alerts {
                if !alert.resolved {
                    html.push_str(&format!("                <div class=\"alert {}\">\n", Self::get_alert_class(&alert.severity)));
                    html.push_str(&format!("                    <h4>{}</h4>\n", alert.message));
                    html.push_str(&format!("                    <p>{}</p>\n", alert.details));
                    html.push_str(&format!("                    <small>{}</small>\n",
                        alert.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
                    html.push_str("                </div>\n");
                }
            }

            html.push_str("            </div>\n");
            html.push_str("        </section>\n");
        }

        // Critical issues
        if !scorecard.critical_issues.is_empty() {
            html.push_str("        <section class=\"critical-issues\">\n");
            html.push_str("            <h2>🚨 Critical Issues</h2>\n");
            html.push_str("            <ul>\n");

            for issue in &scorecard.critical_issues {
                html.push_str(&format!("                <li>{}</li>\n", issue));
            }

            html.push_str("            </ul>\n");
            html.push_str("        </section>\n");
        }

        // Top recommendations
        if !scorecard.top_recommendations.is_empty() {
            html.push_str("        <section class=\"recommendations\">\n");
            html.push_str("            <h2>💡 Top Recommendations</h2>\n");
            html.push_str("            <div class=\"recommendations-list\">\n");

            for recommendation in &scorecard.top_recommendations {
                html.push_str("                <div class=\"recommendation\">\n");
                html.push_str(&format!("                    <h4>{} - {} Impact</h4>\n",
                    Self::get_priority_label(&recommendation.priority),
                    Self::get_effort_label(&recommendation.implementation_effort)));
                html.push_str(&format!("                    <p>{}</p>\n", recommendation.description));
                html.push_str(&format!("                    <small>Timeframe: {} | Expected Impact: {:.0}%</small>\n",
                    recommendation.timeframe,
                    recommendation.expected_impact));
                html.push_str("                </div>\n");
            }

            html.push_str("            </div>\n");
            html.push_str("        </section>\n");
        }

        // Metrics details
        html.push_str("        <section class=\"metrics-details\">\n");
        html.push_str("            <h2>Metrics Details</h2>\n");
        html.push_str("            <div class=\"metrics-grid\">\n");

        for (name, metric) in &dashboard.metrics_summary {
            if let Some(current) = &metric.current_value {
                html.push_str("                <div class=\"metric-card\">\n");
                html.push_str(&format!("                    <h4>{}</h4>\n", name));
                html.push_str(&format!("                    <div class=\"metric-value\">{:.2} {}</div>\n",
                    current.value, metric.unit));
                html.push_str(&format!("                    <small>Trend: {}</small>\n",
                    Self::get_trend_label(&metric.trend)));
                html.push_str("                </div>\n");
            }
        }

        html.push_str("            </div>\n");
        html.push_str("        </section>\n");

        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("        setTimeout(() => location.reload(), {});\n", refresh_seconds * 1000));
        html.push_str("    </script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }

    /// Get CSS styles for the dashboard
    fn get_css_styles() -> &'static str {
        r#"
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #f5f5f5;
            color: #333;
            line-height: 1.6;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }

        header {
            text-align: center;
            margin-bottom: 30px;
            padding: 20px;
            background: white;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }

        h1 {
            color: #2c3e50;
            margin-bottom: 10px;
        }

        .last-updated {
            color: #7f8c8d;
            font-size: 0.9em;
        }

        section {
            margin-bottom: 30px;
            padding: 20px;
            background: white;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }

        h2 {
            color: #2c3e50;
            margin-bottom: 20px;
            border-bottom: 2px solid #ecf0f1;
            padding-bottom: 10px;
        }

        .overall-score {
            text-align: center;
        }

        .score-display {
            font-size: 4em;
            font-weight: bold;
            margin: 20px 0;
            padding: 20px;
            border-radius: 50%;
            display: inline-block;
            min-width: 150px;
            min-height: 150px;
            line-height: 110px;
        }

        .score-excellent { background: linear-gradient(135deg, #27ae60, #2ecc71); color: white; }
        .score-good { background: linear-gradient(135deg, #f39c12, #e67e22); color: white; }
        .score-fair { background: linear-gradient(135deg, #e74c3c, #c0392b); color: white; }
        .score-poor { background: linear-gradient(135deg, #8e44ad, #9b59b6); color: white; }

        .score-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
        }

        .score-card {
            text-align: center;
            padding: 20px;
            border-radius: 10px;
            color: white;
        }

        .score-card h3 {
            margin-bottom: 10px;
            font-size: 1.1em;
        }

        .score-card .score {
            font-size: 2em;
            font-weight: bold;
        }

        .alerts-list, .recommendations-list {
            display: grid;
            gap: 15px;
        }

        .alert, .recommendation {
            padding: 15px;
            border-radius: 8px;
            border-left: 4px solid;
        }

        .alert-critical { background: #fee; border-left-color: #e74c3c; }
        .alert-high { background: #fef5e7; border-left-color: #f39c12; }
        .alert-medium { background: #fef9e7; border-left-color: #f1c40f; }
        .alert-low { background: #f0f9ff; border-left-color: #3498db; }

        .alert h4, .recommendation h4 {
            margin-bottom: 8px;
            font-size: 1em;
        }

        .alert p, .recommendation p {
            margin-bottom: 5px;
        }

        .alert small, .recommendation small {
            color: #7f8c8d;
            font-size: 0.8em;
        }

        .critical-issues ul {
            padding-left: 20px;
        }

        .critical-issues li {
            margin-bottom: 10px;
            color: #e74c3c;
        }

        .metrics-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }

        .metric-card {
            padding: 15px;
            border: 1px solid #ecf0f1;
            border-radius: 8px;
            text-align: center;
        }

        .metric-card h4 {
            margin-bottom: 10px;
            color: #2c3e50;
        }

        .metric-value {
            font-size: 1.5em;
            font-weight: bold;
            color: #27ae60;
            margin-bottom: 5px;
        }

        .metric-card small {
            color: #7f8c8d;
        }

        @media (max-width: 768px) {
            .container {
                padding: 10px;
            }

            .score-grid, .metrics-grid {
                grid-template-columns: 1fr;
            }

            .score-display {
                font-size: 3em;
                min-width: 120px;
                min-height: 120px;
                line-height: 80px;
            }
        }
        "#
    }

    /// Get CSS class for score
    fn get_score_class(score: f64) -> &'static str {
        if score >= 90.0 {
            "score-excellent"
        } else if score >= 75.0 {
            "score-good"
        } else if score >= 60.0 {
            "score-fair"
        } else {
            "score-poor"
        }
    }

    /// Get CSS class for alert severity
    fn get_alert_class(severity: &crate::modules::quality_metrics::AlertSeverity) -> &'static str {
        match severity {
            crate::modules::quality_metrics::AlertSeverity::Critical => "alert-critical",
            crate::modules::quality_metrics::AlertSeverity::High => "alert-high",
            crate::modules::quality_metrics::AlertSeverity::Medium => "alert-medium",
            crate::modules::quality_metrics::AlertSeverity::Low => "alert-low",
            crate::modules::quality_metrics::AlertSeverity::Info => "alert-low",
        }
    }

    /// Get priority label
    fn get_priority_label(priority: &crate::modules::trend_analysis::RecommendationPriority) -> &'static str {
        match priority {
            crate::modules::trend_analysis::RecommendationPriority::Critical => "Critical",
            crate::modules::trend_analysis::RecommendationPriority::High => "High",
            crate::modules::trend_analysis::RecommendationPriority::Medium => "Medium",
            crate::modules::trend_analysis::RecommendationPriority::Low => "Low",
        }
    }

    /// Get effort label
    fn get_effort_label(effort: &crate::modules::trend_analysis::EffortLevel) -> &'static str {
        match effort {
            crate::modules::trend_analysis::EffortLevel::Low => "Low Effort",
            crate::modules::trend_analysis::EffortLevel::Medium => "Medium Effort",
            crate::modules::trend_analysis::EffortLevel::High => "High Effort",
            crate::modules::trend_analysis::EffortLevel::VeryHigh => "Very High Effort",
        }
    }

    /// Get trend label
    fn get_trend_label(trend: &crate::modules::quality_metrics::TrendDirection) -> &'static str {
        match trend {
            crate::modules::quality_metrics::TrendDirection::Improving => "↗️ Improving",
            crate::modules::quality_metrics::TrendDirection::Declining => "↘️ Declining",
            crate::modules::quality_metrics::TrendDirection::Stable => "➡️ Stable",
            crate::modules::quality_metrics::TrendDirection::Unknown => "? Unknown",
        }
    }
}

/// Dashboard API endpoints for external integration
pub struct DashboardApi {
    metrics_manager: Arc<QualityMetricsManager>,
}

impl DashboardApi {
    /// Create a new dashboard API
    pub fn new(metrics_manager: Arc<QualityMetricsManager>) -> Self {
        Self { metrics_manager }
    }

    /// Get dashboard data as JSON
    pub async fn get_dashboard_json(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let dashboard = self.metrics_manager.get_dashboard().await;
        serde_json::to_string_pretty(&dashboard).map_err(|e| e.into())
    }

    /// Get scorecard data as JSON
    pub async fn get_scorecard_json(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let scorecard = self.metrics_manager.generate_quality_scorecard().await;
        serde_json::to_string_pretty(&scorecard).map_err(|e| e.into())
    }

    /// Get alerts data as JSON
    pub async fn get_alerts_json(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let alerts = self.metrics_manager.get_active_alerts().await;
        serde_json::to_string_pretty(&alerts).map_err(|e| e.into())
    }

    /// Export dashboard data for external tools
    pub async fn export_dashboard_data(&self) -> Result<DashboardExportData, Box<dyn std::error::Error + Send + Sync>> {
        let dashboard = self.metrics_manager.get_dashboard().await;
        let scorecard = self.metrics_manager.generate_quality_scorecard().await;
        let alerts = self.metrics_manager.get_active_alerts().await;

        Ok(DashboardExportData {
            dashboard,
            scorecard,
            alerts,
            exported_at: Utc::now(),
        })
    }
}

/// Data structure for dashboard export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardExportData {
    pub dashboard: QualityDashboard,
    pub scorecard: QualityScorecard,
    pub alerts: Vec<QualityAlert>,
    pub exported_at: DateTime<Utc>,
}
