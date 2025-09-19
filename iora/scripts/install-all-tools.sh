#!/bin/bash

# Complete Development Environment Setup for I.O.R.A. Project
# This script installs all required tools and sets up the development environment

set -e

echo "🚀 Setting up complete I.O.R.A. development environment..."
echo "This will install: Rust, Solana CLI, Anchor, and Typesense"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# 1. Install Rust toolchain
echo "=== Step 1: Installing Rust Toolchain ==="
if [ -f "scripts/install-rust.sh" ]; then
    bash scripts/install-rust.sh
else
    echo "❌ Rust installation script not found"
    exit 1
fi

# Source cargo environment if it exists
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

# 2. Install Solana CLI and Anchor
echo ""
echo "=== Step 2: Installing Solana CLI and Anchor ==="
if [ -f "scripts/install-solana.sh" ]; then
    bash scripts/install-solana.sh
else
    echo "❌ Solana installation script not found"
    exit 1
fi

# 3. Setup Typesense
echo ""
echo "=== Step 3: Setting up self-hosted Typesense ==="
if [ -f "scripts/setup-typesense.sh" ]; then
    bash scripts/setup-typesense.sh
else
    echo "❌ Typesense setup script not found"
    exit 1
fi

# 4. Verify installations
echo ""
echo "=== Step 4: Verifying Installations ==="

echo "🔍 Checking Rust installation..."
if command_exists rustc && command_exists cargo; then
    echo "✅ Rust: $(rustc --version)"
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Rust installation failed"
fi

echo ""
echo "🔍 Checking Solana installation..."
if command_exists solana; then
    echo "✅ Solana CLI: $(solana --version)"
else
    echo "❌ Solana CLI installation failed"
fi

if command_exists anchor; then
    echo "✅ Anchor: $(anchor --version)"
else
    echo "⚠️ Anchor installation may have failed (Node.js required)"
fi

echo ""
echo "🔍 Checking Docker services..."
if command_exists docker && command_exists docker-compose; then
    echo "✅ Docker: $(docker --version)"
    echo "✅ Docker Compose: $(docker-compose --version)"

    # Check if Typesense is running
    if curl -s -f -H "X-TYPESENSE-API-KEY: iora_dev_typesense_key_2024" \
        "http://localhost:8108/health" > /dev/null 2>&1; then
        echo "✅ Typesense: Running on localhost:8108"
    else
        echo "❌ Typesense: Not responding"
    fi
else
    echo "❌ Docker/Docker Compose not found"
fi

# 5. Create environment file if it doesn't exist
echo ""
echo "=== Step 5: Setting up Environment Configuration ==="
if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        cp .env.example .env
        echo "✅ Created .env file from template"
        echo "⚠️ Please update .env with your actual API keys:"
        echo "   • GEMINI_API_KEY"
        echo "   • SOLANA_RPC_URL (currently set to Devnet)"
        echo "   • TYPESENSE_API_KEY (set to: iora_dev_typesense_key_2024)"
    else
        echo "⚠️ .env.example not found. Creating basic .env file..."
        cat > .env << EOF
# I.O.R.A. Environment Configuration
# Update these values with your actual credentials

# Gemini AI API Key (get from: https://makersuite.google.com/app/apikey)
GEMINI_API_KEY=your_gemini_api_key_here

# Solana RPC Configuration
SOLANA_RPC_URL=https://api.devnet.solana.com

# Solana Wallet Path (will be created by Solana CLI)
SOLANA_WALLET_PATH=./wallets/devnet-wallet.json

# Self-hosted Typesense Configuration
TYPESENSE_API_KEY=iora_dev_typesense_key_2024
TYPESENSE_URL=http://localhost:8108
EOF
        echo "✅ Created basic .env file - please update with real values"
    fi
else
    echo "✅ .env file already exists"
fi

# 6. Final verification and instructions
echo ""
echo "🎉 Development Environment Setup Complete!"
echo ""
echo "📋 Services Status:"
echo "  • Typesense: http://localhost:8108 (API Key: iora_dev_typesense_key_2024)"
echo "  • Solana Devnet: https://api.devnet.solana.com"
echo "  • Wallet: ./wallets/devnet-wallet.json"
echo ""
echo "🚀 Next Steps:"
echo "  1. Update .env file with your Gemini API key"
echo "  2. Run 'cargo build' to verify the project compiles"
echo "  3. Run 'cargo test' to run the test suite"
echo "  4. Run 'make ci' for full CI simulation"
echo ""
echo "🛠️ Useful Commands:"
echo "  • Start Typesense: docker-compose up -d typesense"
echo "  • Stop Typesense: docker-compose down"
echo "  • Check Solana balance: solana balance"
echo "  • Build project: cargo build"
echo "  • Run tests: cargo test"
echo "  • Format code: cargo fmt"
echo "  • Lint code: cargo clippy"
echo ""
echo "📚 Documentation:"
echo "  • I.O.R.A. README: ./README.md"
echo "  • Solana Docs: https://docs.solana.com/"
echo "  • Anchor Docs: https://www.anchor-lang.com/"
echo "  • Typesense Docs: https://typesense.org/docs/latest/"
echo ""
echo "✨ Happy coding with I.O.R.A.!"
