/**
 * API Key Management System
 * 
 * Provides secure API key generation, validation, and storage for users/organizations.
 * API keys serve as an alternative to session-based authentication for programmatic access.
 */

import crypto from 'crypto';
import { Request, Response, NextFunction } from 'express';
import { clerkClient } from '@clerk/clerk-sdk-node';

export interface ApiKey {
  id: string;
  key: string; // hashed
  keyPrefix: string; // first 8 chars for display (e.g., "iora_pk_...")
  userId: string;
  orgId?: string;
  name: string;
  createdAt: Date;
  lastUsedAt?: Date;
  expiresAt?: Date;
  permissions: string[]; // e.g., ['tools:read', 'tools:write', 'admin:read']
}

// In-memory storage (replace with database in production)
const apiKeys: Map<string, ApiKey> = new Map();

/**
 * Generate a secure API key
 * Format: iora_pk_<32_random_chars>
 */
export function generateApiKey(): { key: string; hash: string; prefix: string } {
  const randomBytes = crypto.randomBytes(24).toString('base64url');
  const key = `iora_pk_${randomBytes}`;
  const hash = hashApiKey(key);
  const prefix = key.substring(0, 15) + '...'; // iora_pk_XXXXXXXX...

  return { key, hash, prefix };
}

/**
 * Hash API key for secure storage
 */
function hashApiKey(key: string): string {
  return crypto.createHash('sha256').update(key).digest('hex');
}

/**
 * Create a new API key for a user/org
 */
export async function createApiKey(params: {
  userId: string;
  orgId?: string;
  name: string;
  permissions?: string[];
  expiresInDays?: number;
}): Promise<{ id: string; key: string; prefix: string }> {
  const { key, hash, prefix } = generateApiKey();
  const id = crypto.randomUUID();

  const apiKey: ApiKey = {
    id,
    key: hash,
    keyPrefix: prefix,
    userId: params.userId,
    orgId: params.orgId,
    name: params.name,
    createdAt: new Date(),
    permissions: params.permissions || ['tools:read', 'tools:write'],
    expiresAt: params.expiresInDays
      ? new Date(Date.now() + params.expiresInDays * 24 * 60 * 60 * 1000)
      : undefined
  };

  apiKeys.set(id, apiKey);

  // In production, store in database
  // await db.apiKeys.create(apiKey);

  return { id, key, prefix }; // Return unhashed key only once
}

/**
 * Validate an API key and return associated user/org context
 */
export async function validateApiKey(key: string): Promise<{
  valid: boolean;
  userId?: string;
  orgId?: string;
  permissions?: string[];
}> {
  const hash = hashApiKey(key);

  // Find API key by hash
  for (const apiKey of apiKeys.values()) {
    if (apiKey.key === hash) {
      // Check expiration
      if (apiKey.expiresAt && apiKey.expiresAt < new Date()) {
        return { valid: false };
      }

      // Update last used timestamp
      apiKey.lastUsedAt = new Date();

      return {
        valid: true,
        userId: apiKey.userId,
        orgId: apiKey.orgId,
        permissions: apiKey.permissions
      };
    }
  }

  return { valid: false };
}

/**
 * List API keys for a user/org (returns redacted keys)
 */
export async function listApiKeys(userId: string, orgId?: string): Promise<Omit<ApiKey, 'key'>[]> {
  const keys: Omit<ApiKey, 'key'>[] = [];

  for (const apiKey of apiKeys.values()) {
    if (apiKey.userId === userId || (orgId && apiKey.orgId === orgId)) {
      const { key, ...safeKey } = apiKey;
      keys.push(safeKey);
    }
  }

  return keys;
}

/**
 * Revoke an API key
 */
export async function revokeApiKey(keyId: string, userId: string): Promise<boolean> {
  const apiKey = apiKeys.get(keyId);

  if (!apiKey || apiKey.userId !== userId) {
    return false;
  }

  apiKeys.delete(keyId);
  return true;
}

/**
 * API Key authentication middleware
 * Alternative to Clerk session auth for programmatic access
 */
export async function apiKeyAuth(req: Request, res: Response, next: NextFunction) {
  try {
    // Extract API key from Authorization header
    const authHeader = req.headers.authorization;
    
    if (!authHeader || !authHeader.startsWith('Bearer iora_pk_')) {
      return res.status(401).json({
        ok: false,
        error: 'Unauthorized',
        message: 'Valid API key required. Format: Authorization: Bearer iora_pk_...'
      });
    }

    const apiKey = authHeader.replace('Bearer ', '');
    const validation = await validateApiKey(apiKey);

    if (!validation.valid) {
      return res.status(401).json({
        ok: false,
        error: 'Unauthorized',
        message: 'Invalid or expired API key'
      });
    }

    // Populate auth context (similar to Clerk)
    req.auth = {
      userId: validation.userId!,
      orgId: validation.orgId,
      role: 'api_key' // Distinguish from session-based auth
    };

    next();
  } catch (error: any) {
    console.error('API key auth error:', error);
    return res.status(401).json({
      ok: false,
      error: 'Authentication failed',
      message: error.message || 'Unable to validate API key'
    });
  }
}

/**
 * Combined auth middleware - supports both Clerk sessions and API keys
 */
export async function flexibleAuth(req: Request, res: Response, next: NextFunction) {
  const authHeader = req.headers.authorization;

  // Check if it's an API key
  if (authHeader?.startsWith('Bearer iora_pk_')) {
    return apiKeyAuth(req, res, next);
  }

  // Otherwise, assume it's a Clerk session (handled elsewhere)
  // or return unauthorized
  return res.status(401).json({
    ok: false,
    error: 'Unauthorized',
    message: 'Valid authentication required (Clerk session or API key)'
  });
}

