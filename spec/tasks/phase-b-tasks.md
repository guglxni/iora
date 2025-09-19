# Phase B Tasks — Reproducible Experiments

**B1. Runner wiring.**
- Replace placeholders in `spec/experiments/round_runner.py` by calling `src/fedzk/experiments/hooks.py::run_round`.
- `run_round` MUST return per-client records with `id`, `proof_ok`, `prove_ms`, `verify_ms`, `proof_size`.

**B2. Configs (4-way baselines).**
- Add configs for Adult (logreg) and CIFAR-10 (small_cnn):
  - Plain FL
  - Signatures-only (no ZK)
  - FEDzk (per-client ZK)
  - FEDzk-batch (ZK + batch verify)
- Keep rounds, clients, alpha small for smoke; larger for real runs.

**B3. Transcript validation + metrics.**
- Validate every transcript against `transcript-schema.json`.
- Collect timings into CSV (prove_ms, verify_ms, batch) and write a run manifest with system info.

**B4. Plots.**
- Add matplotlib scripts for (i) prove/verify time vs. rounds and (ii) batch throughput vs. batch size (when present).
- No seaborn. One chart per file.

**B5. CI smoke.**
- Add a GitHub Actions workflow that runs a 2-round smoke for Adult LR in each of: Plain, FEDzk.
- Validate schema and upload artifacts.

**Acceptance.**
- `make exp-smoke` completes locally and in CI.
- `validate_transcripts.py` passes.
- `timings.csv` produced with non-empty rows.
- Plots generated (PNG) for smoke runs.
