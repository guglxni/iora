//! Test Coverage Analysis Module
//!
//! This module provides comprehensive test coverage analysis and reporting
//! capabilities for the I.O.R.A. system, integrating with tarpaulin and other
//! coverage tools.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command;
use chrono::{DateTime, Utc};
use std::fs;

/// Coverage data structure from tarpaulin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageData {
    pub covered_lines: u64,
    pub total_lines: u64,
    pub coverage_percentage: f64,
    pub files: Vec<FileCoverage>,
    pub timestamp: DateTime<Utc>,
}

/// Coverage information for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    pub file_path: String,
    pub covered_lines: u64,
    pub total_lines: u64,
    pub coverage_percentage: f64,
    pub uncovered_lines: Vec<u64>,
    pub covered_lines_list: Vec<u64>,
}

/// Coverage trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageTrend {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub initial_coverage: f64,
    pub final_coverage: f64,
    pub change_percentage: f64,
    pub trend_direction: String,
}

/// Coverage analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageConfig {
    pub enabled: bool,
    pub min_coverage_threshold: f64,
    pub output_directory: String,
    pub include_tests: bool,
    pub exclude_patterns: Vec<String>,
    pub report_formats: Vec<String>,
}

impl Default for CoverageConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_coverage_threshold: 80.0,
            output_directory: "target/coverage".to_string(),
            include_tests: true,
            exclude_patterns: vec![
                "tests/*".to_string(),
                "src/main.rs".to_string(),
                "src/cli.rs".to_string(),
            ],
            report_formats: vec!["html".to_string(), "json".to_string()],
        }
    }
}

/// Test coverage analyzer
pub struct CoverageAnalyzer {
    config: CoverageConfig,
}

impl CoverageAnalyzer {
    /// Create a new coverage analyzer
    pub fn new(config: CoverageConfig) -> Self {
        Self { config }
    }

    /// Run coverage analysis using tarpaulin
    pub async fn run_coverage_analysis(&self) -> Result<CoverageData, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enabled {
            return Err("Coverage analysis is disabled".into());
        }

        // Ensure output directory exists
        fs::create_dir_all(&self.config.output_directory)?;

        // Build tarpaulin command
        let mut cmd = Command::new("cargo");
        cmd.arg("tarpaulin");

        // Add output format
        cmd.arg("--out").arg("Json");

        // Set output directory
        cmd.arg("--output-dir").arg(&self.config.output_directory);

        // Include tests if configured
        if self.config.include_tests {
            cmd.arg("--include-tests");
        }

        // Add exclude patterns
        for pattern in &self.config.exclude_patterns {
            cmd.arg("--exclude-files").arg(pattern);
        }

        // Run coverage analysis
        let output = cmd.output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Coverage analysis failed: {}", stderr).into());
        }

        // Parse coverage output
        let json_output_path = Path::new(&self.config.output_directory).join("coverage.json");
        if json_output_path.exists() {
            self.parse_tarpaulin_output(&json_output_path).await
        } else {
            // Fallback: parse stdout if JSON file not found
            let stdout = String::from_utf8_lossy(&output.stdout);
            self.parse_tarpaulin_stdout(&stdout)
        }
    }

    /// Parse tarpaulin JSON output
    async fn parse_tarpaulin_output(&self, json_path: &Path) -> Result<CoverageData, Box<dyn std::error::Error + Send + Sync>> {
        let json_content = fs::read_to_string(json_path)?;
        let tarpaulin_data: TarpaulinJson = serde_json::from_str(&json_content)?;

        let mut total_covered = 0u64;
        let mut total_lines = 0u64;
        let mut files = Vec::new();

        for file in tarpaulin_data.files {
            let covered_lines = file.covered_lines.len() as u64;
            let total_lines_file = file.total_lines as u64;
            let coverage_percentage = if total_lines_file > 0 {
                (covered_lines as f64 / total_lines_file as f64) * 100.0
            } else {
                0.0
            };

            let file_coverage = FileCoverage {
                file_path: file.name,
                covered_lines,
                total_lines: total_lines_file,
                coverage_percentage,
                uncovered_lines: file.uncovered_lines,
                covered_lines_list: file.covered_lines,
            };

            files.push(file_coverage);
            total_covered += covered_lines;
            total_lines += total_lines_file;
        }

        let overall_coverage = if total_lines > 0 {
            (total_covered as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };

        Ok(CoverageData {
            covered_lines: total_covered,
            total_lines,
            coverage_percentage: overall_coverage,
            files,
            timestamp: Utc::now(),
        })
    }

    /// Parse tarpaulin stdout (fallback)
    fn parse_tarpaulin_stdout(&self, stdout: &str) -> Result<CoverageData, Box<dyn std::error::Error + Send + Sync>> {
        // Simple regex-based parsing of tarpaulin output
        // This is a basic implementation - in practice, you'd want more robust parsing

        let mut coverage_percentage = 0.0;

        // Look for coverage percentage in output
        for line in stdout.lines() {
            if line.contains("coverage:") || line.contains("Coverage:") {
                if let Some(percent_str) = line.split('%').next() {
                    if let Some(num_str) = percent_str.split(':').last() {
                        if let Ok(percent) = num_str.trim().parse::<f64>() {
                            coverage_percentage = percent;
                            break;
                        }
                    }
                }
            }
        }

        // Create basic coverage data
        Ok(CoverageData {
            covered_lines: 0, // Not available from stdout
            total_lines: 0,
            coverage_percentage,
            files: Vec::new(),
            timestamp: Utc::now(),
        })
    }

    /// Generate coverage reports in different formats
    pub async fn generate_reports(&self, coverage_data: &CoverageData) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut reports = HashMap::new();

        for format in &self.config.report_formats {
            match format.as_str() {
                "json" => {
                    let json_report = serde_json::to_string_pretty(coverage_data)?;
                    reports.insert("json".to_string(), json_report);
                },
                "html" => {
                    let html_report = self.generate_html_report(coverage_data);
                    reports.insert("html".to_string(), html_report);
                },
                "markdown" => {
                    let md_report = self.generate_markdown_report(coverage_data);
                    reports.insert("markdown".to_string(), md_report);
                },
                _ => continue,
            }
        }

        Ok(reports)
    }

    /// Generate HTML coverage report
    fn generate_html_report(&self, coverage_data: &CoverageData) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("<title>I.O.R.A. Test Coverage Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str(".header { background: #f0f0f0; padding: 20px; border-radius: 5px; }\n");
        html.push_str(".coverage-high { color: green; }\n");
        html.push_str(".coverage-medium { color: orange; }\n");
        html.push_str(".coverage-low { color: red; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
        html.push_str("th { background-color: #f2f2f2; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Header
        html.push_str("<div class=\"header\">\n");
        html.push_str("<h1>I.O.R.A. Test Coverage Report</h1>\n");
        html.push_str(&format!("<p><strong>Overall Coverage:</strong> <span class=\"{}\">{:.2}%</span></p>\n",
            self.get_coverage_class(coverage_data.coverage_percentage),
            coverage_data.coverage_percentage));
        html.push_str(&format!("<p><strong>Generated:</strong> {}</p>\n", coverage_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
        html.push_str("</div>\n");

        // File coverage table
        html.push_str("<h2>File Coverage Details</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<tr><th>File</th><th>Coverage</th><th>Covered Lines</th><th>Total Lines</th></tr>\n");

        for file in &coverage_data.files {
            html.push_str("<tr>\n");
            html.push_str(&format!("<td>{}</td>\n", file.file_path));
            html.push_str(&format!("<td class=\"{}\">{:.2}%</td>\n",
                self.get_coverage_class(file.coverage_percentage),
                file.coverage_percentage));
            html.push_str(&format!("<td>{}</td>\n", file.covered_lines));
            html.push_str(&format!("<td>{}</td>\n", file.total_lines));
            html.push_str("</tr>\n");
        }

        html.push_str("</table>\n");
        html.push_str("</body>\n</html>\n");

        html
    }

    /// Generate Markdown coverage report
    fn generate_markdown_report(&self, coverage_data: &CoverageData) -> String {
        let mut md = String::new();

        md.push_str("# I.O.R.A. Test Coverage Report\n\n");

        md.push_str(&format!("**Overall Coverage:** {:.2}%\n\n", coverage_data.coverage_percentage));
        md.push_str(&format!("**Generated:** {}\n\n", coverage_data.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));

        md.push_str("## File Coverage Details\n\n");
        md.push_str("| File | Coverage | Covered Lines | Total Lines |\n");
        md.push_str("|------|----------|---------------|-------------|\n");

        for file in &coverage_data.files {
            md.push_str(&format!("| {} | {:.2}% | {} | {} |\n",
                file.file_path,
                file.coverage_percentage,
                file.covered_lines,
                file.total_lines));
        }

        md
    }

    /// Get CSS class for coverage percentage
    fn get_coverage_class(&self, percentage: f64) -> &'static str {
        if percentage >= 80.0 {
            "coverage-high"
        } else if percentage >= 60.0 {
            "coverage-medium"
        } else {
            "coverage-low"
        }
    }

    /// Check if coverage meets minimum threshold
    pub fn check_coverage_threshold(&self, coverage_data: &CoverageData) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if coverage_data.coverage_percentage < self.config.min_coverage_threshold {
            return Err(format!(
                "Coverage {:.2}% is below minimum threshold of {:.2}%",
                coverage_data.coverage_percentage,
                self.config.min_coverage_threshold
            ).into());
        }
        Ok(())
    }

    /// Analyze coverage trends over time
    pub fn analyze_coverage_trend(&self, historical_data: &[CoverageData]) -> Option<CoverageTrend> {
        if historical_data.len() < 2 {
            return None;
        }

        let first = historical_data.first()?;
        let last = historical_data.last()?;

        let change_percentage = if first.coverage_percentage > 0.0 {
            ((last.coverage_percentage - first.coverage_percentage) / first.coverage_percentage) * 100.0
        } else {
            0.0
        };

        let trend_direction = if change_percentage > 1.0 {
            "improving"
        } else if change_percentage < -1.0 {
            "declining"
        } else {
            "stable"
        };

        Some(CoverageTrend {
            period_start: first.timestamp,
            period_end: last.timestamp,
            initial_coverage: first.coverage_percentage,
            final_coverage: last.coverage_percentage,
            change_percentage,
            trend_direction: trend_direction.to_string(),
        })
    }

    /// Get coverage summary statistics
    pub fn get_coverage_summary(&self, coverage_data: &CoverageData) -> HashMap<String, f64> {
        let mut summary = HashMap::new();

        summary.insert("overall_coverage".to_string(), coverage_data.coverage_percentage);
        summary.insert("total_lines".to_string(), coverage_data.total_lines as f64);
        summary.insert("covered_lines".to_string(), coverage_data.covered_lines as f64);

        // Calculate file-level statistics
        let mut file_coverages: Vec<f64> = coverage_data.files.iter()
            .map(|f| f.coverage_percentage)
            .collect();

        if !file_coverages.is_empty() {
            file_coverages.sort_by(|a, b| a.partial_cmp(b).unwrap());

            summary.insert("min_file_coverage".to_string(), *file_coverages.first().unwrap());
            summary.insert("max_file_coverage".to_string(), *file_coverages.last().unwrap());
            summary.insert("median_file_coverage".to_string(), file_coverages[file_coverages.len() / 2]);
        }

        summary
    }

    /// Identify files with low coverage
    pub fn get_low_coverage_files(&self, coverage_data: &CoverageData, threshold: f64) -> Vec<FileCoverage> {
        coverage_data.files.iter()
            .filter(|f| f.coverage_percentage < threshold)
            .cloned()
            .collect()
    }

    /// Save coverage data to file
    pub async fn save_coverage_data(&self, coverage_data: &CoverageData, filename: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json_data = serde_json::to_string_pretty(coverage_data)?;
        let filepath = Path::new(&self.config.output_directory).join(filename);
        fs::write(filepath, json_data)?;
        Ok(())
    }

    /// Load coverage data from file
    pub async fn load_coverage_data(&self, filename: &str) -> Result<CoverageData, Box<dyn std::error::Error + Send + Sync>> {
        let filepath = Path::new(&self.config.output_directory).join(filename);
        let json_data = fs::read_to_string(filepath)?;
        let coverage_data: CoverageData = serde_json::from_str(&json_data)?;
        Ok(coverage_data)
    }
}

/// Tarpaulin JSON output structure
#[derive(Debug, Deserialize)]
struct TarpaulinJson {
    files: Vec<TarpaulinFile>,
}

/// Tarpaulin file structure
#[derive(Debug, Deserialize)]
struct TarpaulinFile {
    name: String,
    total_lines: usize,
    covered_lines: Vec<u64>,
    uncovered_lines: Vec<u64>,
}
