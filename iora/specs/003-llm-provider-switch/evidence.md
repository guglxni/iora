# Evidence – Feature 003: LLM Provider Switch

## Implementation Summary

Successfully implemented multi-provider LLM support for market analysis with provider selection through CLI and MCP interface.

## Files Created/Modified

### Rust Core
- `src/modules/llm.rs`: Extended with LlmOutput, run_llm() function, and provider-specific clients
- `src/modules/llm/provider.rs`: New provider enum for MCP (Gemini, Mistral, AimlApi)
- `src/modules/llm/prompt.rs`: JSON schema prompt for structured LLM responses
- `src/modules/cli.rs`: Updated analyze_market command to accept --provider parameter

### Node.js MCP
- `mcp/src/tools/analyze_market.ts`: Pass provider parameter from request to CLI
- `mcp/src/schemas.ts`: AnalyzeIn schema accepts provider enum values

## CLI Functionality

### Provider Selection
```bash
# Test different providers (requires API keys)
$ ./target/release/iora analyze_market --symbol BTC --horizon 1d --provider gemini
$ ./target/release/iora analyze_market --symbol BTC --horizon 1d --provider mistral  
$ ./target/release/iora analyze_market --symbol BTC --horizon 1d --provider aimlapi
```

### Provider Parsing
- Accepts: `gemini`, `mistral`, `aimlapi`
- Maps `mistral`/`aimlapi` to custom LLM provider variants
- Validates provider names with clear error messages

## MCP Interface

### Request Format
```json
{
  "symbol": "BTC",
  "horizon": "1d", 
  "provider": "mistral"
}
```

### Response Format (Unchanged)
```json
{
  "summary": "Market analysis...",
  "signals": ["signal1", "signal2"],
  "confidence": 0.85,
  "sources": ["provider_name"]
}
```

## Environment Variables

```bash
# Gemini (default)
GEMINI_API_KEY=...
GEMINI_BASE=https://generativelanguage.googleapis.com
GEMINI_MODEL=gemini-1.5-flash

# Mistral
MISTRAL_API_KEY=...
MISTRAL_BASE=https://api.mistral.ai
MISTRAL_MODEL=mistral-large-latest

# AI-ML API
AIMLAPI_API_KEY=...
AIMLAPI_BASE=https://api.aimlapi.com
AIMLAPI_MODEL=llama-3.1-70b-instruct
```

## Architecture

### Provider Abstraction
- `run_llm(provider, prompt)` function routes to appropriate client
- Each provider implements same interface with 5-second timeouts
- Structured JSON parsing with fallback field handling

### Error Handling
- Missing API keys return clear error messages
- Invalid provider names caught at CLI level
- LLM response parsing handles provider API differences

### Performance
- 5-second hard timeout per LLM call
- No retries to avoid cost accumulation
- Asynchronous processing for responsiveness

## Testing Results

### Build Success
```bash
$ cargo build --release
✅ Compiles successfully with new LLM provider support
```

### CLI Testing
```bash
$ ./target/release/iora analyze_market --symbol BTC --horizon 1d --provider invalid
Error: Invalid provider: unsupported provider: invalid

$ ./target/release/iora analyze_market --symbol BTC --horizon 1d --provider gemini
⚠️  Gemini API key invalid (expected with placeholder key)
```

### Schema Validation
- MCP request validation accepts new provider values
- Response schema unchanged maintains backward compatibility
- Zod validation prevents invalid requests

## Security Considerations

- API keys stored as environment variables (not hardcoded)
- Provider validation prevents injection attacks
- Timeout limits prevent resource exhaustion
- Error messages sanitized (no key exposure)

## Backward Compatibility

- Existing Gemini functionality unchanged
- Default provider remains Gemini
- JSON response schema identical
- MCP interface extensions are additive

## Integration Status

✅ **Rust CLI**: Provider parameter parsing and LLM routing
✅ **MCP Node.js**: Provider passthrough and validation  
✅ **Environment**: Configuration variables documented
✅ **Error Handling**: Clear messages for missing keys/providers
✅ **Performance**: 5-second timeouts with no retries
✅ **Security**: No hardcoded secrets, input validation
