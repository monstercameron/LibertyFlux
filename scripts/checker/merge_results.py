"""Merge proof + regression verdicts into results.json (next to the driver).

Usage: python merge_results.py [DIR] (default DIR: this folder).
Reads DIR/verdicts/*.json and DIR/mutants/*.json, written by one or more
checker2.py runs, and writes the merged summary to DIR/results.json.
"""
import json, glob, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = sys.argv[1] if len(sys.argv) > 1 else HERE
V = os.path.join(OUT, "verdicts")
M = os.path.join(OUT, "mutants")


def load(d, name):
    p = os.path.join(d, name + ".json")
    return json.load(open(p)) if os.path.exists(p) else None


proofs = ["k2_percal1", "k2_percal2", "k2_ind1", "k2_ind2", "k2_dslot",
          "k2_stw1", "k2_stw1b", "k2_ptr1", "k2_ptr1m", "k2_ptr1m_skip",
          "k2_tls1", "k2_tls2", "k2_xmm1", "k2_xmm2", "k2_tail1", "k2_tail2",
          "k2_e1", "k2_e2"]
out = {"proofs": [], "regression": []}
for n in proofs:
    v, m = load(V, n), load(M, n)
    e = {"name": n}
    if v:
        e["correct"] = {"passed": v["passed"], "trials": v["trials"],
                        "fails": v["fails"], "orig_ok": v.get("orig_ok_trials"),
                        "hist": v.get("status_hist"),
                        "coverage": v.get("call_coverage"),
                        "wall_s": v.get("wall_s")}
    if m:
        e["mutant"] = {"passed": m["passed"], "trials": m["trials"],
                       "fails": m["fails"],
                       "first": (m.get("first_mismatch") or {}).get("detail", "")[:160]}
    out["proofs"].append(e)

for f in sorted(glob.glob(os.path.join(V, "*.json"))):
    n = os.path.basename(f)[:-5]
    if n.startswith("k2_"):
        continue
    v = json.load(open(f))
    m = load(M, n)
    e = {"name": n, "correct": {"passed": v["passed"], "trials": v["trials"],
                                "fails": v["fails"]},
         "mutant": ({"passed": m["passed"], "trials": m["trials"], "fails": m["fails"]}
                    if m else None)}
    out["regression"].append(e)

json.dump(out, open(os.path.join(OUT, "results.json"), "w"), indent=1)
np = sum(1 for e in out["proofs"] if (e.get("correct") or {}).get("passed"))
nr = sum(1 for e in out["regression"] if (e.get("correct") or {}).get("passed"))
print("proofs passing: %d/%d; regression: %d/%d" % (np, len(out["proofs"]), nr, len(out["regression"])))
