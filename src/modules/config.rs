use std::env;
use std::path::PathBuf;
use once_cell::sync::OnceCell;

/// Configuration structure for the I.O.R.A. application
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub gemini_api_key: String,
    pub solana_rpc_url: String,
    pub solana_wallet_path: PathBuf,
    pub typesense_api_key: String,
    pub typesense_url: String,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_dotenv(true)
    }

    /// Load configuration from environment variables with dotenv control
    pub fn from_env_with_dotenv(load_dotenv: bool) -> Result<Self, ConfigError> {
        // Load .env file if it exists and requested
        if load_dotenv {
            dotenv::dotenv().ok();
        }

        let gemini_api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| ConfigError::MissingEnvVar("GEMINI_API_KEY".to_string()))?;

        // Validate Gemini API key format (should start with AIza)
        if !gemini_api_key.starts_with("AIza") {
            return Err(ConfigError::InvalidApiKey("Gemini API key should start with 'AIza'".to_string()));
        }

        let solana_rpc_url = env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

        let solana_wallet_path = env::var("SOLANA_WALLET_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./wallets/devnet-wallet.json"));

        let typesense_api_key = env::var("TYPESENSE_API_KEY")
            .unwrap_or_else(|_| "iora_dev_typesense_key_2024".to_string());

        let typesense_url = env::var("TYPESENSE_URL")
            .unwrap_or_else(|_| "http://localhost:8108".to_string());

        // Validate Typesense URL format
        if !typesense_url.starts_with("http") {
            return Err(ConfigError::InvalidUrl("Typesense URL should start with 'http'".to_string()));
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
    pub fn gemini_api_key(&self) -> &str {
        &self.gemini_api_key
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
        if !self.solana_rpc_url.starts_with("https://") && !self.solana_rpc_url.starts_with("http://") {
            return Err(ConfigError::InvalidUrl("Invalid Solana RPC URL format".to_string()));
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

    CONFIG.set(config).map_err(|_| ConfigError::MissingEnvVar("Configuration already initialized".to_string()))
}

/// Get the global configuration instance
pub fn get_config() -> Result<&'static AppConfig, ConfigError> {
    CONFIG.get().ok_or_else(|| ConfigError::MissingEnvVar("Configuration not initialized".to_string()))
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
    use tempfile::NamedTempFile;

    fn setup_test_env() {
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");
        env::set_var("SOLANA_RPC_URL", "https://api.devnet.solana.com");
        env::set_var("SOLANA_WALLET_PATH", "./wallets/devnet-wallet.json");
        env::set_var("TYPESENSE_API_KEY", "test_key_123");
        env::set_var("TYPESENSE_URL", "http://localhost:8108");
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
        assert_eq!(config.gemini_api_key, "AIzaSyTest123456789");
        assert_eq!(config.solana_rpc_url, "https://api.devnet.solana.com");
        assert_eq!(config.typesense_api_key, "test_key_123");
        assert_eq!(config.typesense_url, "http://localhost:8108");

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
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");
        env::set_var("TYPESENSE_URL", "invalid-url");

        let config = AppConfig::from_env_with_dotenv(false);
        assert!(config.is_err());

        if let Err(ConfigError::InvalidUrl(_)) = config {
            // Expected error
        } else {
            panic!("Expected InvalidUrl error");
        }

        env::remove_var("GEMINI_API_KEY");
        env::remove_var("TYPESENSE_URL");
    }

    #[test]
    fn test_config_defaults() {
        // Clear all environment variables first
        env::set_var("GEMINI_API_KEY", "AIzaSyTest123456789");
        env::remove_var("SOLANA_RPC_URL");
        env::remove_var("TYPESENSE_API_KEY");
        env::remove_var("TYPESENSE_URL");

        let config = AppConfig::from_env_with_dotenv(false).unwrap();

        assert_eq!(config.solana_rpc_url, "https://api.devnet.solana.com");
        assert_eq!(config.typesense_api_key, "iora_dev_typesense_key_2024");
        assert_eq!(config.typesense_url, "http://localhost:8108");

        env::remove_var("GEMINI_API_KEY");
    }

    #[test]
    fn test_helper_functions() {
        env::set_var("TEST_VAR", "test_value");

        assert_eq!(get_env_var("TEST_VAR", "default"), "test_value");
        assert_eq!(get_env_var("NON_EXISTENT", "default"), "default");
        assert_eq!(get_optional_env_var("TEST_VAR"), Some("test_value".to_string()));
        assert_eq!(get_optional_env_var("NON_EXISTENT"), None);

        env::remove_var("TEST_VAR");
    }
}
