import datetime
import json
import pathlib
import shlex
import subprocess
import sys

BUILD = pathlib.Path("circuits") / "build"
OUTDIR = pathlib.Path("artifacts") / "meta"
OUTDIR.mkdir(parents=True, exist_ok=True)


def r1cs_info(p: pathlib.Path) -> dict[str, str | bool | int]:
    cmd = f"snarkjs r1cs info {shlex.quote(str(p))}"
    res = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    meta: dict[str, str | bool | int] = {
        "file": str(p),
        "ok": res.returncode == 0,
        "raw": res.stdout,
    }
    # Best-effort parse of common lines
    for line in res.stdout.splitlines():
        if "nConstraints" in line:
            meta["nConstraints"] = int(line.split(":")[-1].strip())
        if "nPrvInputs" in line or "nInputs" in line:
            try:
                meta["nInputs"] = int(line.split(":")[-1].strip())
            except (ValueError, IndexError):
                pass
        if "nSignals" in line:
            try:
                meta["nSignals"] = int(line.split(":")[-1].strip())
            except (ValueError, IndexError):
                pass
    return meta


def main() -> int:
    entries = []
    for r1cs in sorted(BUILD.rglob("*.r1cs")):
        entries.append(r1cs_info(r1cs))
    stamp = datetime.datetime.utcnow().strftime("%Y%m%dT%H%M%SZ")
    (OUTDIR / f"circuits-meta-{stamp}.json").write_text(json.dumps(entries, indent=2))
    print(f"Wrote {(OUTDIR / f'circuits-meta-{stamp}.json')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
