"""Simulation: N lanes claim/accept/defer M functions with random outcomes.

Usage:
    python simulate.py [--db PATH] [--inventory PATH] [--lanes 6] [--n 500]
                       [--seed 1] [--accept-p 0.7]

Builds a fresh queue from the real inventory, then runs LANES threads that
loop {claim 1, accept (verdict generated locally) or defer (random blocker)}
until N functions are decided. Asserts: no function is ever claimed twice
(a shared ever-claimed set guarded by a lock would catch a race the UNIQUE
constraint missed) and the final statistics add up
(queued+claimed+accepted+deferred+excluded == total, no lingering claims).
Prints the outcome counts and the assertion results.
"""

from __future__ import annotations

import argparse
import json
import random
import sys
import threading
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb


def build_fresh(db: str, inventory: str) -> dict:
    Path(db).unlink(missing_ok=True)
    for suffix in ("-wal", "-shm"):
        Path(db + suffix).unlink(missing_ok=True)
    with open(inventory, "r", encoding="utf-8") as fh:
        records, _ = lfdb.normalise_inventory(json.load(fh))
    conn = lfdb.connect(db)
    try:
        # Mirror queue_build defaults: encrypted range excluded, no library
        # ranges here would need libraries.json -- load it if present.
        res = lfdb.import_functions(conn, records, Path(inventory).name)
        conn.execute("BEGIN IMMEDIATE")
        try:
            conn.execute(
                "UPDATE functions SET kind='game' WHERE kind='unknown'")
            conn.execute(
                "UPDATE functions SET kind='encrypted', state='excluded'"
                " WHERE addr>=0x1000 AND addr<0xfb000")
            conn.execute("COMMIT")
        except BaseException:
            conn.execute("ROLLBACK")
            raise
    finally:
        conn.close()
    return res


def worker(db: str, lane: str, rng: random.Random, accept_p: float,
           decided: list, ever_claimed: set, lock: threading.Lock,
           inflight: list, stop_after: int, errors: list) -> None:
    conn = lfdb.connect(db)
    try:
        while True:
            with lock:
                if len(decided) + inflight[0] >= stop_after:
                    return
                inflight[0] += 1  # reserve a slot; released below
            try:
                rows = lfdb.claim_next(conn, lane, n=1)
            except Exception as exc:  # noqa: BLE001 - recorded, fails the run
                with lock:
                    inflight[0] -= 1
                    errors.append("claim: %r" % exc)
                return
            if not rows:
                with lock:
                    inflight[0] -= 1
                return
            addr = rows[0]["addr"]
            with lock:
                if addr in ever_claimed:
                    inflight[0] -= 1
                    errors.append("DOUBLE CLAIM on %s" % lfdb.rva_hex(addr))
                    return
                ever_claimed.add(addr)
            try:
                if rng.random() < accept_p:
                    verdict = {"function": lfdb.rva_hex(addr), "passed": True,
                               "inputs_tested": rng.randint(1, 50),
                               "comparisons": [{"name": "emu", "passed": True}],
                               "checker_version": "sim"}
                    lfdb.accept_function(conn, lane, addr, verdict)
                    outcome = "accepted"
                else:
                    lfdb.defer_function(
                        conn, lane, addr,
                        rng.choice(lfdb.BLOCKER_TAGS), "simulation note")
                    outcome = "deferred"
            except Exception as exc:  # noqa: BLE001
                with lock:
                    inflight[0] -= 1
                    errors.append("decide: %r" % exc)
                return
            with lock:
                inflight[0] -= 1
                decided.append((addr, outcome))
    finally:
        conn.close()


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--db", default=None)
    ap.add_argument("--inventory", default=None)
    ap.add_argument("--lanes", type=int, default=6)
    ap.add_argument("--n", type=int, default=500)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--accept-p", type=float, default=0.7)
    args = ap.parse_args(argv)
    root = lfdb.repo_root()
    db = args.db or str(root / ".artifacts" / "scratch" / "h-queue"
                        / "sim.db")
    inv = args.inventory or str(
        root / ".artifacts" / "scratch" / "p0-function-estimate"
        / "functions_estimate.json")
    res = build_fresh(db, inv)
    decided: list = []
    ever_claimed: set = set()
    errors: list = []
    lock = threading.Lock()
    inflight = [0]
    threads = [threading.Thread(
        target=worker,
        args=(db, "sim-%d" % i, random.Random(args.seed + i),
              args.accept_p, decided, ever_claimed, lock, inflight,
              args.n, errors))
        for i in range(args.lanes)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    conn = lfdb.connect(db)
    try:
        counts = lfdb.state_counts(conn)
        total = sum(counts.values())
        rows = conn.execute("SELECT COUNT(*) n FROM functions").fetchone()["n"]
        dangling = conn.execute("SELECT COUNT(*) n FROM claims").fetchone()["n"]
    finally:
        conn.close()
    n_acc = sum(1 for _, o in decided if o == "accepted")
    n_def = sum(1 for _, o in decided if o == "deferred")
    ok_double = not errors and len(ever_claimed) == len(decided)
    ok_add = (total == rows and dangling == 0
              and counts["accepted"] == n_acc
              and counts["deferred"] == n_def)
    print(json.dumps({"imported": res, "decided": len(decided),
                      "accepted": n_acc, "deferred": n_def,
                      "counts": counts, "errors": errors,
                      "no_double_claim": ok_double,
                      "stats_add_up": ok_add}, indent=1))
    return 0 if (ok_double and ok_add and len(decided) == args.n) else 1


if __name__ == "__main__":
    raise SystemExit(main())
