"""Coordinator helper: totals across production lanes' and re-run lanes' results.json files."""

import json
import os
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
from scan import checker_of_lane, short_version

SCRATCH = common.scratch_dir()
LISTS = common.lists_dir()
LABELS = {"n": "native handlers", "s": "small functions", "b": "functions of 250 bytes and more"}


def main():
    outcomes = defaultdict(Counter)   # (kind, checker) -> outcome counts
    reasons = defaultdict(Counter)
    minutes = defaultdict(list)
    lanes = Counter()
    for results in sorted(SCRATCH.glob("r-*/results.json")):
        lane = results.parent.name
        if not (results.parent / "summary.txt").exists():
            continue
        try:
            rows = json.loads(results.read_text(encoding="utf-8"))
        except ValueError:
            print("unreadable:", lane)
            continue
        rows = common.load_rows(rows, "functions", "results")
        key = (lane[2], short_version(checker_of_lane(LISTS, lane)))
        lanes[key] += 1
        for row in rows:
            if not isinstance(row, dict):
                continue
            outcome = row.get("outcome", "?")
            outcomes[key][outcome] += 1
            if outcome == "deferred":
                reasons[key][row.get("reason", "?")] += 1
            if outcome == "verified" and isinstance(row.get("minutes"), (int, float)):
                minutes[key].append(row["minutes"])
    for key in sorted(outcomes):
        done = sum(outcomes[key].values())
        verified = outcomes[key]["verified"]
        mins = sorted(minutes[key])
        median = f", median {mins[len(mins) // 2]:.1f} min each" if mins else ""
        print(f"{LABELS[key[0]]}, checker {key[1]}: {lanes[key]} lanes, {verified} of {done} verified ({100 * verified / max(done, 1):.0f}%){median}")
        if reasons[key]:
            print("    deferred:", dict(reasons[key].most_common(6)))
    rerun = Counter()
    rerun_lanes = 0
    for results in sorted(SCRATCH.glob("a-*/results.json")):
        if not (results.parent / "summary.txt").exists():
            continue
        try:
            rows = json.loads(results.read_text(encoding="utf-8"))
        except ValueError:
            continue
        rows = common.load_rows(rows, "functions", "results")
        rerun_lanes += 1
        for row in rows:
            if not isinstance(row, dict):
                continue
            rerun[row.get("outcome", "?")] += 1
    if rerun_lanes:
        print(f"re-run under checker v2: {rerun_lanes} lanes finished, outcomes {dict(rerun)}")


if __name__ == "__main__":
    main()
