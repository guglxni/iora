#!/bin/bash

# Solana CLI and Anchor Installation Script for I.O.R.A. Project
# This script installs Solana CLI tools, creates a Devnet wallet, and sets up Anchor

set -e

echo "☀️ Installing Solana CLI tools for I.O.R.A. project..."

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Solana CLI
if command_exists solana; then
    echo "✅ Solana CLI is already installed"
    solana --version
else
    echo "📦 Installing Solana CLI..."

    # Detect OS and architecture
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
    else
        echo "❌ Unsupported OS: $OSTYPE"
        exit 1
    fi

    # Install using official installer
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

    # Add Solana to PATH for current session
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

    echo "✅ Solana CLI installed successfully"
fi

# Configure Solana for Devnet
echo "⚙️ Configuring Solana for Devnet..."
solana config set --url https://api.devnet.solana.com
solana config get

# Create Devnet wallet if it doesn't exist
WALLET_DIR="./wallets"
mkdir -p "$WALLET_DIR"

DEVNET_WALLET="$WALLET_DIR/devnet-wallet.json"

if [ ! -f "$DEVNET_WALLET" ]; then
    echo "🔑 Creating new Devnet wallet..."
    solana-keygen new --outfile "$DEVNET_WALLET" --no-bip39-passphrase
    echo "✅ Devnet wallet created at: $DEVNET_WALLET"
else
    echo "✅ Devnet wallet already exists at: $DEVNET_WALLET"
fi

# Set the wallet as default
solana config set --keypair "$DEVNET_WALLET"

# Check wallet balance and airdrop if needed
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance)

if [[ "$BALANCE" == "0 SOL" ]]; then
    echo "🪂 Requesting airdrop for development..."
    solana airdrop 1

    # Wait a moment for airdrop to process
    sleep 5

    echo "💰 New balance:"
    solana balance
else
    echo "💰 Current balance: $BALANCE"
fi

# Install Anchor (Solana framework)
if command_exists anchor; then
    echo "✅ Anchor is already installed"
    anchor --version
else
    echo "📦 Installing Anchor..."

    # Install using npm/yarn (requires Node.js)
    if command_exists npm; then
        npm i -g @coral-xyz/anchor-cli
        echo "✅ Anchor installed via npm"
    elif command_exists yarn; then
        yarn global add @coral-xyz/anchor-cli
        echo "✅ Anchor installed via yarn"
    else
        echo "⚠️ Node.js not found. Please install Node.js and run:"
        echo "   npm i -g @coral-xyz/anchor-cli"
        echo "   or"
        echo "   yarn global add @coral-xyz/anchor-cli"
    fi
fi

# Install AVN (Anchor Verifier Network) if available
if command_exists npm && ! command_exists avn; then
    echo "📦 Installing Anchor Verifier Network..."
    npm i -g @coral-xyz/avn-cli
fi

echo ""
echo "✅ Solana development environment setup complete!"
echo ""
echo "🔧 Solana Configuration:"
solana config get
echo ""
echo "🔑 Wallet Location: $DEVNET_WALLET"
echo "💰 Wallet Balance: $(solana balance)"
echo ""
echo "🛠️ Available Commands:"
echo "  • solana --help          - Show Solana CLI help"
echo "  • solana balance         - Check wallet balance"
echo "  • solana airdrop <amount> - Request SOL airdrop"
echo "  • anchor --help          - Show Anchor CLI help"
echo ""
echo "📚 Useful Links:"
echo "  • Solana Docs: https://docs.solana.com/"
echo "  • Anchor Docs: https://www.anchor-lang.com/"
echo "  • Devnet Explorer: https://explorer.solana.com/?cluster=devnet"
