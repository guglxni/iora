# PRD — FEDzk: ZK-Verified Federated Learning

**Problem.** FL coordinators cannot trust client updates; clients cannot leak data.  
**Goal.** Per-client zk-SNARK proofs that updates satisfy constraints without revealing Δ:
- L2 bound: ‖Δ‖₂ ≤ B
- Optional per-coordinate bound: |Δ_k| ≤ b
- Non-triviality: Δ ≠ 0
Plus **batch verification** and **reproducible** experiments.

**Non-Goals.** Full proof-of-training; production-grade DP.

**Users.** FL platform engineers and privacy/security researchers.

**MVP.**
- Circom circuits (L2, per-coordinate, non-triviality), fixed-point encoding
- snarkjs pipeline, CLI (`setup`, `generate`, `verify`)
- Batch verification path
- Experiment harness (configs → transcripts → plots) with artifacts

**Success Criteria.**
1) Accuracy within ≤1% of plain FL on CIFAR-10 (small CNN)
2) p50 verify ≤10 ms/proof, batch ≥2k proofs/min on laptop-class HW (config documented)
3) Public artifact bundle (transcripts, timings, configs, plots) + exact commit hashes
