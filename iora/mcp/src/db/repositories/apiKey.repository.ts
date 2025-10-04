/**
 * API Key Repository
 *
 * Handles all database operations related to API keys.
 * Critical for replacing in-memory storage with persistent database storage.
 */

import { query, transaction } from '../queries';

/**
 * API Key data interface
 */
export interface ApiKey {
  id: string;
  key_hash: string;
  key_prefix: string;
  user_id: string;
  org_id?: string;
  name: string;
  permissions: string[];
  created_at: Date;
  last_used_at?: Date;
  expires_at?: Date;
  is_active: boolean;
  rate_limit_tier: 'free' | 'pro' | 'enterprise';
  usage_count: number;
}

/**
 * Create a new API key
 */
export async function createApiKey(apiKey: Omit<ApiKey, 'id' | 'created_at' | 'usage_count'>): Promise<ApiKey> {
  const result = await query<ApiKey>(
    `INSERT INTO api_keys (key_hash, key_prefix, user_id, org_id, name, permissions, expires_at, rate_limit_tier)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
     RETURNING *`,
    [
      apiKey.key_hash,
      apiKey.key_prefix,
      apiKey.user_id,
      apiKey.org_id,
      apiKey.name,
      JSON.stringify(apiKey.permissions),
      apiKey.expires_at,
      apiKey.rate_limit_tier
    ]
  );

  return result.rows[0];
}

/**
 * Get API key by hash (for authentication)
 */
export async function getApiKeyByHash(keyHash: string): Promise<ApiKey | null> {
  const result = await query<ApiKey>(
    `SELECT * FROM api_keys WHERE key_hash = $1 AND is_active = true
     AND (expires_at IS NULL OR expires_at > NOW())`,
    [keyHash]
  );

  if (result.rows[0]) {
    // Update last used timestamp
    await updateApiKeyLastUsed(result.rows[0].id);
  }

  return result.rows[0] || null;
}

/**
 * Get API keys for a user
 */
export async function getApiKeysForUser(userId: string): Promise<ApiKey[]> {
  const result = await query<ApiKey>(
    `SELECT * FROM api_keys WHERE user_id = $1 AND is_active = true
     ORDER BY created_at DESC`,
    [userId]
  );

  return result.rows.map(row => ({
    ...row,
    permissions: Array.isArray(row.permissions) ? row.permissions : JSON.parse(row.permissions || '[]')
  }));
}

/**
 * Get API keys for an organization
 */
export async function getApiKeysForOrganization(orgId: string): Promise<ApiKey[]> {
  const result = await query<ApiKey>(
    `SELECT * FROM api_keys WHERE org_id = $1 AND is_active = true
     ORDER BY created_at DESC`,
    [orgId]
  );

  return result.rows.map(row => ({
    ...row,
    permissions: Array.isArray(row.permissions) ? row.permissions : JSON.parse(row.permissions || '[]')
  }));
}

/**
 * Update API key last used timestamp
 */
export async function updateApiKeyLastUsed(id: string): Promise<void> {
  await query(
    `UPDATE api_keys SET last_used_at = NOW(), usage_count = usage_count + 1 WHERE id = $1`,
    [id]
  );
}

/**
 * Update API key usage count
 */
export async function incrementApiKeyUsage(id: string): Promise<void> {
  await query(
    `UPDATE api_keys SET usage_count = usage_count + 1 WHERE id = $1`,
    [id]
  );
}

/**
 * Revoke API key (soft delete)
 */
export async function revokeApiKey(id: string): Promise<boolean> {
  const result = await query(
    `UPDATE api_keys SET is_active = false WHERE id = $1`,
    [id]
  );

  return result.rowCount > 0;
}

/**
 * Delete API key permanently (hard delete)
 */
export async function deleteApiKey(id: string): Promise<boolean> {
  const result = await query(
    `DELETE FROM api_keys WHERE id = $1`,
    [id]
  );

  return result.rowCount > 0;
}

/**
 * Update API key permissions
 */
export async function updateApiKeyPermissions(id: string, permissions: string[]): Promise<ApiKey | null> {
  const result = await query<ApiKey>(
    `UPDATE api_keys SET permissions = $1 WHERE id = $2 AND is_active = true RETURNING *`,
    [JSON.stringify(permissions), id]
  );

  return result.rows[0] || null;
}

/**
 * Update API key name
 */
export async function updateApiKeyName(id: string, name: string): Promise<ApiKey | null> {
  const result = await query<ApiKey>(
    `UPDATE api_keys SET name = $1 WHERE id = $2 AND is_active = true RETURNING *`,
    [name, id]
  );

  return result.rows[0] || null;
}

/**
 * Get API key by ID
 */
export async function getApiKeyById(id: string): Promise<ApiKey | null> {
  const result = await query<ApiKey>(
    `SELECT * FROM api_keys WHERE id = $1`,
    [id]
  );

  return result.rows[0] || null;
}

/**
 * Get expired API keys
 */
export async function getExpiredApiKeys(): Promise<ApiKey[]> {
  const result = await query<ApiKey>(
    `SELECT * FROM api_keys WHERE expires_at IS NOT NULL
     AND expires_at < NOW() AND is_active = true`
  );

  return result.rows.map(row => ({
    ...row,
    permissions: Array.isArray(row.permissions) ? row.permissions : JSON.parse(row.permissions || '[]')
  }));
}

/**
 * Clean up expired API keys (set to inactive)
 */
export async function cleanupExpiredApiKeys(): Promise<number> {
  const result = await query(
    `UPDATE api_keys SET is_active = false WHERE expires_at IS NOT NULL
     AND expires_at < NOW() AND is_active = true`
  );

  return result.rowCount;
}

/**
 * Get API key usage statistics
 */
export async function getApiKeyStats(userId?: string, orgId?: string): Promise<{
  total: number;
  active: number;
  expired: number;
  byTier: Record<string, number>;
}> {
  let whereClause = '';
  const params: any[] = [];

  if (userId) {
    whereClause = 'WHERE user_id = $1';
    params.push(userId);
  } else if (orgId) {
    whereClause = 'WHERE org_id = $1';
    params.push(orgId);
  }

  // Get total count
  const totalResult = await query(
    `SELECT COUNT(*) as count FROM api_keys ${whereClause}`,
    params
  );

  // Get active count
  const activeResult = await query(
    `SELECT COUNT(*) as count FROM api_keys ${whereClause} AND is_active = true`,
    params
  );

  // Get expired count
  const expiredResult = await query(
    `SELECT COUNT(*) as count FROM api_keys ${whereClause}
     AND expires_at IS NOT NULL AND expires_at < NOW()`,
    params
  );

  // Get count by tier
  const tierResult = await query(
    `SELECT rate_limit_tier, COUNT(*) as count FROM api_keys ${whereClause}
     GROUP BY rate_limit_tier`,
    params
  );

  const byTier: Record<string, number> = {};
  tierResult.rows.forEach(row => {
    byTier[row.rate_limit_tier] = parseInt(row.count);
  });

  return {
    total: parseInt(totalResult.rows[0].count),
    active: parseInt(activeResult.rows[0].count),
    expired: parseInt(expiredResult.rows[0].count),
    byTier,
  };
}
