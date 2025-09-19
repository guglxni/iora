# FEATURE SPEC

Feature-ID: 005-coral-native-binding
Title: Replace HTTP shim with native Coral MCP server

## Problem
HTTP shim requires manual endpoint discovery. Coral Studio should auto-discover IORA tools via native MCP protocol.

## Goals (Must)
1) Native Coral MCP server using official MCP server API
2) Tool auto-discovery: get_price, analyze_market, feed_oracle, health
3) Preserve HMAC authentication on inbound calls
4) mcp.config.json with tool schemas for Studio integration
5) README documentation for Coral Studio connection

## Interfaces
- mcp/coral.server.ts: MCP server implementation
- mcp/mcp.config.json: Tool manifests with schemas
- README: "Connect from Coral Studio" section

## Acceptance
- Coral Studio connects natively, tools visible in UI
- All tools run end-to-end on devnet
- HMAC authentication preserved
- No HTTP endpoints exposed (pure MCP)

## Test Plan
- Unit: MCP server initialization
- Integration: Studio connection and tool execution
- E2E: Full flows with real API calls
