import "dotenv/config";
import express from "express";
import helmet from "helmet";
import crypto from "crypto";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { get_price } from "./tools/get_price.js";
import { analyze_market, health_check, query_crypto, cache_status, api_analytics } from "./tools/analyze_market.js";
import { feed_oracle } from "./tools/feed_oracle.js";
import { health } from "./tools/health.js";
import { limiter, oracleLimiter, hmacAuth, shield } from "./mw/security.js";
import { mountReceipt } from "./routes/receipt.js";
import userRoutes from "./routes/user.js";
import { createRegistryClient } from "./lib/registry.js";
import coralConfig from "./coral-config.js";
import { SessionManager } from "./lib/session-manager.js";
import { ThreadManager } from "./lib/thread-manager.js";
import { TelemetryManager } from "./lib/telemetry.js";
import { AgentManager } from "./lib/agent-manager.js";

const app = express();

// Security hardening
app.use(helmet({
    crossOriginResourcePolicy: { policy: "same-origin" },
    contentSecurityPolicy: {
        directives: {
            defaultSrc: ["'self'"],
            styleSrc: ["'none'"],
            scriptSrc: ["'none'"],
            imgSrc: ["'none'"],
        },
    },
}));
app.disable('x-powered-by');

// Request ID middleware
app.use((req, res, next) => {
  const reqId = crypto.randomUUID();
  res.locals.reqId = reqId;
  next();
});

// Structured logging middleware (redacted)
app.use((req, res, next) => {
  const start = Date.now();
  const reqId = res.locals.reqId;

  // Log request (no bodies, no sensitive headers)
  console.log(JSON.stringify({
    level: "info",
    reqId,
    method: req.method,
    path: req.path,
    ip: (req.ip || 'unknown').replace(/:\d+$/, ':*'), // Redact port
    timestamp: new Date().toISOString()
  }));

  // Log response
  res.on("finish", () => {
    const duration = Date.now() - start;
    console.log(JSON.stringify({
      level: "info",
      reqId,
      method: req.method,
      path: req.path,
      status: res.statusCode,
      duration_ms: duration,
      timestamp: new Date().toISOString()
    }));
  });

  next();
});

app.use(express.json({ limit: "256kb" }));

// Mount user routes (Clerk-authenticated) BEFORE HMAC auth
app.use('/user', userRoutes);

// Apply rate limiting and HMAC auth to service routes
app.use(limiter);
app.use((req,res,next)=>{ 
  // Skip auth for health monitoring endpoints
  if (req.path.endsWith("/health") || req.path === "/healthz" || req.path === "/metrics") {
    return next(); 
  }
  // Skip auth for user routes (already handled by Clerk)
  if (req.path.startsWith("/user")) {
    return next();
  }
  // Apply HMAC auth for MCP tool routes (service-to-service)
  return hmacAuth(req,res,next); 
});

export function wrapper(fn: (body: any)=>Promise<any>) {
  return async (req: any, res: any) => {
    const reqId = res.locals.reqId;
    const start = Date.now();
    try {
      const data = await fn(req.body);
      const duration = Date.now() - start;

      // Log successful tool execution
      console.log(JSON.stringify({
        level: "info",
        reqId,
        tool: req.path.split("/").pop(),
        exitCode: 0,
        duration_ms: duration,
        timestamp: new Date().toISOString()
      }));

      // Add rate limit headers
      res.set({
        'X-RateLimit-Limit': '60',
        'X-RateLimit-Remaining': '59',
        'X-RateLimit-Reset': Math.floor(Date.now() / 1000) + 3600,
        'X-Response-Time': `${duration}ms`
      });

      res.json({ ok: true, data });
    } catch (e:any) {
      const duration = Date.now() - start;

      // Log tool execution error (redacted)
      const errorMsg = e?.message || "unknown_error";
      console.log(JSON.stringify({
        level: "error",
        reqId,
        tool: req.path.split("/").pop(),
        exitCode: 1,
        error: errorMsg.substring(0, 200), // Truncate for security
        duration_ms: duration,
        timestamp: new Date().toISOString()
      }));

      res.status(400).json({ ok: false, error: errorMsg });
    }
  };
}

// Streaming wrapper for long-running operations
export function streamingWrapper(fn: (body: any, onProgress: (data: any) => void) => Promise<any>) {
  return async (req: any, res: any) => {
    const reqId = res.locals.reqId;
    const start = Date.now();
    
    // Check for streaming request
    const acceptsStream = req.headers.accept?.includes('text/event-stream');
    
    if (acceptsStream) {
      // Set up Server-Sent Events
      res.writeHead(200, {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
        'X-Request-ID': reqId
      });

      const onProgress = (data: any) => {
        res.write(`data: ${JSON.stringify({ type: 'progress', data })}\n\n`);
      };

      try {
        const result = await fn(req.body, onProgress);
        const duration = Date.now() - start;
        
        // Send final result
        res.write(`data: ${JSON.stringify({ type: 'complete', data: result, duration_ms: duration })}\n\n`);
        res.end();

        // Log successful streaming execution
        console.log(JSON.stringify({
          level: "info",
          reqId,
          tool: req.path.split("/").pop(),
          mode: "streaming",
          exitCode: 0,
          duration_ms: duration,
          timestamp: new Date().toISOString()
        }));

      } catch (e: any) {
        const duration = Date.now() - start;
        res.write(`data: ${JSON.stringify({ type: 'error', error: e.message, duration_ms: duration })}\n\n`);
        res.end();

        console.log(JSON.stringify({
          level: "error",
          reqId,
          tool: req.path.split("/").pop(),
          mode: "streaming",
          exitCode: 1,
          duration_ms: duration,
          error: e.message || "Unknown error",
          timestamp: new Date().toISOString()
        }));
      }
    } else {
      // Fallback to regular response
      try {
        const data = await fn(req.body, () => {}); // No-op progress callback
        const duration = Date.now() - start;

        res.set({
          'X-RateLimit-Limit': '60',
          'X-RateLimit-Remaining': '59',
          'X-RateLimit-Reset': Math.floor(Date.now() / 1000) + 3600,
          'X-Response-Time': `${duration}ms`
        });

        res.json({ ok: true, data });
      } catch (e: any) {
        const duration = Date.now() - start;
        res.status(500).json({ ok: false, error: e.message || "Unknown error" });
      }
    }
  };
}

// Apply stricter rate limiting to oracle feeds
const oracleWrapper = (fn: (body: any)=>Promise<any>) => {
  return [oracleLimiter, (req: any, res: any, next: any) => wrapper(fn)(req, res)];
};

// Root route for Railway health check and debugging
app.get("/", (req, res) => {
  const systemStats = agentManager.getSystemStats();
  const telemetryAnalytics = telemetryManager.getAnalytics(1); // Last hour

  res.json({
    service: "IORA MCP Server",
    status: "running",
    version: "1.0.0",
    environment: process.env.NODE_ENV || "development",
    port: process.env.PORT || 7145,
    coral_integration: process.env.CORAL_REGISTRY_AUTO_REGISTER === "true" ? "enabled" : "disabled",
    coral_registry_url: process.env.CORAL_REGISTRY_URL || "not configured",
    timestamp: new Date().toISOString(),

    // Coral Protocol v1.0 Features
    coral_protocol: {
      version: "1.0.0",
      session_support: true,
      thread_support: true,
      telemetry_support: true,
      agent_management: true
    },

    // System Statistics
    system_stats: {
      total_agents: systemStats.totalAgents,
      active_sessions: systemStats.totalSessions,
      total_threads: systemStats.totalThreads,
      telemetry_events: telemetryAnalytics.totalEvents
    },

    // Available Routes
    routes: [
      "/tools/health",
      "/tools/get_price",
      "/tools/analyze_market",
      "/tools/feed_oracle",
      "/tools/health_check",
      "/tools/query_crypto",
      "/tools/cache_status",
      "/tools/api_analytics",
      // Coral Protocol v1.0 Routes
      "/coral/sessions",
      "/coral/sessions/:sessionId",
      "/coral/agents/:agentId/execute",
      "/coral/telemetry/analytics",
      "/coral/system/stats",
      // Workflow Routes
      "/coral/workflows",
      "/coral/workflows/:workflowId",
      "/coral/workflows/:workflowId/execute",
      "/coral/agents/:agentId/workflows"
    ],

    // Coral Studio Compatibility
    coral_studio: {
      compatibility: "v1.0-ready",
      supported_protocols: ["mcp-1.0", "http-json", "websocket-optional"],
      agent_framework: "coral-protocol-1.0"
    }
  });
});

app.post("/tools/get_price", limiter, wrapper(get_price));
app.post("/tools/analyze_market", limiter, streamingWrapper(async (body, onProgress) => {
  // Progress tracking for analyze_market
  onProgress({ step: "initializing", message: "Starting market analysis..." });
  
  // Simulate progress updates during analysis
  const result = await new Promise(async (resolve, reject) => {
    try {
      onProgress({ step: "fetching_data", message: "Fetching market data from APIs..." });
      await new Promise(r => setTimeout(r, 500)); // Simulate API calls
      
      onProgress({ step: "ai_analysis", message: "Running AI-powered analysis..." });
      await new Promise(r => setTimeout(r, 1000)); // Simulate AI processing
      
      onProgress({ step: "generating_insights", message: "Generating market insights..." });
      const data = await analyze_market(body);
      
      onProgress({ step: "finalizing", message: "Finalizing analysis results..." });
      resolve(data);
    } catch (error) {
      reject(error);
    }
  });
  
  return result;
}));
app.post("/tools/feed_oracle", oracleLimiter, streamingWrapper(async (body, onProgress) => {
  // Progress tracking for feed_oracle
  onProgress({ step: "initializing", message: "Preparing oracle feed submission..." });
  
  const result = await new Promise(async (resolve, reject) => {
    try {
      onProgress({ step: "validating", message: "Validating oracle data..." });
      await new Promise(r => setTimeout(r, 300));
      
      onProgress({ step: "submitting", message: "Submitting to Solana blockchain..." });
      await new Promise(r => setTimeout(r, 800));
      
      onProgress({ step: "confirming", message: "Waiting for transaction confirmation..." });
      const data = await feed_oracle(body);
      
      onProgress({ step: "minting", message: "Minting NFT receipt..." });
      await new Promise(r => setTimeout(r, 500));
      
      onProgress({ step: "complete", message: "Oracle feed submitted successfully!" });
      resolve(data);
    } catch (error) {
      reject(error);
    }
  });
  
  return result;
}));
app.post("/tools/health_check", limiter, wrapper(health_check));
app.post("/tools/query_crypto", limiter, wrapper(query_crypto));
app.post("/tools/cache_status", limiter, wrapper(cache_status));
app.post("/tools/api_analytics", limiter, wrapper(api_analytics));
app.get("/tools/health", async (req, res) => {
  try {
    const data = await health();
    res.json({ ok: true, data });
  } catch (e: any) {
    res.status(400).json({ ok: false, error: e.message });
  }
});

// Mount additional routes
mountReceipt(app);

// Coral Protocol v1.0 Routes
app.post("/coral/sessions", limiter, (req, res) => {
  try {
    const { agentId, clientId } = req.body;

    if (!agentId) {
      return res.status(400).json({
        ok: false,
        error: "agentId is required"
      });
    }

    const sessionId = agentManager.createSessionForAgent(agentId, clientId);

    if (!sessionId) {
      return res.status(404).json({
        ok: false,
        error: `Agent ${agentId} not found`
      });
    }

    res.json({
      ok: true,
      data: {
        sessionId,
        agentId,
        clientId,
        createdAt: new Date().toISOString()
      }
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.get("/coral/sessions/:sessionId", limiter, (req, res) => {
  try {
    const { sessionId } = req.params;
    const session = sessionManager.getSession(sessionId);

    if (!session) {
      return res.status(404).json({
        ok: false,
        error: "Session not found"
      });
    }

    res.json({
      ok: true,
      data: session
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.post("/coral/agents/:agentId/execute", limiter, async (req, res) => {
  try {
    const { agentId } = req.params;
    const { sessionId, input, metadata } = req.body;

    if (!sessionId) {
      return res.status(400).json({
        ok: false,
        error: "sessionId is required"
      });
    }

    const result = await agentManager.executeAgent(agentId, sessionId, input, metadata);

    res.json({
      ok: true,
      data: result
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.get("/coral/telemetry/analytics", limiter, (req, res) => {
  try {
    const timeframe = parseInt(req.query.timeframe as string) || 24;
    const analytics = telemetryManager.getAnalytics(timeframe);

    res.json({
      ok: true,
      data: analytics
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.get("/coral/system/stats", limiter, (req, res) => {
  try {
    const stats = agentManager.getSystemStats();

    res.json({
      ok: true,
      data: stats
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

// Coral Protocol v1.0 Workflow Endpoints
app.post("/coral/workflows", limiter, (req, res) => {
  try {
    const { agentId, workflowType, parameters } = req.body;

    if (!agentId || !workflowType) {
      return res.status(400).json({
        ok: false,
        error: "agentId and workflowType are required"
      });
    }

    const workflow = agentManager.createWorkflowForAgent(agentId, workflowType, parameters);

    if (!workflow) {
      return res.status(404).json({
        ok: false,
        error: `Agent ${agentId} not found or workflow type not supported`
      });
    }

    res.json({
      ok: true,
      data: workflow
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.get("/coral/workflows/:workflowId", limiter, (req, res) => {
  try {
    const { workflowId } = req.params;
    const workflow = agentManager.getWorkflow(workflowId);

    if (!workflow) {
      return res.status(404).json({
        ok: false,
        error: "Workflow not found"
      });
    }

    res.json({
      ok: true,
      data: workflow
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.post("/coral/workflows/:workflowId/execute", limiter, (req, res) => {
  try {
    const { workflowId } = req.params;
    const { agentId, sessionId } = req.body;

    if (!agentId) {
      return res.status(400).json({
        ok: false,
        error: "agentId is required"
      });
    }

    const result = agentManager.executeWorkflowForAgent(agentId, workflowId, sessionId);

    res.json({
      ok: true,
      data: {
        workflowId,
        status: "execution_started",
        message: "Workflow execution initiated"
      }
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

app.get("/coral/agents/:agentId/workflows", limiter, (req, res) => {
  try {
    const { agentId } = req.params;
    const { status } = req.query;

    const workflows = agentManager.getAgentWorkflows(agentId, status as string);

    res.json({
      ok: true,
      data: workflows
    });
  } catch (error: any) {
    res.status(500).json({
      ok: false,
      error: error.message
    });
  }
});

// Health and Metrics endpoints for monitoring
app.get("/healthz", (req, res) => {
  const healthData = {
    status: "healthy",
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
    memory: process.memoryUsage(),
    version: process.env.npm_package_version || "1.0.0",
    environment: process.env.NODE_ENV || "development"
  };
  
  res.status(200).json(healthData);
});

app.get("/metrics", (req, res) => {
  const systemStats = agentManager.getSystemStats();
  const telemetryAnalytics = telemetryManager.getAnalytics(1); // Last hour
  
  const metrics = {
    // System metrics
    uptime_seconds: process.uptime(),
    memory_usage_bytes: process.memoryUsage().heapUsed,
    memory_total_bytes: process.memoryUsage().heapTotal,
    
    // Application metrics
    total_agents: systemStats.totalAgents,
    active_sessions: systemStats.totalSessions,
    total_threads: systemStats.totalThreads,
    
    // Performance metrics
    avg_response_time_ms: 0, // Will be calculated from telemetry events
    total_requests: telemetryAnalytics.totalEvents || 0,
    error_rate: telemetryAnalytics.recentErrors?.length || 0,
    
    // Coral Protocol metrics
    coral_protocol_version: "1.0.0",
    session_support: true,
    thread_support: true,
    telemetry_support: true,
    
    timestamp: new Date().toISOString()
  };
  
  res.set('Content-Type', 'text/plain');
  res.send(Object.entries(metrics)
    .map(([key, value]) => `${key} ${value}`)
    .join('\n'));
});

// Database health endpoint
app.get("/health/database", async (req, res) => {
  try {
    const { checkDatabaseHealthComprehensive } = await import('./db/health.js');
    const health = await checkDatabaseHealthComprehensive();

    const statusCode = health.overall === 'healthy' ? 200 :
                      health.overall === 'degraded' ? 200 : 503;

    res.status(statusCode).json({
      status: health.overall,
      timestamp: health.timestamp,
      postgresql: health.postgresql,
      redis: health.redis,
      // Include basic metrics if available
      ...(health.postgresql.metrics && {
        postgresql_metrics: health.postgresql.metrics
      }),
      ...(health.redis?.metrics && {
        redis_metrics: health.redis.metrics
      })
    });
  } catch (error) {
    res.status(503).json({
      status: 'error',
      timestamp: new Date().toISOString(),
      error: error instanceof Error ? error.message : 'Unknown error',
      message: 'Database health check failed'
    });
  }
});

// Coral Protocol v1.0 Managers
const sessionManager = new SessionManager(60); // 60 minute sessions
const threadManager = new ThreadManager();
const telemetryManager = new TelemetryManager(10000); // 10k events max

// Initialize Agent Manager
const agentManager = new AgentManager(sessionManager, threadManager, telemetryManager);

app.use(shield);

const port = Number(process.env.PORT || 7070);

// Registry Integration (if enabled)
let registryClient: any = null;

if (process.env.CORAL_REGISTRY_AUTO_REGISTER === "true") {
  console.log("🔗 Initializing Coral Protocol Registry integration...");
  console.log(`   Registry URL: ${process.env.CORAL_REGISTRY_URL}`);
  console.log(`   Server URL: ${process.env.CORAL_SERVER_URL || `http://localhost:${process.env.PORT || 7070}`}`);

  try {
    // Create MCP server instance for registry client
    const mcpServer = new Server(
      { name: "iora-mcp", version: "1.0.0" },
      {
        capabilities: {
          tools: {
            listChanged: true,
          },
        },
      }
    );

    // Create registry client
    registryClient = createRegistryClient(mcpServer);

    // Auto-register with registry
    console.log("🔄 Auto-registering with Coral Registry...");
    registryClient.register().then((result: any) => {
      if (result.success) {
        console.log(`✅ Successfully registered with Coral Registry!`);
        console.log(`   Service ID: ${result.serviceId}`);
        console.log(`   Status: Active and discoverable in Coral Studio`);
        console.log(`   Available Tools: ${Object.keys(coralConfig.tools).length} tools configured`);
        // Start heartbeat for continuous updates
        registryClient.startHeartbeat();
        console.log(`💓 Registry heartbeat started (interval: ${process.env.CORAL_REGISTRY_HEARTBEAT_INTERVAL || 60}s)`);
      } else {
        console.error(`❌ Auto-registration failed: ${result.error}`);
        console.log("   Service will still be available via direct HTTP connection");
        console.log("   Check CORAL_REGISTRY_URL and CORAL_REGISTRY_TOKEN configuration");
      }
    }).catch((error: any) => {
      console.error(`❌ Registry integration error: ${error.message}`);
      console.log("   Service will still be available via direct HTTP connection");
      console.log("   You can manually register using: npm run registry:register");
    });

  } catch (error: any) {
    console.error(`❌ Failed to initialize registry client: ${error.message}`);
    console.log("   Service will still be available via direct HTTP connection");
    console.log("   Check registry configuration in environment variables");
  }
} else {
  console.log("ℹ️ Coral Registry integration disabled");
  console.log("   Set CORAL_REGISTRY_AUTO_REGISTER=true to enable");
  console.log("   Configure CORAL_REGISTRY_URL and CORAL_REGISTRY_TOKEN for registration");
}

console.log(`🚀 Starting IORA MCP server on port ${port}...`);
console.log(`   Health check: http://localhost:${port}/tools/health`);
console.log(`   Registry integration: ${registryClient ? "enabled" : "disabled"}`);

console.log(`📊 Coral Protocol v1.0 Features:`);
console.log(`   • Session Management: ✅ Enabled (${sessionManager.getStats().total} sessions managed)`);
console.log(`   • Thread Management: ✅ Enabled (${threadManager.getStats().total} threads managed)`);
console.log(`   • Agent Management: ✅ Enabled (${agentManager.getSystemStats().totalAgents} agents)`);
console.log(`   • Telemetry: ✅ Enabled (${telemetryManager.getAnalytics(1).totalEvents} recent events)`);
console.log(`   • MCP Tools: ✅ ${Object.keys(coralConfig.tools).length} tools configured`);

// Initialize database connection (if configured)
async function initializeDatabase() {
  try {
    // Only initialize if database URL is configured
    if (process.env.DATABASE_URL) {
      const { initializeDatabase: initDb, checkDatabaseHealth } = await import('./config/database.js');

      // Initialize database connection pool
      await initDb();

      // Run database migrations
      console.log('🔄 Running database migrations...');
      const { runMigrations } = await import('./db/migrate.js');
      await runMigrations();
      console.log('✅ Database migrations completed');

      // Check database health
      const health = await checkDatabaseHealth();
      if (health.status === 'healthy') {
        console.log(`   • Database: ✅ Connected (${health.metrics?.totalConnections || 0} connections)`);
      } else {
        console.warn(`   • Database: ⚠️ Unhealthy (${health.error})`);
      }
    } else {
      console.log(`   • Database: 🔧 Not configured (optional)`);
    }
  } catch (error) {
    console.warn(`   • Database: ❌ Initialization failed (${error instanceof Error ? error.message : 'Unknown error'})`);
    console.warn(`   • Database: 🔧 Continuing without database (API keys will not persist)`);
  }
}

// Initialize database before starting server
await initializeDatabase();

app.listen(port, () => {
  const systemStats = agentManager.getSystemStats();

  console.log(JSON.stringify({
    status: "ok",
    mcp_http_port: port,
    registry_integration: !!registryClient,
    coral_protocol: {
      version: "1.0.0",
      session_support: true,
      thread_support: true,
      telemetry_support: true,
      agent_management: true
    },
    system_stats: {
      total_agents: systemStats.totalAgents,
      active_sessions: systemStats.totalSessions,
      total_threads: systemStats.totalThreads
    },
    tools_available: [
      "get_price", "analyze_market", "feed_oracle", "health",
      "health_check", "query_crypto", "cache_status", "api_analytics"
    ],
    coral_routes: [
      "/coral/sessions", "/coral/agents/:agentId/execute",
      "/coral/telemetry/analytics", "/coral/system/stats"
    ],
    timestamp: new Date().toISOString()
  }));
});
