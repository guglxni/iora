import { CoralAgent } from "./registry-config.js";
import { SessionManager } from "./session-manager.js";
import { ThreadManager } from "./thread-manager.js";
import { TelemetryManager } from "./telemetry.js";
import { WorkflowManager, Workflow } from "./workflow-manager.js";
import crypto from "crypto";

/**
 * Coral Protocol v1.0 Agent Manager
 * Manages AI agents, their capabilities, and interactions
 */
export class AgentManager {
  private agents: Map<string, CoralAgent> = new Map();
  private sessionManager: SessionManager;
  private threadManager: ThreadManager;
  private telemetryManager: TelemetryManager;
  private workflowManager: WorkflowManager;

  constructor(
    sessionManager: SessionManager,
    threadManager: ThreadManager,
    telemetryManager: TelemetryManager
  ) {
    this.sessionManager = sessionManager;
    this.threadManager = threadManager;
    this.telemetryManager = telemetryManager;
    this.workflowManager = new WorkflowManager(this, telemetryManager);

    this.initializeDefaultAgent();
  }

  /**
   * Register a new agent
   */
  registerAgent(agent: CoralAgent): string {
    this.agents.set(agent.id, agent);

    // Record registration event
    this.telemetryManager.recordEvent('agent_registered', agent.id, {
      name: agent.name,
      version: agent.version,
      capabilities: agent.capabilities
    });

    console.log(`🤖 Registered agent: ${agent.name} (${agent.id})`);
    return agent.id;
  }

  /**
   * Get agent by ID
   */
  getAgent(agentId: string): CoralAgent | undefined {
    return this.agents.get(agentId);
  }

  /**
   * List all agents
   */
  listAgents(): CoralAgent[] {
    return Array.from(this.agents.values());
  }

  /**
   * Find agents by capability
   */
  findAgentsByCapability(capability: string): CoralAgent[] {
    return Array.from(this.agents.values()).filter(agent =>
      agent.capabilities.includes(capability)
    );
  }

  /**
   * Update agent capabilities
   */
  updateAgentCapabilities(agentId: string, capabilities: string[]): boolean {
    const agent = this.agents.get(agentId);
    if (agent) {
      agent.capabilities = capabilities;

      this.telemetryManager.recordEvent('agent_updated', agentId, {
        updatedCapabilities: capabilities
      });

      return true;
    }
    return false;
  }

  /**
   * Create session for agent
   */
  createSessionForAgent(agentId: string, clientId?: string): string | null {
    const agent = this.agents.get(agentId);
    if (!agent) {
      return null;
    }

    const session = this.sessionManager.createSession(agentId, clientId);

    this.telemetryManager.recordEvent('session_created', agentId, {
      sessionId: session.id,
      clientId
    });

    return session.id;
  }

  /**
   * Execute agent with session context (Coral Protocol compliant)
   */
  async executeAgent(
    agentId: string,
    sessionId: string,
    input: any,
    metadata?: Record<string, any>
  ): Promise<any> {
    const agent = this.agents.get(agentId);
    if (!agent) {
      throw new Error(`Agent ${agentId} not found`);
    }

    const session = this.sessionManager.getSession(sessionId);
    if (!session) {
      throw new Error(`Session ${sessionId} not found`);
    }

    // Update session activity
    this.sessionManager.updateSessionActivity(sessionId);

    // Record execution event
    this.telemetryManager.recordEvent('agent_execution', agentId, {
      sessionId,
      inputType: typeof input,
      metadata,
      workflowId: metadata?.workflowId,
      taskId: metadata?.taskId
    });

    try {
      // Coral Protocol v1.0: Execute agent workflow
      const workflowResult = await this.executeAgentWorkflow(agent, session, input, metadata);

      // Record successful execution with verification
      this.telemetryManager.recordEvent('agent_execution_success', agentId, {
        sessionId,
        executionTime: Date.now() - session.lastActivity.getTime(),
        workflowId: metadata?.workflowId,
        resultHash: this.generateResultHash(workflowResult),
        delegatedTasks: workflowResult.delegatedTasks || 0
      });

      return workflowResult;

    } catch (error: any) {
      // Record execution error
      this.telemetryManager.recordEvent('agent_execution_error', agentId, {
        sessionId,
        error: error.message,
        workflowId: metadata?.workflowId
      });

      throw error;
    }
  }

  /**
   * Execute agent workflow (Coral Protocol compliant)
   */
  private async executeAgentWorkflow(
    agent: CoralAgent,
    session: any,
    input: any,
    metadata?: Record<string, any>
  ): Promise<any> {
    const startTime = Date.now();

    // Coral Protocol: Agent decision making
    const workflow = this.analyzeInputAndCreateWorkflow(agent, input, metadata);

    // Execute primary task
    let primaryResult = await this.executePrimaryTask(agent, workflow.primaryTask, input);

    // Handle task delegation if needed
    const delegatedResults = await this.handleTaskDelegation(agent, workflow.delegatedTasks, session);

    // Coral Protocol: Generate verifiable result
    const finalResult = {
      primary: primaryResult,
      delegated: delegatedResults,
      workflow: {
        id: workflow.id,
        steps: workflow.steps,
        decisionPath: workflow.decisionPath,
        confidence: workflow.confidence
      },
      metadata: {
        executionTime: Date.now() - startTime,
        agentId: agent.id,
        sessionId: session.id,
        verifiable: true,
        timestamp: new Date().toISOString()
      }
    };

    return finalResult;
  }

  /**
   * Analyze input and create workflow (Coral Protocol agent logic)
   */
  private analyzeInputAndCreateWorkflow(
    agent: CoralAgent,
    input: any,
    metadata?: Record<string, any>
  ) {
    const workflowId = crypto.randomUUID();

    // Basic workflow analysis based on agent capabilities
    const steps: string[] = [];
    const decisionPath: string[] = [];
    let confidence = 0.8; // Default confidence

    // Determine workflow based on input and agent capabilities
    if (input.symbol && agent.capabilities.includes('price-analysis')) {
      steps.push('price_analysis', 'market_context', 'risk_assessment');
      decisionPath.push('symbol_detected', 'price_workflow_selected');
    } else if (input.analysis_type && agent.capabilities.includes('market-analysis')) {
      steps.push('data_collection', 'ai_analysis', 'signal_generation');
      decisionPath.push('analysis_requested', 'ai_workflow_selected');
    } else {
      steps.push('general_query', 'response_generation');
      decisionPath.push('fallback_workflow');
      confidence = 0.6; // Lower confidence for general queries
    }

    return {
      id: workflowId,
      primaryTask: steps[0],
      steps,
      decisionPath,
      confidence,
      delegatedTasks: []
    };
  }

  /**
   * Execute primary task
   */
  private async executePrimaryTask(agent: CoralAgent, task: string, input: any): Promise<any> {
    // Route to appropriate task executor based on agent capabilities
    if (task === 'price_analysis' && agent.capabilities.includes('price-analysis')) {
      return await this.simulatePriceAnalysis(input);
    } else if (task === 'ai_analysis' && agent.capabilities.includes('market-analysis')) {
      return await this.simulateMarketAnalysis(input);
    } else if (task === 'oracle_feeds' && agent.capabilities.includes('oracle-feeds')) {
      return await this.simulateOracleFeed(input);
    }

    return { task, status: 'completed', result: 'Task executed successfully' };
  }

  /**
   * Handle task delegation (Coral Protocol teamwork feature)
   */
  private async handleTaskDelegation(
    agent: CoralAgent,
    delegatedTasks: any[],
    session: any
  ): Promise<any[]> {
    const results: any[] = [];

    // For now, simulate delegation - in real implementation, this would
    // involve calling other agents through the Coral Protocol
    for (const task of delegatedTasks) {
      // Simulate delegated task execution
      results.push({
        taskId: task.id,
        delegatedTo: task.agentId || 'external-agent',
        status: 'completed',
        result: `Delegated task ${task.description} completed`,
        timestamp: new Date().toISOString()
      });
    }

    return results;
  }

  /**
   * Generate result hash for verification (Coral Protocol feature)
   */
  private generateResultHash(result: any): string {
    const hashInput = JSON.stringify(result);
    return crypto.createHash('sha256').update(hashInput).digest('hex');
  }

  /**
   * Get agent statistics
   */
  getAgentStats(agentId: string): {
    agent: CoralAgent;
    sessions: number;
    threads: number;
    executions: number;
    averageExecutionTime: number;
  } | null {
    const agent = this.agents.get(agentId);
    if (!agent) {
      return null;
    }

    const agentSessions = this.sessionManager.getAgentSessions(agentId);
    const agentThreads = agentSessions.flatMap(s =>
      this.threadManager.getSessionThreads(s.id)
    );

    const executionEvents = this.telemetryManager.getEventsForAgent(agentId)
      .filter(e => e.event === 'agent_execution');

    const executionTimes = executionEvents
      .map(e => e.data.executionTime || 0)
      .filter(time => time > 0);

    return {
      agent,
      sessions: agentSessions.length,
      threads: agentThreads.length,
      executions: executionEvents.length,
      averageExecutionTime: executionTimes.length > 0 ?
        executionTimes.reduce((a, b) => a + b, 0) / executionTimes.length : 0
    };
  }

  /**
   * Create workflow for agent
   */
  createWorkflowForAgent(
    agentId: string,
    workflowType: 'price-analysis' | 'market-analysis',
    parameters: any
  ): Workflow | null {
    const agent = this.agents.get(agentId);
    if (!agent) {
      return null;
    }

    // Record workflow creation
    this.telemetryManager.recordEvent('workflow_creation_requested', agentId, {
      workflowType,
      parameters
    });

    switch (workflowType) {
      case 'price-analysis':
        return this.workflowManager.createPriceAnalysisWorkflow(
          agentId,
          parameters.symbol
        );
      case 'market-analysis':
        return this.workflowManager.createMarketAnalysisWorkflow(
          agentId,
          parameters.symbol,
          parameters.analysisType
        );
      default:
        return null;
    }
  }

  /**
   * Execute workflow for agent
   */
  async executeWorkflowForAgent(
    agentId: string,
    workflowId: string,
    sessionId?: string
  ): Promise<Workflow | null> {
    const agent = this.agents.get(agentId);
    if (!agent) {
      return null;
    }

    // Record workflow execution request
    this.telemetryManager.recordEvent('workflow_execution_requested', agentId, {
      workflowId,
      sessionId
    });

    return await this.workflowManager.executeWorkflow(workflowId, sessionId);
  }

  /**
   * Get workflows for agent
   */
  getAgentWorkflows(agentId: string, status?: string): Workflow[] {
    return this.workflowManager.listWorkflows(status).filter(
      workflow => workflow.initiator === agentId
    );
  }

  /**
   * Get workflow by ID
   */
  getWorkflow(workflowId: string): Workflow | undefined {
    return this.workflowManager.getWorkflow(workflowId);
  }

  /**
   * Get overall system statistics
   */
  getSystemStats(): {
    totalAgents: number;
    totalSessions: number;
    totalThreads: number;
    totalExecutions: number;
    activeAgents: number;
    totalWorkflows: number;
    activeWorkflows: number;
  } {
    const agents = Array.from(this.agents.values());
    const sessions = this.sessionManager.getActiveSessions();
    const allThreads = Array.from(this.threadManager['threads'].values());
    const allExecutions = this.telemetryManager.getEventsByType('agent_execution');
    const workflowStats = this.workflowManager.getWorkflowStats();

    return {
      totalAgents: agents.length,
      totalSessions: sessions.length,
      totalThreads: allThreads.length,
      totalExecutions: allExecutions.length,
      activeAgents: agents.filter(agent => {
        const stats = this.getAgentStats(agent.id);
        return stats && stats.executions > 0;
      }).length,
      totalWorkflows: workflowStats.total,
      activeWorkflows: workflowStats.active
    };
  }

  /**
   * Initialize default IORA agent
   */
  private initializeDefaultAgent(): void {
    const ioraAgent: CoralAgent = {
      id: 'iora-mcp-agent',
      name: 'IORA MCP Agent',
      description: 'Intelligent Oracle Rust Assistant - AI-powered cryptocurrency oracle with blockchain feeds',
      version: '1.0.0',
      capabilities: [
        'price-analysis',
        'market-analysis',
        'oracle-feeds',
        'blockchain-integration',
        'ai-analysis',
        'nft-receipts'
      ],
      pricing: {
        perRequest: 0.001, // $0.001 per request
        subscription: {
          monthly: 9.99,
          yearly: 99.99
        }
      },
      metadata: {
        type: 'mcp',
        protocol: '1.0.0',
        features: [
          'real-time-pricing',
          'multi-api-aggregation',
          'solana-integration',
          'crossmint-nft-receipts'
        ]
      }
    };

    this.registerAgent(ioraAgent);
  }

  // Simulation methods for agent capabilities
  private async simulatePriceAnalysis(input: any): Promise<any> {
    // Simulate price analysis
    return {
      symbol: input.symbol || 'BTC',
      price: 45000 + Math.random() * 1000,
      confidence: 0.95,
      timestamp: new Date().toISOString()
    };
  }

  private async simulateMarketAnalysis(input: any): Promise<any> {
    // Simulate market analysis
    return {
      symbol: input.symbol || 'BTC',
      analysis: 'Market shows bullish momentum with strong support levels',
      recommendation: 'BUY',
      confidence: 0.87,
      timestamp: new Date().toISOString()
    };
  }

  private async simulateOracleFeed(input: any): Promise<any> {
    // Simulate oracle feed
    return {
      symbol: input.symbol || 'BTC',
      price: 45000 + Math.random() * 1000,
      transactionSignature: 'simulated_' + crypto.randomBytes(32).toString('hex'),
      slot: Math.floor(Math.random() * 1000000),
      status: 'confirmed',
      timestamp: new Date().toISOString()
    };
  }
}
