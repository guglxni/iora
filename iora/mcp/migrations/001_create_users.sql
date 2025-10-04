-- Users table (synced with Clerk)
-- This table stores user information synchronized with Clerk authentication
CREATE TABLE users (
  id VARCHAR(255) PRIMARY KEY,  -- Clerk user ID
  email VARCHAR(255) NOT NULL UNIQUE,
  first_name VARCHAR(255),
  last_name VARCHAR(255),
  tier VARCHAR(50) DEFAULT 'free' CHECK (tier IN ('free', 'pro', 'enterprise')),
  stripe_customer_id VARCHAR(255),
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  last_login_at TIMESTAMPTZ,
  is_active BOOLEAN DEFAULT TRUE
);

-- Indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_tier ON users(tier);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Comments for documentation
COMMENT ON TABLE users IS 'User accounts synchronized with Clerk authentication system';
COMMENT ON COLUMN users.id IS 'Clerk user ID - primary key';
COMMENT ON COLUMN users.tier IS 'User subscription tier: free, pro, enterprise';
COMMENT ON COLUMN users.stripe_customer_id IS 'Stripe customer ID for billing integration';
