"""Tests for issue_log.py: merging sources, de-duplication and keeping reviewers' statuses."""

import contextlib
import csv
import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import issue_log


def lint(code="header", severity="low", path="functions/fn_00401000.rs"):
    return {"file": path, "code": code, "severity": severity, "category": "quality", "confidence": "certain",
            "when": "now", "title": code, "detail": "d"}


class TestIssueLog(unittest.TestCase):
    def test_merge_orders_and_dedupes(self):
        issues = issue_log.from_lint([lint(), lint("partial", "high"), lint()])
        issues += issue_log.from_build([{"file": "functions/fn_00401000.rs", "error": "error[E0425]: x"}])
        files = issue_log.merge(issues)
        self.assertEqual(len(files), 1)
        self.assertEqual(files[0]["worst"], "high")
        self.assertEqual([i["source"] for i in files[0]["issues"]], ["build", "lint:partial", "lint:header"])

    def test_review_rows_are_validated(self):
        rows = [{"file": "functions/fn_00401000.rs", "severity": "medium", "category": "ub", "title": "t", "detail": "d",
                 "when": "post-bring-up", "label": "Verified", "family_size": 4},
                {"file": "functions/fn_00401010.rs", "severity": "urgent", "title": "bad severity"},
                {"severity": "low", "title": "no file"}]
        issues = issue_log.from_review(rows)
        self.assertEqual(len(issues), 1)
        self.assertEqual(issues[0]["family_size"], 4)
        self.assertEqual(issues[0]["confidence"], "Verified")

    def test_status_survives_regeneration(self):
        first = issue_log.merge(issue_log.from_lint([lint()]))
        first[0]["issues"][0]["status"] = "fixed"
        again = issue_log.merge(issue_log.from_lint([lint()]), {"files": first})
        self.assertEqual(again[0]["issues"][0]["status"], "fixed")

    def test_summary(self):
        files = issue_log.merge(issue_log.from_lint([lint(), lint("partial", "high", "functions/fn_00401010.rs")]))
        summary = issue_log.summarise(files)
        self.assertEqual(summary["files"], 2)
        self.assertEqual(summary["files_with_high"], 1)
        self.assertEqual(summary["by_source"], {"lint": 2})


def review(title, severity="medium", path="functions/fn_00401000.rs"):
    return {"file": path, "severity": severity, "category": "integration", "title": title, "detail": "d",
            "when": "now", "label": "Inferred"}


def sample_log():
    """A log as main writes it: lint, build and review issues over three files."""
    issues = issue_log.from_lint([lint(), lint("partial", "high", "functions/fn_00401010.rs"),
                                  lint("header", "low", "natives/fn_00401020.rs")])
    issues += issue_log.from_build([{"file": "functions/fn_00401010.rs",
                                     "error": "error[E0425]: cannot find function `helper` in this scope: not found in this scope"},
                                    {"file": "natives/fn_00401020.rs",
                                     "error": "error[E0425]: cannot find function `other` in this scope"}])
    issues += issue_log.from_review([review("Callee slots unnamed"), review("Callee slots unnamed", path="natives/fn_00401020.rs")])
    files = issue_log.merge(issues)
    return {"checked": 3, "summary": issue_log.summarise(files), "systemic": [], "files": files}


class TestFilters(unittest.TestCase):
    def test_parse(self):
        self.assertEqual(issue_log.parse_filters(["severity=high", "severity=low", "source=lint"]),
                         {"severity": {"high", "low"}, "source": {"lint"}})
        for bad in ("severity", "colour=red", "severity=hi", "status=done", "when="):
            with self.assertRaises(ValueError, msg=bad):
                issue_log.parse_filters([bad])

    def test_matching(self):
        log = sample_log()
        pick = lambda *items: [(p, i["source"]) for p, i in issue_log.flat_issues(log, issue_log.parse_filters(items))]
        self.assertEqual(len(pick()), 7)
        self.assertEqual({s for _, s in pick("source=lint")}, {"lint:header", "lint:partial"})
        self.assertEqual(pick("source=lint:partial"), [("functions/fn_00401010.rs", "lint:partial")])
        self.assertEqual(len(pick("source=build", "source=review")), 4)  # a repeated key allows any value
        self.assertEqual(pick("source=build", "severity=low"), [])  # different keys must all match
        self.assertEqual(len(pick("status=open")), 7)  # an issue without a status counts as open
        self.assertEqual(pick("when=lift"), [])


class TestSummaryReport(unittest.TestCase):
    def test_classes(self):
        cls = [issue_log.issue_class(i) for _, i in issue_log.flat_issues(sample_log())]
        self.assertIn("build: error[E0425]: cannot find function X in this scope", cls)
        self.assertEqual(cls.count("build: error[E0425]: cannot find function X in this scope"), 2)
        self.assertIn("review: Callee slots unnamed", cls)
        self.assertIn("lint:partial", cls)

    def test_high_first_with_examples_and_open_counts(self):
        log = sample_log()
        issue_log.mark(log, "natives/fn_00401020.rs", "build", "fixed")
        text = issue_log.render_summary(log)
        lines = text.splitlines()
        self.assertEqual(lines[0], "issue log: 7 issues in 3 files, 2 of them with a high-severity issue")
        self.assertIn("  by status: open 6, fixed 1", lines)
        sections = [line.split(":")[0] for line in lines if not line.startswith(" ")][1:]
        self.assertEqual(sections, ["high", "medium", "low"])
        build = next(line for line in lines if "build: error[E0425]" in line)
        # Two issues, one still open; the example is the open one.
        self.assertTrue(build.strip().startswith("2  build: error[E0425]"))
        self.assertTrue(build.endswith("(1 open)  e.g. functions/fn_00401010.rs"))
        filtered = issue_log.render_summary(log, issue_log.parse_filters(["severity=low"]))
        self.assertTrue(filtered.startswith("issue log (filtered: severity=low): 2 of 7 issues in 2 files"))
        self.assertIn("... and 1 more classes (1 issues)", issue_log.render_summary(log, top=1))


class TestCsv(unittest.TestCase):
    def test_one_row_per_issue(self):
        log = sample_log()
        log["files"][0]["issues"][0]["family_size"] = 4
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "issues.csv"
            issue_log.write_csv(issue_log.flat_issues(log), str(path))
            raw = path.read_bytes()
            rows = list(csv.DictReader(io.StringIO(raw.decode("utf-8"))))
        self.assertNotIn(b"\r\n", raw)
        self.assertEqual(list(rows[0]), list(issue_log.CSV_COLUMNS))
        self.assertEqual(len(rows), 7)
        self.assertEqual(rows[0]["family_size"], "4")
        self.assertEqual({r["status"] for r in rows}, {"open"})
        self.assertIn("cannot find function `helper`", " ".join(r["detail"] for r in rows))


class TestMark(unittest.TestCase):
    def test_by_source_or_title(self):
        log = sample_log()
        self.assertEqual(len(issue_log.mark(log, "functions/fn_00401010.rs", "lint:partial", "fixed")), 1)
        self.assertEqual(len(issue_log.mark(log, "rewrites\\verified\\natives\\fn_00401020.rs", "Callee slots unnamed", "wont-fix")), 1)
        self.assertEqual(issue_log.mark(log, "functions/fn_00401010.rs", "no such thing", "fixed"), [])
        statuses = {(p, i["source"]): i["status"] for p, i in issue_log.flat_issues(log)}
        self.assertEqual(statuses[("functions/fn_00401010.rs", "lint:partial")], "fixed")
        self.assertEqual(statuses[("natives/fn_00401020.rs", "review")], "wont-fix")
        self.assertEqual(sum(1 for s in statuses.values() if s == "open"), 5)
        with self.assertRaises(ValueError):
            issue_log.mark(log, "functions/fn_00401010.rs", "lint:partial", "done")

    def test_marked_issue_survives_regeneration(self):
        log = sample_log()
        issue_log.mark(log, "functions/fn_00401010.rs", "lint:partial", "not-an-issue")
        issue_log.mark(log, "natives/fn_00401020.rs", "Callee slots unnamed", "fixed")
        again = issue_log.merge(issue_log.from_lint([lint(), lint("partial", "high", "functions/fn_00401010.rs")])
                                + issue_log.from_review([review("Callee slots unnamed", path="natives/fn_00401020.rs")]), log)
        statuses = {(p, i["source"]): i["status"] for p, i in issue_log.flat_issues({"files": again})}
        self.assertEqual(statuses, {("functions/fn_00401000.rs", "lint:header"): "open",
                                    ("functions/fn_00401010.rs", "lint:partial"): "not-an-issue",
                                    ("natives/fn_00401020.rs", "review"): "fixed"})


HEADER_MISMATCH = "// original: 0x00401010 f\n/// Returns zero.\nexport!(cdecl, rw_00401000() -> u32 { 0 });\n"
CLEAN = "// original: 0x00401020 f\n/// Returns zero.\nexport!(cdecl, rw_00401020() -> u32 { 0 });\n"


class TestMainOnDisk(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        folder = self.root / "rewrites" / "verified"
        (folder / "functions").mkdir(parents=True)
        (folder / "functions" / "fn_00401000.rs").write_text(HEADER_MISMATCH, encoding="utf-8")
        (folder / "functions" / "fn_00401020.rs").write_text(CLEAN, encoding="utf-8")
        (folder / "index.json").write_text(json.dumps([{"address": "0x00401000", "file": "functions/fn_00401000.rs"},
                                                       {"address": "0x00401020", "file": "functions/fn_00401020.rs"}]))
        self.log = self.root / "rewrites" / "review" / "issues.json"
        self.env = mock.patch.dict(os.environ, {"LIBERTYFLUX_ROOT": str(self.root)})
        self.env.start()

    def tearDown(self):
        self.env.stop()
        self.tmp.cleanup()

    def run_main(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = issue_log.main(list(argv))
        return code, out.getvalue(), err.getvalue()

    def statuses(self):
        return [(e["file"], i["source"], i["status"]) for e in json.loads(self.log.read_text())["files"] for i in e["issues"]]

    def test_mark_then_regenerate_keeps_the_status(self):
        self.assertEqual(self.run_main()[0], 0)
        self.assertEqual(self.statuses(), [("functions/fn_00401000.rs", "lint:header", "open")])
        before = self.log.read_text()
        code, out, _ = self.run_main("--mark", "rewrites/verified/functions/fn_00401000.rs", "lint:header", "fixed")
        self.assertEqual((code, out.splitlines()[0]), (0, "marked 1 issue(s) of functions/fn_00401000.rs as fixed:"))
        after = self.log.read_text()
        self.assertEqual(after, before.replace('"status": "open"', '"status": "fixed"'))  # nothing else moves
        self.assertEqual(self.run_main()[0], 0)  # regenerate
        self.assertEqual(self.statuses(), [("functions/fn_00401000.rs", "lint:header", "fixed")])

    def test_read_modes_do_not_regenerate(self):
        code, _, err = self.run_main("--summary")
        self.assertEqual(code, 1)
        self.assertIn("no issue log", err)
        self.assertFalse(self.log.exists())
        self.run_main()
        stamp = self.log.stat().st_mtime_ns
        code, out, _ = self.run_main("--filter", "source=lint")
        self.assertEqual(code, 0)
        self.assertTrue(out.startswith("issue log (filtered: source=lint): 1 of 1 issues in 1 files"))
        code, out, _ = self.run_main("--csv", "-", "--filter", "severity=low")
        self.assertEqual(out.splitlines()[0], ",".join(issue_log.CSV_COLUMNS))
        self.assertEqual(len(out.splitlines()), 2)
        self.assertEqual(self.log.stat().st_mtime_ns, stamp)
        code, _, err = self.run_main("--mark", "functions/fn_00401000.rs", "lint:partial", "fixed")
        self.assertEqual(code, 1)
        self.assertIn("its issues: lint:header", err)
        for argv in (["--mark", "functions/fn_00401000.rs", "lint:header", "done"],
                     ["--mark", "functions/fn_00401000.rs", "lint:header", "fixed", "--summary"],
                     ["--summary", "--build", "b.json"], ["--filter", "severity=urgent"]):
            with self.assertRaises(SystemExit, msg=argv) as stop:
                self.run_main(*argv)
            self.assertEqual(stop.exception.code, 2)


if __name__ == "__main__":
    unittest.main()
