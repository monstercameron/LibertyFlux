"""Tests for ledger.py on synthetic trees."""

import contextlib
import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import ledger

BASE = 0x10000000


def row(i, checker="version 2", kind="function", trials=1000, **extra):
    address = BASE + i * 0x10
    return dict({"address": f"0x{address:08X}", "name": f"f{i}", "kind": kind, "checker": checker,
                 "file": f"{kind}s/fn_{address:08x}.rs", "trials": trials, "lane": "a-1"}, **extra)


def text_for(path):
    return f"// original: 0x{int(Path(path).stem[3:], 16):08X} f\nexport!(cdecl, rw() -> u32 {{ 0 }});\n"


class TestHeaderAddress(unittest.TestCase):
    def test_reads_first_non_blank_line(self):
        self.assertEqual(ledger.header_address("\n// original: 0x00401000 f\n"), 0x00401000)

    def test_malformed_and_missing(self):
        self.assertIsNone(ledger.header_address("// original: 0x0x00401000 f\n"))
        self.assertIsNone(ledger.header_address("/// doc first\n// original: 0x00401000\n"))
        self.assertIsNone(ledger.header_address(""))


class TestCheckTree(unittest.TestCase):
    def check(self, verified, unverified=(), verified_files=None, unverified_files=None, read=text_for):
        verified_files = {r["file"] for r in verified} if verified_files is None else verified_files
        unverified_files = {r["file"] for r in unverified} if unverified_files is None else unverified_files
        return ledger.check_tree(list(verified), list(unverified), verified_files, unverified_files, read)

    def kinds(self, items):
        return sorted(k for k, _ in items)

    def test_clean_tree(self):
        self.assertEqual(self.check([row(0), row(1)]), ([], []))

    def test_missing_and_unindexed_files(self):
        errors, _ = self.check([row(0), row(1)], verified_files={row(0)["file"], "functions/fn_deadbeef.rs"})
        self.assertEqual(self.kinds(errors), ["file not in index", "missing file"])

    def test_duplicates(self):
        errors, _ = self.check([row(0), row(0)])
        self.assertIn("address listed twice", self.kinds(errors))
        self.assertIn("file listed twice", self.kinds(errors))

    def test_both_halves(self):
        unverified = [dict(row(0), file=f"fn_{BASE:08x}.rs", outcome="deferred")]
        errors, _ = self.check([row(0)], unverified)
        self.assertEqual(self.kinds(errors), ["both verified and unverified"])

    def test_file_name_must_match_address(self):
        errors, _ = self.check([dict(row(0), file="functions/fn_00000001.rs")])
        self.assertEqual(self.kinds(errors), ["file name disagrees with address"])

    def test_warnings(self):
        rows = [row(0, checker="version 1"), row(1, trials=None), row(2)]
        bad = rows[2]["file"]
        _, warnings = self.check(rows, read=lambda p: "// original: 0x0x1\n" if p == bad else text_for(p))
        self.assertEqual(self.kinds(warnings), ["checked only by checker version 1", "header missing or malformed", "no trial count"])

    def test_header_names_another_address(self):
        _, warnings = self.check([row(0)], read=lambda p: "// original: 0x00401000 f\n")
        self.assertEqual(self.kinds(warnings), ["header names another address"])


class TestSummaryAndReconcile(unittest.TestCase):
    def test_counts_and_game_restriction(self):
        verified = [row(0), row(1, checker="version 1"), row(2, kind="native"), row(3, checker="version 4")]
        unverified = [{"address": "0x20000000", "outcome": "deferred", "reason": "checker_gap"},
                      {"address": "0x20000010", "outcome": "not yet run", "reason": None}]
        game = {BASE, BASE + 0x20}
        s = ledger.summarise(verified, unverified, game)
        self.assertEqual(s["modern_checker"], 3)
        self.assertEqual(s["verified_game"], 2)
        self.assertEqual(s["verified_outside_game"], 1)
        self.assertEqual(s["unverified_by_reason"], {"checker_gap": 1, "none recorded": 1})
        rows = ledger.reconcile(s, {"verified": 2, "rewritten": 4})
        self.assertEqual(rows[0][1:3], (2, 2))
        self.assertEqual(rows[0][3], "same definition")
        self.assertEqual(rows[1][3], "ok")

    def test_reports_a_difference(self):
        s = ledger.summarise([row(0), row(1)], [])
        verified_row, rewritten_row = ledger.reconcile(s, {"verified": 1, "rewritten": 1})
        self.assertIn("differs by +1", verified_row[3])
        self.assertNotEqual(rewritten_row[3], "ok")


class TestBuildOnDisk(unittest.TestCase):
    def test_build_and_check_exit(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            verified = [row(0), row(1)]
            folder = root / "rewrites" / "verified"
            for r in verified:
                (folder / r["file"]).parent.mkdir(parents=True, exist_ok=True)
                (folder / r["file"]).write_text(text_for(r["file"]), encoding="utf-8")
            (folder / "index.json").write_text(json.dumps(verified), encoding="utf-8")
            (root / "docs" / "data").mkdir(parents=True)
            (root / "docs" / "data" / "progress.json").write_text(json.dumps({"stages": {"verified": 2, "rewritten": 2}}), encoding="utf-8")
            inventory = root / "inventory.json"
            inventory.write_text(json.dumps([{"start": f"0x{BASE:08X}", "kind": "game"},
                                             {"start": f"0x{BASE + 0x10:08X}", "kind": "library"}]), encoding="utf-8")
            old = os.environ.get("LIBERTYFLUX_ROOT")
            os.environ["LIBERTYFLUX_ROOT"] = str(root)
            try:
                built = ledger.build(root, str(inventory))
                self.assertEqual(built["errors"], [])
                self.assertEqual(built["summary"]["verified_game"], 1)
                with contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(ledger.main(["--check", "--json"]), 0)
                    (folder / verified[0]["file"]).unlink()
                    self.assertEqual(ledger.main(["--check", "--json"]), 1)
            finally:
                if old is None:
                    os.environ.pop("LIBERTYFLUX_ROOT", None)
                else:
                    os.environ["LIBERTYFLUX_ROOT"] = old


if __name__ == "__main__":
    unittest.main()
