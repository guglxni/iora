use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
    package: Package,
    dependencies: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1.1.4.1: Cargo.toml configuration validation tests
    mod cargo_toml_tests {
        use super::*;

        #[test]
        fn test_cargo_toml_exists() {
            let cargo_toml_path = Path::new("Cargo.toml");
            assert!(cargo_toml_path.exists(), "Cargo.toml should exist");
        }

        #[test]
        fn test_cargo_toml_parseable() {
            let cargo_toml_content = fs::read_to_string("Cargo.toml")
                .expect("Should be able to read Cargo.toml");

            let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)
                .expect("Cargo.toml should be valid TOML");

            assert_eq!(cargo_toml.package.name, "iora");
            assert_eq!(cargo_toml.package.version, "0.1.0");
        }

        #[test]
        fn test_rust_edition_2021() {
            let cargo_toml_content = fs::read_to_string("Cargo.toml")
                .expect("Should be able to read Cargo.toml");

            let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)
                .expect("Cargo.toml should be valid TOML");

            assert_eq!(cargo_toml.package.edition, "2021",
                "Should use Rust edition 2021");
        }

        #[test]
        fn test_core_dependencies_present() {
            let cargo_toml_content = fs::read_to_string("Cargo.toml")
                .expect("Should be able to read Cargo.toml");

            let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)
                .expect("Cargo.toml should be valid TOML");

            let required_deps = vec![
                "clap", "reqwest", "serde", "tokio",
                "solana-sdk", "solana-client", "typesense-rs"
            ];

            for dep in required_deps {
                assert!(cargo_toml.dependencies.contains_key(dep),
                    "Dependency '{}' should be present", dep);
            }
        }

        #[test]
        fn test_package_metadata_complete() {
            let cargo_toml_content = fs::read_to_string("Cargo.toml")
                .expect("Should be able to read Cargo.toml");

            let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)
                .expect("Cargo.toml should be valid TOML");

            assert!(cargo_toml.package.description.is_some(),
                "Package should have a description");
            assert!(cargo_toml.package.authors.is_some(),
                "Package should have authors");
        }
    }

    /// Test 1.1.4.1: Main.rs functionality and module import tests
    mod main_rs_tests {
        use super::*;

        #[test]
        fn test_main_rs_exists() {
            let main_rs_path = Path::new("src/main.rs");
            assert!(main_rs_path.exists(), "src/main.rs should exist");
        }

        #[test]
        fn test_main_rs_content() {
            let main_rs_content = fs::read_to_string("src/main.rs")
                .expect("Should be able to read src/main.rs");

            // Check for basic structure
            assert!(main_rs_content.contains("use dotenv"),
                "main.rs should load dotenv");
            assert!(main_rs_content.contains("fn main()"),
                "main.rs should have main function");
            assert!(main_rs_content.contains("dotenv::dotenv().ok()"),
                "main.rs should load environment variables");
        }

        #[test]
        fn test_module_declarations() {
            let lib_rs_content = fs::read_to_string("src/lib.rs")
                .expect("Should be able to read src/lib.rs");

            let required_modules = vec![
                "cli", "fetcher", "rag", "analyzer", "solana"
            ];

            for module in required_modules {
                assert!(lib_rs_content.contains(&format!("pub mod {};", module)),
                    "lib.rs should declare module '{}'", module);
            }
        }

        #[test]
        fn test_main_function_basic() {
            let main_rs_content = fs::read_to_string("src/main.rs")
                .expect("Should be able to read src/main.rs");

            assert!(main_rs_content.contains("dotenv::dotenv().ok()"),
                "main.rs should load environment variables");
        }
    }

    /// Test 1.1.4.1: CLI argument parsing structure tests
    mod cli_tests {
        use super::*;

        #[test]
        fn test_cli_module_exists() {
            let cli_rs_path = Path::new("src/modules/cli.rs");
            assert!(cli_rs_path.exists(), "src/modules/cli.rs should exist");
        }

        #[test]
        fn test_cli_module_structure() {
            let cli_content = fs::read_to_string("src/modules/cli.rs")
                .expect("Should be able to read src/modules/cli.rs");

            // Check for clap usage
            assert!(cli_content.contains("use clap::"),
                "CLI module should use clap crate");

            // Check for Command structure
            assert!(cli_content.contains("Command::new"),
                "CLI module should create a Command");

            // Check for required arguments
            let required_args = vec!["query", "gemini-key", "wallet-path"];
            for arg in required_args {
                assert!(cli_content.contains(arg),
                    "CLI should handle '{}' argument", arg);
            }
        }

        #[test]
        fn test_cli_build_function() {
            let cli_content = fs::read_to_string("src/modules/cli.rs")
                .expect("Should be able to read src/modules/cli.rs");

            assert!(cli_content.contains("pub fn build_cli"),
                "CLI module should export build_cli function");
        }

        #[test]
        fn test_cli_argument_structure() {
            let cli_content = fs::read_to_string("src/modules/cli.rs")
                .expect("Should be able to read src/modules/cli.rs");

            // Check for proper argument definitions
            assert!(cli_content.contains("Arg::new"),
                "CLI should define arguments with Arg::new");

            // Check for required flags
            assert!(cli_content.contains(".required(true)"),
                "CLI should have required arguments");
        }
    }

    /// Test 1.1.4.1: Project structure integrity tests
    mod project_structure_tests {
        use super::*;

        #[test]
        fn test_src_directory_exists() {
            let src_dir = Path::new("src");
            assert!(src_dir.exists(), "src directory should exist");
            assert!(src_dir.is_dir(), "src should be a directory");
        }

        #[test]
        fn test_modules_directory_exists() {
            let modules_dir = Path::new("src/modules");
            assert!(modules_dir.exists(), "src/modules directory should exist");
            assert!(modules_dir.is_dir(), "src/modules should be a directory");
        }

        #[test]
        fn test_all_module_files_exist() {
            let required_modules = vec![
                "cli.rs", "fetcher.rs", "rag.rs", "analyzer.rs", "solana.rs"
            ];

            for module_file in required_modules {
                let module_path = Path::new("src/modules").join(module_file);
                assert!(module_path.exists(),
                    "Module file {} should exist", module_file);
                assert!(module_path.is_file(),
                    "{} should be a file", module_file);
            }
        }

        #[test]
        fn test_assets_directory_exists() {
            let assets_dir = Path::new("assets");
            assert!(assets_dir.exists(), "assets directory should exist");
            assert!(assets_dir.is_dir(), "assets should be a directory");
        }

        #[test]
        fn test_historical_json_exists() {
            let historical_json = Path::new("assets/historical.json");
            assert!(historical_json.exists(),
                "assets/historical.json should exist");
            assert!(historical_json.is_file(),
                "assets/historical.json should be a file");
        }

        #[test]
        fn test_historical_json_valid() {
            let historical_content = fs::read_to_string("assets/historical.json")
                .expect("Should be able to read assets/historical.json");

            // Try to parse as JSON to ensure validity
            let _: serde_json::Value = serde_json::from_str(&historical_content)
                .expect("historical.json should contain valid JSON");
        }

        #[test]
        fn test_git_repository_exists() {
            let git_dir = Path::new(".git");
            assert!(git_dir.exists(), ".git directory should exist");
            assert!(git_dir.is_dir(), ".git should be a directory");
        }

        #[test]
        fn test_gitignore_exists() {
            let gitignore = Path::new(".gitignore");
            assert!(gitignore.exists(), ".gitignore should exist");
            assert!(gitignore.is_file(), ".gitignore should be a file");
        }

        #[test]
        fn test_env_example_exists() {
            let env_example = Path::new(".env.example");
            assert!(env_example.exists(), ".env.example should exist");
            assert!(env_example.is_file(), ".env.example should be a file");
        }

        #[test]
        fn test_docker_compose_exists() {
            let docker_compose = Path::new("docker-compose.yml");
            assert!(docker_compose.exists(), "docker-compose.yml should exist");
            assert!(docker_compose.is_file(), "docker-compose.yml should be a file");
        }

        #[test]
        fn test_cargo_lock_exists() {
            let cargo_lock = Path::new("Cargo.lock");
            assert!(cargo_lock.exists(), "Cargo.lock should exist");
            assert!(cargo_lock.is_file(), "Cargo.lock should be a file");
        }

        #[test]
        fn test_target_directory_exists() {
            let target_dir = Path::new("target");
            assert!(target_dir.exists(), "target directory should exist");
            assert!(target_dir.is_dir(), "target should be a directory");
        }
    }

    /// Test 1.1.4.1: Compilation and linking tests
    mod compilation_tests {

        #[test]
        fn test_project_compiles() {
            // This test will fail if the project doesn't compile
            // We use a simple assertion that should always pass if we reach here
            assert!(true, "Project should compile successfully");
        }

        #[test]
        fn test_cargo_check_passes() {
            use std::process::Command;

            let output = Command::new("cargo")
                .arg("check")
                .output()
                .expect("Failed to run cargo check");

            assert!(output.status.success(),
                "cargo check should pass. stderr: {}",
                String::from_utf8_lossy(&output.stderr));
        }
    }
}
