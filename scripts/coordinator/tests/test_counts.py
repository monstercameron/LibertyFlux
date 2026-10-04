"""Tests for counts.py on a small synthetic set."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import counts
from common import va

BASE = 0x10000000


def inventory(n=10, kinds=None, start=BASE):
    rows = []
    for i in range(n):
        rows.append({"start": f"0x{start + i * 0x100:08X}", "size": 100,
                     "kind": (kinds[i] if kinds else "game")})
    return rows


class TestGameSet(unittest.TestCase):
    def test_library_and_runtime_excluded(self):
        game = counts.game_set(inventory(4, ["game", "library", "runtime", "game"]))
        self.assertEqual(game, {BASE, BASE + 0x300})


class TestRuntimeMoves(unittest.TestCase):
    def test_moves_and_reinstatement(self):
        rows = inventory(4)
        game = {BASE + i * 0x100 for i in range(4)}
        verdicts = [{"start": f"0x{BASE:08X}", "verdict": "runtime"},
                    {"start": f"0x{BASE + 0x100:08X}", "verdict": "unsure"},
                    {"start": f"0x{BASE + 0x200:08X}", "verdict": "game"}]
        game, moved = counts.apply_runtime_moves(game, verdicts, {va(r["start"]) for r in rows} | {counts.REINSTATED_GAME_FUNCTION})
        self.assertEqual(moved, 1)
        self.assertNotIn(BASE, game)
        self.assertIn(BASE + 0x100, game)  # unsure stays game
        self.assertIn(counts.REINSTATED_GAME_FUNCTION, game)

    def test_no_reinstatement_when_absent(self):
        game = {BASE}
        game, _ = counts.apply_runtime_moves(game, [], {BASE})
        self.assertNotIn(counts.REINSTATED_GAME_FUNCTION, game)


class TestFragments(unittest.TestCase):
    def test_total_and_game_drop_together(self):
        game = {BASE + i * 0x100 for i in range(10)}
        numbers = {"all": 12, "game": 10, "library_runtime": 2}
        starts = game | {0x9999, 0xAAAA}
        game, numbers, dropped, from_game = counts.apply_fragments(
            game, numbers, starts, [{BASE, 0x9999}, {BASE + 0x100}])
        self.assertEqual(dropped, 3)
        self.assertEqual(from_game, 2)
        self.assertEqual(numbers, {"all": 9, "game": 8, "library_runtime": 1})
        self.assertEqual(len(game), 8)


class TestNameRecords(unittest.TestCase):
    def test_meaningful_kept_placeholders_dropped(self):
        rows = [{"address": f"0x{BASE:08X}", "name": "audio_voice_tick"},
                {"address": f"0x{BASE + 0x100:08X}", "name": "fn_00401100"},
                {"address": f"0x{BASE + 0x200:08X}", "name": ""},
                {"address": "not-an-address", "name": "x"},
                {"address": f"0x{BASE + 0x300:08X}", "name": "Class::vf3"}]
        game = {BASE + i * 0x100 for i in range(4)}
        found = counts.name_records(rows, game, "r-s01")
        self.assertEqual(set(found), {BASE})
        self.assertEqual(found[BASE]["lane"], "r-s01")
        self.assertEqual(found[BASE]["source"], "proposed")

    def test_naming_lane_confidence(self):
        rows = [{"address": f"0x{BASE:08X}", "name": "pool_tick", "confidence": "high"},
                {"address": f"0x{BASE + 0x100:08X}", "name": "pool_tock", "confidence": "low"}]
        game = {BASE, BASE + 0x100}
        found = counts.name_records(rows, game, "nm-01", source_default="naming lane",
                                    require_confidence=("high", "medium"))
        self.assertEqual(set(found), {BASE})
        self.assertEqual(found[BASE]["source"], "naming lane")


class TestVerified(unittest.TestCase):
    def test_verified_needs_rewrite_file(self):
        rows = [{"address": f"0x{BASE:08X}", "outcome": "verified"},
                {"address": f"0x{BASE + 0x100:08X}", "outcome": "verified"},
                {"address": f"0x{BASE + 0x200:08X}", "outcome": "deferred"},
                {"address": f"0x{BASE + 0x300:08X}", "outcome": "not_reached"},
                "junk"]
        passed, deferred = counts.verified_from_rows(rows, lambda a: a == BASE)
        self.assertEqual(passed, {BASE})
        self.assertEqual(deferred, 1)


class TestRebuildMap(unittest.TestCase):
    def test_stages_and_counts(self):
        slices = [{"from": "0x10000000", "to": "0x10001000"},
                  {"from": "0x10001000", "to": "0x10002000"},
                  {"from": "0x10002000", "to": "0x10003000"}]
        game = {BASE + i * 0x100 for i in range(8)} | {BASE + 0x1500}
        named = set(game)
        rewritten = {BASE, BASE + 0x100}  # 2 of 8 in slice 0: not half
        verified = {BASE + 0x1500}  # the only member of slice 1
        counts.rebuild_map(slices, game, named, rewritten, verified)
        self.assertEqual(slices[0]["stage"], "named")
        self.assertEqual(slices[0]["count"], 8)
        self.assertEqual(slices[0]["rewritten"], 2)
        self.assertEqual(slices[1]["stage"], "verified")
        self.assertEqual(slices[1]["count"], 1)
        self.assertEqual(slices[2]["stage"], "unmeasured")
        self.assertEqual(slices[2]["count"], 0)

    def test_half_rounds_up(self):
        slices = [{"from": "0x10000000", "to": "0x10002000"}]
        game = {BASE, BASE + 0x100, BASE + 0x200}
        counts.rebuild_map(slices, game, set(), {BASE}, set())
        self.assertEqual(slices[0]["stage"], "identified")  # 1 of 3 is not half
        counts.rebuild_map(slices, game, set(), {BASE, BASE + 0x100}, set())
        self.assertEqual(slices[0]["stage"], "rewritten")  # 2 of 3 is half

    def test_last_slice_includes_its_end(self):
        slices = [{"from": "0x10000000", "to": "0x10000100"}]
        game = {BASE, BASE + 0x100}
        counts.rebuild_map(slices, game, game, set(), set())
        self.assertEqual(slices[0]["count"], 2)


if __name__ == "__main__":
    unittest.main()
