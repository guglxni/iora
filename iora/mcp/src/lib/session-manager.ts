import { CoralSession } from "./registry-config.js";
import crypto from "crypto";

/**
 * Coral Protocol v1.0 Session Manager
 * Manages agent sessions and conversation state
 */
export class SessionManager {
  private sessions: Map<string, CoralSession> = new Map();
  private sessionTimeout: number; // milliseconds

  constructor(sessionTimeoutMinutes: number = 60) {
    this.sessionTimeout = sessionTimeoutMinutes * 60 * 1000;

    // Cleanup expired sessions every 5 minutes
    setInterval(() => {
      this.cleanupExpiredSessions();
    }, 5 * 60 * 1000);
  }

  /**
   * Create a new session
   */
  createSession(agentId: string, clientId?: string): CoralSession {
    const sessionId = crypto.randomUUID();
    const now = new Date();

    const session: CoralSession = {
      id: sessionId,
      agentId,
      clientId,
      startedAt: now,
      lastActivity: now,
      status: 'active',
      metadata: {},
      threadIds: []
    };

    this.sessions.set(sessionId, session);
    console.log(`📋 Created new session: ${sessionId} for agent: ${agentId}`);

    return session;
  }

  /**
   * Get session by ID
   */
  getSession(sessionId: string): CoralSession | undefined {
    return this.sessions.get(sessionId);
  }

  /**
   * Update session activity
   */
  updateSessionActivity(sessionId: string): boolean {
    const session = this.sessions.get(sessionId);
    if (session && session.status === 'active') {
      session.lastActivity = new Date();
      return true;
    }
    return false;
  }

  /**
   * End a session
   */
  endSession(sessionId: string): boolean {
    const session = this.sessions.get(sessionId);
    if (session) {
      session.status = 'inactive';
      console.log(`🛑 Ended session: ${sessionId}`);
      return true;
    }
    return false;
  }

  /**
   * Add metadata to session
   */
  addSessionMetadata(sessionId: string, key: string, value: any): boolean {
    const session = this.sessions.get(sessionId);
    if (session) {
      session.metadata[key] = value;
      return true;
    }
    return false;
  }

  /**
   * Get active sessions
   */
  getActiveSessions(): CoralSession[] {
    return Array.from(this.sessions.values()).filter(s => s.status === 'active');
  }

  /**
   * Get sessions for agent
   */
  getAgentSessions(agentId: string): CoralSession[] {
    return Array.from(this.sessions.values()).filter(s => s.agentId === agentId);
  }

  /**
   * Cleanup expired sessions
   */
  private cleanupExpiredSessions(): void {
    const now = new Date();
    let cleaned = 0;

    for (const [sessionId, session] of this.sessions.entries()) {
      if (session.status === 'active' &&
          (now.getTime() - session.lastActivity.getTime()) > this.sessionTimeout) {
        session.status = 'expired';
        cleaned++;
      }
    }

    if (cleaned > 0) {
      console.log(`🧹 Cleaned up ${cleaned} expired sessions`);
    }
  }

  /**
   * Get session statistics
   */
  getStats() {
    const sessions = Array.from(this.sessions.values());
    const active = sessions.filter(s => s.status === 'active').length;
    const inactive = sessions.filter(s => s.status === 'inactive').length;
    const expired = sessions.filter(s => s.status === 'expired').length;

    return {
      total: sessions.length,
      active,
      inactive,
      expired,
      averageSessionTime: this.calculateAverageSessionTime()
    };
  }

  private calculateAverageSessionTime(): number {
    const completedSessions = Array.from(this.sessions.values())
      .filter(s => s.status !== 'active');

    if (completedSessions.length === 0) return 0;

    const totalTime = completedSessions.reduce((sum, session) => {
      const endTime = session.status === 'expired' ?
        session.lastActivity.getTime() :
        Date.now();
      return sum + (endTime - session.startedAt.getTime());
    }, 0);

    return totalTime / completedSessions.length;
  }
}
