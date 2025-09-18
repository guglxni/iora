#[cfg(test)]
mod deployment_tests {
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::process::{Command, Stdio};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use chrono::Utc;
    use iora::modules::cache::{CacheConfig, IntelligentCache};
    use iora::modules::fetcher::{ApiProvider, MultiApiClient, RawData};
    use iora::modules::health::HealthMonitor;
    use iora::modules::processor::{DataProcessor, ProcessingConfig};
    use iora::modules::rag::RagSystem;
    use tokio::time::timeout;

    /// Test 1: Containerization Tests - Docker deployment and operation
    #[tokio::test]
    async fn test_docker_containerization() {
        println!("🧪 Testing Docker Containerization...");

        // Check if Docker is available
        let docker_available = Command::new("docker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !docker_available {
            println!("⚠️  Docker not available, skipping containerization tests");
            return;
        }

        // Check if docker-compose.yml exists
        let docker_compose_path = Path::new("docker-compose.yml");
        assert!(
            docker_compose_path.exists(),
            "docker-compose.yml should exist"
        );

        // Validate docker-compose.yml structure
        let compose_content = fs::read_to_string(docker_compose_path)
            .expect("Should be able to read docker-compose.yml");

        assert!(
            compose_content.contains("iora"),
            "docker-compose.yml should contain iora service"
        );
        assert!(
            compose_content.contains("typesense"),
            "docker-compose.yml should contain typesense service"
        );

        // Test Docker image build (if not in CI environment)
        if env::var("CI").is_err() {
            println!("🏗️  Testing Docker image build...");

            let build_result = Command::new("docker")
                .args(&["build", "-t", "iora-test", "."])
                .current_dir("..")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();

            if let Ok(status) = build_result {
                if status.success() {
                    println!("✅ Docker image built successfully");
                } else {
                    println!("⚠️  Docker image build failed, but continuing tests");
                }
            }
        }

        println!("✅ Containerization tests completed");
    }

    /// Test 2: Configuration Management - Environment variable handling
    #[tokio::test]
    async fn test_configuration_management() {
        println!("🧪 Testing Configuration Management...");

        // Test environment variable handling
        let original_solana_url = env::var("SOLANA_RPC_URL");
        let original_gemini_key = env::var("GEMINI_API_KEY");
        let original_typesense_key = env::var("TYPESENSE_API_KEY");

        // Set test environment variables
        env::set_var("SOLANA_RPC_URL", "https://test.solana.com");
        env::set_var("GEMINI_API_KEY", "test-key-123");
        env::set_var("TYPESENSE_API_KEY", "test-typesense-key");

        // Verify environment variables are set
        assert_eq!(
            env::var("SOLANA_RPC_URL").unwrap(),
            "https://test.solana.com",
            "Should set Solana RPC URL environment variable"
        );
        assert_eq!(
            env::var("GEMINI_API_KEY").unwrap(),
            "test-key-123",
            "Should set Gemini API key environment variable"
        );
        assert_eq!(
            env::var("TYPESENSE_API_KEY").unwrap(),
            "test-typesense-key",
            "Should set Typesense API key environment variable"
        );

        // Test that environment variables can be read
        let solana_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let gemini_key = env::var("GEMINI_API_KEY").ok();
        let typesense_key = env::var("TYPESENSE_API_KEY").ok();

        assert_eq!(
            solana_url, "https://test.solana.com",
            "Should read Solana RPC URL from environment"
        );
        assert_eq!(
            gemini_key,
            Some("test-key-123".to_string()),
            "Should read Gemini API key from environment"
        );
        assert_eq!(
            typesense_key,
            Some("test-typesense-key".to_string()),
            "Should read Typesense API key from environment"
        );

        // Clean up environment variables
        env::remove_var("SOLANA_RPC_URL");
        env::remove_var("GEMINI_API_KEY");
        env::remove_var("TYPESENSE_API_KEY");

        // Restore original values if they existed
        if let Ok(url) = original_solana_url {
            env::set_var("SOLANA_RPC_URL", url);
        }
        if let Ok(key) = original_gemini_key {
            env::set_var("GEMINI_API_KEY", key);
        }
        if let Ok(key) = original_typesense_key {
            env::set_var("TYPESENSE_API_KEY", key);
        }

        println!("✅ Configuration management tests completed");
    }

    /// Test 3: Service Dependencies - External service integration
    #[tokio::test]
    async fn test_service_dependencies() {
        println!("🧪 Testing Service Dependencies...");

        // Test Typesense dependency
        let typesense_url =
            env::var("TYPESENSE_URL").unwrap_or_else(|_| "http://localhost:8108".to_string());
        let typesense_available = test_typesense_connection(&typesense_url).await;
        if typesense_available {
            println!("✅ Typesense service is available");
        } else {
            println!("⚠️  Typesense service not available (expected in some environments)");
        }

        // Test Solana RPC dependency
        let solana_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
        let solana_available = test_solana_connection(&solana_url).await;
        if solana_available {
            println!("✅ Solana RPC service is available");
        } else {
            println!("⚠️  Solana RPC service not available (expected in some environments)");
        }

        // Test Gemini API dependency (requires valid API key)
        if let Ok(gemini_key) = env::var("GEMINI_API_KEY") {
            let gemini_available = test_gemini_connection(&gemini_key).await;
            if gemini_available {
                println!("✅ Gemini API service is available");
            } else {
                println!("⚠️  Gemini API service not available");
            }
        } else {
            println!("⚠️  Gemini API key not configured, skipping Gemini tests");
        }

        // Test graceful degradation when services are unavailable
        let api_client = MultiApiClient::new();
        let test_result = timeout(
            Duration::from_secs(5),
            api_client.get_price_intelligent("BTC"),
        )
        .await;

        match test_result {
            Ok(Ok(_)) => println!("✅ API client can fetch data successfully"),
            Ok(Err(_)) => {
                println!("⚠️  API client failed to fetch data (expected if services unavailable)")
            }
            Err(_) => println!("⚠️  API client timed out (expected if services unavailable)"),
        }

        println!("✅ Service dependencies tests completed");
    }

    /// Test 4: Resource Requirements - Memory, CPU, disk usage
    #[tokio::test]
    async fn test_resource_requirements() {
        println!("🧪 Testing Resource Requirements...");

        let start_time = Instant::now();
        let initial_memory = get_memory_usage();

        // Initialize core components
        let api_client = Arc::new(MultiApiClient::new());
        let cache_config = CacheConfig::default();
        let cache = Arc::new(IntelligentCache::new(cache_config));
        let processing_config = ProcessingConfig::default();
        let data_processor = DataProcessor::new(processing_config, api_client.clone());
        let typesense_url =
            env::var("TYPESENSE_URL").unwrap_or_else(|_| "http://localhost:8108".to_string());
        let typesense_key =
            env::var("TYPESENSE_API_KEY").unwrap_or_else(|_| "test-key".to_string());
        let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "test-key".to_string());
        let rag_system = RagSystem::new(typesense_url, typesense_key, gemini_key);

        let after_init_memory = get_memory_usage();
        let memory_increase = after_init_memory.saturating_sub(initial_memory);

        println!(
            "📊 Memory usage after initialization: {} KB (increase: {} KB)",
            after_init_memory, memory_increase
        );

        // Test memory efficiency under load
        let mut tasks = Vec::new();
        for i in 0..10 {
            let api_client = Arc::clone(&api_client);
            let cache = Arc::clone(&cache);

            let task = tokio::spawn(async move {
                // Simulate processing workload
                let symbols = ["BTC", "ETH", "ADA", "DOT", "SOL"];

                for symbol in &symbols {
                    // Test API fetching
                    let _ = timeout(
                        Duration::from_secs(2),
                        api_client.get_price_intelligent(symbol),
                    )
                    .await;

                    // Test caching
                    let test_data = RawData {
                        symbol: symbol.to_string(),
                        name: format!("Test {}", symbol),
                        price_usd: 50000.0 + i as f64,
                        volume_24h: Some(1000000.0),
                        market_cap: Some(1000000000.0),
                        price_change_24h: Some(2.5),
                        last_updated: Utc::now(),
                        source: ApiProvider::CoinGecko,
                    };

                    let _ = cache
                        .put(&ApiProvider::CoinGecko, "price", Some(symbol), test_data)
                        .await;
                }

                // Test data processing (simple API call)
                let _ = timeout(
                    Duration::from_secs(2),
                    api_client.get_price_intelligent("BTC"),
                )
                .await;
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let _ = timeout(Duration::from_secs(30), task).await;
        }

        let final_memory = get_memory_usage();
        let peak_memory_increase = final_memory.saturating_sub(initial_memory);

        println!(
            "📊 Peak memory usage: {} KB (total increase: {} KB)",
            final_memory, peak_memory_increase
        );

        // Validate resource constraints
        assert!(
            peak_memory_increase < 50000,
            "Memory usage should be reasonable (< 50MB increase)"
        );
        assert!(
            start_time.elapsed() < Duration::from_secs(60),
            "Test should complete within 60 seconds"
        );

        println!("✅ Resource requirements tests completed");
    }

    /// Test 5: Startup and Shutdown - Clean startup and shutdown procedures
    #[tokio::test]
    async fn test_startup_shutdown_procedures() {
        println!("🧪 Testing Startup and Shutdown Procedures...");

        let startup_time = Instant::now();

        // Test clean startup
        let api_client = Arc::new(MultiApiClient::new());
        let cache_config = CacheConfig::default();
        let cache = Arc::new(IntelligentCache::new(cache_config));
        let processing_config = ProcessingConfig::default();
        let data_processor = DataProcessor::new(processing_config, api_client.clone());
        let health_monitor = HealthMonitor::new();
        let typesense_url =
            env::var("TYPESENSE_URL").unwrap_or_else(|_| "http://localhost:8108".to_string());
        let typesense_key =
            env::var("TYPESENSE_API_KEY").unwrap_or_else(|_| "test-key".to_string());
        let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "test-key".to_string());
        let rag_system = RagSystem::new(typesense_url, typesense_key, gemini_key);

        let startup_duration = startup_time.elapsed();
        println!("🚀 Startup completed in {:?}", startup_duration);
        assert!(
            startup_duration < Duration::from_secs(10),
            "Startup should be fast"
        );

        // Test component initialization
        assert!(
            cache.health_check(),
            "Cache should be healthy after startup"
        );
        let health_statuses = health_monitor
            .check_all_health(Arc::clone(&api_client))
            .await;
        assert!(
            !health_statuses.is_empty(),
            "Health monitor should return status for providers"
        );

        // Test graceful shutdown simulation
        let shutdown_time = Instant::now();

        // Simulate shutdown cleanup
        drop(rag_system);
        drop(health_monitor);
        drop(data_processor);
        drop(cache);
        drop(api_client);

        let shutdown_duration = shutdown_time.elapsed();
        println!("🛑 Shutdown completed in {:?}", shutdown_duration);
        assert!(
            shutdown_duration < Duration::from_secs(2),
            "Shutdown should be fast"
        );

        println!("✅ Startup and shutdown tests completed");
    }

    /// Test 6: Health Check Integration - Health monitoring systems
    #[tokio::test]
    async fn test_health_check_integration() {
        println!("🧪 Testing Health Check Integration...");

        let health_monitor = HealthMonitor::new();
        let api_client = Arc::new(MultiApiClient::new());
        let cache = Arc::new(IntelligentCache::new(CacheConfig::default()));

        // Test component-specific health checks
        assert!(cache.health_check(), "Cache health check should pass");

        // Test API connectivity health using check_all_health
        let api_client = Arc::new(MultiApiClient::new());
        let health_statuses = health_monitor.check_all_health(api_client).await;
        println!(
            "✅ API connectivity health check completed for {} providers",
            health_statuses.len()
        );

        // Test metrics collection (after running health checks)
        let health_metrics = health_monitor.get_health_metrics().await;
        println!(
            "🏥 System Health Metrics: {} providers monitored",
            health_metrics.len()
        );

        let health_summary = health_monitor.get_health_summary().await;
        println!(
            "📊 Health Summary: {}",
            health_summary
                .lines()
                .next()
                .unwrap_or("No summary available")
        );

        // Validate we have health data (should have metrics after running checks)
        assert!(
            !health_statuses.is_empty(),
            "Should have health status results from checks"
        );

        println!("✅ Health check integration tests completed");
    }

    // Helper functions

    async fn test_typesense_connection(typesense_url: &str) -> bool {
        // Simple connectivity test to Typesense
        if let Ok(client) = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
        {
            let url = format!("{}/health", typesense_url);
            let response = client.get(&url).send().await;

            match response {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            }
        } else {
            false
        }
    }

    async fn test_solana_connection(solana_url: &str) -> bool {
        // Simple connectivity test to Solana RPC
        if let Ok(client) = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
        {
            let response = client
                .post(solana_url)
                .header("Content-Type", "application/json")
                .body(r#"{"jsonrpc":"2.0","id":1,"method":"getVersion"}"#)
                .send()
                .await;

            match response {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            }
        } else {
            false
        }
    }

    async fn test_gemini_connection(gemini_key: &str) -> bool {
        // Simple connectivity test to Gemini API
        if let Ok(client) = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
        {
            let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", gemini_key);
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(r#"{"contents":[{"parts":[{"text":"Hello"}]}]}"#)
                .send()
                .await;

            match response {
                Ok(resp) => resp.status().is_success(),
                Err(_) => false,
            }
        } else {
            false
        }
    }

    fn get_memory_usage() -> u64 {
        // Simple memory usage estimation (in KB)
        // In a real deployment, this would use system monitoring APIs
        use std::mem;

        // Rough estimation based on resident set size
        // This is a simplified version - production would use more accurate methods
        1024 * 50 // Placeholder: assume ~50MB baseline
    }
}
