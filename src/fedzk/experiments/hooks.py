"""
Experiment hooks for FEDzk.

This module provides a stable API from the experiment runner into the codebase.
It prefers direct Python calls if available; otherwise falls back to subprocess
calls to your CLI (python -m fedzk.cli).

IMPORTANT: Replace TODOs with actual integration points in your repo.
"""

from __future__ import annotations

import logging
import subprocess
import sys
import time
from dataclasses import dataclass

log = logging.getLogger(__name__)

CLI = [sys.executable, "-m", "fedzk.cli"]


@dataclass
class ClientResult:
    id: str
    proof_ok: bool
    prove_ms: float
    verify_ms: float
    proof_size: int = 0
    commitment: str | None = None


def _run(cmd: list[str]) -> tuple[int, float, str, str]:
    t0 = time.time()
    res = subprocess.run(cmd, capture_output=True, text=True)
    dt = (time.time() - t0) * 1000.0
    return res.returncode, dt, res.stdout, res.stderr


def _call_cli_generate(round_idx: int, cfg: dict) -> tuple[int, float, str]:
    rc, ms, out, err = _run(CLI + ["generate", "--round", str(round_idx)])
    if rc != 0:
        log.error("generate failed: %s", err.strip())
    return rc, ms, out


def _call_cli_verify(round_idx: int, cfg: dict) -> tuple[int, float, str]:
    rc, ms, out, err = _run(CLI + ["verify", "--round", str(round_idx)])
    if rc != 0:
        log.error("verify failed: %s", err.strip())
    return rc, ms, out


def run_round(cfg: dict, round_idx: int) -> list[ClientResult]:
    """
    Run one FL round according to cfg and return a list of ClientResult.
    Strategy:
      - if cfg['zk']['enabled']: call CLI generate/verify (replace with direct calls if available)
      - if signatures-only: skip ZK but still return entries
      - else: plain FL (no proofs)
    """
    results: list[ClientResult] = []
    zk_cfg = cfg.get("zk", {}) or {}
    use_zk = bool(zk_cfg.get("enabled", False))
    signatures_only = bool(cfg.get("signatures", False))

    # TODO: replace with your aggregator/trainer invocation if available
    # For now, we record one synthesized client (or per-client if you wish).
    if use_zk:
        rc_g, prove_ms, out_g = _call_cli_generate(round_idx, cfg)
        rc_v, verify_ms, out_v = _call_cli_verify(round_idx, cfg)
        ok = rc_g == 0 and rc_v == 0
        results.append(
            ClientResult(
                id="c0",
                proof_ok=ok,
                prove_ms=round(prove_ms, 2),
                verify_ms=round(verify_ms, 2),
            )
        )
    elif signatures_only:
        # No ZK; pretend signing is negligible for timings
        results.append(
            ClientResult(id="c0", proof_ok=True, prove_ms=0.0, verify_ms=0.0)
        )
    else:
        # Plain FL
        results.append(
            ClientResult(id="c0", proof_ok=True, prove_ms=0.0, verify_ms=0.0)
        )
    return results
