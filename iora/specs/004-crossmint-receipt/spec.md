# FEATURE SPEC

Feature-ID: 004-crossmint-receipt
Title: Mint receipt NFT after successful feed_oracle (devnet)

## Problem
We need an auditable, user-visible receipt for each oracle update. Crossmint provides fast custodial minting and has a partner prize.

## Goals (Must)
1) After `feed_oracle` success, POST to Crossmint to mint a receipt NFT (devnet).
2) Metadata includes: {symbol, price, tx, model/provider, ts}.
3) Endpoint: Node adds `POST /receipt` that takes `{symbol, tx, metadata}` and returns `receiptId|nftAddress`.
4) Docs: how to set `CROSSMINT_PROJECT_ID`, `CROSSMINT_API_KEY`, `CROSSMINT_CLIENT_ID`.

## Acceptance
- Live mint visible on Crossmint devnet (or returned address).
- Failure paths: if Crossmint fails, original `feed_oracle` response still succeeds but receipt call returns 502 with terse message.

## Test Plan
- Unit: schema validation.
- Live: gated by env; mint once on devnet; capture ID.
