import json
import pathlib
import subprocess
import tarfile
import time

ROOT = pathlib.Path(".")
arts = sorted([p for p in (ROOT / "artifacts").glob("20*") if p.is_dir()])
assert arts, "No artifacts found"
run = arts[-1]
outdir = ROOT / "artifacts" / "bundles"
outdir.mkdir(parents=True, exist_ok=True)
name = f"fedzk-artifacts-{time.strftime('%Y%m%d-%H%M%S')}.tar.gz"
dest = outdir / name


# capture git info
def git(cmd: list[str]) -> str:
    return subprocess.run(["git"] + cmd, capture_output=True, text=True).stdout.strip()


meta = {
    "git": {
        "commit": git(["rev-parse", "HEAD"]),
        "branch": git(["rev-parse", "--abbrev-ref", "HEAD"]),
        "status": git(["status", "--porcelain"]),
    },
    "run_dir": str(run),
}
(run / "run-git.json").write_text(json.dumps(meta, indent=2))

with tarfile.open(dest, "w:gz") as tar:
    for p in run.rglob("*"):
        tar.add(p, arcname=p.relative_to(run))
    # include configs used
    for p in (ROOT / "spec" / "experiments").glob("**/*.yaml"):
        tar.add(p, arcname=pathlib.Path("configs") / p.name)

print("Bundle:", dest)
