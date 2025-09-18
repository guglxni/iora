# I.O.R.A. Knowledge Base

## Overview

This knowledge base contains common issues, solutions, best practices, and troubleshooting procedures for the I.O.R.A. (Intelligent Oracle Rust Assistant) system.

## Common Issues and Solutions

### Issue 1: High Memory Usage After Deployment

**Symptoms:**
- Memory usage grows steadily over time
- Application becomes unresponsive after hours of operation
- OutOfMemory errors in logs

**Root Causes:**
1. **Memory leaks in async operations**
2. **Improper resource cleanup in error paths**
3. **Large data structures not being released**
4. **Cache not implementing size limits**

**Solutions:**

**Immediate Mitigation:**
```bash
# Restart application to clear memory
systemctl restart iora

# Monitor memory usage
watch -n 5 'ps aux | grep iora | grep -v grep | awk "{print \$6/1024 \" MB\"}"'
```

**Code Fixes:**
```rust
// Before: Potential memory leak
async fn process_data(data: Vec<u8>) -> Result<(), Error> {
    let processed = process_large_dataset(data).await?;
    // 'processed' goes out of scope but might not be dropped immediately
    Ok(())
}

// After: Explicit cleanup
async fn process_data(data: Vec<u8>) -> Result<(), Error> {
    let processed = process_large_dataset(data).await?;
    drop(processed); // Explicit cleanup
    Ok(())
}

// Implement memory monitoring
struct MemoryWatcher;

impl MemoryWatcher {
    async fn monitor_usage() {
        let mut interval = tokio::time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            let usage = get_memory_usage();

            if usage > 512 * 1024 * 1024 { // 512MB threshold
                println!("⚠️  High memory usage: {}MB", usage / 1024 / 1024);
                trigger_garbage_collection().await;
            }
        }
    }
}
```

**Prevention:**
- Implement memory monitoring in production
- Use memory profiling tools during development
- Set appropriate resource limits in deployment

---

### Issue 2: API Rate Limiting Errors

**Symptoms:**
- `429 Too Many Requests` errors
- Intermittent API failures
- Degraded data freshness

**Root Causes:**
1. **Aggressive polling without backoff**
2. **Shared rate limits across multiple instances**
3. **No coordination between service instances**
4. **External API changes or quota reductions**

**Solutions:**

**Configuration Fix:**
```rust
// Update API client configuration
#[derive(Deserialize)]
struct ApiConfig {
    base_url: String,
    requests_per_minute: u32,
    burst_limit: u32,
    backoff_multiplier: f64,
    max_backoff_seconds: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.coingecko.com".to_string(),
            requests_per_minute: 50,    // Conservative limit
            burst_limit: 10,            // Small burst allowance
            backoff_multiplier: 2.0,    // Exponential backoff
            max_backoff_seconds: 300,   // 5 minute max backoff
        }
    }
}
```

**Implementation:**
```rust
struct RateLimitedApiClient {
    client: reqwest::Client,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    config: ApiConfig,
}

impl RateLimitedApiClient {
    async fn execute_request(&self, request: Request) -> Result<Response, Error> {
        let mut limiter = self.rate_limiter.lock().await;

        loop {
            match limiter.check_n(1) {
                Ok(_) => {
                    // Rate limit allows request
                    break;
                }
                Err(wait_time) => {
                    // Rate limit exceeded, wait
                    let wait_duration = std::cmp::min(
                        wait_time,
                        Duration::from_secs(self.config.max_backoff_seconds)
                    );
                    tokio::time::sleep(wait_duration).await;
                }
            }
        }

        // Execute the request
        self.client.execute(request).await
    }
}
```

**Distributed Coordination:**
```rust
// Use Redis for distributed rate limiting
struct DistributedRateLimiter {
    redis_client: redis::Client,
    key_prefix: String,
    window_seconds: u64,
    max_requests: u64,
}

impl DistributedRateLimiter {
    async fn check_limit(&self, identifier: &str) -> Result<(), RateLimitError> {
        let key = format!("{}:{}", self.key_prefix, identifier);
        let current_time = Utc::now().timestamp() as u64;

        // Use Redis sorted set for sliding window
        let cleanup_before = current_time - self.window_seconds;

        // Remove old requests outside window
        redis::cmd("ZREMRANGEBYSCORE")
            .arg(&key)
            .arg(0)
            .arg(cleanup_before)
            .query_async(&mut self.redis_client.get_async_connection().await?)
            .await?;

        // Count current requests in window
        let request_count: u64 = redis::cmd("ZCARD")
            .arg(&key)
            .query_async(&mut self.redis_client.get_async_connection().await?)
            .await?;

        if request_count >= self.max_requests {
            return Err(RateLimitError::Exceeded);
        }

        // Add current request
        redis::cmd("ZADD")
            .arg(&key)
            .arg(current_time)
            .arg(current_time)
            .query_async(&mut self.redis_client.get_async_connection().await?)
            .await?;

        // Set expiration on the key
        redis::cmd("EXPIRE")
            .arg(&key)
            .arg(self.window_seconds * 2)
            .query_async(&mut self.redis_client.get_async_connection().await?)
            .await?;

        Ok(())
    }
}
```

---

### Issue 3: Database Connection Pool Exhaustion

**Symptoms:**
- `connection pool exhausted` errors
- Slow response times
- Database connection timeouts

**Root Causes:**
1. **Long-running queries blocking connections**
2. **Improper connection cleanup**
3. **Connection pool size too small for load**
4. **Connection leaks in error paths**

**Solutions:**

**Connection Pool Optimization:**
```rust
// Configure connection pool properly
#[derive(Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    min_connections: u32,
    acquire_timeout_seconds: u64,
    idle_timeout_seconds: u64,
    max_lifetime_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://localhost/iora".to_string(),
            max_connections: 20,
            min_connections: 5,
            acquire_timeout_seconds: 30,
            idle_timeout_seconds: 600,    // 10 minutes
            max_lifetime_seconds: 1800,   // 30 minutes
        }
    }
}

// Create optimized pool
async fn create_optimized_pool(config: &DatabaseConfig) -> Result<sqlx::PgPool, Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_seconds))
        .idle_timeout(Duration::from_secs(config.idle_timeout_seconds))
        .max_lifetime(Duration::from_secs(config.max_lifetime_seconds))
        .test_on_check_out(true)  // Test connections before use
        .build(&config.url)
        .await
}
```

**Query Timeout Management:**
```rust
struct QueryExecutor {
    pool: sqlx::PgPool,
    default_timeout: Duration,
}

impl QueryExecutor {
    async fn execute_with_timeout<T, F, Fut>(
        &self,
        query_fn: F,
        timeout: Option<Duration>
    ) -> Result<T, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let timeout_duration = timeout.unwrap_or(self.default_timeout);

        tokio::time::timeout(timeout_duration, query_fn()).await
            .map_err(|_| Error::QueryTimeout)?
    }

    async fn execute_critical_query(&self, query: &str) -> Result<Vec<sqlx::postgres::PgRow>, Error> {
        // Set statement timeout at database level
        let setup_query = "SET statement_timeout = 30000"; // 30 seconds

        self.execute_with_timeout(async {
            // Execute setup query
            sqlx::query(setup_query)
                .execute(&self.pool)
                .await?;

            // Execute main query
            sqlx::query(query)
                .fetch_all(&self.pool)
                .await
        }, Some(Duration::from_secs(35))).await // Slightly longer than statement timeout
    }
}
```

**Connection Health Monitoring:**
```rust
struct ConnectionHealthMonitor {
    pool: Arc<sqlx::PgPool>,
    health_check_interval: Duration,
    unhealthy_threshold: u32,
}

impl ConnectionHealthMonitor {
    async fn start_monitoring(&self) {
        let monitor = Arc::new(self.clone());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(monitor.health_check_interval);
            let mut consecutive_failures = 0;

            loop {
                interval.tick().await;

                if monitor.perform_health_check().await.is_err() {
                    consecutive_failures += 1;

                    if consecutive_failures >= monitor.unhealthy_threshold {
                        monitor.handle_unhealthy_pool().await;
                        consecutive_failures = 0;
                    }
                } else {
                    consecutive_failures = 0;
                }
            }
        });
    }

    async fn perform_health_check(&self) -> Result<(), Error> {
        sqlx::query("SELECT 1")
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn handle_unhealthy_pool(&self) {
        println!("⚠️  Database connection pool unhealthy, attempting recovery");

        // Try to reconnect unhealthy connections
        // Implement exponential backoff for reconnection attempts
        // Alert if recovery fails
    }
}
```

---

### Issue 4: Async Task Deadlocks

**Symptoms:**
- Application becomes unresponsive
- CPU usage drops to near zero
- Memory usage remains stable
- No error logs generated

**Root Causes:**
1. **Incorrect async/await usage**
2. **Blocking operations in async contexts**
3. **Circular await dependencies**
4. **Mutex lock ordering issues**

**Solutions:**

**Async Best Practices:**
```rust
// ❌ Wrong: Blocking operations in async context
async fn bad_async_function() -> Result<(), Error> {
    // This blocks the entire async runtime!
    std::thread::sleep(Duration::from_secs(1));
    Ok(())
}

// ✅ Correct: Proper async operations
async fn good_async_function() -> Result<(), Error> {
    // Non-blocking sleep
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(())
}

// ✅ Use spawn_blocking for CPU-intensive work
async fn cpu_intensive_async_function(data: Vec<u8>) -> Result<ProcessedData, Error> {
    let result = tokio::task::spawn_blocking(move || {
        // CPU-intensive synchronous processing
        process_data_synchronously(data)
    }).await?;

    Ok(result)
}
```

**Mutex Lock Ordering:**
```rust
// ❌ Wrong: Inconsistent lock ordering can cause deadlocks
struct BadResourceManager {
    resource_a: Arc<Mutex<ResourceA>>,
    resource_b: Arc<Mutex<ResourceB>>,
}

impl BadResourceManager {
    async fn operation1(&self) {
        let a = self.resource_a.lock().await;
        let b = self.resource_b.lock().await; // Lock order: A then B
        // ... use resources
    }

    async fn operation2(&self) {
        let b = self.resource_b.lock().await;  // Lock order: B then A
        let a = self.resource_a.lock().await;  // DEADLOCK!
        // ... use resources
    }
}

// ✅ Correct: Consistent lock ordering
struct GoodResourceManager {
    resource_a: Arc<Mutex<ResourceA>>,
    resource_b: Arc<Mutex<ResourceB>>,
}

impl GoodResourceManager {
    async fn operation1(&self) {
        self.with_resources(|_a, _b| async move {
            // Use resources in consistent order
        }).await;
    }

    async fn operation2(&self) {
        self.with_resources(|_a, _b| async move {
            // Same lock order guarantees no deadlock
        }).await;
    }

    async fn with_resources<F, Fut, T>(&self, f: F) -> T
    where
        F: FnOnce(Arc<Mutex<ResourceA>>, Arc<Mutex<ResourceB>>) -> Fut,
        Fut: Future<Output = T>,
    {
        // Always acquire locks in the same order
        let a = Arc::clone(&self.resource_a);
        let b = Arc::clone(&self.resource_b);

        let a_lock = a.lock().await;
        let b_lock = b.lock().await;

        f(a, b).await
    }
}
```

**Timeout Prevention:**
```rust
struct TimeoutProtectedExecutor {
    default_timeout: Duration,
    critical_timeout: Duration,
}

impl TimeoutProtectedExecutor {
    async fn execute_with_timeout<T, F, Fut>(
        &self,
        operation: F,
        timeout: Option<Duration>,
        is_critical: bool
    ) -> Result<T, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let timeout_duration = timeout
            .or_else(|| Some(if is_critical { self.critical_timeout } else { self.default_timeout }))
            .unwrap();

        match tokio::time::timeout(timeout_duration, operation()).await {
            Ok(result) => result,
            Err(_) => {
                // Log timeout and cleanup
                println!("Operation timed out after {:?}", timeout_duration);
                Err(Error::Timeout(timeout_duration))
            }
        }
    }

    async fn execute_critical_operation(&self, operation: impl Future<Output = Result<(), Error>>) -> Result<(), Error> {
        self.execute_with_timeout(
            || operation,
            Some(self.critical_timeout),
            true
        ).await
    }
}
```

---

### Issue 5: Data Consistency Problems

**Symptoms:**
- Inconsistent data between different API responses
- Stale data being returned
- Data corruption in cached responses

**Root Causes:**
1. **Race conditions in concurrent data updates**
2. **Improper cache invalidation**
3. **Inconsistent data synchronization**
4. **Transaction isolation issues**

**Solutions:**

**Optimistic Concurrency Control:**
```rust
#[derive(Clone, Debug)]
struct VersionedData<T> {
    data: T,
    version: u64,
    last_modified: DateTime<Utc>,
}

struct OptimisticDataStore<T> {
    data: Arc<RwLock<HashMap<String, VersionedData<T>>>>,
}

impl<T: Clone> OptimisticDataStore<T> {
    async fn update_with_version_check(
        &self,
        key: &str,
        new_data: T,
        expected_version: u64
    ) -> Result<(), ConcurrencyError> {
        let mut data_store = self.data.write().await;

        if let Some(existing) = data_store.get(key) {
            if existing.version != expected_version {
                return Err(ConcurrencyError::VersionConflict {
                    expected: expected_version,
                    actual: existing.version,
                });
            }
        }

        let versioned_data = VersionedData {
            data: new_data,
            version: expected_version + 1,
            last_modified: Utc::now(),
        };

        data_store.insert(key.to_string(), versioned_data);
        Ok(())
    }

    async fn get_with_version(&self, key: &str) -> Option<VersionedData<T>> {
        let data_store = self.data.read().await;
        data_store.get(key).cloned()
    }
}
```

**Cache Consistency Strategies:**
```rust
struct ConsistentCache<T> {
    data: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    invalidation_strategy: InvalidationStrategy,
}

enum InvalidationStrategy {
    TimeBased(Duration),
    VersionBased,
    Manual,
}

impl<T: Clone> ConsistentCache<T> {
    async fn get_or_compute<F, Fut>(
        &self,
        key: &str,
        computation: F
    ) -> Result<T, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        // Check cache first
        if let Some(entry) = self.get_valid_entry(key).await {
            return Ok(entry.data);
        }

        // Compute new value
        let new_value = computation().await?;

        // Store in cache with consistency checks
        self.store_with_consistency(key, new_value.clone()).await?;

        Ok(new_value)
    }

    async fn invalidate_consistent(&self, key: &str) -> Result<(), Error> {
        let mut data = self.data.write().await;

        // Use atomic invalidation to prevent race conditions
        if let Some(entry) = data.get_mut(key) {
            entry.invalidated = true;
            entry.invalidated_at = Some(Utc::now());
        }

        Ok(())
    }

    async fn get_valid_entry(&self, key: &str) -> Option<CacheEntry<T>> {
        let data = self.data.read().await;

        data.get(key).and_then(|entry| {
            if self.is_entry_valid(entry) {
                Some(entry.clone())
            } else {
                None
            }
        })
    }

    fn is_entry_valid(&self, entry: &CacheEntry<T>) -> bool {
        match self.invalidation_strategy {
            InvalidationStrategy::TimeBased(duration) => {
                entry.created_at + duration > Utc::now() && !entry.invalidated
            }
            InvalidationStrategy::VersionBased => {
                // Version-based validation logic
                !entry.invalidated
            }
            InvalidationStrategy::Manual => {
                !entry.invalidated
            }
        }
    }
}
```

---

## Best Practices

### 1. Error Handling Patterns

**Structured Error Types:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum IoraError {
    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error
    },

    #[error("API error: {status} - {message}")]
    Api {
        status: u16,
        message: String
    },

    #[error("Timeout error: operation took longer than {duration:?}")]
    Timeout {
        duration: Duration
    },

    #[error("Validation error: {field} - {reason}")]
    Validation {
        field: String,
        reason: String
    },
}

// Error handling helper
pub trait ResultExt<T> {
    fn with_context<F>(self, f: F) -> Result<T, IoraError>
    where
        F: FnOnce() -> String;
}

impl<T, E: std::error::Error> ResultExt<T> for Result<T, E> {
    fn with_context<F>(self, f: F) -> Result<T, IoraError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| IoraError::Config {
            message: format!("{}: {}", f(), e),
        })
    }
}
```

### 2. Logging Best Practices

**Structured Logging:**
```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(config))]
async fn initialize_service(config: &AppConfig) -> Result<(), Error> {
    info!("Starting service initialization");

    // Log configuration (be careful with sensitive data)
    info!(
        service_name = %config.service_name,
        port = %config.port,
        "Service configuration loaded"
    );

    match setup_database(&config.database).await {
        Ok(_) => info!("Database connection established"),
        Err(e) => {
            error!(error = %e, "Failed to connect to database");
            return Err(e);
        }
    }

    info!("Service initialization completed");
    Ok(())
}
```

**Log Levels Usage:**
- **ERROR**: System errors that require immediate attention
- **WARN**: Potential issues or unexpected conditions
- **INFO**: Important business logic events
- **DEBUG**: Detailed diagnostic information
- **TRACE**: Very detailed execution flow information

### 3. Resource Management

**RAII Pattern Implementation:**
```rust
struct DatabaseConnectionGuard {
    connection: Option<sqlx::pool::PoolConnection<sqlx::Postgres>>,
    pool: sqlx::PgPool,
}

impl DatabaseConnectionGuard {
    async fn new(pool: &sqlx::PgPool) -> Result<Self, Error> {
        let connection = pool.acquire().await?;

        Ok(Self {
            connection: Some(connection),
            pool: pool.clone(),
        })
    }

    fn connection(&mut self) -> &mut sqlx::pool::PoolConnection<sqlx::Postgres> {
        self.connection.as_mut().unwrap()
    }
}

impl Drop for DatabaseConnectionGuard {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            // Return connection to pool
            let pool = self.pool.clone();
            tokio::spawn(async move {
                // This will run when the guard is dropped
                drop(connection); // Connection returned to pool
            });
        }
    }
}
```

### 4. Testing Patterns

**Table-Driven Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T, U> {
        name: &'static str,
        input: T,
        expected: U,
        should_error: bool,
    }

    #[tokio::test]
    async fn test_data_processing() {
        let test_cases = vec![
            TestCase {
                name: "valid data",
                input: "valid_input".to_string(),
                expected: "expected_output".to_string(),
                should_error: false,
            },
            TestCase {
                name: "empty input",
                input: "".to_string(),
                expected: "".to_string(),
                should_error: true,
            },
            // Add more test cases
        ];

        for case in test_cases {
            let result = process_data(case.input).await;

            if case.should_error {
                assert!(result.is_err(), "Test case '{}' should have failed", case.name);
            } else {
                assert_eq!(result.unwrap(), case.expected,
                    "Test case '{}' failed", case.name);
            }
        }
    }
}
```

## Performance Optimization Checklist

### Code Review Checklist
- [ ] **Memory Management**: No obvious memory leaks or excessive allocations
- [ ] **Async Operations**: Proper use of async/await, no blocking operations
- [ ] **Error Handling**: Comprehensive error handling without performance penalties
- [ ] **Data Structures**: Appropriate data structures for access patterns
- [ ] **Algorithm Complexity**: Algorithms scale appropriately with input size

### Performance Testing Checklist
- [ ] **Load Testing**: System tested under expected load conditions
- [ ] **Stress Testing**: System tested beyond normal operating conditions
- [ ] **Memory Testing**: Memory usage tested under sustained load
- [ ] **Concurrency Testing**: Multiple concurrent operations tested
- [ ] **Resource Limits**: System tested with constrained resources

### Monitoring Checklist
- [ ] **Metrics Collection**: Key performance metrics being collected
- [ ] **Alerting**: Appropriate alerts configured for performance issues
- [ ] **Logging**: Sufficient logging for performance debugging
- [ ] **Dashboards**: Performance dashboards available and up-to-date
- [ ] **Historical Data**: Performance trends tracked over time

## Emergency Procedures

### System Recovery Steps

1. **Assess Situation**
   - Check system health endpoints
   - Review recent error logs
   - Monitor resource usage

2. **Implement Immediate Fixes**
   - Restart problematic services
   - Apply emergency configuration changes
   - Enable circuit breakers if needed

3. **Scale Resources**
   - Increase instance count
   - Add more database connections
   - Scale up infrastructure

4. **Communicate**
   - Notify stakeholders of the issue
   - Provide status updates
   - Set expectations for recovery

5. **Post-Mortem**
   - Document the incident
   - Identify root cause
   - Implement preventive measures

### Contact Information

**Development Team:**
- Email: dev@iora.project
- Slack: #iora-dev
- On-call: +1-555-IORA-911

**Infrastructure Team:**
- Email: infra@iora.project
- PagerDuty: IORA-Infrastructure

**Security Team:**
- Email: security@iora.project
- Emergency: +1-555-IORA-SEC

---

**Last Updated:** December 2024
**Version:** 1.0
**Maintained by:** I.O.R.A. Development Team

This knowledge base is continuously updated based on real-world incidents and lessons learned. Contributions and updates are welcome via pull requests.
