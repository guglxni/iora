# Implementation Plan – Feature 005: Coral Native Binding

## Technical Context

**Current State**: HTTP-based MCP shim with manual endpoint configuration
**Target State**: Native Coral MCP server with automatic tool discovery
**Integration**: Official Coral MCP server SDK with HMAC authentication

## Phase 1: Coral MCP Server Setup

### Implementation Steps
- [ ] Install official Coral MCP server SDK
- [ ] Create coral.server.ts with MCP server initialization
- [ ] Register all tools (get_price, analyze_market, feed_oracle, health)
- [ ] Implement HMAC authentication middleware for MCP calls

### Files to Create
- [ ] mcp/package.json: Add Coral MCP SDK dependency
- [ ] mcp/coral.server.ts: Native MCP server implementation
- [ ] mcp/mcp.config.json: Tool manifests and schemas

## Phase 2: Tool Registration

### Implementation Steps
- [ ] Convert existing tool handlers to MCP tool format
- [ ] Register tools with proper schemas and descriptions
- [ ] Implement tool execution with error handling
- [ ] Preserve HMAC authentication for security

### Schema Mapping
- [ ] get_price: Input {symbol}, Output {symbol, price, source, ts}
- [ ] analyze_market: Input {symbol, horizon?, provider?}, Output analysis JSON
- [ ] feed_oracle: Input {symbol}, Output {tx, slot, digest}
- [ ] health: Input {}, Output system status

## Phase 3: Studio Integration

### Implementation Steps
- [ ] Create mcp.config.json for Studio auto-discovery
- [ ] Update README with Studio connection instructions
- [ ] Test Studio connection and tool visibility
- [ ] Validate end-to-end tool execution

## Phase 4: Migration & Testing

### Implementation Steps
- [ ] Migrate from HTTP shim to native MCP
- [ ] Remove HTTP endpoints (keep only MCP)
- [ ] Update environment configuration
- [ ] Comprehensive testing with Studio

## Success Criteria

- [ ] Coral Studio auto-discovers IORA tools
- [ ] Tools execute via native MCP protocol
- [ ] HMAC authentication works with MCP calls
- [ ] No manual HTTP endpoint configuration needed
- [ ] All existing functionality preserved

## Timeline: 4 hours

Phase 1: 1.5 hours (SDK setup and basic server)
Phase 2: 1.5 hours (Tool registration and schemas)
Phase 3: 0.5 hours (Studio integration)
Phase 4: 0.5 hours (Testing and migration)
