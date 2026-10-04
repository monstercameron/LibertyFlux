"""Tests for the dashboard's history and experiment logic (scripts/dashboard/server.py). Pure functions and
files in a temporary folder only; the server itself, the process list and memory readings are not run."""

import importlib.util
import json
import sys
import tempfile
import unittest
from datetime import datetime
from pathlib import Path

SERVER = Path(__file__).resolve().parents[2] / "dashboard" / "server.py"
spec = importlib.util.spec_from_file_location("lf_dashboard_server", SERVER)
server = importlib.util.module_from_spec(spec)
spec.loader.exec_module(server)


class TestHistory(unittest.TestCase):
    def test_sample(self):
        lanes = {"r-s1": {"mem_mb": 512}, "r-s2": {"mem_mb": 1024}}
        mem = {"total_gb": 48.0, "free_gb": 12.0, "commit_limit_gb": 80.0, "commit_headroom_gb": 20.0}
        row = server.history_sample(1000.7, lanes, {"stages": {"verified": 8000, "rewritten": 8300}}, mem)
        self.assertEqual(row, {"t": 1000, "lanes": 2, "verified": 8000, "rewritten": 8300, "agent_gb": 1.5,
                               "mem_pct": 75.0, "commit_pct": 75.0})
        bare = server.history_sample(5, {}, {}, None)
        self.assertEqual((bare["lanes"], bare["verified"], bare["mem_pct"]), (0, None, None))

    def test_file_round_trip_and_trim(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "cache" / "history.jsonl"
            rows = []
            for t in range(13):
                server.append_history(path, {"t": t, "lanes": t}, rows, keep=8)
            self.assertLessEqual(len(rows), 10)  # never more than a quarter over `keep`
            self.assertEqual(rows[-1]["t"], 12)
            with open(path, "a", encoding="utf-8") as handle:
                handle.write("not json\n{\"no_t\": 1}\n")
            loaded = server.load_history(path, keep=8)
            self.assertEqual([r["t"] for r in loaded], [r["t"] for r in rows][-8:])
            self.assertEqual(server.load_history(Path(tmp) / "missing.jsonl"), [])

    def test_review_points(self):
        history = [{"at": "2026-10-04 09:10", "verified": 7000}, {"at": "bad", "verified": 1}, {"at": "2026-10-04 10:12"},
                   {"at": "2026-10-04 08:00", "verified": 6900}]
        points = server.review_points(history)
        self.assertEqual([v for _, v in points], [6900, 7000])
        self.assertEqual(points[1][0], int(datetime(2026, 10, 4, 9, 10).timestamp()))

    def test_thin_keeps_last_of_each_bucket(self):
        points = [[t, t] for t in range(0, 1000)]
        thinned = server.thin(points, 0, 999, max_points=10)
        self.assertEqual(len(thinned), 10)
        self.assertEqual(thinned[-1], [999, 999])  # the true latest value survives
        self.assertEqual(server.thin(points, 100, 104), [[t, t] for t in range(100, 105)])

    def test_compose_backfills_verified_from_reviews(self):
        rows = [{"t": 1000 + 60 * i, "lanes": i, "verified": 50 + i, "mem_pct": None, "commit_pct": 40.0} for i in range(5)]
        reviews = [[100, 10], [500, 20], [1100, 999]]  # the last one overlaps the samples and is dropped
        out = server.compose_history(rows, reviews, 0, 2000)
        self.assertEqual(out["verified"], [[100, 10], [500, 20]] + [[1000 + 60 * i, 50 + i] for i in range(5)])
        self.assertEqual(out["lanes"], [[1000 + 60 * i, i] for i in range(5)])
        self.assertEqual(out["memory"], [])
        self.assertEqual(len(out["commit"]), 5)
        self.assertEqual(out["first_sample"], 1000)
        empty = server.compose_history([], reviews, 0, 2000)
        self.assertEqual((empty["first_sample"], len(empty["verified"])), (None, 3))


class TestExperiments(unittest.TestCase):
    def test_table(self):
        lanes = [
            ("r-s1", [{"outcome": "verified", "minutes": 2, "brief": "b8"}, {"outcome": "verified_v4", "minutes": 4, "brief": "b8"},
                      {"outcome": "deferred", "minutes": 6, "brief": "b8"}, {"outcome": "not_reached", "brief": "b8"}]),
            ("r-s2", [{"outcome": "verified", "minutes": 3}]),            # no field: the experiment tag of its lane
            ("r-b3", [{"outcome": "verified", "minutes": 10}, "junk"]),   # no field, no tag: the brief file's tag
            ("r-b4", [{"outcome": "deferred"}]),
        ]
        experiments = {"r-s2": {"lane": "r-s2", "tag": "b6-x3-n40", "factor": "40 small functions"}}
        table = server.experiment_table(lanes, experiments, {"r-b3": "b10"}.get, current="b8")
        rows = {(r["brief"], r["kind"]): r for r in table}
        b8 = rows[("b8", "small functions")]
        self.assertEqual((b8["functions"], b8["attempted"], b8["verified"], b8["deferred"]), (4, 3, 2, 1))
        self.assertEqual((b8["pass_rate"], b8["median_minutes"], b8["minutes_per_verified"], b8["current"]), (0.667, 3.0, 6.0, True))
        self.assertEqual(rows[("b6-x3-n40", "small functions")]["factor"], "40 small functions")
        self.assertIn(("b10", "large functions"), rows)
        self.assertEqual(rows[("not recorded", "large functions")]["pass_rate"], 0.0)
        # Natural order: b6-x3-n40 before b8 before b10.
        self.assertEqual([r["brief"] for r in table], ["b6-x3-n40", "b8", "b10", "not recorded"])

    def test_brief_version_field_and_no_minutes(self):
        table = server.experiment_table([("a-1", [{"outcome": "verified", "brief_version": "b7", "minutes": True}])], {})
        self.assertEqual((table[0]["brief"], table[0]["kind"], table[0]["median_minutes"]), ("b7", "re-run", None))

    def test_state_reads_lane_folders(self):
        with tempfile.TemporaryDirectory() as tmp:
            scratch = Path(tmp)
            (scratch / "r-s1").mkdir()
            (scratch / "r-s1" / "results.json").write_text(json.dumps({"functions": [{"outcome": "verified", "brief": "b8"}]}))
            (scratch / "coordinator").mkdir()
            old = (server.SCRATCH, server.COORD, server.BRIEFS)
            server.SCRATCH, server.COORD, server.BRIEFS = scratch, scratch / "coordinator", scratch / "coordinator" / "briefs"
            server._experiments.update(key=None, value=None)
            try:
                first = server.experiments_state()
                self.assertIs(server.experiments_state(), first)  # unchanged files: cached
            finally:
                server.SCRATCH, server.COORD, server.BRIEFS = old
                server._experiments.update(key=None, value=None)
        self.assertEqual(first["lanes"], 1)
        self.assertEqual(first["rows"][0]["verified"], 1)
        self.assertRegex(first["current_brief"] or "", r"^b\d+$")


if __name__ == "__main__":
    unittest.main()
