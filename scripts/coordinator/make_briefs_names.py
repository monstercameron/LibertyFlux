"""Briefs for naming lanes (nm-##): propose a descriptive name for game functions that have none yet.

Usage: python make_briefs_names.py <lanes> [<functions per lane>]
Batches are contiguous runs of unnamed functions in address order (neighbours come from the same source file, so
they are named better together), taken from evenly spaced places in the address space. Functions already handed
to a naming lane are skipped.
"""

import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import queue as queue_rules
from briefs import render_naming, render_preamble

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
LISTS = common.lists_dir(ROOT)
OUT = common.briefs_dir(ROOT)


def main():
    va = common.va
    want = int(sys.argv[1])
    per = int(sys.argv[2]) if len(sys.argv) > 2 else 150

    functions = json.loads((SCRATCH / "f-boundaries" / "functions_v2.json").read_text(encoding="utf-8"))
    functions = functions if isinstance(functions, list) else functions.get("functions") or list(functions.values())
    game = {va(f["start"]): f for f in functions if f.get("kind") not in ("library", "runtime")}
    runtime = SCRATCH / "f-runtime" / "runtime_functions.json"
    if runtime.exists():
        rows = json.loads(runtime.read_text(encoding="utf-8"))
        rows = rows if isinstance(rows, list) else rows.get("functions") or list(rows.values())
        for r in rows:
            game.pop(va(r["start"]), None)
    fragments = SCRATCH / "f-ptronly" / "ptr_only_verdicts.json"
    if fragments.exists():
        for r in json.loads(fragments.read_text(encoding="utf-8")):
            if r.get("verdict") == "fragment":
                game.pop(va(r["start"]), None)
    weak_file = SCRATCH / "f-weak" / "weak_verdicts.json"
    if weak_file.exists():
        for r in json.loads(weak_file.read_text(encoding="utf-8")):
            if r.get("verdict") in ("fragment", "data"):
                game.pop(va(r["start"]), None)
    symbols = json.loads((SCRATCH / "m-names" / "symbols.json").read_text(encoding="utf-8"))
    named = {va(s["address"]) for s in symbols if s.get("meaningful")}
    from_lanes = COORD / "names_from_lanes.json"
    if from_lanes.exists():
        named |= {va(r["address"]) for r in json.loads(from_lanes.read_text(encoding="utf-8"))}
    taken = set()
    numbers = [0]
    for path in LISTS.glob("nm-*.json"):
        numbers.append(int(re.sub(r"\D", "", path.stem)))
        taken |= {va(r["address"]) for r in json.loads(path.read_text(encoding="utf-8"))}
    # Encrypted code cannot be read from the file: leave it out.
    enc_end = 0
    enc = COORD / "encrypted_end.txt"
    if enc.exists():
        enc_end = int(enc.read_text().strip(), 16)
    todo = sorted(a for a in game if a not in named and a not in taken and a >= enc_end)
    print(f"game functions {len(game)}, unnamed and not yet handed to a naming lane {len(todo)}")

    made = []
    for i, batch in enumerate(queue_rules.naming_batches(todo, want, per)):
        name = f"nm-{max(numbers) + 1 + i:02d}"
        rows = [{"address": f"0x{a:08X}", "size": game[a].get("size"), "inventory_name": game[a].get("name"),
                 "class": game[a].get("class"), "subsystem": game[a].get("subsystem"), "evidence": game[a].get("evidence")} for a in batch]
        path = LISTS / f"{name}.json"
        path.write_text(json.dumps(rows, indent=1), encoding="utf-8")
        text = render_preamble(name, str(ROOT)) + render_naming(len(rows), str(path), str(ROOT), name[3:])
        (OUT / f"{name}.txt").write_text(text, encoding="utf-8", newline="\n")
        made.append(name)
    print("LANES " + " ".join(made))


if __name__ == "__main__":
    main()
