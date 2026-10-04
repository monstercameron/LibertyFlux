"""Tests for release_failed.py: what a stopped lane gives back, with and without results.json."""

import json
import sys
import tempfile
import time
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import release_failed

BASE = 0x00600000


def entry(i):
    return {"start": f"0x{BASE + i * 0x40:08X}", "size": 40, "sub": "audio"}


def row(i, outcome, reason=None):
    return {"address": f"0x{BASE + i * 0x40:08X}", "outcome": outcome, "reason": reason}


class TestSettled(unittest.TestCase):
    def test_outcomes(self):
        for outcome in ("verified", "verified_v2", "verified_v3", "verified_v4"):
            self.assertTrue(release_failed.settled({"outcome": outcome}))
        self.assertTrue(release_failed.settled({"outcome": "deferred", "reason": "indirect_call"}))
        for bad in ({"outcome": "deferred"}, {"outcome": "deferred", "reason": "  "}, {"outcome": "not_reached"},
                    {"outcome": "not yet run"}, {"outcome": "failed_v3", "reason": "x"}, {}):
            self.assertFalse(release_failed.settled(bad), bad)


class TestSplitBatch(unittest.TestCase):
    def test_split(self):
        batch = [entry(i) for i in range(6)]
        rows = [row(0, "verified_v4"), row(1, "deferred", "thread_local"), row(2, "deferred"), row(3, "not_reached"),
                row(4, "not yet run")]  # 5 has no row at all
        keep, release = release_failed.split_batch(batch, rows)
        self.assertEqual(keep, batch[:2])
        self.assertEqual(release, batch[2:])

    def test_numeric_addresses_and_natives_by_name(self):
        batch = [{"name": "GET_X", "hash": "0x1", "handler": f"0x{BASE:08X}"},
                 {"name": "SET_Y", "hash": "0x2", "handler": f"0x{BASE + 0x40:08X}"}]
        rows = [{"address": BASE, "outcome": "verified"}, {"address": "garbage", "name": "SET_Y", "outcome": "verified"}]
        keep, release = release_failed.split_batch(batch, rows)
        self.assertEqual((len(keep), len(release)), (2, 0))

    def test_unreadable_entry_stays(self):
        keep, release = release_failed.split_batch([{"start": None}], [])
        self.assertEqual((len(keep), len(release)), (1, 0))


class TestReleaseLane(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        base = Path(self.tmp.name)
        self.lists, self.scratch, self.logs = base / "lists", base / "scratch", base / "logs"
        for d in (self.lists, self.scratch / "r-s1", self.logs):
            d.mkdir(parents=True)
        (self.lists / "r-s1.json").write_text(json.dumps([entry(i) for i in range(4)]))
        self.later = time.time() + 3600  # well past the grace period

    def tearDown(self):
        self.tmp.cleanup()

    def release(self, now=None):
        return release_failed.release_lane(self.lists, self.scratch, self.logs, "r-s1", "exited-incomplete", now or self.later)

    def test_no_results_moves_whole_batch(self):
        self.assertEqual(self.release(), "r-s1 (exited-incomplete)")
        self.assertFalse((self.lists / "r-s1.json").exists())
        self.assertEqual(len(json.loads((self.lists / "released" / "r-s1.json").read_text())), 4)

    def test_grace_period(self):
        self.assertIsNone(self.release(now=time.time()))
        self.assertTrue((self.lists / "r-s1.json").exists())

    def test_results_written_early_releases_unsettled_only(self):
        (self.scratch / "r-s1" / "results.json").write_text(json.dumps(
            [row(0, "verified_v4"), row(1, "not yet run"), row(2, "deferred", "other"), row(3, "not_reached")]))
        self.assertEqual(self.release(), "r-s1 (exited-incomplete, 2 unsettled functions)")
        kept = json.loads((self.lists / "r-s1.json").read_text())
        released = json.loads((self.lists / "released" / "r-s1.json").read_text())
        self.assertEqual([e["start"] for e in kept], [entry(0)["start"], entry(2)["start"]])
        self.assertEqual([e["start"] for e in released], [entry(1)["start"], entry(3)["start"]])
        # Idempotent: the next tick finds nothing more to release and writes nothing.
        self.assertIsNone(self.release())
        self.assertEqual(len(json.loads((self.lists / "released" / "r-s1.json").read_text())), 2)

    def test_dict_shaped_results(self):
        (self.scratch / "r-s1" / "results.json").write_text(json.dumps({"functions": [row(i, "verified") for i in range(4)]}))
        self.assertIsNone(self.release())
        self.assertEqual(len(json.loads((self.lists / "r-s1.json").read_text())), 4)

    def test_unreadable_results_left_alone(self):
        (self.scratch / "r-s1" / "results.json").write_text("{not json")
        self.assertIn("left alone", self.release())
        self.assertEqual(len(json.loads((self.lists / "r-s1.json").read_text())), 4)

    def test_no_batch(self):
        (self.lists / "r-s1.json").unlink()
        self.assertIsNone(self.release())


if __name__ == "__main__":
    unittest.main()
