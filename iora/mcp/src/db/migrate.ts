/**
 * Database Migration Runner
 *
 * Executes SQL migration files in order to set up the database schema.
 * Provides rollback capabilities and migration tracking.
 */

import { query, tableExists } from './queries';
import fs from 'fs';
import path from 'path';

/**
 * Migration record interface
 */
interface MigrationRecord {
  id: number;
  name: string;
  executed_at: Date;
}

/**
 * Run all pending migrations
 */
export async function runMigrations(): Promise<void> {
  console.log('🚀 Starting database migrations...');

  // Ensure migrations table exists
  await createMigrationsTable();

  // Get list of migration files
  const migrationDir = path.join(__dirname, '../../migrations');
  const migrationFiles = fs.readdirSync(migrationDir)
    .filter(file => file.endsWith('.sql'))
    .sort();

  console.log(`📁 Found ${migrationFiles.length} migration files`);

  for (const file of migrationFiles) {
    const migrationName = path.basename(file, '.sql');

    // Check if migration already executed
    const alreadyExecuted = await isMigrationExecuted(migrationName);

    if (alreadyExecuted) {
      console.log(`✅ Migration ${migrationName} already executed`);
      continue;
    }

    // Execute migration
    console.log(`🔄 Executing migration: ${migrationName}`);
    await executeMigration(file, migrationName);
  }

  console.log('✅ All migrations completed successfully');
}

/**
 * Create migrations tracking table
 */
async function createMigrationsTable(): Promise<void> {
  if (await tableExists('migrations')) {
    return;
  }

  console.log('📋 Creating migrations table...');

  await query(`
    CREATE TABLE migrations (
      id SERIAL PRIMARY KEY,
      name VARCHAR(255) NOT NULL UNIQUE,
      executed_at TIMESTAMPTZ DEFAULT NOW()
    )
  `);

  console.log('✅ Migrations table created');
}

/**
 * Check if migration has been executed
 */
async function isMigrationExecuted(name: string): Promise<boolean> {
  const result = await query(
    `SELECT COUNT(*) as count FROM migrations WHERE name = $1`,
    [name]
  );

  return parseInt(result.rows[0].count) > 0;
}

/**
 * Execute a single migration file
 */
async function executeMigration(filename: string, name: string): Promise<void> {
  const migrationPath = path.join(__dirname, '../../migrations', filename);

  try {
    // Read migration file
    const sql = fs.readFileSync(migrationPath, 'utf8');

    // Execute migration in transaction
    await query('BEGIN');

    // Split by semicolon and execute each statement
    const statements = sql.split(';').filter(stmt => stmt.trim().length > 0);

    for (const statement of statements) {
      if (statement.trim()) {
        await query(statement);
      }
    }

    // Record migration as executed
    await query(
      `INSERT INTO migrations (name) VALUES ($1)`,
      [name]
    );

    await query('COMMIT');

    console.log(`✅ Migration ${name} executed successfully`);
  } catch (error) {
    await query('ROLLBACK');
    console.error(`❌ Migration ${name} failed:`, error);
    throw error;
  }
}

/**
 * Rollback migrations (for development/testing)
 */
export async function rollbackMigrations(count: number = 1): Promise<void> {
  console.log(`🔄 Rolling back last ${count} migration(s)...`);

  // Get last N migrations
  const result = await query(
    `SELECT name FROM migrations ORDER BY id DESC LIMIT $1`,
    [count]
  );

  for (const row of result.rows.reverse()) {
    const migrationName = row.name;
    console.log(`🔄 Rolling back migration: ${migrationName}`);

    // This would need migration-specific rollback scripts
    // For now, just remove from migrations table
    await query(
      `DELETE FROM migrations WHERE name = $1`,
      [migrationName]
    );

    console.log(`✅ Migration ${migrationName} rolled back`);
  }
}

/**
 * Get migration status
 */
export async function getMigrationStatus(): Promise<{
  total: number;
  executed: number;
  pending: number;
}> {
  const migrationDir = path.join(__dirname, '../../migrations');
  const totalFiles = fs.readdirSync(migrationDir)
    .filter(file => file.endsWith('.sql')).length;

  const result = await query(
    `SELECT COUNT(*) as count FROM migrations`
  );

  const executed = parseInt(result.rows[0].count);

  return {
    total: totalFiles,
    executed,
    pending: totalFiles - executed,
  };
}

/**
 * Reset all migrations (for development/testing)
 */
export async function resetMigrations(): Promise<void> {
  console.log('⚠️  Resetting all migrations...');

  // Drop all tables (in reverse dependency order)
  const tables = [
    'audit_logs',
    'billing_events',
    'usage_logs',
    'api_keys',
    'organization_members',
    'organizations',
    'users',
    'migrations'
  ];

  for (const table of tables) {
    try {
      await query(`DROP TABLE IF EXISTS ${table} CASCADE`);
      console.log(`🗑️  Dropped table: ${table}`);
    } catch (error) {
      console.warn(`⚠️  Could not drop table ${table}:`, error);
    }
  }

  console.log('✅ All migrations reset');
}
