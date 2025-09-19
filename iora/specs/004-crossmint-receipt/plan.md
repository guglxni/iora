# Implementation Plan – Feature 004: Crossmint Receipts

## Technical Context

**Current State**: feed_oracle returns transaction data but no persistent user-visible receipt.

**Target State**: Automatic NFT minting after successful oracle updates for auditability.

**Integration Points**: 
- Crossmint custodial minting API
- Devnet deployment only
- Metadata includes oracle transaction details

## Phase 1: Crossmint Integration (Node.js)

### Implementation Steps
- [ ] Add Crossmint HTTP client to MCP server
- [ ] Create receipt minting function with metadata mapping
- [ ] Add POST /receipt endpoint to MCP server
- [ ] Implement error handling for Crossmint failures

### Files to Modify
- [ ] `mcp/src/lib/crossmint.ts`: Crossmint API client
- [ ] `mcp/src/index.ts`: Add /receipt endpoint
- [ ] `mcp/src/tools/feed_oracle.ts`: Call receipt minting after success

## Phase 2: Metadata Schema

### Implementation Steps  
- [ ] Define receipt NFT metadata structure
- [ ] Map oracle transaction data to NFT attributes
- [ ] Add provider/model information to metadata
- [ ] Validate metadata schema

## Phase 3: Testing & Deployment

### Implementation Steps
- [ ] Unit tests for metadata mapping
- [ ] Integration tests for Crossmint API calls
- [ ] Live test on devnet (gated by environment)
- [ ] Documentation updates

## Success Criteria

- [ ] feed_oracle triggers NFT minting
- [ ] Receipt visible on Crossmint devnet
- [ ] Metadata includes all required fields
- [ ] Graceful failure handling
