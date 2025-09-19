import csv
import json
import pathlib

META = sorted(pathlib.Path("artifacts/meta").glob("circuits-meta-*.json"))
assert META, "No circuits meta found; run `make circuits-meta`"
rows = []
for meta in META[-1:]:
    data = json.loads(meta.read_text())
    for e in data:
        rows.append(
            {
                "file": e.get("file", ""),
                "nConstraints": e.get("nConstraints", ""),
                "nInputs": e.get("nInputs", ""),
                "nSignals": e.get("nSignals", ""),
            }
        )
out = pathlib.Path("artifacts") / "tables"
out.mkdir(parents=True, exist_ok=True)
with (out / "circuits.csv").open("w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=["file", "nConstraints", "nInputs", "nSignals"])
    w.writeheader()
    w.writerows(rows)
print("Wrote", out / "circuits.csv")
