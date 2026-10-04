"""Progress counts from lane machine output. Pure functions, no file IO.

The driver (`apply_counts.py`) loads the inventory, the runtime, fragment and
naming verdicts, the lanes' results and the tracked rewrite index; these
functions turn them into the published counts. Kept pure so the counting rules
are tested on a synthetic set without any lane data.
"""

import bisect

try:
    from common import is_placeholder_name, va
except ImportError:  # run directly: find the sibling module beside this file
    import os
    import sys

    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from common import is_placeholder_name, va

# One function the runtime lane judged runtime that is game code (f-runtime's
# report; applied by the counts since 2026-10-04). A data point the queue and
# the counts share, kept in one place so it cannot drift between them.
REINSTATED_GAME_FUNCTION = 0xA13CB0


def game_set(functions):
    """Addresses of inventory entries that are game code (not library/runtime)."""
    return {va(f["start"]) for f in functions if f.get("kind") not in ("library", "runtime")}


def apply_runtime_moves(game, rows, inventory_starts):
    """Remove what the runtime lane judged runtime; reinstate the one known
    game-code function when it is in the inventory. Returns (game, moved)."""
    rows = rows if isinstance(rows, list) else rows.get("functions") or list(rows.values())
    runtime = {va(r["start"]) for r in rows if r.get("verdict") == "runtime"}
    moved = len(game & runtime)
    game = game - runtime
    if REINSTATED_GAME_FUNCTION in inventory_starts:
        game.add(REINSTATED_GAME_FUNCTION)
    return game, moved


def apply_fragments(game, numbers, inventory_starts, fragment_sets):
    """Drop entries judged fragments of other functions (or data): they leave
    the inventory altogether, so the total and the game count both go down."""
    fragments = set().union(*fragment_sets) if fragment_sets else set()
    dropped = len(fragments & inventory_starts)
    from_game = len(game & fragments)
    game = game - fragments
    numbers = dict(numbers, all=numbers["all"] - dropped, game=len(game),
                   library_runtime=numbers["all"] - dropped - len(game))
    return game, numbers, dropped, from_game


def name_records(rows, game, lane, source_default="proposed", require_confidence=None):
    """Meaningful names from result rows: address -> record. Slot labels,
    placeholders and empty names never count."""
    found = {}
    for row in rows:
        try:
            address = va(row["address"])
        except (KeyError, ValueError, TypeError):
            continue
        label = str(row.get("name") or "")
        if address in game and not is_placeholder_name(label):
            if require_confidence and row.get("confidence") not in require_confidence:
                continue
            found[address] = {"address": f"0x{address:08X}", "name": label, "lane": lane,
                              "source": row.get("name_source") or source_default}
    return found


def verified_from_rows(rows, has_rewrite):
    """Addresses recorded as verified whose rewrite file exists, plus the
    deferred count. `has_rewrite` takes the address, so tests need no files.
    The caller intersects with the game set."""
    passed, deferred = set(), 0
    for row in rows:
        if not isinstance(row, dict):
            continue
        try:
            address = va(row["address"])
        except (KeyError, ValueError, TypeError):
            continue
        if row.get("outcome") == "verified" and has_rewrite(address):
            passed.add(address)
        elif row.get("outcome") == "deferred":
            deferred += 1
    return passed, deferred


def rebuild_map(slices, game, named, rewritten, verified):
    """Rebuild the code map's slices from the same sets as the counts.

    Each slice keeps its address range and carries how many of its game
    functions reached each stage; its shade is the stage at least half of
    them reached (`unmeasured` when it holds no game functions).
    """
    starts = sorted(game)
    redone = set(rewritten) | set(verified)
    labelled = set(named) | redone
    for piece in slices:
        lo, hi = va(piece["from"]), va(piece["to"])
        last = piece is slices[-1]
        members = starts[bisect.bisect_left(starts, lo):
                         (bisect.bisect_right(starts, hi) if last else bisect.bisect_left(starts, hi))]
        count = len(members)
        n_named = sum(1 for a in members if a in labelled)
        n_rewritten = sum(1 for a in members if a in redone)
        n_verified = sum(1 for a in members if a in verified)
        half = (count + 1) // 2
        stage = ("unmeasured" if count == 0 else "verified" if n_verified >= half
                 else "rewritten" if n_rewritten >= half else "named" if n_named >= half else "identified")
        piece.clear()
        piece.update({"stage": stage, "from": f"0x{lo:08X}", "to": f"0x{hi:08X}", "count": count,
                      "named": n_named, "rewritten": n_rewritten, "verified": n_verified})
    return slices
