"""Tests for ledger.py on synthetic trees."""

import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

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


def proof(**edits):
    """A complete, consistent proof record with `edits` applied (None removes a key)."""
    record = {"checker_version": "checker4", "worker_hash": "a" * 40, "driver_hash": "b" * 40,
              "rewrite_hash": "c" * 64, "contract_hash": "d" * 40, "seed": 12648430, "trials": 1000,
              "mutant": {"caught": True, "fails": 412, "trials": 1000, "export": "mut_x"},
              "narrowing": [], "partial": False}
    for key, value in edits.items():
        if value is None:
            record.pop(key, None)
        else:
            record[key] = value
    return record


class TestProofRecords(unittest.TestCase):
    SCHEMA = ledger.load_proof_schema()

    def check(self, rows):
        return ledger.check_proofs(rows, self.SCHEMA)

    def test_schema_is_tracked(self):
        self.assertIsNotNone(self.SCHEMA)

    def test_absent_proofs_are_fine(self):
        self.assertEqual(self.check([row(0), row(1)]), ([], [], []))

    def test_valid_proof_is_counted_and_exposed(self):
        rows = [row(0, proof=proof()), row(1)]
        errors, warnings, records = self.check(rows)
        self.assertEqual((errors, warnings), ([], []))
        self.assertEqual([r["address"] for r in records], [rows[0]["address"]])
        summary = ledger.summarise_proofs(rows, records)
        self.assertEqual((summary["with_proof"], summary["without_proof"], summary["mutant_caught"]), (1, 1, 1))

    def test_malformed_proofs(self):
        for bad in (proof(seed=None), proof(worker_hash="xyz"), proof(trials="1000"),
                    proof(mutant={"caught": True}), proof(extra=1), "not an object"):
            errors, _, records = self.check([row(0, proof=bad)])
            self.assertEqual([k for k, _ in errors], ["bad proof record"], bad)
            self.assertEqual(len(records), 1)

    def test_consistency_rules(self):
        errors, _, _ = self.check([row(0, proof=proof(mutant={"caught": False, "fails": 0}))])
        self.assertEqual([k for k, _ in errors], ["proof: wrong version not caught"])
        errors, _, _ = self.check([row(0, proof=proof(mutant={"caught": True, "fails": 0}))])
        self.assertEqual([k for k, _ in errors], ["proof: wrong version caught with no failing trial"])
        _, warnings, _ = self.check([row(0, proof=proof(partial=True, narrowing=["call-skip:3"], trials=200))])
        self.assertEqual(sorted(k for k, _ in warnings), ["proof: partial", "proof: trial count differs from the entry's"])
        rows = [row(0, proof=proof(narrowing=["call-skip:3"]))]
        self.assertEqual(ledger.summarise_proofs(rows, self.check(rows)[2])["narrowed"], 1)

    def test_without_schema_proofs_are_only_counted(self):
        errors, warnings, records = ledger.check_proofs([row(0, proof={"anything": 1})], None)
        self.assertEqual((errors, warnings, len(records)), ([], [], 1))

    def test_build_reports_proofs(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            verified = [row(0, proof=proof()), row(1, proof=proof(mutant={"caught": False, "fails": 0}))]
            folder = root / "rewrites" / "verified"
            for r in verified:
                (folder / r["file"]).parent.mkdir(parents=True, exist_ok=True)
                (folder / r["file"]).write_text(text_for(r["file"]), encoding="utf-8")
            (folder / "index.json").write_text(json.dumps(verified), encoding="utf-8")
            (root / "docs" / "data").mkdir(parents=True)
            (root / "docs" / "data" / "progress.json").write_text(json.dumps({"stages": {"verified": 2, "rewritten": 2}}), encoding="utf-8")
            built = ledger.build(root)
            self.assertEqual(built["summary"]["proofs"]["with_proof"], 2)
            self.assertEqual(len(built["proofs"]), 2)
            self.assertEqual([e["kind"] for e in built["errors"]], ["proof: wrong version not caught"])
            out = io.StringIO()
            with contextlib.redirect_stdout(out):
                ledger.report(built)
            self.assertIn("proof records: 2 of 2", out.getvalue())


class TestDiffIndex(unittest.TestCase):
    def test_added_removed_changed(self):
        old = [row(0), row(1), row(2, checker="version 2"), {"address": "not hex"}, {"file": "no address"}]
        new = [row(0), row(2, checker="version 4", trials=500), row(3, kind="native", lane="b-2"), row(4)]
        diff = ledger.diff_index(old, new)
        self.assertEqual([r["name"] for r in diff["added"]], ["f3", "f4"])
        self.assertEqual([r["name"] for r in diff["removed"]], ["f1"])
        self.assertEqual([(a["name"], fields) for a, _, fields in diff["changed"]], [("f2", ["checker", "trials"])])
        s = ledger.summarise_diff(diff)
        self.assertEqual((s["added"], s["removed"], s["changed"]), (2, 1, 1))
        self.assertEqual(s["added_by_kind"], {"function": 1, "native": 1})
        self.assertEqual(s["added_by_kind_and_checker"], {"function": {"version 2": 1}, "native": {"version 2": 1}})
        self.assertEqual(s["removed_by_checker"], {"version 2": 1})
        self.assertEqual(s["changed_fields"], {"checker": 1, "trials": 1})
        self.assertEqual(s["checker_transitions"], {"version 2 -> version 4": 1})
        self.assertEqual(s["added_by_lane"], {"a-1": 1, "b-2": 1})

    def test_identical_and_empty(self):
        self.assertEqual(ledger.diff_index([row(0)], [row(0)]), {"added": [], "removed": [], "changed": []})
        self.assertEqual(len(ledger.diff_index([], [row(0), row(1)])["added"]), 2)

    def test_markdown_is_counts_only(self):
        record = ledger.since_record("HEAD~1", [row(0), row(1), row(2)], True,
                                     [row(0, checker="version 4"), row(2), row(3, kind="native")])
        text = ledger.markdown_since(record)
        lines = text.splitlines()
        self.assertEqual(lines[0], "| Since `HEAD~1` | Entries | function | native | version 2 | version 4 |")
        self.assertEqual(lines[2:5], ["| Added | 1 | 0 | 1 | 1 | 0 |", "| Removed | 1 | 1 | 0 | 1 | 0 |",
                                      "| Changed | 1 | 1 | 0 | 0 | 1 |"])
        self.assertIn("Checker changes: version 2 -> version 4 (1).", text)
        self.assertNotRegex(text, r"0x|fn_[0-9a-f]")  # no addresses or file names: it may go to the devlog
        self.assertEqual(record["changed"][0]["before"], {"checker": "version 2"})


def git(root, *args):
    return subprocess.run(["git", *args], cwd=root, capture_output=True, text=True, check=True).stdout


@unittest.skipUnless(subprocess.run(["git", "--version"], capture_output=True).returncode == 0, "git not available")
class TestSinceInARepository(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "docs" / "data").mkdir(parents=True)
        (self.root / "docs" / "data" / "progress.json").write_text(json.dumps({"stages": {"verified": 3, "rewritten": 3}}))
        git(self.root, "init", "-q")
        git(self.root, "config", "user.email", "test@example.invalid")
        git(self.root, "config", "user.name", "test")
        git(self.root, "config", "commit.gpgsign", "false")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "before the index")
        self.write_tree([row(0), row(1), row(2)])
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "first wave")
        self.write_tree([row(0, checker="version 4"), row(2), row(3, kind="native")])  # left uncommitted
        self.env = mock.patch.dict(os.environ, {"LIBERTYFLUX_ROOT": str(self.root)})
        self.env.start()

    def tearDown(self):
        self.env.stop()
        self.tmp.cleanup()

    def write_tree(self, rows):
        folder = self.root / "rewrites" / "verified"
        for path in folder.rglob("*.rs"):
            path.unlink()
        for r in rows:
            (folder / r["file"]).parent.mkdir(parents=True, exist_ok=True)
            (folder / r["file"]).write_text(text_for(r["file"]), encoding="utf-8")
        (folder / "index.json").write_text(json.dumps(rows), encoding="utf-8")

    def run_main(self, *argv):
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = ledger.main(list(argv))
        return code, out.getvalue(), err.getvalue()

    def test_text_report_gains_a_section(self):
        code, plain, _ = self.run_main()
        code_since, out, _ = self.run_main("--since", "HEAD", "--check")
        self.assertEqual((code, code_since), (0, 0))
        self.assertTrue(out.startswith(plain))  # the usual report, unchanged, comes first
        section = out[len(plain):].splitlines()
        self.assertEqual(section[0], "since HEAD: 3 entries at the ref, 3 now")
        self.assertTrue(section[1].startswith("  added 1: by kind {'native': 1}; by checker {'version 2': 1}; from 1 lanes (a-1 1)"))
        self.assertTrue(section[2].startswith("  removed 1: by kind {'function': 1}"))
        self.assertIn(row(1)["file"], section[2])
        self.assertTrue(section[3].startswith("  changed 1: fields {'checker': 1}; checker version 2 -> version 4: 1"))

    def test_json_markdown_and_errors(self):
        code, out, _ = self.run_main("--since", "HEAD~1", "--json")
        since = json.loads(out)["since"]
        self.assertEqual((since["present_at_ref"], since["before"], since["after"]), (False, 0, 3))
        self.assertEqual(since["summary"]["added"], 3)
        _, plain_json, _ = self.run_main("--json")
        self.assertNotIn("since", json.loads(plain_json))
        code, out, _ = self.run_main("--since", "HEAD", "--markdown")
        self.assertTrue(out.startswith("| Since `HEAD` | Entries |"))
        code, out, _ = self.run_main("--markdown")
        self.assertTrue(out.startswith("| Ledger | Count |\n|---|---:|\n| Verified entries | 3 |"))
        code, _, err = self.run_main("--since", "no-such-ref")
        self.assertEqual(code, 2)
        self.assertIn("does not name a commit", err)
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            ledger.main(["--json", "--markdown"])

    def test_check_keeps_its_meaning(self):
        (self.root / "rewrites" / "verified" / row(2)["file"]).unlink()
        self.assertEqual(self.run_main("--since", "HEAD", "--check")[0], 1)
        self.assertEqual(self.run_main("--since", "HEAD", "--check", "--markdown")[0], 1)
        self.assertEqual(self.run_main("--since", "HEAD")[0], 0)


if __name__ == "__main__":
    unittest.main()
