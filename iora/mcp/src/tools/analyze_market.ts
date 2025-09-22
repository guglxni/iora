import { AnalyzeIn, AnalyzeOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
import { z } from "zod";

// Schema for oracle command input
const OracleIn = z.object({
  symbol: z.string().min(1).max(10),
  skip_feed: z.boolean().default(false),
});

export async function analyze_market(input: unknown) {
  const args = AnalyzeIn.parse(input);
  const symbol = args.symbol.toUpperCase();
  const horizon = args.horizon ?? "1d";

  // Use the oracle command for complete RAG-enabled analysis
  const out = await runIora("oracle", ["--symbol", symbol, "--skip-feed"]);
  return AnalyzeOut.parse(out);
}

// Health monitoring tool
export async function health_check(input: unknown) {
  const out = await runIora("health", ["status"]);
  return { status: "healthy", details: out };
}

// Query tool for direct data fetching
export async function query_crypto(input: unknown) {
  const { symbol } = z.object({ symbol: z.string() }).parse(input);
  const out = await runIora("query", ["--symbol", symbol.toUpperCase()]);
  return out;
}

// Cache management tool
export async function cache_status(input: unknown) {
  const out = await runIora("cache", ["status"]);
  return { cache_info: out };
}

// Analytics tool
export async function api_analytics(input: unknown) {
  const out = await runIora("analytics", ["summary"]);
  return { analytics: out };
}
