# I.O.R.A. Test Execution Guidelines

## Overview

This document provides detailed guidelines for executing tests in the I.O.R.A. system, ensuring consistent, reliable, and efficient test execution across different environments and scenarios.

## Test Execution Environments

### Local Development Environment

#### Prerequisites
```bash
# Required tools
rustc --version  # 1.70.0+
cargo --version  # 1.70.0+
docker --version # 20.0.0+

# Optional but recommended
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-audit      # Security auditing
```

#### Environment Setup
```bash
# Clone repository
git clone https://github.com/guglxni/iora.git
cd iora

# Copy environment template
cp .env.example .env

# Edit environment variables (optional for basic testing)
nano .env
```

#### Local Test Execution

```bash
# Run all tests
make test

# Run specific test suites
make test-unit          # Unit tests only
make test-integration   # Integration tests only
make test-functional    # Functional tests only

# Run with verbose output
cargo test --verbose

# Run specific test
cargo test test_data_processing

# Run tests in specific file
cargo test --test integration_tests
```

### CI/CD Environment

#### GitHub Actions Execution

Tests run automatically on:
- **Push to main/master/develop**: Full test suite
- **Pull Requests**: Quality gates + full test suite
- **Scheduled**: Daily regression testing

#### CI/CD Test Commands

```yaml
# Unit tests
cargo test --lib --verbose

# Integration tests
cargo test --test integration_tests --verbose

# Functional tests
cargo test --test functional_quality_tests --verbose

# Resilience tests
cargo test --test resilience_tests --verbose

# Performance tests
cargo test --test performance_tests --release -- --nocapture

# Quality metrics tests
cargo test --test quality_metrics_tests --verbose
```

### Docker Environment

#### Test Execution in Containers

```bash
# Build test image
docker build -t iora:test .

# Run all tests in container
docker run --rm iora:test cargo test

# Run specific tests
docker run --rm iora:test cargo test --test integration_tests

# Run with coverage
docker run --rm -v $(pwd)/coverage:/app/target/tarpaulin \
  iora:test cargo tarpaulin --out Html
```

#### Docker Compose Testing

```bash
# Start test services
make docker-compose-up

# Run tests with services
cargo test --test integration_tests

# Stop services
make docker-compose-down
```

## Test Execution Strategies

### Sequential Execution

For debugging and detailed analysis:

```bash
# Run tests sequentially with detailed output
cargo test -- --nocapture --test-threads=1
```

### Parallel Execution

For CI/CD performance:

```bash
# Run tests in parallel (default)
cargo test

# Control parallelism
cargo test --jobs 4  # Use 4 parallel jobs
```

### Filtered Execution

```bash
# Run tests matching pattern
cargo test data_processing

# Run specific test function
cargo test test_data_validation

# Run tests in specific module
cargo test --lib -- module_name

# Run tests with specific attributes
cargo test --test integration_tests -- api
```

## Test Configuration

### Environment Variables

```bash
# Test environment settings
export RUST_TEST_THREADS=4                    # Parallel test threads
export RUST_BACKTRACE=1                       # Enable backtraces
export RUST_LOG=debug                         # Logging level

# IORA specific settings
export TEST_MODE=true                         # Enable test mode
export MOCK_EXTERNAL_APIS=true               # Mock external services
export TEST_DATABASE_URL=postgres://localhost/iora_test
```

### Test Configuration Files

```toml
# tests/config/test_config.toml
[api]
timeout_seconds = 30
retry_attempts = 3

[database]
url = "postgres://localhost/iora_test"
pool_size = 5

[cache]
enabled = true
ttl_seconds = 300

[external_services]
mock_apis = true
mock_blockchain = true
```

## Test Execution Guidelines

### Pre-Execution Checklist

- [ ] Environment variables configured
- [ ] External services available (or mocked)
- [ ] Database/test data prepared
- [ ] Sufficient disk space (>2GB free)
- [ ] Sufficient memory (>4GB RAM)
- [ ] Network connectivity for external APIs

### Execution Best Practices

#### 1. Test Isolation
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_isolated_operation() {
        // Use unique test data
        let test_id = uuid::Uuid::new_v4();
        let test_data = format!("test_data_{}", test_id);

        // Execute test
        let result = operation_with_isolation(test_data).await;

        // Cleanup
        cleanup_test_data(test_data).await;

        assert!(result.is_ok());
    }
}
```

#### 2. Resource Management
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct TestResources {
        database: TestDatabase,
        cache: TestCache,
        api_client: MockApiClient,
    }

    impl TestResources {
        async fn setup() -> Self { /* setup */ }
        async fn teardown(self) { /* cleanup */ }
    }

    #[tokio::test]
    async fn test_with_resources() {
        let resources = TestResources::setup().await;

        // Test execution
        let result = perform_test_operation(&resources).await;
        assert!(result.is_ok());

        // Automatic cleanup
        resources.teardown().await;
    }
}
```

#### 3. Time-based Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_with_timeout() {
        // Test completes within timeout
        let result = timeout(
            Duration::from_secs(30),
            perform_operation()
        ).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_timing_constraints() {
        let start = std::time::Instant::now();

        let result = perform_timed_operation().await;
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        assert!(elapsed < Duration::from_millis(500)); // Performance requirement
    }
}
```

## Test Result Analysis

### Success Criteria

#### Unit Tests
- ✅ All tests pass
- ✅ Code coverage > 85%
- ✅ No panics or crashes
- ✅ Execution time < 30 seconds

#### Integration Tests
- ✅ All component interactions work
- ✅ External API calls succeed (or fail gracefully)
- ✅ Data consistency maintained
- ✅ Execution time < 2 minutes

#### Functional Tests
- ✅ All user journeys complete successfully
- ✅ Business logic produces correct results
- ✅ Error scenarios handled appropriately
- ✅ Execution time < 3 minutes

#### Performance Tests
- ✅ Meet performance baselines
- ✅ No memory leaks
- ✅ CPU usage within limits
- ✅ Execution time < 5 minutes

### Failure Analysis

#### Common Failure Patterns

1. **Flaky Tests**
```rust
// Symptom: Test passes/fails intermittently
// Solution: Add retry logic or stabilize timing
#[tokio::test]
async fn stable_test_with_retry() {
    for attempt in 0..3 {
        let result = flaky_operation().await;
        if result.is_ok() {
            return; // Success
        }
        tokio::time::sleep(Duration::from_millis(100 * attempt)).await;
    }
    panic!("Test failed after 3 attempts");
}
```

2. **Race Conditions**
```rust
// Symptom: Async tests fail due to timing issues
// Solution: Use proper synchronization
#[tokio::test]
async fn test_concurrent_operations() {
    let mutex = Arc::new(Mutex::new(()));
    let barrier = Arc::new(Barrier::new(2));

    let task1 = tokio::spawn({
        let mutex = Arc::clone(&mutex);
        let barrier = Arc::clone(&barrier);
        async move {
            let _lock = mutex.lock().await;
            barrier.wait().await;
            operation1().await
        }
    });

    let task2 = tokio::spawn({
        let mutex = Arc::clone(&mutex);
        let barrier = Arc::clone(&barrier);
        async move {
            let _lock = mutex.lock().await;
            barrier.wait().await;
            operation2().await
        }
    });

    let (result1, result2) = tokio::try_join!(task1, task2).unwrap();
    assert!(result1.is_ok() && result2.is_ok());
}
```

3. **Resource Leaks**
```rust
// Symptom: Tests fail due to resource exhaustion
// Solution: Proper cleanup and resource limits
struct TestGuard {
    resources: Vec<Box<dyn Drop>>,
}

impl TestGuard {
    fn new() -> Self {
        Self { resources: Vec::new() }
    }

    fn add_resource<T: 'static>(&mut self, resource: T) {
        self.resources.push(Box::new(resource));
    }
}

impl Drop for TestGuard {
    fn drop(&mut self) {
        // Cleanup resources
        self.resources.clear();
    }
}
```

### Debugging Failed Tests

#### Enable Debug Logging
```bash
# Run with debug logging
RUST_LOG=debug cargo test test_failing_test -- --nocapture

# Run with backtraces
RUST_BACKTRACE=1 cargo test test_failing_test
```

#### Isolate Test Execution
```rust
#[tokio::test]
async fn debug_failing_test() {
    // Add detailed logging
    println!("Starting test execution...");

    let intermediate_result = setup_phase().await;
    println!("Setup result: {:?}", intermediate_result);
    assert!(intermediate_result.is_ok());

    let operation_result = operation_phase().await;
    println!("Operation result: {:?}", operation_result);
    assert!(operation_result.is_ok());

    let validation_result = validation_phase().await;
    println!("Validation result: {:?}", validation_result);
    assert!(validation_result.is_ok());
}
```

#### Test Data Inspection
```rust
#[tokio::test]
async fn inspect_test_data() {
    let test_data = generate_test_data().await;

    // Inspect data structure
    println!("Test data: {:#?}", test_data);

    // Validate data properties
    assert!(test_data.price > 0.0);
    assert!(!test_data.symbol.is_empty());

    let result = process_test_data(test_data).await;
    println!("Processing result: {:#?}", result);
    assert!(result.is_ok());
}
```

## Performance Testing Guidelines

### Load Testing Execution

```bash
# Run load tests with specific parameters
cargo test --test performance_tests --release -- \
  --nocapture \
  --test-threads=1 \
  load_test_concurrent_requests

# Environment variables for load testing
export LOAD_TEST_USERS=100
export LOAD_TEST_DURATION=300  # 5 minutes
export LOAD_TEST_RAMP_UP=60   # 1 minute ramp up
```

### Performance Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_data_processing(c: &mut Criterion) {
    let test_data = generate_large_test_dataset();

    c.bench_function("process_large_dataset", |b| {
        b.iter(|| {
            black_box(process_data(test_data.clone()));
        })
    });
}

criterion_group!(benches, benchmark_data_processing);
criterion_main!(benches);
```

### Memory Usage Testing

```rust
#[tokio::test]
async fn test_memory_usage_bounds() {
    let initial_memory = get_current_memory_usage();

    // Perform memory-intensive operation
    let result = memory_intensive_operation().await;

    let final_memory = get_current_memory_usage();
    let memory_delta = final_memory - initial_memory;

    assert!(result.is_ok());
    assert!(memory_delta < 50 * 1024 * 1024); // < 50MB increase
}
```

## Continuous Integration

### Test Execution in CI/CD

```yaml
# .github/workflows/ci.yml
jobs:
  test:
    strategy:
      matrix:
        test-type: [unit, integration, functional, resilience, performance]
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache Dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run ${{ matrix.test-type }} Tests
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
          esac

      - name: Upload Test Results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: test-results-${{ matrix.test-type }}
          path: |
            target/debug/deps/*test*.json
            target/debug/deps/*test*.html
```

### Quality Gates

```yaml
# Quality gate checks
- name: Check Test Coverage
  run: |
    cargo tarpaulin --fail-under 85

- name: Check Test Performance
  run: |
    # Ensure tests complete within time limits
    timeout 600 cargo test  # 10 minute timeout

- name: Check for Flaky Tests
  run: |
    # Run tests multiple times to detect flakes
    for i in {1..3}; do
      cargo test --quiet || exit 1
    done
```

## Test Maintenance

### Regular Maintenance Tasks

1. **Weekly**: Review test execution times and optimize slow tests
2. **Monthly**: Audit test coverage and add missing test cases
3. **Quarterly**: Review and update test data and fixtures
4. **Annually**: Major test framework updates and modernization

### Test Refactoring Guidelines

```rust
// Before: Monolithic test
#[tokio::test]
async fn test_complex_operation() {
    // 50+ lines of setup, execution, and assertions
}

// After: Modular tests
#[tokio::test]
async fn test_operation_setup() {
    let setup = create_test_setup().await;
    assert!(setup.is_valid());
}

#[tokio::test]
async fn test_operation_execution() {
    let setup = create_test_setup().await;
    let result = execute_operation(setup).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_operation_validation() {
    let result = execute_operation_with_setup().await;
    validate_operation_result(result).await;
}
```

### Test Data Management

```rust
// Test data factory pattern
struct TestDataFactory {
    base_data: BaseTestData,
    variations: Vec<TestVariation>,
}

impl TestDataFactory {
    fn new() -> Self {
        Self {
            base_data: BaseTestData::default(),
            variations: Vec::new(),
        }
    }

    fn with_variation(mut self, variation: TestVariation) -> Self {
        self.variations.push(variation);
        self
    }

    async fn generate(&self) -> TestData {
        let mut data = self.base_data.clone();
        for variation in &self.variations {
            data.apply_variation(variation).await;
        }
        data
    }
}
```

## Troubleshooting

### Common Issues and Solutions

#### Tests Hanging
```bash
# Symptom: Tests don't complete
# Solution: Add timeouts and check for deadlocks
#[tokio::test]
async fn test_with_timeout() {
    let result = timeout(
        Duration::from_secs(30),
        operation_that_might_hang()
    ).await;

    assert!(result.is_ok(), "Operation timed out");
}
```

#### Resource Exhaustion
```bash
# Symptom: Tests fail due to resource limits
# Solution: Limit resource usage and add cleanup
#[tokio::test]
async fn test_resource_bounds() {
    // Limit concurrent operations
    let semaphore = Arc::new(Semaphore::new(10));

    let tasks: Vec<_> = (0..50).map(|i| {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        tokio::spawn(async move {
            let result = resource_intensive_operation(i).await;
            drop(permit); // Release permit
            result
        })
    }).collect();

    let results = futures::future::join_all(tasks).await;
    assert!(results.iter().all(|r| r.is_ok()));
}
```

#### Network-dependent Test Failures
```bash
# Symptom: Tests fail due to network issues
# Solution: Use mocks and retry logic
#[tokio::test]
async fn test_network_resilient() {
    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        match network_operation().await {
            Ok(result) => {
                assert!(result.is_valid());
                break;
            }
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                tokio::time::sleep(Duration::from_millis(1000 * attempts)).await;
                continue;
            }
            Err(e) => panic!("Network operation failed after {} attempts: {}", max_attempts, e),
        }
    }
}
```

This comprehensive test execution guidelines document ensures consistent, reliable, and efficient test execution across all environments and scenarios for the I.O.R.A. system.
