-- API Keys table
-- Stores API keys for programmatic access to IORA services
CREATE TABLE api_keys (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  key_hash VARCHAR(255) NOT NULL UNIQUE,  -- bcrypt hash of actual key
  key_prefix VARCHAR(50) NOT NULL,  -- Display prefix: "iora_pk_abc..."
  user_id VARCHAR(255) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  org_id VARCHAR(255) REFERENCES organizations(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  permissions JSONB DEFAULT '["tools:read"]',
  created_at TIMESTAMPTZ DEFAULT NOW(),
  last_used_at TIMESTAMPTZ,
  expires_at TIMESTAMPTZ,
  is_active BOOLEAN DEFAULT TRUE,
  rate_limit_tier VARCHAR(50) DEFAULT 'free',
  usage_count BIGINT DEFAULT 0
);

-- Indexes for performance-critical queries
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_org_id ON api_keys(org_id);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);
CREATE INDEX idx_api_keys_expires_at ON api_keys(expires_at);
CREATE INDEX idx_api_keys_rate_limit_tier ON api_keys(rate_limit_tier);

-- Composite indexes for common query patterns
CREATE INDEX idx_api_keys_user_active ON api_keys(user_id, is_active);
CREATE INDEX idx_api_keys_org_active ON api_keys(org_id, is_active);

-- Comments for documentation
COMMENT ON TABLE api_keys IS 'API keys for programmatic access to IORA services';
COMMENT ON COLUMN api_keys.key_hash IS 'Bcrypt hash of the actual API key for security';
COMMENT ON COLUMN api_keys.key_prefix IS 'First 8 characters for display purposes';
COMMENT ON COLUMN api_keys.permissions IS 'JSON array of permissions: ["tools:read", "tools:write", etc.]';
COMMENT ON COLUMN api_keys.rate_limit_tier IS 'Rate limiting tier: free, pro, enterprise';
