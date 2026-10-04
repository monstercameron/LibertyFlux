"""Tests for history.py: points from progress versions, merging with the existing file, thinning and the output."""

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import history


def progress(game=100, named=10, rewritten=5, verified=4):
    return {"functions": {"total": game + 20, "library": 20, "game": game},
            "stages": {"identified": game, "named": named, "rewritten": rewritten, "verified": verified}}


def pt(t, game=100, named=10, rewritten=5, verified=4):
    return {"t": t, "game": game, "named": named, "rewritten": rewritten, "verified": verified}


class TestPoint(unittest.TestCase):
    def test_counts_and_time_in_utc(self):
        entry = history.point("2026-10-04T11:46:51-04:00", progress(37413, 14690, 7884, 7655))
        self.assertEqual(entry, pt("2026-10-04T15:46:51Z", 37413, 14690, 7884, 7655))

    def test_nothing_counted_is_skipped(self):
        self.assertIsNone(history.point("2026-10-01T00:00:00Z", {"functions": {"game": None}, "stages": {}}))
        self.assertIsNone(history.point("2026-10-01T00:00:00Z", {}))

    def test_missing_stage_is_zero(self):
        entry = history.point("2026-10-01T00:00:00Z", {"functions": {"game": 5}, "stages": {"named": 2}})
        self.assertEqual((entry["rewritten"], entry["verified"]), (0, 0))

    def test_time_without_zone_is_refused(self):
        with self.assertRaises(ValueError):
            history.to_utc("2026-10-04 13:20")


class TestMerge(unittest.TestCase):
    def test_existing_points_survive_a_shallow_clone(self):
        old = [pt("2026-10-01T00:00:00Z", named=1), pt("2026-10-02T00:00:00Z", named=2)]
        fresh = [pt("2026-10-02T00:00:00Z", named=3), pt("2026-10-03T00:00:00Z", named=4)]
        merged = history.merge(old, fresh)
        self.assertEqual([p["t"][:10] for p in merged], ["2026-10-01", "2026-10-02", "2026-10-03"])
        self.assertEqual(merged[1]["named"], 3)  # git is the source: its value replaces the recorded one

    def test_malformed_existing_entries_are_ignored(self):
        merged = history.merge([None, {"no": "time"}, pt("2026-10-01T00:00:00Z")], [])
        self.assertEqual(len(merged), 1)


class TestThin(unittest.TestCase):
    def test_runs_keep_first_and_last(self):
        points = [pt(f"2026-10-01T00:0{i}:00Z", named=n) for i, n in enumerate([1, 2, 2, 2, 2, 3])]
        kept = history.thin(points)
        self.assertEqual([p["named"] for p in kept], [1, 2, 2, 3])
        self.assertEqual([p["t"][14:16] for p in kept], ["00", "01", "04", "05"])

    def test_idempotent_and_keeps_decreases(self):
        points = [pt(f"2026-10-01T00:0{i}:00Z", verified=v) for i, v in enumerate([5, 5, 5, 4, 4, 6])]
        once = history.thin(points)
        self.assertEqual(history.thin(once), once)
        self.assertIn(4, [p["verified"] for p in once])  # a count that went down is recorded, never smoothed


class TestProvisional(unittest.TestCase):
    def test_uncommitted_state_is_added_then_replaced_by_its_commit(self):
        committed = [pt("2026-10-01T00:00:00Z", named=1)]
        provisional = dict(pt("2026-10-01T00:05:00Z", named=2), tree=True)
        first = history.combine([], committed, provisional)
        self.assertEqual([p["named"] for p in first], [1, 2])
        self.assertTrue(first[-1]["tree"])
        # Next tick: the state was committed a few seconds after the file was written.
        second = history.combine(first, committed + [pt("2026-10-01T00:05:09Z", named=2)])
        self.assertEqual([(p["t"][11:], p.get("tree")) for p in second], [("00:00:00Z", None), ("00:05:09Z", None)])

    def test_flag_survives_the_file(self):
        provisional = dict(pt("2026-10-01T00:05:00Z"), tree=True)
        points = json.loads(history.render([pt("2026-10-01T00:00:00Z", named=1), provisional]))["points"]
        self.assertNotIn("tree", points[0])
        self.assertIs(points[1]["tree"], True)


class TestFile(unittest.TestCase):
    def test_render_round_trips_and_reads_back(self):
        points = [pt("2026-10-01T00:00:00Z"), pt("2026-10-01T00:05:00Z", named=11)]
        text = history.render(points)
        self.assertEqual(json.loads(text)["points"], points)
        self.assertTrue(text.endswith("\n"))
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "history.json"
            path.write_text(text, encoding="utf-8")
            self.assertEqual(history.read_existing(path), points)

    def test_unreadable_file_reads_as_empty(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "history.json"
            self.assertEqual(history.read_existing(path), [])
            path.write_text("not json", encoding="utf-8")
            self.assertEqual(history.read_existing(path), [])

    def test_empty_history_renders(self):
        self.assertEqual(json.loads(history.render([]))["points"], [])


class TestRepository(unittest.TestCase):
    def test_git_points_are_ordered_and_bounded(self):
        # Reads this clone's own history: every point must be internally consistent.
        points = history.from_git()
        times = [p["t"] for p in points]
        self.assertEqual(times, sorted(times))
        for p in points:
            self.assertLessEqual(p["verified"], p["rewritten"])
            self.assertLessEqual(p["rewritten"], p["named"])
            self.assertLessEqual(p["named"], p["game"])


if __name__ == "__main__":
    unittest.main()
