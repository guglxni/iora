"""
Attack configurations and acceptance helpers for FEDzk experiments.
Actual gradient/update access should come from your simulator; this module only
declares attack intent and records labels so the simulator can act accordingly.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Literal

AttackKind = Literal["none", "scaling", "sign_flip", "sparse_poison"]


@dataclass
class AttackConfig:
    kind: AttackKind = "none"
    fraction_malicious: float = 0.0  # 0..1
    scale: float = 3.0  # used by scaling/sparse_poison
    sparsity_pct: float = 95.0  # percent zeroed for sparse_poison (0..100)

    @classmethod
    def from_cfg(cls, cfg: Dict[str, Any]) -> "AttackConfig":
        a = (cfg or {}).get("attack", {}) or {}
        return cls(
            kind=a.get("kind", "none"),
            fraction_malicious=float(a.get("fraction_malicious", 0.0)),
            scale=float(a.get("scale", 3.0)),
            sparsity_pct=float(a.get("sparsity_pct", 95.0)),
        )


def label_for_client(idx: int, total: int, ac: AttackConfig) -> str:
    if ac.kind == "none" or ac.fraction_malicious <= 0:
        return "honest"
    cutoff = int(total * ac.fraction_malicious)
    return "malicious" if idx < cutoff else "honest"
