"""Tests for issue_log.py: merging sources, de-duplication and keeping reviewers' statuses."""

import sys
import unittest
from pathlib import Path

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


if __name__ == "__main__":
    unittest.main()
