"""Compare a checker regression run with the reference verdicts: `passed` must match for every proof and mutant.

Usage: python compare_regression.py <new run folder> [--tracked <reference folder>]

Both folders hold `verdicts/` and `mutants/` with one JSON verdict per contract. The reference defaults to
scripts/checker/, whose verdicts/ folder is ignored by git (it exists only where the checker has run), so on any
other machine pass --tracked explicitly.
"""

import json
import os
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

ROOT = common.find_root()
KINDS = ("verdicts", "mutants")


def load_folder(folder, kind):
    return {p.name: json.loads(p.read_text(encoding="utf-8")) for p in (Path(folder) / kind).glob("*.json")}


def compare(tracked, fresh, kind):
    """(report lines, mismatches) for one kind. A reference verdict missing from the new run, or one whose
    `passed` differs, is a mismatch; extra verdicts in the new run are listed but are not mismatches."""
    lines, bad, same = [], 0, 0
    for name, old in sorted(tracked.items()):
        got = fresh.get(name)
        if got is None:
            lines.append(f"{kind}/{name}: missing from the new run")
            bad += 1
        elif bool(got.get("passed")) != bool(old.get("passed")):
            lines.append(f"{kind}/{name}: tracked passed={old.get('passed')} new passed={got.get('passed')} "
                         f"({str((got.get('first_mismatch') or {}).get('detail'))[:140]})")
            bad += 1
        else:
            same += 1
    extra = sorted(set(fresh) - set(tracked))
    retried = sorted(n for n, v in fresh.items() if v.get("worker_retries"))
    versions = sorted({str(v.get("checker_version")) for v in fresh.values()})
    lines.append(f"{kind}: {same} of {len(tracked)} match; extra in new run {len(extra)}; with retries {retried}; versions {versions}")
    return lines, bad


def parse_args(argv):
    """(new folder, reference folder) from the command line."""
    args, reference = list(argv), ROOT / "scripts" / "checker"
    if "--tracked" in args:
        i = args.index("--tracked")
        if i + 1 >= len(args):
            raise SystemExit("--tracked needs a folder")
        reference = Path(args[i + 1])
        del args[i:i + 2]
    if len(args) != 1:
        raise SystemExit(__doc__.strip().splitlines()[2])
    return Path(args[0]), Path(reference)


def main(argv=None):
    new, reference = parse_args(sys.argv[1:] if argv is None else argv)
    bad = 0
    for kind in KINDS:
        tracked = load_folder(reference, kind)
        if not tracked:
            print(f"{kind}: no reference verdicts under {reference.name}/{kind}; pass --tracked")
        lines, mismatches = compare(tracked, load_folder(new, kind), kind)
        print("\n".join(lines))
        bad += mismatches
    print("MISMATCHES", bad)
    return bad


if __name__ == "__main__":
    main()
