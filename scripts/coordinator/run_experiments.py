"""Launch a set of controlled lane experiments. Each varies ONE thing against the current production brief,
writes its own tag into the lanes' results, and is recorded in experiments.json so the results can be compared
with the control lanes the supervisor starts in the same hour.

Usage: python run_experiments.py            (launches everything in EXPERIMENTS)
"""

import json
import os
import re
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import ledger
from start_lane import start_lane

import pefile

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
LISTS = common.lists_dir(ROOT)
BRIEFS = common.briefs_dir(ROOT)
HERE = common.HERE
PY = str(ROOT / ".venv" / "Scripts" / "python.exe")


EXPERIMENTS = [
    {"tag": "b6-x1-high", "factor": "reasoning effort high instead of max", "small": 2, "big": 2, "effort": "high",
     "note": "Nothing about how you should work is different; a setting outside your control was changed."},
    {"tag": "b6-x2-w6", "factor": "six checker workers instead of three", "small": 2, "big": 0, "effort": "max",
     "note": "You may run up to SIX checker worker processes at once instead of three (the machine's processors are two-thirds idle). "
             "Use them: run the 1,000-trial runs of several contracts in parallel, one driver process per contract."},
    {"tag": "b6-x3-n40", "factor": "40 small functions per lane instead of 20", "small": 2, "big": 0, "effort": "max", "env": {"LF_SMALL_N": "40"},
     "note": "Your batch is 40 functions instead of the usual 20, so the fixed cost of setting up is shared by more functions."},
    {"tag": "b6-x4-solo", "factor": "one large function per lane", "small": 0, "big": 2, "effort": "max", "env": {"LF_BIG_MAX": "1"},
     "note": "Your batch is a single function. There is no second function to trade time against: finish this one."},
    {"tag": "b6-x5-family", "factor": "one family of same-shaped neighbours with a generator", "small": 2, "big": 0, "effort": "max", "family": True,
     "note": "Your batch is ONE family of neighbouring functions of identical size, which are very probably one routine instantiated many times. "
             "Do not write them one by one. Work out the routine from three members; establish exactly what varies between members (usually one "
             "constant, one callee or one table address); write a generator that produces every rewrite, contract and wrong version; make the "
             "generator assert each member's shape (same instruction layout, only the expected operands differ) and treat any member that does not "
             "fit as its own case. Every member still gets 1,000 trials and its own wrong version."},
]


def families(count, cap=120):
    """The longest runs of unverified, unassigned, confirmed neighbours of identical size."""
    va = common.va
    pe = pefile.PE(str(common.orig_exe(ROOT)), fast_load=True)
    text = next(s for s in pe.sections if s.Name.rstrip(b"\0") == b".text")
    enc_end = pe.OPTIONAL_HEADER.ImageBase + text.VirtualAddress + 0xFB000
    functions = json.loads((SCRATCH / "f-boundaries" / "functions_v2.json").read_text(encoding="utf-8"))
    functions = functions if isinstance(functions, list) else functions.get("functions") or list(functions.values())
    done = {va(e["address"]) for e in ledger.load_index(ROOT / "rewrites" / "verified")}  # was the untracked rewrites/pending
    assigned = set()
    for path in LISTS.glob("r-*.json"):
        for entry in json.loads(path.read_text(encoding="utf-8")):
            assigned.add(va(entry.get("start") or entry.get("handler")))
    runtime = set()
    rf = SCRATCH / "f-runtime" / "runtime_functions.json"
    if rf.exists():
        rows = json.loads(rf.read_text(encoding="utf-8"))
        rows = rows if isinstance(rows, list) else rows.get("functions") or list(rows.values())
        runtime = {va(r["start"]) for r in rows}
    usable = sorted((f for f in functions if f.get("kind") not in ("library", "runtime") and f.get("class") == "confirmed-ok"
                     and 16 <= int(f.get("size") or 0) < 250 and va(f["start"]) >= enc_end
                     and va(f["start"]) not in done | assigned | runtime), key=lambda f: va(f["start"]))
    runs, run = [], []
    for f in usable:
        if run and f["size"] == run[-1]["size"] and va(f["start"]) - va(run[-1]["start"]) < 4 * max(f["size"], 16):
            run.append(f)
        else:
            if len(run) >= 20:
                runs.append(run)
            run = [f]
    if len(run) >= 20:
        runs.append(run)
    runs.sort(key=len, reverse=True)
    print("family runs available:", [len(r) for r in runs[:10]])
    return [[{"start": f"0x{va(f['start']):08X}", "size": f["size"], "name": f.get("name"), "sub": f.get("subsystem"), "level": 0,
              "direct_callees": None, "calls": "unknown: check each"} for f in r[:cap]] for r in runs[:count]]


def main():
    record_path = COORD / "experiments.json"
    record = json.loads(record_path.read_text(encoding="utf-8")) if record_path.exists() else []
    for exp in EXPERIMENTS:
        env = dict(os.environ, LF_EXP_TAG=exp["tag"], LF_EXP_NOTE=exp["note"], **exp.get("env", {}))
        out = subprocess.run([PY, str(HERE / "make_briefs_prod.py"), "0", str(exp["small"]), str(exp["big"])],
                             capture_output=True, text=True, env=env).stdout
        match = re.search(r"^LANES (.*)$", out, re.M)
        lanes = match.group(1).split() if match else []
        if exp.get("family"):
            for lane, members in zip(lanes, families(len(lanes))):
                (LISTS / f"{lane}.json").write_text(json.dumps(members, indent=1), encoding="utf-8")
                brief = (BRIEFS / f"{lane}.txt").read_text(encoding="utf-8")
                brief, n = re.subn(r"Your batch: the \d+ functions", f"Your batch: the {len(members)} functions", brief)
                assert n == 1, lane
                (BRIEFS / f"{lane}.txt").write_text(brief, encoding="utf-8", newline="\n")
                print(f"  {lane}: family of {len(members)} functions of {members[0]['size']} bytes")
        for lane in lanes:
            start_lane(lane, exp["effort"])
            record.append({"lane": lane, "tag": exp["tag"], "factor": exp["factor"], "effort": exp["effort"],
                           "started": common.now_stamp()})
        print(f"{exp['tag']}: {exp['factor']}: lanes {lanes}")
    record_path.write_text(json.dumps(record, indent=1), encoding="utf-8")


if __name__ == "__main__":
    main()
