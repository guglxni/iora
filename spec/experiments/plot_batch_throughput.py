import json
import pathlib

import matplotlib.pyplot as plt
import numpy as np


def latest_run() -> pathlib.Path:
    arts = sorted(p for p in pathlib.Path("artifacts").glob("20*") if p.is_dir())
    return arts[-1]


def proofs_per_sec(run: pathlib.Path) -> tuple[list[int], list[float]]:
    xs, y = [], []
    for p in sorted((run / "transcripts").glob("round_*.json")):
        j = json.loads(p.read_text())
        vs = [c.get("verify_ms", 0.0) for c in j.get("clients", [])]
        # Approx: throughput based on avg verify_ms per proof
        v = np.mean(vs) if vs else 0.0
        tput = 1000.0 / v if v > 0 else 0.0
        xs.append(j["round"])
        y.append(tput)
    return xs, y


def main() -> None:
    run = latest_run()
    xs, y = proofs_per_sec(run)
    plt.figure()
    plt.plot(xs, y, label="proofs/sec")
    plt.xlabel("round")
    plt.ylabel("proofs/sec")
    plt.legend()
    out = run / "timings" / "batch_times.png"
    plt.tight_layout()
    plt.savefig(out, dpi=160)
    print("Wrote", out)


if __name__ == "__main__":
    main()
