"""Tests for quality.py: the class rules, the aggregates, label hygiene and the guard against per-file detail."""

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import quality

# Strings the guard must refuse, assembled at run time so this file itself passes the publication check.
ADDRESS = "0x" + "00A1B2C3"
FILE_NAME = "fn_" + "00a1b2c3.rs"


def issue(title, severity="high", category="integration", source="review", detail="", when="post-bring-up"):
    return {"title": title, "severity": severity, "category": category, "source": source, "detail": detail,
            "when": when, "status": "open", "confidence": "Inferred"}


def log(files, systemic=None, checked=10):
    return {"checked": checked, "systemic": systemic or [],
            "files": [{"file": name, "worst": min((i["severity"] for i in issues),
                                                  key=quality.SEVERITIES.index), "issues": issues}
                      for name, issues in files]}


def rule(key):
    return next(r for k, r, _ in quality.CLASSES if k == key)


class TestClassRules(unittest.TestCase):
    def test_lane_runtime_from_build_errors_and_reviews(self):
        self.assertTrue(rule("lane-runtime")(issue("Does not compile from the repository", source="build",
                                                   detail="error[E0433]: cannot find module or crate")))
        self.assertTrue(rule("lane-runtime")(issue("Written against lf_k2_rt, a lane runtime crate")))
        self.assertFalse(rule("lane-runtime")(issue("Does not compile", source="build",
                                                    detail="error: this file contains an unclosed delimiter")))

    def test_stack_cookie_ignores_a_helper_named_cookie(self):
        self.assertTrue(rule("stack-cookie")(issue("Stack-cookie check called without the ECX cookie")))
        self.assertFalse(rule("stack-cookie")(issue("Undefined helpers cookie and round_trunc")))
        self.assertFalse(rule("stack-cookie")(issue("Helper cookie14 not defined")))

    def test_out_buffer_and_float_register(self):
        self.assertTrue(rule("out-buffer")(issue("Leaderboard query out-buffer of 5 words; helper fills at least six")))
        self.assertFalse(rule("out-buffer")(issue("Helper out-slot is immutable but passed as a pointer")))
        self.assertTrue(rule("float-register")(issue("Angle helper's float result taken from EAX")))
        self.assertTrue(rule("float-register")(issue("Engine ceiling routine returns in ST0; rewrite converts EAX")))
        self.assertFalse(rule("float-register")(issue("NaN routing returns signalling NaNs unquieted", category="float")))

    def test_partial_misaligned_and_overflow(self):
        self.assertTrue(rule("partial-proof")(issue("anything", category="narrow-proof")))
        self.assertTrue(rule("misaligned-deref")(issue("Aligned dereference at an odd offset", source="lint:misaligned-deref")))
        self.assertTrue(rule("misaligned-deref")(issue("Plain u32 store at odd offset", category="ub")))
        self.assertTrue(rule("debug-overflow")(issue("Checked + on address values (obj)", category="panic")))
        self.assertFalse(rule("debug-overflow")(issue("Checked + on address values (obj)", category="ub")))


class TestAggregates(unittest.TestCase):
    def test_issue_section_counts_files_once_and_keeps_no_names(self):
        data = log([("a", [issue("Stack-cookie check with no cookie"), issue("Float result taken from EAX", "low")]),
                    ("b", [issue("Stack-cookie check passed 0", "medium")])],
                   systemic=[{"code": "plain-deref", "files": 7, "severity": "high", "category": "ub",
                              "when": "post-bring-up", "title": "Plain dereferences", "detail": "Aligned loads."}])
        section = quality.issues_section(data)
        self.assertEqual((section["files"], section["issues"], section["files_with_high"]), (2, 3, 1))
        self.assertEqual(list(section["by_severity"]), ["high", "medium", "low"])
        cookie = next(c for c in section["classes"] if c["key"] == "stack-cookie")
        self.assertEqual((cookie["files"], cookie["issues"]), (2, 2))
        misaligned = next(c for c in section["classes"] if c["key"] == "misaligned-deref")
        self.assertEqual(misaligned["systemic_files"], 7)
        self.assertNotIn('"a"', json.dumps(section))

    def test_tree_section_through_the_ledger(self):
        verified = [{"address": "0x10000000", "kind": "function", "checker": "version 4", "trials": 1000, "lane": "x",
                     "file": "f/fn_10000000.rs"},
                    {"address": "0x10000010", "kind": "native", "checker": "version 1", "trials": 2000, "lane": "y",
                     "file": "f/fn_10000010.rs"}]
        unverified = [{"address": "0x10000020", "outcome": "deferred", "reason": "needs_encrypted_code"},
                      {"address": "0x10000030", "outcome": "not yet run", "reason": None},
                      {"address": "0x10000040", "outcome": "deferred", "reason": "a sentence a lane wrote at " + ADDRESS}]
        tree = quality.tree_section(verified, unverified)
        self.assertEqual((tree["verified"], tree["modern_checker"], tree["unverified"]), (2, 1, 3))
        self.assertEqual(tree["trials"], {"with_count": 2, "at_least_1000": 2, "least": 1000})
        self.assertEqual(tree["unverified_by_reason"],
                         {"needs_encrypted_code": 1, "none recorded": 1, "other (unlisted)": 1})
        self.assertEqual(quality.leaks(tree), [])

    def test_build_section_families(self):
        section = quality.build_section({"checked": 10, "edition": "2024", "failed": [
            {"file": FILE_NAME, "error": "error[E0432]: unresolved import `x`"},
            {"file": FILE_NAME, "error": "error: expected identifier, found reserved keyword `gen`"},
            {"file": FILE_NAME, "error": "error[E0599]: no method"}]})
        self.assertEqual((section["compile"], section["fail"], section["edition"]), (7, 3, "2024"))
        self.assertEqual(section["families"], {"missing-helper": 1, "syntax-or-edition": 1, "other": 1})
        self.assertEqual(quality.leaks(section), [])


class TestHygiene(unittest.TestCase):
    def test_labels(self):
        self.assertEqual(quality.label("not_reached"), "not_reached")
        self.assertEqual(quality.label(None), "none recorded")
        self.assertEqual(quality.label("Something a lane typed, at length"), "other (unlisted)")
        self.assertEqual(quality.label("other", quality.REASONS), "other")
        self.assertEqual(quality.label("invented_reason", quality.REASONS), "other (unlisted)")

    def test_guard_finds_addresses_file_names_and_paths(self):
        found = quality.leaks({"a": [ADDRESS], "b": {"c": FILE_NAME}, "d": "/ho" + "me/alice/x"})
        self.assertEqual(len(found), 3)
        self.assertEqual(quality.leaks({"detail": "convert to read_unaligned (1046 files already do)"}), [])


class TestRepository(unittest.TestCase):
    def test_document_from_this_checkout_is_clean_and_consistent(self):
        with tempfile.TemporaryDirectory() as folder:
            document = quality.build(Path(folder) / "absent.json")
        self.assertIsNone(document["build"])
        self.assertEqual(quality.leaks(document), [])
        issues = document["issues"]
        self.assertEqual(sum(issues["by_severity"].values()), issues["issues"])
        self.assertLessEqual(issues["files_with_high"], issues["files"])
        tree = document["tree"]
        self.assertEqual(sum(tree["by_kind"].values()), tree["verified"])
        self.assertEqual(sum(tree["unverified_by_outcome"].values()), tree["unverified"])
        self.assertEqual([c["key"] for c in issues["classes"]], [k for k, _, _ in quality.CLASSES])


if __name__ == "__main__":
    unittest.main()
