import { GetPriceIn, GetPriceOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
export async function get_price(input: unknown) {
  const args = GetPriceIn.parse(input);
  const out = await runIora("get_price", ["--symbol", args.symbol]);
  return GetPriceOut.parse(out);
}
