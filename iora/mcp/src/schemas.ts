import { z } from "zod";

const finite = () => z.number().refine(Number.isFinite, "must be finite");

export const SymbolSchema = z.string().min(1).max(32).regex(/^[A-Z0-9:_\-\.]+$/);

export const GetPriceIn = z.object({ symbol: SymbolSchema });
export const GetPriceOut = z.object({
  symbol: SymbolSchema,
  price: finite(),
  source: z.string().min(1),
  ts: z.number().int().positive()
});

export const AnalyzeIn = z.object({
  symbol: SymbolSchema,
  horizon: z.enum(["1h","1d","1w"]).default("1d").optional(),
  provider: z.enum(["gemini","mistral","aimlapi"]).default("gemini").optional()
});
export const AnalyzeOut = z.object({
  summary: z.string().min(1),
  signals: z.array(z.string()).min(1),
  confidence: z.number().min(0).max(1),
  sources: z.array(z.string())
});

export const FeedOracleIn = z.object({ symbol: SymbolSchema });
export const FeedOracleOut = z.object({
  tx: z.string().min(16),
  slot: z.number().int().nonnegative(),
  digest: z.string().min(16)
});

export const HealthOut = z.object({
  status: z.literal("ok"),
  versions: z.object({ iora: z.string(), mcp: z.string().optional() }),
  uptime_sec: z.number().int().nonnegative()
});

export const ReceiptIn = z.object({
  symbol: SymbolSchema,
  price: z.number().finite(),
  tx: z.string().min(16),
  model: z.string().min(1),
  ts: z.number().int().positive()
});
export const ReceiptOut = z.object({
  ok: z.literal(true),
  provider: z.literal("crossmint"),
  id: z.string().min(8),
  url: z.string().url().optional()
});
