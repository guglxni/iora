# Implementation Plan: Expose IORA as a Coral MCP Agent (Studio + Registry ready)

**Branch**: `002-expose-iora-as` | **Date**: 2025-01-18 | **Spec**: [specs/002-expose-iora-as/spec.md](specs/002-expose-iora-as/spec.md)
**Input**: Feature specification from `/specs/002-expose-iora-as/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → Feature spec loaded successfully from specs/002-expose-iora-as/spec.md
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Project Type: Multi-language (Rust + Node.js/TS)
   → Structure Decision: Hybrid - new MCP directory alongside existing Rust project
3. Fill the Constitution Check section based on the content of the constitution document.
   → Constitution emphasizes library-first, CLI interfaces, test-first development
4. Evaluate Constitution Check section below
   → Adding Node.js/TS component aligns with multi-language approach
   → CLI-first design maintained through IORA binary integration
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → No major NEEDS CLARIFICATION - MCP protocol well-documented
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file
7. Re-evaluate Constitution Check section
   → No new violations introduced
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Create a Node.js/TypeScript MCP server that exposes IORA's crypto analysis functionality through standardized tool interfaces. The server will support both CLI mode (spawning IORA binary) and HTTP mode (calling local endpoints), enabling seamless integration with Coral Studio and Registry for the Internet of Agents Hackathon.

## Technical Context
**Language/Version**: Node.js 18+ with TypeScript 5.x, Rust 1.75+ (existing IORA)  
**Primary Dependencies**: @modelcontextprotocol/sdk, zod (schema validation), child_process, node-fetch  
**Storage**: N/A (stateless MCP server, delegates to IORA)  
**Testing**: Jest with schema validation tests  
**Target Platform**: Node.js runtime, Docker containerizable  
**Project Type**: Hybrid multi-language project (Rust + Node.js/TS)  
**Performance Goals**: <2s response time for price queries, <10s for analysis  
**Constraints**: Must work in Coral Studio, support CLI and HTTP modes, JSON I/O only  
**Scale/Scope**: 4 MCP tools, ~500 LOC Node.js/TS, 1 Docker service

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Based on IORA Constitution principles:

**✅ Library-First**: MCP server is a standalone library exposing IORA functionality  
**✅ CLI Interface**: MCP tools provide text I/O protocol, integrate with existing IORA CLI  
**✅ Test-First**: Unit tests for schema validation, integration tests for tool execution  
**✅ Integration Testing**: Contract tests for MCP tool interfaces and IORA integration  
**✅ Observability**: Structured logging for all tool executions and errors

## Project Structure

### Documentation (this feature)
```
specs/002-expose-iora-as/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
│   ├── get_price.json
│   ├── analyze_market.json
│   ├── feed_oracle.json
│   └── health.json
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# NEW: mcp/ directory alongside existing iora/
mcp/
├── package.json
├── tsconfig.json
├── src/
│   ├── index.ts              # MCP server bootstrap
│   ├── tools/
│   │   ├── get_price.ts
│   │   ├── analyze_market.ts
│   │   ├── feed_oracle.ts
│   │   └── health.ts
│   └── types.ts              # Shared interfaces
├── coral.server.config.ts    # Tool manifests & rate limits
├── .env.example
├── Dockerfile
└── tests/
    ├── unit/
    └── integration/

# EXISTING: iora/ (unchanged)
iora/
├── src/
├── Cargo.toml
└── ...
```

**Structure Decision**: Hybrid structure - new `mcp/` directory for Node.js/TS MCP server alongside existing Rust `iora/` project. Clear separation of concerns while enabling Docker composition.

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context**:
   - MCP protocol specifics for Coral integration
   - JSON schema definitions for tool I/O
   - Error handling patterns for CLI vs HTTP modes
   - Docker networking between MCP and IORA services

2. **Generate and dispatch research agents**:
   ```
   For MCP protocol integration:
     Task: "Research Model Context Protocol SDK for Coral Studio compatibility"
   For dual-mode architecture:
     Task: "Find patterns for CLI spawning vs HTTP calling in Node.js"
   For schema validation:
     Task: "Compare Zod vs AJV vs Joi for MCP tool validation"
   For Docker composition:
     Task: "Design networking between MCP and IORA services in docker-compose"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: Use @modelcontextprotocol/sdk for Coral compatibility
   - Rationale: Official SDK ensures Studio and Registry compatibility
   - Alternatives considered: Custom MCP implementation (too complex), other MCP libraries (less mature)

**Output**: research.md with all implementation decisions documented

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - MCP Tool Request/Response schemas
   - Configuration entities (env vars, CLI paths)
   - Error response formats
   - Health status structure

2. **Generate API contracts** from functional requirements:
   - Four MCP tool contracts in JSON Schema format
   - Input validation schemas with Zod
   - Error response contracts
   - Output: `/contracts/*.json` files

3. **Generate contract tests** from contracts:
   - Unit tests for each tool's input/output validation
   - Mock tests for CLI execution paths
   - HTTP endpoint mocking for alternative mode
   - Tests must fail initially (red in TDD cycle)

4. **Extract test scenarios** from user stories:
   - Judge evaluation workflow → integration test
   - Coral Studio connection → e2e test scenario
   - Tool execution validation → contract tests

5. **Update agent file incrementally**:
   - Run `.specify/scripts/bash/update-agent-context.sh cursor`
   - Add Node.js/TS, MCP, Docker context
   - Preserve existing Rust/AI context
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CURSOR.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each tool contract → implementation task [P] (parallel)
- Docker setup → infrastructure task
- CLI vs HTTP mode selection → configuration task
- Schema validation → testing task [P]
- Integration testing → e2e task

**Ordering Strategy**:
- TDD order: Contract tests → implementation → integration tests
- Parallel execution: Tool implementations can be done independently [P]
- Sequential: Docker setup after all tools implemented
- Dependencies: CLI mode first, HTTP mode as enhancement

**Estimated Output**: 15-20 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Multi-language (Rust + Node.js) | MCP requires Node.js, IORA is Rust | Single language would require rewriting IORA or custom MCP |
| Dual execution modes | Flexibility for different deployment scenarios | Single mode limits integration options |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on IORA Constitution v1.0 - See `.specify/memory/constitution.md`*"
