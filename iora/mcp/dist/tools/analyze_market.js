import { AnalyzeIn, AnalyzeOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
export async function analyze_market(input) {
    const args = AnalyzeIn.parse(input);
    const provider = args.provider ?? (process.env.LLM_PROVIDER || "gemini");
    const out = await runIora("analyze_market", [
        args.symbol,
        args.horizon ?? "1d",
        provider
    ]);
    return AnalyzeOut.parse(out);
}
