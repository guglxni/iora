-- Usage logs for billing and analytics
-- Tracks all API usage for billing, monitoring, and analytics
CREATE TABLE usage_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id VARCHAR(255) REFERENCES users(id) ON DELETE SET NULL,
  org_id VARCHAR(255) REFERENCES organizations(id) ON DELETE SET NULL,
  api_key_id UUID REFERENCES api_keys(id) ON DELETE SET NULL,
  endpoint VARCHAR(255) NOT NULL,
  method VARCHAR(10) NOT NULL,
  symbol VARCHAR(50),  -- For crypto queries
  status_code INTEGER NOT NULL,
  response_time_ms INTEGER,
  error_message TEXT,
  request_metadata JSONB,  -- Store additional context
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance and analytics queries
CREATE INDEX idx_usage_logs_user_timestamp ON usage_logs(user_id, created_at DESC);
CREATE INDEX idx_usage_logs_org_timestamp ON usage_logs(org_id, created_at DESC);
CREATE INDEX idx_usage_logs_api_key ON usage_logs(api_key_id);
CREATE INDEX idx_usage_logs_created_at ON usage_logs(created_at DESC);
CREATE INDEX idx_usage_logs_endpoint ON usage_logs(endpoint);
CREATE INDEX idx_usage_logs_status_code ON usage_logs(status_code);
CREATE INDEX idx_usage_logs_symbol ON usage_logs(symbol);

-- Composite indexes for common query patterns
CREATE INDEX idx_usage_logs_user_endpoint ON usage_logs(user_id, endpoint, created_at DESC);
CREATE INDEX idx_usage_logs_org_endpoint ON usage_logs(org_id, endpoint, created_at DESC);

-- Partitioning setup for large scale (commented out for now, can be enabled later)
-- ALTER TABLE usage_logs PARTITION BY RANGE (created_at);

-- Create partitions for the next 12 months (uncomment when needed)
-- CREATE TABLE usage_logs_2025_01 PARTITION OF usage_logs
--   FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

-- Comments for documentation
COMMENT ON TABLE usage_logs IS 'API usage logs for billing, analytics, and monitoring';
COMMENT ON COLUMN usage_logs.request_metadata IS 'Additional context like request headers, IP, etc.';
COMMENT ON COLUMN usage_logs.response_time_ms IS 'Response time in milliseconds for performance monitoring';
