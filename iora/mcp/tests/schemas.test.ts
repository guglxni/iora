import { describe, it, expect } from "vitest";
import { GetPriceIn, AnalyzeIn } from "../src/schemas.js";
describe("schemas", () => {
  it("valid symbol", () => {
    expect(() => GetPriceIn.parse({ symbol: "BTC" })).not.toThrow();
  });
  it("defaults for analyze", () => {
    const v = AnalyzeIn.parse({ symbol: "ETH" });
    expect(v.horizon ?? "1d").toBeDefined();
  });
});



