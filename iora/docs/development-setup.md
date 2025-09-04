# I.O.R.A. Development Environment Setup Guide

This guide provides detailed instructions for setting up the complete I.O.R.A. development environment with all required tools and services.

## 🎯 Quick Setup (Recommended)

For the fastest setup, run the comprehensive installation script:

```bash
# From the project root directory
./scripts/install-all-tools.sh
```

This script will automatically install and configure:
- ✅ Rust toolchain and development tools
- ✅ Solana CLI and Anchor framework
- ✅ Self-hosted Typesense via Docker
- ✅ Development environment configuration

## 🛠️ Manual Setup Instructions

If you prefer to install components individually, follow these steps:

### 1. Rust Toolchain Setup

```bash
# Run the Rust installation script
./scripts/install-rust.sh

# Or install manually:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy
cargo install cargo-watch cargo-tarpaulin cargo-audit
```

**Installed Components:**
- ✅ Rust compiler (`rustc`)
- ✅ Cargo package manager
- ✅ Code formatter (`rustfmt`)
- ✅ Linter (`clippy`)
- ✅ Development tools (`cargo-watch`, `cargo-tarpaulin`, `cargo-audit`)

### 2. Solana CLI and Anchor Setup

```bash
# Run the Solana installation script
./scripts/install-solana.sh

# Or install manually:
# Install Solana CLI (macOS example)
sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

# Install Anchor (requires Node.js)
npm i -g @coral-xyz/anchor-cli
```

**Configured Services:**
- ✅ Solana CLI (v1.18.4)
- ✅ Devnet wallet with initial SOL airdrop
- ✅ Anchor framework for Solana programs
- ✅ Development wallet: `./wallets/devnet-wallet.json`

### 3. Self-Hosted Typesense Setup

```bash
# Run the Typesense setup script
./scripts/setup-typesense.sh

# Or start manually:
docker-compose up -d typesense
```

**Typesense Configuration:**
- ✅ **Dashboard**: http://localhost:8108
- ✅ **API Key**: `iora_dev_typesense_key_2024`
- ✅ **Data Directory**: `./assets/data`
- ✅ **Health Checks**: Automatic container health monitoring

## 🐳 Docker Services

The project includes a comprehensive `docker-compose.yml` with multiple services:

### Core Services (Always Available)

```yaml
# Self-hosted Typesense for RAG functionality
typesense:
  image: typesense/typesense:27.0
  ports:
    - "8108:8108"
  environment:
    TYPESENSE_API_KEY: "iora_dev_typesense_key_2024"
```

### Optional Services (Development Only)

To run additional services for full development setup:

```bash
# Start all services including optional ones
docker-compose --profile full up -d

# Or start specific services
docker-compose up -d postgres redis
```

**Available Services:**
- ✅ **Typesense** (RAG vector database)
- 🔧 **PostgreSQL** (optional data persistence)
- 🔧 **Redis** (optional caching)

## ⚙️ Environment Configuration

### Required Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Copy the template
cp .env.example .env

# Edit with your actual values
nano .env
```

**Required Variables:**
```env
# Gemini AI API Key (required)
GEMINI_API_KEY=your_actual_gemini_api_key

# Solana Configuration (pre-configured)
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WALLET_PATH=./wallets/devnet-wallet.json

# Typesense Configuration (pre-configured)
TYPESENSE_API_KEY=iora_dev_typesense_key_2024
TYPESENSE_URL=http://localhost:8108
```

### API Key Setup

1. **Gemini API Key**: Get from [Google AI Studio](https://makersuite.google.com/app/apikey)
2. **Solana Wallet**: Automatically created during setup
3. **Typesense**: Pre-configured with development key

## 🧪 Testing the Setup

### Run All Tests
```bash
# Run the complete test suite
cargo test

# Run specific test suites
cargo test --test unit_tests          # Unit tests (27 tests)
cargo test --test integration_tests   # Integration tests (21 tests)
cargo test --test config_tests        # Configuration tests (20 tests)
```

### CI Pipeline Simulation
```bash
# Simulate the full CI pipeline locally
make ci

# Or run individual CI steps
cargo check                    # Compilation check
cargo test                     # Test execution
cargo clippy -- -D warnings    # Linting
cargo fmt --all -- --check     # Formatting check
cargo tarpaulin --ignore-tests # Coverage analysis
```

## 🚀 Development Workflow

### Starting Development

```bash
# 1. Start required services
docker-compose up -d typesense

# 2. Verify environment
cargo check

# 3. Run in development mode
cargo run

# 4. Run tests continuously
cargo watch -x test
```

### Code Quality Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run security audit
cargo audit

# Generate coverage report
cargo tarpaulin --ignore-tests --out Html
```

### Solana Development

```bash
# Check Solana configuration
solana config get

# Check wallet balance
solana balance

# Request additional SOL (Devnet)
solana airdrop 1

# Build Anchor programs (when available)
anchor build
```

## 🐛 Troubleshooting

### Common Issues

#### Rust Not Found
```bash
# Re-source environment
source ~/.cargo/env

# Or reinstall Rust
./scripts/install-rust.sh
```

#### Solana CLI Issues
```bash
# Check Solana installation
solana --version

# Reinstall if needed
./scripts/install-solana.sh
```

#### Docker Services Not Starting
```bash
# Check Docker status
docker --version
docker-compose --version

# View service logs
docker-compose logs typesense

# Restart services
docker-compose restart
```

#### Typesense Connection Issues
```bash
# Test Typesense health
curl -H "X-TYPESENSE-API-KEY: iora_dev_typesense_key_2024" \
     http://localhost:8108/health

# Restart Typesense
docker-compose restart typesense
```

### Service Management

```bash
# Start all services
docker-compose up -d

# Start only Typesense
docker-compose up -d typesense

# View running services
docker-compose ps

# View service logs
docker-compose logs -f typesense

# Stop all services
docker-compose down

# Clean up volumes (⚠️ destroys data)
docker-compose down -v
```

## 📊 Performance Optimization

### Development Optimizations

```bash
# Use cargo watch for automatic rebuilds
cargo watch -x run

# Enable incremental compilation
echo "incremental = true" >> ~/.cargo/config

# Optimize for development
export RUSTFLAGS="-C debuginfo=1"
```

### Production Considerations

- Use release builds: `cargo build --release`
- Enable Link Time Optimization (LTO)
- Configure appropriate memory limits for Docker services

## 🔐 Security Considerations

### Development Security

- ✅ Use development API keys (not production keys)
- ✅ Isolate development wallet from main wallet
- ✅ Use local services for development
- ✅ Regularly update dependencies with `cargo audit`

### Production Deployment

- 🔒 Use environment-specific API keys
- 🔒 Implement proper access controls
- 🔒 Use production-grade Docker configurations
- 🔒 Enable security headers and monitoring

## 📚 Additional Resources

### Documentation Links
- [I.O.R.A. README](../README.md)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Typesense Documentation](https://typesense.org/docs/latest/)

### Community Resources
- [Solana Discord](https://discord.com/invite/solana)
- [Rust User Forum](https://users.rust-lang.org/)
- [Anchor Discord](https://discord.gg/8HwmBtt2ss)

## 🎯 Next Steps

After completing the setup:

1. **Update Environment**: Add your Gemini API key to `.env`
2. **Test Integration**: Run `cargo test` to verify everything works
3. **Start Development**: Begin implementing the core I.O.R.A. functionality
4. **Monitor Services**: Keep Docker services running for development

---

**🎉 Your I.O.R.A. development environment is now ready!**

Happy coding and building the future of AI-powered blockchain oracles! 🚀
