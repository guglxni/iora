import copy
import csv
import pathlib
import subprocess
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]

CLIENTS = [32, 128]
ALPHAS = [0.1, 1.0]
SCALES = [8, 12]
BOUNDS = [0.5, 1.0]


def main(base_cfg_path: str) -> None:
    base = yaml.safe_load(pathlib.Path(base_cfg_path).read_text())
    out_dir = ROOT / "artifacts" / "grid"
    out_dir.mkdir(parents=True, exist_ok=True)
    index_rows = []
    for c in CLIENTS:
        for a in ALPHAS:
            for s in SCALES:
                for b in BOUNDS:
                    cfg = copy.deepcopy(base)
                    cfg["clients"] = c
                    cfg["dirichlet_alpha"] = a
                    cfg.setdefault("zk", {})["scale_bits"] = s
                    cfg["zk"]["l2_bound"] = b
                    tmp = (
                        ROOT
                        / "spec"
                        / "experiments"
                        / f"_grid_c{c}_a{a}_s{s}_b{b}.yaml"
                    )
                    tmp.write_text(yaml.safe_dump(cfg))
                    subprocess.run(
                        [sys.executable, "spec/experiments/round_runner.py", str(tmp)],
                        check=False,
                    )
                    index_rows.append(
                        {
                            "clients": c,
                            "alpha": a,
                            "scale_bits": s,
                            "l2_bound": b,
                            "cfg": str(tmp),
                        }
                    )
    with (out_dir / "index.csv").open("w", newline="") as f:
        w = csv.DictWriter(
            f, fieldnames=["clients", "alpha", "scale_bits", "l2_bound", "cfg"]
        )
        w.writeheader()
        w.writerows(index_rows)
    print("Grid complete:", out_dir / "index.csv")


if __name__ == "__main__":
    main(sys.argv[1])
