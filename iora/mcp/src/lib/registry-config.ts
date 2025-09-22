// Registry configuration and authentication
export interface RegistryAuth {
  token?: string;
  apiKey?: string;
  username?: string;
  password?: string;
}

export interface RegistrySettings {
  url: string;
  auth?: RegistryAuth;
  autoRegister: boolean;
  heartbeatInterval: number; // seconds
  retryAttempts: number;
  timeout: number; // milliseconds
}

// Load registry settings from environment
export function loadRegistrySettings(): RegistrySettings {
  return {
    url: process.env.CORAL_REGISTRY_URL || "http://localhost:8080",
    auth: {
      token: process.env.CORAL_REGISTRY_TOKEN,
      apiKey: process.env.CORAL_REGISTRY_API_KEY,
    },
    autoRegister: process.env.CORAL_REGISTRY_AUTO_REGISTER === "true",
    heartbeatInterval: parseInt(process.env.CORAL_REGISTRY_HEARTBEAT_INTERVAL || "60"),
    retryAttempts: parseInt(process.env.CORAL_REGISTRY_RETRY_ATTEMPTS || "3"),
    timeout: parseInt(process.env.CORAL_REGISTRY_TIMEOUT || "5000"),
  };
}

// Validate registry settings
export function validateRegistrySettings(settings: RegistrySettings): { valid: boolean; errors: string[] } {
  const errors: string[] = [];

  if (!settings.url) {
    errors.push("Registry URL is required");
  } else {
    try {
      new URL(settings.url);
    } catch {
      errors.push("Registry URL must be a valid URL");
    }
  }

  if (settings.heartbeatInterval < 10) {
    errors.push("Heartbeat interval must be at least 10 seconds");
  }

  if (settings.retryAttempts < 0) {
    errors.push("Retry attempts must be non-negative");
  }

  if (settings.timeout < 1000) {
    errors.push("Timeout must be at least 1000ms");
  }

  return {
    valid: errors.length === 0,
    errors
  };
}

// Get authorization header for registry requests
export function getRegistryAuthHeader(settings: RegistrySettings): Record<string, string> | undefined {
  if (settings.auth?.token) {
    return { "Authorization": `Bearer ${settings.auth.token}` };
  }

  if (settings.auth?.apiKey) {
    return { "X-API-Key": settings.auth.apiKey };
  }

  if (settings.auth?.username && settings.auth?.password) {
    const credentials = Buffer.from(`${settings.auth.username}:${settings.auth.password}`).toString("base64");
    return { "Authorization": `Basic ${credentials}` };
  }

  return undefined;
}

// Registry service metadata
export interface ServiceMetadata {
  name: string;
  version: string;
  description: string;
  author: string;
  homepage?: string;
  repository?: string;
  license?: string;
  tags: string[];
  capabilities: string[];
  protocols: string[];
  endpoints: ServiceEndpoint[];
  dependencies?: string[];
  healthCheck: {
    endpoint: string;
    interval: number; // seconds
    timeout: number; // milliseconds
  };
  monitoring?: {
    metrics: boolean;
    logs: boolean;
    traces: boolean;
  };
  // Coral Protocol v1.0 specific fields
  coralVersion: string;
  agentType: 'mcp' | 'native' | 'hybrid';
  sessionSupport: boolean;
  threadSupport: boolean;
  paymentSupport: boolean;
  telemetrySupport: boolean;
}

export interface ServiceEndpoint {
  name: string;
  method: string;
  path: string;
  description: string;
  inputSchema?: any;
  outputSchema?: any;
  authentication?: boolean;
  rateLimited?: boolean;
}

// Default service metadata for IORA
export const DEFAULT_SERVICE_METADATA: ServiceMetadata = {
  name: "iora-mcp",
  version: "1.0.0",
  description: "Intelligent Oracle Rust Assistant - MCP server for comprehensive cryptocurrency analysis and blockchain oracle feeds",
  author: "IORA Dev Team",
  homepage: "https://github.com/guglxni/iora",
  repository: "https://github.com/guglxni/iora",
  license: "MIT",
  tags: [
    "cryptocurrency",
    "oracle",
    "blockchain",
    "solana",
    "ai",
    "mcp",
    "market-analysis",
    "price-data",
    "trading-signals"
  ],
  capabilities: [
    "real-time-price-data",
    "multi-api-aggregation",
    "ai-powered-analysis",
    "rag-augmented-insights",
    "solana-oracle-feeds",
    "nft-receipt-minting",
    "health-monitoring",
    "performance-metrics"
  ],
  protocols: [
    "mcp-1.0",
    "http-json",
    "websocket-optional"
  ],
  endpoints: [
    {
      name: "get_price",
      method: "POST",
      path: "/tools/get_price",
      description: "Retrieve real-time cryptocurrency price data from multiple APIs with consensus pricing",
      authentication: true,
      rateLimited: true,
      inputSchema: {
        type: "object",
        properties: {
          symbol: { type: "string", minLength: 1, maxLength: 32, description: "Cryptocurrency symbol (e.g., BTC, ETH)" },
          currency: { type: "string", default: "USD", description: "Target currency for price conversion" }
        },
        required: ["symbol"]
      },
      outputSchema: {
        type: "object",
        properties: {
          symbol: { type: "string" },
          price: { type: "number" },
          currency: { type: "string" },
          timestamp: { type: "string", format: "date-time" },
          source: { type: "string" },
          change_24h: { type: "number" },
          sources: { type: "array", items: { type: "string" } }
        }
      }
    },
    {
      name: "analyze_market",
      method: "POST",
      path: "/tools/analyze_market",
      description: "AI-powered market analysis with RAG context and trading signal generation",
      authentication: true,
      rateLimited: true,
      inputSchema: {
        type: "object",
        properties: {
          symbol: { type: "string", minLength: 1, maxLength: 32, description: "Cryptocurrency symbol to analyze" },
          analysis_type: { type: "string", enum: ["technical", "fundamental", "sentiment"], default: "technical" },
          provider: { type: "string", enum: ["gemini", "mistral", "aimlapi"], default: "gemini" },
          include_historical: { type: "boolean", default: true, description: "Include historical data in analysis" }
        },
        required: ["symbol"]
      },
      outputSchema: {
        type: "object",
        properties: {
          symbol: { type: "string" },
          analysis: { type: "string" },
          signals: { type: "array", items: { type: "string" } },
          confidence: { type: "number", minimum: 0, maximum: 1 },
          recommendation: { type: "string", enum: ["BUY", "SELL", "HOLD", "WAIT"] },
          sources: { type: "array", items: { type: "string" } },
          timestamp: { type: "string", format: "date-time" }
        }
      }
    },
    {
      name: "feed_oracle",
      method: "POST",
      path: "/tools/feed_oracle",
      description: "Submit analyzed price data to Solana blockchain oracle contracts with NFT receipt minting",
      authentication: true,
      rateLimited: true,
      inputSchema: {
        type: "object",
        properties: {
          symbol: { type: "string", minLength: 1, maxLength: 32, description: "Cryptocurrency symbol for oracle feed" },
          price: { type: "number", minimum: 0, description: "Price value to submit to oracle" },
          confidence: { type: "number", minimum: 0, maximum: 1, default: 0.95, description: "Confidence score for the price data" },
          mint_receipt: { type: "boolean", default: true, description: "Whether to mint an NFT receipt for this oracle feed" }
        },
        required: ["symbol", "price"]
      },
      outputSchema: {
        type: "object",
        properties: {
          transaction_signature: { type: "string" },
          oracle_address: { type: "string" },
          slot: { type: "number" },
          receipt_mint: { type: "string" },
          status: { type: "string", enum: ["confirmed", "pending", "failed"] },
          block_time: { type: "number" }
        }
      }
    },
    {
      name: "health",
      method: "GET",
      path: "/tools/health",
      description: "Comprehensive system health check with operational metrics and service status",
      authentication: false,
      rateLimited: false,
      outputSchema: {
        type: "object",
        properties: {
          status: { type: "string", enum: ["healthy", "degraded", "unhealthy"] },
          version: { type: "string" },
          uptime: { type: "number" },
          services: {
            type: "object",
            properties: {
              mcp_server: { type: "string", enum: ["running", "stopped", "error"] },
              ai_providers: { type: "array", items: { type: "string" } },
              blockchain: { type: "string", enum: ["connected", "disconnected"] },
              vector_db: { type: "string", enum: ["operational", "degraded", "down"] },
              apis: { type: "array", items: { type: "string" } }
            }
          },
          metrics: {
            type: "object",
            properties: {
              total_requests: { type: "number" },
              avg_response_time: { type: "string" },
              error_rate: { type: "string" },
              active_connections: { type: "number" }
            }
          },
          timestamp: { type: "string", format: "date-time" }
        }
      }
    }
  ],
  healthCheck: {
    endpoint: "/tools/health",
    interval: 30,
    timeout: 5000
  },
  monitoring: {
    metrics: true,
    logs: true,
    traces: false
  },
  // Coral Protocol v1.0 specific fields
  coralVersion: "1.0.0",
  agentType: "mcp",
  sessionSupport: true,
  threadSupport: true,
  paymentSupport: false, // Will be enabled with payment integration
  telemetrySupport: true,
  dependencies: [
    "iora-core (>=1.0.0)",
    "solana-client (>=1.18)",
    "typesense (>=27.0)",
    "@modelcontextprotocol/sdk (>=1.18)"
  ]
};

// Coral Protocol v1.0 specific types
export interface CoralSession {
  id: string;
  agentId: string;
  clientId?: string;
  startedAt: Date;
  lastActivity: Date;
  status: 'active' | 'inactive' | 'expired';
  metadata: Record<string, any>;
  threadIds: string[];
}

export interface CoralThread {
  id: string;
  sessionId: string;
  title?: string;
  createdAt: Date;
  updatedAt: Date;
  messageCount: number;
  tags: string[];
  metadata: Record<string, any>;
}

export interface CoralAgent {
  id: string;
  name: string;
  description: string;
  version: string;
  capabilities: string[];
  pricing?: {
    perRequest?: number;
    subscription?: {
      monthly: number;
      yearly: number;
    };
  };
  metadata: Record<string, any>;
}

export interface TelemetryEvent {
  id: string;
  timestamp: Date;
  event: string;
  agentId: string;
  sessionId?: string;
  threadId?: string;
  data: Record<string, any>;
}
