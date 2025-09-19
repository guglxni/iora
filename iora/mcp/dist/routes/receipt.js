import { mintReceipt } from "../receipts/crossmint.js";
import { wrapper } from "../index.js";
export function mountReceipt(app) {
    app.post("/receipt", wrapper(async (body) => {
        return await mintReceipt(body);
    }));
}
