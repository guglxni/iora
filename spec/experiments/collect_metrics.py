import csv
import json
import pathlib

ART = pathlib.Path("artifacts")
runs = sorted(p for p in ART.glob("20*") if p.is_dir())
assert runs, "No artifacts found"
last = runs[-1]

rows = []
for p in (last / "transcripts").glob("round_*.json"):
    j = json.loads(p.read_text())
    for c in j["clients"]:
        rows.append(
            {
                "round": j["round"],
                "prove_ms": c.get("prove_ms", 0.0),
                "verify_ms": c.get("verify_ms", 0.0),
                "batch": j.get("batch_verify", {}).get("enabled", False),
            }
        )

out = last / "timings" / "timings.csv"
out.parent.mkdir(parents=True, exist_ok=True)
with out.open("w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=["round", "prove_ms", "verify_ms", "batch"])
    w.writeheader()
    w.writerows(rows)
print("Wrote", out)
