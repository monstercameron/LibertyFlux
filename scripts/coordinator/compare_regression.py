"""Compare a checker regression run with the tracked verdicts: `passed` must match for every proof and mutant."""

import json
import os
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

ROOT = common.find_root()


def main():
    new = Path(sys.argv[1])
    bad = 0
    for kind in ("verdicts", "mutants"):
        tracked = {p.name: json.loads(p.read_text(encoding="utf-8")) for p in (ROOT / "scripts" / "checker" / kind).glob("*.json")}
        fresh = {p.name: json.loads(p.read_text(encoding="utf-8")) for p in (new / kind).glob("*.json")}
        same = 0
        for name, old in sorted(tracked.items()):
            got = fresh.get(name)
            if got is None:
                print(f"{kind}/{name}: missing from the new run")
                bad += 1
            elif bool(got.get("passed")) != bool(old.get("passed")):
                print(f"{kind}/{name}: tracked passed={old.get('passed')} new passed={got.get('passed')} "
                      f"({str((got.get('first_mismatch') or {}).get('detail'))[:140]})")
                bad += 1
            else:
                same += 1
        extra = sorted(set(fresh) - set(tracked))
        retried = [n for n, v in fresh.items() if v.get("worker_retries")]
        versions = sorted({str(v.get("checker_version")) for v in fresh.values()})
        print(f"{kind}: {same} of {len(tracked)} match; extra in new run {len(extra)}; with retries {retried}; versions {versions}")
    print("MISMATCHES", bad)


if __name__ == "__main__":
    main()
