# I.O.R.A. Best Practices Guide

## Overview

This guide outlines the best practices for developing, deploying, and maintaining the I.O.R.A. (Intelligent Oracle Rust Assistant) system. Following these practices ensures high-quality, maintainable, and performant code.

## Development Best Practices

### 1. Code Organization

#### Module Structure
```rust
// src/
// ├── lib.rs                    // Main library exports
// ├── main.rs                   // Application entry point
// ├── modules/
// │   ├── api/                  // API-related modules
// │   │   ├── client.rs         // API client implementations
// │   │   ├── mod.rs            // Module declarations
// │   │   └── types.rs          // API types and DTOs
// │   ├── cache/                // Caching layer
// │   ├── config/               // Configuration management
// │   ├── data/                 // Data processing and models
// │   └── health/               // Health monitoring
// ├── tests/                    // Integration tests
// └── benches/                  // Performance benchmarks
```

#### File Organization Principles
- **Single Responsibility**: Each module/file should have one clear purpose
- **Logical Grouping**: Related functionality should be grouped together
- **Clear Naming**: Module and file names should clearly indicate their purpose
- **Consistent Structure**: Follow the same patterns across all modules

### 2. Error Handling

#### Custom Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum IoraError {
    #[error("Configuration error: {message}")]
    Config { message: String, source: Option<Box<dyn std::error::Error + Send + Sync>> },

    #[error("Network error: {message}")]
    Network { message: String, status_code: Option<u16> },

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {field} - {reason}")]
    Validation { field: String, reason: String },

    #[error("Timeout error after {duration:?}")]
    Timeout { duration: std::time::Duration },
}

// Result type alias for convenience
pub type Result<T> = std::result::Result<T, IoraError>;
```

#### Error Handling Patterns
```rust
// ✅ Good: Early return with context
fn validate_config(config: &Config) -> Result<()> {
    if config.api_key.is_empty() {
        return Err(IoraError::Validation {
            field: "api_key".to_string(),
            reason: "API key cannot be empty".to_string(),
        });
    }

    if !is_valid_api_key(&config.api_key) {
        return Err(IoraError::Validation {
            field: "api_key".to_string(),
            reason: "Invalid API key format".to_string(),
        });
    }

    Ok(())
}

// ✅ Good: Error propagation with context
async fn fetch_data(client: &ApiClient, symbol: &str) -> Result<Data> {
    client.get_price(symbol)
        .await
        .map_err(|e| IoraError::Network {
            message: format!("Failed to fetch price for {}: {}", symbol, e),
            status_code: None,
        })
}

// ❌ Avoid: Generic error handling
fn bad_error_handling() -> Result<()> {
    some_operation().map_err(|_| IoraError::Config {
        message: "Something went wrong".to_string(),
        source: None,
    })
}
```

### 3. Async Programming

#### Async Function Guidelines
```rust
// ✅ Good: Clear async function names
async fn fetch_market_data(symbol: &str) -> Result<MarketData> {
    // Implementation
}

// ✅ Good: Async trait implementations
#[async_trait]
pub trait DataProvider: Send + Sync {
    async fn get_price(&self, symbol: &str) -> Result<Decimal>;
    async fn get_historical_data(&self, symbol: &str, period: Period) -> Result<Vec<HistoricalData>>;
}

// ✅ Good: Proper error handling in async functions
async fn process_with_timeout<T, F, Fut>(
    operation: F,
    timeout: Duration
) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    match tokio::time::timeout(timeout, operation()).await {
        Ok(result) => result,
        Err(_) => Err(IoraError::Timeout { duration: timeout }),
    }
}
```

#### Avoid Common Async Pitfalls
```rust
// ❌ Avoid: Blocking operations in async contexts
async fn bad_async_function() {
    std::thread::sleep(Duration::from_secs(1)); // Blocks the async runtime!
}

// ✅ Good: Use async sleep
async fn good_async_function() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// ❌ Avoid: Holding locks across await points
async fn bad_lock_usage(shared_data: Arc<Mutex<Data>>) {
    let data = shared_data.lock().await; // Lock held across await
    some_async_operation().await;        // DEADLOCK RISK!
}

// ✅ Good: Minimize lock scope
async fn good_lock_usage(shared_data: Arc<Mutex<Data>>) {
    {
        let data = shared_data.lock().await;
        // Use data briefly
    } // Lock released here

    some_async_operation().await; // Safe to await now
}
```

### 4. Memory Management

#### Smart Pointer Usage
```rust
// ✅ Good: Use Arc for shared ownership
#[derive(Clone)]
pub struct ApiClient {
    inner: Arc<ApiClientInner>,
}

// ✅ Good: Use Weak references to prevent cycles
pub struct Cache {
    entries: Arc<Mutex<HashMap<String, CacheEntry>>>,
    cleanup_handle: Option<Weak<Self>>, // Prevent strong reference cycles
}

// ✅ Good: Explicit cleanup when needed
impl Drop for ResourceHandle {
    fn drop(&mut self) {
        // Cleanup logic here
        if let Some(cleanup) = &self.cleanup_fn {
            cleanup();
        }
    }
}
```

#### Memory Leak Prevention
```rust
// ✅ Good: RAII pattern for resource management
struct DatabaseConnection {
    connection: sqlx::pool::PoolConnection<sqlx::Postgres>,
}

impl DatabaseConnection {
    async fn new(pool: &sqlx::PgPool) -> Result<Self> {
        Ok(Self {
            connection: pool.acquire().await?,
        })
    }

    fn as_ref(&self) -> &sqlx::pool::PoolConnection<sqlx::Postgres> {
        &self.connection
    }
}

// ✅ Good: Bounded data structures
struct BoundedCache<T> {
    entries: HashMap<String, T>,
    max_size: usize,
    access_order: VecDeque<String>, // For LRU eviction
}

impl<T> BoundedCache<T> {
    fn insert(&mut self, key: String, value: T) {
        if self.entries.len() >= self.max_size {
            // Evict oldest entry
            if let Some(oldest_key) = self.access_order.pop_front() {
                self.entries.remove(&oldest_key);
            }
        }

        self.entries.insert(key.clone(), value);
        self.access_order.push_back(key);
    }
}
```

### 5. Testing

#### Unit Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    struct TestFixture {
        client: ApiClient,
        // Other test dependencies
    }

    impl TestFixture {
        async fn new() -> Self {
            Self {
                client: ApiClient::new_test_instance().await,
            }
        }
    }

    #[test]
    async fn test_successful_price_fetch() {
        let fixture = TestFixture::new().await;

        let result = fixture.client.get_price("BTC").await;

        assert!(result.is_ok());
        let price = result.unwrap();
        assert!(price > Decimal::ZERO);
    }

    #[test]
    async fn test_invalid_symbol_handling() {
        let fixture = TestFixture::new().await;

        let result = fixture.client.get_price("INVALID").await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IoraError::Network { .. }));
    }

    // Property-based testing
    #[test]
    async fn test_price_consistency() {
        // Test that multiple calls return consistent results
        // within a reasonable time window
    }
}
```

#### Integration Test Guidelines
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::clients::Cli;

    struct TestEnvironment {
        docker_client: Cli,
        database_url: String,
        redis_url: String,
    }

    impl TestEnvironment {
        async fn setup() -> Self {
            // Start test containers
            // Initialize test database
            // Configure test services
        }

        async fn teardown(&self) {
            // Clean up test resources
        }
    }

    #[tokio::test]
    async fn test_full_data_pipeline() {
        let env = TestEnvironment::setup().await;

        // Test complete data flow from API to storage
        let api_client = ApiClient::new(&env.database_url, &env.redis_url).await;
        let processor = DataProcessor::new(&api_client).await;

        // Insert test data
        // Process data
        // Verify results in database

        env.teardown().await;
    }
}
```

### 6. Configuration Management

#### Configuration Patterns
```rust
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub apis: ApiConfigs,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub timeout_seconds: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: num_cpus::get(),
            timeout_seconds: 30,
        }
    }
}

// Environment variable loading with validation
impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let config = envy::from_env::<AppConfig>()?;

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.server.port == 0 {
            return Err(IoraError::Config {
                message: "Server port cannot be 0".to_string(),
                source: None,
            });
        }

        if self.database.url.is_empty() {
            return Err(IoraError::Config {
                message: "Database URL cannot be empty".to_string(),
                source: None,
            });
        }

        Ok(())
    }
}
```

### 7. Logging and Monitoring

#### Structured Logging
```rust
use tracing::{info, warn, error, instrument, field};

#[instrument(skip(config), fields(service = %config.service_name))]
pub async fn initialize_service(config: &AppConfig) -> Result<()> {
    info!("Starting service initialization");

    // Log configuration (without sensitive data)
    info!(
        host = %config.server.host,
        port = %config.server.port,
        workers = %config.server.workers,
        "Server configuration"
    );

    match setup_database(&config.database).await {
        Ok(_) => info!("Database connection established"),
        Err(e) => {
            error!(
                error = %e,
                db_url = %config.database.url, // Be careful with sensitive data
                "Database connection failed"
            );
            return Err(e);
        }
    }

    info!("Service initialization completed");
    Ok(())
}

// Custom metrics
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    request_counter: Arc<AtomicU64>,
    error_counter: Arc<AtomicU64>,
    response_time_histogram: Arc<Histogram>,
}

impl MetricsCollector {
    pub fn record_request(&self, method: &str, endpoint: &str, duration: Duration, status: u16) {
        self.request_counter.fetch_add(1, Ordering::Relaxed);

        if status >= 400 {
            self.error_counter.fetch_add(1, Ordering::Relaxed);
        }

        self.response_time_histogram.record(duration.as_secs_f64());
    }
}
```

## Deployment Best Practices

### 1. Containerization

#### Dockerfile Best Practices
```dockerfile
# Use multi-stage build
FROM rust:1.70-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build with release optimizations
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd --create-home --shell /bin/bash app

# Copy binary from builder
COPY --from=builder /app/target/release/iora /usr/local/bin/

# Use non-root user
USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["iora"]
```

#### Docker Compose for Development
```yaml
version: '3.8'
services:
  iora:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://iora:password@db:5432/iora
      - REDIS_URL=redis://cache:6379
    depends_on:
      - db
      - cache
      - typesense
    volumes:
      - .:/app
      - /app/target
    profiles:
      - dev

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: iora
      POSTGRES_USER: iora
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    profiles:
      - dev

  cache:
    image: redis:7-alpine
    profiles:
      - dev

  typesense:
    image: typesense/typesense:0.24.1
    environment:
      TYPESENSE_DATA_DIR: /data
      TYPESENSE_API_KEY: test_key
    volumes:
      - typesense_data:/data
    profiles:
      - dev

volumes:
  postgres_data:
  typesense_data:
```

### 2. Environment Management

#### Environment Variable Security
```bash
# Use .env files for development only
echo "DATABASE_URL=postgres://user:password@localhost/db" > .env
echo "API_KEY=your_secret_key" >> .env

# Add .env to .gitignore
echo ".env" >> .gitignore

# Use environment-specific configs
cp config/production.env .env.production
cp config/staging.env .env.staging

# Load environment with validation
#!/bin/bash
set -a
source .env
set +a

# Validate required variables
required_vars=("DATABASE_URL" "API_KEY" "REDIS_URL")
for var in "${required_vars[@]}"; do
    if [[ -z "${!var}" ]]; then
        echo "Error: $var is not set"
        exit 1
    fi
done
```

### 3. Health Checks and Monitoring

#### Comprehensive Health Checks
```rust
#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: HealthState,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub checks: HashMap<String, CheckResult>,
}

#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub status: HealthState,
    pub message: Option<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthState {
    Up,
    Down,
    Degraded,
}

pub struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck>>,
}

#[async_trait]
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    async fn check(&self) -> CheckResult;
}

pub struct DatabaseHealthCheck {
    pool: sqlx::PgPool,
}

#[async_trait]
impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        "database"
    }

    async fn check(&self) -> CheckResult {
        let start = Instant::now();

        match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => CheckResult {
                status: HealthState::Up,
                message: None,
                duration_ms: start.elapsed().as_millis() as u64,
            },
            Err(e) => CheckResult {
                status: HealthState::Down,
                message: Some(format!("Database health check failed: {}", e)),
                duration_ms: start.elapsed().as_millis() as u64,
            },
        }
    }
}
```

### 4. Security Best Practices

#### Input Validation
```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct PriceRequest {
    #[validate(length(min = 1, max = 10))]
    pub symbol: String,

    #[validate(range(min = 1, max = 365))]
    pub days: Option<u32>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ApiKey {
    #[validate(length(equal = 32))]
    #[validate(custom = "validate_api_key_format")]
    pub key: String,
}

fn validate_api_key_format(key: &str) -> Result<(), ValidationError> {
    // Custom validation logic
    if !key.chars().all(|c| c.is_alphanumeric()) {
        return Err(ValidationError::new("API key must be alphanumeric"));
    }

    Ok(())
}

// Sanitize inputs
fn sanitize_input(input: &str) -> String {
    // Remove potentially harmful characters
    input.chars()
        .filter(|c| c.is_alphanumeric() || ".-_".contains(*c))
        .collect()
}
```

#### Secure Headers and CORS
```rust
use warp::Filter;

fn create_api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_origins(vec!["https://yourdomain.com", "https://app.yourdomain.com"])
        .allow_headers(vec!["authorization", "content-type"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .max_age(3600);

    let security_headers = warp::reply::with::headers::function(|reply| {
        warp::reply::with::header::header("X-Content-Type-Options", "nosniff")(reply)
    })
    .and(warp::reply::with::header("X-Frame-Options", "DENY"))
    .and(warp::reply::with::header("X-XSS-Protection", "1; mode=block"))
    .and(warp::reply::with::header("Strict-Transport-Security", "max-age=31536000; includeSubDomains"));

    // Combine filters
    api_routes
        .with(cors)
        .with(security_headers)
}
```

## Performance Optimization

### 1. Profiling and Benchmarking

#### Benchmark Setup
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_price_fetching(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("fetch_btc_price", |b| {
        b.to_async(&runtime).iter(|| async {
            let client = ApiClient::new_test().await.unwrap();
            black_box(client.get_price("BTC").await.unwrap());
        });
    });
}

fn bench_data_processing(c: &mut Criterion) {
    c.bench_function("process_market_data", |b| {
        b.iter(|| {
            let data = generate_test_data();
            black_box(process_data(data));
        });
    });
}

criterion_group!(benches, bench_price_fetching, bench_data_processing);
criterion_main!(benches);
```

#### Performance Profiling
```toml
# Cargo.toml additions for profiling
[profile.release]
debug = true  # Enable debug symbols for profiling
lto = true    # Link-time optimization
codegen-units = 1  # Better optimization

[profile.profiling]
inherits = "release"
debug = true
strip = false
```

### 2. Memory Optimization

#### Memory Pool Usage
```rust
use bytes::{Bytes, BytesMut};

struct BufferPool {
    buffers: Vec<BytesMut>,
    max_size: usize,
}

impl BufferPool {
    fn get_buffer(&mut self, capacity: usize) -> BytesMut {
        // Try to reuse existing buffer
        if let Some(mut buffer) = self.buffers.pop() {
            if buffer.capacity() >= capacity {
                buffer.clear();
                return buffer;
            }
        }

        // Create new buffer if none available or too small
        BytesMut::with_capacity(capacity)
    }

    fn return_buffer(&mut self, mut buffer: BytesMut) {
        if self.buffers.len() < self.max_size {
            buffer.clear();
            self.buffers.push(buffer);
        }
    }
}
```

### 3. Concurrent Processing

#### Worker Pool Pattern
```rust
use tokio::sync::{mpsc, Semaphore};
use std::sync::Arc;

pub struct WorkerPool<T, R> {
    workers: Vec<tokio::task::JoinHandle<()>>,
    sender: mpsc::UnboundedSender<(T, oneshot::Sender<R>)>,
    semaphore: Arc<Semaphore>,
}

impl<T, R> WorkerPool<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    pub fn new<F>(num_workers: usize, processor: F) -> Self
    where
        F: Fn(T) -> R + Send + Clone + 'static,
    {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let semaphore = Arc::new(Semaphore::new(num_workers));

        let mut workers = Vec::new();

        for _ in 0..num_workers {
            let receiver = receiver.clone();
            let processor = processor.clone();
            let semaphore = semaphore.clone();

            let worker = tokio::spawn(async move {
                while let Some((input, reply_to)) = receiver.recv().await {
                    let _permit = semaphore.acquire().await.unwrap();

                    let result = processor(input);
                    let _ = reply_to.send(result);
                }
            });

            workers.push(worker);
        }

        Self {
            workers,
            sender,
            semaphore,
        }
    }

    pub async fn process(&self, input: T) -> Result<R> {
        let (reply_tx, reply_rx) = oneshot::channel();

        self.sender.send((input, reply_tx))
            .map_err(|_| IoraError::Network {
                message: "Worker pool channel closed".to_string(),
                status_code: None,
            })?;

        reply_rx.await.map_err(|_| IoraError::Network {
            message: "Worker task panicked".to_string(),
            status_code: None,
        })
    }
}
```

## Maintenance Best Practices

### 1. Code Reviews

#### Review Checklist
- [ ] **Functionality**: Code works as intended
- [ ] **Error Handling**: Comprehensive error handling
- [ ] **Performance**: No obvious performance issues
- [ ] **Security**: No security vulnerabilities
- [ ] **Testing**: Adequate test coverage
- [ ] **Documentation**: Code is well-documented
- [ ] **Style**: Follows project conventions

### 2. Documentation

#### Code Documentation Standards
```rust
/// Processes market data for a given symbol with intelligent caching and validation.
///
/// This function fetches the latest market data for the specified symbol, applying
/// various quality checks and caching strategies to ensure data freshness and
/// reliability.
///
/// # Arguments
///
/// * `symbol` - The market symbol to process (e.g., "BTC", "ETH")
/// * `options` - Processing options including cache settings and validation rules
///
/// # Returns
///
/// Returns a `Result` containing the processed market data or an error if processing fails.
///
/// # Errors
///
/// This function can return the following errors:
/// * `IoraError::Network` - If API calls fail
/// * `IoraError::Validation` - If data validation fails
/// * `IoraError::Config` - If configuration is invalid
///
/// # Performance
///
/// This function uses intelligent caching to minimize API calls. Expected performance:
/// - Cached data: < 1ms
/// - Fresh data: 100-500ms depending on API response times
///
/// # Examples
///
/// ```rust
/// use iora::modules::processor::DataProcessor;
///
/// let processor = DataProcessor::new().await?;
/// let data = processor.process_symbol("BTC", Default::default()).await?;
/// println!("BTC Price: ${}", data.price);
/// ```
pub async fn process_symbol(
    &self,
    symbol: &str,
    options: ProcessingOptions
) -> Result<ProcessedData> {
    // Implementation
}
```

### 3. Version Management

#### Semantic Versioning
```
MAJOR.MINOR.PATCH

MAJOR: Breaking changes
MINOR: New features (backward compatible)
PATCH: Bug fixes (backward compatible)
```

#### Changelog Format
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- New feature for enhanced data processing

### Changed
- Improved error handling in API client

### Deprecated
- Legacy configuration format (will be removed in v2.0.0)

### Removed
- Unused legacy code

### Fixed
- Memory leak in cache implementation

### Security
- Updated dependencies to address security vulnerabilities

## [1.2.0] - 2024-01-15

### Added
- Support for new cryptocurrency exchanges
- Performance metrics collection

### Fixed
- Race condition in concurrent data fetching
```

---

## Conclusion

Following these best practices ensures that the I.O.R.A. system remains maintainable, performant, and reliable as it grows. Regular review and updates to these practices based on new learnings and technological advancements are essential.

**Key Takeaways:**
- Write clear, well-documented code
- Handle errors comprehensively
- Test thoroughly at all levels
- Monitor and profile performance
- Keep security as a top priority
- Maintain comprehensive documentation
- Follow established patterns and conventions

---

**Last Updated:** December 2024
**Version:** 1.0
**Authors:** I.O.R.A. Development Team
