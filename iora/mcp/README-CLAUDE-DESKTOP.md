# IORA MCP Server - Claude Desktop Integration

## 🚀 Quick Install for Claude Desktop

### Option 1: One-Click Desktop Extension (Recommended)
1. Download: [IORA-Desktop-Extension.zip](./IORA-Desktop-Extension.zip)
2. Extract the ZIP file
3. Follow the included installation guide

### Option 2: Manual Configuration

#### Step 1: Build IORA MCP Server
```bash
cd /path/to/iora/iora/mcp
npm install
npm run build
```

#### Step 2: Add to Claude Desktop Configuration

**macOS:** Edit `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** Edit `%APPDATA%\Claude\claude_desktop_config.json`

Add this configuration:

```json
{
  "mcpServers": {
    "iora": {
      "command": "node",
      "args": ["/absolute/path/to/iora/iora/mcp/dist/index.js"],
      "env": {
        "PORT": "7070",
        "NODE_ENV": "production",
        "CORAL_REGISTRY_AUTO_REGISTER": "false",
        "CORAL_SERVER_URL": "http://localhost:7070",
        "LOG_LEVEL": "info"
      }
    }
  }
}
```

**Important:** Replace `/absolute/path/to/iora/iora/mcp/dist/index.js` with your actual path.

#### Step 3: Restart Claude Desktop

After adding the configuration, restart Claude Desktop to load IORA.

## 🛠️ Available Tools

Once installed, you'll have access to these IORA tools in Claude:

### 📊 Price & Market Data
- **`get_price`** - Get real-time cryptocurrency prices
- **`analyze_market`** - AI-powered market analysis with RAG
- **`query_crypto`** - Query cryptocurrency information

### 🔗 Blockchain Integration  
- **`feed_oracle`** - Submit data to Solana oracle feeds
- **`health`** - Check IORA system health

### 📈 Analytics & Monitoring
- **`cache_status`** - Check cache performance
- **`api_analytics`** - View API usage analytics
- **`health_check`** - Comprehensive system diagnostics

### 🤖 Coral Protocol Features
- **Session Management** - Persistent AI conversations
- **Thread Organization** - Structured conversation management
- **Agent Workflows** - Multi-step task orchestration
- **Telemetry Analytics** - Performance monitoring

## 🎯 Example Usage

Try these commands in Claude Desktop after installation:

```
Get the current price of Bitcoin and Ethereum
```

```
Analyze the Bitcoin market trends and provide insights
```

```
Check IORA's system health and performance metrics
```

```
Query information about Solana (SOL) cryptocurrency
```

## 🔧 Troubleshooting

### Server Won't Start
1. Check that Node.js is installed: `node --version`
2. Verify the path in your configuration is correct
3. Check logs: Look for error messages in Claude Desktop logs

### Tools Not Appearing
1. Restart Claude Desktop completely
2. Verify the JSON configuration syntax is correct
3. Check that the server started successfully

### Permission Issues
1. Ensure the IORA directory has proper read permissions
2. On macOS, you may need to grant Claude Desktop file access

## 📋 Configuration Options

### Environment Variables
- `PORT` - Server port (default: 7070)
- `NODE_ENV` - Environment mode (production/development)
- `LOG_LEVEL` - Logging verbosity (error/warn/info/debug)
- `CORAL_REGISTRY_AUTO_REGISTER` - Auto-register with Coral Registry
- `CORAL_SERVER_URL` - Server URL for registry registration

### Advanced Configuration
For advanced users, additional environment variables can be set:

```json
{
  "mcpServers": {
    "iora": {
      "command": "node",
      "args": ["/path/to/iora/iora/mcp/dist/index.js"],
      "env": {
        "PORT": "7070",
        "NODE_ENV": "production",
        "LOG_LEVEL": "info",
        "CORAL_REGISTRY_AUTO_REGISTER": "false",
        "CORAL_SERVER_URL": "http://localhost:7070",
        "GEMINI_API_KEY": "your-gemini-key",
        "MISTRAL_API_KEY": "your-mistral-key",
        "COINGECKO_API_KEY": "your-coingecko-key",
        "COINMARKETCAP_API_KEY": "your-coinmarketcap-key"
      }
    }
  }
}
```

## 🌟 Features

### Multi-API Data Aggregation
IORA intelligently aggregates data from multiple cryptocurrency APIs:
- CoinGecko
- CoinMarketCap  
- Binance
- CryptoCompare

### AI-Augmented Analysis
- RAG-powered market insights
- Multi-provider AI routing (Gemini, Mistral)
- Real-time trend analysis
- Intelligent data processing

### Blockchain Integration
- Solana oracle feed submission
- NFT receipt minting via Crossmint
- Transaction verification
- Multi-chain support

### Coral Protocol v1.0
- Session management for persistent conversations
- Thread organization for structured interactions
- Agent workflows for complex tasks
- Telemetry and performance monitoring

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/guglxni/iora/issues)
- **Documentation:** [Full Documentation](https://github.com/guglxni/iora)
- **Discord:** [IORA Community](https://discord.gg/iora-protocol)

---

**IORA - Intelligent Oracle Rust Assistant**  
Advanced cryptocurrency oracle system with AI-augmented analysis and blockchain integration.
