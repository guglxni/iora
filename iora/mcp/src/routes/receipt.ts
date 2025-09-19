import { mintReceipt } from "../receipts/crossmint.js";
import { wrapper } from "../index.js";

export function mountReceipt(app: any) {
  app.post("/receipt", wrapper(async (body: any) => {
    return await mintReceipt(body);
  }));
}



