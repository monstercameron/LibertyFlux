"""Tests for the pure parts of the small operating scripts: tick.py, start_lane.py, prod_report.py,
compare_regression.py and swap_worker.py. The parts that drive PowerShell, git or the lanes are not run."""

import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import compare_regression
import prod_report
import start_lane
import swap_worker
import tick

INTEGRATE_OUT = """\
r-s12: integrated as #production-s12: A finding
  abc1234 Devlog: A finding (1 files)
x-layout: NOT integrated (address in text)
k-checker4: integrated as #checker-v4: Checker version 4 (forced)
batch pushed
"""


class TestTick(unittest.TestCase):
    def test_lanes_from_integrate_output(self):
        self.assertEqual(tick.integrated_lanes(INTEGRATE_OUT), ["r-s12", "k-checker4"])
        self.assertEqual(tick.failed_lanes(INTEGRATE_OUT), ["x-layout"])
        self.assertEqual(tick.integrated_lanes(""), [])

    def test_summary_head(self):
        text = "\nfirst\n\n  second\n" + "x" * 300 + "\nfourth\n"
        head = tick.summary_head(text, 3)
        self.assertEqual(head[:2], ["  first", "    second"])
        self.assertEqual(len(head[2]), 202)
        self.assertEqual(len(tick.summary_head(text, 10)), 4)

    def test_sync_report_keeps_what_matters(self):
        text = ("added 3; re-checked under a later checker 0; total\nsomething else\nleft out by the scan 1\n"
                "abc Rewrites: 3 more functions pass the checker (9 in total) (5 files)\nFAILED: git commit\n"
                "demoted to rewrites/unverified by a later checker's failure or deferral 1\nschema warning: r-s1: 2 problems\n")
        self.assertEqual(len(tick.sync_report(text)), 6)
        self.assertNotIn("something else", tick.sync_report(text))


class TestStartLane(unittest.TestCase):
    def test_command(self):
        command = start_lane.lane_command("r-s5", "high", Path("scripts") / "coordinator")
        self.assertEqual(command[:5], ["powershell.exe", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
        self.assertTrue(command[5].endswith("run-lane.ps1"))
        self.assertEqual(command[6:], ["-Lane", "r-s5", "-Effort", "high"])

    def test_default_effort_is_max(self):
        import inspect
        self.assertEqual(inspect.signature(start_lane.start_lane).parameters["effort"].default, "max")


class TestProdReport(unittest.TestCase):
    def test_all_verified_forms_count(self):
        rows = [{"outcome": "verified", "minutes": 2}, {"outcome": "verified_v4", "minutes": 4},
                {"outcome": "deferred", "reason": "thread_local"}, {"outcome": "not_reached"}, "junk"]
        outcomes, reasons, minutes, lanes = prod_report.tally([("r-s1", "v4", rows), ("r-s2", "v4", [])])
        self.assertEqual(prod_report.verified_in(outcomes[("s", "v4")]), 2)
        self.assertEqual(sorted(minutes[("s", "v4")]), [2, 4])
        self.assertEqual(reasons[("s", "v4")]["thread_local"], 1)
        self.assertEqual(lanes[("s", "v4")], 2)
        lines = prod_report.report_lines(outcomes, reasons, minutes, lanes)
        self.assertTrue(lines[0].startswith("small functions, checker v4: 2 lanes, 2 of 4 verified (50%)"), lines[0])
        self.assertIn("thread_local", lines[1])

    def test_unknown_kind_does_not_crash(self):
        lines = prod_report.report_lines(*prod_report.tally([("r-x1", "v1", [{"outcome": "verified"}])]))
        self.assertIn("lanes r-x", lines[0])


class TestCompareRegression(unittest.TestCase):
    def test_compare(self):
        tracked = {"a.json": {"passed": True}, "b.json": {"passed": False}, "c.json": {"passed": True}}
        fresh = {"a.json": {"passed": True, "checker_version": "checker4"}, "b.json": {"passed": True, "first_mismatch": {"detail": "ret"}},
                 "d.json": {"passed": True, "worker_retries": 1}}
        lines, bad = compare_regression.compare(tracked, fresh, "verdicts")
        self.assertEqual(bad, 2)  # b differs, c missing; d is extra, not a mismatch
        self.assertIn("verdicts/b.json: tracked passed=False new passed=True (ret)", lines)
        self.assertIn("verdicts/c.json: missing from the new run", lines)
        self.assertIn("verdicts: 1 of 3 match; extra in new run 1; with retries ['d.json']", lines[-1])

    def test_parse_args(self):
        new, ref = compare_regression.parse_args(["run1", "--tracked", "ref"])
        self.assertEqual((new, ref), (Path("run1"), Path("ref")))
        self.assertEqual(compare_regression.parse_args(["run1"])[1].name, "checker")
        with self.assertRaises(SystemExit):
            compare_regression.parse_args(["run1", "--tracked"])
        with self.assertRaises(SystemExit):
            compare_regression.parse_args([])

    def test_main_on_folders(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            for side, passed in (("ref", True), ("new", False)):
                (tmp / side / "verdicts").mkdir(parents=True)
                (tmp / side / "verdicts" / "fn_1.json").write_text(json.dumps({"passed": passed}))
            out = io.StringIO()
            with contextlib.redirect_stdout(out):
                bad = compare_regression.main([str(tmp / "new"), "--tracked", str(tmp / "ref")])
        self.assertEqual(bad, 1)
        self.assertIn("no reference verdicts under ref/mutants", out.getvalue())


class TestSwapWorker(unittest.TestCase):
    def test_swap_and_refusals(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            new, live = tmp / "new" / "w.exe", tmp / "live" / "lf-checker-worker.exe"
            new.parent.mkdir()
            live.parent.mkdir()
            live.write_bytes(b"old")
            self.assertIsNone(swap_worker.swap(new, live, "v3"))  # no new build: nothing touched
            self.assertEqual(live.read_bytes(), b"old")
            new.write_bytes(b"newer")
            aside = swap_worker.swap(new, live, "v3")
            self.assertEqual((aside.name, aside.read_bytes(), live.read_bytes()), ("lf-checker-worker.v3.exe", b"old", b"newer"))
            self.assertIsNone(swap_worker.swap(new, live, "v3"))  # aside name taken: refused
            self.assertEqual(aside.read_bytes(), b"old")


if __name__ == "__main__":
    unittest.main()
