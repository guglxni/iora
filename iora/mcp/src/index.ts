import "dotenv/config";
import express from "express";
import helmet from "helmet";
import crypto from "crypto";
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { get_price } from "./tools/get_price.js";
import { analyze_market } from "./tools/analyze_market.js";
import { feed_oracle } from "./tools/feed_oracle.js";
import { health } from "./tools/health.js";
import { limiter, oracleLimiter, hmacAuth, shield } from "./mw/security.js";
import { mountReceipt } from "./routes/receipt.js";
import { createRegistryClient } from "./lib/registry.js";

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
app.use(limiter);
app.use((req,res,next)=>{ if (req.path.endsWith("/health")) return next(); return hmacAuth(req,res,next); });

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

// Apply stricter rate limiting to oracle feeds
const oracleWrapper = (fn: (body: any)=>Promise<any>) => {
  return [oracleLimiter, (req: any, res: any, next: any) => wrapper(fn)(req, res)];
};

// Root route for Railway health check and debugging
app.get("/", (req, res) => {
  res.json({
    service: "IORA MCP Server",
    status: "running",
    version: "1.0.0",
    environment: process.env.NODE_ENV || "development",
    port: process.env.PORT || 7145,
    timestamp: new Date().toISOString(),
    routes: ["/tools/health", "/tools/get_price", "/tools/analyze_market", "/tools/feed_oracle"]
  });
});

app.post("/tools/get_price", limiter, wrapper(get_price));
app.post("/tools/analyze_market", limiter, wrapper(analyze_market));
app.post("/tools/feed_oracle", ...oracleWrapper(feed_oracle));
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

app.use(shield);

const port = Number(process.env.PORT || 7070);

// Registry Integration (if enabled)
let registryClient: any = null;

if (process.env.CORAL_REGISTRY_AUTO_REGISTER === "true") {
  console.log("🔗 Initializing Coral Registry integration...");

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
        console.log(`✅ Auto-registered with Coral Registry (ID: ${result.serviceId})`);
        // Start heartbeat for continuous updates
        registryClient.startHeartbeat();
      } else {
        console.error(`❌ Auto-registration failed: ${result.error}`);
        console.log("   Service will still be available via direct HTTP connection");
      }
    }).catch((error: any) => {
      console.error(`❌ Registry integration error: ${error.message}`);
      console.log("   Service will still be available via direct HTTP connection");
    });

  } catch (error: any) {
    console.error(`❌ Failed to initialize registry client: ${error.message}`);
    console.log("   Service will still be available via direct HTTP connection");
  }
}

console.log(`🚀 Starting IORA MCP server on port ${port}...`);
console.log(`   Health check: http://localhost:${port}/tools/health`);
console.log(`   Registry integration: ${registryClient ? "enabled" : "disabled"}`);

app.listen(port, () => {
  console.log(JSON.stringify({
    status: "ok",
    mcp_http_port: port,
    registry_integration: !!registryClient,
    tools_available: ["get_price", "analyze_market", "feed_oracle", "health"],
    timestamp: new Date().toISOString()
  }));
});
