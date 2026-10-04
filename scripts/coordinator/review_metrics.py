"""Numbers for the hourly critical review: what the last hour produced, where time and memory went, what was
deferred and why. Appends a snapshot to review_history.json so each review can be compared with the one before.

Usage: python review_metrics.py [minutes, default 60]
"""

import collections
import datetime
import json
import os
import re
import statistics
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import ledger


def main():
    root = common.find_root()
    coord = common.coord_dir(root)
    scratch = common.scratch_dir(root)
    lists = common.lists_dir(root)
    window = int(sys.argv[1]) if len(sys.argv) > 1 else 60
    since = time.time() - window * 60
    now = datetime.datetime.now()
    va = common.va

    progress = json.loads((root / "docs" / "data" / "progress.json").read_text(encoding="utf-8"))
    stages = progress["stages"]
    functions = json.loads((scratch / "f-boundaries" / "functions_v2.json").read_text(encoding="utf-8"))
    functions = functions if isinstance(functions, list) else functions.get("functions") or list(functions.values())
    size = {va(f["start"]): int(f.get("size") or 0) for f in functions if f.get("kind") not in ("library", "runtime")}
    # The tracked verified tree, through the function ledger (it read the untracked rewrites/pending junction
    # before 2026-10-04, which exists only on the coordinator's machine).
    index = ledger.load_index(root / "rewrites" / "verified")
    from scan import MODERN

    done = {va(e["address"]) for e in index if e.get("checker") in MODERN} & set(size)
    done_bytes = sum(size[a] for a in done)
    total_bytes = sum(size.values())

    snapshot = {"at": now.strftime("%Y-%m-%d %H:%M"), "named": stages["named"], "rewritten": stages["rewritten"],
                "verified": stages["verified"], "verified_bytes": done_bytes, "game": progress["functions"]["game"]}
    history_path = coord / "review_history.json"
    coord.mkdir(parents=True, exist_ok=True)
    history = json.loads(history_path.read_text(encoding="utf-8")) if history_path.exists() else []
    previous = history[-1] if history else None
    history.append(snapshot)
    history_path.write_text(json.dumps(history, indent=1), encoding="utf-8")

    print(f"REVIEW at {snapshot['at']} (window {window} min)")
    print(f"totals: game {snapshot['game']}, named {stages['named']}, rewritten {stages['rewritten']}, verified {stages['verified']} "
          f"({100 * stages['verified'] / snapshot['game']:.1f}% of functions, {100 * done_bytes / total_bytes:.1f}% of bytes)")
    if previous:
        t0 = datetime.datetime.strptime(previous["at"], "%Y-%m-%d %H:%M")
        hours = max((now - t0).total_seconds() / 3600, 1e-6)
        print(f"since the last review ({previous['at']}, {hours:.2f} h): named +{stages['named'] - previous['named']}, "
              f"rewritten +{stages['rewritten'] - previous['rewritten']}, verified +{stages['verified'] - previous['verified']} "
              f"({(stages['verified'] - previous['verified']) / hours:.0f}/h), verified bytes +{done_bytes - previous['verified_bytes']} "
              f"({(done_bytes - previous['verified_bytes']) / hours:.0f}/h)")
        left = total_bytes - done_bytes
        rate = (done_bytes - previous["verified_bytes"]) / hours
        if rate > 0:
            print(f"at this hour's byte rate the remaining {left} bytes take {left / rate:.0f} hours ({left / rate / 24:.1f} days)")

    kinds = collections.defaultdict(lambda: {"lanes": 0, "verified": 0, "rows": 0, "minutes": [], "reasons": collections.Counter(), "bytes": 0})
    finished = 0
    for results in sorted(scratch.glob("r-*/results.json")):
        if results.stat().st_mtime < since or not (results.parent / "summary.txt").exists():
            continue
        try:
            rows = json.loads(results.read_text(encoding="utf-8"))
        except (ValueError, OSError):
            continue
        rows = common.load_rows(rows, "functions", "results")
        lane = results.parent.name
        kind = {"n": "natives", "s": "small", "b": "large"}.get(lane[2], lane[2])
        k = kinds[kind]
        k["lanes"] += 1
        finished += 1
        for row in rows:
            if not isinstance(row, dict):
                continue
            k["rows"] += 1
            if str(row.get("outcome")).startswith("verified"):
                k["verified"] += 1
                try:
                    k["bytes"] += size.get(va(row["address"]), 0)
                except (KeyError, ValueError, TypeError):
                    pass
                if isinstance(row.get("minutes"), (int, float)):
                    k["minutes"].append(row["minutes"])
            else:
                k["reasons"][str(row.get("reason") or row.get("outcome"))] += 1
    print(f"production lanes finished in the window: {finished}")
    for kind, k in sorted(kinds.items()):
        med = statistics.median(k["minutes"]) if k["minutes"] else float("nan")
        print(f"  {kind}: {k['lanes']} lanes, {k['verified']} of {k['rows']} verified ({100 * k['verified'] / max(1, k['rows']):.0f}%), "
              f"{k['bytes']} bytes, median {med:.1f} min per verified function (lane-reported); not verified: {dict(k['reasons'].most_common(5))}")

    named_lanes = [p for p in scratch.glob("nm-*/names.json") if p.stat().st_mtime >= since]
    if named_lanes:
        total = good = 0
        for p in named_lanes:
            rows = json.loads(p.read_text(encoding="utf-8"))
            rows = rows if isinstance(rows, list) else rows.get("names") or []
            total += len(rows)
            good += sum(1 for r in rows if isinstance(r, dict) and r.get("name") and r.get("confidence") in ("high", "medium"))
        print(f"naming lanes finished in the window: {len(named_lanes)}, {good} of {total} named")

    sup = common.logs_dir(root) / "supervisor.log"
    if sup.exists():
        lanes, waiting, lines = [], 0, 0
        for line in sup.read_text(encoding="utf-8", errors="replace").splitlines()[-200:]:
            m = re.search(r"lanes (\d+)/(\d+), free ([\d.]+) GB, commit headroom ([\d.]+) GB, ([^;]+)", line)
            if m:
                lines += 1
                lanes.append(int(m.group(1)))
                waiting += m.group(5).startswith("waiting")
        if lines:
            print(f"supervisor (last {lines} passes): mean lanes {statistics.mean(lanes):.1f}, waiting for memory in {100 * waiting / lines:.0f}% of passes")

    logs = common.logs_dir(root)
    died = []
    for log in logs.glob("*.log"):
        if log.stat().st_mtime < since:
            continue
        tail = log.read_text(encoding="utf-8", errors="replace").splitlines()[-1:]
        if tail and re.search(r"lane exited with code (?!0\b)\d+", tail[0]):
            died.append(log.stem)
    print(f"lanes that exited with an error in the window: {len(died)} {died[:12]}")

    handed = set()
    for path in lists.glob("r-*.json"):
        try:
            for row in json.loads(path.read_text(encoding="utf-8")):
                a = row.get("a") or row.get("address") or row.get("handler")
                if a:
                    handed.add(va(a))
        except (ValueError, OSError, AttributeError):
            continue
    print(f"functions handed to production lanes so far: {len(handed)}; verified among all game functions by size: "
          + ", ".join(f"{lo}-{hi if hi < 10 ** 9 else 'up'} B {sum(1 for a in done if lo <= size[a] < hi)}/{sum(1 for s in size.values() if lo <= s < hi)}"
                      for lo, hi in ((0, 64), (64, 250), (250, 1000), (1000, 4000), (4000, 10 ** 9))))


if __name__ == "__main__":
    main()
