# I.O.R.A. - Intelligent Oracle Rust Assistant

[![CI](https://github.com/guglxni/iora/actions/workflows/ci.yml/badge.svg)](https://github.com/guglxni/iora/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.6+-blue.svg)](https://www.typescriptlang.org/)
[![Solana](https://img.shields.io/badge/solana-devnet-purple.svg)](https://solana.com/)

I.O.R.A. (Intelligent Oracle Rust Assistant) is a comprehensive AI-Web3 oracle system that combines real-time cryptocurrency data fetching, RAG-augmented analysis, and blockchain oracle feeds. Built for the **Internet of Agents Hackathon**, it provides both a powerful CLI toolset and MCP (Model Context Protocol) server integration for Coral Studio compatibility.

## 🚀 Core Features

### 🔄 **Real-time Data Pipeline**
- **Multi-API Data Fetching**: CoinGecko, CoinMarketCap, and custom provider integration
- **Real-time Price Feeds**: Live cryptocurrency and market data with multiple sources
- **Data Validation**: Comprehensive input validation and error handling
- **Rate Limiting**: Intelligent request throttling and retry mechanisms

### 🧠 **AI-Powered Analysis**
- **Gemini API Integration**: Google's advanced LLM for market analysis and insights
- **RAG Augmentation**: Contextual information retrieval using Typesense vector database
- **Multi-Provider Support**: Configurable AI providers with fallback chains
- **Intelligent Prompts**: Customizable analysis templates and prompt engineering

### ⛓️ **Solana Blockchain Integration**
- **Devnet Oracle Feeds**: Real transaction submissions to Solana smart contracts
- **Wallet Management**: Automated keypair handling and transaction signing
- **Program Deployment**: Anchor-based oracle contract deployment and management
- **Crossmint Receipts**: NFT-based proof-of-oracle receipts with on-chain verification

### 🛠️ **Advanced CLI Toolset (15+ Command Groups)**
- **`config`**: Configuration management and environment profiles
- **`apis`**: API provider management (add, remove, test, prioritize)
- **`ai`**: AI provider configuration, model comparison, benchmarking
- **`blockchain`**: Network switching, wallet management, deployment
- **`rag`**: Vector database initialization, indexing, optimization
- **`mcp`**: MCP server lifecycle management and configuration
- **`deploy`**: Multi-target deployment (Docker, Kubernetes, Cloud)
- **`monitor`**: Real-time metrics, health checks, and alerting
- **`analytics`**: Performance analytics and reporting
- **`plugins`**: Extensible plugin system and marketplace
- **`profile`**: Environment profile management
- **`template`**: Reusable configuration templates

### 🔌 **MCP Server Integration**
- **Coral Studio Compatible**: Full MCP protocol implementation for agent interoperability
- **4 Core Tools**: `get_price`, `analyze_market`, `feed_oracle`, `health`
- **HTTP & CLI Modes**: Flexible execution with binary spawning or direct HTTP calls
- **Registry Ready**: Publishable to local Coral Registry for discovery

### 📊 **Enterprise-Grade Capabilities**
- **Comprehensive Testing**: 25+ test suites covering unit, integration, performance, and security
- **CI/CD Pipeline**: GitHub Actions with automated testing, coverage, and security scanning
- **Load Testing**: Concurrent user simulation and stress testing
- **Performance Monitoring**: Real-time metrics and alerting system
- **Security Auditing**: Automated dependency vulnerability scanning
- **Docker & Kubernetes**: Containerized deployment with orchestration

## 🛠️ **Technology Stack**

### **Core Technologies**
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| **Backend** | Rust | 1.70+ | High-performance systems programming |
| **CLI Framework** | Clap | 4.5+ | Advanced command-line argument parsing |
| **Async Runtime** | Tokio | 1.0+ | Asynchronous operations and concurrency |
| **HTTP Client** | Reqwest | 0.12+ | REST API communications |
| **Serialization** | Serde | 1.0+ | JSON/data structure handling |
| **Blockchain** | Solana SDK | 1.18+ | Solana blockchain integration |

### **AI & Data Processing**
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| **LLM Integration** | Google Gemini API | Latest | AI-powered market analysis |
| **Vector Database** | Typesense | 27.0+ | RAG context retrieval |
| **Embeddings** | Custom/OpenAI | Latest | Text vectorization |
| **Data Processing** | Custom Rust | - | Real-time data pipelines |

### **MCP Server (Node.js/TypeScript)**
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| **Runtime** | Node.js | 18+ | JavaScript execution environment |
| **Language** | TypeScript | 5.6+ | Type-safe JavaScript development |
| **Web Framework** | Express.js | 4.19+ | REST API server |
| **Process Management** | Execa | 9.3+ | External command execution |
| **Validation** | Zod | 3.23+ | Runtime type validation |
| **Security** | Helmet | 8.1+ | HTTP security headers |

### **DevOps & Testing**
| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| **Containerization** | Docker | Latest | Application packaging |
| **Orchestration** | Docker Compose | Latest | Multi-service deployment |
| **CI/CD** | GitHub Actions | Latest | Automated pipelines |
| **Test Coverage** | Tarpaulin | Latest | Code coverage analysis |
| **Linting** | Clippy | Latest | Code quality enforcement |
| **Formatting** | Rustfmt | Latest | Code formatting |

### **External Integrations**
| Service | Purpose | Environment |
|---------|---------|-------------|
| **CoinGecko API** | Cryptocurrency data | Production |
| **CoinMarketCap API** | Market data | Production |
| **Solana Devnet** | Blockchain testing | Development |
| **Crossmint** | NFT receipts | Production |
| **Coral Studio** | Agent interoperability | Hackathon |

## 📋 Prerequisites

### **Required Software**
- **Rust**: 1.70.0 or later ([Installation Guide](https://rustup.rs/))
- **Node.js**: 18.0.0 or later ([Download](https://nodejs.org/))
- **Docker**: Latest stable ([Installation](https://docs.docker.com/get-docker/))
- **Docker Compose**: V2.0+ (included with Docker Desktop)
- **Solana CLI**: Latest ([Installation](https://docs.solana.com/cli/install-solana-cli-tools))
- **Git**: Latest ([Installation](https://git-scm.com/))

### **API Keys Required**
- **Google Gemini API Key**: For AI analysis ([Get Key](https://makersuite.google.com/app/apikey))
- **CoinGecko API Key**: For enhanced rate limits ([Get Key](https://www.coingecko.com/en/api))
- **CoinMarketCap API Key**: For market data ([Get Key](https://coinmarketcap.com/api/))
- **Crossmint API Keys**: For NFT receipts ([Get Keys](https://www.crossmint.com/))

### **System Requirements**
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 5GB free space
- **Network**: Stable internet connection
- **OS**: macOS 12+, Linux (Ubuntu 20.04+), Windows 10+ (WSL2)

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
├── 📁 iora/                          # Main Rust application
│   ├── src/
│   │   ├── main.rs                   # CLI entry point
│   │   ├── lib.rs                    # Core library interface
│   │   └── modules/                  # Feature modules
│   │       ├── cli_toolset.rs        # Advanced CLI commands (15+ groups)
│   │       ├── fetcher.rs            # Multi-API data fetching
│   │       ├── analyzer.rs           # Gemini AI integration
│   │       ├── rag.rs                # Typesense vector operations
│   │       ├── solana.rs             # Blockchain integration
│   │       ├── cache.rs              # Performance caching
│   │       ├── config.rs             # Configuration management
│   │       ├── health.rs             # Health monitoring
│   │       ├── historical.rs         # Historical data processing
│   │       ├── load_testing.rs       # Performance testing
│   │       ├── processor.rs          # Data processing pipeline
│   │       └── resilience.rs         # Error handling & recovery
│   ├── mcp/                          # MCP Server (Node.js/TypeScript)
│   │   ├── src/
│   │   │   ├── index.ts              # MCP server entry point
│   │   │   ├── lib/spawnIORA.ts      # IORA binary execution
│   │   │   └── tools/                # MCP tools
│   │   │       ├── get_price.ts      # Price data retrieval
│   │   │       ├── analyze_market.ts # AI market analysis
│   │   │       ├── feed_oracle.ts    # Blockchain oracle feeds
│   │   │       └── health.ts         # System health status
│   │   ├── package.json              # Node.js dependencies
│   │   ├── tsconfig.json             # TypeScript configuration
│   │   └── coral.server.config.ts    # Coral Studio configuration
│   ├── programs/oracle/              # Solana smart contracts
│   │   └── src/lib.rs                # Anchor oracle program
│   ├── tests/                        # Comprehensive test suite (25+ files)
│   │   ├── unit_tests.rs             # Core functionality tests
│   │   ├── integration_tests.rs      # End-to-end integration
│   │   ├── cli_toolset_tests.rs      # CLI command validation
│   │   ├── performance_tests.rs      # Load & stress testing
│   │   ├── solana_tests.rs           # Blockchain integration
│   │   └── analyzer_tests.rs         # AI functionality
│   ├── specs/                        # Feature specifications
│   ├── docs/                         # Development documentation
│   ├── scripts/                      # Setup & utility scripts
│   ├── wallets/                      # Solana wallet storage
│   └── iora-config.json              # CLI configuration
├── 📁 scripts/                       # Development setup scripts
├── 📁 specs/                         # Feature specifications
├── 📁 docs/                          # Development guides
├── 📁 assets/                        # Sample data & resources
├── 📁 .github/workflows/             # CI/CD pipelines
├── 📄 Cargo.toml                     # Rust dependencies
├── 📄 docker-compose.yml             # Multi-service orchestration
├── 📄 Makefile                       # Development shortcuts
├── 📄 .env.example                   # Environment template
└── 📄 README.md                      # This documentation
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

### **CLI Toolset Usage**

#### **Basic Commands**
```bash
# Initialize IORA configuration
iora init

# Show current configuration
iora config show

# List available API providers
iora apis list

# Add a new API provider
iora apis add coingecko --key your_api_key

# Test API provider connectivity
iora apis test coingecko

# Get real-time price data
iora apis stats
```

#### **AI & Analysis Commands**
```bash
# List available AI models
iora ai models

# Configure default AI provider
iora ai config gemini

# Test AI provider connectivity
iora ai test gemini

# Compare AI models performance
iora ai compare gemini openai

# Run AI benchmarking
iora ai benchmark
```

#### **Blockchain Commands**
```bash
# Show available networks
iora blockchain networks

# Switch to Devnet
iora blockchain switch devnet

# Show wallet information
iora blockchain wallet info

# Deploy oracle program
iora blockchain deploy
```

#### **MCP Server Management**
```bash
# Start MCP server
iora mcp start

# Check MCP server status
iora mcp status

# View MCP server logs
iora mcp logs

# Configure MCP server
iora mcp config
```

#### **RAG System Commands**
```bash
# Initialize vector database
iora rag init

# Index market data
iora rag index market_data.json

# Check RAG system status
iora rag status

# Optimize vector search
iora rag optimize
```

### **MCP Server Usage (Coral Studio Compatible)**

#### **Starting the MCP Server**
```bash
# Navigate to MCP directory
cd iora/mcp

# Install dependencies
npm install

# Start development server
npm run dev

# Server runs on http://localhost:7070
```

#### **MCP Tools Available in Coral Studio**

##### **1. get_price** - Real-time Price Data
```typescript
// Tool Input
{
  "symbol": "BTC",
  "currency": "USD"
}

// Tool Response
{
  "symbol": "BTC",
  "price": 45123.45,
  "currency": "USD",
  "timestamp": "2025-01-20T10:30:00Z",
  "source": "coingecko",
  "change_24h": 2.34
}
```

##### **2. analyze_market** - AI-Powered Analysis
```typescript
// Tool Input
{
  "symbol": "BTC",
  "analysis_type": "technical"
}

// Tool Response
{
  "symbol": "BTC",
  "analysis": "Bitcoin shows strong bullish momentum...",
  "confidence_score": 0.85,
  "signals": ["BUY", "STRONG"],
  "recommendation": "Accumulate on dips",
  "timestamp": "2025-01-20T10:30:00Z"
}
```

##### **3. feed_oracle** - Blockchain Oracle Update
```typescript
// Tool Input
{
  "symbol": "BTC",
  "price": 45123.45,
  "confidence": 0.95
}

// Tool Response
{
  "transaction_signature": "4xKpN8d...9VzL",
  "oracle_address": "OraCLE...8Yz",
  "slot": 123456789,
  "receipt_mint": "NFT...123",
  "status": "confirmed"
}
```

##### **4. health** - System Health Check
```typescript
// Tool Input
{}

// Tool Response
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": "2h 15m",
  "services": {
    "mcp_server": "running",
    "ai_providers": ["gemini"],
    "blockchain": "connected",
    "vector_db": "operational"
  },
  "metrics": {
    "total_requests": 1247,
    "avg_response_time": "450ms",
    "error_rate": "0.02%"
  }
}
```

### **Advanced Usage Examples**

#### **Complete Oracle Pipeline**
```bash
# 1. Configure environment
iora profile create production
iora config edit

# 2. Set up AI providers
iora ai set-default gemini
iora ai add-fallback openai

# 3. Configure API providers
iora apis add coingecko --key $COINGECKO_KEY --priority 1
iora apis add coinmarketcap --key $CMC_KEY --priority 2

# 4. Initialize RAG system
iora rag init
iora rag index historical_data.json

# 5. Start MCP server for Coral Studio
iora mcp start

# 6. Monitor system health
iora monitor metrics
```

#### **Development Workflow**
```bash
# Run full test suite
make test

# Check code quality
make lint
make format

# Generate coverage report
make coverage

# Run performance benchmarks
make bench

# Deploy to staging
iora deploy docker
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

## 🎯 **Internet of Agents Hackathon**

I.O.R.A. was built specifically for the **Internet of Agents Hackathon**, focusing on multi-agent interoperability through the **Coral Protocol**. The system demonstrates:

### **Hackathon Achievements**
- ✅ **Coral Studio Integration**: Full MCP (Model Context Protocol) compatibility
- ✅ **Multi-Agent Communication**: HTTP-based tool interfaces with HMAC authentication
- ✅ **Real Data Pipelines**: Production-grade cryptocurrency data aggregation
- ✅ **AI-Powered Analysis**: Multi-provider LLM integration with RAG augmentation
- ✅ **Blockchain Oracles**: Live Solana devnet oracle feeds with NFT receipts
- ✅ **Enterprise Security**: Rate limiting, request validation, and structured logging
- ✅ **Docker Deployment**: One-command containerized deployment
- ✅ **Comprehensive Testing**: 25+ test suites covering all functionality

### **Coral Protocol Compliance**
- **MCP Server**: Node.js/TypeScript implementation with Express.js
- **Tool Registry**: 4 structured tools (`get_price`, `analyze_market`, `feed_oracle`, `health`)
- **Authentication**: HMAC-SHA256 signature verification
- **Error Handling**: Structured JSON responses with proper HTTP status codes
- **Observability**: Request tracing and performance monitoring

### **Judge Evaluation Points**
- **Production Readiness**: Docker deployment, security hardening, comprehensive testing
- **Real Integration**: No mocks - actual API calls, LLM queries, blockchain transactions
- **Scalability**: Async Rust backend, connection pooling, rate limiting
- **Documentation**: Complete setup guides, API documentation, demo scripts
- **Innovation**: Multi-provider AI, NFT receipts, RAG-augmented analysis

### **Quick Demo (90 seconds)**
```bash
# 1. Setup (2 minutes)
git clone <repo-url> && cd iora
make build
export CORAL_SHARED_SECRET=<random-hex>
export GEMINI_API_KEY=<your-key>
make run &

# 2. Health check
curl http://localhost:7070/tools/health

# 3. Get price data
curl -H "x-iora-signature: $(echo -n '{"symbol":"BTC"}' | openssl dgst -sha256 -hmac $CORAL_SHARED_SECRET | awk '{print $2}')" \
     -d '{"symbol":"BTC"}' http://localhost:7070/tools/get_price

# 4. AI market analysis
curl -H "x-iora-signature: $(echo -n '{"symbol":"BTC","provider":"gemini"}' | openssl dgst -sha256 -hmac $CORAL_SHARED_SECRET | awk '{print $2}')" \
     -d '{"symbol":"BTC","provider":"gemini"}' http://localhost:7070/tools/analyze_market

# 5. Oracle feed with NFT receipt
curl -H "x-iora-signature: $(echo -n '{"symbol":"BTC"}' | openssl dgst -sha256 -hmac $CORAL_SHARED_SECRET | awk '{print $2}')" \
     -d '{"symbol":"BTC"}' http://localhost:7070/tools/feed_oracle
```

---

**🏆 Built with ❤️ in Rust for the Internet of Agents Hackathon - Coral Protocol Edition**
