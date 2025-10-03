# IORA Authentication Guide

This document explains the multi-tenant authentication system implemented in IORA using Clerk.

## Table of Contents
- [Overview](#overview)
- [Authentication Methods](#authentication-methods)
- [Setup Guide](#setup-guide)
- [API Reference](#api-reference)
- [Code Examples](#code-examples)
- [Security Best Practices](#security-best-practices)

## Overview

IORA uses a **dual authentication system**:

1. **Service-Level Auth (HMAC-SHA256)**: For MCP agent-to-server communication
2. **User-Level Auth (Clerk)**: For end-user dashboard, API key management, and organization features

This separation allows:
- **Secure service-to-service** communication (AI agents, webhooks)
- **Multi-tenant user management** (teams, organizations, roles)
- **API key generation** for programmatic access
- **Billing integration** via user/org metadata

## Authentication Methods

### 1. HMAC Authentication (Service Auth)

**Used for**: MCP tool endpoints (`/tools/*`)

**How it works**:
- Server shares a secret key with clients
- Client signs requests with HMAC-SHA256
- Server verifies signature

**Endpoints**:
- `/tools/get_price`
- `/tools/analyze_market`
- `/tools/feed_oracle`
- `/tools/health`

**Example**:
```typescript
import crypto from 'crypto';

const secret = process.env.CORAL_SHARED_SECRET;
const body = { symbol: 'BTC' };
const signature = crypto
  .createHmac('sha256', secret)
  .update(JSON.stringify(body))
  .digest('hex');

await fetch('http://localhost:7070/tools/get_price', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'X-IORA-Signature': signature
  },
  body: JSON.stringify(body)
});
```

### 2. Clerk Session Authentication (User Auth)

**Used for**: User dashboard endpoints (`/user/*`)

**How it works**:
- Users sign in via Clerk
- Clerk issues session tokens
- Tokens sent in Authorization header

**Endpoints**:
- `/user/profile` - Get user details
- `/user/organizations` - List organizations
- `/user/api-keys` - Manage API keys
- `/user/usage` - View usage statistics

**Example**:
```typescript
// Get session token from Clerk
const sessionToken = await clerk.session.getToken();

await fetch('http://localhost:7070/user/profile', {
  headers: {
    'Authorization': `Bearer ${sessionToken}`
  }
});
```

### 3. API Key Authentication (Programmatic Access)

**Used for**: Programmatic access to tools (alternative to session auth)

**How it works**:
- Users generate API keys via `/user/api-keys`
- API keys used in Authorization header
- Keys are scoped to user/organization

**Format**: `iora_pk_<24_random_chars>`

**Example**:
```bash
curl -X POST http://localhost:7070/tools/get_price \
  -H "Authorization: Bearer iora_pk_abc123..." \
  -H "Content-Type: application/json" \
  -d '{"symbol": "BTC"}'
```

## Setup Guide

### Step 1: Create Clerk Account

1. Go to [clerk.com](https://clerk.com)
2. Sign up (free tier: 10,000 MAUs)
3. Create a new application
4. Note your API keys

### Step 2: Configure Environment Variables

Add to `iora/mcp/.env`:

```bash
CLERK_PUBLISHABLE_KEY=pk_test_...
CLERK_SECRET_KEY=sk_test_...
CLERK_WEBHOOK_SECRET=whsec_...
```

### Step 3: Enable Organizations (Optional)

In Clerk Dashboard:
1. Go to **Settings** → **Organizations**
2. Enable organization feature
3. Configure roles: Admin, Editor, Viewer

### Step 4: Test Authentication

```bash
# Start server
cd iora/mcp
npm run dev

# Test user endpoint (requires valid session)
curl http://localhost:7070/user/profile \
  -H "Authorization: Bearer <session_token>"

# Test service endpoint (requires HMAC)
curl -X POST http://localhost:7070/tools/health \
  -H "X-IORA-Signature: <hmac_signature>" \
  -H "Content-Type: application/json" \
  -d '{}'
```

## API Reference

### User Endpoints

#### `GET /user/profile`
Get current user profile.

**Auth**: Clerk session required

**Response**:
```json
{
  "ok": true,
  "data": {
    "id": "user_abc123",
    "email": "user@example.com",
    "firstName": "John",
    "lastName": "Doe",
    "tier": "pro",
    "createdAt": "2025-01-01T00:00:00.000Z"
  }
}
```

#### `GET /user/organizations`
List user's organizations.

**Auth**: Clerk session required

**Response**:
```json
{
  "ok": true,
  "data": [
    {
      "id": "org_abc123",
      "name": "Acme Corp",
      "slug": "acme-corp",
      "role": "admin",
      "tier": "enterprise"
    }
  ]
}
```

#### `POST /user/api-keys`
Create a new API key.

**Auth**: Clerk session required

**Request Body**:
```json
{
  "name": "Production Key",
  "permissions": ["tools:read", "tools:write"],
  "expiresInDays": 90
}
```

**Response**:
```json
{
  "ok": true,
  "data": {
    "id": "key_abc123",
    "key": "iora_pk_xyz789...",
    "keyPrefix": "iora_pk_xyz789...",
    "message": "Save this key securely. It will not be shown again."
  }
}
```

#### `DELETE /user/api-keys/:keyId`
Revoke an API key.

**Auth**: Clerk session required

**Response**:
```json
{
  "ok": true,
  "message": "API key revoked successfully"
}
```

#### `GET /user/usage`
Get usage statistics.

**Auth**: Clerk session required

**Response**:
```json
{
  "ok": true,
  "data": {
    "tier": "pro",
    "limits": {
      "requestsPerMinute": 1000,
      "requestsPerMonth": 100000
    },
    "usage": {
      "requestsThisMonth": 5234,
      "requestsToday": 142
    },
    "remaining": {
      "requestsThisMonth": 94766
    }
  }
}
```

## Code Examples

### React Component with Clerk

```typescript
import { useUser, useOrganization } from '@clerk/nextjs';
import { useState } from 'react';

export function ApiKeyManager() {
  const { user } = useUser();
  const { organization } = useOrganization();
  const [apiKeys, setApiKeys] = useState([]);

  async function createKey() {
    const session = await clerk.session.getToken();
    
    const res = await fetch('http://localhost:7070/user/api-keys', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${session}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        name: 'New Key',
        permissions: ['tools:read', 'tools:write']
      })
    });

    const data = await res.json();
    alert(`API Key: ${data.data.key}`);
  }

  return (
    <div>
      <h2>API Keys for {organization?.name || user?.fullName}</h2>
      <button onClick={createKey}>Create New Key</button>
    </div>
  );
}
```

### Python Client with API Key

```python
import requests
import os

API_KEY = os.getenv('IORA_API_KEY')
BASE_URL = 'http://localhost:7070'

def get_crypto_price(symbol):
    response = requests.post(
        f'{BASE_URL}/tools/get_price',
        headers={
            'Authorization': f'Bearer {API_KEY}',
            'Content-Type': 'application/json'
        },
        json={'symbol': symbol}
    )
    return response.json()

price_data = get_crypto_price('BTC')
print(f"BTC Price: ${price_data['data']['price']}")
```

## Security Best Practices

### 1. API Key Storage
- **Never commit** API keys to version control
- Store in environment variables
- Use secrets management (e.g., AWS Secrets Manager, Vault)

### 2. Key Rotation
- Rotate keys every 90 days
- Revoke compromised keys immediately
- Support multiple active keys for zero-downtime rotation

### 3. Rate Limiting
- Enforce tier-based rate limits
- Monitor for abuse patterns
- Implement exponential backoff

### 4. HMAC Secret Management
- Generate strong secrets (32+ characters)
- Rotate HMAC secrets periodically
- Use different secrets for dev/staging/prod

### 5. Session Management
- Set appropriate session timeouts (Clerk default: 7 days)
- Implement refresh token rotation
- Enforce re-authentication for sensitive operations

### 6. Organization Isolation
- Ensure data isolation between orgs
- Validate orgId in all queries
- Audit cross-org access attempts

## Troubleshooting

### "Unauthorized - No session token provided"
- Check that Authorization header is set
- Verify session token is valid (not expired)
- Ensure Clerk SDK is properly configured

### "Invalid or expired API key"
- Verify API key format: `iora_pk_...`
- Check key hasn't been revoked
- Confirm key hasn't expired

### "HMAC signature verification failed"
- Ensure CORAL_SHARED_SECRET matches server
- Verify request body matches signature input
- Check for trailing whitespace/newlines

## Migration from HMAC-Only

If you're upgrading from HMAC-only auth:

1. Existing HMAC endpoints (`/tools/*`) **continue to work**
2. New user endpoints (`/user/*`) use Clerk
3. No breaking changes for existing clients
4. Gradually migrate to API keys for programmatic access

## Next Steps

- [Billing Integration](./BILLING.md) - Add Stripe subscriptions
- [Admin Dashboard](./ADMIN_DASHBOARD.md) - Build operator UI
- [Rate Limiting](./RATE_LIMITING.md) - Tier-based quotas

---

**Last Updated**: October 2, 2025  
**Version**: 1.0.0

