"""Defer a claimed function with a blocker tag (a normal outcome).

Usage:
    python defer.py --lane LANE --addr 0xRVA --blocker TAG --note TEXT [--db PATH]

TAG must be one of lfdb.BLOCKER_TAGS (see --list). The note must say what is
blocked on; one sentence is enough. The claim is released and the function
becomes 'deferred'; deferred functions are later served to Sonnet lanes.
Use --list to print the fixed tag vocabulary.
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
    ap.add_argument("--lane", required=False, default=None)
    ap.add_argument("--addr", required=False, default=None)
    ap.add_argument("--blocker", required=False, default=None)
    ap.add_argument("--note", required=False, default=None)
    ap.add_argument("--db", default=None)
    ap.add_argument("--list", action="store_true",
                    help="Print blocker tags and exit")
    args = ap.parse_args(argv)
    if args.list:
        for tag in lfdb.BLOCKER_TAGS:
            print(tag)
        return 0
    for req in ("lane", "addr", "blocker", "note"):
        if not getattr(args, req):
            print("missing --%s" % req, file=sys.stderr)
            return 1
    conn = lfdb.connect(args.db)
    try:
        addr = lfdb.resolve_addr(conn, args.addr)
        try:
            res = lfdb.defer_function(conn, args.lane, addr,
                                      args.blocker, args.note)
        except ValueError as exc:
            print("refused: %s" % exc, file=sys.stderr)
            return 1
    finally:
        conn.close()
    res["addr"] = lfdb.rva_hex(res["addr"])
    print(json.dumps(res))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
