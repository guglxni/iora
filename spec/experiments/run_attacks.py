import copy
import pathlib
import subprocess
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]

ATTACKS = [
    {"kind": "scaling", "fraction_malicious": 0.25, "scale": 3.0},
    {"kind": "sign_flip", "fraction_malicious": 0.25},
    {
        "kind": "sparse_poison",
        "fraction_malicious": 0.25,
        "scale": 3.0,
        "sparsity_pct": 95.0,
    },
]


def main(base_cfg_path: str) -> None:
    base = yaml.safe_load(pathlib.Path(base_cfg_path).read_text())
    for atk in ATTACKS:
        cfg = copy.deepcopy(base)
        cfg["attack"] = atk
        tmp = ROOT / "spec" / "experiments" / f"_tmp_attack_{atk['kind']}.yaml"
        tmp.write_text(yaml.safe_dump(cfg))
        subprocess.run(
            [sys.executable, "spec/experiments/round_runner.py", str(tmp)], check=False
        )
    print("Attack runs complete.")


if __name__ == "__main__":
    main(sys.argv[1])
