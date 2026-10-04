"""Deterministic text export of the publishable queue tables.

Usage:
    python export.py [--db PATH] --out DIR

Writes DIR/functions.json (one object per function: address, size, kind,
subsystem, name, name_confidence, state, class, slot), DIR/batches.json
(ledger rows) and DIR/deferral_reasons.json (active blocker counts).
Sorted by address, keys sorted, indent 1, LF newlines: re-running on an
unchanged database produces byte-identical files, so git diffs are
meaningful. This export is what the repository tracks; the database
itself is private and never committed.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb


def dump(path: Path, obj) -> None:
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        json.dump(obj, fh, indent=1, sort_keys=True, ensure_ascii=True)
        fh.write("\n")


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--db", default=None)
    ap.add_argument("--out", required=True, help="Output directory")
    args = ap.parse_args(argv)
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    conn = lfdb.connect(args.db)
    try:
        funcs = []
        for r in conn.execute(
                "SELECT addr, size_bytes, kind, subsystem, name,"
                " name_confidence, state FROM functions ORDER BY addr"):
            slots = lfdb.class_slots_for(conn, r["addr"])
            first = slots[0] if slots else {}
            funcs.append({
                "address": lfdb.rva_hex(r["addr"]),
                "size": r["size_bytes"],
                "kind": r["kind"],
                "subsystem": r["subsystem"],
                "name": r["name"],
                "name_confidence": r["name_confidence"],
                "state": r["state"],
                "class": first.get("demangled"),
                "slot": first.get("slot"),
            })
        batches = [dict(r) for r in conn.execute(
            "SELECT * FROM batches ORDER BY id")]
        reasons = {r["blocker"]: r["n"] for r in conn.execute(
            "SELECT blocker, COUNT(*) n FROM deferrals WHERE active=1"
            " GROUP BY blocker ORDER BY blocker")}
    finally:
        conn.close()
    dump(out / "functions.json", funcs)
    dump(out / "batches.json", batches)
    dump(out / "deferral_reasons.json", reasons)
    print(json.dumps({"dir": str(out), "functions": len(funcs),
                      "batches": len(batches)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
