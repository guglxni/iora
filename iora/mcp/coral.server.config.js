// IORA MCP Server Configuration for Coral Protocol v1.0
// Comprehensive tool definitions for Coral Studio integration
export default {
    name: "iora-mcp",
    version: "1.0.0",
    description: "Intelligent Oracle Rust Assistant - AI-powered cryptocurrency oracle with blockchain feeds",
    author: "IORA Dev Team",
    transport: "http",
    baseUrl: process.env.CORAL_SERVER_URL || "http://localhost:7070",
    // Tool definitions with detailed schemas for Coral Studio
    tools: [
        // Original MCP Tools
        {
            name: "get_price",
            description: "Retrieve real-time cryptocurrency price data from multiple APIs with consensus pricing",
            method: "POST",
            path: "/tools/get_price",
            inputSchema: {
                type: "object",
                properties: {
                    symbol: {
                        type: "string",
                        description: "Cryptocurrency symbol (e.g., BTC, ETH, SOL)",
                        minLength: 1,
                        maxLength: 32
                    },
                    currency: {
                        type: "string",
                        description: "Target currency for price conversion",
                        default: "USD"
                    }
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
            description: "AI-powered market analysis with RAG context and trading signal generation",
            method: "POST",
            path: "/tools/analyze_market",
            inputSchema: {
                type: "object",
                properties: {
                    symbol: {
                        type: "string",
                        description: "Cryptocurrency symbol to analyze",
                        minLength: 1,
                        maxLength: 32
                    },
                    analysis_type: {
                        type: "string",
                        enum: ["technical", "fundamental", "sentiment"],
                        default: "technical",
                        description: "Type of analysis to perform"
                    },
                    provider: {
                        type: "string",
                        enum: ["gemini", "mistral", "aimlapi"],
                        default: "gemini",
                        description: "AI provider for analysis"
                    },
                    include_historical: {
                        type: "boolean",
                        default: true,
                        description: "Include historical data in analysis"
                    }
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
            description: "Submit analyzed price data to Solana blockchain oracle contracts with NFT receipt minting",
            method: "POST",
            path: "/tools/feed_oracle",
            inputSchema: {
                type: "object",
                properties: {
                    symbol: {
                        type: "string",
                        description: "Cryptocurrency symbol for oracle feed",
                        minLength: 1,
                        maxLength: 32
                    },
                    price: {
                        type: "number",
                        minimum: 0,
                        description: "Price value to submit to oracle"
                    },
                    confidence: {
                        type: "number",
                        minimum: 0,
                        maximum: 1,
                        default: 0.95,
                        description: "Confidence score for the price data"
                    },
                    mint_receipt: {
                        type: "boolean",
                        default: true,
                        description: "Whether to mint an NFT receipt for this oracle feed"
                    }
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
            name: "health_check",
            description: "System health check with operational metrics and service status",
            method: "POST",
            path: "/tools/health_check",
            inputSchema: {
                type: "object",
                properties: {
                    detailed: {
                        type: "boolean",
                        default: false,
                        description: "Include detailed system metrics"
                    }
                }
            },
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
                            registry: { type: "string", enum: ["connected", "disconnected", "error"] }
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
        },
        {
            name: "query_crypto",
            description: "Query cryptocurrency data with advanced filtering and historical analysis",
            method: "POST",
            path: "/tools/query_crypto",
            inputSchema: {
                type: "object",
                properties: {
                    query: {
                        type: "string",
                        description: "Natural language query about cryptocurrency data"
                    },
                    symbols: {
                        type: "array",
                        items: { type: "string" },
                        description: "Specific symbols to include in query"
                    },
                    timeframe: {
                        type: "string",
                        enum: ["1h", "24h", "7d", "30d", "90d", "1y"],
                        default: "24h",
                        description: "Timeframe for historical data"
                    }
                },
                required: ["query"]
            },
            outputSchema: {
                type: "object",
                properties: {
                    query: { type: "string" },
                    results: { type: "array", items: { type: "object" } },
                    insights: { type: "string" },
                    timestamp: { type: "string", format: "date-time" }
                }
            }
        },
        {
            name: "cache_status",
            description: "Get cache status and performance metrics",
            method: "POST",
            path: "/tools/cache_status",
            outputSchema: {
                type: "object",
                properties: {
                    cache_enabled: { type: "boolean" },
                    total_entries: { type: "number" },
                    hit_rate: { type: "number" },
                    memory_usage: { type: "string" },
                    last_cleanup: { type: "string", format: "date-time" }
                }
            }
        },
        {
            name: "api_analytics",
            description: "Get API usage analytics and performance metrics",
            method: "POST",
            path: "/tools/api_analytics",
            inputSchema: {
                type: "object",
                properties: {
                    timeframe: {
                        type: "string",
                        enum: ["1h", "24h", "7d", "30d"],
                        default: "24h",
                        description: "Timeframe for analytics"
                    },
                    include_details: {
                        type: "boolean",
                        default: false,
                        description: "Include detailed per-endpoint metrics"
                    }
                }
            },
            outputSchema: {
                type: "object",
                properties: {
                    total_requests: { type: "number" },
                    successful_requests: { type: "number" },
                    error_rate: { type: "number" },
                    avg_response_time: { type: "number" },
                    top_endpoints: { type: "array", items: { type: "object" } },
                    timestamp: { type: "string", format: "date-time" }
                }
            }
        }
    ],
    // Health check configuration for Coral Protocol
    healthCheck: {
        endpoint: "/tools/health",
        interval: 30,
        timeout: 5000
    },
    // Metadata for Coral Protocol discovery
    metadata: {
        tags: [
            "cryptocurrency",
            "oracle",
            "blockchain",
            "solana",
            "ai",
            "mcp",
            "market-analysis",
            "price-data",
            "trading-signals",
            "nft-receipts",
            "coral-protocol",
            "sessions",
            "threads",
            "telemetry",
            "agent-management"
        ],
        capabilities: [
            "real-time-price-data",
            "multi-api-aggregation",
            "ai-powered-analysis",
            "rag-augmented-insights",
            "solana-oracle-feeds",
            "nft-receipt-minting",
            "health-monitoring",
            "performance-metrics",
            // Coral Protocol v1.0 capabilities
            "session-management",
            "thread-management",
            "agent-execution",
            "telemetry-analytics",
            "system-monitoring"
        ],
        protocols: [
            "mcp-1.0",
            "http-json",
            "websocket-optional",
            "coral-protocol-1.0"
        ]
    },
    // Coral Protocol v1.0 specific configuration
    coral: {
        version: "1.0.0",
        features: {
            sessions: {
                enabled: true,
                maxDuration: 3600, // seconds
                cleanupInterval: 300 // seconds
            },
            threads: {
                enabled: true,
                maxPerSession: 50,
                archiving: {
                    enabled: true,
                    olderThanDays: 30
                }
            },
            agents: {
                enabled: true,
                execution: true,
                management: true
            },
            telemetry: {
                enabled: true,
                maxEvents: 10000,
                retentionDays: 7,
                analytics: true
            }
        },
        endpoints: {
            sessions: "/coral/sessions",
            sessionDetails: "/coral/sessions/:sessionId",
            agentExecution: "/coral/agents/:agentId/execute",
            telemetry: "/coral/telemetry/analytics",
            systemStats: "/coral/system/stats"
        }
    }
};
