import csv
import pathlib
import sys

import matplotlib.pyplot as plt


def main(csv_path: str, out_png: str) -> None:
    xs, prove, verify = [], [], []
    with open(csv_path, "r", newline="") as f:
        r = csv.DictReader(f)
        for row in r:
            xs.append(int(row["round"]))
            prove.append(float(row["prove_ms"]))
            verify.append(float(row["verify_ms"]))
    plt.figure()
    plt.plot(xs, prove, label="prove_ms")
    plt.plot(xs, verify, label="verify_ms")
    plt.xlabel("round")
    plt.ylabel("ms")
    plt.legend()
    plt.tight_layout()
    plt.savefig(out_png, dpi=160)


if __name__ == "__main__":
    run_dir = pathlib.Path(sys.argv[1])
    out = run_dir / "timings" / "times.png"
    main(str(run_dir / "timings" / "timings.csv"), str(out))
    print("Wrote", out)
