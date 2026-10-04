"""Queue statistics, printed and in the site progress.json shape.

Usage:
    python stats.py [--db PATH] [--out PATH] [--json]

Prints counts by state (queued, claimed, accepted, deferred, excluded) and,
with --out, writes a JSON file shaped for the site's docs/data/progress.json
consumer:
    {"functions": {"total": T, "library": L, "game": G},
     "stages": {"identified": G, "named": N, "rewritten": R, "verified": V}}

Mapping (documented, coordinator owns the final wiring):
  total      = all function rows except kind 'encrypted'
  library    = kinds library + runtime (+ stub, which is launcher code)
  game       = total - library
  identified = game rows in the database
  named      = game rows with a non-null name
  rewritten  = accepted rows (a rewrite exists only once the checker passed;
               attempt tracking from the checker lane refines this later)
  verified   = accepted rows (checker verdict on file)

So rewritten == verified for now; both come from machine records, never
estimates. --json prints the same object instead of the human table.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb


def progress_shape(conn) -> dict:
    kinds = {r["kind"]: r["n"] for r in conn.execute(
        "SELECT kind, COUNT(*) n FROM functions GROUP BY kind")}
    total = sum(n for k, n in kinds.items() if k != "encrypted")
    library = kinds.get("library", 0) + kinds.get("runtime", 0) + kinds.get("stub", 0)
    game = total - library
    named = conn.execute(
        "SELECT COUNT(*) n FROM functions WHERE kind='game' AND name IS NOT NULL"
    ).fetchone()["n"]
    verified = conn.execute(
        "SELECT COUNT(*) n FROM functions WHERE kind='game' AND state='accepted'"
    ).fetchone()["n"]
    return {"functions": {"total": total, "library": library, "game": game},
            "stages": {"identified": game, "named": named,
                       "rewritten": verified, "verified": verified}}


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--db", default=None)
    ap.add_argument("--out", default=None, help="Write progress-shaped JSON here")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)
    conn = lfdb.connect(args.db)
    try:
        counts = lfdb.state_counts(conn)
        shape = progress_shape(conn)
        shape["by_state"] = counts
        shape["generated_at"] = lfdb.now_iso()
    finally:
        conn.close()
    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            json.dump(shape, fh, indent=1, sort_keys=True)
            fh.write("\n")
    if args.json:
        print(json.dumps(shape, indent=1, sort_keys=True))
    else:
        total = sum(counts.values())
        for state in lfdb.STATES:
            print("%-9s %7d" % (state, counts[state]))
        print("%-9s %7d" % ("total", total))
        f, s = shape["functions"], shape["stages"]
        print("game=%d library=%d identified=%d named=%d rewritten=%d verified=%d"
              % (f["game"], f["library"], s["identified"], s["named"],
                 s["rewritten"], s["verified"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
