"""Coordinator helper: put machine-derived counts into docs/data/progress.json.

Sources (all lane machine output under .artifacts/scratch):
  f-boundaries/functions_v2.json and summary_numbers.json   repaired function inventory
  m-names/symbols.json                                       merged names
  r-*/results.json and r-*/out/rewrites/fn_*.rs              production lanes' rewrites that passed the checker
`verified` counts rewrites that passed under checker version 2 or later and are in the tracked tree.
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import counts
from scan import MODERN

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
PROGRESS = ROOT / "docs" / "data" / "progress.json"


def main():
    va = common.va
    progress = json.loads(PROGRESS.read_text(encoding="utf-8"))
    numbers = json.loads((SCRATCH / "f-boundaries" / "summary_numbers.json").read_text(encoding="utf-8"))["corrected_totals"]
    functions = json.loads((SCRATCH / "f-boundaries" / "functions_v2.json").read_text(encoding="utf-8"))
    functions = common.load_rows(functions, "functions") or list(functions.values())
    game = counts.game_set(functions)
    assert len(game) == numbers["game"], (len(game), numbers["game"])
    inventory_starts = {va(f["start"]) for f in functions}
    # The f-runtime lane found C runtime and QuickTime glue that the inventory had counted as game code, and one
    # function marked runtime that is game code. Apply both; its four "unsure" functions stay counted as game.
    runtime_file = SCRATCH / "f-runtime" / "runtime_functions.json"
    if runtime_file.exists():
        rows = json.loads(runtime_file.read_text(encoding="utf-8"))
        game, moved = counts.apply_runtime_moves(game, rows, inventory_starts)
        numbers = dict(numbers, game=len(game), library_runtime=numbers["all"] - len(game))
        print(f"moved from game to library by the runtime lane: {moved}")
    # The f-ptronly lane found inventory entries that are pieces of other functions (exception handler blocks), not
    # functions. They leave the inventory altogether: the total and the game count both go down by that many.
    fragments_file = SCRATCH / "f-ptronly" / "ptr_only_verdicts.json"
    if fragments_file.exists():
        fragments = {va(r["start"]) for r in json.loads(fragments_file.read_text(encoding="utf-8")) if r.get("verdict") == "fragment"}
        weak_file = SCRATCH / "f-weak" / "weak_verdicts.json"
        weak = set()
        if weak_file.exists():
            weak = {va(r["start"]) for r in json.loads(weak_file.read_text(encoding="utf-8")) if r.get("verdict") in ("fragment", "data")}
        game, numbers, dropped, from_game = counts.apply_fragments(game, numbers, inventory_starts, [fragments, weak])
        print(f"dropped as fragments of other functions: {dropped} ({from_game} had been counted as game code)")

    symbols = json.loads((SCRATCH / "m-names" / "symbols.json").read_text(encoding="utf-8"))
    named = {va(s["address"]) for s in symbols if s.get("meaningful")} & game
    merged_names = len(named)
    naming_file = COORD / "names_from_lanes.json"
    COORD.mkdir(parents=True, exist_ok=True)
    lane_names = {}

    lanes = 0
    deferred = 0
    passed = set()
    for results in sorted(SCRATCH.glob("r-*/results.json")):
        try:
            rows = common.load_rows(json.loads(results.read_text(encoding="utf-8")), "functions", "results")
        except (ValueError, OSError):
            continue
        lanes += 1
        lane_dir = results.parent
        lane_names.update(counts.name_records(rows, game, lane_dir.name))
        lane_passed, lane_deferred = counts.verified_from_rows(
            rows, lambda a: (lane_dir / "out" / "rewrites" / f"fn_{a:08x}.rs").exists())
        passed |= lane_passed
        deferred += lane_deferred
    rewritten = passed & game
    # Names from naming lanes (nm-*), when there are any: names.json rows with address, name and confidence.
    for names_path in sorted(SCRATCH.glob("nm-*/names.json")):
        try:
            rows = json.loads(names_path.read_text(encoding="utf-8"))
        except (ValueError, OSError):
            continue
        rows = common.load_rows(rows, "names")
        for address, record in counts.name_records(rows, game, names_path.parent.name, source_default="naming lane",
                                                  require_confidence=("high", "medium")).items():
            if address not in lane_names:
                lane_names[address] = record
    naming_file.write_text(json.dumps(sorted(lane_names.values(), key=lambda r: r["address"]), indent=1), encoding="utf-8")
    from_lanes = len(set(lane_names) - named)
    named |= set(lane_names)
    print(f"named: {merged_names} from the symbol merge, {from_lanes} more named by lanes")
    print(f"functions all {numbers['all']} library {numbers['library_runtime']} game {numbers['game']}")
    print(f"named {len(named)}; production lanes reporting {lanes}; passed the checker {len(passed)} "
          f"({len(rewritten)} in the game inventory); deferred {deferred}")

    progress["functions"] = {"total": numbers["all"], "library": numbers["library_runtime"], "game": numbers["game"]}
    stages = progress["stages"]
    stages["identified"] = numbers["game"]
    stages["named"] = max(len(named), len(rewritten))
    stages["rewritten"] = len(rewritten)
    stages["verified"] = min(stages.get("verified", 0), len(rewritten))

    # Structures and symbols are counted from tracked files only (rule 4).
    # The layouts are split into one module per subsystem; count across every file of the crate.
    types = ROOT / "crates" / "lf-types-draft" / "src"
    if types.is_dir():
        text = "".join("\n" + p.read_text(encoding="utf-8") for p in sorted(types.glob("*.rs")))
        progress["structures"] = max(0, text.count("\npub struct ") - 1)  # minus the Ptr32 helper
    index = ROOT / "rewrites" / "verified" / "index.json"
    if index.exists():
        entries = json.loads(index.read_text(encoding="utf-8"))
        progress["symbols"] = sum(1 for e in entries if e.get("name"))
        # Verified means passed under checker version 2 or later and present in the tracked tree.
        verified_set = {va(e["address"]) for e in entries if e.get("checker") in MODERN} & game
        # A function a production lane deferred and a re-run lane later verified is in the tracked tree: count it.
        rewritten |= {va(e["address"]) for e in entries} & game
        stages["rewritten"] = len(rewritten)
        stages["named"] = max(len(named), len(rewritten))
        stages["verified"] = min(stages["rewritten"], len(verified_set))
    else:
        verified_set = set()
    print(f"structures {progress['structures']} symbols {progress['symbols']}")

    # The code map, rebuilt from the same sets as the counts (rule 4). The slices keep their address ranges. Each
    # carries how many of its game functions have reached each stage; its shade is the stage that at least half of
    # them have reached, and the page draws the verified share as a band inside the square.
    slices = progress.get("map") or []
    if slices:
        counts.rebuild_map(slices, game, named, rewritten, verified_set)
        from collections import Counter
        print("map:", dict(Counter(p["stage"] for p in slices)), "functions placed", sum(p["count"] for p in slices), "of", len(game))

    def fact(label, value, unit, source):
        for item in progress["measured"]:
            if item["label"] == label:
                item.update(value=value, unit=unit, source=source)
                return
        progress["measured"].append({"label": label, "value": value, "unit": unit, "source": source})

    fact("Functions after the inventory repair", numbers["all"], "functions", "f-boundaries lane: recursive-descent check of every inventory entry")
    fact("Functions with a meaningful name", len(named), "functions",
         f"m-names lane's merge ({merged_names}) plus descriptive names given by production and naming lanes ({from_lanes}); slot labels and placeholders are not counted")
    fact("Rewrites that pass checker version 1", len(rewritten), "functions", "production lanes' results files; to be re-run under checker version 2 before they count as verified")
    fact("Rewrites verified under checker version 2", stages["verified"], "functions", "rewrites/verified/index.json in the repository")
    PROGRESS.write_text(json.dumps(progress, indent=2) + "\n", encoding="utf-8", newline="\n")
    print("progress.json updated")


if __name__ == "__main__":
    main()
