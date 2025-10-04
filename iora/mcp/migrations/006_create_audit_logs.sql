-- Audit logs for security and compliance
-- Comprehensive audit trail for all data access and modifications
CREATE TABLE audit_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id VARCHAR(255),
  action VARCHAR(100) NOT NULL,  -- 'api_key_created', 'tier_upgraded', etc.
  resource_type VARCHAR(50),  -- 'api_key', 'user', 'organization'
  resource_id VARCHAR(255),
  ip_address INET,
  user_agent TEXT,
  changes JSONB,  -- Before/after for updates
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for audit queries
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_logs_ip_address ON audit_logs(ip_address);

-- Composite indexes for common audit queries
CREATE INDEX idx_audit_logs_user_action ON audit_logs(user_id, action, created_at DESC);
CREATE INDEX idx_audit_logs_resource_action ON audit_logs(resource_type, resource_id, action);

-- Comments for documentation
COMMENT ON TABLE audit_logs IS 'Comprehensive audit trail for security and compliance';
COMMENT ON COLUMN audit_logs.changes IS 'JSON object with before/after values for updates';
COMMENT ON COLUMN audit_logs.ip_address IS 'IP address of the request for security tracking';
