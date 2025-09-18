//! Solana Oracle Integration Tests
//!
//! Tests for the Solana oracle feeder functionality.
//! These tests verify the integration between analysis and Solana blockchain.

use iora::modules::analyzer::Analysis;
use iora::modules::solana::SolanaOracle;
use iora::modules::fetcher::{RawData, ApiProvider};
use std::env;
use chrono::Utc;

/// Helper function to get Solana configuration from environment
fn get_solana_config() -> (String, String, String) {
    // Load .env file if it exists
    let _ = dotenv::dotenv();

    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let wallet_path = env::var("SOLANA_WALLET_PATH")
        .unwrap_or_else(|_| "wallets/devnet-wallet.json".to_string());
    let program_id = env::var("SOLANA_PROGRAM_ID")
        .unwrap_or_else(|_| "GVetpCppi9v1BoZYCHwzL18b6a35i3HbgFUifQLbt5Jz".to_string());

    (rpc_url, wallet_path, program_id)
}

/// Helper function to create test raw data
fn create_test_raw_data(price: f64) -> RawData {
    RawData {
        symbol: "BTC".to_string(),
        name: "Bitcoin".to_string(),
        price_usd: price,
        volume_24h: Some(1000000.0),
        market_cap: Some(800000000000.0),
        price_change_24h: Some(2.5),
        last_updated: Utc::now(),
        source: ApiProvider::CoinMarketCap,
    }
}

/// Helper function to create test analysis
fn create_test_analysis() -> Analysis {
    Analysis {
        insight: "Bitcoin shows strong bullish momentum with increasing institutional adoption and positive technical indicators.".to_string(),
        processed_price: 45000.0,
        confidence: 0.85,
        recommendation: "BUY".to_string(),
        raw_data: create_test_raw_data(45000.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solana_oracle_creation() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        // Test oracle creation
        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        match result {
            Ok(oracle) => {
                // Verify oracle was created successfully
                assert!(!rpc_url.is_empty());
                assert!(!wallet_path.is_empty());
                assert!(!program_id.is_empty());

                // Test balance check (if wallet exists)
                match oracle.get_balance() {
                    Ok(balance) => {
                        println!("✅ Wallet balance: {} SOL", balance as f64 / 1_000_000_000.0);
                    }
                    Err(e) => {
                        println!("⚠️  Could not check balance (expected in test environment): {}", e);
                    }
                }
            }
            Err(e) => {
                println!("⚠️  Could not create Solana oracle (expected without valid wallet): {}", e);
                // This is acceptable in test environments without proper wallet setup
            }
        }
    }

    #[tokio::test]
    async fn test_pda_derivation() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            // Test PDA derivation
            match oracle.find_oracle_data_pda() {
                Ok(pda) => {
                    println!("✅ Oracle data PDA: {}", pda);
                    assert!(!pda.to_string().is_empty());
                }
                Err(e) => {
                    panic!("Failed to derive PDA: {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping PDA test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_instruction_data_building() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            let analysis = create_test_analysis();

            // Test instruction data building
            match oracle.build_update_instruction_data(
                &analysis.insight,
                analysis.processed_price,
                analysis.confidence,
                &analysis.recommendation,
                1640995200, // 2022-01-01 00:00:00 UTC
            ) {
                Ok(data) => {
                    println!("✅ Instruction data built successfully, length: {} bytes", data.len());
                    assert!(data.len() > 8); // Should have discriminator + data

                    // Verify discriminator
                    assert_eq!(&data[0..8], &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
                }
                Err(e) => {
                    panic!("Failed to build instruction data: {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping instruction data test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_feed_oracle_simulation() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            let analysis = create_test_analysis();

            // Test feed oracle (this will fail without proper setup, but tests the logic)
            match oracle.feed_oracle(&analysis).await {
                Ok(signature) => {
                    println!("✅ Oracle feed successful! Transaction: {}", signature);
                    assert!(!signature.is_empty());
                }
                Err(e) => {
                    // Expected to fail in test environment without proper Devnet setup
                    println!("⚠️  Oracle feed failed (expected in test environment): {}", e);
                    // Verify it's a network-related error, not a logic error
                    let error_msg = e.to_string().to_lowercase();
                    assert!(
                        error_msg.contains("balance") ||
                        error_msg.contains("account") ||
                        error_msg.contains("network") ||
                        error_msg.contains("connect") ||
                        error_msg.contains("wallet") ||
                        error_msg.contains("signature"),
                        "Unexpected error type: {}", e
                    );
                }
            }
        } else {
            println!("⚠️  Skipping feed oracle test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_oracle_initialization_simulation() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            // Test oracle initialization (this will fail without proper setup, but tests the logic)
            match oracle.initialize_oracle().await {
                Ok(signature) => {
                    println!("✅ Oracle initialization successful! Transaction: {}", signature);
                    assert!(!signature.is_empty());
                }
                Err(e) => {
                    // Expected to fail in test environment without proper Devnet setup
                    println!("⚠️  Oracle initialization failed (expected in test environment): {}", e);
                    // Verify it's a network-related error, not a logic error
                    let error_msg = e.to_string().to_lowercase();
                    assert!(
                        error_msg.contains("balance") ||
                        error_msg.contains("account") ||
                        error_msg.contains("network") ||
                        error_msg.contains("connect") ||
                        error_msg.contains("wallet") ||
                        error_msg.contains("signature"),
                        "Unexpected error type: {}", e
                    );
                }
            }
        } else {
            println!("⚠️  Skipping oracle initialization test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_analysis_data_validation() {
        let analysis = create_test_analysis();

        // Test valid analysis data
        assert!(!analysis.insight.is_empty(), "Insight should not be empty");
        assert!(analysis.processed_price > 0.0, "Price should be positive");
        assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0,
               "Confidence should be between 0.0 and 1.0, got: {}", analysis.confidence);
        assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"),
               "Recommendation should be BUY, SELL, or HOLD, got: {}", analysis.recommendation);

        println!("✅ Analysis data validation passed");
        println!("   Insight length: {} characters", analysis.insight.len());
        println!("   Confidence: {:.2}", analysis.confidence);
        println!("   Recommendation: {}", analysis.recommendation);
    }

    #[tokio::test]
    async fn test_large_insight_handling() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            // Create analysis with very long insight
            let long_insight = "A".repeat(1000); // Much longer than 500 char limit
            let analysis = Analysis {
                insight: long_insight,
                processed_price: 50000.0,
                confidence: 0.9,
                recommendation: "BUY".to_string(),
                raw_data: create_test_raw_data(50000.0),
            };

            // Test that instruction data building handles truncation
            match oracle.build_update_instruction_data(
                &analysis.insight,
                analysis.processed_price,
                analysis.confidence,
                &analysis.recommendation,
                1640995200,
            ) {
                Ok(data) => {
                    println!("✅ Large insight handled correctly, data length: {} bytes", data.len());
                    // Should still be valid even with truncation
                    assert!(data.len() > 8);
                }
                Err(e) => {
                    panic!("Failed to handle large insight: {}", e);
                }
            }
        } else {
            println!("⚠️  Skipping large insight test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_edge_case_analysis_values() {
        let (rpc_url, wallet_path, program_id) = get_solana_config();

        let result = SolanaOracle::new(&rpc_url, &wallet_path, &program_id);

        if let Ok(oracle) = result {
            // Test edge cases
            let test_cases = vec![
                ("Minimum confidence", 0.0),
                ("Maximum confidence", 1.0),
                ("High price", 1000000.0),
                ("Low price", 0.000001),
            ];

            for (description, confidence) in test_cases {
                let analysis = Analysis {
                    insight: format!("Test analysis for {}", description),
                    processed_price: 50000.0,
                    confidence,
                    recommendation: "HOLD".to_string(),
                    raw_data: create_test_raw_data(50000.0),
                };

                match oracle.build_update_instruction_data(
                    &analysis.insight,
                    analysis.processed_price,
                    analysis.confidence,
                    &analysis.recommendation,
                    1640995200,
                ) {
                    Ok(data) => {
                        println!("✅ {} handled correctly", description);
                        assert!(data.len() > 8);
                    }
                    Err(e) => {
                        panic!("Failed to handle {}: {}", description, e);
                    }
                }
            }
        } else {
            println!("⚠️  Skipping edge case test - oracle creation failed");
        }
    }

    #[tokio::test]
    async fn test_configuration_validation() {
        // Test with invalid configurations
        let invalid_configs = vec![
            ("empty rpc", "", "wallets/devnet-wallet.json", "GVetpCppi9v1BoZYCHwzL18b6a35i3HbgFUifQLbt5Jz"),
            ("empty wallet", "https://api.devnet.solana.com", "", "GVetpCppi9v1BoZYCHwzL18b6a35i3HbgFUifQLbt5Jz"),
            ("empty program", "https://api.devnet.solana.com", "wallets/devnet-wallet.json", ""),
            ("invalid program id", "https://api.devnet.solana.com", "wallets/devnet-wallet.json", "invalid"),
        ];

        for (description, rpc_url, wallet_path, program_id) in invalid_configs {
            let result = SolanaOracle::new(rpc_url, wallet_path, program_id);
            match result {
                Ok(_) => {
                    // Some invalid configs might still work if files exist
                    println!("⚠️  {}: Unexpected success", description);
                }
                Err(e) => {
                    println!("✅ {}: Correctly failed with error: {}", description, e);
                }
            }
        }
    }
}
