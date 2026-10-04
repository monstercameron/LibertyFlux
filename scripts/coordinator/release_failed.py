"""Give back the batches of production lanes that stopped, so new lanes can take what they did not settle.

A lane counts as stopped when status.py reports it as exited-incomplete or not-running. Lanes that are still
running are never touched, and neither is anything launched or written in the last 15 minutes: a lane that was
launched a moment ago has no process yet and looks dead. Runs on every tick.

- No results.json: the whole batch list moves to lists/released/.
- results.json (since brief b8 lanes write it in their first ten minutes, so a dead lane usually has one): the
  functions it settled stay assigned in the lane's list, verified under any checker or deferred with a reason;
  every other function (not reached, not yet run, failed, deferred without a reason, no row at all) is moved to
  lists/released/<lane>.json and goes back to the queue. Until 2026-10-04 such lanes kept their whole batch
  forever. A results.json that cannot be read is left alone and reported.
"""

import json
import os
import re
import shutil
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import scan

# A batch is never released within this long of its launch.
RELEASE_GRACE_MINUTES = 15


def settled(row):
    """True when a result row closes its function for now: verified under any checker, or deferred with a reason."""
    outcome = row.get("outcome")
    return outcome in scan.PASSED or (outcome == "deferred" and bool(str(row.get("reason") or "").strip()))


def entry_address(entry):
    """A batch-list entry's address (`start` for functions, `handler` for natives), or None."""
    try:
        return common.va(entry.get("start") or entry.get("handler") or entry.get("address"))
    except (TypeError, ValueError):
        return None


def split_batch(batch, rows):
    """(keep, release) for a lane's batch list given its result rows. An entry stays when a settled row names its
    address (or, for a native, its name); an entry whose address cannot be read stays too, since nothing proves
    it unsettled. Pure, so the rule is tested without lane folders."""
    done_addresses, done_names = set(), set()
    for row in rows:
        if not isinstance(row, dict) or not settled(row):
            continue
        try:
            done_addresses.add(common.va(row["address"]))
        except (KeyError, TypeError, ValueError):
            pass
        if row.get("name"):
            done_names.add(row["name"])
    keep, release = [], []
    for entry in batch:
        address = entry_address(entry)
        stays = (address is None or address in done_addresses
                 or ("handler" in entry and entry.get("name") in done_names))
        (keep if stays else release).append(entry)
    return keep, release


def release_partial(lists, lane, batch, rows):
    """Move a dead lane's unsettled entries to lists/released/<lane>.json (merged with an earlier release) and
    keep only the settled ones in its list. Returns how many were released; writes nothing when that is 0."""
    keep, release = split_batch(batch, rows)
    if not release:
        return 0
    out = lists / "released" / f"{lane}.json"
    out.parent.mkdir(exist_ok=True)
    earlier = json.loads(out.read_text(encoding="utf-8")) if out.exists() else []
    known = {entry_address(e) for e in earlier}
    merged = earlier + [e for e in release if entry_address(e) not in known]
    out.write_text(json.dumps(merged, indent=1), encoding="utf-8")
    (lists / f"{lane}.json").write_text(json.dumps(keep, indent=1), encoding="utf-8")
    return len(release)


def release_lane(lists, scratch, logs, lane, state, now=None):
    """Release one stopped lane's batch as described at the top. Returns a report string, or None."""
    batch_path = lists / f"{lane}.json"
    if not batch_path.exists():
        return None
    results = scratch / lane / "results.json"
    log = logs / f"{lane}.log"
    newest = max(p.stat().st_mtime for p in (batch_path, log, results) if p.exists())
    if (now or time.time()) - newest < RELEASE_GRACE_MINUTES * 60:
        return None
    if not results.exists():
        (lists / "released").mkdir(exist_ok=True)
        shutil.move(str(batch_path), str(lists / "released" / f"{lane}.json"))
        return f"{lane} ({state})"
    try:
        rows = common.load_rows(json.loads(results.read_text(encoding="utf-8")), "functions", "results")
        batch = json.loads(batch_path.read_text(encoding="utf-8"))
    except (ValueError, OSError):
        return f"{lane} ({state}): results.json or batch list unreadable, left alone"
    released = release_partial(lists, lane, batch, rows)
    return f"{lane} ({state}, {released} unsettled functions)" if released else None


def main():
    root = common.find_root()
    lists = common.lists_dir(root)
    out = subprocess.run([sys.executable, str(root / "scripts" / "status.py"), "--brief"], cwd=root,
                         capture_output=True, text=True, encoding="utf-8").stdout
    released = []
    for state, lane in re.findall(r"^\s+(exited-incomplete|not-running)\s+(r-[nsb]\d+)\s", out, re.M):
        report = release_lane(lists, common.scratch_dir(root), common.logs_dir(root), lane, state)
        if report:
            released.append(report)
    print(f"released {len(released)} batches:", ", ".join(released) or "none")


if __name__ == "__main__":
    main()
