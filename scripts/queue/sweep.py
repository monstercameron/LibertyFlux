"""End-of-batch sweep: release stale claims, append to the ledger.

Usage:
    python sweep.py [--db PATH] [--note TEXT]

Releases every claim older than its expiry (functions go back to 'queued'),
then appends one row to the batches ledger table with the current queue
statistics, deferral reasons and per-lane claim counts. Also appends the same
record as one JSON line to .artifacts/db/ledger.jsonl (private mirror for
humans; the table is authoritative). Prints the batch record.
"""

from __future__ import annotations

import argparse
import json
import sqlite3
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--db", default=None)
    ap.add_argument("--note", default=None)
    args = ap.parse_args(argv)
    started = lfdb.now_iso()
    conn = lfdb.connect(args.db)
    try:
        released = lfdb.release_expired(conn)
        counts = lfdb.state_counts(conn)
        reasons = {r["blocker"]: r["n"] for r in conn.execute(
            "SELECT blocker, COUNT(*) n FROM deferrals WHERE active=1"
            " GROUP BY blocker")}
        lanes = {r["lane"]: r["n"] for r in conn.execute(
            "SELECT lane, COUNT(*) n FROM claims GROUP BY lane")}
        attempts = conn.execute("SELECT COUNT(*) n FROM attempts").fetchone()["n"]
        ended = lfdb.now_iso()
        detail = json.dumps({"deferral_reasons": reasons, "active_lanes": lanes,
                             "attempts_total": attempts}, sort_keys=True)
        conn.execute("BEGIN IMMEDIATE")
        try:
            cur = conn.execute(
                "INSERT INTO batches (started_at, ended_at, released_stale,"
                " n_queued, n_claimed, n_accepted, n_deferred, n_excluded,"
                " note, detail) VALUES (?,?,?,?,?,?,?,?,?,?)",
                (started, ended, released, counts["queued"], counts["claimed"],
                 counts["accepted"], counts["deferred"], counts["excluded"],
                 args.note, detail))
            batch_id = cur.lastrowid
            conn.execute("COMMIT")
        except BaseException:
            conn.execute("ROLLBACK")
            raise
    finally:
        conn.close()
    record = {"batch": batch_id, "started_at": started, "ended_at": ended,
              "released_stale": released, "counts": counts,
              "deferral_reasons": reasons, "active_lanes": lanes, "note": args.note}
    ledger = lfdb.repo_root() / ".artifacts" / "db" / "ledger.jsonl"
    if args.db:
        ledger = Path(args.db).parent / "ledger.jsonl"
    with open(ledger, "a", encoding="utf-8") as fh:
        fh.write(json.dumps(record, sort_keys=True) + "\n")
    print(json.dumps(record, indent=1))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
