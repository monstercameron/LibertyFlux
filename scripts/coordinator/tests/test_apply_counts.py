"""Tests for apply_counts.py: the published counts from the tracked index come from the ledger, unchanged.

`legacy_tree_counts` is a frozen copy of the counting block apply_counts.main() ran before it called the ledger
(2026-10-04). Every synthetic tree below must give the same stages, symbols and map set through both, so the
switch to `ledger.summarise` cannot have moved a published number.
"""

import json
import random
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import apply_counts
import ledger
from common import va
from scan import MODERN

BASE = 0x00500000
CHECKERS = ("version 1", "version 2", "version 3", "version 4", None, "v4")


def legacy_tree_counts(progress, entries, game, named, rewritten):
    """The pre-ledger block, verbatim apart from being a function."""
    stages = progress["stages"]
    progress["symbols"] = sum(1 for e in entries if e.get("name"))
    verified_set = {va(e["address"]) for e in entries if e.get("checker") in MODERN} & game
    rewritten |= {va(e["address"]) for e in entries} & game
    stages["rewritten"] = len(rewritten)
    stages["named"] = max(len(named), len(rewritten))
    stages["verified"] = min(stages["rewritten"], len(verified_set))
    return verified_set


def synthetic(seed):
    """A random game set, lane passes, names and index (some entries outside the game set, some v1)."""
    rng = random.Random(seed)
    universe = [BASE + 0x10 * i for i in range(400)]
    game = set(rng.sample(universe, 300))
    named = set(rng.sample(sorted(game), rng.randrange(0, 300)))
    passed = set(rng.sample(universe, rng.randrange(0, 200)))
    entries = []
    for address in rng.sample(universe, rng.randrange(0, 250)):
        address_text = f"0x{address:08X}" if rng.random() < 0.9 else address  # some indexes wrote numbers
        entries.append({"address": address_text, "name": rng.choice([None, "", f"f_{address:x}"]),
                        "checker": rng.choice(CHECKERS), "file": f"functions/fn_{address:08x}.rs", "kind": "function"})
    return game, named, passed, entries


class TestLedgerSwitch(unittest.TestCase):
    def test_same_counts_as_before(self):
        for seed in range(200):
            game, named, passed, entries = synthetic(seed)
            old = {"stages": {"verified": 7}, "symbols": None}
            new = json.loads(json.dumps(old))
            old_rewritten, new_rewritten = passed & game, passed & game
            old_set = legacy_tree_counts(old, entries, game, named, set(old_rewritten))
            new_set = apply_counts.apply_tree(new, entries, game, named, set(new_rewritten))
            self.assertEqual(old, new, seed)
            self.assertEqual(old_set, new_set, seed)

    def test_rewritten_widened_in_place(self):
        game, named, passed, entries = synthetic(3)
        rewritten = passed & game
        apply_counts.apply_tree({"stages": {}}, entries, game, named, rewritten)
        self.assertTrue({va(e["address"]) for e in entries} & game <= rewritten)

    def test_count_is_the_ledgers(self):
        game, named, passed, entries = synthetic(11)
        progress = {"stages": {}}
        apply_counts.apply_tree(progress, entries, game, named, set(passed & game) | game)
        self.assertEqual(progress["stages"]["verified"], ledger.summarise(entries, [], game)["verified_game"])

    def test_through_load_index_on_disk(self):
        game, named, passed, entries = synthetic(5)
        with tempfile.TemporaryDirectory() as tmp:
            folder = Path(tmp)
            (folder / "index.json").write_text(json.dumps(entries), encoding="utf-8")
            loaded = ledger.load_index(folder)
        old, new = {"stages": {}}, {"stages": {}}
        legacy_tree_counts(old, json.loads(json.dumps(entries)), game, named, set(passed & game))
        apply_counts.apply_tree(new, loaded, game, named, set(passed & game))
        self.assertEqual(old, new)

    def test_empty_index(self):
        progress = {"stages": {"verified": 5}}
        self.assertEqual(apply_counts.apply_tree(progress, [], {BASE}, set(), set()), set())
        self.assertEqual(progress["stages"], {"rewritten": 0, "named": 0, "verified": 0})
        self.assertEqual(progress["symbols"], 0)


class TestSetFact(unittest.TestCase):
    def test_update_existing(self):
        measured = [{"label": "a", "value": 1, "unit": "u", "source": "s"}]
        apply_counts.set_fact(measured, "a", 2, "u", "t")
        self.assertEqual(measured, [{"label": "a", "value": 2, "unit": "u", "source": "t"}])

    def test_renames_old_label_in_place(self):
        measured = [{"label": "x", "value": 0, "unit": "u", "source": "s"},
                    {"label": apply_counts.REWRITTEN_FACT_WAS, "value": 1, "unit": "functions", "source": "old"}]
        apply_counts.set_fact(measured, apply_counts.REWRITTEN_FACT, 9, "functions", "new",
                              replaces=apply_counts.REWRITTEN_FACT_WAS)
        self.assertEqual([m["label"] for m in measured], ["x", apply_counts.REWRITTEN_FACT])
        self.assertEqual(measured[1]["value"], 9)

    def test_new_label_wins_over_old(self):
        measured = [{"label": "old", "value": 1, "unit": "u", "source": "s"},
                    {"label": "new", "value": 2, "unit": "u", "source": "s"}]
        apply_counts.set_fact(measured, "new", 3, "u", "s", replaces="old")
        self.assertEqual([(m["label"], m["value"]) for m in measured], [("old", 1), ("new", 3)])

    def test_append(self):
        measured = []
        apply_counts.set_fact(measured, "n", 1, "u", "s", replaces="gone")
        self.assertEqual(measured, [{"label": "n", "value": 1, "unit": "u", "source": "s"}])


if __name__ == "__main__":
    unittest.main()
