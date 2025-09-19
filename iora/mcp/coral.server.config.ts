// Placeholder manifest so we can wire real Coral bindings later.
// Export a description of available tools + HTTP endpoints for local Studio testing.
export default {
  name: "iora-mcp",
  transport: "http",
  baseUrl: "http://localhost:7070",
  tools: [
    { name: "get_price", method: "POST", path: "/tools/get_price" },
    { name: "analyze_market", method: "POST", path: "/tools/analyze_market" },
    { name: "feed_oracle", method: "POST", path: "/tools/feed_oracle" },
    { name: "health", method: "GET", path: "/tools/health" }
  ]
};



