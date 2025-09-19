import csv
import json
import pathlib
import statistics as stats

run = sorted(p for p in pathlib.Path("artifacts").glob("20*") if p.is_dir())[-1]
rows = []
for p in sorted((run / "transcripts").glob("round_*.json")):
    j = json.loads(p.read_text())
    prs = [c.get("prove_ms", 0.0) for c in j.get("clients", [])]
    vrs = [c.get("verify_ms", 0.0) for c in j.get("clients", [])]
    if prs and vrs:
        rows.append(
            {
                "round": j["round"],
                "prove_p50": round(stats.median(prs), 2),
                "verify_p50": round(stats.median(vrs), 2),
                "prove_p90": round(sorted(prs)[int(0.9 * (len(prs) - 1))], 2),
                "verify_p90": round(sorted(vrs)[int(0.9 * (len(vrs) - 1))], 2),
            }
        )
out = run / "tables"
out.mkdir(parents=True, exist_ok=True)
with (out / "timings.csv").open("w", newline="") as f:
    w = csv.DictWriter(
        f, fieldnames=["round", "prove_p50", "verify_p50", "prove_p90", "verify_p90"]
    )
    w.writeheader()
    w.writerows(rows)
print("Wrote", out / "timings.csv")
