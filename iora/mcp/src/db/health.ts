/**
 * Database Health Check Module
 *
 * Provides comprehensive health monitoring for PostgreSQL and Redis databases.
 * Used by the MCP server to ensure database connectivity before accepting requests.
 */

import { checkDatabaseHealth } from '../config/database';

/**
 * Database health status interface
 */
export interface DatabaseHealthStatus {
  postgresql: {
    status: 'healthy' | 'unhealthy';
    error?: string;
    metrics?: {
      totalConnections: number;
      idleConnections: number;
      waitingClients: number;
    };
  };
  redis?: {
    status: 'healthy' | 'unhealthy';
    error?: string;
    metrics?: {
      connectedClients: number;
      usedMemory: number;
      hitRate: number;
    };
  };
  overall: 'healthy' | 'degraded' | 'unhealthy';
  timestamp: string;
}

/**
 * Check comprehensive database health
 */
export async function checkDatabaseHealthComprehensive(): Promise<DatabaseHealthStatus> {
  const timestamp = new Date().toISOString();

  try {
    // Check PostgreSQL health
    const postgresHealth = await checkDatabaseHealth();

    // Check Redis health (if Redis is configured)
    let redisHealth: DatabaseHealthStatus['redis'];
    try {
      if (process.env.REDIS_URL) {
        const redis = await import('ioredis');
        const redisClient = new redis.default(process.env.REDIS_URL);
        await redisClient.ping();
        redisHealth = {
          status: 'healthy',
        };
        await redisClient.quit();
      }
    } catch (redisError) {
      redisHealth = {
        status: 'unhealthy',
        error: redisError instanceof Error ? redisError.message : 'Unknown Redis error',
      };
    }

    // Determine overall health
    const postgresHealthy = postgresHealth.status === 'healthy';
    const redisHealthy = redisHealth?.status === 'healthy';

    let overall: 'healthy' | 'degraded' | 'unhealthy' = 'healthy';
    if (!postgresHealthy) {
      overall = 'unhealthy';
    } else if (!redisHealthy && redisHealth) {
      overall = 'degraded';
    }

    return {
      postgresql: postgresHealth,
      redis: redisHealth,
      overall,
      timestamp,
    };
  } catch (error) {
    return {
      postgresql: {
        status: 'unhealthy',
        error: error instanceof Error ? error.message : 'Unknown error',
      },
      overall: 'unhealthy',
      timestamp,
    };
  }
}

/**
 * Get detailed database metrics
 */
export async function getDatabaseMetrics(): Promise<{
  postgresql: {
    poolStats: {
      totalConnections: number;
      idleConnections: number;
      waitingClients: number;
    };
    queryStats?: {
      totalQueries: number;
      averageQueryTime: number;
      slowQueries: number;
    };
  };
  redis?: {
    info: {
      connectedClients: number;
      usedMemory: number;
      hitRate: number;
      keyspaceHits: number;
      keyspaceMisses: number;
    };
  };
}> {
  const metrics: any = {
    postgresql: {
      poolStats: {
        totalConnections: 0,
        idleConnections: 0,
        waitingClients: 0,
      },
    },
  };

  try {
    // PostgreSQL pool metrics
    if ((global as any).dbPool) {
      const pool = (global as any).dbPool;
      metrics.postgresql.poolStats = {
        totalConnections: pool.totalCount || 0,
        idleConnections: pool.idleCount || 0,
        waitingClients: pool.waitingCount || 0,
      };
    }

    // Redis metrics (if available)
    if (process.env.REDIS_URL) {
      try {
        const redis = await import('ioredis');
        const redisClient = new redis.default(process.env.REDIS_URL);
        const info = await redisClient.info();

        // Parse Redis INFO command output
        const lines = info.split('\r\n');
        let connectedClients = 0;
        let usedMemory = 0;
        let keyspaceHits = 0;
        let keyspaceMisses = 0;

        for (const line of lines) {
          if (line.startsWith('connected_clients:')) {
            connectedClients = parseInt(line.split(':')[1]);
          } else if (line.startsWith('used_memory:')) {
            usedMemory = parseInt(line.split(':')[1]);
          } else if (line.startsWith('keyspace_hits:')) {
            keyspaceHits = parseInt(line.split(':')[1]);
          } else if (line.startsWith('keyspace_misses:')) {
            keyspaceMisses = parseInt(line.split(':')[1]);
          }
        }

        const totalRequests = keyspaceHits + keyspaceMisses;
        const hitRate = totalRequests > 0 ? (keyspaceHits / totalRequests) * 100 : 0;

        metrics.redis = {
          info: {
            connectedClients,
            usedMemory,
            hitRate,
            keyspaceHits,
            keyspaceMisses,
          },
        };

        await redisClient.quit();
      } catch (redisError) {
        // Redis metrics not available, skip
      }
    }
  } catch (error) {
    // Metrics collection failed, return partial data
  }

  return metrics;
}
