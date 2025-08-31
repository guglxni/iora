# I.O.R.A. Development Environment Guide

This guide provides comprehensive instructions for setting up and using the I.O.R.A. development environment with all the tools and configurations needed for efficient Rust development.

## 🎯 Quick Start

### 1. Environment Setup
```bash
# Clone and setup the project
git clone <repository-url>
cd iora

# Run complete setup (installs all tools and configures environment)
./scripts/dev-workflow.sh setup

# Or run individual setup steps
./scripts/install-all-tools.sh
```

### 2. VS Code Setup
```bash
# Open the project in VS Code
code .

# Install recommended extensions (should prompt automatically)
# Or manually install from .vscode/extensions.json
```

### 3. Development Workflow
```bash
# Start development
./scripts/dev-workflow.sh watch

# In another terminal, run tests continuously
./scripts/dev-workflow.sh test-watch
```

## 🛠️ Development Tools

### Core Rust Tools
All essential Rust development tools are pre-installed:

- **rustc** (1.89.0) - Rust compiler
- **cargo** (1.89.0) - Package manager and build tool
- **rustfmt** (1.8.0) - Code formatter
- **clippy** (0.1.89) - Linter

### Development Tools
- **cargo-watch** (8.5.3) - File watcher for automatic rebuilds
- **cargo-tarpaulin** (0.32.8) - Test coverage analysis
- **cargo-audit** (0.21.2) - Security vulnerability scanner
- **cargo-expand** - Macro expansion tool
- **cargo-edit** - Dependency management

### Blockchain Tools
- **Solana CLI** (1.18.20) - Blockchain interaction
- **Anchor CLI** (0.31.1) - Solana program framework
- **Typesense** (self-hosted) - Vector search database

## 🖥️ VS Code Configuration

### Recommended Extensions
The following extensions are automatically recommended:

#### Essential Extensions
- **rust-analyzer** - Rust language server with advanced features
- **even-better-toml** - TOML syntax highlighting and formatting
- **roo-cline** - AI-assisted development

#### Development Extensions
- **vscode-docker** - Docker integration
- **gitlens** - Enhanced Git capabilities
- **vscode-icons** - File icons
- **path-intellisense** - Path autocompletion

#### Quality & Productivity
- **prettier** - Code formatting
- **vscode-env** - Environment file support

### VS Code Settings
The `.vscode/settings.json` file includes:

#### Rust Analyzer Configuration
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.completion.autoimport.enable": true,
  "rust-analyzer.inlayHints.enable": true,
  "rust-analyzer.lens.run.enable": true
}
```

#### Editor Configuration
```json
{
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": "explicit",
    "source.organizeImports": "explicit"
  },
  "editor.rulers": [100],
  "editor.tabSize": 4
}
```

#### Custom Tasks
Pre-configured tasks for common operations:
- `cargo build` - Build the project
- `cargo test` - Run tests
- `cargo run` - Run the application
- `cargo watch` - Watch mode for development

## 🚀 Development Workflow

### Using the Workflow Script

The `scripts/dev-workflow.sh` script provides convenient commands:

```bash
# Development commands
./scripts/dev-workflow.sh build      # Build project
./scripts/dev-workflow.sh run        # Run application
./scripts/dev-workflow.sh test       # Run tests
./scripts/dev-workflow.sh watch      # Watch mode

# Code quality
./scripts/dev-workflow.sh fmt        # Format code
./scripts/dev-workflow.sh lint       # Run linter
./scripts/dev-workflow.sh fix        # Auto-fix issues

# Services
./scripts/dev-workflow.sh docker-up  # Start services
./scripts/dev-workflow.sh docker-down # Stop services

# Status checks
./scripts/dev-workflow.sh status     # Environment status
./scripts/dev-workflow.sh env-check  # Config validation
```

### Watch Mode Development
```bash
# Start watch mode in one terminal
./scripts/dev-workflow.sh watch

# Run tests continuously in another terminal
./scripts/dev-workflow.sh test-watch

# The application will automatically rebuild on file changes
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
./scripts/dev-workflow.sh test

# Run specific test suites
cargo test config     # Configuration tests
cargo test unit_tests # Unit tests
cargo test integration_tests # Integration tests

# Run with coverage
./scripts/dev-workflow.sh coverage
```

### Test Coverage
```bash
# Generate HTML coverage report
./scripts/dev-workflow.sh coverage

# View report in browser
open tarpaulin-report.html
```

## 🔧 Code Quality

### Formatting
```bash
# Format all code
./scripts/dev-workflow.sh fmt

# Check formatting without changes
cargo fmt --all -- --check
```

### Linting
```bash
# Run clippy linter
./scripts/dev-workflow.sh lint

# Fix auto-fixable issues
./scripts/dev-workflow.sh fix
```

### Security Audit
```bash
# Check for security vulnerabilities
./scripts/dev-workflow.sh audit
```

## 🐳 Docker Services

### Typesense (Vector Database)
```bash
# Start Typesense service
./scripts/dev-workflow.sh docker-up

# Check service status
./scripts/dev-workflow.sh typesense-status

# View service logs
./scripts/dev-workflow.sh docker-logs

# Access dashboard
open http://localhost:8108
```

### Service Health Checks
```bash
# Check all services
docker-compose ps

# Check Typesense health
curl -H "X-TYPESENSE-API-KEY: iora_dev_typesense_key_2024" \
     http://localhost:8108/health
```

## 🔐 Environment Configuration

### Environment Variables
The `.env` file contains all necessary configuration:

```env
# Gemini AI API Key
GEMINI_API_KEY=your_api_key_here

# Solana Configuration
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_WALLET_PATH=./wallets/devnet-wallet.json

# Typesense Configuration
TYPESENSE_API_KEY=iora_dev_typesense_key_2024
TYPESENSE_URL=http://localhost:8108
```

### Configuration Validation
```bash
# Check environment configuration
./scripts/dev-workflow.sh env-check

# Validate all settings
cargo run  # Application will validate config on startup
```

## 🚦 CI/CD Pipeline

### Local CI Simulation
```bash
# Run full CI pipeline locally
./scripts/dev-workflow.sh ci

# This runs:
# 1. Code formatting check
# 2. Clippy linting
# 3. Project build
# 4. Test execution
# 5. Security audit
# 6. Coverage generation
```

### Pre-commit Hooks
Pre-commit hooks ensure code quality:

```bash
# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Run hooks manually
pre-commit run --all-files

# Update hooks
pre-commit autoupdate
```

## 🐛 Debugging

### VS Code Debugging
1. Open the project in VS Code
2. Go to Run and Debug (Ctrl+Shift+D)
3. Select "Debug I.O.R.A." configuration
4. Press F5 to start debugging

### Debug Configurations
Available debug configurations:
- **Debug I.O.R.A.** - Debug the main application
- **Debug I.O.R.A. (Release)** - Debug release build
- **Debug Tests** - Debug test execution

## 📊 Performance Optimization

### Development Optimizations
```bash
# Enable incremental compilation
export CARGO_INCREMENTAL=1

# Use more CPU cores for compilation
export RUSTC_WRAPPER=sccache  # If installed

# Optimize for development builds
export RUSTFLAGS="-C debuginfo=1"
```

### Release Builds
```bash
# Build optimized release
cargo build --release

# Run release build
./target/release/iora
```

## 🔄 Version Management

### Rust Version
```bash
# Check current version
rustc --version
cargo --version

# Update Rust
rustup update stable

# Use specific version
rustup install 1.89.0
rustup default 1.89.0
```

### Tool Updates
```bash
# Update cargo tools
cargo install cargo-watch --force
cargo install cargo-tarpaulin --force
cargo install cargo-audit --force
```

## 📚 Additional Resources

### Documentation
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Analyzer Manual](https://rust-analyzer.github.io/manual.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)

### Community Resources
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [Solana Discord](https://discord.gg/solana)

## 🆘 Troubleshooting

### Common Issues

#### VS Code Issues
```bash
# Reload VS Code window
Ctrl+Shift+P → "Developer: Reload Window"

# Restart Rust Analyzer
Ctrl+Shift+P → "Rust Analyzer: Restart Server"
```

#### Build Issues
```bash
# Clean and rebuild
./scripts/dev-workflow.sh clean
./scripts/dev-workflow.sh build

# Check dependencies
cargo tree
```

#### Test Issues
```bash
# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

#### Docker Issues
```bash
# Restart Docker services
./scripts/dev-workflow.sh docker-down
./scripts/dev-workflow.sh docker-up

# Check Docker logs
docker-compose logs typesense
```

### Getting Help
1. Check the [Troubleshooting Guide](./troubleshooting.md)
2. Review the [Development Setup Guide](./development-setup.md)
3. Check existing issues in the project repository
4. Ask in the development channels

---

**🎉 Happy coding with I.O.R.A.!**

Your development environment is now fully configured for efficient Rust development with AI assistance, blockchain integration, and comprehensive tooling support.
