"""Copy lanes' rewrites that passed the checker into the tracked tree under rewrites/verified/.

Sources: production lanes (r-n*, r-s*, r-b*) and re-run lanes (a-*). A production lane counts as checker
version 2 when lists/<lane>.v2 exists; re-run lanes are always version 2 and replace a version 1 copy.
Only functions recorded as verified whose rewrite file exists are copied. Every file is scanned first:
anything that looks like disassembly, a byte dump, a machine path or inline assembly is left out and listed.
Lane-specific runtime imports are dropped, since the shared runtime replaces them at assembly.

Usage: python sync_rewrites.py            copy and report
       python sync_rewrites.py --commit   copy, then commit the new and changed files as one wave
Flags the coordinator turns on when it decides to (both off by default, so a tick behaves as before):
       --demote    an indexed rewrite that a lane later recorded as failed or deferred under a LATER checker version
                   than the one it passed is moved to rewrites/unverified/ with that lane's reason, and stays there
                   until it passes again under at least that version (an older pass cannot bring it back)
       --validate  check every lane's results.json rows against lane_results.schema.json and print a warning
                   line per lane with problems; never blocks an import
"""

import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import lane_results
import scan

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
LISTS = common.lists_dir(ROOT)
DEST = ROOT / "rewrites" / "verified"
UNVERIFIED = ROOT / "rewrites" / "unverified"
DEMOTE_FLAG = "--demote"
VALIDATE_FLAG = "--validate"


def judged_against(row):
    """True when a row records a function as failed or deferred: a judgement, unlike not_reached or not yet run."""
    outcome = str(row.get("outcome") or "")
    return outcome == "deferred" or outcome.startswith("failed")


def note_against(against, key, version, lane, row):
    """Remember the judgement against `key` made under the latest checker version seen so far."""
    if key not in against or scan.RANK[version] > scan.RANK[against[key][0]]:
        against[key] = (version, lane, row)


def demotion_holds(unverified, key, version):
    """True when `key` was demoted under a later checker version than `version`, so a pass under `version` must
    not bring it back: the lane whose old pass was demoted is still in scratch and is read on every run."""
    entry = unverified.get(key)
    return bool(entry and entry.get("demoted_from") and scan.RANK[version] < scan.RANK.get(entry.get("checker"), 1))


def lift_demotion(unverified, key, unverified_dir):
    """`key` passed again under at least the demoting version: drop its demotion record and the demoted copy."""
    entry = unverified.get(key)
    if entry and entry.get("demoted_from"):
        (unverified_dir / entry["file"]).unlink(missing_ok=True)
        del unverified[key]


def demote(index, unverified, against, verified_dir, unverified_dir):
    """Move each indexed entry that a later checker version judged against into the unverified tree.

    `against` maps an index key to (version, lane, row) from `note_against`. An entry is demoted only when that
    version ranks above the one it passed under; a judgement under the same or an older version leaves it
    alone (a pass under a checker is proof, and a deferral under the same one may just mean a lane gave up).
    The rewrite file is moved, not deleted, to rewrites/unverified/fn_<address>.rs, and the unverified index
    records the lane's outcome and reason, the demoting checker version and what the verified entry said.
    A demoted copy replaces any unverified copy of the same address. An entry whose file is missing is not
    demoted. Returns (demoted keys, keys left because the file is missing).
    """
    demoted, missing = [], []
    for key, (version, lane, row) in sorted(against.items()):
        known = index.get(key)
        if not known or scan.RANK[version] <= scan.RANK.get(known.get("checker"), 1):
            continue
        source = verified_dir / known["file"]
        if not source.exists():
            missing.append(key)
            continue
        name = f"fn_{common.va(key):08x}.rs"
        unverified_dir.mkdir(parents=True, exist_ok=True)
        os.replace(source, unverified_dir / name)
        outcome = row.get("outcome")
        reason = str(row.get("reason") or "").strip() or str(outcome)
        unverified[key] = {"address": key, "name": known.get("name"), "file": name, "lane": lane, "outcome": outcome,
                           "reason": reason, "checker": version,
                           "detail": f"demoted: {outcome} under checker {version} in lane {lane}, after passing under "
                                     f"{known.get('checker')} in lane {known.get('lane')}",
                           "demoted_from": {"checker": known.get("checker"), "lane": known.get("lane"),
                                            "file": known["file"], "trials": known.get("trials")}}
        del index[key]
        demoted.append(key)
    return demoted, missing


def write_index(path, entries):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes((json.dumps(sorted(entries.values(), key=lambda e: e["address"]), indent=1) + "\n").encode("utf-8"))


def main():
    demoting, validating = DEMOTE_FLAG in sys.argv, VALIDATE_FLAG in sys.argv
    index_path = DEST / "index.json"
    index = {e["address"]: e for e in json.loads(index_path.read_text(encoding="utf-8"))} if index_path.exists() else {}
    unverified_path = UNVERIFIED / "index.json"
    unverified = {e["address"]: e for e in json.loads(unverified_path.read_text(encoding="utf-8"))} if demoting and unverified_path.exists() else {}
    unverified_before = json.dumps(unverified, sort_keys=True)
    against = {}
    added, upgraded, skipped, held = [], [], [], []
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
        if validating:
            warning = lane_results.summarise(lane, lane_results.validate_rows(rows))
            if warning:
                print(warning)
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
            if demoting and judged_against(row):
                note_against(against, key, version, lane, row)
            if row.get("outcome") not in scan.PASSED:
                continue
            proof_review = row.get("proof_review")
            if isinstance(proof_review, dict) and proof_review.get("promotion") == "held":
                reason = str(proof_review.get("reason") or "proof review hold")
                review = proof_review.get("review")
                followup = proof_review.get("independent_followup")
                if not review and isinstance(followup, dict):
                    review = followup.get("review")
                held.append((lane, key, reason, str(review or "")))
                continue
            if not source.exists():
                continue
            known = index.get(key)
            if known and scan.RANK[version] <= scan.RANK.get(known["checker"], 1):
                continue
            if demoting and demotion_holds(unverified, key, version):
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
            if demoting:
                lift_demotion(unverified, key, UNVERIFIED)
    demoted, missing = demote(index, unverified, against, DEST, UNVERIFIED) if demoting else ([], [])
    write_index(index_path, index)
    if demoting and json.dumps(unverified, sort_keys=True) != unverified_before:
        write_index(unverified_path, unverified)
    v2 = sum(1 for e in index.values() if e["checker"] in scan.MODERN)
    v3 = sum(1 for e in index.values() if e["checker"] == "version 3")
    print(f"added {len(added)}; re-checked under a later checker {len(upgraded)}; total in rewrites/verified {len(index)} "
          f"({v2} under version 2 or later, {v3} under version 3); left out by the scan {len(skipped)}")
    if held:
        print(f"held from sync import by proof_review.promotion=held: {len(held)} passed row(s)")
        for lane, key, reason, review in held:
            detail = f"  {lane} {key}: {reason}"
            if review:
                detail += f" [review: {review}]"
            print(detail)
    if demoting:
        print(f"demoted to rewrites/unverified by a later checker's failure or deferral {len(demoted)}"
              + (f" ({', '.join(demoted[:8])})" if demoted else "")
              + (f"; not demoted because the file is missing {len(missing)}" if missing else ""))
    if "--commit" not in sys.argv:
        return
    # Whatever is in the tree but not yet committed is this wave, whether it was copied now or by an earlier run.
    status = subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all", "--", "rewrites/verified"],
                            capture_output=True, text=True).stdout.splitlines()
    new = [line for line in status if line.strip().endswith(".rs") and line.startswith("??")]
    gone = [line for line in status if line.strip().endswith(".rs") and "D" in line[:2]]
    changed = [line for line in status if line.strip().endswith(".rs") and not line.startswith("??") and line not in gone]
    if not new and not changed and not gone:
        return
    # Demotions and lifted demotions change the unverified tree too; it goes into the same commit only then.
    unverified_moved = demoting and bool(subprocess.run(
        ["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=all", "--", "rewrites/unverified"],
        capture_output=True, text=True).stdout.strip())
    natives = sum(1 for line in new if "/natives/" in line)
    changes = []
    if new:
        changes.append(f"{len(new)} rewrites added under rewrites/verified: {natives} script native handlers and {len(new) - natives} other functions.")
    if changed:
        changes.append(f"{len(changed)} rewrites were re-run under a later version of the checker and replaced by the version that passed it.")
    if gone:
        changes.append(f"{len(gone)} rewrites were moved to rewrites/unverified because a lane recorded them as failed or deferred under a later "
                       "checker version than the one they had passed; the verified count goes down by those that were game functions, and "
                       "rewrites/unverified/index.json records each one's reason and the verified entry it replaced.")
    changes.append(f"The tree now holds {len(index)} rewrites, {v2} of them passed under checker version 2 or later; index.json records the address, name, trial count and checker version of each.")
    changes.append("They are in the checker's harness form (calls through numbered slots, globals by original address) and are not yet linked into the workspace crates.")
    if skipped:
        changes.append(f"{len(skipped)} rewrites are held back by the publication scan (comments that read like disassembly or byte dumps) and stay in scratch until cleaned.")
    title = (f"Rewrites: {len(new)} more functions pass the checker ({len(index)} in total)" if new
             else f"Rewrites: {len(changed)} functions re-checked under a later checker version" if changed
             else f"Rewrites: {len(gone)} functions moved back to unverified by a later checker version")
    entry = COORD / "entry-rewrites.json"
    entry.write_text(json.dumps({"title": title, "summary": "Rust rewrites of original game functions, each proven against the original by the checker.",
                                 "changes": changes, "files": ["rewrites/verified"] + (["rewrites/unverified"] if unverified_moved else [])}), encoding="utf-8")
    result = subprocess.run([sys.executable, str(Path(__file__).resolve().parent / "commit_one.py"), str(entry)], cwd=ROOT,
                            env=dict(os.environ), capture_output=True, text=True, encoding="utf-8")
    print((result.stdout + result.stderr).strip())


if __name__ == "__main__":
    main()
