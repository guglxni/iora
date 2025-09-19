import json
import pathlib

import matplotlib.pyplot as plt
import numpy as np


def latest_run() -> pathlib.Path:
    arts = sorted(p for p in pathlib.Path("artifacts").glob("20*") if p.is_dir())
    return arts[-1]


def main() -> None:
    run = latest_run()
    xs, ys = [], []
    for p in sorted((run / "transcripts").glob("round_*.json")):
        j = json.loads(p.read_text())
        vs = [c.get("verify_ms", 0.0) for c in j.get("clients", [])]
        v = float(np.mean(vs)) if vs else 0.0
        xs.append(j["round"])
        ys.append(1000.0 / v if v > 0 else 0.0)
    out = run / "figs" / "throughput_batch.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    plt.figure()
    plt.plot(xs, ys, label="proofs/sec")
    plt.xlabel("round")
    plt.ylabel("proofs/sec")
    plt.legend()
    plt.tight_layout()
    plt.savefig(out, dpi=160)
    print("Wrote", out)


if __name__ == "__main__":
    main()
