"""Record an accepted rewrite. Refuses without a passing checker verdict.

Usage:
    python accept.py --lane LANE --addr 0xRVA --verdict PATH [--db PATH]

PATH is the checker's verdict file, JSON:
    {"function": "0x<rva hex>", "passed": true, "inputs_tested": N,
     "comparisons": [{"name": "...", "passed": true}, ...],
     "checker_version": "..."}

The verdict must pass, name this function, test at least one input and list
at least one comparison, all passing -- otherwise this script exits 1 and the
claim stays with the lane. On success the claim is released, the verdict and
an attempt row are stored, and the function becomes 'accepted'.
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
    ap.add_argument("--lane", required=True)
    ap.add_argument("--addr", required=True, help="Function RVA, hex or int")
    ap.add_argument("--verdict", required=True, help="Checker verdict JSON file")
    ap.add_argument("--db", default=None)
    args = ap.parse_args(argv)
    try:
        verdict = lfdb.load_verdict(args.verdict)
    except (OSError, json.JSONDecodeError) as exc:
        print("cannot read verdict: %s" % exc, file=sys.stderr)
        return 1
    conn = lfdb.connect(args.db)
    try:
        addr = lfdb.resolve_addr(conn, args.addr)
        try:
            res = lfdb.accept_function(conn, args.lane, addr, verdict,
                                       verdict_path=args.verdict)
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
