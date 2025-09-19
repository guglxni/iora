import { HealthOut } from "../schemas.js";
import { runIora } from "../lib/spawnIORA.js";
export async function health() {
  const out = await runIora("health", []);
  return HealthOut.parse(out);
}



