# I.O.R.A. Development Makefile
# Provides convenient commands for development, testing, and CI/CD

.PHONY: help setup build test lint format coverage audit clean docker-build docker-run

# Default target
help: ## Show this help message
	@echo "I.O.R.A. Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Setup development environment
setup: ## Set up development environment with all tools
	@echo "🚀 Setting up development environment..."
	@./scripts/setup-dev.sh

# Build commands
build: ## Build the project in debug mode
	@echo "🔨 Building project..."
	@cargo build

build-release: ## Build the project in release mode
	@echo "🔨 Building project (release)..."
	@cargo build --release

# Testing commands
test: ## Run all tests
	@echo "🧪 Running all tests..."
	@cargo test

test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	@cargo test --lib

test-integration: ## Run integration tests only
	@echo "🧪 Running integration tests..."
	@cargo test --test integration_tests

test-config: ## Run configuration tests only
	@echo "🧪 Running configuration tests..."
	@cargo test --test config_tests

test-watch: ## Run tests in watch mode
	@echo "👀 Running tests in watch mode..."
	@cargo watch -x test

# Code quality commands
lint: ## Run clippy linter
	@echo "🔍 Running clippy..."
	@cargo clippy -- -D warnings

format: ## Format code with rustfmt
	@echo "🎨 Formatting code..."
	@cargo fmt

format-check: ## Check code formatting
	@echo "🔍 Checking code formatting..."
	@cargo fmt --all -- --check

# Coverage commands
coverage: ## Generate test coverage report
	@echo "📊 Generating coverage report..."
	@cargo tarpaulin --ignore-tests

coverage-html: ## Generate HTML coverage report
	@echo "📊 Generating HTML coverage report..."
	@cargo tarpaulin --ignore-tests --out Html
	@echo "📂 Open coverage/tarpaulin-report.html in your browser"

# Security audit
audit: ## Run security audit
	@echo "🔒 Running security audit..."
	@cargo audit

# Pre-commit hooks
pre-commit: ## Run pre-commit hooks on all files
	@echo "🔗 Running pre-commit hooks..."
	@pre-commit run --all-files

pre-commit-install: ## Install pre-commit hooks
	@echo "🔗 Installing pre-commit hooks..."
	@pre-commit install
	@pre-commit install --hook-type commit-msg

# Docker commands
docker-build: ## Build Docker image
	@echo "🐳 Building Docker image..."
	@docker build -t iora .

docker-run: ## Run Docker container
	@echo "🐳 Running Docker container..."
	@docker run --rm -it iora

docker-compose-up: ## Start services with docker-compose
	@echo "🐳 Starting services..."
	@docker-compose up -d

docker-compose-down: ## Stop services with docker-compose
	@echo "🐳 Stopping services..."
	@docker-compose down

# Check commands
check: ## Run cargo check
	@echo "🔍 Running cargo check..."
	@cargo check

check-all: ## Run all checks (check, test, lint, format)
	@echo "🔍 Running all checks..."
	@cargo check
	@cargo test
	@cargo clippy -- -D warnings
	@cargo fmt --all -- --check

# Clean commands
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

clean-all: ## Clean all artifacts including coverage and target
	@echo "🧹 Cleaning all artifacts..."
	@cargo clean
	@rm -rf coverage/
	@rm -f *.tar.gz *.sha256

# CI simulation
ci: ## Simulate CI pipeline locally
	@echo "🔄 Simulating CI pipeline..."
	@make check-all
	@make coverage
	@make audit

# Development helpers
run: ## Run the application
	@echo "🚀 Running application..."
	@cargo run

run-release: ## Run the application in release mode
	@echo "🚀 Running application (release)..."
	@cargo run --release

watch: ## Run in watch mode (requires cargo-watch)
	@echo "👀 Running in watch mode..."
	@cargo watch -x run

# Documentation
doc: ## Generate documentation
	@echo "📚 Generating documentation..."
	@cargo doc --open

# Release helpers
release-prep: ## Prepare for release
	@echo "📦 Preparing for release..."
	@cargo test
	@cargo build --release
	@cargo clippy -- -D warnings
	@cargo fmt --all -- --check

# Utility commands
deps-tree: ## Show dependency tree
	@echo "📦 Showing dependency tree..."
	@cargo tree

deps-update: ## Update dependencies
	@echo "📦 Updating dependencies..."
	@cargo update

# Help for specific targets
help-%: ## Show help for a specific target
	@grep -E "^$*:.*?## .*$$" $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
