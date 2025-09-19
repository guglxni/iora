import { FeedOracleIn, FeedOracleOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
import fetch from "node-fetch";
export async function feed_oracle(input) {
    const args = FeedOracleIn.parse(input);
    // Kill-switch for oracle feeds
    if (process.env.DISABLE_FEED_ORACLE === "1") {
        throw new Error("feed_oracle_disabled: Oracle feeds are currently disabled for maintenance");
    }
    // Execute the oracle feed
    const out = await runIora("feed_oracle", [args.symbol]);
    const result = FeedOracleOut.parse(out);
    // Attempt to mint receipt asynchronously (don't block oracle success)
    // This runs in background and doesn't affect the oracle response
    setImmediate(async () => {
        try {
            if (process.env.CROSSMINT_API_KEY && process.env.CROSSMINT_PROJECT_ID) {
                // Get current price for receipt metadata
                const priceData = await runIora("get_price", [args.symbol]);
                const receiptPayload = {
                    symbol: args.symbol,
                    price: priceData.price,
                    tx: result.tx,
                    model: "oracle-feed", // Could be enhanced to include LLM provider info
                    ts: Math.floor(Date.now() / 1000)
                };
                // Call receipt endpoint
                const receiptRes = await fetch("http://localhost:7070/receipt", {
                    method: "POST",
                    headers: {
                        "content-type": "application/json",
                        "x-iora-signature": generateSignature(receiptPayload)
                    },
                    body: JSON.stringify(receiptPayload)
                });
                if (receiptRes.ok) {
                    console.log(`Receipt minted for ${args.symbol} oracle feed`);
                }
                else {
                    console.warn(`Receipt minting failed for ${args.symbol}: ${receiptRes.status}`);
                }
            }
        }
        catch (error) {
            console.warn(`Receipt minting error for ${args.symbol}:`, error);
        }
    });
    return result;
}
// Simple signature generation for internal receipt calls
function generateSignature(body) {
    const secret = process.env.CORAL_SHARED_SECRET || "";
    const crypto = require("crypto");
    return crypto.createHmac("sha256", secret)
        .update(JSON.stringify(body))
        .digest("hex");
}
