"""Coordinator helper: totals across production lanes' and re-run lanes' results.json files."""

import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
from scan import PASSED, checker_of_lane, short_version

SCRATCH = common.scratch_dir()
LISTS = common.lists_dir()
LABELS = {"n": "native handlers", "s": "small functions", "b": "functions of 250 bytes and more"}


def tally(lanes):
    """Totals per (lane kind letter, short checker version) from (lane, checker, rows) triples.

    Returns (outcomes, reasons, minutes, lane counts). Every verified form in scan.PASSED counts as verified
    (until 2026-10-04 only the bare `verified` did, so lanes writing `verified_v4` showed as 0% verified).
    """
    outcomes = defaultdict(Counter)   # (kind, checker) -> outcome counts
    reasons = defaultdict(Counter)
    minutes = defaultdict(list)
    lanes_seen = Counter()
    for lane, checker, rows in lanes:
        key = (lane[2:3], checker)
        lanes_seen[key] += 1
        for row in rows:
            if not isinstance(row, dict):
                continue
            outcome = row.get("outcome", "?")
            outcomes[key][outcome] += 1
            if outcome == "deferred":
                reasons[key][row.get("reason", "?")] += 1
            if outcome in PASSED and isinstance(row.get("minutes"), (int, float)):
                minutes[key].append(row["minutes"])
    return outcomes, reasons, minutes, lanes_seen


def verified_in(counter):
    return sum(counter[o] for o in PASSED)


def report_lines(outcomes, reasons, minutes, lanes_seen):
    lines = []
    for key in sorted(outcomes):
        done = sum(outcomes[key].values())
        verified = verified_in(outcomes[key])
        mins = sorted(minutes[key])
        median = f", median {mins[len(mins) // 2]:.1f} min each" if mins else ""
        lines.append(f"{LABELS.get(key[0], 'lanes r-' + key[0])}, checker {key[1]}: {lanes_seen[key]} lanes, "
                     f"{verified} of {done} verified ({100 * verified / max(done, 1):.0f}%){median}")
        if reasons[key]:
            lines.append(f"    deferred: {dict(reasons[key].most_common(6))}")
    return lines


def finished_lanes(pattern):
    """(lane, rows) for finished lanes matching `pattern`; unreadable files are reported and skipped."""
    for results in sorted(SCRATCH.glob(f"{pattern}/results.json")):
        lane = results.parent.name
        if not (results.parent / "summary.txt").exists():
            continue
        try:
            rows = json.loads(results.read_text(encoding="utf-8"))
        except ValueError:
            print("unreadable:", lane)
            continue
        yield lane, common.load_rows(rows, "functions", "results")


def main():
    production = ((lane, short_version(checker_of_lane(LISTS, lane)), rows) for lane, rows in finished_lanes("r-*"))
    for line in report_lines(*tally(production)):
        print(line)
    rerun = Counter()
    rerun_lanes = 0
    for _, rows in finished_lanes("a-*"):
        rerun_lanes += 1
        rerun.update(row.get("outcome", "?") for row in rows if isinstance(row, dict))
    if rerun_lanes:
        print(f"re-run lanes: {rerun_lanes} finished, outcomes {dict(rerun)}")


if __name__ == "__main__":
    main()
