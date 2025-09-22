import { TelemetryEvent } from "./registry-config.js";
import crypto from "crypto";

/**
 * Coral Protocol v1.0 Telemetry Manager
 * Handles monitoring, analytics, and event tracking
 */
export class TelemetryManager {
  private events: TelemetryEvent[] = [];
  private maxEvents: number;
  private eventHandlers: Map<string, ((event: TelemetryEvent) => void)[]> = new Map();

  constructor(maxEvents: number = 10000) {
    this.maxEvents = maxEvents;

    // Cleanup old events every hour
    setInterval(() => {
      this.cleanupOldEvents();
    }, 60 * 60 * 1000);
  }

  /**
   * Record a telemetry event
   */
  recordEvent(eventType: string, agentId: string, data: Record<string, any>, sessionId?: string, threadId?: string): string {
    const eventId = crypto.randomUUID();
    const event: TelemetryEvent = {
      id: eventId,
      timestamp: new Date(),
      event: eventType,
      agentId,
      sessionId,
      threadId,
      data: {
        ...data,
        userAgent: data.userAgent || 'unknown',
        ip: data.ip || 'unknown',
        duration: data.duration || 0
      }
    };

    this.events.push(event);

    // Trigger event handlers
    const handlers = this.eventHandlers.get(eventType) || [];
    handlers.forEach(handler => {
      try {
        handler(event);
      } catch (error) {
        console.error(`Telemetry event handler error for ${eventType}:`, error);
      }
    });

    // Cleanup if we exceed max events
    if (this.events.length > this.maxEvents) {
      this.cleanupOldEvents();
    }

    return eventId;
  }

  /**
   * Add event handler for specific event type
   */
  on(eventType: string, handler: (event: TelemetryEvent) => void): void {
    const handlers = this.eventHandlers.get(eventType) || [];
    handlers.push(handler);
    this.eventHandlers.set(eventType, handlers);
  }

  /**
   * Remove event handler
   */
  off(eventType: string, handler: (event: TelemetryEvent) => void): void {
    const handlers = this.eventHandlers.get(eventType) || [];
    const index = handlers.indexOf(handler);
    if (index > -1) {
      handlers.splice(index, 1);
      this.eventHandlers.set(eventType, handlers);
    }
  }

  /**
   * Get events by type
   */
  getEventsByType(eventType: string, limit?: number): TelemetryEvent[] {
    const filtered = this.events.filter(e => e.event === eventType);
    return limit ? filtered.slice(-limit) : filtered;
  }

  /**
   * Get events for agent
   */
  getEventsForAgent(agentId: string, limit?: number): TelemetryEvent[] {
    const filtered = this.events.filter(e => e.agentId === agentId);
    return limit ? filtered.slice(-limit) : filtered;
  }

  /**
   * Get events for session
   */
  getEventsForSession(sessionId: string, limit?: number): TelemetryEvent[] {
    const filtered = this.events.filter(e => e.sessionId === sessionId);
    return limit ? filtered.slice(-limit) : filtered;
  }

  /**
   * Get analytics data
   */
  getAnalytics(timeframeHours: number = 24): {
    totalEvents: number;
    eventsByType: Record<string, number>;
    eventsByAgent: Record<string, number>;
    averageEventsPerHour: number;
    topAgents: Array<{ agentId: string; count: number }>;
    recentErrors: TelemetryEvent[];
  } {
    const cutoffTime = new Date();
    cutoffTime.setHours(cutoffTime.getHours() - timeframeHours);

    const recentEvents = this.events.filter(e => e.timestamp >= cutoffTime);

    const eventsByType: Record<string, number> = {};
    const eventsByAgent: Record<string, number> = {};
    const errors: TelemetryEvent[] = [];

    recentEvents.forEach(event => {
      // Count by type
      eventsByType[event.event] = (eventsByType[event.event] || 0) + 1;

      // Count by agent
      eventsByAgent[event.agentId] = (eventsByAgent[event.agentId] || 0) + 1;

      // Collect errors
      if (event.event === 'error' || event.data.error) {
        errors.push(event);
      }
    });

    // Get top agents
    const topAgents = Object.entries(eventsByAgent)
      .map(([agentId, count]) => ({ agentId, count }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);

    return {
      totalEvents: recentEvents.length,
      eventsByType,
      eventsByAgent,
      averageEventsPerHour: recentEvents.length / timeframeHours,
      topAgents,
      recentErrors: errors.slice(-10) // Last 10 errors
    };
  }

  /**
   * Get performance metrics
   */
  getPerformanceMetrics(): {
    averageResponseTime: number;
    errorRate: number;
    requestsPerMinute: number;
    slowestOperations: Array<{ operation: string; avgTime: number; count: number }>;
  } {
    const performanceEvents = this.events.filter(e =>
      e.data.duration || e.data.responseTime
    );

    if (performanceEvents.length === 0) {
      return {
        averageResponseTime: 0,
        errorRate: 0,
        requestsPerMinute: 0,
        slowestOperations: []
      };
    }

    const totalTime = performanceEvents.reduce((sum, e) => {
      return sum + (e.data.duration || e.data.responseTime || 0);
    }, 0);

    const errorEvents = this.events.filter(e =>
      e.event === 'error' || e.data.error
    );

    const timeWindow = Math.max(
      (Date.now() - this.events[0]?.timestamp.getTime()) / (1000 * 60),
      1
    );

    const operations: Record<string, { totalTime: number; count: number }> = {};

    performanceEvents.forEach(event => {
      const operation = event.data.operation || event.event;
      if (!operations[operation]) {
        operations[operation] = { totalTime: 0, count: 0 };
      }
      operations[operation].totalTime += event.data.duration || event.data.responseTime || 0;
      operations[operation].count++;
    });

    const slowestOperations = Object.entries(operations)
      .map(([operation, data]) => ({
        operation,
        avgTime: data.totalTime / data.count,
        count: data.count
      }))
      .sort((a, b) => b.avgTime - a.avgTime)
      .slice(0, 5);

    return {
      averageResponseTime: totalTime / performanceEvents.length,
      errorRate: errorEvents.length / this.events.length,
      requestsPerMinute: this.events.length / timeWindow,
      slowestOperations
    };
  }

  /**
   * Export events for external analysis
   */
  exportEvents(format: 'json' | 'csv' = 'json'): string {
    if (format === 'csv') {
      const headers = ['id', 'timestamp', 'event', 'agentId', 'sessionId', 'threadId', 'data'];
      const rows = this.events.map(event => [
        event.id,
        event.timestamp.toISOString(),
        event.event,
        event.agentId,
        event.sessionId || '',
        event.threadId || '',
        JSON.stringify(event.data)
      ]);

      return [headers, ...rows].map(row =>
        row.map(field => `"${field}"`).join(',')
      ).join('\n');
    }

    return JSON.stringify(this.events, null, 2);
  }

  /**
   * Clear all events
   */
  clearEvents(): void {
    this.events = [];
    console.log('🧹 Cleared all telemetry events');
  }

  /**
   * Cleanup old events
   */
  private cleanupOldEvents(): void {
    const cutoffTime = new Date();
    cutoffTime.setDate(cutoffTime.getDate() - 7); // Keep 7 days

    const initialLength = this.events.length;
    this.events = this.events.filter(e => e.timestamp >= cutoffTime);

    const cleaned = initialLength - this.events.length;
    if (cleaned > 0) {
      console.log(`🧹 Cleaned up ${cleaned} old telemetry events`);
    }
  }
}
