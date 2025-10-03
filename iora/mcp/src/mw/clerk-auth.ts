/**
 * Clerk Authentication Middleware
 * 
 * Provides user authentication using Clerk for user-facing endpoints.
 * Separates service-level auth (HMAC) from user-level auth (Clerk).
 */

import { Request, Response, NextFunction } from 'express';
import { clerkClient } from '@clerk/clerk-sdk-node';

// Extend Express Request type to include auth context
declare global {
  namespace Express {
    interface Request {
      auth?: {
        userId: string;
        orgId?: string;
        sessionId?: string;
        role?: string;
      };
    }
  }
}

/**
 * Clerk authentication middleware for user endpoints
 * Validates session tokens and populates req.auth
 */
export async function clerkAuth(req: Request, res: Response, next: NextFunction) {
  try {
    // Extract session token from Authorization header or cookie
    const sessionToken = 
      req.headers.authorization?.replace('Bearer ', '') ||
      req.cookies?.__session;

    if (!sessionToken) {
      return res.status(401).json({
        ok: false,
        error: 'Unauthorized - No session token provided',
        message: 'Please sign in to access this resource'
      });
    }

    // Verify the session token with Clerk
    const session = await clerkClient.sessions.verifySession(sessionToken, sessionToken);

    if (!session) {
      return res.status(401).json({
        ok: false,
        error: 'Unauthorized - Invalid session',
        message: 'Your session has expired. Please sign in again.'
      });
    }

    // Get user details
    const user = await clerkClient.users.getUser(session.userId);

    // Populate auth context
    req.auth = {
      userId: session.userId,
      sessionId: session.id,
      orgId: (session as any).orgId || undefined, // orgId may not be on base Session type
      role: user.publicMetadata?.role as string || 'user'
    };

    next();
  } catch (error: any) {
    console.error('Clerk auth error:', error);
    return res.status(401).json({
      ok: false,
      error: 'Authentication failed',
      message: error.message || 'Unable to verify session'
    });
  }
}

/**
 * Require admin role middleware
 * Must be used after clerkAuth
 */
export function requireAdmin(req: Request, res: Response, next: NextFunction) {
  if (!req.auth) {
    return res.status(401).json({
      ok: false,
      error: 'Unauthorized',
      message: 'Authentication required'
    });
  }

  if (req.auth.role !== 'admin') {
    return res.status(403).json({
      ok: false,
      error: 'Forbidden',
      message: 'Admin access required'
    });
  }

  next();
}

/**
 * Require organization membership middleware
 * Must be used after clerkAuth
 */
export function requireOrg(req: Request, res: Response, next: NextFunction) {
  if (!req.auth) {
    return res.status(401).json({
      ok: false,
      error: 'Unauthorized',
      message: 'Authentication required'
    });
  }

  if (!req.auth.orgId) {
    return res.status(403).json({
      ok: false,
      error: 'Forbidden',
      message: 'Organization membership required'
    });
  }

  next();
}

/**
 * Get user tier based on organization or user metadata
 */
export async function getUserTier(userId: string, orgId?: string): Promise<'free' | 'pro' | 'enterprise'> {
  try {
    if (orgId) {
      // Get organization metadata
      const org = await clerkClient.organizations.getOrganization({ organizationId: orgId });
      return (org.publicMetadata?.tier as any) || 'free';
    }

    // Get user metadata
    const user = await clerkClient.users.getUser(userId);
    return (user.publicMetadata?.tier as any) || 'free';
  } catch (error) {
    console.error('Error fetching user tier:', error);
    return 'free';
  }
}

/**
 * Update user or organization tier
 */
export async function updateTier(
  userId: string,
  tier: 'free' | 'pro' | 'enterprise',
  orgId?: string
): Promise<void> {
  try {
    if (orgId) {
      await clerkClient.organizations.updateOrganization(orgId, {
        publicMetadata: { tier }
      });
    } else {
      await clerkClient.users.updateUser(userId, {
        publicMetadata: { tier }
      });
    }
  } catch (error) {
    console.error('Error updating tier:', error);
    throw new Error('Failed to update tier');
  }
}

