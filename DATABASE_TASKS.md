# IORA Database Implementation Plan

**Document Version:** 1.0  
**Created:** October 4, 2025  
**Status:** Ready for Implementation  
**Priority:** CRITICAL (API keys currently lost on restart)

---

## 📋 Executive Summary

### Current State
- ✅ **Typesense**: Running (Vector DB for RAG)
- ✅ **Clerk**: Active (External auth SaaS)
- ⚠️ **PostgreSQL**: Configured but INACTIVE
- ⚠️ **Redis**: Configured but INACTIVE
- ❌ **API Keys**: Stored in-memory (LOST ON RESTART)

### Target State
- ✅ **Typesense**: Keep as-is (optimal for RAG)
- ✅ **Clerk**: Keep as-is (managed auth)
- ✅ **PostgreSQL**: ACTIVATE (persistent data)
- ✅ **Redis**: ACTIVATE (caching/rate limiting)
- ✅ **API Keys**: Persisted in PostgreSQL

### Business Impact
- 🔴 **Current Risk**: User API keys disappear on server restart
- 🟢 **After Implementation**: Production-ready persistence
- 💰 **Cost**: $0 (self-hosted) vs $139+/mo (managed services)

---

## 🎯 Phase 1: Critical Database Setup (Week 1)

**Goal:** Enable PostgreSQL for persistent storage of user data, API keys, and usage logs.

**Priority:** CRITICAL  
**Estimated Time:** 3-5 days  
**Dependencies:** Docker, Node.js, Rust toolchain

---

### Task 1.1: PostgreSQL Infrastructure Setup

**Objective:** Activate and configure PostgreSQL for production use.

#### Subtasks:

- [x] **1.1.1** Start PostgreSQL container
  ```bash
  cd /Volumes/MacExt/desktop-backup-sep-24/iora
  docker-compose --profile full up -d postgres
  ```
  - Verify container is running: `docker ps | grep postgres`
  - Check logs: `docker logs iora-postgres`
  - **Success Criteria**: Container status "Up" and healthy

- [x] **1.1.2** Verify database connectivity
  ```bash
  # Connect to PostgreSQL
  docker exec -it iora-postgres psql -U iora_user -d iora_dev
  ```
  - Run test query: `SELECT version();`
  - List databases: `\l`
  - Exit: `\q`
  - **Success Criteria**: Successful connection and query execution

- [x] **1.1.3** Configure connection pooling
  - Create database configuration file: `iora/mcp/src/config/database.ts`
  - Set connection pool size: 10 connections (development)
  - Set connection timeout: 30 seconds
  - Enable SSL for production: `sslmode=require`
  - **Success Criteria**: Connection pool configuration file created

- [x] **1.1.4** Set up environment variables
  - Add to `iora/mcp/.env`:
    ```bash
    DATABASE_URL=postgresql://iora_user:iora_password_2024@localhost:5432/iora_dev
    DATABASE_POOL_MIN=2
    DATABASE_POOL_MAX=10
    DATABASE_CONNECTION_TIMEOUT=30000
    ```
  - Document in `iora/mcp/api-keys-template.env`
  - **Success Criteria**: Environment variables configured and documented

---

### Task 1.2: Database Schema Design & Migration

**Objective:** Create production-ready database schema for IORA.

#### Subtasks:

- [x] **1.2.1** Create migrations directory structure
  ```bash
  mkdir -p iora/mcp/migrations
  mkdir -p iora/mcp/src/db
  ```
  - **Success Criteria**: Directory structure created

- [x] **1.2.2** Design core schema (users table)
  - Create migration: `iora/mcp/migrations/001_create_users.sql`
  ```sql
  -- Users table (synced with Clerk)
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

  CREATE INDEX idx_users_email ON users(email);
  CREATE INDEX idx_users_tier ON users(tier);
  CREATE INDEX idx_users_created_at ON users(created_at);
  ```
  - **Success Criteria**: Users table migration file created

- [x] **1.2.3** Design organizations schema
  - Create migration: `iora/mcp/migrations/002_create_organizations.sql`
  ```sql
  -- Organizations table (synced with Clerk)
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

  CREATE INDEX idx_orgs_slug ON organizations(slug);
  CREATE INDEX idx_orgs_tier ON organizations(tier);

  -- Organization members (many-to-many)
  CREATE TABLE organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id VARCHAR(255) REFERENCES organizations(id) ON DELETE CASCADE,
    user_id VARCHAR(255) REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) DEFAULT 'member' CHECK (role IN ('owner', 'admin', 'member', 'viewer')),
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(org_id, user_id)
  );

  CREATE INDEX idx_org_members_org_id ON organization_members(org_id);
  CREATE INDEX idx_org_members_user_id ON organization_members(user_id);
  ```
  - **Success Criteria**: Organizations and membership tables created

- [x] **1.2.4** Design API keys schema (CRITICAL)
  - Create migration: `iora/mcp/migrations/003_create_api_keys.sql`
  ```sql
  -- API Keys table
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

  CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
  CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
  CREATE INDEX idx_api_keys_org_id ON api_keys(org_id);
  CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);
  CREATE INDEX idx_api_keys_expires_at ON api_keys(expires_at);
  ```
  - **Success Criteria**: API keys table with proper indexes created

- [x] **1.2.5** Design usage logging schema
  - Create migration: `iora/mcp/migrations/004_create_usage_logs.sql`
  ```sql
  -- Usage logs for billing and analytics
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

  -- Partitioning by month for performance
  CREATE INDEX idx_usage_logs_user_timestamp ON usage_logs(user_id, created_at DESC);
  CREATE INDEX idx_usage_logs_org_timestamp ON usage_logs(org_id, created_at DESC);
  CREATE INDEX idx_usage_logs_api_key ON usage_logs(api_key_id);
  CREATE INDEX idx_usage_logs_created_at ON usage_logs(created_at DESC);
  CREATE INDEX idx_usage_logs_endpoint ON usage_logs(endpoint);

  -- Consider partitioning for large scale
  -- ALTER TABLE usage_logs PARTITION BY RANGE (created_at);
  ```
  - **Success Criteria**: Usage logs table with time-series optimization

- [x] **1.2.6** Design billing events schema
  - Create migration: `iora/mcp/migrations/005_create_billing.sql`
  ```sql
  -- Billing events for Stripe integration
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

  CREATE INDEX idx_billing_events_user_id ON billing_events(user_id);
  CREATE INDEX idx_billing_events_org_id ON billing_events(org_id);
  CREATE INDEX idx_billing_events_created_at ON billing_events(created_at DESC);
  CREATE INDEX idx_billing_events_stripe_id ON billing_events(stripe_event_id);
  ```
  - **Success Criteria**: Billing events table ready for Stripe webhooks

- [x] **1.2.7** Create audit log schema (security)
  - Create migration: `iora/mcp/migrations/006_create_audit_logs.sql`
  ```sql
  -- Audit logs for security and compliance
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

  CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
  CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at DESC);
  CREATE INDEX idx_audit_logs_action ON audit_logs(action);
  CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
  ```
  - **Success Criteria**: Audit logging infrastructure in place

- [x] **1.2.8** Run all migrations
  ```bash
  # Connect to PostgreSQL
  docker exec -it iora-postgres psql -U iora_user -d iora_dev

  # Run migrations in order
  \i /path/to/migrations/001_create_users.sql
  \i /path/to/migrations/002_create_organizations.sql
  \i /path/to/migrations/003_create_api_keys.sql
  \i /path/to/migrations/004_create_usage_logs.sql
  \i /path/to/migrations/005_create_billing.sql
  \i /path/to/migrations/006_create_audit_logs.sql

  # Verify tables
  \dt

  # Check indexes
  \di
  ```
  - **Success Criteria**: All tables created with proper indexes

---

### Task 1.3: TypeScript Database Client Setup

**Objective:** Integrate PostgreSQL with the Node.js/TypeScript MCP server.

#### Subtasks:

- [x] **1.3.1** Install PostgreSQL client libraries
  ```bash
  cd iora/mcp
  npm install pg @types/pg
  npm install --save-dev @types/node
  ```
  - **Success Criteria**: Dependencies installed in package.json

- [x] **1.3.2** Create database connection module
  - Create file: `iora/mcp/src/config/database.ts`
  ```typescript
  import { Pool, PoolConfig } from 'pg';

  const poolConfig: PoolConfig = {
    connectionString: process.env.DATABASE_URL,
    min: parseInt(process.env.DATABASE_POOL_MIN || '2'),
    max: parseInt(process.env.DATABASE_POOL_MAX || '10'),
    idleTimeoutMillis: 30000,
    connectionTimeoutMillis: parseInt(process.env.DATABASE_CONNECTION_TIMEOUT || '30000'),
  };

  export const pool = new Pool(poolConfig);

  // Graceful shutdown
  process.on('SIGTERM', async () => {
    await pool.end();
  });

  // Connection health check
  export async function checkDatabaseHealth(): Promise<boolean> {
    try {
      const client = await pool.connect();
      await client.query('SELECT 1');
      client.release();
      return true;
    } catch (error) {
      console.error('Database health check failed:', error);
      return false;
    }
  }
  ```
  - **Success Criteria**: Database connection module created

- [x] **1.3.3** Create database query utilities
  - Create file: `iora/mcp/src/db/queries.ts`
  ```typescript
  import { pool } from './connection';
  import { QueryResult } from 'pg';

  export async function query<T = any>(
    text: string,
    params?: any[]
  ): Promise<QueryResult<T>> {
    const start = Date.now();
    try {
      const result = await pool.query<T>(text, params);
      const duration = Date.now() - start;
      console.log('Query executed', { text, duration, rows: result.rowCount });
      return result;
    } catch (error) {
      console.error('Query error', { text, error });
      throw error;
    }
  }

  export async function transaction<T>(
    callback: (client: any) => Promise<T>
  ): Promise<T> {
    const client = await pool.connect();
    try {
      await client.query('BEGIN');
      const result = await callback(client);
      await client.query('COMMIT');
      return result;
    } catch (error) {
      await client.query('ROLLBACK');
      throw error;
    } finally {
      client.release();
    }
  }
  ```
  - **Success Criteria**: Query utilities with logging and transactions

- [x] **1.3.4** Replace in-memory API key storage
  - Update file: `iora/mcp/src/lib/api-keys.ts`
  - Replace: `const apiKeys: Map<string, ApiKey> = new Map();`
  - With: PostgreSQL queries using `query()` function
  - **Key functions to update:**
    - `generateSecureApiKey()` - INSERT into api_keys table
    - `validateApiKey()` - SELECT with key_hash lookup
    - `revokeApiKey()` - UPDATE is_active = false
    - `getApiKeysForUser()` - SELECT with user_id filter
    - `getApiKeysForOrganization()` - SELECT with org_id filter
  - **Success Criteria**: All API key operations use PostgreSQL

- [x] **1.3.5** Implement API key hashing
  ```bash
  npm install bcryptjs @types/bcryptjs
  ```
  - Create file: `iora/mcp/src/lib/crypto.ts`
  ```typescript
  import bcrypt from 'bcryptjs';

  const SALT_ROUNDS = 10;

  export async function hashApiKey(key: string): Promise<string> {
    return bcrypt.hash(key, SALT_ROUNDS);
  }

  export async function verifyApiKey(key: string, hash: string): Promise<boolean> {
    return bcrypt.compare(key, hash);
  }

  export function generateApiKey(): string {
    const crypto = require('crypto');
    const randomBytes = crypto.randomBytes(32);
    return `iora_pk_${randomBytes.toString('hex')}`;
  }
  ```
  - **Success Criteria**: Secure key hashing implemented

- [x] **1.3.6** Create database models/repositories
  - Create file: `iora/mcp/src/db/repositories/user.repository.ts`
  - Create file: `iora/mcp/src/db/repositories/apiKey.repository.ts`
  - Create file: `iora/mcp/src/db/repositories/usage.repository.ts`
  - Implement repository pattern for clean separation
  - **Success Criteria**: Repository files created with CRUD operations

- [x] **1.3.7** Add database health check to server startup
  - Update file: `iora/mcp/src/index.ts`
  ```typescript
  import { checkDatabaseHealth } from './db/connection';

  async function startServer() {
    // Check database before starting
    const dbHealthy = await checkDatabaseHealth();
    if (!dbHealthy) {
      console.error('❌ Database connection failed. Exiting...');
      process.exit(1);
    }
    console.log('✅ Database connection established');

    // ... rest of server startup
  }
  ```
  - **Success Criteria**: Server fails fast if database unavailable

- [x] **1.3.8** Test API key persistence
  ```bash
  # Start server
  npm run dev

  # Create API key via API
  curl -X POST http://localhost:7145/user/api-keys \
    -H "Authorization: Bearer <clerk_token>" \
    -H "Content-Type: application/json" \
    -d '{"name": "Test Key", "permissions": ["tools:read"]}'

  # Restart server
  # Verify API key still exists
  curl http://localhost:7145/user/api-keys \
    -H "Authorization: Bearer <clerk_token>"
  ```
  - **Success Criteria**: API keys persist across server restarts

---

### Task 1.4: Rust Database Client Setup (Optional)

**Objective:** Connect Rust backend to PostgreSQL for advanced features.

#### Subtasks:

- [x] **1.4.1** Install SQLx for Rust
  ```bash
  cd /Volumes/MacExt/desktop-backup-sep-24/iora
  cargo add sqlx --features postgres,runtime-tokio-native-tls,migrate
  ```
  - **Success Criteria**: SQLx added to Cargo.toml

- [x] **1.4.2** Create database connection module
  - Create file: `src/modules/database.rs`
  ```rust
  use sqlx::{postgres::PgPoolOptions, PgPool};
  use std::time::Duration;

  pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
      let database_url = std::env::var("DATABASE_URL")
          .expect("DATABASE_URL must be set");

      PgPoolOptions::new()
          .max_connections(10)
          .min_connections(2)
          .acquire_timeout(Duration::from_secs(30))
          .connect(&database_url)
          .await
  }

  pub async fn check_health(pool: &PgPool) -> Result<(), sqlx::Error> {
      sqlx::query("SELECT 1")
          .execute(pool)
          .await?;
      Ok(())
  }
  ```
  - **Success Criteria**: Rust database module created

- [x] **1.4.3** Add database module to lib.rs
  ```rust
  pub mod database;
  ```
  - **Success Criteria**: Module exported

- [x] **1.4.4** Integrate with CLI for advanced analytics
  - Use PostgreSQL for usage analytics queries
  - Generate reports directly from Rust
  - **Success Criteria**: Rust can query PostgreSQL

---

### Task 1.5: Comprehensive Live Testing Framework

**Objective:** Implement automated testing for all Phase 1 database infrastructure.

**Priority:** CRITICAL
**Estimated Time:** 1-2 days
**Dependencies:** All Phase 1 tasks complete

#### Subtasks:

- [x] **1.5.1** Create database connectivity test suite
  - Test PostgreSQL container startup and connectivity
  - Verify database connection pool initialization
  - Test health check endpoints functionality
  - Validate environment variable configuration
  - **Success Criteria**: All connectivity tests pass

- [x] **1.5.2** Create migration testing framework
  - Test migration execution and rollback
  - Verify table creation and schema integrity
  - Test migration tracking and idempotency
  - Validate index creation and performance
  - **Success Criteria**: All migrations execute successfully

- [x] **1.5.3** Create API key persistence testing
  - Test API key creation and storage in PostgreSQL
  - Verify API key retrieval and validation
  - Test API key expiration and cleanup
  - Validate API key persistence across server restarts
  - **Success Criteria**: API keys persist correctly

- [x] **1.5.4** Create repository pattern testing
  - Test user repository CRUD operations
  - Test API key repository functionality
  - Test usage logging and analytics queries
  - Validate transaction handling and rollbacks
  - **Success Criteria**: All repository operations work correctly

- [x] **1.5.5** Create end-to-end database workflow tests
  - Test complete user registration → API key creation → usage logging flow
  - Verify data consistency across all tables
  - Test database performance under load
  - Validate error handling and recovery
  - **Success Criteria**: Complete workflows function correctly

- [x] **1.5.6** Create database health monitoring tests
  - Test `/health/database` endpoint functionality
  - Verify PostgreSQL and Redis health checks
  - Test graceful degradation when services unavailable
  - Validate monitoring data accuracy
  - **Success Criteria**: Health monitoring works correctly

- [x] **1.5.7** Create CLI database command testing
  - Test `iora analytics database` command
  - Verify database statistics display
  - Test error handling for missing database
  - Validate CLI integration with database layer
  - **Success Criteria**: CLI database commands work correctly

- [x] **1.5.8** Create performance and stress testing
  - Test database performance under concurrent load
  - Verify connection pool behavior under stress
  - Test query performance and optimization
  - Validate system stability under high load
  - **Success Criteria**: Performance meets requirements

---

## 🚀 Phase 2: Redis Caching Layer (Week 2)

**Goal:** Implement Redis for high-performance caching and rate limiting.

**Priority:** HIGH  
**Estimated Time:** 2-3 days  
**Dependencies:** Phase 1 complete

---

### Task 2.1: Redis Infrastructure Setup

**Objective:** Activate and configure Redis for caching.

#### Subtasks:

- [ ] **2.1.1** Start Redis container
  ```bash
  docker-compose --profile full up -d redis
  ```
  - Verify: `docker ps | grep redis`
  - Test: `docker exec -it iora-redis redis-cli ping` (should return "PONG")
  - **Success Criteria**: Redis container running

- [ ] **2.1.2** Install Redis clients
  ```bash
  # TypeScript
  cd iora/mcp
  npm install ioredis @types/ioredis

  # Rust (optional)
  cd /Volumes/MacExt/desktop-backup-sep-24/iora
  cargo add redis tokio
  ```
  - **Success Criteria**: Redis clients installed

- [ ] **2.1.3** Configure Redis connection
  - Add to `iora/mcp/.env`:
    ```bash
    REDIS_URL=redis://localhost:6379
    REDIS_PREFIX=iora:
    REDIS_TTL_DEFAULT=3600
    ```
  - **Success Criteria**: Redis configuration added

- [ ] **2.1.4** Create Redis connection module
  - Create file: `iora/mcp/src/cache/redis.ts`
  ```typescript
  import Redis from 'ioredis';

  const redis = new Redis(process.env.REDIS_URL || 'redis://localhost:6379', {
    retryStrategy: (times) => {
      const delay = Math.min(times * 50, 2000);
      return delay;
    },
    maxRetriesPerRequest: 3,
  });

  redis.on('connect', () => {
    console.log('✅ Redis connected');
  });

  redis.on('error', (error) => {
    console.error('❌ Redis error:', error);
  });

  export default redis;

  export async function checkRedisHealth(): Promise<boolean> {
    try {
      const pong = await redis.ping();
      return pong === 'PONG';
    } catch (error) {
      return false;
    }
  }
  ```
  - **Success Criteria**: Redis connection module created

---

### Task 2.2: Implement Caching Strategies

**Objective:** Cache expensive operations to reduce latency.

#### Subtasks:

- [ ] **2.2.1** Cache cryptocurrency price data
  - Create file: `iora/mcp/src/cache/price-cache.ts`
  ```typescript
  import redis from './redis';

  const PRICE_CACHE_TTL = 60; // 60 seconds

  export async function getCachedPrice(
    symbol: string,
    currency: string = 'USD'
  ): Promise<any | null> {
    const key = `price:${symbol}:${currency}`;
    const cached = await redis.get(key);
    return cached ? JSON.parse(cached) : null;
  }

  export async function setCachedPrice(
    symbol: string,
    currency: string,
    data: any
  ): Promise<void> {
    const key = `price:${symbol}:${currency}`;
    await redis.setex(key, PRICE_CACHE_TTL, JSON.stringify(data));
  }
  ```
  - Integrate with `iora/mcp/src/tools/get_price.ts`
  - **Success Criteria**: Price data cached for 60 seconds

- [ ] **2.2.2** Cache Clerk session data
  - Create file: `iora/mcp/src/cache/session-cache.ts`
  ```typescript
  import redis from './redis';

  const SESSION_CACHE_TTL = 3600; // 1 hour

  export async function getCachedSession(sessionId: string): Promise<any | null> {
    const key = `session:${sessionId}`;
    const cached = await redis.get(key);
    return cached ? JSON.parse(cached) : null;
  }

  export async function setCachedSession(
    sessionId: string,
    sessionData: any
  ): Promise<void> {
    const key = `session:${sessionId}`;
    await redis.setex(key, SESSION_CACHE_TTL, JSON.stringify(sessionData));
  }

  export async function invalidateSession(sessionId: string): Promise<void> {
    const key = `session:${sessionId}`;
    await redis.del(key);
  }
  ```
  - Update `iora/mcp/src/mw/clerk-auth.ts` to use cache
  - **Success Criteria**: Clerk API calls reduced by ~80%

- [ ] **2.2.3** Cache market analysis results
  - Cache AI analysis for 5 minutes
  - Key format: `analysis:${symbol}:${horizon}`
  - **Success Criteria**: Repeated analysis requests served from cache

- [ ] **2.2.4** Implement cache warming
  - Pre-cache popular symbols (BTC, ETH, SOL) on server startup
  - Background job to refresh cache every 30 seconds
  - **Success Criteria**: Popular queries always hit cache

---

### Task 2.3: Rate Limiting with Redis

**Objective:** Implement distributed rate limiting per user/org.

#### Subtasks:

- [ ] **2.3.1** Create rate limiting module
  - Create file: `iora/mcp/src/cache/rate-limiter.ts`
  ```typescript
  import redis from './redis';

  interface RateLimitConfig {
    windowSeconds: number;
    maxRequests: number;
  }

  const TIER_LIMITS: Record<string, RateLimitConfig> = {
    free: { windowSeconds: 60, maxRequests: 60 },
    pro: { windowSeconds: 60, maxRequests: 1000 },
    enterprise: { windowSeconds: 60, maxRequests: -1 }, // Unlimited
  };

  export async function checkRateLimit(
    identifier: string,  // userId or orgId
    tier: string = 'free'
  ): Promise<{ allowed: boolean; remaining: number; resetAt: Date }> {
    const config = TIER_LIMITS[tier];
    if (config.maxRequests === -1) {
      return { allowed: true, remaining: -1, resetAt: new Date() };
    }

    const key = `rate_limit:${identifier}`;
    const now = Date.now();
    const windowStart = now - (config.windowSeconds * 1000);

    // Use Redis sorted set for sliding window
    const multi = redis.multi();
    multi.zremrangebyscore(key, 0, windowStart);
    multi.zadd(key, now, `${now}`);
    multi.zcard(key);
    multi.expire(key, config.windowSeconds);

    const results = await multi.exec();
    const count = results![2][1] as number;

    const allowed = count <= config.maxRequests;
    const remaining = Math.max(0, config.maxRequests - count);
    const resetAt = new Date(now + (config.windowSeconds * 1000));

    return { allowed, remaining, resetAt };
  }
  ```
  - **Success Criteria**: Accurate sliding window rate limiting

- [ ] **2.3.2** Integrate rate limiting middleware
  - Update `iora/mcp/src/mw/security.ts`
  - Add rate limit headers to responses:
    - `X-RateLimit-Limit`
    - `X-RateLimit-Remaining`
    - `X-RateLimit-Reset`
  - Return 429 when limit exceeded
  - **Success Criteria**: Rate limiting enforced per tier

- [ ] **2.3.3** Add rate limit bypass for internal services
  - Check for HMAC signature first (service-to-service)
  - Skip rate limiting for service auth
  - **Success Criteria**: MCP agents not rate limited

---

### Task 2.4: Redis Monitoring & Optimization

**Objective:** Monitor Redis performance and optimize usage.

#### Subtasks:

- [ ] **2.4.1** Add Redis metrics endpoint
  - Create endpoint: `GET /metrics/redis`
  - Return:
    - Connected clients
    - Memory usage
    - Hit rate
    - Commands per second
  - **Success Criteria**: Redis metrics exposed

- [ ] **2.4.2** Implement cache invalidation strategy
  - Invalidate on data updates
  - TTL-based expiration for stale data
  - Manual flush for emergencies
  - **Success Criteria**: Cache consistency maintained

- [ ] **2.4.3** Set up Redis persistence
  - Enable RDB snapshots: Every 5 minutes if 100+ writes
  - Enable AOF: Append-only file for durability
  - Configure in `docker-compose.yml`:
    ```yaml
    command: redis-server --appendonly yes --save 300 100
    ```
  - **Success Criteria**: Redis data survives restarts

---

## 📊 Phase 3: Database Optimization (Week 3)

**Goal:** Optimize database performance for production scale.

**Priority:** MEDIUM  
**Estimated Time:** 3-4 days  
**Dependencies:** Phase 1 & 2 complete

---

### Task 3.1: Query Performance Optimization

#### Subtasks:

- [ ] **3.1.1** Enable PostgreSQL query logging
  ```sql
  ALTER SYSTEM SET log_statement = 'all';
  ALTER SYSTEM SET log_duration = on;
  ALTER SYSTEM SET log_min_duration_statement = 100; -- Log queries > 100ms
  SELECT pg_reload_conf();
  ```
  - **Success Criteria**: Slow queries logged

- [ ] **3.1.2** Analyze query performance
  ```sql
  -- Enable pg_stat_statements extension
  CREATE EXTENSION IF NOT EXISTS pg_stat_statements;

  -- View slow queries
  SELECT
    query,
    calls,
    total_exec_time,
    mean_exec_time,
    max_exec_time
  FROM pg_stat_statements
  ORDER BY mean_exec_time DESC
  LIMIT 20;
  ```
  - **Success Criteria**: Query bottlenecks identified

- [ ] **3.1.3** Add missing indexes
  - Review `EXPLAIN ANALYZE` output
  - Add indexes for common query patterns
  - **Success Criteria**: All queries use indexes

- [ ] **3.1.4** Implement connection pooling optimization
  - Monitor connection pool usage
  - Adjust pool size based on load
  - **Success Criteria**: No connection pool exhaustion

---

### Task 3.2: Data Archival & Partitioning

#### Subtasks:

- [ ] **3.2.1** Implement usage logs partitioning
  ```sql
  -- Partition by month
  CREATE TABLE usage_logs_2025_10 PARTITION OF usage_logs
    FOR VALUES FROM ('2025-10-01') TO ('2025-11-01');

  -- Create partitions for next 12 months
  -- Set up automatic partition creation
  ```
  - **Success Criteria**: Usage logs partitioned by month

- [ ] **3.2.2** Archive old data
  - Move data older than 6 months to archive table
  - Compress archived data
  - **Success Criteria**: Active tables remain performant

- [ ] **3.2.3** Implement audit log rotation
  - Keep last 90 days in hot storage
  - Archive older logs to S3/cold storage
  - **Success Criteria**: Audit logs manageable size

---

### Task 3.3: Database Backup & Recovery

#### Subtasks:

- [ ] **3.3.1** Set up automated backups
  ```bash
  # Daily backup script
  #!/bin/bash
  BACKUP_DIR="/backups/postgres"
  DATE=$(date +%Y%m%d_%H%M%S)
  
  docker exec iora-postgres pg_dump \
    -U iora_user \
    -d iora_dev \
    -F c \
    -f /tmp/backup_${DATE}.dump

  docker cp iora-postgres:/tmp/backup_${DATE}.dump \
    ${BACKUP_DIR}/backup_${DATE}.dump

  # Keep last 30 days
  find ${BACKUP_DIR} -name "backup_*.dump" -mtime +30 -delete
  ```
  - Schedule as cron job: `0 2 * * * /path/to/backup.sh`
  - **Success Criteria**: Daily backups automated

- [ ] **3.3.2** Test backup restoration
  ```bash
  # Restore from backup
  docker exec -i iora-postgres pg_restore \
    -U iora_user \
    -d iora_dev_test \
    -c \
    /tmp/backup_20251004.dump
  ```
  - **Success Criteria**: Successful restore tested

- [ ] **3.3.3** Document disaster recovery procedures
  - Create `DATABASE_RECOVERY.md`
  - Step-by-step restoration guide
  - RTO/RPO targets
  - **Success Criteria**: DR documentation complete

---

### Task 3.4: Database Monitoring & Alerting

#### Subtasks:

- [ ] **3.4.1** Set up database metrics collection
  - Monitor disk usage
  - Monitor connection count
  - Monitor query latency
  - **Success Criteria**: Metrics dashboard created

- [ ] **3.4.2** Configure alerts
  - Alert on disk > 80%
  - Alert on connection pool > 90%
  - Alert on slow queries > 5s
  - **Success Criteria**: Alert system configured

- [ ] **3.4.3** Create database health check endpoint
  - Endpoint: `GET /health/database`
  - Check PostgreSQL, Redis, Typesense
  - Return detailed status
  - **Success Criteria**: Health check endpoint live

---

## 🔐 Phase 4: Security & Compliance (Week 4)

**Goal:** Ensure database security and compliance readiness.

**Priority:** HIGH  
**Estimated Time:** 2-3 days  
**Dependencies:** Phase 1-3 complete

---

### Task 4.1: Database Security Hardening

#### Subtasks:

- [ ] **4.1.1** Implement row-level security (RLS)
  ```sql
  -- Enable RLS on api_keys table
  ALTER TABLE api_keys ENABLE ROW LEVEL SECURITY;

  -- Policy: Users can only see their own API keys
  CREATE POLICY api_keys_user_policy ON api_keys
    FOR ALL
    USING (user_id = current_setting('app.current_user_id')::text);
  ```
  - **Success Criteria**: RLS policies enforce data isolation

- [ ] **4.1.2** Encrypt sensitive data at rest
  - Enable PostgreSQL encryption
  - Encrypt API key hashes with additional layer
  - **Success Criteria**: Data encrypted at rest

- [ ] **4.1.3** Implement database user segregation
  ```sql
  -- Create read-only user for analytics
  CREATE USER iora_readonly WITH PASSWORD 'readonly_password';
  GRANT CONNECT ON DATABASE iora_dev TO iora_readonly;
  GRANT SELECT ON ALL TABLES IN SCHEMA public TO iora_readonly;
  
  -- Create write user for API
  CREATE USER iora_api WITH PASSWORD 'api_password';
  GRANT INSERT, UPDATE, DELETE ON api_keys, usage_logs TO iora_api;
  ```
  - **Success Criteria**: Least privilege access implemented

- [ ] **4.1.4** Enable SSL/TLS for database connections
  - Generate SSL certificates
  - Configure PostgreSQL to require SSL
  - Update connection strings: `sslmode=require`
  - **Success Criteria**: All connections encrypted

---

### Task 4.2: Audit & Compliance

#### Subtasks:

- [ ] **4.2.1** Implement comprehensive audit logging
  - Log all data access
  - Log all data modifications
  - Include IP address, user agent, timestamp
  - **Success Criteria**: Full audit trail available

- [ ] **4.2.2** Set up GDPR compliance measures
  - User data deletion endpoint
  - Data export endpoint
  - Consent tracking
  - **Success Criteria**: GDPR-compliant data handling

- [ ] **4.2.3** Implement data retention policies
  - Auto-delete usage logs after 2 years
  - Auto-delete expired API keys after 90 days
  - Document retention policy
  - **Success Criteria**: Automated data cleanup

- [ ] **4.2.4** Create security audit checklist
  - Regular password rotation
  - Access review quarterly
  - Backup verification monthly
  - **Success Criteria**: Security checklist documented

---

## 📈 Phase 5: Production Readiness (Week 5)

**Goal:** Final preparation for production deployment.

**Priority:** HIGH  
**Estimated Time:** 2-3 days  
**Dependencies:** All previous phases complete

---

### Task 5.1: Performance Testing

#### Subtasks:

- [ ] **5.1.1** Load test database queries
  ```bash
  # Use pgbench for PostgreSQL
  docker exec -it iora-postgres pgbench \
    -i \
    -s 50 \
    -U iora_user \
    iora_dev

  docker exec -it iora-postgres pgbench \
    -c 10 \
    -j 2 \
    -t 10000 \
    -U iora_user \
    iora_dev
  ```
  - Target: >1000 TPS
  - **Success Criteria**: Performance benchmarks met

- [ ] **5.1.2** Load test Redis cache
  - Use redis-benchmark
  - Target: >10,000 ops/sec
  - **Success Criteria**: Redis performance validated

- [ ] **5.1.3** Test database failover
  - Simulate database crash
  - Verify automatic reconnection
  - **Success Criteria**: Graceful failover handling

---

### Task 5.2: Documentation

#### Subtasks:

- [ ] **5.2.1** Create database documentation
  - ER diagram
  - Table descriptions
  - Index strategy
  - **Success Criteria**: `DATABASE_SCHEMA.md` created

- [ ] **5.2.2** Document migration procedures
  - How to add new tables
  - How to modify existing tables
  - Rollback procedures
  - **Success Criteria**: `DATABASE_MIGRATIONS.md` created

- [ ] **5.2.3** Create operational runbooks
  - Common issues and solutions
  - Emergency procedures
  - Contact information
  - **Success Criteria**: Operations documentation complete

---

### Task 5.3: Production Deployment

#### Subtasks:

- [ ] **5.3.1** Update production environment variables
  - Production database URL
  - Production Redis URL
  - SSL certificates
  - **Success Criteria**: Production config ready

- [ ] **5.3.2** Run migrations on production
  - Backup production database first
  - Run migrations with zero downtime
  - Verify data integrity
  - **Success Criteria**: Production database migrated

- [ ] **5.3.3** Enable monitoring in production
  - Set up alerts
  - Configure dashboards
  - Test alert notifications
  - **Success Criteria**: Production monitoring active

- [ ] **5.3.4** Conduct post-deployment verification
  - API key persistence test
  - Rate limiting test
  - Cache performance test
  - **Success Criteria**: All systems operational

---

## 🎯 Success Metrics

### Critical Metrics
- [ ] **Data Persistence**: API keys survive server restart (MUST PASS)
- [ ] **Query Performance**: <100ms for API key lookups (MUST PASS)
- [ ] **Cache Hit Rate**: >80% for price queries (TARGET)
- [ ] **Rate Limiting**: Accurate per-tier enforcement (MUST PASS)
- [ ] **Backup Success**: 100% successful daily backups (MUST PASS)

### Performance Benchmarks
- [ ] **Database**: >1000 transactions/second
- [ ] **Redis**: >10,000 operations/second
- [ ] **API Response**: <200ms p95 latency
- [ ] **Cache Hit Rate**: >80%

### Operational Metrics
- [ ] **Uptime**: >99.9% database availability
- [ ] **Backup Success Rate**: 100%
- [ ] **Restore Time**: <30 minutes RTO
- [ ] **Data Loss**: <5 minutes RPO

---

## 📚 References

### Documentation
- PostgreSQL: https://www.postgresql.org/docs/
- Redis: https://redis.io/documentation
- SQLx (Rust): https://github.com/launchbadge/sqlx
- node-postgres: https://node-postgres.com/
- ioredis: https://github.com/luin/ioredis

### Best Practices
- [PostgreSQL Performance Tuning](https://wiki.postgresql.org/wiki/Performance_Optimization)
- [Redis Best Practices](https://redis.io/docs/manual/patterns/)
- [Database Security Checklist](https://owasp.org/www-project-database-security/)

---

## 🚨 Risk Mitigation

### High-Risk Items
1. **Data Loss During Migration**
   - Mitigation: Full backup before any migration
   - Rollback plan documented
   - Test migrations on staging first

2. **Connection Pool Exhaustion**
   - Mitigation: Monitor pool usage
   - Implement connection limits
   - Graceful degradation

3. **Cache Stampede**
   - Mitigation: Implement cache warming
   - Use probabilistic early expiration
   - Implement request coalescing

---

## ✅ Final Checklist

Before marking complete:

- [ ] All Phase 1 tasks completed (PostgreSQL setup)
- [ ] All Phase 2 tasks completed (Redis caching)
- [ ] All Phase 3 tasks completed (Optimization)
- [ ] All Phase 4 tasks completed (Security)
- [ ] All Phase 5 tasks completed (Production)
- [ ] API keys persist across restarts (CRITICAL)
- [ ] Rate limiting works correctly
- [ ] Caching improves performance
- [ ] Backups automated and tested
- [ ] Monitoring and alerts active
- [ ] Documentation complete
- [ ] Team trained on operations

---

**Document Status:** READY FOR IMPLEMENTATION  
**Next Action:** Begin Phase 1, Task 1.1  
**Estimated Total Time:** 3-5 weeks  
**Required Resources:** Docker, PostgreSQL, Redis, Node.js, Rust

