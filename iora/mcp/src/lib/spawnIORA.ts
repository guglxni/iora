import { execa } from "execa";
import path from "path";
import fs from "fs";

export type IoraCmd = "get_price" | "analyze_market" | "feed_oracle" | "health" | "oracle" | "query" | "cache" | "analytics";

function ensureBin() {
  const bin = process.env.IORA_BIN || "./target/release/iora";
  if (!bin) throw new Error("IORA_BIN missing");
  return bin;
}

export async function runIora(
  cmd: IoraCmd,
  args: string[] = [],
  env: Record<string, string | undefined> = {}
) {
  const bin = ensureBin();

  const child = await execa(bin, [cmd, ...args], {
    env: { ...process.env, ...env },
    reject: false,
    timeout: 30_000,            // increased timeout for LLM calls
    killSignal: "SIGKILL",
    maxBuffer: 2 * 1024 * 1024, // 2 MB stdout limit
  });

  if (child.timedOut) throw new Error(`iora ${cmd} timed out`);
  if (child.exitCode !== null && child.exitCode !== 0) {
    const msg = (child.stderr || child.stdout || "").toString().slice(0, 400);
    throw new Error(`iora ${cmd} failed (code ${child.exitCode}): ${msg}`);
  }

  const raw = child.stdout?.trim();
  if (!raw) throw new Error(`iora ${cmd} returned empty stdout`);

  // Extract JSON from output - handle cases where logs precede JSON
  const lines = raw.split('\n').filter(line => line.trim());
  const jsonLine = lines[lines.length - 1]; // Take the last line (should be JSON)

  let parsed: unknown;
  try {
    parsed = JSON.parse(jsonLine);
  } catch {
    throw new Error(`iora ${cmd} stdout not valid JSON: ${jsonLine?.slice(0, 400)} (full output: ${raw.slice(0, 400)})`);
  }
  return parsed;
}
