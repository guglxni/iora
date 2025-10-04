-- Organizations table (synced with Clerk)
-- This table stores organization information for multi-tenant features
CREATE TABLE organizations (
  id VARCHAR(255) PRIMARY KEY,  -- Clerk org ID
  name VARCHAR(255) NOT NULL,
  slug VARCHAR(255) UNIQUE,
  tier VARCHAR(50) DEFAULT 'free' CHECK (tier IN ('free', 'pro', 'enterprise')),
  stripe_subscription_id VARCHAR(255),
  max_api_keys INTEGER DEFAULT 5,
  max_requests_per_month BIGINT DEFAULT 10000,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  is_active BOOLEAN DEFAULT TRUE
);

-- Indexes for performance
CREATE INDEX idx_orgs_slug ON organizations(slug);
CREATE INDEX idx_orgs_tier ON organizations(tier);
CREATE INDEX idx_orgs_is_active ON organizations(is_active);

-- Organization members (many-to-many)
-- Links users to organizations with role-based access
CREATE TABLE organization_members (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  org_id VARCHAR(255) REFERENCES organizations(id) ON DELETE CASCADE,
  user_id VARCHAR(255) REFERENCES users(id) ON DELETE CASCADE,
  role VARCHAR(50) DEFAULT 'member' CHECK (role IN ('owner', 'admin', 'member', 'viewer')),
  joined_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(org_id, user_id)
);

-- Indexes for organization member queries
CREATE INDEX idx_org_members_org_id ON organization_members(org_id);
CREATE INDEX idx_org_members_user_id ON organization_members(user_id);
CREATE INDEX idx_org_members_role ON organization_members(role);

-- Comments for documentation
COMMENT ON TABLE organizations IS 'Organization accounts for multi-tenant features';
COMMENT ON TABLE organization_members IS 'User membership in organizations with role assignments';
COMMENT ON COLUMN organizations.slug IS 'URL-friendly organization identifier';
COMMENT ON COLUMN organizations.max_api_keys IS 'Maximum API keys allowed for this organization';
COMMENT ON COLUMN organizations.max_requests_per_month IS 'Monthly request quota for organization';
