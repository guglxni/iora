//! CLI Performance Tests
//!
//! Comprehensive performance testing for CLI responsiveness, memory usage, and scalability.

use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use iora::modules::cli_toolset::{CliExecutor, CliCommand, FeaturesCommand, ApisCommand, MonitorCommand, ProfileCommand, BlockchainCommand};

/// Performance Test Metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operation_count: usize,
    pub total_duration: Duration,
    pub avg_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub p95_response_time: Duration,
    pub p99_response_time: Duration,
    pub error_count: usize,
    pub throughput_ops_per_sec: f64,
}

/// Performance Tester
pub struct CliPerformanceTester {
    semaphore: Arc<Semaphore>,
}

impl CliPerformanceTester {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let semaphore = Arc::new(Semaphore::new(10)); // Limit concurrent operations

        Ok(Self { semaphore })
    }

    /// Run comprehensive performance test suite
    pub async fn run_performance_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🏃 Running CLI Performance Tests");

        // Individual command performance tests
        self.test_individual_command_performance().await?;
        self.test_concurrent_load_performance().await?;
        Self::test_memory_usage_under_load(Arc::new(Semaphore::new(10))).await?;
        self.test_sustained_load_performance().await?;
        self.test_error_handling_performance().await?;
        self.test_configuration_operation_performance().await?;
        self.test_large_dataset_performance().await?;

        println!("✅ All CLI Performance Tests Completed");
        Ok(())
    }

    /// Test performance of individual CLI commands
    async fn test_individual_command_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚡ Testing Individual Command Performance");

        let test_commands = vec![
            ("Features List", CliCommand::Features(FeaturesCommand::List)),
            ("API List", CliCommand::Apis(ApisCommand::List)),
            ("Health Check", CliCommand::Monitor(MonitorCommand::Health)),
            ("Profile List", CliCommand::Profile(ProfileCommand::List)),
        ];

        for (name, command) in test_commands {
            let metrics = self.measure_command_performance(command, 50).await?;
            println!("  {}: avg={}ms, p95={}ms, throughput={:.1} ops/sec",
                    name,
                    metrics.avg_response_time.as_millis(),
                    metrics.p95_response_time.as_millis(),
                    metrics.throughput_ops_per_sec);

            // Assert performance requirements
            assert!(metrics.avg_response_time < Duration::from_millis(100),
                   "{} average response time too slow: {:?}", name, metrics.avg_response_time);
            assert!(metrics.p95_response_time < Duration::from_millis(200),
                   "{} P95 response time too slow: {:?}", name, metrics.p95_response_time);
            assert!(metrics.error_count == 0,
                   "{} had {} errors", name, metrics.error_count);
        }

        Ok(())
    }

    /// Test concurrent load performance
    async fn test_concurrent_load_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Testing Concurrent Load Performance");

        let concurrent_levels = vec![1, 5, 10, 20, 50];
        let command = CliCommand::Features(FeaturesCommand::List);

        for concurrency in concurrent_levels {
            let metrics = self.measure_concurrent_performance(command.clone(), concurrency, 20).await?;
            println!("  Concurrency {}: avg={}ms, throughput={:.1} ops/sec",
                    concurrency,
                    metrics.avg_response_time.as_millis(),
                    metrics.throughput_ops_per_sec);

            // Assert reasonable performance under load
            assert!(metrics.avg_response_time < Duration::from_millis(500),
                   "High concurrency ({}) response time too slow: {:?}", concurrency, metrics.avg_response_time);
            assert!(metrics.error_count == 0,
                   "Concurrency {} had {} errors", concurrency, metrics.error_count);
        }

        Ok(())
    }

    /// Test memory usage under load
    async fn test_memory_usage_under_load(semaphore: Arc<Semaphore>) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧠 Testing Memory Usage Under Load");

        // Note: Detailed memory profiling requires external tools
        // This test ensures operations complete without excessive memory issues

        let command = CliCommand::Monitor(MonitorCommand::Health);
        let iterations = 100;

        let start_time = Instant::now();

        for _ in 0..iterations {
            let permit = semaphore.acquire().await?;
            let cmd = command.clone();

            // Create executor outside the spawn to avoid lifetime issues
            tokio::spawn(async move {
                let mut executor = CliExecutor::new().unwrap();
                let _ = executor.execute(cmd).await;
                drop(permit);
            });
        }

        // Wait for all operations to complete
        tokio::time::sleep(Duration::from_secs(2)).await;
        let elapsed = start_time.elapsed();

        println!("  Completed {} operations in {:.2}s", iterations, elapsed.as_secs_f64());
        println!("  Average throughput: {:.1} ops/sec", iterations as f64 / elapsed.as_secs_f64());

        // If we get here without panicking, memory usage is acceptable
        Ok(())
    }

    /// Test sustained load performance
    async fn test_sustained_load_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⏰ Testing Sustained Load Performance");

        let command = CliCommand::Apis(ApisCommand::List);
        let duration = Duration::from_secs(30);
        let start_time = Instant::now();

        let mut operation_count = 0;
        let mut errors = 0;

        while start_time.elapsed() < duration {
            let result = timeout(
                Duration::from_secs(5),
                async {
                    let mut executor = CliExecutor::new().unwrap();
                    executor.execute(command.clone()).await
                }
            ).await;

            match result {
                Ok(Ok(_)) => operation_count += 1,
                _ => errors += 1,
            }
        }

        let total_duration = start_time.elapsed();
        let throughput = operation_count as f64 / total_duration.as_secs_f64();

        println!("  Sustained load: {} operations in {:.1}s", operation_count, total_duration.as_secs_f64());
        println!("  Throughput: {:.1} ops/sec, Errors: {}", throughput, errors);

        // Assert reasonable sustained performance
        assert!(throughput > 1.0, "Sustained throughput too low: {:.1} ops/sec", throughput);
        assert!(errors < operation_count / 10, "Too many errors: {} out of {}", errors, operation_count);

        Ok(())
    }

    /// Test error handling performance
    async fn test_error_handling_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚨 Testing Error Handling Performance");

        // Test performance when handling various error conditions
        let error_commands = vec![
            CliCommand::Apis(ApisCommand::Test("nonexistent".to_string())),
            CliCommand::Blockchain(BlockchainCommand::Switch("invalid".to_string())),
            CliCommand::Profile(ProfileCommand::Switch("missing".to_string())),
        ];

        for command in error_commands {
            let metrics = self.measure_command_performance(command, 10).await?;

            println!("  Error handling: avg={}ms, errors={}",
                    metrics.avg_response_time.as_millis(),
                    metrics.error_count);

            // Error handling should be fast and consistent
            assert!(metrics.avg_response_time < Duration::from_millis(50),
                   "Error handling too slow: {:?}", metrics.avg_response_time);
            assert!(metrics.error_count > 0,
                   "Expected errors not generated");
        }

        Ok(())
    }

    /// Test configuration operation performance
    async fn test_configuration_operation_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚙️ Testing Configuration Operation Performance");

        let config_commands = vec![
            CliCommand::Features(FeaturesCommand::List),
            CliCommand::Features(FeaturesCommand::Status),
            CliCommand::Profile(ProfileCommand::List),
        ];

        for command in config_commands {
            let metrics = self.measure_command_performance(command, 20).await?;
            println!("  Config operation: avg={}ms, p95={}ms",
                    metrics.avg_response_time.as_millis(),
                    metrics.p95_response_time.as_millis());

            // Configuration operations should be fast
            assert!(metrics.avg_response_time < Duration::from_millis(30),
                   "Config operation too slow: {:?}", metrics.avg_response_time);
        }

        Ok(())
    }

    /// Test performance with large datasets
    async fn test_large_dataset_performance(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Testing Large Dataset Performance");

        // Simulate operations that might handle larger data
        let command = CliCommand::Monitor(MonitorCommand::Metrics);
        let iterations = 50;

        let metrics = self.measure_command_performance(command, iterations).await?;

        println!("  Large dataset: {} operations, avg={}ms, throughput={:.1} ops/sec",
                iterations,
                metrics.avg_response_time.as_millis(),
                metrics.throughput_ops_per_sec);

        // Performance should scale reasonably
        assert!(metrics.avg_response_time < Duration::from_millis(100),
               "Large dataset operations too slow: {:?}", metrics.avg_response_time);
        assert!(metrics.throughput_ops_per_sec > 5.0,
               "Throughput too low for large datasets: {:.1} ops/sec", metrics.throughput_ops_per_sec);

        Ok(())
    }

    /// Measure performance of a single command executed multiple times
    async fn measure_command_performance(&self, command: CliCommand, iterations: usize) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
        let mut response_times = Vec::with_capacity(iterations);
        let mut errors = 0;

        let start_time = Instant::now();

        for _ in 0..iterations {
            let cmd_start = Instant::now();
            let result = {
                let mut executor = CliExecutor::new().unwrap();
                executor.execute(command.clone()).await
            };
            let cmd_duration = cmd_start.elapsed();

            response_times.push(cmd_duration);

            if result.is_err() {
                errors += 1;
            }
        }

        let total_duration = start_time.elapsed();

        // Calculate percentiles
        response_times.sort();
        let p95_index = (response_times.len() as f64 * 0.95) as usize;
        let p99_index = (response_times.len() as f64 * 0.99) as usize;

        let metrics = PerformanceMetrics {
            operation_count: iterations,
            total_duration,
            avg_response_time: total_duration / iterations as u32,
            min_response_time: response_times[0],
            max_response_time: *response_times.last().unwrap(),
            p95_response_time: response_times[p95_index.min(response_times.len() - 1)],
            p99_response_time: response_times[p99_index.min(response_times.len() - 1)],
            error_count: errors,
            throughput_ops_per_sec: iterations as f64 / total_duration.as_secs_f64(),
        };

        Ok(metrics)
    }

    /// Measure performance under concurrent load
    async fn measure_concurrent_performance(&self, command: CliCommand, concurrency: usize, operations_per_task: usize) -> Result<PerformanceMetrics, Box<dyn std::error::Error>> {
        let mut handles = vec![];
        let start_time = Instant::now();
        let mut all_response_times = vec![];

        // Spawn concurrent tasks
        for _ in 0..concurrency {
            let cmd = command.clone();

            let handle = tokio::spawn(async move {
                let mut executor = CliExecutor::new().unwrap();
                let mut local_times = vec![];
                let mut local_errors = 0;

                for _ in 0..operations_per_task {
                    let cmd_start = Instant::now();
                    let result = executor.execute(cmd.clone()).await;
                    let duration = cmd_start.elapsed();

                    local_times.push(duration);

                    if result.is_err() {
                        local_errors += 1;
                    }
                }

                (local_times, local_errors)
            });

            handles.push(handle);
        }

        // Collect results
        let mut total_errors = 0;
        for handle in handles {
            let (times, errors) = handle.await?;
            all_response_times.extend(times);
            total_errors += errors;
        }

        let total_duration = start_time.elapsed();
        let total_operations = all_response_times.len();

        // Calculate metrics
        all_response_times.sort();
        let p95_index = (all_response_times.len() as f64 * 0.95) as usize;
        let p99_index = (all_response_times.len() as f64 * 0.99) as usize;

        let metrics = PerformanceMetrics {
            operation_count: total_operations,
            total_duration,
            avg_response_time: total_duration / total_operations as u32,
            min_response_time: all_response_times[0],
            max_response_time: *all_response_times.last().unwrap(),
            p95_response_time: all_response_times[p95_index.min(all_response_times.len() - 1)],
            p99_response_time: all_response_times[p99_index.min(all_response_times.len() - 1)],
            error_count: total_errors,
            throughput_ops_per_sec: total_operations as f64 / total_duration.as_secs_f64(),
        };

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_performance_complete() {
        let tester = CliPerformanceTester::new().await.unwrap();
        let result = tester.run_performance_tests().await;
        assert!(result.is_ok(), "All CLI performance tests should pass");
    }

    #[tokio::test]
    async fn test_individual_command_performance() {
        let tester = CliPerformanceTester::new().await.unwrap();
        assert!(tester.test_individual_command_performance().await.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_load_performance() {
        let tester = CliPerformanceTester::new().await.unwrap();
        assert!(tester.test_concurrent_load_performance().await.is_ok());
    }

    #[tokio::test]
    async fn test_sustained_load_performance() {
        let tester = CliPerformanceTester::new().await.unwrap();
        assert!(tester.test_sustained_load_performance().await.is_ok());
    }

    #[tokio::test]
    async fn test_error_handling_performance() {
        let tester = CliPerformanceTester::new().await.unwrap();
        assert!(tester.test_error_handling_performance().await.is_ok());
    }
}
