/**
 * User-Facing API Routes
 * 
 * Provides endpoints for user profile, organization management,
 * API key management, and usage tracking.
 * 
 * These routes use Clerk authentication (not HMAC).
 */

import express, { Request, Response } from 'express';
import { clerkAuth, requireAdmin, getUserTier, updateTier } from '../mw/clerk-auth.js';
import { createApiKey, listApiKeys, revokeApiKey } from '../lib/api-keys.js';
import { clerkClient } from '@clerk/clerk-sdk-node';

const router = express.Router();

// Apply Clerk auth to all user routes
router.use(clerkAuth);

/**
 * GET /user/profile
 * Get current user profile
 */
router.get('/profile', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const user = await clerkClient.users.getUser(req.auth.userId);

    res.json({
      ok: true,
      data: {
        id: user.id,
        email: user.emailAddresses[0]?.emailAddress,
        firstName: user.firstName,
        lastName: user.lastName,
        imageUrl: user.imageUrl,
        tier: user.publicMetadata?.tier || 'free',
        createdAt: user.createdAt,
        metadata: user.publicMetadata
      }
    });
  } catch (error: any) {
    console.error('Error fetching user profile:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to fetch profile',
      message: error.message
    });
  }
});

/**
 * GET /user/organizations
 * List user's organizations
 */
router.get('/organizations', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const orgs = await clerkClient.users.getOrganizationMembershipList({
      userId: req.auth.userId
    });

    const organizations = await Promise.all(
      orgs.map(async (membership: any) => {
        const org = await clerkClient.organizations.getOrganization({
          organizationId: membership.organization.id
        });

        return {
          id: org.id,
          name: org.name,
          slug: org.slug,
          imageUrl: org.imageUrl,
          role: membership.role,
          tier: org.publicMetadata?.tier || 'free',
          createdAt: org.createdAt
        };
      })
    );

    res.json({
      ok: true,
      data: organizations
    });
  } catch (error: any) {
    console.error('Error fetching organizations:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to fetch organizations',
      message: error.message
    });
  }
});

/**
 * GET /user/api-keys
 * List user's API keys
 */
router.get('/api-keys', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const keys = await listApiKeys(req.auth.userId, req.auth.orgId);

    res.json({
      ok: true,
      data: keys.map(key => ({
        id: key.id,
        name: key.name,
        keyPrefix: key.keyPrefix,
        createdAt: key.createdAt,
        lastUsedAt: key.lastUsedAt,
        expiresAt: key.expiresAt,
        permissions: key.permissions
      }))
    });
  } catch (error: any) {
    console.error('Error listing API keys:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to list API keys',
      message: error.message
    });
  }
});

/**
 * POST /user/api-keys
 * Create a new API key
 */
router.post('/api-keys', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const { name, permissions, expiresInDays } = req.body;

    if (!name || typeof name !== 'string') {
      return res.status(400).json({
        ok: false,
        error: 'Bad Request',
        message: 'API key name is required'
      });
    }

    const result = await createApiKey({
      userId: req.auth.userId,
      orgId: req.auth.orgId,
      name,
      permissions,
      expiresInDays
    });

    res.json({
      ok: true,
      data: {
        id: result.id,
        key: result.key, // Only shown once!
        keyPrefix: result.prefix,
        message: 'Save this key securely. It will not be shown again.'
      }
    });
  } catch (error: any) {
    console.error('Error creating API key:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to create API key',
      message: error.message
    });
  }
});

/**
 * DELETE /user/api-keys/:keyId
 * Revoke an API key
 */
router.delete('/api-keys/:keyId', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const success = await revokeApiKey(req.params.keyId, req.auth.userId);

    if (!success) {
      return res.status(404).json({
        ok: false,
        error: 'Not Found',
        message: 'API key not found or unauthorized'
      });
    }

    res.json({
      ok: true,
      message: 'API key revoked successfully'
    });
  } catch (error: any) {
    console.error('Error revoking API key:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to revoke API key',
      message: error.message
    });
  }
});

/**
 * GET /user/usage
 * Get usage statistics for current billing period
 */
router.get('/usage', async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    // Get user tier to determine limits
    const tier = await getUserTier(req.auth.userId, req.auth.orgId);

    // Usage limits by tier
    const limits: Record<string, { requestsPerMinute: number; requestsPerMonth: number }> = {
      free: { requestsPerMinute: 60, requestsPerMonth: 10000 },
      pro: { requestsPerMinute: 1000, requestsPerMonth: 100000 },
      enterprise: { requestsPerMinute: -1, requestsPerMonth: -1 } // unlimited
    };

    // TODO: Fetch actual usage from telemetry/database
    const currentUsage = {
      requestsThisMonth: 0, // Replace with actual count
      requestsToday: 0,
      lastRequest: null
    };

    res.json({
      ok: true,
      data: {
        tier,
        limits: limits[tier],
        usage: currentUsage,
        remaining: {
          requestsThisMonth: limits[tier].requestsPerMonth === -1 
            ? -1 
            : limits[tier].requestsPerMonth - currentUsage.requestsThisMonth
        }
      }
    });
  } catch (error: any) {
    console.error('Error fetching usage:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to fetch usage',
      message: error.message
    });
  }
});

/**
 * POST /user/tier
 * Update user tier (admin only or via billing webhook)
 */
router.post('/tier', requireAdmin, async (req: Request, res: Response) => {
  try {
    if (!req.auth?.userId) {
      return res.status(401).json({ ok: false, error: 'Unauthorized' });
    }

    const { tier, targetUserId, targetOrgId } = req.body;

    if (!tier || !['free', 'pro', 'enterprise'].includes(tier)) {
      return res.status(400).json({
        ok: false,
        error: 'Bad Request',
        message: 'Invalid tier. Must be: free, pro, or enterprise'
      });
    }

    await updateTier(targetUserId || req.auth.userId, tier, targetOrgId || req.auth.orgId);

    res.json({
      ok: true,
      message: `Tier updated to ${tier} successfully`
    });
  } catch (error: any) {
    console.error('Error updating tier:', error);
    res.status(500).json({
      ok: false,
      error: 'Failed to update tier',
      message: error.message
    });
  }
});

export default router;

