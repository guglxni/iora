# FEATURE SPEC

Feature-ID: 003-llm-provider-switch
Title: Add partner LLM providers (Mistral / AI-ML API) with provider passthrough

## Problem
Judging includes partner-tech usage. We need first-class Mistral and/or AI-ML API so `analyze_market` can select provider.

## Goals (Must)
1) Support `provider = gemini|mistral|aimlapi` end-to-end (CLI → Rust → HTTP calls).
2) Env keys: `MISTRAL_API_KEY`, `AIMLAPI_API_KEY`; documented in README.
3) Latency budget: p50 ≤ 3.5s on short prompts.
4) Conformance: JSON output schema unchanged.

## Interfaces
- CLI: `iora analyze_market <symbol> <horizon> <provider>`
- Rust: enum `LlmProvider::{Gemini,Mistral,AimlApi}` with http clients + timeouts.
- Node: pass-through of `provider` from request to CLI args.

## Acceptance
- Two real runs show different model IDs in `sources[]`.
- 3× load: all complete under 10s budget.
- Unit tests for provider selection + unhappy paths (missing key → 400).

## Test Plan
- Wiremock (or minimal test server) for http fallbacks in unit tests.
- Live smoke tests gated by presence of API keys.
