"""
Experiment hooks for FEDzk — Phase D (direct integration when available).
If FEDZK_DIRECT=1 and the internal APIs import successfully, we call them to avoid
CLI timing skew. Otherwise, we fallback to CLI as in earlier phases.
"""

from __future__ import annotations

import logging
import os
import pathlib
import subprocess
import sys
import time
from dataclasses import dataclass

import psutil

from .attacks import AttackConfig, label_for_client

log = logging.getLogger(__name__)
CLI = [sys.executable, "-m", "fedzk.cli"]
DIRECT = os.environ.get("FEDZK_DIRECT", "0") == "1"

# Attempt direct imports (user can rewire here to actual modules).
try:
    # Replace these with your real entry points if available.
    # Example placeholders (safe to fail if not present):
    # from fedzk.client.trainer import train_local_update as _train_update
    # from fedzk.coordinator.aggregator import aggregate_round as _agg_round
    # from fedzk.eval import evaluate_accuracy as _eval_acc
    HAVE_DIRECT = False  # Set to True when real imports are available
except Exception:
    HAVE_DIRECT = False


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


def _rss_mb() -> float:
    try:
        return psutil.Process().memory_info().rss / (1024 * 1024)
    except Exception:
        return 0.0


def _proof_sizes_in_dir(proofs_dir: pathlib.Path) -> list[int]:
    sizes = []
    for p in proofs_dir.glob("*.proof"):
        try:
            sizes.append(p.stat().st_size)
        except FileNotFoundError:
            continue
    return sizes


def run_round(cfg: dict, round_idx: int) -> list[ClientResult]:
    """
    Return a list of ClientResult and write accuracy/memory into outer transcript
    (runner will attach).
    Strategy:
      - If DIRECT and HAVE_DIRECT: call internal APIs for updates/proofs/verify + accuracy eval
      - Else: CLI generate/verify; accuracy left None (runner can compute if available)
    """
    clients = int(cfg.get("clients", 1))
    results: list[ClientResult] = []
    zk_cfg = cfg.get("zk", {}) or {}
    use_zk = bool(zk_cfg.get("enabled", False))
    signatures_only = bool(cfg.get("signatures", False))
    ac = AttackConfig.from_cfg(cfg)

    if DIRECT and HAVE_DIRECT and use_zk:
        t0 = time.time()
        # Pseudocode example: you must adapt to your real APIs.
        # local_updates = [_train_update(i, cfg, round_idx) for i in range(clients)]
        # proofs = prove_updates(local_updates, cfg)
        # verify = verify_updates(proofs, batch=use_batch)
        prove_ms = (time.time() - t0) * 1000.0
        t1 = time.time()
        verify_ms = (time.time() - t1) * 1000.0
        ok = True  # replace with actual verify result
        for i in range(clients):
            role = label_for_client(i, clients, ac)
            results.append(
                ClientResult(
                    id=f"c{i}",
                    proof_ok=ok,
                    prove_ms=round(prove_ms / clients or 0.0, 2),
                    verify_ms=round(verify_ms / clients or 0.0, 2),
                    proof_size=0,
                    reject_reason=(None if ok else "verify_failed|" + role),
                )
            )
        return results

    # Fallback: CLI path (Phase B/C behavior).
    if use_zk:
        rc_g, prove_ms, _ = _call_cli_generate(round_idx, cfg)
        rc_v, verify_ms, _ = (
            _call_cli_verify_batch(round_idx, cfg)
            if zk_cfg.get("batch_verify")
            else _call_cli_verify(round_idx, cfg)
        )
        ok = rc_g == 0 and rc_v == 0
        for i in range(clients):
            role = label_for_client(i, clients, ac)
            results.append(
                ClientResult(
                    id=f"c{i}",
                    proof_ok=ok,
                    prove_ms=round(prove_ms / clients if clients else prove_ms, 2),
                    verify_ms=round(verify_ms / clients if clients else verify_ms, 2),
                    proof_size=0,
                    reject_reason=(None if ok else "verify_failed|" + role),
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
