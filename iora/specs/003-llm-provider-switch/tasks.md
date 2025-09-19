# Task Breakdown – Feature 003: LLM Provider Switch

## Phase 1: Rust LLM Provider Extension (3 hours)

### Task 1.1: Extend LlmProvider Enum
- [ ] Add Mistral and AimlApi variants to LlmProvider enum in `src/modules/llm.rs`
- [ ] Update Display trait implementation
- [ ] Add API key validation logic

### Task 1.2: Add HTTP Client Implementations
- [ ] Create MistralApiClient struct with HTTP client and configuration
- [ ] Create AimlApiClient struct with HTTP client and configuration  
- [ ] Implement common LLM client interface/trait
- [ ] Add 5-second timeout configuration to all clients

### Task 1.3: Implement Provider-Specific Logic
- [ ] Add analyze() method implementation for Mistral client
- [ ] Add analyze() method implementation for AI-ML API client
- [ ] Ensure consistent error handling across providers
- [ ] Add provider-specific request/response mapping

### Task 1.4: Update LlmConfig Factory Methods
- [ ] Add LlmConfig::mistral() factory method
- [ ] Add LlmConfig::aimlapi() factory method
- [ ] Update environment variable handling for new providers
- [ ] Add validation for required environment variables

## Phase 2: CLI Provider Parameter (2 hours)

### Task 2.1: Update CLI Argument Parsing
- [ ] Modify analyze_market command to accept --provider parameter
- [ ] Add provider validation against enum values
- [ ] Update help text and command documentation

### Task 2.2: Update Command Handler
- [ ] Modify handle_analyze_market_command to parse provider parameter
- [ ] Add provider string to LlmProvider enum conversion
- [ ] Update LlmConfig creation to use selected provider
- [ ] Add error handling for invalid provider names

## Phase 3: Node.js Provider Passthrough (1 hour)

### Task 3.1: Update Request Schema
- [ ] Ensure AnalyzeIn schema accepts new provider values
- [ ] Update Zod validation for provider enum
- [ ] Add schema tests for new provider options

### Task 3.2: Modify Tool Implementation
- [ ] Update analyze_market.ts to extract provider from request
- [ ] Modify spawnIORA call to include provider in CLI arguments
- [ ] Ensure proper argument ordering for CLI command

## Phase 4: Testing & Validation (2 hours)

### Task 4.1: Unit Tests (Rust)
- [ ] Add tests for LlmProvider enum creation and validation
- [ ] Add tests for LlmConfig factory methods
- [ ] Add tests for API key environment variable handling
- [ ] Add tests for timeout behavior and error handling

### Task 4.2: Integration Tests (Node.js)
- [ ] Add tests for provider parameter validation in schemas
- [ ] Add tests for CLI argument passing in analyze_market tool
- [ ] Add tests for error responses when API keys are missing
- [ ] Add tests for invalid provider name handling

### Task 4.3: Live Smoke Tests (Optional)
- [ ] Add conditional tests that run only when API keys are present
- [ ] Validate different model IDs returned in sources array
- [ ] Performance testing for latency requirements
- [ ] Load testing with 3x concurrent requests

## Phase 5: Documentation & Deployment (1 hour)

### Task 5.1: Update Documentation
- [ ] Add MISTRAL_API_KEY and AIMLAPI_API_KEY to README.md
- [ ] Update MCP_RUNBOOK.md with provider examples
- [ ] Add provider selection examples to documentation

### Task 5.2: Create Evidence & Retro
- [ ] Create evidence.md with test results and examples
- [ ] Create retro.md with lessons learned
- [ ] Document any deviations from original plan

## Parallel Execution Markers

### Can Run in Parallel
- Task 1.1 (enum extension) can run with Task 3.1 (schema updates)
- Task 4.1 (Rust unit tests) can run with Task 4.2 (Node integration tests)
- Task 5.1 (documentation) can start early and run with implementation

### Sequential Dependencies
- Task 1.2 depends on Task 1.1 (enum definition)
- Task 1.3 depends on Task 1.2 (HTTP client implementations)
- Task 1.4 depends on Task 1.3 (provider implementations)
- Task 2.1-2.2 depend on Task 1.4 (LlmConfig updates)
- Task 3.2 depends on Task 2.2 (CLI parameter support)
- Task 4.3 depends on Task 3.2 (end-to-end integration)
- Task 5.2 depends on all implementation tasks

## Quality Gates

### Before Phase 2
- [ ] LlmProvider enum extended with new variants
- [ ] HTTP client implementations compile
- [ ] Basic analyze() method stubs work

### Before Phase 3
- [ ] CLI accepts provider parameter
- [ ] LlmConfig factory methods work for all providers
- [ ] Unit tests pass for provider selection

### Before Phase 4
- [ ] Node.js passes provider to CLI
- [ ] End-to-end flow works (may use mock responses)
- [ ] Error handling works for missing keys

### Before Phase 5
- [ ] All unit and integration tests pass
- [ ] Live smoke tests work (when API keys available)
- [ ] Performance requirements met

## Risk Indicators

### Red Flags
- HTTP client implementations taking >2 hours each
- API differences requiring significant interface changes
- Timeout handling causing complex retry logic
- Environment variable handling breaking existing functionality

### Mitigation
- Start with minimal HTTP client implementations
- Use existing Gemini client as template
- Test timeout behavior early
- Run existing tests after each change

## Definition of Done

- [ ] CLI command: `iora analyze_market BTC 1d mistral` works
- [ ] MCP endpoint accepts: `{"symbol":"BTC","horizon":"1d","provider":"mistral"}`
- [ ] Different providers return different model IDs in sources array
- [ ] JSON output schema unchanged from Feature 002
- [ ] Unit tests cover provider selection and error paths
- [ ] Documentation includes environment variable setup
- [ ] Performance meets latency budget (p50 ≤ 3.5s)
- [ ] Load testing passes (3x concurrent under 10s budget)
