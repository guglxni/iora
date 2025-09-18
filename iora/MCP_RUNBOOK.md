# IORA MCP Adapter - Production Runbook

This document explains how to deploy and use the production-grade Coral MCP adapter for IORA.

## Prerequisites

- Node.js 20+
- Rust toolchain
- API keys for cryptocurrency data providers (CoinMarketCap, CoinGecko, etc.)
- API key for LLM provider (Gemini, Mistral, or AIMLAPI)
- Solana wallet and RPC endpoint for oracle feeds

## Build IORA Binary

```bash
cd iora
cargo build --release
```

This creates `target/release/iora` binary.

## Configure Environment

Create `.env` file in the `mcp/` directory:

```bash
cd mcp
cp .env.example .env
```

Edit `.env`:

```bash
IORA_BIN=../target/release/iora
CORAL_SHARED_SECRET=your_secure_random_secret_here
LLM_PROVIDER=gemini
LOG_LEVEL=info
```

## Install Dependencies

```bash
cd mcp
npm install
```

## Start MCP Server

```bash
cd mcp
npm run dev
```

Server will start on port 7070 with structured logging.

## Test Health Endpoint

```bash
curl -s http://localhost:7070/tools/health | jq
```

Expected response:
```json
{
  "ok": true,
  "data": {
    "status": "ok",
    "versions": {
      "iora": "0.1.0",
      "mcp": "1.0.0"
    },
    "uptime_sec": 0
  }
}
```

## Test Price Endpoint (with HMAC auth)

Generate HMAC signature:

```bash
# Using shell commands
body='{"symbol":"BTC"}'
secret="your_secure_random_secret_here"
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$secret" | awk '{print $2}')

curl -s -H "content-type: application/json" \
     -H "x-iora-signature: $sig" \
     -d "$body" \
     http://localhost:7070/tools/get_price | jq
```

Expected response:
```json
{
  "ok": true,
  "data": {
    "symbol": "BTC",
    "price": 45000.0,
    "source": "CoinMarketCap",
    "ts": 1703123456
  }
}
```

## Test Analysis Endpoint

```bash
body='{"symbol":"BTC","horizon":"1d","provider":"gemini"}'
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$secret" | awk '{print $2}')

curl -s -H "content-type: application/json" \
     -H "x-iora-signature: $sig" \
     -d "$body" \
     http://localhost:7070/tools/analyze_market | jq
```

Expected response:
```json
{
  "ok": true,
  "data": {
    "summary": "Bitcoin shows strong bullish momentum...",
    "signals": ["price_analysis", "market_context"],
    "confidence": 0.85,
    "sources": ["CoinMarketCap"]
  }
}
```

## Test Oracle Feed Endpoint

```bash
body='{"symbol":"BTC"}'
sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$secret" | awk '{print $2}')

curl -s -H "content-type: application/json" \
     -H "x-iora-signature: $sig" \
     -d "$body" \
     http://localhost:7070/tools/feed_oracle | jq
```

Expected response:
```json
{
  "ok": true,
  "data": {
    "tx": "5K8q8cB9dXwJ...",
    "slot": 123456789,
    "digest": "abc123..."
  }
}
```

## Docker Deployment

```bash
# Build and run with Docker Compose
docker-compose up --build mcp
```

## Monitoring

The MCP server emits structured JSON logs:

```json
{"level":"info","reqId":"uuid","method":"POST","path":"/tools/get_price","ip":"127.0.0.1","timestamp":"2024-01-01T12:00:00.000Z"}
{"level":"info","reqId":"uuid","tool":"get_price","exitCode":0,"duration_ms":1250,"timestamp":"2024-01-01T12:00:01.250Z"}
{"level":"info","reqId":"uuid","method":"POST","path":"/tools/get_price","status":200,"duration_ms":1275,"timestamp":"2024-01-01T12:00:01.275Z"}
```

## Troubleshooting

### Common Issues

1. **"IORA_BIN missing"**: Ensure `IORA_BIN` environment variable points to the compiled binary.

2. **Authentication failures**: Verify `CORAL_SHARED_SECRET` matches between client and server.

3. **Rate limiting**: Server allows 30 requests per 10 seconds. Wait or increase limits if needed.

4. **JSON parsing errors**: Ensure IORA binary outputs valid JSON to stdout.

5. **Timeout errors**: Commands taking >10 seconds will be killed. Consider optimizing or increasing timeout.

### Debug Commands

```bash
# Test IORA binary directly
./target/release/iora get_price BTC

# Check logs
tail -f /dev/null & npm run dev 2>&1 | jq

# Test without auth (health only)
curl http://localhost:7070/tools/health
```

## Security Notes

- HMAC-SHA256 authentication required for all endpoints except health
- Rate limiting prevents abuse (30 req/10s)
- Request/response size limits prevent DoS
- All errors are sanitized to prevent information leakage
- Logs include request IDs for tracing
