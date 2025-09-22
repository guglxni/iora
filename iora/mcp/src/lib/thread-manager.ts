import { CoralThread } from "./registry-config.js";
import crypto from "crypto";

/**
 * Coral Protocol v1.0 Thread Manager
 * Manages conversation threads within sessions
 */
export class ThreadManager {
  private threads: Map<string, CoralThread> = new Map();

  /**
   * Create a new thread
   */
  createThread(sessionId: string, title?: string): CoralThread {
    const threadId = crypto.randomUUID();
    const now = new Date();

    const thread: CoralThread = {
      id: threadId,
      sessionId,
      title: title || `Thread ${threadId.substring(0, 8)}`,
      createdAt: now,
      updatedAt: now,
      messageCount: 0,
      tags: [],
      metadata: {}
    };

    this.threads.set(threadId, thread);
    console.log(`🧵 Created new thread: ${threadId} in session: ${sessionId}`);

    return thread;
  }

  /**
   * Get thread by ID
   */
  getThread(threadId: string): CoralThread | undefined {
    return this.threads.get(threadId);
  }

  /**
   * Update thread activity
   */
  updateThreadActivity(threadId: string, messageCount?: number): boolean {
    const thread = this.threads.get(threadId);
    if (thread) {
      thread.updatedAt = new Date();
      if (messageCount !== undefined) {
        thread.messageCount = messageCount;
      }
      return true;
    }
    return false;
  }

  /**
   * Add tag to thread
   */
  addThreadTag(threadId: string, tag: string): boolean {
    const thread = this.threads.get(threadId);
    if (thread && !thread.tags.includes(tag)) {
      thread.tags.push(tag);
      return true;
    }
    return false;
  }

  /**
   * Remove tag from thread
   */
  removeThreadTag(threadId: string, tag: string): boolean {
    const thread = this.threads.get(threadId);
    if (thread) {
      const index = thread.tags.indexOf(tag);
      if (index > -1) {
        thread.tags.splice(index, 1);
        return true;
      }
    }
    return false;
  }

  /**
   * Add metadata to thread
   */
  addThreadMetadata(threadId: string, key: string, value: any): boolean {
    const thread = this.threads.get(threadId);
    if (thread) {
      thread.metadata[key] = value;
      return true;
    }
    return false;
  }

  /**
   * Get threads for session
   */
  getSessionThreads(sessionId: string): CoralThread[] {
    return Array.from(this.threads.values()).filter(t => t.sessionId === sessionId);
  }

  /**
   * Get threads with specific tag
   */
  getThreadsWithTag(tag: string): CoralThread[] {
    return Array.from(this.threads.values()).filter(t => t.tags.includes(tag));
  }

  /**
   * Search threads by title or tags
   */
  searchThreads(query: string): CoralThread[] {
    const lowercaseQuery = query.toLowerCase();
    return Array.from(this.threads.values()).filter(thread =>
      thread.title?.toLowerCase().includes(lowercaseQuery) ||
      thread.tags.some(tag => tag.toLowerCase().includes(lowercaseQuery))
    );
  }

  /**
   * Delete thread
   */
  deleteThread(threadId: string): boolean {
    const deleted = this.threads.delete(threadId);
    if (deleted) {
      console.log(`🗑️ Deleted thread: ${threadId}`);
    }
    return deleted;
  }

  /**
   * Get thread statistics
   */
  getStats() {
    const threads = Array.from(this.threads.values());
    const totalMessages = threads.reduce((sum, thread) => sum + thread.messageCount, 0);

    return {
      total: threads.length,
      averageMessagesPerThread: threads.length > 0 ? totalMessages / threads.length : 0,
      threadsWithTags: threads.filter(t => t.tags.length > 0).length,
      oldestThread: threads.length > 0 ? new Date(Math.min(...threads.map(t => t.createdAt.getTime()))) : null,
      newestThread: threads.length > 0 ? new Date(Math.max(...threads.map(t => t.createdAt.getTime()))) : null
    };
  }

  /**
   * Archive old threads (older than specified days)
   */
  archiveOldThreads(daysOld: number = 30): number {
    const cutoffDate = new Date();
    cutoffDate.setDate(cutoffDate.getDate() - daysOld);

    let archived = 0;
    for (const [threadId, thread] of this.threads.entries()) {
      if (thread.createdAt < cutoffDate) {
        thread.metadata.archived = true;
        thread.metadata.archivedAt = new Date().toISOString();
        archived++;
      }
    }

    if (archived > 0) {
      console.log(`📦 Archived ${archived} old threads`);
    }

    return archived;
  }
}
