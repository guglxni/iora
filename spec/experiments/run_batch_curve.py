import copy
import pathlib
import subprocess
import sys

import yaml

ROOT = pathlib.Path(__file__).resolve().parents[2]


def main(base_cfg_path: str) -> None:
    base = yaml.safe_load(pathlib.Path(base_cfg_path).read_text())
    for bs in [0, 16, 64, 256]:  # 0 = unspecified (auto)
        cfg = copy.deepcopy(base)
        cfg.setdefault("zk", {})["batch_verify"] = True
        cfg["zk"]["batch_size"] = bs
        tmp = ROOT / "spec" / "experiments" / f"_tmp_batch_{bs or 'auto'}.yaml"
        tmp.write_text(yaml.safe_dump(cfg))
        subprocess.run(
            [sys.executable, "spec/experiments/round_runner.py", str(tmp)], check=False
        )
    print("Batch curve runs complete.")


if __name__ == "__main__":
    main(sys.argv[1])
