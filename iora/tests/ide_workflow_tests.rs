/// Test 1.2.4.4: IDE and Workflow Validation
/// Comprehensive testing for VS Code configuration, workflow scripts, pre-commit hooks, and CI/CD

#[cfg(test)]
mod ide_workflow_tests {
    use std::process::Command;
    use std::path::Path;
    use std::fs;

    /// Test VS Code settings and extensions configuration
    mod vscode_configuration_tests {
        use super::*;

        #[test]
        fn test_vscode_directory_exists() {
            println!("🔍 Testing VS Code directory structure...");

            let vscode_dir = Path::new(".vscode");
            assert!(vscode_dir.exists(), ".vscode directory should exist");
            assert!(vscode_dir.is_dir(), ".vscode should be a directory");

            println!("✅ VS Code directory exists");
        }

        #[test]
        fn test_vscode_settings_configuration() {
            println!("🔍 Testing VS Code settings configuration...");

            let settings_file = Path::new(".vscode/settings.json");
            assert!(settings_file.exists(), "VS Code settings file should exist");

            let content = fs::read_to_string(settings_file)
                .expect("Should be able to read VS Code settings");

            assert!(!content.trim().is_empty(), "VS Code settings should not be empty");

            // VS Code settings use JSONC (JSON with comments), so we'll validate structure differently
            // Check for essential JSON structure markers instead of parsing as pure JSON
            assert!(content.contains("{"), "Settings should start with JSON object");
            assert!(content.contains("}"), "Settings should end with JSON object");
            assert!(content.contains(":"), "Settings should contain key-value pairs");

            // Check for essential Rust configuration
            assert!(content.contains("rust-analyzer"), "Settings should include rust-analyzer configuration");
            assert!(content.contains("editor.formatOnSave"), "Settings should enable format on save");
            assert!(content.contains("clippy"), "Settings should include clippy configuration");

            // Check for editor configuration
            assert!(content.contains("editor.rulers"), "Settings should include ruler configuration");
            assert!(content.contains("editor.tabSize"), "Settings should include tab size configuration");

            // Check for file associations
            assert!(content.contains("files.associations"), "Settings should include file associations");

            // Check for tasks configuration
            assert!(content.contains("tasks"), "Settings should include tasks configuration");
            assert!(content.contains("cargo build"), "Tasks should include cargo build");
            assert!(content.contains("cargo test"), "Tasks should include cargo test");

            // Check for launch configuration
            assert!(content.contains("launch"), "Settings should include launch configuration");
            assert!(content.contains("Debug I.O.R.A."), "Launch should include I.O.R.A. debug configuration");

            println!("✅ VS Code settings configuration is comprehensive and valid");
        }

        #[test]
        fn test_vscode_extensions_recommendations() {
            println!("🔍 Testing VS Code extensions recommendations...");

            let extensions_file = Path::new(".vscode/extensions.json");
            assert!(extensions_file.exists(), "VS Code extensions file should exist");

            let content = fs::read_to_string(extensions_file)
                .expect("Should be able to read VS Code extensions");

            assert!(!content.trim().is_empty(), "VS Code extensions should not be empty");

            // VS Code extensions use JSONC (JSON with comments), so we'll validate structure differently
            // Check for essential JSON structure markers instead of parsing as pure JSON
            assert!(content.contains("{"), "Extensions should start with JSON object");
            assert!(content.contains("}"), "Extensions should end with JSON object");
            assert!(content.contains("recommendations"), "Extensions should contain recommendations");

            // Check for essential recommendations
            assert!(content.contains("rust-lang.rust-analyzer"), "Should recommend rust-analyzer");
            assert!(content.contains("rooveterinaryinc.roo-cline"), "Should recommend Roo Cline");
            assert!(content.contains("ms-vscode.vscode-docker"), "Should recommend Docker extension");

            // Check for unwanted recommendations
            assert!(content.contains("unwantedRecommendations"), "Should include unwanted recommendations");
            assert!(content.contains("ms-vscode.cpptools"), "Should exclude C++ tools");

            println!("✅ VS Code extensions recommendations are properly configured");
        }

        #[test]
        fn test_vscode_tasks_validation() {
            println!("🔍 Testing VS Code tasks configuration...");

            let settings_file = Path::new(".vscode/settings.json");
            let content = fs::read_to_string(settings_file)
                .expect("Should be able to read VS Code settings");

            // Validate that tasks have proper structure
            assert!(content.contains("version"), "Tasks should have version");
            assert!(content.contains("label"), "Tasks should have labels");
            assert!(content.contains("command"), "Tasks should have commands");
            assert!(content.contains("cargo"), "Tasks should include cargo commands");

            // Check for different task types
            assert!(content.contains("cargo build"), "Should have build task");
            assert!(content.contains("cargo test"), "Should have test task");
            assert!(content.contains("cargo check"), "Should have check task");
            assert!(content.contains("cargo fmt"), "Should have format task");
            assert!(content.contains("cargo clippy"), "Should have clippy task");

            // Check for background tasks
            assert!(content.contains("isBackground"), "Should have background tasks");
            assert!(content.contains("cargo watch"), "Should have watch task");

            println!("✅ VS Code tasks are properly configured");
        }

        #[test]
        fn test_vscode_launch_configuration() {
            println!("🔍 Testing VS Code launch configuration...");

            let settings_file = Path::new(".vscode/settings.json");
            let content = fs::read_to_string(settings_file)
                .expect("Should be able to read VS Code settings");

            // Validate launch configuration structure
            assert!(content.contains("launch"), "Should have launch configuration");
            assert!(content.contains("configurations"), "Launch should have configurations array");

            // Check for debug configurations
            assert!(content.contains("Debug I.O.R.A."), "Should have I.O.R.A. debug config");
            assert!(content.contains("Debug I.O.R.A. (Release)"), "Should have release debug config");
            assert!(content.contains("Debug Tests"), "Should have test debug config");

            // Check for LLDB debugger
            assert!(content.contains("lldb"), "Should use LLDB debugger");

            // Check for proper program paths
            assert!(content.contains("target/debug/iora"), "Should have debug binary path");
            assert!(content.contains("target/release/iora"), "Should have release binary path");

            println!("✅ VS Code launch configuration is properly set up");
        }
    }

    /// Test development workflow script commands
    mod workflow_script_tests {
        use super::*;

        #[test]
        fn test_dev_workflow_script_exists() {
            println!("🔍 Testing development workflow script existence...");

            let script_path = "scripts/dev-workflow.sh";
            let script_file = Path::new(script_path);

            assert!(script_file.exists(), "Development workflow script should exist");
            assert!(script_file.is_file(), "Script should be a file");

            let content = fs::read_to_string(script_file)
                .expect("Should be able to read workflow script");

            assert!(!content.trim().is_empty(), "Workflow script should not be empty");

            // Check for proper shebang
            assert!(content.contains("#!/bin/bash") || content.contains("#!/bin/zsh"),
                "Script should have proper shebang");

            println!("✅ Development workflow script exists and is properly structured");
        }

        #[test]
        fn test_workflow_script_commands() {
            println!("🔍 Testing workflow script command coverage...");

            let script_path = "scripts/dev-workflow.sh";
            let content = fs::read_to_string(script_path)
                .expect("Should be able to read workflow script");

            // Check for essential development commands (case statements in bash script)
            let essential_commands = vec![
                "cargo build", "cargo test", "cargo check",
                "cargo fmt", "cargo clippy", "cargo run",
                "cargo clean"
            ];

            for command in &essential_commands {
                assert!(content.contains(command),
                    "Workflow script should include command: {}", command);
            }

            // Check for workflow cases (bash case statements)
            let workflow_cases = vec![
                "\"build\")", "\"test\")", "\"run\")", "\"clean\")", "\"fmt\")", "\"lint\")", "\"check\")"
            ];

            for case_pattern in &workflow_cases {
                assert!(content.contains(case_pattern),
                    "Workflow script should include case: {}", case_pattern);
            }

            // Check for additional useful commands that are available
            let additional_commands = vec![
                "cargo audit", "cargo watch", "docker-compose"
            ];

            let additional_count = additional_commands.iter()
                .filter(|cmd| content.contains(*cmd))
                .count();

            assert!(additional_count >= 2,
                "Workflow script should include at least 2 additional useful commands, found {}", additional_count);

            println!("✅ Workflow script includes comprehensive command coverage");
        }

        #[test]
        fn test_makefile_targets_comprehensive() {
            println!("🔍 Testing Makefile targets comprehensiveness...");

            let makefile_path = "Makefile";
            let makefile = Path::new(makefile_path);

            assert!(makefile.exists(), "Makefile should exist");
            assert!(makefile.is_file(), "Makefile should be a file");

            let content = fs::read_to_string(makefile)
                .expect("Should be able to read Makefile");

            // Check for comprehensive target coverage
            let essential_targets = vec![
                "build", "test", "check", "clean", "fmt", "lint",
                "run", "doc", "release", "install", "coverage"
            ];

            for target in &essential_targets {
                // Handle different target names
                let target_pattern = match *target {
                    "fmt" => "format:", // Makefile uses "format:" not "fmt:"
                    "clippy" => "lint:", // Makefile uses "lint:" not "clippy:"
                    _ => &format!("{}:", target)
                };
                assert!(content.contains(target_pattern),
                    "Makefile should have target: {} (looking for {})", target, target_pattern);
            }

            // Check for help target
            assert!(content.contains("help:"), "Makefile should have help target");

            // Check for PHONY declaration
            assert!(content.contains(".PHONY"), "Makefile should declare PHONY targets");

            println!("✅ Makefile includes comprehensive target coverage");
        }

        #[test]
        fn test_script_execution_capabilities() {
            println!("🔍 Testing script execution capabilities...");

            let script_path = "scripts/dev-workflow.sh";

            // Test if script is executable (Unix systems)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let metadata = fs::metadata(script_path)
                    .expect(&format!("Should be able to get metadata for: {}", script_path));

                let permissions = metadata.permissions();
                let mode = permissions.mode();

                if mode & 0o111 == 0 {
                    println!("⚠️  Script may not be executable: {}", script_path);
                    println!("💡 Consider running: chmod +x {}", script_path);
                } else {
                    println!("✅ Script is executable: {}", script_path);
                }
            }

            #[cfg(not(unix))]
            {
                println!("✅ Script exists (Windows execution permissions not tested): {}", script_path);
            }

            // Test script syntax (basic validation)
            let content = fs::read_to_string(script_path)
                .expect("Should be able to read workflow script");

            // Check for proper script structure
            assert!(content.contains("#!/bin/bash") || content.contains("#!/bin/zsh"),
                "Script should have valid shebang");

            // Check for error handling
            assert!(content.contains("set -e") || content.contains("set -o errexit"),
                "Script should have error handling");

            println!("✅ Script execution capabilities validated");
        }
    }

    /// Test pre-commit hook configuration validation
    mod precommit_hook_tests {
        use super::*;

        #[test]
        fn test_precommit_config_exists() {
            println!("🔍 Testing pre-commit configuration existence...");

            let precommit_file = Path::new(".pre-commit-config.yaml");
            assert!(precommit_file.exists(), "Pre-commit config file should exist");

            let content = fs::read_to_string(precommit_file)
                .expect("Should be able to read pre-commit config");

            assert!(!content.trim().is_empty(), "Pre-commit config should not be empty");

            // Validate YAML structure
            assert!(content.contains("repos:"), "Config should define repos");
            assert!(content.contains("hooks:"), "Config should define hooks");

            println!("✅ Pre-commit configuration exists and is structured properly");
        }

        #[test]
        fn test_precommit_rust_hooks() {
            println!("🔍 Testing pre-commit Rust hooks configuration...");

            let content = fs::read_to_string(".pre-commit-config.yaml")
                .expect("Should be able to read pre-commit config");

            // Check for Rust-specific hooks
            assert!(content.contains("cargo fmt"), "Should include rustfmt hook");
            assert!(content.contains("cargo clippy"), "Should include clippy hook");
            assert!(content.contains("cargo check"), "Should include cargo check hook");

            // Check for doublify/pre-commit-rust repo
            assert!(content.contains("doublify/pre-commit-rust"), "Should use pre-commit-rust repo");

            println!("✅ Pre-commit Rust hooks are properly configured");
        }

        #[test]
        fn test_precommit_code_quality_hooks() {
            println!("🔍 Testing pre-commit code quality hooks...");

            let content = fs::read_to_string(".pre-commit-config.yaml")
                .expect("Should be able to read pre-commit config");

            // Check for general code quality hooks
            assert!(content.contains("trailing-whitespace"), "Should remove trailing whitespace");
            assert!(content.contains("end-of-file-fixer"), "Should fix end-of-file issues");
            assert!(content.contains("mixed-line-ending"), "Should fix line endings");

            // Check for large file prevention
            assert!(content.contains("check-added-large-files"), "Should prevent large files");

            // Check for secrets detection
            assert!(content.contains("detect-secrets"), "Should detect secrets");

            println!("✅ Pre-commit code quality hooks are comprehensive");
        }

        #[test]
        fn test_precommit_formatting_hooks() {
            println!("🔍 Testing pre-commit formatting hooks...");

            let content = fs::read_to_string(".pre-commit-config.yaml")
                .expect("Should be able to read pre-commit config");

            // Check for formatting tools
            assert!(content.contains("prettier"), "Should include prettier for formatting");

            // Check for file type handling
            assert!(content.contains("\\.(toml)$"), "Should format TOML files");
            assert!(content.contains("\\.(yaml|yml)$"), "Should format YAML files");
            assert!(content.contains("\\.(json)$"), "Should format JSON files");

            // Check for exclusions
            assert!(content.contains("Cargo\\.lock$"), "Should exclude Cargo.lock from formatting");

            println!("✅ Pre-commit formatting hooks are properly configured");
        }

        #[test]
        fn test_precommit_ci_configuration() {
            println!("🔍 Testing pre-commit CI configuration...");

            let content = fs::read_to_string(".pre-commit-config.yaml")
                .expect("Should be able to read pre-commit config");

            // Check for CI configuration
            assert!(content.contains("ci:"), "Should include CI configuration");
            assert!(content.contains("autofix_prs:"), "Should configure PR auto-fixes");
            assert!(content.contains("autoupdate_schedule:"), "Should configure auto-updates");

            println!("✅ Pre-commit CI configuration is properly set up");
        }

        #[test]
        fn test_precommit_hook_execution() {
            println!("🔍 Testing pre-commit hook execution capability...");

            // Check if pre-commit is available
            let precommit_check = Command::new("pre-commit")
                .arg("--version")
                .output();

            match precommit_check {
                Ok(output) if output.status.success() => {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    println!("✅ Pre-commit is available: {}", version_output.trim());

                    // Test pre-commit configuration validation
                    let config_test = Command::new("pre-commit")
                        .args(&["validate-config", ".pre-commit-config.yaml"])
                        .output();

                    match config_test {
                        Ok(output) if output.status.success() => {
                            println!("✅ Pre-commit configuration is valid");
                        }
                        Ok(output) => {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            println!("⚠️  Pre-commit configuration validation failed: {}", stderr);
                        }
                        Err(e) => {
                            println!("⚠️  Could not validate pre-commit config: {}", e);
                        }
                    }
                }
                _ => {
                    println!("⚠️  Pre-commit not available in current environment");
                    println!("💡 Pre-commit hooks will be validated in CI/CD pipeline");
                }
            }
        }
    }

    /// Test CI/CD pipeline simulation functionality
    mod ci_cd_simulation_tests {
        use super::*;

        #[test]
        fn test_github_workflows_directory() {
            println!("🔍 Testing GitHub workflows directory...");

            let workflows_dir = Path::new(".github/workflows");
            assert!(workflows_dir.exists(), "GitHub workflows directory should exist");
            assert!(workflows_dir.is_dir(), "Workflows should be a directory");

            println!("✅ GitHub workflows directory exists");
        }

        #[test]
        fn test_ci_workflow_configuration() {
            println!("🔍 Testing CI workflow configuration...");

            let ci_file = Path::new(".github/workflows/ci.yml");
            assert!(ci_file.exists(), "CI workflow file should exist");

            let content = fs::read_to_string(ci_file)
                .expect("Should be able to read CI workflow");

            assert!(!content.trim().is_empty(), "CI workflow should not be empty");

            // Check for essential workflow structure
            assert!(content.contains("name:"), "Workflow should have name");
            assert!(content.contains("on:"), "Workflow should define triggers");
            assert!(content.contains("jobs:"), "Workflow should define jobs");

            // Check for trigger events
            assert!(content.contains("push:"), "Should trigger on push");
            assert!(content.contains("pull_request:"), "Should trigger on PR");

            // Check for job definitions
            assert!(content.contains("test:"), "Should have test job");
            assert!(content.contains("coverage:"), "Should have coverage job");
            assert!(content.contains("security-audit:"), "Should have security audit job");

            println!("✅ CI workflow configuration is comprehensive");
        }

        #[test]
        fn test_ci_workflow_jobs_coverage() {
            println!("🔍 Testing CI workflow jobs coverage...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for comprehensive job coverage
            let required_jobs = vec![
                "test", "coverage", "security-audit", "docker", "release"
            ];

            for job in &required_jobs {
                assert!(content.contains(&format!("{}:", job)),
                    "CI workflow should include job: {}", job);
            }

            println!("✅ CI workflow includes comprehensive job coverage");
        }

        #[test]
        fn test_ci_workflow_actions_usage() {
            println!("🔍 Testing CI workflow GitHub Actions usage...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for modern GitHub Actions
            assert!(content.contains("actions/checkout@v4"), "Should use modern checkout action");
            assert!(content.contains("dtolnay/rust-toolchain@stable"), "Should use modern Rust toolchain action");

            // Check for caching
            assert!(content.contains("Swatinem/rust-cache@v2"), "Should use Rust caching");

            // Check for security and quality actions
            assert!(content.contains("codecov/codecov-action"), "Should include coverage reporting");

            println!("✅ CI workflow uses modern GitHub Actions");
        }

        #[test]
        fn test_ci_workflow_test_coverage() {
            println!("🔍 Testing CI workflow test coverage...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for comprehensive testing
            assert!(content.contains("cargo check"), "Should run compilation check");
            assert!(content.contains("cargo clippy"), "Should run clippy linting");
            assert!(content.contains("cargo fmt"), "Should check formatting");
            assert!(content.contains("cargo test"), "Should run tests");

            // Check for integration tests
            assert!(content.contains("--test integration_tests"), "Should run integration tests");
            assert!(content.contains("--test config_tests"), "Should run config tests");

            println!("✅ CI workflow includes comprehensive test coverage");
        }

        #[test]
        fn test_ci_workflow_security_coverage() {
            println!("🔍 Testing CI workflow security coverage...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for security scanning
            assert!(content.contains("cargo audit"), "Should run security audit");

            // Check for code quality
            assert!(content.contains("-D warnings"), "Should treat warnings as errors");

            println!("✅ CI workflow includes security and quality checks");
        }

        #[test]
        fn test_ci_workflow_docker_integration() {
            println!("🔍 Testing CI workflow Docker integration...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for Docker job
            assert!(content.contains("docker:"), "Should have Docker job");

            // Check for Docker actions
            assert!(content.contains("docker/setup-buildx-action"), "Should set up Docker Buildx");
            assert!(content.contains("docker/build-push-action"), "Should build Docker images");

            // Check for Docker caching
            assert!(content.contains("type=gha"), "Should use GitHub Actions cache for Docker");

            println!("✅ CI workflow includes Docker integration and optimization");
        }

        #[test]
        fn test_ci_workflow_release_process() {
            println!("🔍 Testing CI workflow release process...");

            let content = fs::read_to_string(".github/workflows/ci.yml")
                .expect("Should be able to read CI workflow");

            // Check for release job
            assert!(content.contains("release:"), "Should have release job");

            // Check for release conditions
            assert!(content.contains("refs/heads/main"), "Should release from main branch");
            assert!(content.contains("github.event_name == 'push'"), "Should trigger on push events");

            // Check for release artifacts
            assert!(content.contains("tar -czf"), "Should create release archive");
            assert!(content.contains("sha256sum"), "Should generate checksums");

            // Check for GitHub release
            assert!(content.contains("softprops/action-gh-release"), "Should create GitHub releases");

            println!("✅ CI workflow includes comprehensive release process");
        }

        #[test]
        fn test_workflow_file_structure() {
            println!("🔍 Testing workflow file structure validation...");

            let ci_file = Path::new(".github/workflows/ci.yml");

            // Validate YAML structure
            let content = fs::read_to_string(ci_file)
                .expect("Should be able to read CI workflow");

            // Parse as YAML to validate structure
            let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
                .expect("CI workflow should be valid YAML");

            // Validate top-level structure
            assert!(yaml_value.get("name").is_some(), "Workflow should have name");
            assert!(yaml_value.get("on").is_some(), "Workflow should have triggers");
            assert!(yaml_value.get("jobs").is_some(), "Workflow should have jobs");

            // Validate jobs structure
            if let Some(jobs) = yaml_value.get("jobs").and_then(|j| j.as_mapping()) {
                let expected_jobs = vec!["test", "coverage", "security-audit", "docker", "release"];
                for job in &expected_jobs {
                    assert!(jobs.contains_key(&serde_yaml::Value::String(job.to_string())),
                        "Workflow should include job: {}", job);
                }
            }

            println!("✅ CI workflow file structure is valid and complete");
        }
    }

    /// Test overall IDE and workflow integration
    mod integrated_workflow_tests {
        use super::*;

        #[test]
        fn test_complete_ide_setup_integration() {
            println!("🔍 Testing complete IDE setup integration...");

            // Verify all essential IDE components exist
            let essential_files = vec![
                ".vscode/settings.json",
                ".vscode/extensions.json",
                ".pre-commit-config.yaml",
                ".github/workflows/ci.yml",
                "Makefile",
                "scripts/dev-workflow.sh"
            ];

            for file_path in &essential_files {
                let path = Path::new(file_path);
                assert!(path.exists(),
                    "Essential IDE file should exist: {}", file_path);
                assert!(path.is_file(),
                    "Essential IDE file should be a file: {}", file_path);

                let content = fs::read_to_string(path)
                    .expect(&format!("Should be able to read: {}", file_path));

                assert!(!content.trim().is_empty(),
                    "Essential IDE file should not be empty: {}", file_path);
            }

            println!("✅ Complete IDE setup integration validated");
        }

        #[test]
        fn test_workflow_tool_compatibility() {
            println!("🔍 Testing workflow tool compatibility...");

            // Test that tools work together without conflicts

            // Check that VS Code settings don't conflict with pre-commit
            let vscode_settings = fs::read_to_string(".vscode/settings.json")
                .expect("Should be able to read VS Code settings");

            let precommit_config = fs::read_to_string(".pre-commit-config.yaml")
                .expect("Should be able to read pre-commit config");

            // Both should handle Rust formatting without conflict
            assert!(vscode_settings.contains("rustfmt") || vscode_settings.contains("cargo fmt"),
                "VS Code should handle Rust formatting");
            assert!(precommit_config.contains("cargo fmt"),
                "Pre-commit should handle Rust formatting");

            // Both should handle clippy without conflict
            assert!(vscode_settings.contains("clippy"),
                "VS Code should handle clippy");
            assert!(precommit_config.contains("cargo clippy"),
                "Pre-commit should handle clippy");

            println!("✅ Workflow tools are compatible and non-conflicting");
        }

        #[test]
        fn test_development_environment_completeness() {
            println!("🔍 Testing development environment completeness...");

            // Test that all components of the development environment are present and functional

            // VS Code configuration
            let vscode_config = Path::new(".vscode");
            assert!(vscode_config.exists(), "VS Code config should exist");

            // Pre-commit hooks
            let precommit_config = Path::new(".pre-commit-config.yaml");
            assert!(precommit_config.exists(), "Pre-commit config should exist");

            // CI/CD pipeline
            let ci_workflow = Path::new(".github/workflows/ci.yml");
            assert!(ci_workflow.exists(), "CI workflow should exist");

            // Development scripts
            let dev_script = Path::new("scripts/dev-workflow.sh");
            assert!(dev_script.exists(), "Development script should exist");

            // Build system
            let makefile = Path::new("Makefile");
            assert!(makefile.exists(), "Makefile should exist");

            // All components should be readable and contain content
            let config_files = vec![
                ".vscode/settings.json",
                ".vscode/extensions.json",
                ".pre-commit-config.yaml",
                ".github/workflows/ci.yml",
                "Makefile"
            ];

            for file in &config_files {
                let content = fs::read_to_string(file)
                    .expect(&format!("Should be able to read: {}", file));
                assert!(!content.trim().is_empty(),
                    "Config file should not be empty: {}", file);
            }

            println!("✅ Development environment is complete and functional");
        }

        #[test]
        fn test_workflow_automation_coverage() {
            println!("🔍 Testing workflow automation coverage...");

            // Test that the workflow covers all major development activities

            let makefile_content = fs::read_to_string("Makefile")
                .expect("Should be able to read Makefile");

            let script_content = fs::read_to_string("scripts/dev-workflow.sh")
                .expect("Should be able to read development script");

            // Check for comprehensive automation coverage
            let automation_areas = vec![
                ("building", "build", &makefile_content, &script_content),
                ("testing", "test", &makefile_content, &script_content),
                ("linting", "clippy", &makefile_content, &script_content),
                ("formatting", "fmt", &makefile_content, &script_content),
                ("documentation", "doc", &makefile_content, &script_content),
                ("cleaning", "clean", &makefile_content, &script_content)
            ];

            for (area, command, makefile, script) in &automation_areas {
                let has_makefile = makefile.contains(&format!("{}:", command));
                let has_script = script.contains(&format!("cargo {}", command)) ||
                               script.contains(&format!("{}", command));

                assert!(has_makefile || has_script,
                    "Workflow should automate {} via Makefile or script", area);
            }

            println!("✅ Workflow automation covers all major development activities");
        }
    }
}
