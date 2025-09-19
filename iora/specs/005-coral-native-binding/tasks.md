# Task Breakdown – Feature 005: Coral Native Binding

## Phase 1: MCP Server Foundation (1.5 hours)

### Task 1.1: Install Coral MCP SDK
- [ ] Research official Coral MCP server SDK
- [ ] Add SDK dependency to package.json
- [ ] Install and verify SDK compatibility

### Task 1.2: Create Basic MCP Server
- [ ] Create mcp/coral.server.ts with server initialization
- [ ] Set up MCP transport and connection handling
- [ ] Implement basic server lifecycle (start/stop)

### Task 1.3: HMAC Authentication Integration
- [ ] Adapt existing HMAC middleware for MCP protocol
- [ ] Implement session-based authentication
- [ ] Test authentication flow with MCP calls

## Phase 2: Tool Registration (1.5 hours)

### Task 2.1: Tool Schema Definition
- [ ] Define MCP tool schemas for all 4 tools
- [ ] Map Zod schemas to MCP tool definitions
- [ ] Add tool descriptions and parameter documentation

### Task 2.2: Handler Adaptation
- [ ] Convert existing tool handlers to MCP format
- [ ] Implement tool execution with proper error handling
- [ ] Preserve existing business logic and validation

### Task 2.3: Tool Registration
- [ ] Register all tools with MCP server
- [ ] Implement tool discovery and listing
- [ ] Test tool availability and metadata

## Phase 3: Studio Integration (0.5 hours)

### Task 3.1: Configuration File
- [ ] Create mcp/mcp.config.json with tool manifests
- [ ] Include connection details and schemas
- [ ] Document Studio integration steps

### Task 3.2: README Updates
- [ ] Add "Connect from Coral Studio" section
- [ ] Include step-by-step Studio setup instructions
- [ ] Add screenshots or code examples

## Phase 4: Migration & Testing (0.5 hours)

### Task 4.1: Remove HTTP Shim
- [ ] Deprecate HTTP endpoints
- [ ] Update startup scripts to use native MCP
- [ ] Ensure backward compatibility during transition

### Task 4.2: Comprehensive Testing
- [ ] Test Studio connection and tool discovery
- [ ] Validate end-to-end tool execution
- [ ] Performance testing with MCP protocol

## Quality Gates

- [ ] MCP server starts without HTTP endpoints
- [ ] Coral Studio auto-discovers all tools
- [ ] Tool execution works via native MCP
- [ ] HMAC authentication preserved
- [ ] README includes Studio setup instructions

## Risk Mitigation

- Keep HTTP shim as fallback during development
- Test extensively with Studio before full migration
- Document any protocol differences or limitations
- Maintain compatibility with existing tool logic

## Success Metrics

✅ Studio auto-discovers tools without manual configuration
✅ All tools execute successfully via MCP protocol
✅ HMAC authentication works with session-based auth
✅ Tool schemas properly defined and validated
✅ README provides clear Studio integration steps
