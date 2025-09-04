#!/bin/bash

# I.O.R.A. Development Environment Setup Script
# This script sets up the development environment with all necessary tools

set -e

echo "🚀 Setting up I.O.R.A. development environment..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Rust if not present
if ! command_exists rustc; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install required Rust components
echo "🔧 Installing Rust components..."
rustup component add rustfmt
rustup component add clippy

# Install cargo tools
echo "📦 Installing cargo tools..."
cargo install cargo-tarpaulin
cargo install cargo-audit
cargo install cargo-watch

# Install pre-commit if not present
if ! command_exists pre-commit; then
    echo "📦 Installing pre-commit..."
    if command_exists pip3; then
        pip3 install pre-commit
    elif command_exists pip; then
        pip install pre-commit
    else
        echo "⚠️  Warning: pip not found. Please install pre-commit manually:"
        echo "   pip install pre-commit"
    fi
fi

# Setup pre-commit hooks
if command_exists pre-commit; then
    echo "🔗 Setting up pre-commit hooks..."
    pre-commit install
    pre-commit install --hook-type commit-msg
else
    echo "⚠️  Skipping pre-commit setup (pre-commit not found)"
fi

# Install Docker if not present (optional)
if ! command_exists docker; then
    echo "⚠️  Docker not found. Please install Docker Desktop for full functionality:"
    echo "   https://www.docker.com/products/docker-desktop"
fi

# Setup environment file
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file from template..."
    if [ -f ".env.example" ]; then
        cp .env.example .env
        echo "✅ Created .env file. Please update with your actual values."
    else
        echo "⚠️  .env.example not found"
    fi
fi

# Run initial checks
echo "🔍 Running initial project checks..."

# Check Rust code
echo "Checking Rust code..."
cargo check

# Format code
echo "Formatting code..."
cargo fmt

# Run lints
echo "Running clippy..."
cargo clippy

# Run tests
echo "Running tests..."
cargo test

echo "✅ Development environment setup complete!"
echo ""
echo "🎯 Next steps:"
echo "  1. Update .env file with your actual configuration"
echo "  2. Run 'cargo run' to start the application"
echo "  3. Run 'cargo test' to run all tests"
echo "  4. Run 'pre-commit run --all-files' to check all files"
echo ""
echo "📚 Useful commands:"
echo "  • cargo build          - Build the project"
echo "  • cargo test           - Run all tests"
echo "  • cargo fmt            - Format code"
echo "  • cargo clippy         - Run lints"
echo "  • cargo tarpaulin      - Generate coverage report"
echo "  • pre-commit run       - Run pre-commit hooks"
