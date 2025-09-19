# I.O.R.A. Test Automation Framework

## Overview

This document describes the comprehensive test automation framework implemented for the I.O.R.A. system, providing automated testing capabilities across all levels of the testing pyramid.

## Framework Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Test Automation Framework                │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │  Test Runner    │  │ Test Discovery  │  │ Test Config │ │
│  │  (cargo test)   │  │  & Execution    │  │ Management  │ │
│  └─────────────────┘  └─────────────────┘  └─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ Test Utilities  │  │ Mock Framework │  │ Data Factory│ │
│  │ & Helpers       │  │                 │  │             │ │
│  └─────────────────┘  └─────────────────┘  └─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐ │
│  │ CI/CD Pipeline  │  │ Quality Gates   │  │ Reporting   │ │
│  │ Integration     │  │                 │  │ Framework   │ │
│  └─────────────────┘  └─────────────────┘  └─────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Test Runner (`cargo test`)

#### Configuration
```toml
# Cargo.toml test configuration
[package]
name = "iora"

[[test]]
name = "integration_tests"
path = "tests/integration_tests.rs"

[[test]]
name = "functional_quality_tests"
path = "tests/functional_quality_tests.rs"

# Test-specific dependencies
[dev-dependencies]
tokio-test = "0.4"
tempfile = "3.0"
```

#### Execution Options
```bash
# Basic execution
cargo test

# Specific test execution
cargo test test_name
cargo test --test integration_tests
cargo test --lib data_processing

# Advanced options
cargo test --release -- --nocapture --test-threads=1
cargo test -- --ignored  # Run ignored tests
cargo test -- --include-ignored  # Run all including ignored
```

### 2. Test Discovery and Organization

#### Directory Structure
```
tests/
├── common/
│   ├── mod.rs              # Shared test utilities
│   ├── fixtures.rs         # Test data fixtures
│   ├── mocks.rs            # Mock implementations
│   └── helpers.rs          # Test helper functions
├── unit_tests.rs           # Unit tests (27 tests)
├── integration_tests.rs    # Integration tests (21 tests)
├── functional_quality_tests.rs  # Functional tests (18 tests)
├── resilience_tests.rs     # Resilience tests (12 tests)
├── performance_tests.rs    # Performance tests (8 tests)
├── deployment_tests.rs     # Deployment tests (6 tests)
├── operational_readiness_tests.rs  # Operational tests (5 tests)
├── production_validation_tests.rs  # Production tests (4 tests)
└── quality_metrics_tests.rs  # Quality metrics tests (14 tests)
```

#### Test Organization Patterns

```rust
// tests/common/mod.rs - Shared test infrastructure
pub mod fixtures;
pub mod mocks;
pub mod helpers;

pub use fixtures::*;
pub use mocks::*;
pub use helpers::*;

// Test module organization
#[cfg(test)]
mod tests {
    use super::*;
    use iora::modules::*;

    // Unit tests
    mod unit {
        // Pure logic tests
    }

    // Integration tests
    mod integration {
        // Component interaction tests
    }

    // Property tests
    mod property {
        use proptest::*;
        // Property-based testing
    }
}
```

### 3. Test Configuration Management

#### Environment-based Configuration
```rust
// tests/common/config.rs
use std::env;

pub struct TestConfig {
    pub api_timeout: u64,
    pub database_url: String,
    pub mock_external_apis: bool,
    pub performance_baseline: f64,
}

impl TestConfig {
    pub fn from_env() -> Self {
        Self {
            api_timeout: env::var("TEST_API_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            database_url: env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/iora_test".to_string()),
            mock_external_apis: env::var("MOCK_EXTERNAL_APIS")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            performance_baseline: env::var("PERFORMANCE_BASELINE")
                .unwrap_or_else(|_| "100.0".to_string())
                .parse()
                .unwrap_or(100.0),
        }
    }

    pub fn for_unit_tests() -> Self {
        Self {
            mock_external_apis: true,
            ..Self::from_env()
        }
    }

    pub fn for_integration_tests() -> Self {
        Self {
            mock_external_apis: false,
            ..Self::from_env()
        }
    }
}
```

#### Test Profile Configuration
```toml
# tests/config/profiles.toml
[unit]
mock_all = true
timeout_seconds = 5
parallel_execution = true

[integration]
mock_external = false
timeout_seconds = 30
parallel_execution = false

[performance]
mock_external = true
timeout_seconds = 300
parallel_execution = false
iterations = 1000

[stress]
mock_external = false
timeout_seconds = 600
parallel_execution = true
concurrent_users = 100
```

## Test Utilities and Helpers

### Common Test Helpers

```rust
// tests/common/helpers.rs
use std::sync::Arc;
use tokio::sync::Mutex;

/// Async test setup helper
pub async fn with_test_setup<F, Fut, T>(test_fn: F) -> T
where
    F: FnOnce(TestContext) -> Fut,
    Fut: Future<Output = T>,
{
    let context = TestContext::setup().await;
    let result = test_fn(context.clone()).await;
    context.cleanup().await;
    result
}

/// Retry helper for flaky operations
pub async fn retry_async<F, Fut, T>(
    operation: F,
    max_attempts: usize,
    delay_ms: u64
) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
{
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_attempts {
                    tokio::time::sleep(Duration::from_millis(delay_ms * attempt as u64)).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
}

/// Performance measurement helper
pub async fn measure_performance<F, Fut, T>(
    operation: F
) -> (T, std::time::Duration)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let start = std::time::Instant::now();
    let result = operation().await;
    let duration = start.elapsed();
    (result, duration)
}
```

### Mock Framework

```rust
// tests/common/mocks.rs
use mockito::{mock, server_url};
use serde_json::json;

/// API mock server setup
pub struct ApiMockServer {
    server: mockito::ServerGuard,
    mocks: Vec<mockito::Mock>,
}

impl ApiMockServer {
    pub async fn new() -> Self {
        let server = mockito::Server::new_async().await;

        Self {
            server,
            mocks: Vec::new(),
        }
    }

    pub fn url(&self) -> String {
        self.server.url()
    }

    pub fn mock_coin_gecko_price(&mut self, symbol: &str, price: f64) {
        let mock = mock("GET", format!("/api/v3/simple/price?ids={}&vs_currencies=usd", symbol).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                symbol.to_lowercase(): {
                    "usd": price
                }
            }).to_string())
            .create();

        self.mocks.push(mock);
    }

    pub fn mock_blockchain_rpc(&mut self, method: &str, result: serde_json::Value) {
        let mock = mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "result": result
            }).to_string())
            .create();

        self.mocks.push(mock);
    }
}

impl Drop for ApiMockServer {
    fn drop(&mut self) {
        // Mocks are automatically cleaned up
    }
}
```

### Test Data Factory

```rust
// tests/common/fixtures.rs
use chrono::{DateTime, Utc};

/// Cryptocurrency test data
#[derive(Clone, Debug)]
pub struct CryptoTestData {
    pub symbol: String,
    pub price: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub price_change_24h: f64,
    pub last_updated: DateTime<Utc>,
}

impl CryptoTestData {
    pub fn bitcoin() -> Self {
        Self {
            symbol: "BTC".to_string(),
            price: 50000.0,
            volume_24h: 25000000.0,
            market_cap: 950000000000.0,
            price_change_24h: 2.5,
            last_updated: Utc::now(),
        }
    }

    pub fn ethereum() -> Self {
        Self {
            symbol: "ETH".to_string(),
            price: 3000.0,
            volume_24h: 15000000.0,
            market_cap: 360000000000.0,
            price_change_24h: -1.2,
            last_updated: Utc::now(),
        }
    }

    pub fn with_price(mut self, price: f64) -> Self {
        self.price = price;
        self
    }

    pub fn with_volume(mut self, volume: f64) -> Self {
        self.volume_24h = volume;
        self
    }

    pub fn stale(mut self) -> Self {
        self.last_updated = Utc::now() - chrono::Duration::hours(2);
        self
    }
}

/// Test data factory
pub struct TestDataFactory;

impl TestDataFactory {
    pub fn crypto_data_set() -> Vec<CryptoTestData> {
        vec![
            CryptoTestData::bitcoin(),
            CryptoTestData::ethereum(),
            CryptoTestData::bitcoin().with_symbol("ADA").with_price(1.5),
        ]
    }

    pub fn invalid_crypto_data() -> Vec<CryptoTestData> {
        vec![
            CryptoTestData::bitcoin().with_price(-100.0), // Invalid negative price
            CryptoTestData::bitcoin().with_symbol(""),     // Empty symbol
            CryptoTestData::bitcoin().stale(),             // Stale data
        ]
    }

    pub async fn generate_realistic_dataset(size: usize) -> Vec<CryptoTestData> {
        let mut data = Vec::with_capacity(size);

        for i in 0..size {
            let symbol = format!("CRYPTO{}", i);
            let price = 100.0 + (i as f64 * 10.0) + (rand::random::<f64>() * 50.0);
            let volume = 1000000.0 + (rand::random::<f64>() * 9000000.0);

            data.push(CryptoTestData {
                symbol,
                price,
                volume_24h: volume,
                market_cap: price * 1000000.0, // Simplified calculation
                price_change_24h: (rand::random::<f64>() - 0.5) * 10.0, // -5% to +5%
                last_updated: Utc::now() - chrono::Duration::minutes(rand::random::<i64>() % 60),
            });
        }

        data
    }
}
```

## CI/CD Integration

### GitHub Actions Integration

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on:
  push:
    branches: [ main, master, develop ]
  pull_request:
    branches: [ main, master, develop ]

jobs:
  test-matrix:
    strategy:
      matrix:
        test-type: [unit, integration, functional, resilience, performance, quality-metrics]
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy

    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

    - name: Run ${{ matrix.test-type }} tests
      run: |
        case ${{ matrix.test-type }} in
          unit)
            cargo test --lib --verbose
            ;;
          integration)
            cargo test --test integration_tests --verbose
            ;;
          functional)
            cargo test --test functional_quality_tests --verbose
            ;;
          resilience)
            cargo test --test resilience_tests --verbose
            ;;
          performance)
            cargo test --test performance_tests --release -- --nocapture
            ;;
          quality-metrics)
            cargo test --test quality_metrics_tests --verbose
            ;;
        esac

    - name: Upload test results
      uses: actions/upload-artifact@v3
      if: always()
      with:
        name: test-results-${{ matrix.test-type }}
        path: |
          target/debug/deps/*test*.json
          target/debug/deps/*test*.html

  coverage:
    needs: test-matrix
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install cargo-tarpaulin
      run: cargo install cargo-tarpaulin

    - name: Run coverage analysis
      run: cargo tarpaulin --out Xml --output-dir coverage

    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./coverage/cobertura.xml

  quality-gates:
    needs: [test-matrix, coverage]
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Check test coverage threshold
      run: |
        # Extract coverage percentage from coverage report
        COVERAGE=$(grep -oP '(?<=<coverage line-rate=")[^"]*' coverage/cobertura.xml | head -1)
        COVERAGE_PERCENT=$(echo "scale=2; $COVERAGE * 100" | bc)

        echo "Coverage: $COVERAGE_PERCENT%"
        if (( $(echo "$COVERAGE_PERCENT < 85.0" | bc -l) )); then
          echo "❌ Coverage below threshold: $COVERAGE_PERCENT% < 85.0%"
          exit 1
        else
          echo "✅ Coverage meets threshold: $COVERAGE_PERCENT% >= 85.0%"
        fi

    - name: Check test execution time
      run: |
        # Ensure tests complete within reasonable time
        if [ "$SECONDS" -gt 600 ]; then  # 10 minutes
          echo "❌ Tests took too long: $SECONDS seconds"
          exit 1
        else
          echo "✅ Tests completed in reasonable time: $SECONDS seconds"
        fi
```

### Quality Gates

```yaml
# Quality gate definitions
- name: Code Quality Gates
  run: |
    # Check for critical issues
    cargo clippy -- -D warnings || exit 1

    # Security audit
    cargo audit --deny warnings || exit 1

    # Format check
    cargo fmt --all -- --check || exit 1

- name: Performance Gates
  run: |
    # Check binary size
    BINARY_SIZE=$(stat -f%z target/release/iora)
    if [ "$BINARY_SIZE" -gt 50000000 ]; then  # 50MB limit
      echo "❌ Binary too large: $BINARY_SIZE bytes"
      exit 1
    fi

    # Check compile time
    COMPILE_TIME=$SECONDS
    if [ "$COMPILE_TIME" -gt 300 ]; then  # 5 minutes
      echo "❌ Compilation too slow: $COMPILE_TIME seconds"
      exit 1
    fi

- name: Test Quality Gates
  run: |
    # Check test count hasn't decreased
    TEST_COUNT=$(cargo test --lib -- --list | grep -c "test ")
    if [ "$TEST_COUNT" -lt 80 ]; then  # Minimum test count
      echo "❌ Insufficient tests: $TEST_COUNT < 80"
      exit 1
    fi

    # Check for test failures
    if [ $? -ne 0 ]; then
      echo "❌ Tests failed"
      exit 1
    fi
```

## Reporting Framework

### Test Result Aggregation

```rust
// tests/common/reporting.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSuiteResult {
    pub suite_name: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub skipped_tests: usize,
    pub execution_time_seconds: f64,
    pub coverage_percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestRunReport {
    pub timestamp: DateTime<Utc>,
    pub git_commit: String,
    pub branch: String,
    pub results: HashMap<String, TestSuiteResult>,
    pub overall_status: TestStatus,
    pub summary: TestSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSummary {
    pub total_suites: usize,
    pub total_tests: usize,
    pub total_passed: usize,
    pub total_failed: usize,
    pub total_skipped: usize,
    pub average_execution_time: f64,
    pub overall_coverage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Partial,
}

impl TestRunReport {
    pub fn generate_markdown(&self) -> String {
        let mut md = format!(
            "# Test Execution Report\n\n\
             **Timestamp:** {}\n\
             **Commit:** `{}`\n\
             **Branch:** `{}`\n\
             **Overall Status:** {}\n\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            &self.git_commit[..8],
            self.branch,
            match self.overall_status {
                TestStatus::Passed => "✅ PASSED",
                TestStatus::Failed => "❌ FAILED",
                TestStatus::Partial => "⚠️ PARTIAL",
            }
        );

        md.push_str("## Summary\n\n");
        md.push_str(&format!(
            "| Metric | Value |\n\
             |--------|-------|\n\
             | Total Suites | {} |\n\
             | Total Tests | {} |\n\
             | Passed | {} |\n\
             | Failed | {} |\n\
             | Skipped | {} |\n\
             | Avg Execution Time | {:.2}s |\n",
            self.summary.total_suites,
            self.summary.total_tests,
            self.summary.total_passed,
            self.summary.total_failed,
            self.summary.total_skipped,
            self.summary.average_execution_time
        ));

        if let Some(coverage) = self.summary.overall_coverage {
            md.push_str(&format!("| Coverage | {:.1}% |\n", coverage));
        }

        md.push_str("\n## Suite Results\n\n");
        md.push_str("| Suite | Tests | Passed | Failed | Skipped | Time | Status |\n");
        md.push_str("|-------|-------|--------|--------|---------|------|--------|\n");

        for (suite_name, result) in &self.results {
            let status = if result.failed_tests > 0 {
                "❌"
            } else if result.skipped_tests > 0 {
                "⚠️"
            } else {
                "✅"
            };

            md.push_str(&format!(
                "| {} | {} | {} | {} | {} | {:.2}s | {} |\n",
                suite_name,
                result.total_tests,
                result.passed_tests,
                result.failed_tests,
                result.skipped_tests,
                result.execution_time_seconds,
                status
            ));
        }

        md
    }

    pub fn save_json(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
```

### Coverage Integration

```bash
# Generate coverage reports
cargo tarpaulin --out Html --output-dir coverage/html
cargo tarpaulin --out Json --output-dir coverage/json
cargo tarpaulin --out Xml --output-dir coverage/xml

# Integrate with CI/CD
- name: Upload coverage reports
  uses: actions/upload-artifact@v3
  with:
    name: coverage-reports
    path: |
      coverage/html/
      coverage/json/
      coverage/xml/
```

## Extensibility and Customization

### Custom Test Attributes

```rust
// Custom test attributes for categorization
#[macro_export]
macro_rules! integration_test {
    ($(#[$attr:meta])* $name:ident $body:block) => {
        $(#[$attr])*
        #[tokio::test]
        #[cfg_attr(not(feature = "integration-tests"), ignore)]
        async fn $name() $body
    };
}

#[macro_export]
macro_rules! performance_test {
    ($(#[$attr:meta])* $name:ident $body:block) => {
        $(#[$attr])*
        #[tokio::test]
        #[cfg_attr(not(feature = "performance-tests"), ignore)]
        async fn $name() $body
    };
}

// Usage
integration_test! {
    async fn test_api_integration() {
        // Integration test implementation
    }
}

performance_test! {
    async fn test_high_load_performance() {
        // Performance test implementation
    }
}
```

### Plugin Architecture

```rust
// Plugin interface for custom test extensions
pub trait TestPlugin {
    fn name(&self) -> &str;
    fn setup(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn teardown(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn before_test(&self, test_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn after_test(&self, test_name: &str, success: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

// Example plugin: Metrics collection
pub struct MetricsPlugin {
    metrics: Arc<Mutex<HashMap<String, TestMetric>>>,
}

impl TestPlugin for MetricsPlugin {
    fn name(&self) -> &str { "metrics" }

    fn before_test(&self, test_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start timing
        Ok(())
    }

    fn after_test(&self, test_name: &str, success: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Record metrics
        Ok(())
    }
}
```

## Best Practices

### Test Organization

1. **Single Responsibility**: Each test validates one specific behavior
2. **Descriptive Naming**: `test_[behavior]_[condition]_[expected_result]`
3. **Independent Tests**: No test depends on the execution order of others
4. **Fast Execution**: Tests complete quickly to enable frequent execution
5. **Realistic Data**: Use realistic test data that reflects production scenarios

### Test Maintenance

1. **Regular Refactoring**: Keep test code clean and maintainable
2. **DRY Principle**: Extract common test logic into shared helpers
3. **Version Control**: Keep test data and fixtures under version control
4. **Documentation**: Document complex test scenarios and edge cases
5. **Performance Monitoring**: Track and optimize test execution times

### CI/CD Optimization

1. **Parallel Execution**: Run independent test suites in parallel
2. **Caching**: Cache dependencies and build artifacts
3. **Selective Testing**: Run only affected tests when possible
4. **Early Feedback**: Fail fast on critical issues
5. **Resource Optimization**: Balance test coverage with execution time

This comprehensive test automation framework provides the foundation for reliable, maintainable, and scalable automated testing across the entire I.O.R.A. system.
