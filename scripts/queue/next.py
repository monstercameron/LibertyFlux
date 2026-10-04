"""Claim the next easiest functions for a lane (lanes never choose).

Usage:
    python next.py --lane LANE [--n N] [--db PATH]
                   [--subsystem NAME | --class NAME] [--json]
    python next.py --lane LANE --renew 0xADDR [--db PATH]

Claims the N easiest still-queued functions atomically (BEGIN IMMEDIATE +
UNIQUE constraint: two lanes racing for the same function cannot both win).
A claim expires after 2 hours unless renewed. --subsystem serves one
subsystem's queue; --class serves one RTTI class's virtual methods together.
--renew extends one claim by 2 hours (used when an attempt is recorded).

Prints one hex RVA per line (or JSON with --json).
Exit 2 when the filtered queue is empty.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--lane", required=True, help="Lane id, e.g. r-net-3")
    ap.add_argument("--n", type=int, default=1)
    ap.add_argument("--db", default=None)
    ap.add_argument("--subsystem", default=None)
    ap.add_argument("--class", dest="class_name", default=None,
                    help="RTTI class demangled or mangled name")
    ap.add_argument("--renew", default=None,
                    help="Renew the claim on this RVA instead of claiming")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args(argv)
    conn = lfdb.connect(args.db)
    try:
        if args.renew is not None:
            res = lfdb.renew_claim(conn, args.lane,
                                   lfdb.resolve_addr(conn, args.renew))
            print(json.dumps(res))
            return 0
        rows = lfdb.claim_next(conn, args.lane, n=args.n,
                               subsystem=args.subsystem,
                               class_name=args.class_name)
    finally:
        conn.close()
    if not rows:
        print("queue empty for this filter", file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps(
            [{"addr": lfdb.rva_hex(r["addr"]), "difficulty": r["difficulty"],
              "size_bytes": r["size_bytes"], "n_insns": r["n_insns"],
              "name": r["name"], "subsystem": r["subsystem"]} for r in rows],
            indent=1))
    else:
        for r in rows:
            print(lfdb.rva_hex(r["addr"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
