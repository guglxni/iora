/**
 * Database Query Utilities
 *
 * Provides type-safe database queries with proper error handling,
 * logging, and transaction support for the IORA MCP server.
 */

import { pool } from '../config/database';
import { QueryResult } from 'pg';

/**
 * Execute a database query with logging and error handling
 */
export async function query<T = any>(
  text: string,
  params?: any[]
): Promise<QueryResult<T>> {
  const start = Date.now();
  try {
    const result = await pool.query<T>(text, params);
    const duration = Date.now() - start;

    console.log('Query executed', {
      text,
      duration: `${duration}ms`,
      rows: result.rowCount,
      timestamp: new Date().toISOString()
    });

    return result;
  } catch (error) {
    const duration = Date.now() - start;
    console.error('Query error', {
      text,
      error: error instanceof Error ? error.message : 'Unknown error',
      duration: `${duration}ms`,
      timestamp: new Date().toISOString()
    });
    throw error;
  }
}

/**
 * Execute multiple queries in a transaction
 */
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

/**
 * Check if a table exists in the database
 */
export async function tableExists(tableName: string): Promise<boolean> {
  try {
    const result = await query(
      `SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = $1)`,
      [tableName]
    );
    return result.rows[0].exists;
  } catch (error) {
    console.error(`Error checking if table ${tableName} exists:`, error);
    return false;
  }
}

/**
 * Get database statistics
 */
export async function getDatabaseStats(): Promise<{
  version: string;
  totalConnections: number;
  idleConnections: number;
  waitingClients: number;
}> {
  try {
    // Get PostgreSQL version
    const versionResult = await query('SELECT version() as version');
    const version = versionResult.rows[0].version;

    // Get connection pool stats
    const poolStats = {
      totalConnections: (pool as any).totalCount || 0,
      idleConnections: (pool as any).idleCount || 0,
      waitingClients: (pool as any).waitingCount || 0,
    };

    return {
      version,
      ...poolStats,
    };
  } catch (error) {
    console.error('Error getting database stats:', error);
    return {
      version: 'unknown',
      totalConnections: 0,
      idleConnections: 0,
      waitingClients: 0,
    };
  }
}

/**
 * Health check query for database connectivity
 */
export async function healthCheckQuery(): Promise<boolean> {
  try {
    await query('SELECT 1');
    return true;
  } catch (error) {
    console.error('Database health check failed:', error);
    return false;
  }
}
