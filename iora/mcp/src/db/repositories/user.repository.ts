/**
 * User Repository
 *
 * Handles all database operations related to users and their data.
 * Provides type-safe CRUD operations with proper error handling.
 */

import { query, transaction } from '../queries';

/**
 * User data interface
 */
export interface User {
  id: string;
  email: string;
  first_name?: string;
  last_name?: string;
  tier: 'free' | 'pro' | 'enterprise';
  stripe_customer_id?: string;
  created_at: Date;
  updated_at: Date;
  last_login_at?: Date;
  is_active: boolean;
}

/**
 * Create a new user record
 */
export async function createUser(user: Omit<User, 'created_at' | 'updated_at'>): Promise<User> {
  const result = await query<User>(
    `INSERT INTO users (id, email, first_name, last_name, tier, stripe_customer_id, last_login_at, is_active)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
     RETURNING *`,
    [
      user.id,
      user.email,
      user.first_name,
      user.last_name,
      user.tier,
      user.stripe_customer_id,
      user.last_login_at,
      user.is_active
    ]
  );

  return result.rows[0];
}

/**
 * Get user by ID
 */
export async function getUserById(id: string): Promise<User | null> {
  const result = await query<User>(
    `SELECT * FROM users WHERE id = $1 AND is_active = true`,
    [id]
  );

  return result.rows[0] || null;
}

/**
 * Get user by email
 */
export async function getUserByEmail(email: string): Promise<User | null> {
  const result = await query<User>(
    `SELECT * FROM users WHERE email = $1 AND is_active = true`,
    [email]
  );

  return result.rows[0] || null;
}

/**
 * Update user information
 */
export async function updateUser(id: string, updates: Partial<User>): Promise<User | null> {
  const fields = Object.keys(updates).filter(key => key !== 'id' && key !== 'created_at');
  const values = fields.map(key => updates[key as keyof User]);
  const setClause = fields.map((field, index) => `${field} = $${index + 2}`).join(', ');

  if (fields.length === 0) {
    return getUserById(id);
  }

  const result = await query<User>(
    `UPDATE users SET ${setClause}, updated_at = NOW() WHERE id = $1 AND is_active = true RETURNING *`,
    [id, ...values]
  );

  return result.rows[0] || null;
}

/**
 * Update user last login time
 */
export async function updateUserLastLogin(id: string): Promise<void> {
  await query(
    `UPDATE users SET last_login_at = NOW() WHERE id = $1`,
    [id]
  );
}

/**
 * Update user tier (for subscription changes)
 */
export async function updateUserTier(id: string, tier: 'free' | 'pro' | 'enterprise'): Promise<User | null> {
  return updateUser(id, { tier });
}

/**
 * Soft delete user (mark as inactive)
 */
export async function deactivateUser(id: string): Promise<boolean> {
  const result = await query(
    `UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1`,
    [id]
  );

  return result.rowCount > 0;
}

/**
 * Get users by tier
 */
export async function getUsersByTier(tier: 'free' | 'pro' | 'enterprise'): Promise<User[]> {
  const result = await query<User>(
    `SELECT * FROM users WHERE tier = $1 AND is_active = true ORDER BY created_at DESC`,
    [tier]
  );

  return result.rows;
}

/**
 * Get all active users (for analytics)
 */
export async function getAllActiveUsers(): Promise<User[]> {
  const result = await query<User>(
    `SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC`
  );

  return result.rows;
}

/**
 * Search users by email (for admin purposes)
 */
export async function searchUsers(emailPattern: string): Promise<User[]> {
  const result = await query<User>(
    `SELECT * FROM users WHERE email ILIKE $1 AND is_active = true ORDER BY email`,
    [`%${emailPattern}%`]
  );

  return result.rows;
}
