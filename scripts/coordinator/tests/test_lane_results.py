"""Tests for lane_results.py and lane_results.schema.json."""

import contextlib
import io
import json
import re
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import lane_results
import scan

GOOD_PASS = {"address": "0x00509FF0", "name": "f", "name_source": "proposed", "size": 40, "outcome": "verified",
             "reason": None, "detail": "", "trials": 1000, "attempts": 1, "minutes": 3.5, "rust_lines": 20,
             "checker": "v4", "brief": "b8", "mutant_caught": True, "mutant": "dropped the store", "narrowed": [],
             "calls": 0, "touches_globals": False, "uses_float": False}


def fields(problems):
    return sorted(field for _, field, _ in problems)


class TestSchemaFile(unittest.TestCase):
    def test_only_supported_keywords(self):
        self.assertEqual(lane_results.unsupported_keywords(lane_results.load_schema()), [])

    def test_unsupported_keyword_is_named(self):
        schema = {"properties": {"a": {"maxLength": 3}}, "oneOf": []}
        self.assertEqual(lane_results.unsupported_keywords(schema), ["#/properties/a/maxLength", "#/oneOf"])

    def test_outcomes_cover_what_the_code_accepts(self):
        outcomes = set(lane_results.load_schema()["properties"]["outcome"]["enum"])
        self.assertTrue(scan.PASSED <= outcomes)
        self.assertTrue({"deferred", "not_reached", "not yet run"} <= outcomes)

    def test_reason_codes_from_the_brief_are_documented(self):
        brief = (Path(lane_results.__file__).parent / "briefs" / "production.txt").read_text(encoding="utf-8")
        codes = set(re.findall(r"`([a-z_]+)` \(", brief.split("exact reason codes")[1].split("\n")[0]))
        codes.add("other")
        description = lane_results.load_schema()["properties"]["reason"]["description"]
        self.assertTrue(codes, "no reason codes found in the brief")
        for code in codes:
            self.assertIn(code, description)


class TestValidate(unittest.TestCase):
    def test_good_pass_and_deferral(self):
        deferral = dict(GOOD_PASS, outcome="deferred", reason="thread_local", mutant_caught=None, narrowed=None, trials=None)
        self.assertEqual(lane_results.validate_rows([GOOD_PASS, deferral]), [])

    def test_minimal_not_reached_and_numeric_address(self):
        self.assertEqual(lane_results.validate_rows([{"address": 0x401000, "outcome": "not_reached"}]), [])

    def test_missing_required(self):
        self.assertEqual(fields(lane_results.validate_rows([{"name": "x"}])), ["/address", "/outcome"])

    def test_bad_outcome_address_and_types(self):
        row = dict(GOOD_PASS, outcome="passed", address="00509FF0", trials="1000", uses_float=1)
        self.assertEqual(fields(lane_results.validate_rows([row])), ["/address", "/outcome", "/trials", "/uses_float"])

    def test_booleans_are_not_numbers(self):
        self.assertEqual(fields(lane_results.validate_rows([dict(GOOD_PASS, minutes=True)])), ["/minutes"])

    def test_deferral_needs_reason(self):
        self.assertEqual(fields(lane_results.validate_rows([{"address": "0x1", "outcome": "deferred"}])), ["/reason"])
        self.assertEqual(fields(lane_results.validate_rows([{"address": "0x1", "outcome": "deferred", "reason": ""}])), ["/reason"])

    def test_pass_needs_proof_fields(self):
        problems = lane_results.validate_rows([{"address": "0x1", "outcome": "verified_v4", "trials": 999, "mutant_caught": False}])
        self.assertEqual(fields(problems), ["/brief", "/checker", "/mutant_caught", "/narrowed", "/trials"])

    def test_narrowed_items_are_strings(self):
        self.assertEqual(fields(lane_results.validate_rows([dict(GOOD_PASS, narrowed=["masked arg 2", 3])])), ["/narrowed/1"])

    def test_non_object_row(self):
        self.assertEqual(lane_results.validate_rows(["x"])[0][:2], (0, "/"))

    def test_extra_fields_allowed(self):
        self.assertEqual(lane_results.validate_rows([dict(GOOD_PASS, private_worker=None, source_lane="r-s1", extra=1)]), [])


class TestFilesAndSummary(unittest.TestCase):
    def test_validate_file_shapes(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "results.json"
            path.write_text(json.dumps({"functions": [GOOD_PASS]}))
            self.assertEqual(lane_results.validate_file(path), [])
            path.write_text("{")
            self.assertIn("unreadable", lane_results.validate_file(path)[0][2])
            path.write_text(json.dumps({"other": 1}))
            self.assertEqual(len(lane_results.validate_file(path)), 1)

    def test_summarise(self):
        self.assertIsNone(lane_results.summarise("r-s1", []))
        line = lane_results.summarise("r-s1", lane_results.validate_rows([{"name": 1}, {"name": 2}]))
        self.assertTrue(line.startswith("schema warning: r-s1: 6 problems in 2 rows"), line)

    def test_main_never_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "results.json"
            path.write_text("[1]")
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(lane_results.main([str(path)]), 0)


if __name__ == "__main__":
    unittest.main()
