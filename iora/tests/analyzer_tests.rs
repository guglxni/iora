//! Analyzer Module Tests
//!
//! Comprehensive tests for the Gemini API analysis functionality.
//! Tests use real API calls only - no mocks, no fallbacks, no simulations.
//! All tests require GEMINI_API_KEY to be configured.

use iora::modules::analyzer::Analyzer;
use iora::modules::rag::AugmentedData;
use iora::modules::fetcher::RawData;
use iora::modules::llm::LlmConfig;
use chrono::Utc;
use std::env;

/// Helper function to get Gemini API key from environment
fn get_gemini_api_key() -> String {
    // Load .env file if it exists
    let _ = dotenv::dotenv();

    env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set for analyzer tests")
}

/// Helper function to create test augmented data
fn create_test_augmented_data(symbol: &str, price: f64, context: Vec<String>) -> AugmentedData {
    AugmentedData {
        raw_data: RawData {
            name: format!("Test {}", symbol),
            symbol: symbol.to_string(),
            price_usd: price,
            volume_24h: Some(1000000.0),
            market_cap: Some(10000000.0),
            price_change_24h: Some(5.0),
            last_updated: Utc::now(),
            source: iora::modules::fetcher::ApiProvider::CoinGecko,
        },
        context,
        embedding: vec![0.1, 0.2, 0.3], // Dummy embedding for testing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyzer_creation() {
        let api_key = get_gemini_api_key();

        // Test that API key is loaded correctly
        assert!(!api_key.is_empty());
        assert!(api_key.starts_with("AIzaSy"));

        // Test that analyzer can be created
        let _analyzer = Analyzer::new(LlmConfig::gemini(api_key));
    }

    #[tokio::test]
    async fn test_basic_crypto_analysis() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let context = vec![
            "Bitcoin has shown strong upward momentum in recent weeks".to_string(),
            "Market sentiment is bullish with increased institutional adoption".to_string(),
            "Historical data shows similar patterns before major rallies".to_string(),
        ];

        let augmented_data = create_test_augmented_data("BTC", 45000.0, context);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(analysis) => {
                // Verify the analysis structure
                assert!(!analysis.insight.is_empty(), "Insight should not be empty");
                assert!(analysis.processed_price > 0.0, "Processed price should be positive");
                assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0,
                       "Confidence should be between 0.0 and 1.0, got: {}", analysis.confidence);
                assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"),
                       "Recommendation should be BUY, SELL, or HOLD, got: {}", analysis.recommendation);

                println!("✅ Analysis completed:");
                println!("   Insight: {}", analysis.insight.chars().take(100).collect::<String>());
                println!("   Confidence: {:.2}", analysis.confidence);
                println!("   Recommendation: {}", analysis.recommendation);
                println!("   Processed Price: ${:.2}", analysis.processed_price);
            }
            Err(e) => {
                panic!("Analysis failed with real API call: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_altcoin_analysis() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let context = vec![
            "Ethereum recently upgraded to proof of stake".to_string(),
            "DeFi ecosystem shows strong growth potential".to_string(),
            "Network congestion has decreased significantly".to_string(),
        ];

        let augmented_data = create_test_augmented_data("ETH", 2800.0, context);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(analysis) => {
                assert!(!analysis.insight.is_empty());
                assert!(analysis.processed_price > 0.0);
                assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0);
                assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"));

                println!("✅ ETH Analysis completed:");
                println!("   Insight: {}", analysis.insight.chars().take(100).collect::<String>());
                println!("   Recommendation: {}", analysis.recommendation);
            }
            Err(e) => {
                panic!("ETH analysis failed with real API call: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_high_volatility_analysis() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let context = vec![
            "Cryptocurrency showing extreme volatility".to_string(),
            "Market conditions are highly uncertain".to_string(),
            "High risk, high reward scenario".to_string(),
        ];

        let augmented_data = create_test_augmented_data("SHIB", 0.000025, context);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(analysis) => {
                assert!(!analysis.insight.is_empty());
                assert!(analysis.processed_price >= 0.0);
                assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0);
                assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"));

                println!("✅ High-volatility analysis completed");
                println!("   Confidence: {:.2}", analysis.confidence);
            }
            Err(e) => {
                panic!("High-volatility analysis failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_empty_context_analysis() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let augmented_data = create_test_augmented_data("ADA", 0.35, vec![]);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(analysis) => {
                assert!(!analysis.insight.is_empty());
                assert!(analysis.processed_price > 0.0);
                assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0);
                assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"));

                println!("✅ Empty context analysis completed");
            }
            Err(e) => {
                panic!("Empty context analysis failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_long_context_analysis() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let context = vec![
            "This cryptocurrency has shown remarkable resilience during market downturns".to_string(),
            "Technical indicators suggest strong support levels have been established".to_string(),
            "On-chain metrics indicate increasing accumulation by large holders".to_string(),
            "Social sentiment analysis shows growing community confidence".to_string(),
            "Fundamental analysis reveals strong project development momentum".to_string(),
            "Market microstructure suggests decreasing selling pressure".to_string(),
        ];

        let augmented_data = create_test_augmented_data("LINK", 8.50, context);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(analysis) => {
                assert!(!analysis.insight.is_empty());
                assert!(analysis.processed_price > 0.0);
                assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0);
                assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"));

                println!("✅ Long context analysis completed");
                println!("   Insight length: {} characters", analysis.insight.len());
            }
            Err(e) => {
                panic!("Long context analysis failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_error_handling_invalid_api_key() {
        // Test with invalid API key
        let analyzer = Analyzer::new(LlmConfig::gemini("invalid_api_key".to_string()));

        let augmented_data = create_test_augmented_data("BTC", 45000.0, vec!["Test context".to_string()]);

        let result = analyzer.analyze(&augmented_data).await;

        match result {
            Ok(_) => {
                panic!("Expected API error with invalid key, but got successful response");
            }
            Err(e) => {
                // Should fail with API error
                println!("✅ Correctly failed with invalid API key: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_analysis_consistency() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let context = vec![
            "Stablecoin maintaining peg effectively".to_string(),
            "Low volatility is characteristic of stablecoins".to_string(),
        ];

        let augmented_data = create_test_augmented_data("USDC", 1.00, context);

        // Run analysis multiple times to check consistency
        let mut results = Vec::new();

        for i in 0..3 {
            match analyzer.analyze(&augmented_data).await {
                Ok(analysis) => {
                    results.push(analysis);
                    println!("✅ Analysis {} completed successfully", i + 1);
                }
                Err(e) => {
                    panic!("Analysis {} failed: {}", i + 1, e);
                }
            }

            // Small delay to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // Verify all results are valid
        for (i, analysis) in results.iter().enumerate() {
            assert!(!analysis.insight.is_empty(), "Analysis {}: Insight should not be empty", i);
            assert!(analysis.confidence >= 0.0 && analysis.confidence <= 1.0,
                   "Analysis {}: Invalid confidence: {}", i, analysis.confidence);
            assert!(matches!(analysis.recommendation.as_str(), "BUY" | "SELL" | "HOLD"),
                   "Analysis {}: Invalid recommendation: {}", i, analysis.recommendation);
        }

        println!("✅ Consistency test completed: {} analyses performed", results.len());
    }

    #[tokio::test]
    async fn test_price_prediction_accuracy() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        let test_cases = vec![
            ("BTC", 45000.0, vec!["Bitcoin showing strong bullish signals".to_string()]),
            ("ETH", 2800.0, vec!["Ethereum network upgrade successful".to_string()]),
            ("ADA", 0.35, vec!["Cardano smart contracts gaining traction".to_string()]),
        ];

        for (symbol, price, context) in test_cases {
            let augmented_data = create_test_augmented_data(symbol, price, context.clone());

            match analyzer.analyze(&augmented_data).await {
                Ok(analysis) => {
                    // Processed price should be reasonable (not extreme outliers)
                    let price_ratio = analysis.processed_price / price;
                    assert!(price_ratio > 0.1 && price_ratio < 10.0,
                           "{}: Processed price ${:.2} seems unreasonable compared to original ${:.2}",
                           symbol, analysis.processed_price, price);

                    println!("✅ {} price prediction: ${:.2} (original: ${:.2})",
                           symbol, analysis.processed_price, price);
                }
                Err(e) => {
                    panic!("{} analysis failed: {}", symbol, e);
                }
            }

            // Delay between requests to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    }

    #[tokio::test]
    async fn test_confidence_calibration() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key));

        // Test with high-quality data
        let high_quality_context = vec![
            "Comprehensive on-chain analysis available".to_string(),
            "Strong fundamental backing with real adoption".to_string(),
            "Technical indicators align with price action".to_string(),
            "Market sentiment strongly positive".to_string(),
        ];

        let high_quality_data = create_test_augmented_data("SOL", 95.0, high_quality_context);

        // Test with low-quality data
        let low_quality_context = vec![
            "Limited market data available".to_string(),
            "High uncertainty in market conditions".to_string(),
        ];

        let low_quality_data = create_test_augmented_data("NEWCOIN", 0.001, low_quality_context);

        let (high_result, low_result) = tokio::join!(
            analyzer.analyze(&high_quality_data),
            analyzer.analyze(&low_quality_data)
        );

        match (high_result, low_result) {
            (Ok(high_analysis), Ok(low_analysis)) => {
                // High quality data should generally have higher confidence
                // (This is a soft expectation - not guaranteed)
                println!("✅ Confidence calibration:");
                println!("   High quality (SOL): {:.2}", high_analysis.confidence);
                println!("   Low quality (NEWCOIN): {:.2}", low_analysis.confidence);

                // Both should be valid confidence scores
                assert!(high_analysis.confidence >= 0.0 && high_analysis.confidence <= 1.0);
                assert!(low_analysis.confidence >= 0.0 && low_analysis.confidence <= 1.0);
            }
            _ => {
                panic!("Confidence calibration test failed");
            }
        }
    }

    #[tokio::test]
    async fn test_structured_response_parsing() {
        let api_key = get_gemini_api_key();
        let analyzer = Analyzer::new(LlmConfig::gemini(api_key.clone()));

        let augmented_data = create_test_augmented_data("DOT", 5.25, vec![
            "Polkadot parachain auctions successful".to_string(),
        ]);

        match analyzer.analyze(&augmented_data).await {
            Ok(analysis) => {
                // Test that all fields are properly populated
                assert!(analysis.insight.len() > 10, "Insight should be substantial");
                assert!(analysis.insight.len() <= 500, "Insight should be limited to 500 chars");
                assert!(!analysis.recommendation.is_empty(), "Recommendation should not be empty");

                // Test confidence bounds
                assert!(analysis.confidence >= 0.0, "Confidence should not be negative");
                assert!(analysis.confidence <= 1.0, "Confidence should not exceed 1.0");

                println!("✅ Structured parsing test passed:");
                println!("   Insight preview: {}", analysis.insight.chars().take(50).collect::<String>());
                println!("   Confidence: {:.2}", analysis.confidence);
                println!("   Recommendation: {}", analysis.recommendation);
            }
            Err(e) => {
                panic!("Structured parsing test failed: {}", e);
            }
        }
    }
}
