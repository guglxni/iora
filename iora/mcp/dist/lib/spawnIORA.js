import { execa } from "execa";
function ensureBin() {
    const bin = process.env.IORA_BIN || "../target/release/iora";
    if (!bin)
        throw new Error("IORA_BIN missing");
    return bin;
}
export async function runIora(cmd, args = [], env = {}) {
    const bin = ensureBin();
    const child = await execa(bin, [cmd, ...args], {
        env: { ...process.env, ...env },
        reject: false,
        timeout: 10_000, // hard timeout
        killSignal: "SIGKILL",
        maxBuffer: 2 * 1024 * 1024, // 2 MB stdout limit
    });
    if (child.timedOut)
        throw new Error(`iora ${cmd} timed out`);
    if (child.exitCode !== 0) {
        const msg = (child.stderr || child.stdout || "").toString().slice(0, 400);
        throw new Error(`iora ${cmd} failed (code ${child.exitCode}): ${msg}`);
    }
    const raw = child.stdout?.trim();
    if (!raw)
        throw new Error(`iora ${cmd} returned empty stdout`);
    let parsed;
    try {
        parsed = JSON.parse(raw);
    }
    catch {
        throw new Error(`iora ${cmd} stdout not JSON: ${raw.slice(0, 400)}`);
    }
    return parsed;
}
