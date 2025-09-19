import json
import pathlib

import matplotlib.pyplot as plt


def latest_run() -> pathlib.Path:
    arts = sorted(p for p in pathlib.Path("artifacts").glob("20*") if p.is_dir())
    return arts[-1]


def main() -> None:
    run = latest_run()
    xs, rates = [], []
    for p in sorted((run / "transcripts").glob("round_*.json")):
        j = json.loads(p.read_text())
        total = len(j.get("clients", [])) or 1
        acc = len(j.get("accepted_clients", []))
        xs.append(j["round"])
        rates.append(100.0 * acc / total)
    plt.figure()
    plt.plot(xs, rates, label="acceptance %")
    plt.xlabel("round")
    plt.ylabel("% accepted")
    plt.legend()
    out = run / "timings" / "acceptance.png"
    plt.tight_layout()
    plt.savefig(out, dpi=160)
    print("Wrote", out)


if __name__ == "__main__":
    main()
