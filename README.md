# I.O.R.A. - Intelligent Oracle Rust Assistant

[![CI](https://github.com/guglxni/iora/workflows/CI/badge.svg)](https://github.com/guglxni/iora/actions)
[![Coverage](https://codecov.io/gh/guglxni/iora/branch/main/graph/badge.svg)](https://codecov.io/gh/guglxni/iora)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

I.O.R.A. is an intelligent oracle system built in Rust that fetches real-world data, augments it with RAG (Retrieval-Augmented Generation), analyzes it using Gemini API, and feeds the results as an oracle to Solana smart contracts.

## 🚀 Features

- **Real-time Data Fetching**: Fetch cryptocurrency and market data from multiple APIs
- **RAG Augmentation**: Enhance data with contextual information using Typesense
- **AI-Powered Analysis**: Leverage Google's Gemini API for intelligent insights
- **Solana Integration**: Feed analyzed data to Solana smart contracts
- **Comprehensive Testing**: 68+ automated tests covering all components
- **CI/CD Ready**: Full GitHub Actions pipeline with coverage and security scanning

## 📋 Prerequisites

- **Rust**: 1.70.0 or later
- **Docker**: For running Typesense and local testing
- **Solana CLI**: For blockchain interactions
- **Git**: For version control

## 🛠️ Quick Setup

### Automated Setup
```bash
# Clone the repository
git clone https://github.com/guglxni/iora.git
cd iora

# Run the automated setup script
./scripts/setup-dev.sh
```

### Manual Setup
```bash
# Install Rust and components
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Install development tools
cargo install cargo-tarpaulin cargo-audit cargo-watch

# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Copy environment template
cp .env.example .env
```

## 🏗️ Building

```bash
# Debug build
make build
# or
cargo build

# Release build
make build-release
# or
cargo build --release

# Run the application
make run
# or
cargo run
```

## 🧪 Testing

### All Tests
```bash
make test
# or
cargo test
```

### Specific Test Suites
```bash
# Unit tests
make test-unit
cargo test --lib

# Integration tests
make test-integration
cargo test --test integration_tests

# Configuration tests
make test-config
cargo test --test config_tests
```

### Test Coverage
```bash
# Generate coverage report
make coverage

# Generate HTML coverage report
make coverage-html
```

## 🔧 Code Quality

### Linting
```bash
make lint
# or
cargo clippy -- -D warnings
```

### Formatting
```bash
make format
# or
cargo fmt
```

### Pre-commit Hooks
```bash
# Run all pre-commit hooks
make pre-commit

# Install pre-commit hooks
make pre-commit-install
```

## 🐳 Docker

### Build and Run
```bash
# Build Docker image
make docker-build

# Run container
make docker-run

# Use docker-compose for services
make docker-compose-up
make docker-compose-down
```

## 🔒 Security

### Security Audit
```bash
make audit
# or
cargo audit
```

## 📊 CI/CD Pipeline

The project includes a comprehensive GitHub Actions CI/CD pipeline that runs on every push and pull request:

### CI Jobs
- **Test Suite**: Runs all tests, linting, and formatting checks
- **Code Coverage**: Generates coverage reports with cargo-tarpaulin
- **Security Audit**: Scans for security vulnerabilities
- **Docker Build**: Ensures Docker images build correctly
- **Release Build**: Creates release artifacts for tagged commits

### Local CI Simulation
```bash
make ci
```

## 🏗️ Project Structure

```
iora/
├── .github/
│   └── workflows/
│       └── ci.yml              # GitHub Actions CI/CD pipeline
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library interface
│   └── modules/
│       ├── cli.rs              # Command-line interface
│       ├── fetcher.rs          # Data fetching logic
│       ├── rag.rs              # RAG augmentation
│       ├── analyzer.rs         # Gemini API integration
│       └── solana.rs           # Solana blockchain integration
├── tests/
│   ├── unit_tests.rs           # Unit tests (27 tests)
│   ├── integration_tests.rs    # Integration tests (21 tests)
│   └── config_tests.rs         # Configuration tests (20 tests)
├── scripts/
│   └── setup-dev.sh           # Development environment setup
├── assets/
│   └── historical.json        # Sample historical data
├── clippy.toml                # Clippy linting configuration
├── rustfmt.toml               # Code formatting configuration
├── tarpaulin.toml             # Test coverage configuration
├── .pre-commit-config.yaml    # Pre-commit hooks configuration
├── docker-compose.yml         # Docker services configuration
├── Makefile                   # Development shortcuts
└── README.md                  # This file
```

## ⚙️ Configuration

### Environment Variables

Create a `.env` file based on `.env.example`:

```bash
# Copy template
cp .env.example .env

# Edit with your values
nano .env
```

Required environment variables:
- `GEMINI_API_KEY`: Your Google Gemini API key
- `SOLANA_RPC_URL`: Solana RPC endpoint URL
- `SOLANA_WALLET_PATH`: Path to your Solana wallet keypair
- `TYPESENSE_URL`: Typesense server URL
- `TYPESENSE_API_KEY`: Typesense API key

### Docker Services

The `docker-compose.yml` file includes:
- **Typesense**: Vector database for RAG functionality
- **PostgreSQL**: Optional database for data persistence

## 🚀 Usage

### Basic Usage
```bash
# Run with default settings
cargo run

# Run with specific query
cargo run -- --query "BTC price analysis"

# Show help
cargo run -- --help
```

### Advanced Usage
```bash
# Run with custom Gemini API key
GEMINI_API_KEY=your_key_here cargo run

# Use different Solana network
SOLANA_RPC_URL=https://api.devnet.solana.com cargo run
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests: `make test`
5. Run linting: `make lint`
6. Format code: `make format`
7. Commit changes: `git commit -m "Add your feature"`
8. Push to branch: `git push origin feature/your-feature`
9. Create a Pull Request

### Development Workflow
```bash
# Set up development environment
make setup

# Work on your feature
git checkout -b feature/your-feature

# Run tests and checks frequently
make check-all

# Generate coverage report
make coverage

# Ensure everything works
make ci
```

## 📈 Performance

- **Fast Compilation**: Optimized build times with cargo caching
- **Efficient Testing**: Parallel test execution with comprehensive coverage
- **Optimized Binaries**: Release builds with full optimizations
- **Low Resource Usage**: Minimal memory footprint for oracle operations

## 🔐 Security

- **Dependency Scanning**: Regular security audits with `cargo audit`
- **Code Review**: All changes require review before merging
- **Secure Defaults**: Conservative security settings in configuration
- **API Key Protection**: Environment variables for sensitive data

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Rust Community**: For the excellent Rust ecosystem
- **Solana Foundation**: For blockchain infrastructure
- **Google**: For Gemini AI API
- **Typesense**: For vector search capabilities
- **GitHub**: For CI/CD and collaboration tools

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/guglxni/iora/issues)
- **Discussions**: [GitHub Discussions](https://github.com/guglxni/iora/discussions)
- **Documentation**: [Wiki](https://github.com/guglxni/iora/wiki)

---

**Built with ❤️ in Rust for the Roo Code Hackathon**
