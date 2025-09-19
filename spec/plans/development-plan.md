# Development Plan

**Phase A — Hygiene & Baseline.**
- Replace bare `except:` with specific exceptions; add logging + tests
- Replace `print()` in src/ with structured logging
- Remove real secrets from VCS; wire SOPS/Vault/K8s Secrets
- Freeze circuit params (scale_bits, bounds) and emit constraint counts/keys metadata

**Phase B — Reproducible Experiments.**
- YAML configs for Adult (logreg) and CIFAR-10 (small CNN)
- Round runner that emits JSON transcripts + timings
- Collector to aggregate into CSV; plotting scripts
- Baselines: Plain FL / Signatures-only / FEDzk / FEDzk-batch

**Phase C — Batch & Robustness.**
- Batch verification throughput scaling
- Attacks: scaling, sign-flip, sparse poisoning; acceptance vs. accuracy impact
- Publish artifact bundle with DOI

Each phase ships code, tests, CI, docs, and artifacts.
