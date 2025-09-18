# Implementation Plan – Feature 003: LLM Provider Switch

## Technical Context

**Current State**: IORA supports only Gemini AI for market analysis via hardcoded `LlmConfig::gemini()` calls.

**Target State**: Support for Gemini, Mistral, and AI-ML API with provider selection through CLI and MCP interface.

**Constraints**:
- Keep existing JSON output schema unchanged
- Hard 5s HTTP timeout per LLM call, retries=0
- Provider selection plumbed from Node → CLI → Rust
- Unit tests for routing + missing key errors

## Constitution Check

✅ **Business Value**: Partner tech integration for hackathon judging
✅ **Technical Feasibility**: HTTP client pattern already exists for Gemini
✅ **Scope Control**: Focused on analysis endpoint only
✅ **Testing Strategy**: Unit tests + optional live smoke tests

## Phase 0: Design & Research

### Research Tasks
- [ ] Review Mistral API documentation and authentication
- [ ] Review AI-ML API documentation and authentication  
- [ ] Analyze current Gemini integration for extension points
- [ ] Define common LLM client interface

### Design Decisions
- [ ] Extend LlmProvider enum with Mistral and AimlApi variants
- [ ] Add HTTP client implementations for each provider
- [ ] Define timeout and retry policies
- [ ] Design error handling for missing API keys

## Phase 1: Rust LLM Provider Extension

### Files to Modify
- [ ] `src/modules/llm.rs`: 
  - Add Mistral and AimlApi to LlmProvider enum
  - Add HTTP client implementations
  - Add timeout handling (5s hard limit)
  - Add API key validation

### Implementation Steps
- [ ] Extend LlmProvider enum with new variants
- [ ] Add HTTP client structs for Mistral and AI-ML API
- [ ] Implement analyze() method for each provider
- [ ] Add API key environment variable handling
- [ ] Update error types for provider-specific failures

## Phase 2: CLI Provider Parameter

### Files to Modify  
- [ ] `src/modules/cli.rs`:
  - Update analyze_market command to accept provider parameter
  - Add provider validation
  - Pass provider to LLM config creation

### Implementation Steps
- [ ] Update CLI argument parsing for provider parameter
- [ ] Add provider validation (enum matching)
- [ ] Modify analyze_market handler to create appropriate LlmConfig
- [ ] Update help text and error messages

## Phase 3: Node.js Provider Passthrough

### Files to Modify
- [ ] `mcp/src/tools/analyze_market.ts`:
  - Extract provider from request body
  - Pass provider to CLI arguments
- [ ] `mcp/src/schemas.ts`:
  - Ensure provider validation allows new options

### Implementation Steps
- [ ] Update request schema validation
- [ ] Modify spawnIORA call to include provider argument
- [ ] Add provider parameter to CLI argument array
- [ ] Test provider passthrough end-to-end

## Phase 4: Testing & Validation

### Unit Tests (Rust)
- [ ] Add tests for provider enum creation
- [ ] Add tests for API key validation
- [ ] Add tests for timeout behavior
- [ ] Add tests for HTTP client implementations

### Integration Tests (Node.js)
- [ ] Add tests for provider parameter validation
- [ ] Add tests for CLI argument passing
- [ ] Add tests for error handling (missing keys)

### Live Tests (Optional)
- [ ] Add smoke tests gated by API key presence
- [ ] Validate different model IDs in sources array
- [ ] Performance testing under load (3x concurrent)

## Phase 5: Documentation & Deployment

### Files to Create/Update
- [ ] `README.md`: Add environment variable documentation
- [ ] `MCP_RUNBOOK.md`: Update provider examples
- [ ] `specs/003-llm-provider-switch/evidence.md`: Test evidence
- [ ] `specs/003-llm-provider-switch/retro.md`: Lessons learned

### Deployment Checklist
- [ ] Environment variables documented
- [ ] Provider selection examples added
- [ ] Error messages user-friendly
- [ ] Fallback behavior defined

## Risk Mitigation

### Technical Risks
- **API Compatibility**: Different LLM APIs have different request/response formats
- **Rate Limiting**: New providers may have different rate limits
- **Cost Management**: API costs vary between providers

### Mitigation Strategies
- **Interface Design**: Common LLM client interface abstracts provider differences  
- **Timeout Discipline**: Hard 5s timeout prevents hanging requests
- **Error Handling**: Clear error messages for missing/invalid API keys
- **Testing**: Comprehensive unit tests before integration

## Success Criteria

### Functional
- [ ] CLI accepts `iora analyze_market BTC 1d mistral`
- [ ] MCP accepts `{"symbol":"BTC","horizon":"1d","provider":"mistral"}`
- [ ] Different providers return different model IDs in sources
- [ ] JSON output schema unchanged

### Performance  
- [ ] Individual requests complete in ≤ 3.5s (p50)
- [ ] 3x concurrent load completes under 10s budget
- [ ] No memory leaks or resource exhaustion

### Quality
- [ ] Unit test coverage for provider selection
- [ ] Integration tests for end-to-end flow
- [ ] Error handling for missing API keys
- [ ] Documentation updated with examples

## Dependencies

- **External APIs**: Mistral API, AI-ML API access
- **Environment**: API keys for testing
- **Testing**: Optional live API testing infrastructure

## Timeline Estimate

- Phase 0: 1 hour (research & design)
- Phase 1: 3 hours (Rust LLM extension)
- Phase 2: 2 hours (CLI parameter)
- Phase 3: 1 hour (Node passthrough)  
- Phase 4: 2 hours (testing)
- Phase 5: 1 hour (documentation)

**Total: 10 hours** for complete implementation and testing.
