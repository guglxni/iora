#!/bin/bash

# I.O.R.A. Development Workflow Script
# Provides convenient commands for development tasks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}[I.O.R.A.]${NC} $1"
}

# Check if we're in the right directory
check_project_root() {
    if [ ! -f "Cargo.toml" ]; then
        print_error "Please run this script from the project root directory (where Cargo.toml is located)"
        exit 1
    fi
}

# Development workflow commands
case "${1:-help}" in
    "build")
        print_header "Building I.O.R.A. project..."
        cargo build
        print_status "Build completed successfully!"
        ;;

    "run")
        print_header "Running I.O.R.A. application..."
        cargo run
        ;;

    "test")
        print_header "Running test suite..."
        cargo test
        print_status "All tests completed!"
        ;;

    "test-watch")
        print_header "Running tests in watch mode..."
        print_status "Press Ctrl+C to stop watching"
        cargo watch -x test
        ;;

    "check")
        print_header "Running cargo check..."
        cargo check
        print_status "Check completed successfully!"
        ;;

    "fmt")
        print_header "Formatting code..."
        cargo fmt
        print_status "Code formatting completed!"
        ;;

    "lint")
        print_header "Running clippy linter..."
        cargo clippy -- -D warnings
        print_status "Linting completed successfully!"
        ;;

    "fix")
        print_header "Auto-fixing code issues..."
        cargo fix --allow-dirty
        cargo fmt
        print_status "Auto-fix completed!"
        ;;

    "clean")
        print_header "Cleaning build artifacts..."
        cargo clean
        print_status "Clean completed!"
        ;;

    "audit")
        print_header "Running security audit..."
        cargo audit
        print_status "Security audit completed!"
        ;;

    "coverage")
        print_header "Generating test coverage report..."
        cargo tarpaulin --ignore-tests --out Html
        print_status "Coverage report generated in tarpaulin-report.html"
        ;;

    "watch")
        print_header "Starting development watch mode..."
        print_status "Watching for file changes. Press Ctrl+C to stop."
        cargo watch -x check
        ;;

    "ci")
        print_header "Running CI pipeline simulation..."
        echo "Step 1/6: Formatting check..."
        cargo fmt --all -- --check
        echo "Step 2/6: Clippy linting..."
        cargo clippy -- -D warnings
        echo "Step 3/6: Building project..."
        cargo build
        echo "Step 4/6: Running tests..."
        cargo test
        echo "Step 5/6: Security audit..."
        cargo audit
        echo "Step 6/6: Generating coverage..."
        cargo tarpaulin --ignore-tests --out Html
        print_status "CI pipeline completed successfully! ✅"
        ;;

    "docker-up")
        print_header "Starting Docker services..."
        docker-compose up -d
        print_status "Docker services started!"
        ;;

    "docker-down")
        print_header "Stopping Docker services..."
        docker-compose down
        print_status "Docker services stopped!"
        ;;

    "docker-logs")
        print_header "Showing Docker service logs..."
        docker-compose logs -f
        ;;

    "solana-status")
        print_header "Checking Solana development environment..."
        echo "Solana CLI version:"
        solana --version
        echo ""
        echo "Current configuration:"
        solana config get
        echo ""
        echo "Wallet balance:"
        solana balance
        ;;

    "typesense-status")
        print_header "Checking Typesense status..."
        if curl -s -f -H "X-TYPESENSE-API-KEY: iora_dev_typesense_key_2024" http://localhost:8108/health > /dev/null 2>&1; then
            echo "✅ Typesense is running and healthy"
            echo "🌐 Dashboard: http://localhost:8108"
        else
            echo "❌ Typesense is not responding"
            echo "💡 Try running: ./scripts/dev-workflow.sh docker-up"
        fi
        ;;

    "env-check")
        print_header "Checking environment configuration..."
        if [ -f ".env" ]; then
            echo "✅ .env file exists"
            echo "Environment variables:"
            grep -E "^[A-Z_]+" .env | head -10
            if [ $(grep -c "^[A-Z_]+" .env) -gt 10 ]; then
                echo "... and $(($(grep -c "^[A-Z_]+" .env) - 10)) more"
            fi
        else
            echo "❌ .env file not found"
            echo "💡 Create one based on .env.example"
        fi
        ;;

    "setup")
        print_header "Setting up complete development environment..."
        print_status "This will install all required tools and configure the environment"

        # Run the comprehensive setup script
        if [ -f "scripts/install-all-tools.sh" ]; then
            bash scripts/install-all-tools.sh
        else
            print_error "Setup script not found. Run from project root."
            exit 1
        fi
        ;;

    "status")
        print_header "Development Environment Status"
        echo ""
        echo "📦 Project Information:"
        echo "  Project: $(grep '^name' Cargo.toml | cut -d'"' -f2)"
        echo "  Version: $(grep '^version' Cargo.toml | cut -d'"' -f2)"
        echo "  Rust: $(rustc --version | cut -d' ' -f2)"
        echo ""
        echo "🔧 Development Tools:"
        echo "  cargo-watch: $(cargo watch --version 2>/dev/null | head -1 || echo 'Not installed')"
        echo "  cargo-tarpaulin: $(cargo tarpaulin --version 2>/dev/null | head -1 || echo 'Not installed')"
        echo "  cargo-audit: $(cargo audit --version 2>/dev/null | head -1 || echo 'Not installed')"
        echo ""
        echo "🐳 Services Status:"
        if docker-compose ps | grep -q "iora-typesense"; then
            echo "  Typesense: ✅ Running"
        else
            echo "  Typesense: ❌ Stopped"
        fi
        echo ""
        echo "⚙️ Configuration:"
        if [ -f ".env" ]; then
            echo "  Environment: ✅ Configured"
        else
            echo "  Environment: ❌ Missing .env file"
        fi
        ;;

    "help"|*)
        print_header "I.O.R.A. Development Workflow Commands"
        echo ""
        echo "BUILD & RUN:"
        echo "  build        - Build the project"
        echo "  run          - Run the application"
        echo "  check        - Run cargo check (fast compilation check)"
        echo ""
        echo "TESTING:"
        echo "  test         - Run all tests"
        echo "  test-watch   - Run tests in watch mode"
        echo "  coverage     - Generate test coverage report"
        echo ""
        echo "CODE QUALITY:"
        echo "  fmt          - Format code with rustfmt"
        echo "  lint         - Run clippy linter"
        echo "  fix          - Auto-fix code issues"
        echo "  audit        - Run security audit"
        echo ""
        echo "DEVELOPMENT:"
        echo "  watch        - Watch for file changes and run checks"
        echo "  clean        - Clean build artifacts"
        echo "  ci           - Run full CI pipeline simulation"
        echo ""
        echo "SERVICES:"
        echo "  docker-up    - Start Docker services"
        echo "  docker-down  - Stop Docker services"
        echo "  docker-logs  - Show Docker service logs"
        echo ""
        echo "STATUS CHECKS:"
        echo "  status       - Show development environment status"
        echo "  solana-status - Check Solana development setup"
        echo "  typesense-status - Check Typesense service status"
        echo "  env-check    - Verify environment configuration"
        echo ""
        echo "SETUP:"
        echo "  setup        - Run complete development environment setup"
        echo ""
        echo "USAGE: $0 <command>"
        echo "Example: $0 build"
        ;;
esac
