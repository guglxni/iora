/**
 * Database Configuration Module
 *
 * Manages PostgreSQL connection pooling and configuration for IORA MCP server.
 * Provides centralized database connection management with proper error handling.
 */

import { Pool, PoolConfig } from 'pg';

/**
 * Database connection pool configuration
 */
const poolConfig: PoolConfig = {
  connectionString: process.env.DATABASE_URL,
  min: parseInt(process.env.DATABASE_POOL_MIN || '2'),
  max: parseInt(process.env.DATABASE_POOL_MAX || '10'),
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: parseInt(process.env.DATABASE_CONNECTION_TIMEOUT || '30000'),
  ssl: process.env.NODE_ENV === 'production' ? { rejectUnauthorized: false } : false,
};

/**
 * Global database connection pool instance
 */
export const pool = new Pool(poolConfig);

/**
 * Initialize database connection pool
 */
export async function initializeDatabase(): Promise<void> {
  try {
    // Test connection
    const client = await pool.connect();
    await client.query('SELECT 1');
    client.release();

    console.log('✅ Database connection pool initialized successfully');
    console.log(`📊 Pool config: min=${poolConfig.min}, max=${poolConfig.max}`);
  } catch (error) {
    console.error('❌ Failed to initialize database connection pool:', error);
    throw new Error(`Database connection failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

/**
 * Check database health and connectivity
 */
export async function checkDatabaseHealth(): Promise<{
  status: 'healthy' | 'unhealthy';
  error?: string;
  metrics?: {
    totalConnections: number;
    idleConnections: number;
    waitingClients: number;
  };
}> {
  try {
    // Test basic connectivity
    const client = await pool.connect();
    await client.query('SELECT 1');
    client.release();

    // Get pool metrics
    const metrics = {
      totalConnections: (pool as any).totalCount || 0,
      idleConnections: (pool as any).idleCount || 0,
      waitingClients: (pool as any).waitingCount || 0,
    };

    return {
      status: 'healthy',
      metrics,
    };
  } catch (error) {
    return {
      status: 'unhealthy',
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

/**
 * Graceful shutdown handler for database connections
 */
export async function closeDatabaseConnection(): Promise<void> {
  try {
    await pool.end();
    console.log('✅ Database connection pool closed gracefully');
  } catch (error) {
    console.error('❌ Error closing database connection pool:', error);
  }
}

/**
 * Handle process termination
 */
process.on('SIGTERM', async () => {
  console.log('🛑 SIGTERM received, closing database connections...');
  await closeDatabaseConnection();
});

process.on('SIGINT', async () => {
  console.log('🛑 SIGINT received, closing database connections...');
  await closeDatabaseConnection();
});

/**
 * Database configuration constants
 */
export const DATABASE_CONFIG = {
  CONNECTION_STRING: process.env.DATABASE_URL || 'postgresql://iora_user:iora_password_2024@localhost:5432/iora_dev',
  POOL_MIN: parseInt(process.env.DATABASE_POOL_MIN || '2'),
  POOL_MAX: parseInt(process.env.DATABASE_POOL_MAX || '10'),
  CONNECTION_TIMEOUT: parseInt(process.env.DATABASE_CONNECTION_TIMEOUT || '30000'),
  SSL_ENABLED: process.env.NODE_ENV === 'production',
} as const;

/**
 * Default database configuration for development
 */
export const DEFAULT_DATABASE_CONFIG = {
  host: 'localhost',
  port: 5432,
  database: 'iora_dev',
  username: 'iora_user',
  password: 'iora_password_2024',
  ssl: false,
  poolMin: 2,
  poolMax: 10,
  connectionTimeoutMillis: 30000,
  idleTimeoutMillis: 30000,
} as const;

/**
 * Production database configuration template
 */
export const PRODUCTION_DATABASE_CONFIG = {
  host: process.env.DB_HOST || 'localhost',
  port: parseInt(process.env.DB_PORT || '5432'),
  database: process.env.DB_NAME || 'iora_prod',
  username: process.env.DB_USER || 'iora_user',
  password: process.env.DB_PASSWORD, // Must be set in production
  ssl: { rejectUnauthorized: false },
  poolMin: parseInt(process.env.DB_POOL_MIN || '5'),
  poolMax: parseInt(process.env.DB_POOL_MAX || '20'),
  connectionTimeoutMillis: parseInt(process.env.DB_CONNECTION_TIMEOUT || '60000'),
  idleTimeoutMillis: 30000,
} as const;
