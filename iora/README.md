# I.O.R.A. - Intelligent Oracle Rust Assistant

[![CI](https://github.com/guglxni/iora/actions/workflows/ci.yml/badge.svg)](https://github.com/guglxni/iora/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

I.O.R.A. is an intelligent oracle system built in Rust that fetches real-world data, augments it with RAG (Retrieval-Augmented Generation), analyzes it using Gemini API, and feeds the results as an oracle to Solana smart contracts.

## 🚀 Features

- **Real-time Data Fetching**: Fetch cryptocurrency and market data from multiple APIs
- **RAG Augmentation**: Enhance data with contextual information using Typesense
- **AI-Powered Analysis**: Leverage Google's Gemini API for intelligent insights
- **Solana Integration**: Feed analyzed data to Solana smart contracts
- **Comprehensive Testing**: 68+ automated tests covering all components
- **CI/CD Ready**: Full GitHub Actions pipeline with coverage and security scanning
- **Automated Quality Gates**: PR validation, security scanning, and deployment automation

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

## 🎯 Judge Quickstart (Local)

1) **Build:**
   ```bash
   make build
   ```

2) **Set secrets:**
   ```bash
   export CORAL_SHARED_SECRET=<random-hex>
   export GEMINI_API_KEY=... (or MISTRAL_API_KEY / AIMLAPI_API_KEY)
   export SOLANA_RPC_URL=https://api.devnet.solana.com
   export CROSSMINT_PROJECT_ID=... ; export CROSSMINT_API_KEY=...
   ```

3) **Run MCP:**
   ```bash
   make run   # serves MCP on :7070
   ```

4) **Smoke test:**
   ```bash
   make demo  # prints health, price, analysis, oracle tx, receipt mint
   ```

5) **Coral Studio:**
   - Add local MCP server (see `mcp/mcp.config.json`)
   - Run tools: get_price → analyze_market → feed_oracle

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

## 🔄 CI/CD Pipeline

I.O.R.A. features a comprehensive automated testing and deployment pipeline powered by GitHub Actions.

### Pipeline Overview

#### 🚀 Main CI/CD Pipeline (`ci.yml`)
**Triggers**: Push to main/master/develop, Pull Requests
- **Quality Gates**: Code formatting, Clippy linting, security audit, documentation
- **Test Execution**: Unit, integration, functional, resilience, performance, and advanced tests
- **Performance Regression**: Automated performance regression detection with tarpaulin coverage
- **Load Testing**: Concurrent user and resource stress testing
- **Build & Release**: Automated release builds for multiple platforms
- **Docker Integration**: Automated Docker image building and testing
- **Final Quality Gate**: Comprehensive validation before deployment

#### 🔍 PR Quality Gates (`pr-quality-gate.yml`)
**Triggers**: Pull request events (opened, synchronize, review)
- **PR Validation**: Size checks, sensitive data scanning
- **Code Quality**: Automated formatting and linting checks
- **Security Scanning**: Dependency vulnerability and unsafe code detection
- **Test Execution**: Unit and integration test validation
- **Coverage Requirements**: 80% minimum test coverage enforcement
- **Performance Impact**: Automated performance regression checks
- **Auto-Review**: Automated approval for passing PRs

#### 📅 Scheduled Testing (`scheduled-testing.yml`)
**Triggers**: Daily at 2 AM UTC
- **Regression Testing**: Full test suite execution with coverage reporting
- **Dependency Audits**: Automated security vulnerability scanning
- **API Health Monitoring**: Real-time external service connectivity validation
- **Performance Trending**: Automated performance baseline tracking
- **Documentation Validation**: Automated docs validation and link checking

#### 🔄 Dependency Management (`dependency-updates.yml`)
**Triggers**: Weekly on Mondays
- **Automated Updates**: Weekly dependency version updates via Dependabot
- **Security Scanning**: Continuous vulnerability monitoring
- **Lockfile Maintenance**: Automated Cargo.lock updates with testing
- **Update PRs**: Automated pull request creation for dependency updates

#### 📦 Release Automation (`release.yml`)
**Triggers**: Git tags (v*.*.*) or manual dispatch
- **Multi-Platform Builds**: Automated binaries for Linux, macOS (Intel/ARM), Windows
- **Docker Images**: Automated container image building and publishing
- **GitHub Releases**: Automated release creation with changelogs
- **Release Validation**: Pre-release testing and quality assurance

### Quality Metrics & Monitoring

#### 📊 Comprehensive Quality Metrics System ✅ **Task 3.2.7.2 COMPLETED**

I.O.R.A. includes a sophisticated quality metrics and monitoring system that provides real-time insights into system health, performance trends, and quality improvements.

##### Test Coverage Analysis
- **Automated Coverage Collection**: Real-time test coverage monitoring using tarpaulin
- **Coverage Trend Analysis**: Historical coverage tracking with statistical analysis
- **Coverage Forecasting**: Predictive coverage trend analysis
- **Coverage Reporting**: HTML, JSON, and Markdown coverage reports

##### Performance Monitoring
- **Response Time Tracking**: Real-time API response time monitoring
- **Throughput Analysis**: Request per second throughput monitoring
- **Resource Usage**: Memory, CPU, and disk usage tracking
- **Performance Baselines**: Configurable performance thresholds and baselines
- **Regression Detection**: Automated performance regression alerts

##### Quality Trend Analysis
- **Statistical Trend Detection**: Linear regression and correlation analysis
- **Confidence Intervals**: Statistical confidence in trend analysis
- **Forecasting**: 7-day performance and quality forecasting
- **Seasonality Detection**: Automatic detection of periodic patterns
- **Quality Scorecards**: Comprehensive quality assessment reports

##### Automated Alerting System
- **Threshold-based Alerts**: Configurable metric threshold monitoring
- **Trend-based Alerts**: Statistical trend deviation alerts
- **Regression Alerts**: Performance and quality regression detection
- **Severity Classification**: Critical, High, Medium, Low alert levels
- **Alert Acknowledgment**: Alert tracking and resolution workflow

##### Dashboard Integration
- **Web-based Dashboard**: Real-time quality metrics visualization at `http://localhost:8080`
- **RESTful API**: JSON API endpoints for external integration
- **Real-time Updates**: Auto-refreshing dashboard with live metrics
- **Historical Data**: Trend visualization and historical comparisons
- **Export Capabilities**: JSON and CSV export for external analysis

##### Continuous Improvement
- **Automated Recommendations**: AI-powered improvement suggestions
- **Quality Score Calculation**: Weighted quality score across all metrics
- **Improvement Tracking**: Progress monitoring against quality goals
- **Best Practice Enforcement**: Automated code quality and performance standards

#### Quality Metrics API

```rust
use iora::modules::quality_metrics::QualityMetricsManager;

// Initialize quality metrics manager
let config = QualityMetricsConfig::default();
let manager = QualityMetricsManager::new(config);

// Collect metrics
manager.collect_metrics().await?;

// Get dashboard data
let dashboard = manager.get_dashboard().await;

// Generate scorecard
let scorecard = manager.generate_quality_scorecard().await;

// Get active alerts
let alerts = manager.get_active_alerts().await;
```

#### Dashboard Usage

```bash
# Start the quality metrics dashboard
cargo run --bin dashboard

# Access at http://localhost:8080
# API endpoints:
# GET /api/metrics - Current metrics
# GET /api/scorecard - Quality scorecard
# GET /api/alerts - Active alerts
```

#### Performance Baselines
- **Response Time**: < 500ms average response time (P95)
- **Throughput**: > 50 requests/second sustained
- **Memory Usage**: < 256MB maximum resident memory
- **CPU Usage**: < 70% average CPU utilization
- **Error Rate**: < 0.1% error rate threshold
- **Test Coverage**: > 85% code coverage required

#### Security Standards
- **Dependency Audits**: Daily security vulnerability scanning
- **Unsafe Code Detection**: Zero unsafe code blocks allowed
- **Secrets Detection**: Automated sensitive data scanning
- **Code Quality Gates**: Clippy warnings as errors
- **Audit Trail**: Comprehensive security event logging

### Pipeline Status Badges

```
CI:         ![CI](https://github.com/guglxni/iora/actions/workflows/ci.yml/badge.svg)
Coverage:   ![Coverage](https://codecov.io/gh/guglxni/iora/branch/main/graph/badge.svg)
Security:   ![Security](https://github.com/guglxni/iora/actions/workflows/scheduled-testing.yml/badge.svg)
```

### Local Development with CI/CD

#### Pre-commit Hooks
```bash
# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Run all checks locally
pre-commit run --all-files
```

#### Local Quality Gates
```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Run security audit
cargo audit

# Run tests with coverage
cargo tarpaulin --out Html --output-dir coverage
```

#### Docker Testing
```bash
# Build test image
docker build -t iora:test .

# Run tests in container
docker run --rm iora:test cargo test
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

**Built with ❤️ in Rust - A comprehensive AI-Web3 oracle system**
