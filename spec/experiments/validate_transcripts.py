import json
import pathlib
import sys

from jsonschema import Draft7Validator

ROOT = pathlib.Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "spec" / "experiments" / "transcript-schema.json"


def main(run_dir: str) -> None:
    run = pathlib.Path(run_dir)
    schema = json.loads(SCHEMA.read_text())
    errors = []
    for p in (run / "transcripts").glob("round_*.json"):
        j = json.loads(p.read_text())
        v = Draft7Validator(schema)
        errs = sorted(v.iter_errors(j), key=lambda e: e.path)
        if errs:
            for e in errs:
                errors.append(f"{p.name}: {e.message}")
    if errors:
        raise SystemExit("Transcript validation errors:\n" + "\n".join(errors))
    print("All transcripts valid.")


if __name__ == "__main__":
    main(
        sys.argv[1]
        if len(sys.argv) > 1
        else sys.exit("Usage: validate_transcripts.py <run_dir>")
    )
