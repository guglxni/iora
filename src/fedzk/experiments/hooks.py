"""
Experiment hooks for FEDzk — Phase C (batch + attacks).
Replace TODOs with actual simulator integration when available.
"""

from __future__ import annotations

import logging
import subprocess
import sys
import time
from dataclasses import dataclass

from .attacks import AttackConfig, label_for_client

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
    reject_reason: str | None = None


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


def _call_cli_verify_batch(round_idx: int, cfg: dict) -> tuple[int, float, str]:
    # Try commonly used batch forms; fallback to loop if not supported.
    attempts = [
        CLI + ["verify-batch", "--round", str(round_idx)],
        CLI + ["verify", "--round", str(round_idx), "--batch"],
    ]
    for cmd in attempts:
        rc, ms, out, err = _run(cmd)
        if rc == 0:
            return rc, ms, out
        log.warning(
            "batch verify attempt failed cmd=%s rc=%s err=%s",
            " ".join(cmd),
            rc,
            err.strip(),
        )
    # Fallback: per-proof verify (same wall time as non-batch)
    return _call_cli_verify(round_idx, cfg)


def run_round(cfg: dict, round_idx: int) -> list[ClientResult]:
    """
    Return a list of ClientResult.
    If zk.enabled: call generate + verify (batch or not).
    If signatures-only: return ok records with zero timings.
    Else: plain FL.
    Attack labels are included in reject_reason for bookkeeping (simulator should act).
    """
    clients = int(cfg.get("clients", 1))
    results: list[ClientResult] = []
    zk_cfg = cfg.get("zk", {}) or {}
    use_zk = bool(zk_cfg.get("enabled", False))
    use_batch = bool(zk_cfg.get("batch_verify", False))
    signatures_only = bool(cfg.get("signatures", False))
    ac = AttackConfig.from_cfg(cfg)

    if use_zk:
        rc_g, prove_ms, _ = _call_cli_generate(round_idx, cfg)
        if use_batch:
            rc_v, verify_ms, _ = _call_cli_verify_batch(round_idx, cfg)
        else:
            rc_v, verify_ms, _ = _call_cli_verify(round_idx, cfg)
        ok = rc_g == 0 and rc_v == 0
        for i in range(clients):
            role = label_for_client(i, clients, ac)
            results.append(
                ClientResult(
                    id=f"c{i}",
                    proof_ok=ok,
                    prove_ms=round(prove_ms / clients if clients else prove_ms, 2),
                    verify_ms=round(verify_ms / clients if clients else verify_ms, 2),
                    reject_reason=None if ok else "verify_failed|" + role,
                )
            )
    elif signatures_only:
        for i in range(clients):
            results.append(
                ClientResult(id=f"c{i}", proof_ok=True, prove_ms=0.0, verify_ms=0.0)
            )
    else:
        for i in range(clients):
            results.append(
                ClientResult(id=f"c{i}", proof_ok=True, prove_ms=0.0, verify_ms=0.0)
            )
    return results
