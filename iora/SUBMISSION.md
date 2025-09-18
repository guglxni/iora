# Coral Protocol Hackathon Submission: IORA MCP Adapter

## Overview

IORA (Intelligent Oracle Rust Assistant) is a multi-source cryptocurrency data aggregator with AI-powered market analysis and Solana oracle integration. This submission provides a production-grade MCP (Multi-Agent Communication Protocol) adapter that enables Coral Studio to interact with IORA's core functionality.

## Features Delivered

✅ **Production MCP Adapter** - Zero-trust HTTP API with HMAC authentication
✅ **Multi-Provider LLM Support** - Gemini, Mistral, AI-ML API with provider selection
✅ **Real Data Integration** - No mocks; shells out to actual IORA binary
✅ **Structured JSON Contracts** - Deterministic I/O schemas for all tools
✅ **Security Hardened** - Rate limiting, request validation, error sanitization, kill-switch
✅ **Observability** - Structured logging with request tracing (redacted)
✅ **NFT Receipt Minting** - Crossmint custodial receipts for oracle transactions
✅ **E2E Tests** - Real integration tests hitting devnet
✅ **Docker Ready** - One-command deployment via docker-compose
✅ **Judge-Proof Demo** - Makefile, demo.sh, OpenAPI, Postman collection

## 90-Second Judge Demo Script

### Prerequisites Setup (2 minutes)

```bash
# 1. Clone, build, and run
git clone <repo-url> iora && cd iora
make build
export CORAL_SHARED_SECRET=<random-hex>
export GEMINI_API_KEY=... # or MISTRAL_API_KEY / AIMLAPI_API_KEY
make run &
sleep 3

# 2. Smoke test (optional)
make demo
```

### Demo Flow (90 seconds)

```bash
# Terminal 1: MCP Server Logs (keep visible)
# Shows structured JSON logging for all requests

# Terminal 2: Health Check (5 sec)
curl -s http://localhost:7070/tools/health | jq
# → {"ok":true,"data":{"status":"ok","versions":{"iora":"0.1.0","mcp":"1.0.0"},"uptime_sec":0}}

# Terminal 2: Get Price (10 sec)
curl -s http://localhost:7070/tools/health | jq
# → {"ok":true,"data":{"status":"ok","versions":{"iora":"0.1.0"},"uptime_sec":5}}

# Get Price (15 sec)
body='{"symbol":"BTC"}'
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$CORAL_SHARED_SECRET" | awk '{print $2}')
curl -s -H "content-type: application/json" -H "x-iora-signature: $sig" -d "$body" http://localhost:7070/tools/get_price | jq
# → {"ok":true,"data":{"symbol":"BTC","price":45000.0,"source":"CoinMarketCap","ts":1703123456}}

# Market Analysis with Provider Switch (30 sec)
body='{"symbol":"BTC","horizon":"1d","provider":"mistral"}'
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$CORAL_SHARED_SECRET" | awk '{print $2}')
curl -s -H "content-type: application/json" -H "x-iora-signature: $sig" -d "$body" http://localhost:7070/tools/analyze_market | jq
# → {"ok":true,"data":{"summary":"Bitcoin analysis...","signals":["signal1","signal2"],"confidence":0.85,"sources":["mistral-large-latest"]}}

# Oracle Feed + Receipt Mint (30 sec)
body='{"symbol":"BTC"}'
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$CORAL_SHARED_SECRET" | awk '{print $2}')
response=$(curl -s -H "content-type: application/json" -H "x-iora-signature: $sig" -d "$body" http://localhost:7070/tools/feed_oracle | jq)
echo "$response"
# → {"ok":true,"data":{"tx":"5K8q8cB9dXwJ...","slot":123456789,"digest":"abc123..."}}
# Terminal 1 logs: "Receipt minted for BTC oracle feed"

# Terminal 1: Check logs show successful execution
# All requests logged with reqId, timing, and exit codes
```

### Expected Log Output (during demo)

```json
{"level":"info","reqId":"550e8400-e29b-41d4-a716-446655440000","method":"GET","path":"/tools/health","ip":"::1","timestamp":"2024-01-01T12:00:00.000Z"}
{"level":"info","reqId":"550e8400-e29b-41d4-a716-446655440000","tool":"health","exitCode":0,"duration_ms":45,"timestamp":"2024-01-01T12:00:00.045Z"}
{"level":"info","reqId":"550e8400-e29b-41d4-a716-446655440001","method":"POST","path":"/tools/get_price","ip":"::1","timestamp":"2024-01-01T12:00:01.000Z"}
{"level":"info","reqId":"550e8400-e29b-41d4-a716-446655440001","tool":"get_price","exitCode":0,"duration_ms":1250,"timestamp":"2024-01-01T12:00:02.250Z"}
```

## Coral Studio Integration

The MCP adapter provides these tools to Coral Studio:

1. **get_price(symbol)** → Returns real-time price data from multiple APIs
2. **analyze_market(symbol, horizon, provider)** → AI-powered market analysis with RAG context
3. **feed_oracle(symbol)** → Posts price data to Solana devnet oracle
4. **health()** → System health status (no auth required)

### Tool Schemas (for Studio registration)

```typescript
// get_price
input: { symbol: string } // 1-32 chars, regex validated
output: { symbol: string, price: number, source: string, ts: number }

// analyze_market (multi-provider)
input: {
  symbol: string,
  horizon?: "1h" | "1d" | "1w",
  provider?: "gemini" | "mistral" | "aimlapi"
}
output: {
  summary: string,
  signals: string[],
  confidence: number, // 0-1
  sources: string[]
}

// feed_oracle
input: { symbol: string }
output: { tx: string, slot: number, digest: string }

// health
input: {}
output: {
  status: "ok",
  versions: { iora: string, mcp?: string },
  uptime_sec: number
}

// receipt (NFT minting)
input: {
  symbol: string,
  price: number,
  tx: string,
  model: string,
  ts: number
}
output: { ok: true, provider: "crossmint", id: string, url?: string }
```

## Technical Architecture

```
Coral Studio
    ↓ (HTTP + HMAC)
MCP Server (Node.js/Express)
    ↓ (CLI spawn + JSON)
IORA Binary (Rust)
    ↓ (Multi-API fetching + Multi-LLM analysis)
External APIs (CMC, CG, etc.) + AI Models (Gemini/Mistral/AI-ML)
    ↓ (Oracle feeds + Receipt minting)
Solana Devnet + Crossmint Staging
```

### Security Features

- **HMAC-SHA256 Authentication** on all tool endpoints (health exempt)
- **Rate Limiting** (30 req/10s general, 3/min oracle feeds)
- **Helmet Security Headers** (CSP, no X-Powered-By, CORS disabled)
- **Request Validation** (256KB body limit, strict JSON schemas)
- **Kill-Switch** (DISABLE_FEED_ORACLE=1 for emergency shutdown)
- **Structured Logging** with reqId tracing (redacted for security)
- **Request Validation** via Zod schemas with strict type checking
- **Timeout Protection** (5s LLM calls, 10s CLI operations, 7s Crossmint calls)

### Production Readiness

- **Zero Dependencies** on Coral SDK (clean HTTP interface)
- **Structured Logging** for monitoring and debugging
- **Docker Support** via docker-compose
- **Configuration Management** via environment variables
- **Graceful Degradation** with proper error handling

## Files Changed

### Core Implementation
- `mcp/src/index.ts` - HTTP server with security & observability
- `mcp/src/lib/spawnIORA.ts` - Hardened CLI bridge
- `mcp/src/schemas.ts` - Strengthened validation schemas
- `mcp/src/mw/security.ts` - Authentication & rate limiting
- `mcp/tests/e2e.real.test.ts` - Real integration tests

### Rust CLI Contracts
- `src/modules/cli.rs` - JSON output handlers for all commands
- `cargo build --release` produces deterministic binary

### Configuration & Docs
- `mcp/.env.example` - Required environment variables
- `MCP_RUNBOOK.md` - Complete deployment guide
- `docker-compose.yml` - Updated with MCP service

## Testing

```bash
# Unit tests
cd mcp && npm test

# Integration tests (requires running server)
cd mcp && VITEST_REAL=1 npm run test:e2e

# Manual testing (see demo script above)
```

## Deployment

```bash
# Build everything
cargo build --release
cd mcp && npm install

# Start services
docker-compose up --build mcp

# Verify
curl http://localhost:7070/tools/health
```

This submission demonstrates a complete, production-ready MCP adapter that enables Coral Protocol agents to leverage IORA's sophisticated crypto data aggregation and AI analysis capabilities.
