"""Load a function inventory JSON into the queue database.

Usage:
    python import_inventory.py --inventory PATH [--db PATH] [--source NAME]

Reads any inventory the normaliser understands (p0 estimate format or the
richer f-function-inventory / Ghidra format) and upserts it: new addresses
are added as queued, existing rows keep their state, name, kind and claims
and only get fresh metrics. Safe to re-run; the second run is a no-op
apart from refreshing numbers. Prints {"inserted": n, "updated": m}.

This is the forward path when the richer inventory replaces the estimate:
run this script with the new file and the queue keeps all progress.
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
    ap.add_argument("--inventory", required=True,
                    help="Function inventory JSON file")
    ap.add_argument("--db", default=None, help="Queue database path")
    ap.add_argument("--source", default=None,
                    help="Source label (default: inventory file name)")
    ap.add_argument("--addr-form", choices=("rva", "va"), default="rva",
                    help="Address form of bare entry/address keys"
                    " (Ghidra exports: va)")
    args = ap.parse_args(argv)
    with open(args.inventory, "r", encoding="utf-8") as fh:
        data = json.load(fh)
    records, skipped = lfdb.normalise_inventory(data, addr_form=args.addr_form)
    source = args.source or Path(args.inventory).name
    conn = lfdb.connect(args.db)
    try:
        result = lfdb.import_functions(conn, records, source)
    finally:
        conn.close()
    result["source"] = source
    result["skipped"] = skipped
    print(json.dumps(result))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
