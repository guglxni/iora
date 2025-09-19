# Phase D Tasks — Direct Integration & Paper Figures

**D1. Direct integration (no-CLI timing skew).**
- Update `fedzk.experiments.hooks` to *try* direct Python calls into the FL stack
  (e.g., coordinator/aggregator and client trainer). Gate by env `FEDZK_DIRECT=1`.
- If direct path not importable, fallback to CLI (current behavior).

**D2. Accuracy metric.**
- Add `metrics.accuracy` in transcripts from an evaluation callback on a fixed test set.
- Accuracy must be present for Plain, Signatures, FEDzk, FEDzk-batch.

**D3. Proof sizes & memory.**
- Record proof file sizes (if proofs are emitted) into `proof_size` per client.
- Record process RSS (MB) during prove/verify into `metrics` (peak_memory_mb).

**D4. Plots & tables for paper.**
- Accuracy vs. rounds overlay (4-way baselines) → `plot_accuracy.py`.
- Throughput vs. batch size (already have per-round; add sweep plot) → `plot_throughput_vs_batch.py`.
- Table: constraint counts, key sizes, avg proof size → `table_circuits.py` (reads artifacts/meta + proofs dir).
- Table: per-client p50/p90 prove & verify times across runs → `table_timings.py`.

**D5. Grid runner.**
- Add `run_grid.py` to sweep: clients ∈ {32,128}, alpha ∈ {0.1,1.0},
  scale_bits ∈ {8,12}, B ∈ {0.5,1.0}. Emit one artifacts folder per run + an index CSV.

**D6. Make targets + CI smoke.**
- `make exp-grid` (small grid), `make paper-figs` (produce all figs/tables from last run).
- CI workflow `experiments-phase-d-smoke.yml`:
  - 2-round Plain vs FEDzk direct mode if env allows; else CLI fallback.
  - Build accuracy plot and timing table; upload artifacts.

**Acceptance.**
- `exp-grid` creates an index CSV listing runs and configs.
- `paper-figs` writes `figs/accuracy.png`, `figs/throughput_batch.png`,
  `tables/circuits.csv`, `tables/timings.csv` under the latest artifacts run.
- Transcripts include `metrics.accuracy` and memory stats.
