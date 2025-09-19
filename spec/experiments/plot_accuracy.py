import json
import pathlib

import matplotlib.pyplot as plt


def latest_run() -> pathlib.Path:
    arts = sorted(p for p in pathlib.Path("artifacts").glob("20*") if p.is_dir())
    return arts[-1]


def read_acc(run: pathlib.Path) -> tuple[list[int], list[float]]:
    xs, accs = [], []
    for p in sorted((run / "transcripts").glob("round_*.json")):
        j = json.loads(p.read_text())
        m = (j.get("metrics") or {}).get("accuracy", None)
        if m is not None:
            xs.append(j["round"])
            accs.append(float(m))
    return xs, accs


def main() -> None:
    run = latest_run()
    xs, acc = read_acc(run)
    plt.figure()
    plt.plot(xs, acc, label="accuracy")
    plt.xlabel("round")
    plt.ylabel("accuracy")
    plt.legend()
    out = run / "figs" / "accuracy.png"
    out.parent.mkdir(parents=True, exist_ok=True)
    plt.tight_layout()
    plt.savefig(out, dpi=160)
    print("Wrote", out)


if __name__ == "__main__":
    main()
