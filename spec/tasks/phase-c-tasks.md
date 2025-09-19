# Phase C Tasks — Batch Verification & Robustness

**C1. Batch verification.**
- Enable batch verification path in experiments.
- If CLI supports `verify-batch` (or `verify --batch`), use it; otherwise fallback to per-proof verify.
- Record per-round verify time and compute proofs/sec (throughput).

**C2. Attack simulations.**
- Implement three attacks: scaling (Δ ← s·Δ), sign-flip (Δ ← −Δ), sparse poisoning (randomly zero all but p% largest coords, then scale).
- For each round, record acceptance_rate = accepted/total and (if available) accuracy metric from simulator.
- Attacks configurable via YAML.

**C3. Schema & validation.**
- Extend transcript schema with optional per-client `reject_reason` and top-level `metrics` (e.g., {"accuracy": float}).
- Keep old fields valid (backward-compatible).
- Add validator that fails on unknown required fields but allows our new optional ones.

**C4. Plots.**
- Add matplotlib scripts: (i) batch throughput vs. round, (ii) acceptance rate vs. round.
- No seaborn; one chart per file.

**C5. Batch curve & attacks harness.**
- Add runners to sweep batch settings and attacks with small smoke defaults.
- Make targets: `exp-batch-curve`, `exp-attacks`.

**C6. CI smoke.**
- Add a GitHub Actions workflow that runs:
  - small batch-throughput smoke (2 rounds, Adult LR, ZK+batch)
  - sign-flip attack smoke (2 rounds)
- Upload artifacts.

**Acceptance.**
- `make exp-batch-curve` produces a run with `timings.csv` and `batch_times.png`.
- `make exp-attacks` produces acceptance CSV and `acceptance.png`.
- CI smoke green.
