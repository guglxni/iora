import fetch from "node-fetch";

export interface ReceiptInput {
  symbol: string;
  price: number;
  tx: string;
  model: string;
  ts: number;
}

export interface ReceiptOutput {
  ok: true;
  provider: "crossmint";
  id: string;
  url?: string;
}

type MintResponse = { id?: string; nftAddress?: string; url?: string };

export async function mintReceipt(input: ReceiptInput): Promise<ReceiptOutput> {
  const key = process.env.CROSSMINT_API_KEY!;
  const proj = process.env.CROSSMINT_PROJECT_ID!;
  const base = process.env.CROSSMINT_BASE_URL || "https://staging.crossmint.com";
  const path = process.env.CROSSMINT_MINT_PATH || "/api/2022-06-09/collections/default/nfts";

  const res = await fetch(new URL(path, base), {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "x-client-id": proj,
      "x-api-key": key
    },
    body: JSON.stringify({
      chain: "solana",
      recipient: process.env.CROSSMINT_RECIPIENT || "email:demo@example.com",
      metadata: {
        name: `IORA Receipt ${input.symbol}`,
        description: "On-chain oracle update receipt",
        attributes: [
          { trait_type: "symbol", value: input.symbol },
          { trait_type: "price", value: input.price },
          { trait_type: "tx", value: input.tx },
          { trait_type: "model", value: input.model },
          { trait_type: "ts", value: input.ts }
        ]
      }
    }),
    // 7s total to avoid blocking user flows
    // @ts-ignore
    timeout: 7000
  });

  if (!res.ok) {
    const msg = (await res.text()).slice(0, 300);
    throw new Error(`crossmint_mint_failed: ${res.status} ${msg}`);
  }

  const j = (await res.json()) as MintResponse;
  const id = j.id || j.nftAddress || "";
  if (!id) throw new Error("crossmint_no_id");

  return { ok: true, provider: "crossmint", id, url: j.url };
}



