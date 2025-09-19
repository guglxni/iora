import "dotenv/config";
import express from "express";
import helmet from "helmet";
import crypto from "crypto";
import { get_price } from "./tools/get_price.js";
import { analyze_market } from "./tools/analyze_market.js";
import { feed_oracle } from "./tools/feed_oracle.js";
import { health } from "./tools/health.js";
import { limiter, oracleLimiter, hmacAuth, shield } from "./mw/security.js";
import { mountReceipt } from "./routes/receipt.js";

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

app.post("/tools/get_price", limiter, wrapper(get_price));
app.post("/tools/analyze_market", limiter, wrapper(analyze_market));
app.post("/tools/feed_oracle", ...oracleWrapper(feed_oracle));
app.get("/tools/health", wrapper(async ()=> await health()));

// Mount additional routes
mountReceipt(app);

app.use(shield);

const port = Number(process.env.PORT || 7070);
app.listen(port, () => {
  console.log(JSON.stringify({ status: "ok", mcp_http_port: port }));
});
