import { describe, it, expect } from "vitest";
import fetch from "node-fetch";

const base = "http://localhost:7070";
const secret = process.env.CORAL_SHARED_SECRET!;
function sig(body:any){ return require("crypto").createHmac("sha256",secret).update(JSON.stringify(body)).digest("hex"); }

describe("real e2e", () => {
  it("health", async () => {
    const r = await fetch(`${base}/tools/health`);
    const j = await r.json();
    expect(j.ok).toBe(true);
    expect(j.data.status).toBe("ok");
  });

  it("price + analyze + feed_oracle", async () => {
    const body1 = { symbol:"BTC" };
    const r1 = await fetch(`${base}/tools/get_price`, { method:"POST", headers:{ "content-type":"application/json", "x-iora-signature":sig(body1) }, body: JSON.stringify(body1) });
    const j1 = await r1.json(); expect(j1.ok).toBe(true); expect(j1.data.price).toBeGreaterThan(0);

    const body2 = { symbol:"BTC", horizon:"1d" };
    const r2 = await fetch(`${base}/tools/analyze_market`, { method:"POST", headers:{ "content-type":"application/json", "x-iora-signature":sig(body2) }, body: JSON.stringify(body2) });
    const j2 = await r2.json(); expect(j2.ok).toBe(true); expect(j2.data.signals.length).toBeGreaterThan(0);

    const body3 = { symbol:"BTC" };
    const r3 = await fetch(`${base}/tools/feed_oracle`, { method:"POST", headers:{ "content-type":"application/json", "x-iora-signature":sig(body3) }, body: JSON.stringify(body3) });
    const j3 = await r3.json(); expect(j3.ok).toBe(true); expect(j3.data.tx.length).toBeGreaterThan(16);
  }, 30_000);
});
