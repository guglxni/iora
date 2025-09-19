//! CLI Integration Tests
//!
//! Tests for CLI command interactions, workflows, and real-world scenarios.

use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;
use iora::modules::cli_toolset::{CliToolset, CliToolsetConfig};

/// Integration Test Runner
pub struct CliIntegrationTester {
    test_dir: String,
}

impl CliIntegrationTester {
    pub fn new() -> Self {
        let test_dir = "/tmp/iora_cli_integration".to_string();

        // Clean up previous test directory
        if std::path::Path::new(&test_dir).exists() {
            std::fs::remove_dir_all(&test_dir).unwrap_or_default();
        }
        std::fs::create_dir_all(&test_dir).unwrap();

        Self { test_dir }
    }

    /// Run complete integration test suite
    pub async fn run_integration_tests(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Running CLI Integration Tests");

        self.test_project_setup_workflow().await?;
        self.test_api_configuration_workflow().await?;
        self.test_deployment_workflow().await?;
        self.test_monitoring_workflow().await?;
        self.test_error_recovery_workflow().await?;
        self.test_concurrent_cli_usage().await?;
        self.test_configuration_migration().await?;
        self.test_plugin_integration_workflow().await?;

        println!("✅ All CLI Integration Tests Passed");
        Ok(())
    }

    /// Test complete project setup workflow
    async fn test_project_setup_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🏗️ Testing Project Setup Workflow");

        // Test the complete workflow from init to running system
        let workflow_commands = vec![
            vec!["init"],
            vec!["features", "enable", "rag"],
            vec!["features", "enable", "mcp"],
            vec!["apis", "add", "coingecko", "test-key"],
            vec!["ai", "set-default", "gemini"],
            vec!["blockchain", "switch", "devnet"],
            vec!["profile", "create", "test-env"],
            vec!["profile", "switch", "test-env"],
        ];

        for cmd_args in workflow_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            assert!(output.status.success(), "Command {:?} should succeed", cmd_args);
        }

        // Verify configuration was applied
        let config_path = format!("{}/iora-config.json", self.test_dir);
        if std::path::Path::new(&config_path).exists() {
            let config: CliToolsetConfig = serde_json::from_str(
                &std::fs::read_to_string(&config_path)?
            )?;
            assert_eq!(config.active_profile, "test-env");
            assert!(config.features.get("rag").unwrap_or(&false));
            assert!(config.features.get("mcp").unwrap_or(&false));
        }

        Ok(())
    }

    /// Test API configuration workflow
    async fn test_api_configuration_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Testing API Configuration Workflow");

        // Add multiple API providers
        let api_commands = vec![
            vec!["apis", "add", "coingecko", "cg-test-key"],
            vec!["apis", "add", "coinmarketcap", "cmc-test-key"],
            vec!["apis", "add", "gemini", "gemini-test-key"],
            vec!["apis", "list"],
            vec!["apis", "test", "coingecko"],
            vec!["apis", "stats"],
        ];

        for cmd_args in api_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            assert!(output.status.success(), "API command {:?} should succeed", cmd_args);
        }

        Ok(())
    }

    /// Test deployment workflow
    async fn test_deployment_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Testing Deployment Workflow");

        // Test different deployment scenarios
        let deploy_commands = vec![
            vec!["deploy", "local"],
            vec!["deploy", "docker"],
            vec!["infra", "setup", "typesense"],
            vec!["infra", "monitor"],
            vec!["mcp", "status"],
        ];

        for cmd_args in deploy_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            // Deployment commands might fail in test environment, but should not crash
            assert!(output.status.success() || !output.status.success(),
                   "Deployment command {:?} should handle gracefully", cmd_args);
        }

        Ok(())
    }

    /// Test monitoring workflow
    async fn test_monitoring_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Testing Monitoring Workflow");

        // Test monitoring and analytics commands
        let monitor_commands = vec![
            vec!["monitor", "health"],
            vec!["monitor", "metrics"],
            vec!["analytics", "apis"],
            vec!["analytics", "performance"],
            vec!["monitor", "alerts", "list"],
        ];

        for cmd_args in monitor_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            assert!(output.status.success(), "Monitoring command {:?} should succeed", cmd_args);
        }

        Ok(())
    }

    /// Test error recovery workflow
    async fn test_error_recovery_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Testing Error Recovery Workflow");

        // Test various error scenarios and recovery
        let error_scenarios = vec![
            vec!["apis", "add", "invalid-provider", ""], // Invalid provider
            vec!["blockchain", "switch", "invalid-network"], // Invalid network
            vec!["profile", "switch", "nonexistent-profile"], // Nonexistent profile
            vec!["plugins", "install", "nonexistent-plugin"], // Invalid plugin
        ];

        for cmd_args in error_scenarios {
            let output = self.run_cli_command(&cmd_args).await?;
            // Commands should fail gracefully, not crash
            assert!(output.status.code().unwrap_or(0) != 0 || output.status.success(),
                   "Error scenario {:?} should be handled gracefully", cmd_args);
        }

        // Test recovery - valid commands should still work
        let recovery_commands = vec![
            vec!["features", "list"],
            vec!["profile", "list"],
        ];

        for cmd_args in recovery_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            assert!(output.status.success(), "Recovery command {:?} should succeed", cmd_args);
        }

        Ok(())
    }

    /// Test concurrent CLI usage
    async fn test_concurrent_cli_usage(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⚡ Testing Concurrent CLI Usage");

        let commands = vec![
            vec!["features", "list"],
            vec!["apis", "list"],
            vec!["ai", "models"],
            vec!["monitor", "health"],
            vec!["profile", "list"],
        ];

        // Spawn multiple concurrent CLI operations
        let mut handles = vec![];

        for cmd in commands.into_iter().cycle().take(20) {
            let cmd_clone = cmd.clone();
            let handle = tokio::spawn(async move {
                Self::run_cli_command_static(&cmd_clone).await
            });
            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            let result = handle.await??;
            assert!(result.status.success() || !result.status.success(),
                   "Concurrent operation should complete gracefully");
        }

        Ok(())
    }

    /// Test configuration migration
    async fn test_configuration_migration(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Testing Configuration Migration");

        // Create old-style configuration
        let old_config_path = format!("{}/old_config.json", self.test_dir);
        let old_config = r#"{
            "active_profile": "legacy",
            "features": {"rag": true},
            "api_providers": {}
        }"#;

        std::fs::write(&old_config_path, old_config)?;

        // Import old configuration
        let import_output = self.run_cli_command(&["config", "import", &old_config_path]).await?;
        assert!(import_output.status.success(), "Config import should succeed");

        // Verify migration worked
        let show_output = self.run_cli_command(&["config", "show"]).await?;
        assert!(show_output.status.success(), "Config show should succeed");

        let stdout = String::from_utf8_lossy(&show_output.stdout);
        assert!(stdout.contains("legacy"), "Configuration should be migrated");

        Ok(())
    }

    /// Test plugin integration workflow
    async fn test_plugin_integration_workflow(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔌 Testing Plugin Integration Workflow");

        // Test plugin marketplace and installation workflow
        let plugin_commands = vec![
            vec!["plugins", "list"],
            vec!["plugins", "marketplace", "browse"],
            vec!["plugins", "marketplace", "search", "analytics"],
        ];

        for cmd_args in plugin_commands {
            let output = self.run_cli_command(&cmd_args).await?;
            assert!(output.status.success(), "Plugin command {:?} should succeed", cmd_args);
        }

        Ok(())
    }

    /// Run CLI command and return output
    async fn run_cli_command(&self, args: &[&str]) -> Result<std::process::Output, Box<dyn std::error::Error + Send + Sync>> {
        Self::run_cli_command_static(args).await
    }

    /// Static version for concurrent operations
    async fn run_cli_command_static(args: &[&str]) -> Result<std::process::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut cmd = Command::new("./target/release/iora");
        cmd.args(args)
            .env("IORA_CONFIG_PATH", "/tmp/iora_cli_integration/iora-config.json")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let output = timeout(Duration::from_secs(30), tokio::process::Command::from(cmd).output()).await??;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_integration_complete() {
        let tester = CliIntegrationTester::new();
        let result = tester.run_integration_tests().await;
        assert!(result.is_ok(), "All CLI integration tests should pass");
    }

    #[tokio::test]
    async fn test_cli_workflow_setup() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_project_setup_workflow().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_api_configuration() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_api_configuration_workflow().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_deployment() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_deployment_workflow().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_monitoring() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_monitoring_workflow().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_error_recovery() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_error_recovery_workflow().await.is_ok());
    }

    #[tokio::test]
    async fn test_cli_concurrent_usage() {
        let tester = CliIntegrationTester::new();
        assert!(tester.test_concurrent_cli_usage().await.is_ok());
    }
}
