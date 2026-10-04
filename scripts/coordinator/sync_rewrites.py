"""Copy lanes' rewrites that passed the checker into the tracked tree under rewrites/verified/.

Sources: production lanes (r-n*, r-s*, r-b*) and re-run lanes (a-*). A production lane counts as checker
version 2 when lists/<lane>.v2 exists; re-run lanes are always version 2 and replace a version 1 copy.
Only functions recorded as verified whose rewrite file exists are copied. Every file is scanned first:
anything that looks like disassembly, a byte dump, a machine path or inline assembly is left out and listed.
Lane-specific runtime imports are dropped, since the shared runtime replaces them at assembly.

Usage: python sync_rewrites.py          copy and report
       python sync_rewrites.py --commit copy, then commit the new and changed files as one wave
"""

import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import scan

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
LISTS = common.lists_dir(ROOT)
DEST = ROOT / "rewrites" / "verified"


def main():
    index_path = DEST / "index.json"
    index = {e["address"]: e for e in json.loads(index_path.read_text(encoding="utf-8"))} if index_path.exists() else {}
    added, upgraded, skipped = [], [], []
    sources = sorted(SCRATCH.glob("r-*/results.json")) + sorted(SCRATCH.glob("a-*/results.json"))
    for results in sources:
        lane = results.parent.name
        if not (results.parent / "summary.txt").exists():
            continue  # lane still running
        try:
            rows = json.loads(results.read_text(encoding="utf-8"))
        except ValueError:
            continue
        rows = common.load_rows(rows, "functions", "results")
        version = scan.checker_of_lane(LISTS, lane)
        for row in rows:
            if not isinstance(row, dict):
                continue
            try:
                address = common.va(row["address"])
            except (KeyError, ValueError, TypeError):
                continue
            key = f"0x{address:08X}"
            source = results.parent / "out" / "rewrites" / f"fn_{address:08x}.rs"
            if row.get("outcome") not in scan.PASSED or not source.exists():
                continue
            known = index.get(key)
            if known and scan.RANK[version] <= scan.RANK.get(known["checker"], 1):
                continue
            text = source.read_text(encoding="utf-8", errors="replace").replace("\r\n", "\n")
            text = scan.strip_lane_imports(text)
            reason = scan.scan_text(text)
            if reason:
                skipped.append((lane, source.name, reason))
                continue
            origin = str(row.get("source_lane") or lane)
            kind = known["kind"] + "s" if known else ("natives" if origin.startswith("r-n") else "functions")
            target = DEST / kind / source.name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(text.encode("utf-8"))
            index[key] = {"address": key, "name": row.get("name") or (known or {}).get("name"), "kind": kind[:-1],
                          "file": f"{kind}/{source.name}", "trials": row.get("trials"), "checker": version, "lane": lane}
            (upgraded if known else added).append(key)
    DEST.mkdir(parents=True, exist_ok=True)
    index_path.write_bytes((json.dumps(sorted(index.values(), key=lambda e: e["address"]), indent=1) + "\n").encode("utf-8"))
    v2 = sum(1 for e in index.values() if e["checker"] in scan.MODERN)
    v3 = sum(1 for e in index.values() if e["checker"] == "version 3")
    print(f"added {len(added)}; re-checked under a later checker {len(upgraded)}; total in rewrites/verified {len(index)} "
          f"({v2} under version 2 or 3, {v3} under version 3); left out by the scan {len(skipped)}")
    if "--commit" not in sys.argv:
        return
    # Whatever is in the tree but not yet committed is this wave, whether it was copied now or by an earlier run.
    status = subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all", "--", "rewrites/verified"],
                            capture_output=True, text=True).stdout.splitlines()
    new = [line for line in status if line.strip().endswith(".rs") and line.startswith("??")]
    changed = [line for line in status if line.strip().endswith(".rs") and not line.startswith("??")]
    if not new and not changed:
        return
    natives = sum(1 for line in new if "/natives/" in line)
    changes = []
    if new:
        changes.append(f"{len(new)} rewrites added under rewrites/verified: {natives} script native handlers and {len(new) - natives} other functions.")
    if changed:
        changes.append(f"{len(changed)} rewrites were re-run under a later version of the checker and replaced by the version that passed it.")
    changes.append(f"The tree now holds {len(index)} rewrites, {v2} of them passed under checker version 2; index.json records the address, name, trial count and checker version of each.")
    changes.append("They are in the checker's harness form (calls through numbered slots, globals by original address) and are not yet linked into the workspace crates.")
    if skipped:
        changes.append(f"{len(skipped)} rewrites are held back by the publication scan (comments that read like disassembly or byte dumps) and stay in scratch until cleaned.")
    title = (f"Rewrites: {len(new)} more functions pass the checker ({len(index)} in total)" if new
             else f"Rewrites: {len(changed)} functions re-checked under checker version 2")
    entry = COORD / "entry-rewrites.json"
    entry.write_text(json.dumps({"title": title, "summary": "Rust rewrites of original game functions, each proven against the original by the checker.",
                                 "changes": changes, "files": ["rewrites/verified"]}), encoding="utf-8")
    result = subprocess.run([sys.executable, str(Path(__file__).resolve().parent / "commit_one.py"), str(entry)], cwd=ROOT,
                            env=dict(os.environ), capture_output=True, text=True, encoding="utf-8")
    print((result.stdout + result.stderr).strip())


if __name__ == "__main__":
    main()
