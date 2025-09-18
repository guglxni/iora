# Implementation Tasks: Expose IORA as a Coral MCP Agent

**Feature**: 002-expose-iora-as | **Date**: 2025-01-18
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

## Task List (TDD-First Approach)

### Phase 1: Bootstrap & Infrastructure [SEQUENTIAL]
**Priority**: HIGH | **Estimated**: 2 hours

1. **Create MCP project structure** [INFRA]
   ```
   mkdir mcp/
   cd mcp/
   npm init -y
   npm install @modelcontextprotocol/sdk zod typescript @types/node ts-node-dev jest @types/jest
   npm install -D typescript ts-node-dev jest @types/jest
   ```
   **Files**: package.json, tsconfig.json, src/types.ts
   **Acceptance**: npm install succeeds, TypeScript compiles

2. **Set up Docker configuration** [INFRA]
   ```
   Create Dockerfile, .env.example, docker-compose service
   Add mcp service to root docker-compose.yml
   ```
   **Files**: Dockerfile, .env.example, docker-compose.yml (update)
   **Acceptance**: docker build succeeds

3. **Configure MCP server bootstrap** [INFRA]
   ```
   Create src/index.ts with MCP server initialization
   Add tool registration framework
   ```
   **Files**: src/index.ts, coral.server.config.ts
   **Acceptance**: Server starts without tools, logs "MCP server ready"

### Phase 2: Tool Implementation - CLI Mode [PARALLEL]
**Priority**: HIGH | **Estimated**: 4 hours

4. **Implement get_price tool** [TOOL] [P]
   ```
   CLI mode: spawn iora with args, parse JSON stdout
   Schema: {symbol: string} → {symbol, price, source, ts}
   Error handling: invalid symbol, network issues
   ```
   **Files**: src/tools/get_price.ts, tests/unit/get_price.test.ts
   **Acceptance**: Unit tests pass, CLI execution works

5. **Implement analyze_market tool** [TOOL] [P]
   ```
   CLI mode: spawn iora analyze command with provider selection
   Schema: {symbol, horizon?, provider?} → {summary, signals[], confidence, sources[]}
   Timeout: 10s max for LLM calls
   ```
   **Files**: src/tools/analyze_market.ts, tests/unit/analyze_market.test.ts
   **Acceptance**: Handles all provider options, timeout protection

6. **Implement feed_oracle tool** [TOOL] [P]
   ```
   CLI mode: spawn iora feed-oracle command
   Schema: {symbol: string} → {tx, slot, digest}
   Dry-run flag support for testing
   ```
   **Files**: src/tools/feed_oracle.ts, tests/unit/feed_oracle.test.ts
   **Acceptance**: Returns transaction data or dry-run confirmation

7. **Implement health tool** [TOOL] [P]
   ```
   CLI mode: spawn iora --version + status check
   Schema: {} → {status:"ok", versions:{iora, mcp}, uptimeSec}
   ```
   **Files**: src/tools/health.ts, tests/unit/health.test.ts
   **Acceptance**: Returns accurate version and status info

### Phase 3: Testing & Validation [SEQUENTIAL]
**Priority**: HIGH | **Estimated**: 3 hours

8. **Add comprehensive unit tests** [TEST]
   ```
   Schema validation for all tools (Zod)
   CLI argument parsing tests
   Error response format tests
   Mock child_process for CLI mode
   ```
   **Files**: tests/unit/*.test.ts
   **Acceptance**: All unit tests pass (15+ test cases)

9. **Create integration test setup** [TEST]
   ```
   Test with actual IORA binary in Docker
   E2E tool execution scenarios
   Environment configuration validation
   ```
   **Files**: tests/integration/e2e.test.ts, test scripts
   **Acceptance**: Full tool workflows execute successfully

10. **Add contract validation tests** [TEST]
    ```
    JSON Schema compliance tests
    Tool registration verification
    MCP protocol adherence
    ```
    **Files**: tests/contracts/*.test.ts
    **Acceptance**: All contracts validate correctly

### Phase 4: HTTP Mode Enhancement [OPTIONAL]
**Priority**: MEDIUM | **Estimated**: 2 hours

11. **Implement HTTP mode support** [ENHANCE]
    ```
    Add IORA_HTTP_BASE env var detection
    Replace child_process with fetch calls
    Maintain same tool interfaces
    ```
    **Files**: src/http-client.ts, update all tools
    **Acceptance**: Both CLI and HTTP modes work

12. **Add HTTP integration tests** [TEST]
    ```
    Mock HTTP server for IORA endpoints
    Test HTTP vs CLI mode switching
    Latency comparison tests
    ```
    **Files**: tests/integration/http-mode.test.ts
    **Acceptance**: HTTP mode passes all tests

### Phase 5: Documentation & Demo [SEQUENTIAL]
**Priority**: HIGH | **Estimated**: 2 hours

13. **Create comprehensive README** [DOCS]
    ```
    Installation and setup instructions
    CLI vs HTTP mode configuration
    Docker deployment guide
    Coral Studio integration steps
    ```
    **Files**: mcp/README.md, SUBMISSION.md
    **Acceptance**: Clear setup instructions for judges

14. **Add demo scripts and examples** [DOCS]
    ```
    90-second demo script for hackathon submission
    Example tool calls with expected outputs
    Troubleshooting guide
    ```
    **Files**: demo-script.sh, examples/*.json
    **Acceptance**: Demo script runs successfully

15. **Final validation and polish** [QA]
    ```
    Code formatting and linting
    Performance optimization (<2s responses)
    Error message improvements
    Registry-ready configuration
    ```
    **Files**: Update all files for production readiness
    **Acceptance**: Ready for Coral Registry submission

## Execution Guidelines

### TDD Approach
- Write failing tests first (red)
- Implement minimal code to pass (green)
- Refactor while maintaining tests (refactor)

### Parallel Execution [P]
Tasks marked [P] can be executed in parallel by different developers

### Dependencies
- Tasks 1-3 must complete before tool implementation (4-7)
- Testing (8-10) requires all tools implemented
- HTTP mode (11-12) is optional enhancement
- Documentation (13-15) requires full implementation

### Environment Setup
```bash
# For development
cd mcp/
npm install
npm run dev  # uses ts-node-dev

# For testing
npm test
npm run test:integration

# For production
npm run build
npm start
```

### Success Criteria
- ✅ All 4 MCP tools implemented and tested
- ✅ Both CLI and HTTP modes functional
- ✅ Docker deployment working
- ✅ Coral Studio integration verified
- ✅ Registry-ready configuration
- ✅ Comprehensive documentation and demo

**Total Estimated Time**: 11-13 hours (CLI-only: 9 hours, with HTTP: 13 hours)
**Deliverable**: Production-ready MCP server for IORA crypto analysis agent
