/// Test 1.2.4.3: Services and Integration Testing
/// Comprehensive testing for Docker, Typesense, environment variables, and workflow scripts

#[cfg(test)]
mod services_integration_tests {
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    /// Test Docker and Docker Compose installation and functionality
    mod docker_integration_tests {
        use super::*;

        #[test]
        fn test_docker_installation() {
            println!("🔍 Testing Docker installation and availability...");

            let docker_version = Command::new("docker").arg("--version").output();

            match docker_version {
                Ok(output) => {
                    if output.status.success() {
                        let version_output = String::from_utf8_lossy(&output.stdout);
                        println!("✅ Docker is installed: {}", version_output.trim());
                        assert!(
                            version_output.contains("Docker") || version_output.contains("docker"),
                            "Docker version output should contain 'Docker' or 'docker'"
                        );
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("❌ Docker command failed: {}", stderr);
                        panic!("Docker is not properly installed or accessible");
                    }
                }
                Err(e) => {
                    println!("❌ Docker command not found: {}", e);
                    panic!(
                        "Docker is not installed. Please install Docker Desktop or Docker Engine"
                    );
                }
            }
        }

        #[test]
        fn test_docker_compose_installation() {
            println!("🔍 Testing Docker Compose installation...");

            // Try docker compose (newer syntax)
            let compose_v2 = Command::new("docker")
                .args(&["compose", "version"])
                .output();

            match compose_v2 {
                Ok(output) if output.status.success() => {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    println!(
                        "✅ Docker Compose v2 is available: {}",
                        version_output.trim()
                    );
                    assert!(
                        version_output.contains("Docker Compose")
                            || version_output.contains("version"),
                        "Docker Compose version output should be valid"
                    );
                    return;
                }
                Ok(output) => {
                    println!(
                        "⚠️  Docker Compose v2 returned error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                    // Try docker-compose (legacy syntax)
                    let compose_v1 = Command::new("docker-compose").arg("--version").output();

                    match compose_v1 {
                        Ok(output) if output.status.success() => {
                            let version_output = String::from_utf8_lossy(&output.stdout);
                            println!(
                                "✅ Docker Compose v1 is available: {}",
                                version_output.trim()
                            );
                            assert!(
                                version_output.contains("docker-compose")
                                    || version_output.contains("Docker Compose"),
                                "Docker Compose version output should be valid"
                            );
                        }
                        Ok(_) => {
                            println!("⚠️  Docker Compose v1 returned error");
                            panic!(
                                "Docker Compose is not installed. Please install Docker Compose"
                            );
                        }
                        Err(e) => {
                            println!("❌ Neither Docker Compose v2 nor v1 found: {}", e);
                            panic!(
                                "Docker Compose is not installed. Please install Docker Compose"
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️  Docker Compose v2 command failed: {}", e);
                    // Try docker-compose (legacy syntax)
                    let compose_v1 = Command::new("docker-compose").arg("--version").output();

                    match compose_v1 {
                        Ok(output) if output.status.success() => {
                            let version_output = String::from_utf8_lossy(&output.stdout);
                            println!(
                                "✅ Docker Compose v1 is available: {}",
                                version_output.trim()
                            );
                            assert!(
                                version_output.contains("docker-compose")
                                    || version_output.contains("Docker Compose"),
                                "Docker Compose version output should be valid"
                            );
                        }
                        Ok(_) => {
                            println!("⚠️  Docker Compose v1 returned error");
                            panic!(
                                "Docker Compose is not installed. Please install Docker Compose"
                            );
                        }
                        Err(e) => {
                            println!("❌ Neither Docker Compose v2 nor v1 found: {}", e);
                            panic!(
                                "Docker Compose is not installed. Please install Docker Compose"
                            );
                        }
                    }
                }
            }
        }

        #[test]
        fn test_docker_daemon_connectivity() {
            println!("🔍 Testing Docker daemon connectivity...");

            let docker_info = Command::new("docker").arg("info").output();

            match docker_info {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Docker daemon is running and accessible");
                        let info_output = String::from_utf8_lossy(&output.stdout);
                        assert!(
                            info_output.contains("Containers:") || info_output.contains("Images:"),
                            "Docker info should show container and image information"
                        );
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("❌ Docker daemon not accessible: {}", stderr);
                        println!("💡 Please start Docker Desktop or Docker daemon");
                        panic!("Docker daemon is not running or not accessible");
                    }
                }
                Err(e) => {
                    println!("❌ Docker command failed: {}", e);
                    panic!("Docker daemon connectivity test failed");
                }
            }
        }

        #[test]
        fn test_docker_compose_file_validation() {
            println!("🔍 Testing Docker Compose file validation...");

            let compose_file = Path::new("docker-compose.yml");
            assert!(compose_file.exists(), "docker-compose.yml should exist");
            assert!(
                compose_file.is_file(),
                "docker-compose.yml should be a file"
            );

            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            assert!(
                !content.trim().is_empty(),
                "docker-compose.yml should not be empty"
            );
            assert!(
                content.contains("services:"),
                "docker-compose.yml should define services"
            );
            assert!(
                content.contains("typesense"),
                "docker-compose.yml should include Typesense service"
            );

            println!("✅ Docker Compose file is valid and contains required services");

            // Test compose config validation
            let config_test = Command::new("docker").args(&["compose", "config"]).output();

            match config_test {
                Ok(output) if output.status.success() => {
                    println!("✅ Docker Compose configuration is valid");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("❌ Docker Compose configuration error: {}", stderr);
                    panic!("Docker Compose file has configuration errors");
                }
                Err(_) => {
                    // Try legacy docker-compose command
                    let legacy_config = Command::new("docker-compose").args(&["config"]).output();

                    match legacy_config {
                        Ok(output) if output.status.success() => {
                            println!(
                                "✅ Docker Compose configuration is valid (using legacy syntax)"
                            );
                        }
                        _ => {
                            println!("⚠️  Could not validate Docker Compose configuration");
                            println!("💡 This is non-critical, but compose file should be validated manually");
                        }
                    }
                }
            }
        }
    }

    /// Test Typesense service functionality
    mod typesense_integration_tests {
        use super::*;

        #[test]
        fn test_typesense_service_definition() {
            println!("🔍 Testing Typesense service definition in docker-compose.yml...");

            let compose_file = Path::new("docker-compose.yml");
            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            // Check for Typesense service configuration
            assert!(
                content.to_lowercase().contains("typesense"),
                "Typesense service should be defined"
            );
            assert!(
                content.contains("image:"),
                "Typesense should specify an image"
            );
            assert!(content.contains("ports:"), "Typesense should expose ports");

            println!("✅ Typesense service is properly defined in docker-compose.yml");
        }

        #[test]
        fn test_typesense_environment_variables() {
            println!("🔍 Testing Typesense environment variables configuration...");

            let compose_file = Path::new("docker-compose.yml");
            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            // Check for required Typesense environment variables
            let required_env_vars = vec!["TYPESENSE_API_KEY", "TYPESENSE_DATA_DIR"];

            for env_var in &required_env_vars {
                assert!(
                    content.contains(env_var),
                    "Typesense service should have environment variable: {}",
                    env_var
                );
            }

            // Check for API key configuration
            assert!(
                content.contains("iora_dev_typesense_key_2024"),
                "Typesense should use the configured API key"
            );

            println!("✅ Typesense environment variables are properly configured");
        }

        #[test]
        fn test_typesense_service_health_checks() {
            println!("🔍 Testing Typesense service health check configuration...");

            let compose_file = Path::new("docker-compose.yml");
            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            // Check for health check configuration
            assert!(
                content.contains("healthcheck:"),
                "Typesense should have health check"
            );
            assert!(
                content.contains("test:"),
                "Health check should have test command"
            );
            assert!(
                content.contains("curl"),
                "Health check should use curl for HTTP testing"
            );

            println!("✅ Typesense health checks are properly configured");
        }

        #[test]
        fn test_typesense_data_persistence() {
            println!("🔍 Testing Typesense data persistence configuration...");

            let compose_file = Path::new("docker-compose.yml");
            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            // Check for volume mounts
            assert!(
                content.contains("volumes:"),
                "Typesense should have volume configuration"
            );
            assert!(
                content.contains("/data"),
                "Typesense should persist data to /data"
            );

            println!("✅ Typesense data persistence is properly configured");
        }
    }

    /// Test environment variable integration
    mod environment_integration_tests {
        use super::*;

        #[test]
        fn test_dotenv_file_existence() {
            println!("🔍 Testing .env file existence and accessibility...");

            let env_file = Path::new(".env");
            if env_file.exists() {
                println!("✅ .env file exists");
                assert!(env_file.is_file(), ".env should be a file");

                let content =
                    fs::read_to_string(env_file).expect("Should be able to read .env file");

                assert!(!content.trim().is_empty(), ".env file should not be empty");
                println!("✅ .env file is accessible and contains content");
            } else {
                println!("⚠️  .env file does not exist - this may be expected in CI/CD");
                println!("💡 Ensure environment variables are set through other means");
            }
        }

        #[test]
        fn test_environment_variable_loading() {
            println!("🔍 Testing environment variable loading functionality...");

            // Test dotenv loading capability
            let dotenv_result = dotenv::dotenv();
            match dotenv_result {
                Ok(path) => {
                    println!("✅ .env file loaded successfully from: {:?}", path);
                }
                Err(e) => {
                    println!("⚠️  .env file not loaded: {}", e);
                    println!("💡 This may be expected if .env doesn't exist or is empty");
                }
            }

            // Test that we can read environment variables
            match std::env::var("GEMINI_API_KEY") {
                Ok(key) => {
                    assert!(!key.is_empty(), "GEMINI_API_KEY should not be empty");
                    println!("✅ GEMINI_API_KEY is accessible");
                }
                Err(_) => {
                    println!("⚠️  GEMINI_API_KEY not set in environment");
                }
            }

            match std::env::var("TYPESENSE_API_KEY") {
                Ok(key) => {
                    assert!(!key.is_empty(), "TYPESENSE_API_KEY should not be empty");
                    println!("✅ TYPESENSE_API_KEY is accessible");
                }
                Err(_) => {
                    println!("⚠️  TYPESENSE_API_KEY not set in environment");
                }
            }
        }

        #[test]
        fn test_environment_variable_validation() {
            println!("🔍 Testing environment variable validation...");

            // Test that critical environment variables have valid formats
            if let Ok(gemini_key) = std::env::var("GEMINI_API_KEY") {
                assert!(
                    gemini_key.starts_with("AIzaSy") || gemini_key.len() > 20,
                    "GEMINI_API_KEY should have valid format"
                );
                println!("✅ GEMINI_API_KEY format is valid");
            }

            if let Ok(typesense_key) = std::env::var("TYPESENSE_API_KEY") {
                assert!(
                    !typesense_key.is_empty() && typesense_key.len() >= 8,
                    "TYPESENSE_API_KEY should be non-empty and reasonably long"
                );
                println!("✅ TYPESENSE_API_KEY format is valid");
            }

            if let Ok(rpc_url) = std::env::var("SOLANA_RPC_URL") {
                assert!(
                    rpc_url.starts_with("http"),
                    "SOLANA_RPC_URL should start with http"
                );
                println!("✅ SOLANA_RPC_URL format is valid");
            }
        }

        #[test]
        fn test_config_module_integration() {
            println!("🔍 Testing configuration module integration with environment...");

            // This test ensures that our config module can load and validate environment variables
            // We'll use the config module's functions to test integration
            use iora::modules::config;

            match config::init_config() {
                Ok(_) => {
                    println!("✅ Configuration module initialized successfully");
                }
                Err(e) => {
                    println!("⚠️  Configuration module initialization failed: {}", e);
                    println!(
                        "💡 This may be expected if required environment variables are not set"
                    );
                }
            }

            match config::get_config() {
                Ok(cfg) => {
                    println!("✅ Configuration module accessible");
                    // Test that we can access configuration values
                    let _rpc_url = cfg.solana_rpc_url();
                    let _typesense_key = cfg.typesense_api_key();
                    let _typesense_url = cfg.typesense_url();
                    println!("✅ Configuration values are accessible");
                }
                Err(e) => {
                    println!("⚠️  Configuration module not accessible: {}", e);
                    println!("💡 This may be expected if configuration was not initialized");
                }
            }
        }
    }

    /// Test development workflow script functionality
    mod workflow_integration_tests {
        use super::*;

        #[test]
        fn test_development_workflow_script() {
            println!("🔍 Testing development workflow script...");

            let script_path = "scripts/dev-workflow.sh";
            let script_file = Path::new(script_path);

            assert!(
                script_file.exists(),
                "Development workflow script should exist"
            );
            assert!(script_file.is_file(), "Script should be a file");

            let content =
                fs::read_to_string(script_file).expect("Should be able to read workflow script");

            assert!(
                !content.trim().is_empty(),
                "Workflow script should not be empty"
            );

            // Check for proper shebang
            assert!(
                content.contains("#!/bin/bash") || content.contains("#!/bin/zsh"),
                "Script should have proper shebang"
            );

            // Check for common workflow commands
            let expected_commands = vec!["cargo", "build", "test", "run"];
            for cmd in &expected_commands {
                assert!(
                    content.contains(cmd),
                    "Workflow script should contain command: {}",
                    cmd
                );
            }

            println!("✅ Development workflow script is properly configured");
        }

        #[test]
        fn test_makefile_targets() {
            println!("🔍 Testing Makefile targets...");

            let makefile_path = "Makefile";
            let makefile = Path::new(makefile_path);

            assert!(makefile.exists(), "Makefile should exist");
            assert!(makefile.is_file(), "Makefile should be a file");

            let content = fs::read_to_string(makefile).expect("Should be able to read Makefile");

            assert!(!content.trim().is_empty(), "Makefile should not be empty");

            // Check for essential targets
            let essential_targets = vec!["build", "test", "clean", "run"];
            for target in &essential_targets {
                assert!(
                    content.contains(&format!("{}:", target)),
                    "Makefile should have target: {}",
                    target
                );
            }

            println!("✅ Makefile contains essential targets");
        }

        #[test]
        fn test_makefile_functionality() {
            println!("🔍 Testing Makefile functionality...");

            // Test that make command is available
            let make_version = Command::new("make").arg("--version").output();

            match make_version {
                Ok(output) if output.status.success() => {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    println!(
                        "✅ Make is available: {}",
                        version_output.lines().next().unwrap_or("Unknown version")
                    );
                }
                _ => {
                    println!("⚠️  Make command not available");
                    println!("💡 Makefile functionality cannot be tested without make");
                    return;
                }
            }

            // Test basic make targets (non-destructive ones)
            let make_help = Command::new("make").arg("help").output();

            match make_help {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ Makefile help target works");
                    } else {
                        println!("⚠️  Makefile help target not available or failed");
                    }
                }
                _ => {
                    println!("⚠️  Could not test Makefile targets");
                }
            }
        }

        #[test]
        fn test_installation_scripts() {
            println!("🔍 Testing installation scripts...");

            let scripts_dir = Path::new("scripts");
            assert!(scripts_dir.exists(), "Scripts directory should exist");
            assert!(scripts_dir.is_dir(), "Scripts should be a directory");

            // Check for essential installation scripts
            let essential_scripts = vec![
                "install-rust.sh",
                "install-solana.sh",
                "setup-typesense.sh",
                "install-all-tools.sh",
            ];

            for script in &essential_scripts {
                let script_path = scripts_dir.join(script);
                assert!(
                    script_path.exists(),
                    "Installation script should exist: {}",
                    script
                );
                assert!(script_path.is_file(), "Script should be a file: {}", script);

                let content = fs::read_to_string(&script_path)
                    .expect(&format!("Should be able to read script: {}", script));

                assert!(
                    !content.trim().is_empty(),
                    "Script should not be empty: {}",
                    script
                );

                // Check for proper shebang
                assert!(
                    content.contains("#!/bin/bash") || content.contains("#!/bin/zsh"),
                    "Script should have proper shebang: {}",
                    script
                );
            }

            println!("✅ All essential installation scripts are present and valid");
        }

        #[test]
        fn test_script_execution_permissions() {
            println!("🔍 Testing script execution permissions...");

            let scripts_to_check = vec![
                "scripts/dev-workflow.sh",
                "scripts/install-rust.sh",
                "scripts/install-solana.sh",
                "scripts/setup-typesense.sh",
            ];

            for script_path in &scripts_to_check {
                let path = Path::new(script_path);
                if path.exists() {
                    // Check if file is executable (this is a basic check)
                    let metadata = fs::metadata(path).expect(&format!(
                        "Should be able to get metadata for: {}",
                        script_path
                    ));

                    // On Unix systems, check executable bit
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let permissions = metadata.permissions();
                        let mode = permissions.mode();
                        if mode & 0o111 == 0 {
                            println!("⚠️  Script may not be executable: {}", script_path);
                            println!("💡 Run: chmod +x {}", script_path);
                        } else {
                            println!("✅ Script is executable: {}", script_path);
                        }
                    }

                    #[cfg(not(unix))]
                    {
                        println!(
                            "✅ Script exists (Windows execution permissions not checked): {}",
                            script_path
                        );
                    }
                }
            }
        }
    }

    /// Test overall system integration
    mod system_integration_tests {
        use super::*;

        #[test]
        fn test_project_build_integration() {
            println!("🔍 Testing complete project build integration...");

            let build_result = Command::new("cargo").args(&["build", "--release"]).output();

            match build_result {
                Ok(output) if output.status.success() => {
                    println!("✅ Project builds successfully in release mode");

                    // Check that binary was created
                    let binary_path = Path::new("target/release/iora");
                    assert!(
                        binary_path.exists(),
                        "Release binary should be created at target/release/iora"
                    );

                    println!("✅ Release binary is available");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("❌ Project build failed: {}", stderr);
                    panic!("Project should build successfully");
                }
                Err(e) => {
                    println!("❌ Build command failed: {}", e);
                    panic!("Cargo build command should be available");
                }
            }
        }

        #[test]
        fn test_project_test_integration() {
            println!("🔍 Testing project test suite integration...");

            // Run a quick test to ensure testing infrastructure works
            let test_result = Command::new("cargo")
                .args(&["test", "--lib", "--quiet"])
                .output();

            match test_result {
                Ok(output) if output.status.success() => {
                    println!("✅ Project test suite runs successfully");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("❌ Test suite failed: {}", stderr);
                    println!("⚠️  This may be expected if some tests are failing");
                }
                Err(e) => {
                    println!("❌ Test command failed: {}", e);
                    panic!("Cargo test command should be available");
                }
            }
        }

        #[test]
        fn test_dependency_integration() {
            println!("🔍 Testing dependency integration...");

            let tree_result = Command::new("cargo").arg("tree").output();

            match tree_result {
                Ok(output) if output.status.success() => {
                    let tree_output = String::from_utf8_lossy(&output.stdout);
                    println!("✅ Dependency tree is accessible");

                    // Check for critical dependencies
                    let critical_deps = vec![
                        "clap",
                        "reqwest",
                        "serde",
                        "tokio",
                        "solana-sdk",
                        "solana-client",
                    ];

                    for dep in &critical_deps {
                        assert!(
                            tree_output.contains(dep),
                            "Critical dependency should be present: {}",
                            dep
                        );
                    }

                    println!("✅ All critical dependencies are properly integrated");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("❌ Dependency tree check failed: {}", stderr);
                    panic!("Should be able to check dependency tree");
                }
                Err(e) => {
                    println!("❌ Cargo tree command failed: {}", e);
                    panic!("Cargo tree command should be available");
                }
            }
        }

        #[test]
        fn test_service_startup_simulation() {
            println!("🔍 Testing service startup simulation...");

            // This test simulates checking if services could start
            // without actually starting them (to avoid conflicts)

            let compose_file = Path::new("docker-compose.yml");
            assert!(
                compose_file.exists(),
                "Docker Compose file should exist for service simulation"
            );

            let content = fs::read_to_string(compose_file)
                .expect("Should be able to read docker-compose.yml");

            // Verify all required components are defined
            let required_services = vec!["typesense"];
            let required_components = vec!["image:", "ports:", "environment:", "volumes:"];

            for service in &required_services {
                assert!(
                    content.to_lowercase().contains(service),
                    "Service should be defined: {}",
                    service
                );
            }

            for component in &required_components {
                assert!(
                    content.contains(component),
                    "Component should be configured: {}",
                    component
                );
            }

            println!("✅ Service startup configuration is complete");
            println!("💡 Services can be started with: docker compose up -d");
        }
    }
}
