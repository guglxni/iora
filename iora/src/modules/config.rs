use once_cell::sync::OnceCell;
use std::env;
use std::path::PathBuf;

/// Configuration structure for the I.O.R.A. application
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub gemini_api_key: Option<String>,
    pub solana_rpc_url: String,
    pub solana_wallet_path: PathBuf,
    pub typesense_api_key: String,
    pub typesense_url: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        println!("🔧 Loading configuration from environment...");
        Self::from_env_with_dotenv(true)
    }

    /// Load configuration from environment variables with dotenv control
    pub fn from_env_with_dotenv(load_dotenv: bool) -> Result<Self, ConfigError> {
        // Load .env file if it exists and requested
        if load_dotenv {
            match dotenv::dotenv() {
                Ok(path) => println!("✅ Loaded .env file from: {:?}", path),
                Err(e) => println!("⚠️  Could not load .env file: {}", e),
            }
        }

        // Check LLM provider to determine which API keys are required
        let llm_provider = env::var("LLM_PROVIDER").unwrap_or_else(|_| "gemini".to_string());

        let gemini_api_key = if llm_provider == "gemini" {
            let key = env::var("GEMINI_API_KEY")
                .map_err(|_| ConfigError::MissingEnvVar("GEMINI_API_KEY".to_string()))?;

            // Validate Gemini API key format (should start with AIza)
            if !key.starts_with("AIza") {
                return Err(ConfigError::InvalidApiKey(
                    "Gemini API key should start with 'AIza'".to_string(),
                ));
            }
            Some(key)
        } else {
            // For non-Gemini providers, Gemini key is optional
            env::var("GEMINI_API_KEY").ok()
        };

        let solana_rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

        let solana_wallet_path = env::var("SOLANA_WALLET_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./wallets/mainnet-wallet.json"));

        let typesense_api_key = env::var("TYPESENSE_API_KEY")
            .unwrap_or_else(|_| "iora_dev_typesense_key_2024".to_string());

        let typesense_url = env::var("TYPESENSE_URL")
            .unwrap_or_else(|_| "https://typesense.your-domain.com".to_string());

        // Validate Typesense URL format
        if !typesense_url.starts_with("http") {
            return Err(ConfigError::InvalidUrl(
                "Typesense URL should start with 'http'".to_string(),
            ));
        }

        Ok(AppConfig {
            gemini_api_key,
            solana_rpc_url,
            solana_wallet_path,
            typesense_api_key,
            typesense_url,
        })
    }

    /// Get the Solana RPC URL
    pub fn solana_rpc_url(&self) -> &str {
        &self.solana_rpc_url
    }

    /// Get the Solana wallet path
    pub fn solana_wallet_path(&self) -> &PathBuf {
        &self.solana_wallet_path
    }

    /// Get the Gemini API key
    pub fn gemini_api_key(&self) -> Option<&str> {
        self.gemini_api_key.as_deref()
    }

    /// Get the Typesense API key
    pub fn typesense_api_key(&self) -> &str {
        &self.typesense_api_key
    }

    /// Get the Typesense URL
    pub fn typesense_url(&self) -> &str {
        &self.typesense_url
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Check if wallet file exists
        if !self.solana_wallet_path.exists() {
            return Err(ConfigError::WalletNotFound(self.solana_wallet_path.clone()));
        }

        // Validate URLs
        if !self.solana_rpc_url.starts_with("https://")
            && !self.solana_rpc_url.starts_with("http://")
        {
            return Err(ConfigError::InvalidUrl(
                "Invalid Solana RPC URL format".to_string(),
            ));
        }

        Ok(())
    }
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid API key: {0}")]
    InvalidApiKey(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Wallet file not found: {0}")]
    WalletNotFound(PathBuf),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Environment variable error: {0}")]
    VarError(#[from] std::env::VarError),
}

/// Global configuration instance
static CONFIG: OnceCell<AppConfig> = OnceCell::new();

/// Initialize the global configuration
pub fn init_config() -> Result<(), ConfigError> {
    let config = AppConfig::from_env()?;
    config.validate()?;

    CONFIG
        .set(config)
        .map_err(|_| ConfigError::MissingEnvVar("Configuration already initialized".to_string()))
}

/// Get the global configuration instance
pub fn get_config() -> Result<&'static AppConfig, ConfigError> {
    CONFIG
        .get()
        .ok_or_else(|| ConfigError::MissingEnvVar("Configuration not initialized".to_string()))
}

/// Helper function to get a configuration value with a default
pub fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Helper function to get an optional configuration value
pub fn get_optional_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn setup_test_env() {
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");
        env::set_var("SOLANA_RPC_URL", "https://api.mainnet-beta.solana.com");
        env::set_var("SOLANA_WALLET_PATH", "./wallets/mainnet-wallet.json");
        env::set_var("TYPESENSE_API_KEY", "iora_prod_typesense_key_2024");
        env::set_var("TYPESENSE_URL", "https://typesense.your-domain.com");
    }

    fn cleanup_test_env() {
        env::remove_var("GEMINI_API_KEY");
        env::remove_var("SOLANA_RPC_URL");
        env::remove_var("SOLANA_WALLET_PATH");
        env::remove_var("TYPESENSE_API_KEY");
        env::remove_var("TYPESENSE_URL");
    }

    #[test]
    fn test_config_from_env_success() {
        setup_test_env();

        let config = AppConfig::from_env_with_dotenv(false);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.gemini_api_key(), Some("AIzaSyTest123456789"));
        assert_eq!(
            config.solana_rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(config.typesense_api_key(), "iora_prod_typesense_key_2024");
        assert_eq!(config.typesense_url(), "https://typesense.your-domain.com");

        cleanup_test_env();
    }

    #[test]
    fn test_config_missing_gemini_key() {
        env::remove_var("GEMINI_API_KEY");

        let config = AppConfig::from_env_with_dotenv(false);
        assert!(config.is_err());

        if let Err(ConfigError::MissingEnvVar(var)) = config {
            assert_eq!(var, "GEMINI_API_KEY");
        } else {
            panic!("Expected MissingEnvVar error");
        }
    }

    #[test]
    fn test_config_invalid_gemini_key() {
        env::set_var("GEMINI_API_KEY", "invalid_key_format");

        let config = AppConfig::from_env_with_dotenv(false);
        assert!(config.is_err());

        if let Err(ConfigError::InvalidApiKey(_)) = config {
            // Expected error
        } else {
            panic!("Expected InvalidApiKey error");
        }

        env::remove_var("GEMINI_API_KEY");
    }

    #[test]
    fn test_config_invalid_typesense_url() {
        // Clean up any existing environment variables
        env::remove_var("GEMINI_API_KEY");
        env::remove_var("TYPESENSE_URL");

        // Set test environment variables
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");
        env::set_var("TYPESENSE_URL", "invalid-url");

        // Ensure environment variables are set
        assert_eq!(env::var("TYPESENSE_URL").unwrap(), "invalid-url");

        let config = AppConfig::from_env_with_dotenv(false);
        assert!(
            config.is_err(),
            "Config creation should fail with invalid URL"
        );

        match config {
            Err(ConfigError::InvalidUrl(msg)) => {
                assert!(
                    msg.contains("http"),
                    "Error message should mention http requirement"
                );
            }
            other => panic!("Expected InvalidUrl error, got: {:?}", other),
        }

        env::remove_var("GEMINI_API_KEY");
        env::remove_var("TYPESENSE_URL");
    }

    #[test]
    fn test_config_defaults() {
        // Test default values by ensuring specific variables are NOT set
        // This allows the unwrap_or_else defaults to take effect

        // Remove the variables we want to test defaults for
        env::remove_var("SOLANA_RPC_URL");
        env::remove_var("TYPESENSE_API_KEY");
        env::remove_var("TYPESENSE_URL");

        // Keep GEMINI_API_KEY set for the test to succeed
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");

        // Create config - should use defaults for unset variables
        let config = AppConfig::from_env_with_dotenv(false).unwrap();

        // Test that defaults are used when environment variables are not set
        assert_eq!(
            config.solana_rpc_url(),
            "https://api.mainnet-beta.solana.com",
            "SOLANA_RPC_URL should default to Mainnet URL when not set"
        );
        assert_eq!(
            config.typesense_api_key(),
            "iora_dev_typesense_key_2024",
            "TYPESENSE_API_KEY should default to development key when not set"
        );
        assert_eq!(
            config.typesense_url(),
            "https://typesense.your-domain.com",
            "TYPESENSE_URL should default to production URL when not set"
        );

        // Clean up
        env::remove_var("GEMINI_API_KEY");
    }

    #[test]
    fn test_helper_functions() {
        env::set_var("TEST_VAR", "test_value");

        assert_eq!(get_env_var("TEST_VAR", "default"), "test_value");
        assert_eq!(get_env_var("NON_EXISTENT", "default"), "default");
        assert_eq!(
            get_optional_env_var("TEST_VAR"),
            Some("test_value".to_string())
        );
        assert_eq!(get_optional_env_var("NON_EXISTENT"), None);

        env::remove_var("TEST_VAR");
    }
}
