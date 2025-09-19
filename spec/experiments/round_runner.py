import json, time, subprocess, hashlib, yaml, pathlib, sys, os
from datetime import datetime

ROOT = pathlib.Path(__file__).resolve().parents[2]
ART = ROOT / "artifacts" / datetime.now().strftime("%Y%m%d-%H%M%S")
(ART / "transcripts").mkdir(parents=True, exist_ok=True)
(ART / "timings").mkdir(parents=True, exist_ok=True)

# Adjust if your CLI entrypoint differs:
FEDZK_CLI = [sys.executable, "-m", "fedzk.cli"]

def sha256_str(s: bytes) -> str:
    import hashlib
    h = hashlib.sha256(); h.update(s); return "sha256:" + h.hexdigest()

def run_cmd(cmd):
    t0 = time.time()
    res = subprocess.run(cmd, capture_output=True, text=True)
    dt_ms = (time.time() - t0) * 1000.0
    return res.returncode, dt_ms, res.stdout, res.stderr

def main(cfg_path: str):
    cfg = yaml.safe_load(pathlib.Path(cfg_path).read_text())
    # TODO: hook into your FL simulator; the calls below are placeholders.
    for r in range(int(cfg["rounds"])):
        # Prove (placeholder)
        rc_p, prove_ms, out_p, err_p = run_cmd(FEDZK_CLI + ["generate", "--round", str(r)])
        # Verify (placeholder)
        rc_v, verify_ms, out_v, err_v = run_cmd(FEDZK_CLI + ["verify", "--round", str(r)])

        transcript = {
            "round": r,
            "model_hash": "sha256:TODO",
            "params": cfg,
            "clients": [{
                "id": "c0",
                "commitment": "TODO",
                "proof_ok": rc_v == 0,
                "proof_size": 0,
                "prove_ms": round(prove_ms, 2),
                "verify_ms": round(verify_ms, 2)
            }],
            "accepted_clients": ["c0" if rc_v == 0 else []],
            "aggregate_hash": "sha256:TODO",
            "batch_verify": {"enabled": bool(cfg["zk"].get("batch_verify", False))}
        }
        (ART / "transcripts" / f"round_{r:03d}.json").write_text(json.dumps(transcript, indent=2))
    print("Artifacts at:", ART)

if __name__ == "__main__":
    main(sys.argv[1])
