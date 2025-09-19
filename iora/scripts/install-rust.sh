#!/bin/bash

# Rust Toolchain Installation Script for I.O.R.A. Project
# This script installs the Rust toolchain and development tools

set -e

echo "🦀 Installing Rust toolchain for I.O.R.A. project..."

# Check if Rust is already installed
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "✅ Rust is already installed"
    rustc --version
    cargo --version
else
    echo "📦 Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

    # Source the cargo environment
    source $HOME/.cargo/env

    echo "✅ Rust installed successfully"
    rustc --version
    cargo --version
fi

# Install additional Rust components
echo "🔧 Installing Rust components..."
rustup component add rustfmt
rustup component add clippy
rustup component add rust-src  # Useful for development

echo "📦 Installing useful cargo tools..."
cargo install cargo-watch
cargo install cargo-expand
cargo install cargo-edit
cargo install cargo-audit
cargo install cargo-tarpaulin

# Verify installation
echo ""
echo "✅ Rust toolchain installation complete!"
echo ""
echo "🔧 Available components:"
rustup component list --installed
echo ""
echo "📦 Installed cargo tools:"
cargo install --list | grep -E "(cargo-watch|cargo-expand|cargo-edit|cargo-audit|cargo-tarpaulin)" || echo "Some tools may not be listed"
echo ""
echo "🚀 Ready for Rust development!"
