-- Billing events for Stripe integration
-- Stores billing events for subscription management and revenue tracking
CREATE TABLE billing_events (
  id BIGSERIAL PRIMARY KEY,
  user_id VARCHAR(255) REFERENCES users(id) ON DELETE SET NULL,
  org_id VARCHAR(255) REFERENCES organizations(id) ON DELETE SET NULL,
  event_type VARCHAR(50) NOT NULL,  -- 'subscription_created', 'payment_success', etc.
  stripe_event_id VARCHAR(255) UNIQUE,
  amount_cents INTEGER,
  currency VARCHAR(3) DEFAULT 'USD',
  status VARCHAR(50),  -- 'pending', 'completed', 'failed'
  metadata JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  processed_at TIMESTAMPTZ
);

-- Indexes for billing queries
CREATE INDEX idx_billing_events_user_id ON billing_events(user_id);
CREATE INDEX idx_billing_events_org_id ON billing_events(org_id);
CREATE INDEX idx_billing_events_created_at ON billing_events(created_at DESC);
CREATE INDEX idx_billing_events_stripe_id ON billing_events(stripe_event_id);
CREATE INDEX idx_billing_events_status ON billing_events(status);
CREATE INDEX idx_billing_events_event_type ON billing_events(event_type);

-- Composite indexes for common billing queries
CREATE INDEX idx_billing_events_user_status ON billing_events(user_id, status, created_at DESC);
CREATE INDEX idx_billing_events_org_status ON billing_events(org_id, status, created_at DESC);

-- Comments for documentation
COMMENT ON TABLE billing_events IS 'Billing events for subscription management and revenue tracking';
COMMENT ON COLUMN billing_events.stripe_event_id IS 'Stripe webhook event ID for deduplication';
COMMENT ON COLUMN billing_events.amount_cents IS 'Amount in cents (e.g., $10.00 = 1000)';
COMMENT ON COLUMN billing_events.metadata IS 'Additional Stripe event data';
