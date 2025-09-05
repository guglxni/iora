use clap::{Arg, ArgMatches, Command};
use std::time::Duration;

pub fn build_cli() -> Command {
    Command::new("iora")
        .version("0.1.0")
        .author("IORA Dev Team <dev@iora.project>")
        .about("Intelligent Oracle Rust Assistant - Multi-API Crypto Data Fetching with RAG Intelligence")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("config")
                .about("Manage API configuration and BYOK settings")
                .subcommand_required(true)
                .subcommand(
                    Command::new("status")
                        .about("Show configuration status for all API providers")
                )
                .subcommand(
                    Command::new("set")
                        .about("Set API key for a specific provider")
                        .arg(
                            Arg::new("provider")
                                .short('p')
                                .long("provider")
                                .value_name("PROVIDER")
                                .help("API provider (coingecko, coinmarketcap, cryptocompare)")
                                .required(true)
                        )
                        .arg(
                            Arg::new("key")
                                .short('k')
                                .long("key")
                                .value_name("API_KEY")
                                .help("API key to set")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("validate")
                        .about("Validate API key format for a provider")
                        .arg(
                            Arg::new("provider")
                                .short('p')
                                .long("provider")
                                .value_name("PROVIDER")
                                .help("API provider to validate")
                                .required(true)
                        )
                        .arg(
                            Arg::new("key")
                                .short('k')
                                .long("key")
                                .value_name("API_KEY")
                                .help("API key to validate")
                        )
                )
        )
        .subcommand(
            Command::new("resilience")
                .about("Monitor and manage API resilience features")
                .subcommand_required(true)
                .subcommand(
                    Command::new("status")
                        .about("Show resilience status for all API providers")
                )
                .subcommand(
                    Command::new("metrics")
                        .about("Show detailed resilience metrics for all providers")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset circuit breaker for a specific provider")
        .arg(
                            Arg::new("provider")
                                .short('p')
                                .long("provider")
                                .value_name("PROVIDER")
                                .help("API provider to reset")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("health")
                        .about("Show health status of all providers")
                )
        )
        .subcommand(
            Command::new("cache")
                .about("Manage intelligent caching system")
                .subcommand_required(true)
                .subcommand(
                    Command::new("status")
                        .about("Show cache status and statistics")
                )
                .subcommand(
                    Command::new("stats")
                        .about("Show detailed cache statistics")
                )
                .subcommand(
                    Command::new("clear")
                        .about("Clear entire cache")
                )
                .subcommand(
                    Command::new("invalidate")
                        .about("Invalidate cache for a specific provider")
                        .arg(
                            Arg::new("provider")
                                .short('p')
                                .long("provider")
                                .value_name("PROVIDER")
                                .help("API provider to invalidate")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("warm")
                        .about("Warm cache with popular data")
                        .subcommand(
                            Command::new("symbols")
                                .about("Warm cache with popular symbols")
        .arg(
                                    Arg::new("symbols")
                                        .short('s')
                                        .long("symbols")
                                        .value_name("SYMBOLS")
                                        .help("Comma-separated list of symbols")
                                        .required(false)
                                )
                        )
                        .subcommand(
                            Command::new("global")
                                .about("Warm cache with global market data")
                        )
                )
                .subcommand(
                    Command::new("health")
                        .about("Show cache health status")
                )
        )
        .subcommand(
            Command::new("analytics")
                .about("API usage analytics and optimization")
                .subcommand_required(true)
                .subcommand(
                    Command::new("dashboard")
                        .about("Show analytics dashboard with performance metrics")
                )
                .subcommand(
                    Command::new("usage")
                        .about("Show API usage metrics for all providers")
                )
                .subcommand(
                    Command::new("performance")
                        .about("Show performance metrics and statistics")
                )
                .subcommand(
                    Command::new("costs")
                        .about("Show cost analysis for API usage")
                )
                .subcommand(
                    Command::new("recommend")
                        .about("Show optimization recommendations")
                )
                .subcommand(
                    Command::new("export")
                        .about("Export analytics data for external analysis")
                )
        )
        .subcommand(
            Command::new("health")
                .about("API health monitoring and performance benchmarking")
                .subcommand_required(true)
                .subcommand(
                    Command::new("status")
                        .about("Show health status of all API providers")
                )
                .subcommand(
                    Command::new("check")
                        .about("Perform health check on all APIs")
                )
                .subcommand(
                    Command::new("monitor")
                        .about("Start continuous health monitoring")
                )
                .subcommand(
                    Command::new("alerts")
                        .about("Show recent health alerts")
                )
                .subcommand(
                    Command::new("benchmark")
                        .about("Run performance benchmarks on all APIs")
                )
                .subcommand(
                    Command::new("dashboard")
                        .about("Show health monitoring dashboard")
                )
                .subcommand(
                    Command::new("summary")
                        .about("Show health status summary")
                )
        )
        .subcommand(
            Command::new("rag")
                .about("RAG (Retrieval-Augmented Generation) system management")
                .subcommand_required(true)
                .subcommand(
                    Command::new("init")
                        .about("Initialize Typesense client and create historical_data collection")
                )
                .subcommand(
                    Command::new("status")
                        .about("Check RAG system status and initialization")
                )
                .subcommand(
                    Command::new("index")
                        .about("Index historical data from JSON file")
                        .arg(
                            Arg::new("file")
                                .short('f')
                                .long("file")
                                .value_name("FILE")
                                .help("Path to historical data JSON file")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("search")
                        .about("Search for relevant historical data")
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("QUERY")
                                .help("Search query")
                                .required(true)
        )
        .arg(
                            Arg::new("limit")
                                .short('l')
                                .long("limit")
                                .value_name("LIMIT")
                                .help("Maximum number of results to return")
                                .default_value("5")
                        )
                )
                .subcommand(
                    Command::new("augment")
                        .about("Augment data with hybrid search retrieval")
        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol (e.g., bitcoin)")
                                .required(true)
                        )
                        .arg(
                            Arg::new("price")
                                .short('p')
                                .long("price")
                                .value_name("PRICE")
                                .help("Current price in USD")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("benchmark")
                        .about("Run comprehensive RAG performance benchmarks")
                        .arg(
                            Arg::new("data_file")
                                .short('f')
                                .long("data-file")
                                .value_name("FILE")
                                .help("Path to historical data file for benchmarking (optional)")
                                .required(false)
                        )
                )
        )
        .subcommand(
            Command::new("load-test")
                .about("Load testing and scalability validation")
                .subcommand_required(true)
                .subcommand(
                    Command::new("concurrent-users")
                        .about("Test concurrent user load scenarios")
                        .arg(
                            Arg::new("users")
                                .short('u')
                                .long("users")
                                .value_name("COUNT")
                                .help("Number of concurrent users")
                                .default_value("10")
                        )
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("60")
                        )
                        .arg(
                            Arg::new("operations")
                                .short('o')
                                .long("operations")
                                .value_name("COUNT")
                                .help("Operations per user")
                                .default_value("50")
                        )
                )
                .subcommand(
                    Command::new("data-volume")
                        .about("Test data volume scalability")
                        .arg(
                            Arg::new("size")
                                .short('s')
                                .long("size")
                                .value_name("MB")
                                .help("Data size in MB")
                                .default_value("100")
                        )
                        .arg(
                            Arg::new("batch")
                                .short('b')
                                .long("batch")
                                .value_name("SIZE")
                                .help("Batch size for operations")
                                .default_value("1000")
                        )
                )
                .subcommand(
                    Command::new("resource-stress")
                        .about("Test system resource limits")
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("30")
                        )
                        .arg(
                            Arg::new("memory")
                                .short('m')
                                .long("memory")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable memory pressure testing")
                        )
                        .arg(
                            Arg::new("cpu")
                                .short('c')
                                .long("cpu")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable CPU pressure testing")
                        )
                        .arg(
                            Arg::new("io")
                                .short('i')
                                .long("io")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable I/O pressure testing")
                        )
                        .arg(
                            Arg::new("network")
                                .short('n')
                                .long("network")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable network pressure testing")
                        )
                )
                .subcommand(
                    Command::new("mixed-workload")
                        .about("Test mixed workload scenarios")
                        .arg(
                            Arg::new("users")
                                .short('u')
                                .long("users")
                                .value_name("COUNT")
                                .help("Number of concurrent users")
                                .default_value("20")
                        )
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("120")
                        )
                )
                .subcommand(
                    Command::new("full-suite")
                        .about("Run complete load testing suite")
                        .arg(
                            Arg::new("output")
                                .short('o')
                                .long("output")
                                .value_name("FILE")
                                .help("Output file for results")
                                .default_value("load_test_results.json")
                        )
                )
        )
        .subcommand(
            Command::new("resilience-test")
                .about("Error handling and resilience testing")
                .subcommand_required(true)
                .subcommand(
                    Command::new("api-failures")
                        .about("Test API failure scenarios")
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("30")
                        )
                        .arg(
                            Arg::new("circuit-breaker")
                                .short('c')
                                .long("circuit-breaker")
                                .action(clap::ArgAction::SetTrue)
                                .help("Enable circuit breaker testing")
                        )
                )
                .subcommand(
                    Command::new("network-failures")
                        .about("Test network connectivity failures")
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("30")
                        )
                )
                .subcommand(
                    Command::new("recovery-test")
                        .about("Test system recovery from failures")
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("60")
                        )
                        .arg(
                            Arg::new("failure-rate")
                                .short('f')
                                .long("failure-rate")
                                .value_name("RATE")
                                .help("Simulated failure rate (0.0-1.0)")
                                .default_value("0.3")
                        )
                )
                .subcommand(
                    Command::new("comprehensive")
                        .about("Run comprehensive resilience test suite")
                        .arg(
                            Arg::new("duration")
                                .short('d')
                                .long("duration")
                                .value_name("SECONDS")
                                .help("Test duration in seconds")
                                .default_value("120")
                        )
                        .arg(
                            Arg::new("output")
                                .short('o')
                                .long("output")
                                .value_name("FILE")
                                .help("Output file for results")
                                .default_value("resilience_test_results.json")
                        )
                )
        )
        .subcommand(
            Command::new("process")
                .about("Data processing and normalization commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("price")
                        .about("Get normalized price data")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("stats")
                        .about("Show processing statistics")
                )
                .subcommand(
                    Command::new("historical")
                        .about("Get normalized historical data")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                        .arg(
                            Arg::new("limit")
                                .short('l')
                                .long("limit")
                                .value_name("LIMIT")
                                .help("Number of data points")
                                .default_value("100")
                        )
                )
        )
        .subcommand(
            Command::new("historical")
                .about("Historical data management commands")
                .subcommand_required(true)
                .subcommand(
                    Command::new("fetch")
                        .about("Fetch and store historical data")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                        .arg(
                            Arg::new("start")
                                .long("start")
                                .value_name("START_DATE")
                                .help("Start date (YYYY-MM-DD)")
                                .default_value("2023-01-01")
                        )
                        .arg(
                            Arg::new("end")
                                .long("end")
                                .value_name("END_DATE")
                                .help("End date (YYYY-MM-DD)")
                                .default_value("2024-01-01")
                        )
                        .arg(
                            Arg::new("interval")
                                .short('i')
                                .long("interval")
                                .value_name("INTERVAL")
                                .help("Data interval (1d, 1h, etc.)")
                                .default_value("1d")
                        )
                )
                .subcommand(
                    Command::new("query")
                        .about("Query stored historical data")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                        .arg(
                            Arg::new("start")
                                .long("start")
                                .value_name("START_DATE")
                                .help("Start date (YYYY-MM-DD)")
                        )
                        .arg(
                            Arg::new("end")
                                .long("end")
                                .value_name("END_DATE")
                                .help("End date (YYYY-MM-DD)")
                        )
                        .arg(
                            Arg::new("limit")
                                .short('l')
                                .long("limit")
                                .value_name("LIMIT")
                                .help("Maximum number of data points")
                        )
                )
                .subcommand(
                    Command::new("stats")
                        .about("Show historical data storage statistics")
                )
                .subcommand(
                    Command::new("metadata")
                        .about("Show metadata for a symbol")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("optimize")
                        .about("Optimize historical data for RAG training")
                        .arg(
                            Arg::new("symbol")
                                .short('s')
                                .long("symbol")
                                .value_name("SYMBOL")
                                .help("Cryptocurrency symbol")
                                .required(true)
                        )
                )
        )
        .subcommand(
            Command::new("query")
                .about("Execute a crypto data query")
                .arg(
                    Arg::new("symbol")
                        .short('s')
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Cryptocurrency symbol (e.g., BTC, ETH)")
                .required(true)
        )
        )
}

/// Handle CLI commands and return appropriate exit code
pub async fn handle_cli_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("config", config_matches)) => {
            handle_config_command(config_matches).await
        }
        Some(("query", query_matches)) => {
            handle_query_command(query_matches).await
        }
        Some(("resilience", resilience_matches)) => {
            handle_resilience_command(resilience_matches).await
        }
        Some(("cache", cache_matches)) => {
            handle_cache_command(cache_matches).await
        }
        Some(("process", process_matches)) => {
            handle_process_command(process_matches).await
        }
        Some(("historical", historical_matches)) => {
            handle_historical_command(historical_matches).await
        }
        Some(("analytics", analytics_matches)) => {
            handle_analytics_command(analytics_matches).await
        }
        Some(("health", health_matches)) => {
            handle_health_command(health_matches).await
        }
        Some(("rag", rag_matches)) => {
            handle_rag_command(rag_matches).await
        }
        Some(("load-test", load_test_matches)) => {
            handle_load_test_command(load_test_matches).await
        }
        Some(("resilience-test", resilience_matches)) => {
            handle_resilience_test_command(resilience_matches).await
        }
        _ => Ok(()), // No subcommand, handled in main
    }
}

/// Handle configuration subcommands
async fn handle_config_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::ByokConfigManager;

    let config_manager = ByokConfigManager::new();
    config_manager.load_from_env().await?;

    match matches.subcommand() {
        Some(("status", _)) => {
            let status = config_manager.get_config_status().await;
            println!("📊 API Configuration Status:");
            println!("{}", "=".repeat(50));

            for (provider, config_status) in status {
                let status_icon = match config_status {
                    crate::modules::fetcher::ConfigStatus::Configured => "✅",
                    crate::modules::fetcher::ConfigStatus::NotConfigured => "❌",
                    crate::modules::fetcher::ConfigStatus::Invalid => "⚠️ ",
                };

                let status_text = match config_status {
                    crate::modules::fetcher::ConfigStatus::Configured => "Configured",
                    crate::modules::fetcher::ConfigStatus::NotConfigured => "Not Configured",
                    crate::modules::fetcher::ConfigStatus::Invalid => "Invalid Configuration",
                };

                println!("{} {:<15} {}", status_icon, provider.to_string(), status_text);
            }
        }
        Some(("set", set_matches)) => {
            let provider_str = set_matches.get_one::<String>("provider").unwrap();
            let api_key = set_matches.get_one::<String>("key").unwrap();

            let provider = match provider_str.as_str() {
                "coingecko" => crate::modules::fetcher::ApiProvider::CoinGecko,
                "coinmarketcap" => crate::modules::fetcher::ApiProvider::CoinMarketCap,
                "cryptocompare" => crate::modules::fetcher::ApiProvider::CryptoCompare,
                _ => {
                    eprintln!("❌ Unknown provider: {}", provider_str);
                    eprintln!("Available providers: coingecko, coinmarketcap, cryptocompare");
                    std::process::exit(1);
                }
            };

            match config_manager.update_api_key(provider, api_key.clone()).await {
                Ok(()) => {
                    println!("✅ Successfully set API key for {}", provider_str);
                    println!("💡 Key validation passed!");
                }
                Err(e) => {
                    eprintln!("❌ Failed to set API key: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("validate", validate_matches)) => {
            let provider_str = validate_matches.get_one::<String>("provider").unwrap();
            let api_key = validate_matches.get_one::<String>("key");

            let provider = match provider_str.as_str() {
                "coingecko" => crate::modules::fetcher::ApiProvider::CoinGecko,
                "coinmarketcap" => crate::modules::fetcher::ApiProvider::CoinMarketCap,
                "cryptocompare" => crate::modules::fetcher::ApiProvider::CryptoCompare,
                "coinpaprika" => crate::modules::fetcher::ApiProvider::CoinPaprika,
                _ => {
                    eprintln!("❌ Unknown provider: {}", provider_str);
                    std::process::exit(1);
                }
            };

            let key_to_validate = api_key.cloned().unwrap_or_else(|| {
                std::env::var(&format!("{}_API_KEY", provider_str.to_uppercase())).unwrap_or_default()
            });

            match config_manager.validate_api_key(provider, &key_to_validate) {
                Ok(()) => {
                    println!("✅ API key validation passed for {}", provider_str);
                    println!("🔐 Key format is valid!");
                }
                Err(e) => {
                    eprintln!("❌ API key validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("❌ Invalid config subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle health monitoring subcommands
async fn handle_health_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis()
        .with_health_monitoring(); // Enable health monitoring

    match matches.subcommand() {
        Some(("status", _)) => {
            println!("🏥 API Health Status");
            println!("===================");

            if let Some(metrics) = client.get_health_metrics().await {
                for (provider, metric) in metrics {
                    let status_icon = match metric.status {
                        crate::modules::health::HealthStatus::Healthy => "✅",
                        crate::modules::health::HealthStatus::Degraded => "⚠️",
                        crate::modules::health::HealthStatus::Unhealthy => "🚨",
                        crate::modules::health::HealthStatus::Down => "❌",
                        crate::modules::health::HealthStatus::Unknown => "❓",
                    };

                    println!("{} {}: {:.1}% uptime, {:.2}s avg response",
                        status_icon,
                        provider,
                        metric.uptime_percentage,
                        metric.average_response_time.as_secs_f64()
                    );
                }
            } else {
                println!("❌ Health monitoring not enabled");
                println!("💡 Enable health monitoring by using: client.with_health_monitoring()");
            }
        }
        Some(("check", _)) => {
            println!("🔍 Performing Health Check");
            println!("==========================");

            if let Some(results) = client.check_all_api_health().await {
                for (provider, status) in results {
                    let status_icon = match status {
                        crate::modules::health::HealthStatus::Healthy => "✅",
                        crate::modules::health::HealthStatus::Degraded => "⚠️",
                        crate::modules::health::HealthStatus::Unhealthy => "🚨",
                        crate::modules::health::HealthStatus::Down => "❌",
                        crate::modules::health::HealthStatus::Unknown => "❓",
                    };

                    println!("{} {}: {}", status_icon, provider, format!("{:?}", status));
                }
            } else {
                println!("❌ Health monitoring not enabled");
            }
        }
        Some(("monitor", _)) => {
            println!("📊 Starting Continuous Health Monitoring");
            println!("=======================================");
            println!("🔄 Health monitoring started in background...");
            println!("📋 Monitoring all API providers every 60 seconds");
            println!("🔔 Alerts will be displayed in console");
            println!("💡 Press Ctrl+C to stop monitoring");

            client.start_continuous_health_monitoring();

            // Keep the process running
            tokio::signal::ctrl_c().await?;
            println!("\n🛑 Health monitoring stopped");
        }
        Some(("alerts", _)) => {
            println!("🚨 Recent Health Alerts");
            println!("======================");

            if let Some(alerts) = client.get_health_alerts(10).await {
                if alerts.is_empty() {
                    println!("✅ No recent alerts - all systems healthy!");
                } else {
                    for alert in alerts {
                        let severity_icon = match alert.severity {
                            crate::modules::health::AlertSeverity::Info => "ℹ️",
                            crate::modules::health::AlertSeverity::Warning => "⚠️",
                            crate::modules::health::AlertSeverity::Critical => "🚨",
                            crate::modules::health::AlertSeverity::Emergency => "🚨🚨",
                        };

                        println!(
                            "{} {} - {} | {} failures | {}",
                            severity_icon,
                            alert.timestamp.format("%H:%M:%S"),
                            alert.provider,
                            alert.consecutive_failures,
                            alert.title
                        );

                        if !alert.resolved {
                            println!("   📝 Status: ACTIVE");
                        } else {
                            println!("   ✅ Status: RESOLVED");
                        }
                    }
                }
            } else {
                println!("❌ Health monitoring not enabled");
            }
        }
        Some(("benchmark", _)) => {
            println!("⚡ Running Performance Benchmarks");
            println!("================================");

            if let Some(results) = client.run_performance_benchmarks().await {
                println!("📊 Benchmark Results:");
                println!("Total Requests: {}", results.len());

                let successful = results.iter().filter(|r| r.success).count();
                println!("Successful: {} ({:.1}%)", successful, (successful as f64 / results.len() as f64) * 100.0);

                if !results.is_empty() {
                    let avg_response_time = results.iter()
                        .map(|r| r.response_time)
                        .sum::<std::time::Duration>() / results.len() as u32;

                    println!("Average Response Time: {:.2}ms", avg_response_time.as_millis());

                    // Find fastest and slowest
                    if let Some(fastest) = results.iter().filter(|r| r.success).min_by_key(|r| r.response_time) {
                        println!("Fastest Provider: {} ({:.2}ms)", fastest.provider, fastest.response_time.as_millis());
                    }

                    if let Some(slowest) = results.iter().filter(|r| r.success).max_by_key(|r| r.response_time) {
                        println!("Slowest Provider: {} ({:.2}ms)", slowest.provider, slowest.response_time.as_millis());
                    }
                }
            } else {
                println!("❌ Health monitoring not enabled");
            }
        }
        Some(("dashboard", _)) => {
            println!("📊 Health Monitoring Dashboard");
            println!("==============================");

            if let Some(dashboard) = client.get_health_dashboard().await {
                println!("{}", serde_json::to_string_pretty(&dashboard)?);
            } else {
                println!("❌ Health monitoring not enabled");
                println!("💡 Enable health monitoring by using: client.with_health_monitoring()");
            }
        }
        Some(("summary", _)) => {
            println!("📋 Health Status Summary");
            println!("========================");

            if let Some(summary) = client.get_health_summary().await {
                println!("{}", summary);
            } else {
                println!("❌ Health monitoring not enabled");
            }
        }
        _ => {
            eprintln!("❌ Unknown health subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle resilience subcommands
async fn handle_resilience_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis();

    match matches.subcommand() {
        Some(("status", _)) => {
            let status = client.get_all_resilience_status();
            println!("🛡️  API Resilience Status:");
            println!("{}", "=".repeat(70));

            for (provider, resilience_status) in status {
                let circuit_icon = match resilience_status.circuit_state {
                    crate::modules::fetcher::CircuitState::Closed => "🟢",
                    crate::modules::fetcher::CircuitState::Open => "🔴",
                    crate::modules::fetcher::CircuitState::HalfOpen => "🟡",
                };

                let health_icon = if resilience_status.is_healthy { "✅" } else { "❌" };

                println!("{} {:<15} Circuit: {} | Success: {:.1}% | Failures: {} | Health: {}",
                    health_icon,
                    provider.to_string(),
                    circuit_icon,
                    resilience_status.success_rate * 100.0,
                    resilience_status.consecutive_failures,
                    if resilience_status.is_healthy { "Good" } else { "Poor" }
                );
            }
        }
        Some(("metrics", _)) => {
            let metrics = client.get_resilience_metrics();
            println!("📊 Detailed Resilience Metrics:");
            println!("{}", "=".repeat(80));

            for (provider, provider_metrics) in metrics {
                println!("🔧 {}", provider);
                println!("   Total Requests: {}", provider_metrics.total_requests.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Successful: {}", provider_metrics.successful_requests.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Failed: {}", provider_metrics.failed_requests.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Timeouts: {}", provider_metrics.timeout_count.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Rate Limits: {}", provider_metrics.rate_limit_count.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Consecutive Failures: {}", provider_metrics.consecutive_failures.load(std::sync::atomic::Ordering::SeqCst));
                println!("   Success Rate: {:.1}%", provider_metrics.get_success_rate() * 100.0);
                println!();
            }
        }
        Some(("reset", reset_matches)) => {
            let provider_str = reset_matches.get_one::<String>("provider").unwrap();

            let provider = match provider_str.as_str() {
                "coingecko" => crate::modules::fetcher::ApiProvider::CoinGecko,
                "coinmarketcap" => crate::modules::fetcher::ApiProvider::CoinMarketCap,
                "cryptocompare" => crate::modules::fetcher::ApiProvider::CryptoCompare,
                "coinpaprika" => crate::modules::fetcher::ApiProvider::CoinPaprika,
                _ => {
                    eprintln!("❌ Unknown provider: {}", provider_str);
                    std::process::exit(1);
                }
            };

            client.reset_circuit_breaker(&provider);
        }
        Some(("health", _)) => {
            let status = client.get_all_resilience_status();
            let config = client.get_resilience_config();

            println!("🏥 API Health Dashboard:");
            println!("{}", "=".repeat(60));
            println!("🔄 Resilience Configuration:");
            println!("   Max Retries: {}", config.max_retries);
            println!("   Base Delay: {}ms", config.base_delay_ms);
            println!("   Max Delay: {}ms", config.max_delay_ms);
            println!("   Timeout: {}s", config.timeout_seconds);
            println!("   Circuit Breaker Threshold: {}", config.circuit_breaker_threshold);
            println!();

            println!("📈 Health Summary:");
            let healthy_count = status.values().filter(|s| s.is_healthy).count();
            let total_count = status.len();
            println!("   Healthy APIs: {}/{}", healthy_count, total_count);
            println!("   Overall Health: {:.1}%", (healthy_count as f64 / total_count as f64) * 100.0);

            let open_circuits = status.values().filter(|s| matches!(s.circuit_state, crate::modules::fetcher::CircuitState::Open)).count();
            if open_circuits > 0 {
                println!("   ⚠️  Open Circuit Breakers: {}", open_circuits);
            }
        }
        _ => {
            eprintln!("❌ Invalid resilience subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle cache management subcommands
async fn handle_cache_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis().with_caching();

    match matches.subcommand() {
        Some(("status", _)) => {
            println!("🗄️  Intelligent Cache Status:");
            println!("{}", "=".repeat(50));

            if client.is_caching_enabled() {
                println!("✅ Caching: Enabled");

                if let Some(hit_rate) = client.get_cache_hit_rate() {
                    println!("🎯 Hit Rate: {:.1}%", hit_rate * 100.0);
                }

                if let Some((current_size, max_size, utilization)) = client.get_cache_info() {
                    println!("💾 Cache Size: {:.2} MB / {:.2} MB ({:.1}% utilization)",
                        current_size as f64 / (1024.0 * 1024.0),
                        max_size as f64 / (1024.0 * 1024.0),
                        utilization);
                }

                if let Some(health) = client.get_cache_health() {
                    let health_icon = if health { "✅" } else { "❌" };
                    println!("🏥 Health: {} {}", health_icon, if health { "Good" } else { "Poor" });
                }
            } else {
                println!("❌ Caching: Disabled");
                println!("💡 Enable caching with: iora config --enable-cache");
            }
        }
        Some(("stats", _)) => {
            println!("📊 Detailed Cache Statistics:");
            println!("{}", "=".repeat(60));

            if let Some(stats) = client.get_cache_stats() {
                println!("📈 Total Requests: {}", stats.total_requests);
                println!("✅ Cache Hits: {}", stats.cache_hits);
                println!("❌ Cache Misses: {}", stats.cache_misses);
                println!("🗑️  Evictions: {}", stats.evictions);
                println!("🗜️  Compression Savings: {} bytes", stats.compression_savings);

                if stats.total_requests > 0 {
                    let avg_response_time = stats.average_response_time.num_milliseconds() as f64;
                    println!("⏱️  Average Response Time: {:.2}ms", avg_response_time);
                }
            } else {
                println!("❌ Cache not enabled or no statistics available");
            }
        }
        Some(("clear", _)) => {
            println!("🧹 Clearing cache...");
            client.clear_cache().await;
            println!("✅ Cache cleared successfully");
        }
        Some(("invalidate", invalidate_matches)) => {
            let provider_str = invalidate_matches.get_one::<String>("provider").unwrap();

            let provider = match provider_str.as_str() {
                "coingecko" => crate::modules::fetcher::ApiProvider::CoinGecko,
                "coinmarketcap" => crate::modules::fetcher::ApiProvider::CoinMarketCap,
                "cryptocompare" => crate::modules::fetcher::ApiProvider::CryptoCompare,
                "coinpaprika" => crate::modules::fetcher::ApiProvider::CoinPaprika,
                _ => {
                    eprintln!("❌ Unknown provider: {}", provider_str);
                    std::process::exit(1);
                }
            };

            println!("🔄 Invalidating cache for {}...", provider_str);
            client.invalidate_provider_cache(&provider).await;
            println!("✅ Cache invalidated for {}", provider_str);
        }
        Some(("warm", warm_matches)) => {
            match warm_matches.subcommand() {
                Some(("symbols", symbols_matches)) => {
                    let symbols = if let Some(symbols_str) = symbols_matches.get_one::<String>("symbols") {
                        symbols_str.split(',').map(|s| s.trim().to_string()).collect()
                    } else {
                        // Default popular symbols
                        vec!["BTC".to_string(), "ETH".to_string(), "USDT".to_string(), "BNB".to_string()]
                    };

                    println!("🔥 Warming cache with symbols: {:?}", symbols);
                    client.warm_cache_with_popular_symbols(symbols).await;
                    println!("✅ Cache warming completed");
                }
                Some(("global", _)) => {
                    println!("🌍 Warming cache with global market data...");
                    client.warm_cache_with_global_data().await;
                    println!("✅ Global data cache warming completed");
                }
                _ => {
                    eprintln!("❌ Invalid warm subcommand");
                    std::process::exit(1);
                }
            }
        }
        Some(("health", _)) => {
            println!("🏥 Cache Health Check:");
            println!("{}", "=".repeat(40));

            if let Some(health) = client.get_cache_health() {
                if health {
                    println!("✅ Cache System: Healthy");
                    println!("   All components operational");
                } else {
                    println!("❌ Cache System: Unhealthy");
                    println!("   Some components may be malfunctioning");
                }
            } else {
                println!("❌ Cache System: Not Enabled");
                println!("   Enable caching to monitor health");
            }

            // Show popular keys if available
            if let Some(popular_keys) = client.get_popular_cache_keys(5) {
                if !popular_keys.is_empty() {
                    println!();
                    println!("🔥 Popular Cache Keys:");
                    for (i, key) in popular_keys.iter().enumerate() {
                        println!("   {}. {}", i + 1, key);
                    }
                }
            }
        }
        _ => {
            eprintln!("❌ Invalid cache subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle data processing subcommands
async fn handle_process_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis().with_caching().with_processing();

    match matches.subcommand() {
        Some(("price", price_matches)) => {
            let symbol = price_matches.get_one::<String>("symbol").unwrap();

            println!("🔄 Processing normalized price data for {}...", symbol);
            println!("{}", "=".repeat(60));

            match client.get_normalized_price(symbol).await {
                Ok(normalized_data) => {
                    println!("📊 Normalized Price Data:");
                    println!("   Symbol: {}", normalized_data.symbol);
                    println!("   Name: {}", normalized_data.name);
                    println!("   Price: ${:.2}", normalized_data.price_usd);
                    println!("   Sources: {}", normalized_data.sources.len());
                    println!("   Quality Score: {:.2}", normalized_data.quality_score);
                    println!("   Reliability Score: {:.2}", normalized_data.reliability_score);
                    println!("   Last Updated: {}", normalized_data.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));

                    if let Some(volume) = normalized_data.volume_24h {
                        println!("   24h Volume: ${:.0}", volume);
                    }

                    if let Some(change) = normalized_data.price_change_24h {
                        println!("   24h Change: {:.2}%", change);
                    }

                    println!("\n📈 Consensus Analysis:");
                    println!("   Consensus Price: ${:.2}", normalized_data.consensus.consensus_price);
                    println!("   Price Range: ${:.2}", normalized_data.consensus.price_range);
                    println!("   Standard Deviation: ${:.2}", normalized_data.consensus.price_std_dev);
                    println!("   Confidence: {:.2}%", normalized_data.consensus.consensus_confidence * 100.0);

                    if !normalized_data.consensus.outliers.is_empty() {
                        println!("   ⚠️  Outliers: {}", normalized_data.consensus.outliers.len());
                    }

                    println!("\n🏷️  Metadata:");
                    if !normalized_data.metadata.exchanges.is_empty() {
                        println!("   Exchanges: {}", normalized_data.metadata.exchanges.join(", "));
                    }
                    if !normalized_data.metadata.categories.is_empty() {
                        println!("   Categories: {}", normalized_data.metadata.categories.join(", "));
                    }
                    if let Some(market_cap) = normalized_data.market_cap {
                        println!("   Market Cap: ${:.0}", market_cap);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Processing failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("stats", _)) => {
            println!("📊 Processing Statistics:");
            println!("{}", "=".repeat(40));

            if let Some(stats) = client.get_processing_stats().await {
                println!("📈 Cache Entries: {}", stats.cache_entries);
                println!("🏷️  Metadata Cache: {}", stats.metadata_cache_entries);
                println!("⚡ Active Operations: {}", stats.active_semaphore_permits);
            } else {
                println!("❌ Processing not enabled");
            }
        }
        Some(("historical", historical_matches)) => {
            let symbol = historical_matches.get_one::<String>("symbol").unwrap();
            let limit: usize = historical_matches.get_one::<String>("limit")
                .unwrap()
                .parse()
                .unwrap_or(100);

            println!("📈 Processing normalized historical data for {} (limit: {})...", symbol, limit);
            println!("{}", "=".repeat(60));

            match client.get_normalized_historical(symbol, limit).await {
                Ok(data) => {
                    if data.is_empty() {
                        println!("❌ No historical data available");
                    } else {
                        println!("✅ Successfully processed {} data points", data.len());
                        for (i, point) in data.iter().enumerate() {
                            if i >= 5 { // Show only first 5 for brevity
                                println!("   ... and {} more data points", data.len() - 5);
                                break;
                            }
                            println!("   {}: ${:.2} (Quality: {:.2})",
                                point.last_updated.format("%Y-%m-%d %H:%M"),
                                point.price_usd,
                                point.quality_score
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Processing failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("❌ Invalid process subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle historical data management subcommands
async fn handle_historical_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis()
        .with_caching()
        .with_processing()
        .with_historical();

    match matches.subcommand() {
        Some(("fetch", fetch_matches)) => {
            let symbol = fetch_matches.get_one::<String>("symbol").unwrap();
            let start_date_str = fetch_matches.get_one::<String>("start").unwrap();
            let end_date_str = fetch_matches.get_one::<String>("end").unwrap();
            let interval = fetch_matches.get_one::<String>("interval").unwrap();

            // Parse dates
            let start_date = chrono::NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d")
                .map_err(|_| "Invalid start date format. Use YYYY-MM-DD")?
                .and_hms_opt(0, 0, 0).unwrap();
            let end_date = chrono::NaiveDate::parse_from_str(end_date_str, "%Y-%m-%d")
                .map_err(|_| "Invalid end date format. Use YYYY-MM-DD")?
                .and_hms_opt(23, 59, 59).unwrap();

            let start_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(start_date, chrono::Utc);
            let end_utc = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(end_date, chrono::Utc);

            println!("📈 Fetching historical data for {} from {} to {} (interval: {})",
                     symbol, start_date_str, end_date_str, interval);
            println!("{}", "=".repeat(80));

            match client.fetch_historical_data(symbol, start_utc, end_utc, interval).await {
                Ok(_) => {
                    println!("✅ Successfully fetched and stored historical data for {}", symbol);
                }
                Err(e) => {
                    eprintln!("❌ Failed to fetch historical data: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("query", query_matches)) => {
            let symbol = query_matches.get_one::<String>("symbol").unwrap();

            // Parse optional dates
            let start_date = if let Some(start_str) = query_matches.get_one::<String>("start") {
                Some(chrono::NaiveDate::parse_from_str(start_str, "%Y-%m-%d")
                    .map_err(|_| "Invalid start date format. Use YYYY-MM-DD")?
                    .and_hms_opt(0, 0, 0).unwrap())
            } else {
                None
            };

            let end_date = if let Some(end_str) = query_matches.get_one::<String>("end") {
                Some(chrono::NaiveDate::parse_from_str(end_str, "%Y-%m-%d")
                    .map_err(|_| "Invalid end date format. Use YYYY-MM-DD")?
                    .and_hms_opt(23, 59, 59).unwrap())
            } else {
                None
            };

            let start_utc = start_date.map(|d| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc));
            let end_utc = end_date.map(|d| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc));
            let limit = query_matches.get_one::<String>("limit")
                .and_then(|s| s.parse().ok());

            println!("🔍 Querying historical data for {}", symbol);
            if let Some(limit) = limit {
                println!("   Limit: {} data points", limit);
            }
            println!("{}", "=".repeat(60));

            match client.query_historical_data(symbol, start_utc, end_utc, limit).await {
                Ok(data) => {
                    if data.is_empty() {
                        println!("❌ No historical data found for {}", symbol);
                    } else {
                        println!("✅ Found {} historical data points", data.len());
                        println!("\n📊 Recent Data Points:");

                        // Show last 5 data points
                        let display_count = std::cmp::min(5, data.len());
                        for (i, point) in data.iter().rev().take(display_count).enumerate() {
                            let idx = data.len() - display_count + i;
                            println!("   {}. {}: O:${:.2} H:${:.2} L:${:.2} C:${:.2} V:{:.0}",
                                idx + 1,
                                point.timestamp.format("%Y-%m-%d %H:%M"),
                                point.open,
                                point.high,
                                point.low,
                                point.close,
                                point.volume
                            );
                        }

                        if data.len() > display_count {
                            println!("   ... and {} more data points", data.len() - display_count);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to query historical data: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("stats", _)) => {
            println!("📊 Historical Data Storage Statistics:");
            println!("{}", "=".repeat(50));

            if let Some(stats) = client.get_historical_stats().await {
                println!("📈 Total Symbols: {}", stats.total_symbols);
                println!("📊 Total Data Points: {}", stats.total_points);
                println!("💾 Compressed Size: {:.2} MB", stats.compressed_size as f64 / (1024.0 * 1024.0));
                println!("📦 Uncompressed Size: {:.2} MB", stats.uncompressed_size as f64 / (1024.0 * 1024.0));
                println!("🗜️  Compression Ratio: {:.2}x", stats.compression_ratio);
                println!("🎯 Cache Hit Rate: {:.1}%", stats.cache_hit_rate * 100.0);
            } else {
                println!("❌ Historical data management not enabled");
            }
        }
        Some(("metadata", metadata_matches)) => {
            let symbol = metadata_matches.get_one::<String>("symbol").unwrap();

            println!("🏷️  Historical Data Metadata for {}:", symbol);
            println!("{}", "=".repeat(50));

            if let Some(metadata) = client.get_historical_metadata(symbol).await {
                println!("📅 Date Range: {} to {}",
                    metadata.data_range.start.format("%Y-%m-%d"),
                    metadata.data_range.end.format("%Y-%m-%d"));
                println!("📊 Total Points: {}", metadata.total_points);
                println!("🗜️  Compressed Blocks: {}", metadata.compressed_blocks);
                println!("🔄 Last Updated: {}", metadata.last_updated.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("📡 Data Sources: {}", metadata.sources.len());

                println!("\n📈 Quality Metrics:");
                println!("   📊 Completeness: {:.1}%", metadata.quality_metrics.completeness_score * 100.0);
                println!("   📈 Consistency: {:.1}%", metadata.quality_metrics.consistency_score * 100.0);
                println!("   🎯 Accuracy: {:.1}%", metadata.quality_metrics.accuracy_score * 100.0);
                println!("   🔍 Gap Percentage: {:.1}%", metadata.quality_metrics.gap_percentage * 100.0);
                println!("   ⚠️  Outlier Percentage: {:.1}%", metadata.quality_metrics.outlier_percentage * 100.0);

                println!("\n🧹 Data Processing:");
                println!("   🗑️  Duplicates Removed: {}", metadata.deduplication_stats.duplicates_removed);
                println!("   🔧 Gaps Filled: {}", metadata.gaps_filled);
            } else {
                println!("❌ No metadata found for {}", symbol);
            }
        }
        Some(("optimize", optimize_matches)) => {
            let symbol = optimize_matches.get_one::<String>("symbol").unwrap();

            println!("🚀 Optimizing historical data for RAG training: {}", symbol);
            println!("{}", "=".repeat(60));

            match client.optimize_historical_for_rag(symbol).await {
                Ok(insights) => {
                    println!("✅ Generated {} insights for RAG training:", insights.len());
                    println!();

                    for (i, insight) in insights.iter().enumerate() {
                        println!("{}. {}", i + 1, insight);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to optimize historical data: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("❌ Invalid historical subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle query subcommands
async fn handle_query_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let symbol = matches.get_one::<String>("symbol").unwrap();

    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis();

    println!("🔍 Querying {} price...", symbol);

    match client.get_price_intelligent(symbol).await {
        Ok(price_data) => {
            println!("💰 Price Result:");
            println!("   Symbol: {}", price_data.symbol);
            println!("   Price: ${:.2}", price_data.price_usd);
            if let Some(volume) = price_data.volume_24h {
                println!("   24h Volume: ${:.0}", volume);
            }
            if let Some(market_cap) = price_data.market_cap {
                println!("   Market Cap: ${:.0}", market_cap);
            }
            println!("   Source: {}", price_data.source);
            println!("   Last Updated: {}", price_data.last_updated);
        }
        Err(e) => {
            eprintln!("❌ Price query failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle analytics subcommands
async fn handle_analytics_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::fetcher::MultiApiClient;

    let client = MultiApiClient::new_with_all_apis()
        .with_analytics(); // Enable analytics with default config

    match matches.subcommand() {
        Some(("dashboard", _)) => {
            println!("📊 Analytics Dashboard");
            println!("======================");

            if let Some(dashboard) = client.get_analytics_dashboard().await {
                println!("{}", serde_json::to_string_pretty(&dashboard)?);
            } else {
                println!("❌ Analytics not enabled or no data available");
                println!("💡 Enable analytics by using: client.with_analytics()");
            }
        }
        Some(("usage", _)) => {
            println!("📈 API Usage Metrics");
            println!("====================");

            if let Some(metrics) = client.get_analytics_usage_metrics().await {
                for (provider, metric) in metrics {
                    println!("🔹 {}:", provider);
                    println!("   Total Requests: {}", metric.total_requests);
                    println!("   Successful: {} ({:.1}%)",
                        metric.successful_requests,
                        if metric.total_requests > 0 {
                            (metric.successful_requests as f64 / metric.total_requests as f64) * 100.0
                        } else { 0.0 }
                    );
                    println!("   Failed: {}", metric.failed_requests);
                    println!("   Avg Response Time: {:.2}ms",
                        metric.average_response_time.as_millis());
                    println!("   Total Cost: ${:.4}", metric.total_cost);
                    println!("   Last Updated: {}", metric.last_updated);
                    println!();
                }
            } else {
                println!("❌ No usage metrics available");
            }
        }
        Some(("performance", _)) => {
            println!("⚡ Performance Metrics");
            println!("=====================");

            if let Some(perf) = client.get_analytics_performance_metrics().await {
                println!("Overall Success Rate: {:.1}%", perf.overall_success_rate);
                println!("Average Response Time: {:.2}ms",
                    perf.average_response_time.as_millis());
                println!("Requests/Minute: {:.1}", perf.total_requests_per_minute);
                println!("Cost/Request: ${:.6}", perf.cost_per_request);
                println!("Cost/Hour: ${:.4}", perf.total_cost_per_hour);
                println!("Most Used Provider: {}", perf.most_used_provider);
                if let Some(reliable) = perf.least_reliable_provider {
                    println!("Least Reliable Provider: {}", reliable);
                }
                println!("Fastest Provider: {}", perf.fastest_provider);
                println!("Timestamp: {}", perf.timestamp);
            } else {
                println!("❌ No performance metrics available");
            }
        }
        Some(("costs", _)) => {
            println!("💰 Cost Analysis");
            println!("===============");

            if let Some(analyses) = client.get_cost_analysis().await {
                for (combination_name, analysis) in analyses {
                    println!("🔹 {}:", combination_name);
                    println!("   Total Cost: ${:.4}", analysis.total_cost);
                    println!("   Cost/Request: ${:.6}", analysis.cost_per_request);
                    println!("   Cost Efficiency: {:.4}", analysis.cost_efficiency);
                    println!("   Reliability Score: {:.2}", analysis.reliability_score);
                    println!("   Performance Score: {:.4}", analysis.performance_score);
                    println!("   Overall Score: {:.4}", analysis.overall_score);
                    println!();
                }
            } else {
                println!("❌ No cost analysis available");
            }
        }
        Some(("recommend", _)) => {
            println!("💡 Optimization Recommendations");
            println!("==============================");

            if let Some(recommendations) = client.get_optimization_recommendations().await {
                if recommendations.is_empty() {
                    println!("✅ No optimization recommendations - system performing optimally!");
                } else {
                    for (i, rec) in recommendations.iter().enumerate() {
                        println!("{}. {} - {} (Priority: {:?})",
                            i + 1,
                            match rec.recommendation_type {
                                crate::modules::analytics::RecommendationType::SwitchProvider => "🔄 Switch Provider",
                                crate::modules::analytics::RecommendationType::UseCacheMore => "💾 Use Cache More",
                                crate::modules::analytics::RecommendationType::ReduceFrequency => "⏱️  Reduce Frequency",
                                crate::modules::analytics::RecommendationType::ChangeCombination => "🔀 Change Combination",
                                crate::modules::analytics::RecommendationType::UpgradePlan => "⬆️  Upgrade Plan",
                                crate::modules::analytics::RecommendationType::ImplementCircuitBreaker => "🔌 Circuit Breaker",
                            },
                            rec.description,
                            rec.implementation_priority
                        );

                        if rec.expected_savings > 0.0 {
                            println!("   💸 Expected Savings: ${:.4}", rec.expected_savings);
                        }
                        if rec.expected_improvement > 0.0 {
                            println!("   📈 Expected Improvement: {:.1}%", rec.expected_improvement * 100.0);
                        }
                        println!("   🎯 Confidence: {:.1}%", rec.confidence_score * 100.0);
                        println!();
                    }
                }
            } else {
                println!("❌ No recommendations available");
            }
        }
        Some(("export", _)) => {
            println!("📤 Exporting Analytics Data");
            println!("==========================");

            if let Some(data) = client.export_analytics_data().await {
                println!("{}", serde_json::to_string_pretty(&data)?);
                println!("\n💡 Tip: Save this output to a file for external analysis");
            } else {
                println!("❌ No analytics data available to export");
            }
        }
        _ => {
            eprintln!("❌ Unknown analytics subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle RAG subcommands
async fn handle_rag_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::rag::RagSystem;

    // Get configuration from environment
    let typesense_url = std::env::var("TYPESENSE_URL")
        .unwrap_or_else(|_| "https://typesense.your-domain.com".to_string());
    let typesense_api_key = std::env::var("TYPESENSE_API_KEY")
        .unwrap_or_else(|_| "iora_dev_typesense_key_2024".to_string());
    let gemini_api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable is required - no fallbacks allowed")?;


    let mut rag_system = RagSystem::new(typesense_url, typesense_api_key, gemini_api_key);

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("🚀 Initializing Typesense RAG System");
            println!("====================================");

            match rag_system.init_typesense().await {
                Ok(_) => {
                    println!("\n✅ Typesense RAG system initialized successfully!");
                    println!("💡 You can now index historical data and perform searches");
                }
                Err(e) => {
                    println!("\n❌ Failed to initialize Typesense: {}", e);
                    println!("💡 Make sure Typesense is running and accessible");
                    println!("   Docker command: docker run -p 8108:8108 -v typesense-data:/data typesense/typesense:27.0");
                    std::process::exit(1);
                }
            }
        }
        Some(("status", _)) => {
            println!("📊 RAG System Status");
            println!("===================");

            println!("🔗 Typesense URL: {}", rag_system.get_typesense_url());
            println!("🔑 API Key: {}...", rag_system.get_masked_api_key());
            println!("📍 Initialized: {}", if rag_system.is_initialized() { "✅ Yes" } else { "❌ No" });

            if rag_system.is_initialized() {
                println!("\n✅ RAG system is ready for operations!");
                println!("💡 Available commands:");
                println!("   • iora rag index -f data.json    # Index historical data");
                println!("   • iora rag search -q \"bitcoin\"  # Search for relevant data");
            } else {
                println!("\n⚠️  RAG system not initialized");
                println!("💡 Run: iora rag init");
            }
        }
        Some(("index", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();

            println!("📊 Indexing Historical Data");
            println!("===========================");
            println!("📁 File: {}", file_path);

            if !rag_system.is_initialized() {
                println!("\n❌ RAG system not initialized. Run 'iora rag init' first.");
                std::process::exit(1);
            }

            // If file_path is just a filename, look in assets directory
            let actual_path = if file_path.contains('/') {
                file_path.to_string()
            } else {
                format!("./assets/{}", file_path)
            };

            match rag_system.index_historical_data(&actual_path).await {
                Ok(_) => {
                    println!("\n✅ Historical data indexed successfully!");
                    println!("💡 You can now search for relevant data using: iora rag search -q \"bitcoin price\"");
                }
                Err(e) => {
                    println!("\n❌ Failed to index data: {}", e);
                    println!("💡 Make sure:");
                    println!("   • Typesense is running: docker run -p 8108:8108 -v typesense-data:/data typesense/typesense:27.0");
                    println!("   • RAG system is initialized: iora rag init");
                    println!("   • File exists: {}", actual_path);
                    std::process::exit(1);
                }
            }
        }
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            let limit: usize = sub_matches.get_one::<String>("limit")
                .unwrap()
                .parse()
                .unwrap_or(5);

            println!("🔍 Searching Historical Data");
            println!("===========================");
            println!("🔎 Query: {}", query);
            println!("📊 Limit: {}", limit);

            if !rag_system.is_initialized() {
                println!("\n❌ RAG system not initialized. Run 'iora rag init' first.");
                std::process::exit(1);
            }

            match rag_system.search_historical_data(query, limit).await {
                Ok(results) => {
                    println!("\n📋 Search Results ({} found)", results.len());

                    if results.is_empty() {
                        println!("❌ No relevant documents found");
                    } else {
                        for (i, doc) in results.iter().enumerate() {
                            println!("\n--- Result {} ---", i + 1);
                            println!("📄 Text: {}", doc.text);
                            println!("💰 Price: ${:.2}", doc.price);
                            println!("🏷️  Symbol: {}", doc.symbol);
                            println!("📅 Timestamp: {}", doc.timestamp);
                        }
                    }
                }
                Err(e) => {
                    println!("\n❌ Search failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("augment", sub_matches)) => {
            let symbol = sub_matches.get_one::<String>("symbol").unwrap();
            let price: f64 = sub_matches.get_one::<String>("price")
                .unwrap()
                .parse()
                .unwrap_or(0.0);

            println!("🤖 Augmenting Data with Hybrid Search");
            println!("====================================");
            println!("🏷️  Symbol: {}", symbol);
            println!("💰 Price: ${:.2}", price);

            if !rag_system.is_initialized() {
                println!("\n❌ RAG system not initialized. Run 'iora rag init' first.");
                std::process::exit(1);
            }

            // Create raw data for testing
            let raw_data = super::fetcher::RawData {
                symbol: symbol.clone(),
                name: symbol.clone(),
                price_usd: price,
                volume_24h: Some(1000000.0),
                market_cap: Some(10000000.0),
                price_change_24h: Some(5.0),
                last_updated: chrono::Utc::now(),
                source: super::fetcher::ApiProvider::CoinGecko,
            };

            match rag_system.augment_data(raw_data).await {
                Ok(augmented) => {
                    println!("\n📊 Augmented Data Results");
                    println!("=========================");
                    println!("🔗 Context ({})", augmented.context.len());

                    for context in &augmented.context {
                        println!("  {}", context);
                    }

                    println!("\n🔍 Embedding: {} dimensions", augmented.embedding.len());
                    println!("✅ Hybrid search completed successfully!");
                }
                Err(e) => {
                    println!("\n❌ Augmentation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("benchmark", sub_matches)) => {
            let data_file = sub_matches.get_one::<String>("data_file");

            println!("🚀 I.O.R.A. RAG Performance Benchmark Suite");
            println!("===========================================");
            println!("Task 3.2.2: Performance Optimization and Benchmarking");
            println!();

            if let Some(file_path) = data_file {
                println!("📁 Using data file: {}", file_path);
            } else {
                println!("📁 Using synthetic test data (no data file specified)");
            }

            println!("\n⚠️  Note: This requires GEMINI_API_KEY and Typesense to be running");
            println!("💡 Make sure environment variables are configured properly");
            println!();

            match rag_system.run_cli_benchmarks(data_file.map(|x| x.as_str())).await {
                Ok(_) => {
                    println!("\n✅ Performance benchmarking completed successfully!");
                    println!("📄 Results exported to: benchmark_results.json");
                }
                Err(e) => {
                    println!("\n❌ Benchmark execution failed: {}", e);
                    println!("💡 Make sure:");
                    println!("   • GEMINI_API_KEY is set in environment variables");
                    println!("   • Typesense is running: docker run -p 8108:8108 -v typesense-data:/data typesense/typesense:27.0");
                    println!("   • RAG system is initialized: iora rag init");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("❌ Unknown RAG subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle load testing subcommands
async fn handle_load_test_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;
    use crate::modules::load_testing::{LoadTestingEngine, LoadTestConfig, ConcurrentUserConfig, DataVolumeConfig, ResourceStressConfig, OperationType};

    // Initialize components
    let config = crate::modules::config::get_config().map_err(|e| format!("Failed to load config: {}", e))?;
    let api_client = Arc::new(crate::modules::fetcher::MultiApiClient::new());
    let cache = Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default()));
    let processor = Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default()));

    // Initialize RAG system if available
    let rag_system = Some(Arc::new(crate::modules::rag::RagSystem::new(
        config.typesense_url().to_string(),
        config.typesense_api_key().to_string(),
        std::env::var("GEMINI_API_KEY").unwrap_or_default(),
    )));

    let load_test_config = LoadTestConfig {
        concurrent_users: 10,
        test_duration_seconds: 60,
        request_rate_per_second: 100,
        data_volume_multiplier: 1,
        memory_limit_mb: None,
        enable_resource_monitoring: true,
        enable_performance_profiling: true,
    };

    let engine = LoadTestingEngine::new(
        api_client,
        cache,
        processor,
        rag_system,
        load_test_config.clone(),
    );

    match matches.subcommand() {
        Some(("concurrent-users", sub_matches)) => {
            let users: usize = sub_matches.get_one::<String>("users")
                .unwrap()
                .parse()
                .unwrap_or(10);
            let duration: u64 = sub_matches.get_one::<String>("duration")
                .unwrap()
                .parse()
                .unwrap_or(60);
            let operations: usize = sub_matches.get_one::<String>("operations")
                .unwrap()
                .parse()
                .unwrap_or(50);

            let scenario = ConcurrentUserConfig {
                user_count: users,
                operations_per_user: operations,
                operation_types: vec![
                    OperationType::PriceFetch("BTC".to_string()),
                    OperationType::PriceFetch("ETH".to_string()),
                    OperationType::PriceFetch("ADA".to_string()),
                    OperationType::CacheOperation,
                    OperationType::AnalyticsOperation,
                ],
            };

            let mut config = load_test_config.clone();
            config.test_duration_seconds = duration;

            let engine = LoadTestingEngine::new(
                Arc::new(crate::modules::fetcher::MultiApiClient::new()),
                Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default())),
                Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default())),
                None,
                config,
            );

            let results = engine.run_concurrent_user_test(scenario).await?;
            engine.export_results_to_json(&results, "concurrent_users_results.json").await?;
        }

        Some(("data-volume", sub_matches)) => {
            let size_mb: usize = sub_matches.get_one::<String>("size")
                .unwrap()
                .parse()
                .unwrap_or(100);
            let batch_size: usize = sub_matches.get_one::<String>("batch")
                .unwrap()
                .parse()
                .unwrap_or(1000);

            let scenario = DataVolumeConfig {
                data_size_mb: size_mb,
                batch_size,
                indexing_operations: true,
                search_operations: true,
            };

            let results = engine.run_data_volume_test(scenario).await?;
            engine.export_results_to_json(&results, "data_volume_results.json").await?;
        }

        Some(("resource-stress", sub_matches)) => {
            let duration: u64 = sub_matches.get_one::<String>("duration")
                .unwrap()
                .parse()
                .unwrap_or(30);

            let scenario = ResourceStressConfig {
                memory_pressure: sub_matches.get_flag("memory"),
                cpu_pressure: sub_matches.get_flag("cpu"),
                io_pressure: sub_matches.get_flag("io"),
                network_pressure: sub_matches.get_flag("network"),
            };

            let mut config = load_test_config.clone();
            config.test_duration_seconds = duration;

            let engine = LoadTestingEngine::new(
                Arc::new(crate::modules::fetcher::MultiApiClient::new()),
                Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default())),
                Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default())),
                None,
                config,
            );

            let results = engine.run_resource_stress_test(scenario).await?;
            engine.export_results_to_json(&results, "resource_stress_results.json").await?;
        }

        Some(("mixed-workload", sub_matches)) => {
            let users: usize = sub_matches.get_one::<String>("users")
                .unwrap()
                .parse()
                .unwrap_or(20);
            let duration: u64 = sub_matches.get_one::<String>("duration")
                .unwrap()
                .parse()
                .unwrap_or(120);

            println!("🔄 Starting Mixed Workload Test");
            println!("================================");
            println!("👥 Users: {}", users);
            println!("⏱️  Duration: {} seconds", duration);
            println!();

            // Run concurrent user test with mixed operations
            let scenario = ConcurrentUserConfig {
                user_count: users,
                operations_per_user: 100,
                operation_types: vec![
                    OperationType::PriceFetch("BTC".to_string()),
                    OperationType::HistoricalDataFetch("BTC".to_string()),
                    OperationType::SearchQuery("bitcoin price trends".to_string()),
                    OperationType::CacheOperation,
                    OperationType::AnalyticsOperation,
                ],
            };

            let mut config = load_test_config.clone();
            config.test_duration_seconds = duration;

            let engine = LoadTestingEngine::new(
                Arc::new(crate::modules::fetcher::MultiApiClient::new()),
                Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default())),
                Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default())),
                None,
                config,
            );

            let results = engine.run_concurrent_user_test(scenario).await?;
            engine.export_results_to_json(&results, "mixed_workload_results.json").await?;
        }

        Some(("full-suite", sub_matches)) => {
            let output_file = sub_matches.get_one::<String>("output").unwrap();

            println!("🚀 Starting Complete Load Testing Suite");
            println!("======================================");
            println!("📁 Output file: {}", output_file);
            println!();

            let mut all_results = Vec::new();

            // 1. Concurrent Users Test
            println!("📊 Running Concurrent Users Test...");
            let concurrent_scenario = ConcurrentUserConfig {
                user_count: 5,
                operations_per_user: 25,
                operation_types: vec![
                    OperationType::PriceFetch("BTC".to_string()),
                    OperationType::CacheOperation,
                ],
            };

            let mut config = load_test_config.clone();
            config.test_duration_seconds = 30;

            let engine = LoadTestingEngine::new(
                Arc::new(crate::modules::fetcher::MultiApiClient::new()),
                Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default())),
                Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default())),
                None,
                config,
            );

            match engine.run_concurrent_user_test(concurrent_scenario).await {
                Ok(results) => {
                    all_results.push(results);
                    println!("✅ Concurrent Users Test completed");
                }
                Err(e) => println!("❌ Concurrent Users Test failed: {}", e),
            }

            // 2. Data Volume Test
            println!("\n📊 Running Data Volume Test...");
            let data_scenario = DataVolumeConfig {
                data_size_mb: 50,
                batch_size: 500,
                indexing_operations: true,
                search_operations: false,
            };

            match engine.run_data_volume_test(data_scenario).await {
                Ok(results) => {
                    all_results.push(results);
                    println!("✅ Data Volume Test completed");
                }
                Err(e) => println!("❌ Data Volume Test failed: {}", e),
            }

            // 3. Resource Stress Test
            println!("\n📊 Running Resource Stress Test...");
            let stress_scenario = ResourceStressConfig {
                memory_pressure: true,
                cpu_pressure: true,
                io_pressure: false,
                network_pressure: false,
            };

            let mut stress_config = load_test_config.clone();
            stress_config.test_duration_seconds = 15;

            let stress_engine = LoadTestingEngine::new(
                Arc::new(crate::modules::fetcher::MultiApiClient::new()),
                Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default())),
                Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default())),
                None,
                stress_config,
            );

            match stress_engine.run_resource_stress_test(stress_scenario).await {
                Ok(results) => {
                    all_results.push(results);
                    println!("✅ Resource Stress Test completed");
                }
                Err(e) => println!("❌ Resource Stress Test failed: {}", e),
            }

            // Export all results
            let summary = serde_json::json!({
                "test_suite": "full_load_test_suite",
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "total_tests": all_results.len(),
                "results": all_results
            });

            tokio::fs::write(output_file, serde_json::to_string_pretty(&summary)?).await?;
            println!("\n✅ Complete Load Testing Suite Finished");
            println!("📄 Results exported to: {}", output_file);
            println!("📊 Tests completed: {}/3", all_results.len());
        }

        _ => {
            eprintln!("❌ Unknown load testing subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Handle resilience testing subcommands
async fn handle_resilience_test_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::modules::resilience::{ResilienceTestingEngine, ResilienceTestConfig};
    use std::sync::Arc;

    // Initialize components
    let config = crate::modules::config::AppConfig::from_env()?;
    let api_client = Arc::new(crate::modules::fetcher::MultiApiClient::new());
    let cache = Arc::new(crate::modules::cache::IntelligentCache::new(crate::modules::cache::CacheConfig::default()));
    let processor = Arc::new(crate::modules::processor::DataProcessor::new_with_default_client(crate::modules::processor::ProcessingConfig::default()));
    let historical_manager = Arc::new(crate::modules::historical::HistoricalDataManager::new(crate::modules::historical::TimeSeriesConfig::default()));

    // Initialize RAG system if available
    let rag_system = Some(Arc::new(crate::modules::rag::RagSystem::new(
        config.typesense_url().to_string(),
        config.typesense_api_key().to_string(),
        config.gemini_api_key().to_string(),
    )));

    match matches.subcommand() {
        Some(("api-failures", sub_matches)) => {
            let duration = sub_matches.get_one::<String>("duration")
                .unwrap().parse::<u64>().unwrap_or(30);
            let circuit_breaker = sub_matches.get_flag("circuit-breaker");

            println!("🛡️  API Failure Resilience Test");
            println!("=================================");
            println!("⏱️  Duration: {} seconds", duration);
            println!("🔌 Circuit Breaker: {}", if circuit_breaker { "Enabled" } else { "Disabled" });

            let test_config = ResilienceTestConfig {
                test_duration_seconds: duration,
                failure_injection_enabled: true,
                circuit_breaker_enabled: circuit_breaker,
                retry_attempts: 3,
                timeout_duration_seconds: 5,
                recovery_delay_ms: 1000,
            };

            let engine = ResilienceTestingEngine::new(
                api_client,
                cache,
                processor,
                rag_system,
                historical_manager,
                test_config,
            );

            // Add overall timeout to prevent hanging
            let test_result = tokio::time::timeout(
                Duration::from_secs(duration + 30), // Add 30s buffer
                engine.run_comprehensive_resilience_test()
            ).await;

            let results = match test_result {
                Ok(result) => result?,
                Err(_) => {
                    println!("⏰ Test timed out after {} seconds", duration + 30);
                    return Ok(());
                }
            };

            engine.export_results_to_json(&results, "api_failures_results.json").await?;

            println!("\n✅ API Failure Test Completed");
            println!("📊 Results: {} total, {} successful, {} failed",
                    results.total_operations,
                    results.successful_operations,
                    results.failed_operations);
            println!("📄 Results exported to: api_failures_results.json");
        }

        Some(("network-failures", sub_matches)) => {
            let duration = sub_matches.get_one::<String>("duration")
                .unwrap().parse::<u64>().unwrap_or(30);

            println!("🌐 Network Failure Resilience Test");
            println!("===================================");
            println!("⏱️  Duration: {} seconds", duration);

            let test_config = ResilienceTestConfig {
                test_duration_seconds: duration,
                failure_injection_enabled: true,
                circuit_breaker_enabled: true,
                retry_attempts: 2,
                timeout_duration_seconds: 3,
                recovery_delay_ms: 2000,
            };

            let engine = ResilienceTestingEngine::new(
                api_client,
                cache,
                processor,
                rag_system,
                historical_manager,
                test_config,
            );

            // Add overall timeout to prevent hanging
            let test_result = tokio::time::timeout(
                Duration::from_secs(duration + 30), // Add 30s buffer
                engine.run_comprehensive_resilience_test()
            ).await;

            let results = match test_result {
                Ok(result) => result?,
                Err(_) => {
                    println!("⏰ Test timed out after {} seconds", duration + 30);
                    return Ok(());
                }
            };

            engine.export_results_to_json(&results, "network_failures_results.json").await?;

            println!("\n✅ Network Failure Test Completed");
            println!("📊 Results: {} total, {} successful, {} failed",
                    results.total_operations,
                    results.successful_operations,
                    results.failed_operations);
            println!("📄 Results exported to: network_failures_results.json");
        }

        Some(("recovery-test", sub_matches)) => {
            let duration = sub_matches.get_one::<String>("duration")
                .unwrap().parse::<u64>().unwrap_or(60);
            let failure_rate = sub_matches.get_one::<String>("failure-rate")
                .unwrap().parse::<f64>().unwrap_or(0.3);

            println!("🔄 Recovery Resilience Test");
            println!("===========================");
            println!("⏱️  Duration: {} seconds", duration);
            println!("⚠️  Simulated Failure Rate: {:.1}%", failure_rate * 100.0);

            let test_config = ResilienceTestConfig {
                test_duration_seconds: duration,
                failure_injection_enabled: true,
                circuit_breaker_enabled: true,
                retry_attempts: 3,
                timeout_duration_seconds: 10,
                recovery_delay_ms: 500,
            };

            let engine = ResilienceTestingEngine::new(
                api_client,
                cache,
                processor,
                rag_system,
                historical_manager,
                test_config,
            );

            // Add overall timeout to prevent hanging
            let test_result = tokio::time::timeout(
                Duration::from_secs(duration + 30), // Add 30s buffer
                engine.run_comprehensive_resilience_test()
            ).await;

            let results = match test_result {
                Ok(result) => result?,
                Err(_) => {
                    println!("⏰ Test timed out after {} seconds", duration + 30);
                    return Ok(());
                }
            };

            engine.export_results_to_json(&results, "recovery_test_results.json").await?;

            println!("\n✅ Recovery Test Completed");
            println!("📊 Results: {} total, {} successful, {} failed",
                    results.total_operations,
                    results.successful_operations,
                    results.failed_operations);
            println!("📄 Results exported to: recovery_test_results.json");
        }

        Some(("comprehensive", sub_matches)) => {
            let duration = sub_matches.get_one::<String>("duration")
                .unwrap().parse::<u64>().unwrap_or(120);
            let output_file = sub_matches.get_one::<String>("output").unwrap();

            println!("🛡️  Comprehensive Resilience Test Suite");
            println!("=======================================");
            println!("⏱️  Duration: {} seconds", duration);
            println!("📄 Output: {}", output_file);

            let test_config = ResilienceTestConfig {
                test_duration_seconds: duration,
                failure_injection_enabled: true,
                circuit_breaker_enabled: true,
                retry_attempts: 3,
                timeout_duration_seconds: 5,
                recovery_delay_ms: 1000,
            };

            let engine = ResilienceTestingEngine::new(
                api_client,
                cache,
                processor,
                rag_system,
                historical_manager,
                test_config,
            );

            // Add overall timeout to prevent hanging
            let test_result = tokio::time::timeout(
                Duration::from_secs(duration + 60), // Add 60s buffer for comprehensive test
                engine.run_comprehensive_resilience_test()
            ).await;

            let results = match test_result {
                Ok(result) => result?,
                Err(_) => {
                    println!("⏰ Comprehensive test timed out after {} seconds", duration + 60);
                    return Ok(());
                }
            };

            engine.export_results_to_json(&results, output_file).await?;

            println!("\n✅ Comprehensive Resilience Test Suite Completed");
            println!("📊 Total Operations: {}", results.total_operations);
            println!("✅ Successful: {} ({:.1}%)",
                    results.successful_operations,
                    (results.successful_operations as f64 / results.total_operations as f64) * 100.0);
            println!("❌ Failed: {} ({:.1}%)",
                    results.failed_operations,
                    (results.failed_operations as f64 / results.total_operations as f64) * 100.0);
            println!("⏱️  Circuit Breaker Trips: {}", results.circuit_breaker_trips);
            println!("📄 Results exported to: {}", output_file);
        }

        _ => {
            eprintln!("❌ Unknown resilience testing subcommand");
            std::process::exit(1);
        }
    }

    Ok(())
}
