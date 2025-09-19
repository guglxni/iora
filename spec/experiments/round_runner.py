import json
import pathlib
import platform
import sys
from datetime import datetime

import yaml

from fedzk.experiments.hooks import run_round

ROOT = pathlib.Path(__file__).resolve().parents[2]
RUN_DIR = ROOT / "artifacts" / datetime.now().strftime("%Y%m%d-%H%M%S")
TRANS = RUN_DIR / "transcripts"
TIMINGS = RUN_DIR / "timings"
RUN_DIR.mkdir(parents=True, exist_ok=True)
TRANS.mkdir(parents=True, exist_ok=True)
TIMINGS.mkdir(parents=True, exist_ok=True)


def system_info() -> dict:
    return {
        "python": sys.version.split()[0],
        "platform": platform.platform(),
        "machine": platform.machine(),
        "processor": platform.processor(),
    }


def main(cfg_path: str) -> None:
    cfg = yaml.safe_load(pathlib.Path(cfg_path).read_text())
    manifest = {
        "config_path": str(cfg_path),
        "config": cfg,
        "created_at": datetime.utcnow().isoformat() + "Z",
        "system": system_info(),
    }
    (RUN_DIR / "run.json").write_text(json.dumps(manifest, indent=2))

    for r in range(int(cfg["rounds"])):
        client_results = run_round(cfg, r)
        transcript = {
            "round": r,
            "model_hash": "sha256:TODO",
            "params": cfg,
            "clients": [cr.__dict__ for cr in client_results],
            "accepted_clients": [cr.id for cr in client_results if cr.proof_ok],
            "aggregate_hash": "sha256:TODO",
            "batch_verify": (
                {"enabled": bool(cfg["zk"].get("batch_verify", False))}
                if cfg.get("zk")
                else {"enabled": False}
            ),
        }
        (TRANS / f"round_{r:03d}.json").write_text(json.dumps(transcript, indent=2))
    print("Artifacts at:", RUN_DIR)


if __name__ == "__main__":
    main(sys.argv[1])
