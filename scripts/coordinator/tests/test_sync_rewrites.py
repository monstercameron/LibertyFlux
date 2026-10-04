"""Tests for sync_rewrites.py on a synthetic scratch tree: import, demotion (behind --demote) and schema
warnings (behind --validate). The default run must behave as it always did."""

import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import sync_rewrites

A, B, C = 0x00510000, 0x00510040, 0x00510080


def key(address):
    return f"0x{address:08X}"


def rewrite_text(address):
    return f"// original: 0x{address:08X} f\nexport!(cdecl, rw() -> u32 {{ 0 }});\n"


class SyncTree(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        root = Path(self.tmp.name)
        self.scratch = root / ".artifacts" / "scratch"
        self.lists = self.scratch / "coordinator" / "lists"
        self.dest = root / "rewrites" / "verified"
        self.unverified = root / "rewrites" / "unverified"
        self.lists.mkdir(parents=True)
        self.patches = [mock.patch.object(sync_rewrites, name, value) for name, value in (
            ("ROOT", root), ("SCRATCH", self.scratch), ("COORD", self.scratch / "coordinator"), ("LISTS", self.lists),
            ("DEST", self.dest), ("UNVERIFIED", self.unverified))]
        for patch in self.patches:
            patch.start()

    def tearDown(self):
        for patch in self.patches:
            patch.stop()
        self.tmp.cleanup()

    def lane(self, name, rows, version=None, files=(), finished=True):
        folder = self.scratch / name
        (folder / "out" / "rewrites").mkdir(parents=True)
        (folder / "results.json").write_text(json.dumps(rows))
        if finished:
            (folder / "summary.txt").write_text("done\n")
        for address in files:
            (folder / "out" / "rewrites" / f"fn_{address:08x}.rs").write_text(rewrite_text(address))
        if version:
            (self.lists / f"{name}.{version}").write_text("marker\n")

    def run_sync(self, *flags):
        out = io.StringIO()
        with mock.patch.object(sys, "argv", ["sync_rewrites.py", *flags]), contextlib.redirect_stdout(out):
            sync_rewrites.main()
        return out.getvalue()

    def index(self, folder):
        path = folder / "index.json"
        return {e["address"]: e for e in json.loads(path.read_text())} if path.exists() else {}


class TestImport(SyncTree):
    def test_adds_passes_only(self):
        self.lane("r-s1", [{"address": key(A), "outcome": "verified_v4", "trials": 1000},
                           {"address": key(B), "outcome": "deferred", "reason": "other"}], "v4", files=[A, B])
        self.run_sync()
        index = self.index(self.dest)
        self.assertEqual(list(index), [key(A)])
        self.assertEqual(index[key(A)]["checker"], "version 4")
        self.assertTrue((self.dest / "functions" / f"fn_{A:08x}.rs").exists())

    def test_running_lane_skipped(self):
        self.lane("r-s1", [{"address": key(A), "outcome": "verified"}], "v4", files=[A], finished=False)
        self.run_sync()
        self.assertEqual(self.index(self.dest), {})

    def test_default_run_never_demotes(self):
        self.lane("a-1", [{"address": key(A), "outcome": "verified", "trials": 1000}], files=[A])  # version 2
        self.lane("r-s2", [{"address": key(A), "outcome": "deferred", "reason": "thread_local"}], "v4")
        out = self.run_sync()
        self.assertEqual(list(self.index(self.dest)), [key(A)])
        self.assertFalse(self.unverified.exists())
        self.assertNotIn("demoted", out)


class TestDemotion(SyncTree):
    def test_later_checker_deferral_demotes(self):
        self.lane("a-1", [{"address": key(A), "outcome": "verified", "trials": 1000, "name": "f"}], files=[A])
        self.run_sync()
        self.lane("r-s2", [{"address": key(A), "outcome": "deferred", "reason": "thread_local"}], "v4")
        out = self.run_sync("--demote")
        self.assertIn("demoted to rewrites/unverified by a later checker's failure or deferral 1", out)
        self.assertEqual(self.index(self.dest), {})
        entry = self.index(self.unverified)[key(A)]
        self.assertEqual((entry["outcome"], entry["reason"], entry["checker"], entry["lane"]),
                         ("deferred", "thread_local", "version 4", "r-s2"))
        self.assertEqual(entry["demoted_from"]["checker"], "version 2")
        self.assertEqual(entry["file"], f"fn_{A:08x}.rs")
        self.assertEqual((self.unverified / entry["file"]).read_text(), rewrite_text(A))  # file kept, moved
        self.assertFalse((self.dest / "functions" / f"fn_{A:08x}.rs").exists())
        # The old pass is still in scratch: further runs must not bring it back.
        self.run_sync("--demote")
        self.assertEqual(self.index(self.dest), {})
        self.assertIn(key(A), self.index(self.unverified))

    def test_same_or_older_checker_judgement_leaves_it(self):
        self.lane("r-s1", [{"address": key(A), "outcome": "verified_v4", "trials": 1000}], "v4", files=[A])
        self.lane("r-s2", [{"address": key(A), "outcome": "failed_v4"}], "v4")
        self.lane("r-s3", [{"address": key(A), "outcome": "deferred", "reason": "other"}], "v3")
        self.run_sync("--demote")
        self.assertEqual(list(self.index(self.dest)), [key(A)])
        self.assertEqual(self.index(self.unverified), {})

    def test_failed_outcome_demotes_and_not_reached_does_not(self):
        self.lane("a-1", [{"address": key(A), "outcome": "verified"}, {"address": key(B), "outcome": "verified"}], files=[A, B])
        self.lane("r-s2", [{"address": key(A), "outcome": "failed_v3"}, {"address": key(B), "outcome": "not_reached"}], "v3")
        self.run_sync("--demote")
        self.assertEqual(list(self.index(self.dest)), [key(B)])
        self.assertEqual(self.index(self.unverified)[key(A)]["reason"], "failed_v3")  # no reason given: the outcome

    def test_pass_under_demoting_version_lifts_demotion(self):
        self.lane("a-1", [{"address": key(A), "outcome": "verified"}], files=[A])
        self.lane("r-s2", [{"address": key(A), "outcome": "deferred", "reason": "other"}], "v3")
        self.run_sync("--demote")
        self.assertIn(key(A), self.index(self.unverified))
        self.lane("r-s3", [{"address": key(A), "outcome": "verified_v3", "trials": 1000}], "v3", files=[A])
        self.run_sync("--demote")
        self.assertEqual(self.index(self.dest)[key(A)]["checker"], "version 3")
        self.assertEqual(self.index(self.unverified), {})
        self.assertFalse((self.unverified / f"fn_{A:08x}.rs").exists())

    def test_other_unverified_entries_untouched(self):
        self.unverified.mkdir(parents=True)
        other = {"address": key(C), "name": None, "file": f"fn_{C:08x}.rs", "lane": "r-s9", "outcome": "not yet run", "reason": None}
        (self.unverified / "index.json").write_text(json.dumps([other], indent=1) + "\n")
        (self.unverified / other["file"]).write_text(rewrite_text(C))
        before = (self.unverified / "index.json").read_bytes()
        self.lane("a-1", [{"address": key(A), "outcome": "verified"}], files=[A])
        self.run_sync("--demote")
        self.assertEqual((self.unverified / "index.json").read_bytes(), before)  # nothing changed: not rewritten

    def test_missing_file_not_demoted(self):
        self.lane("a-1", [{"address": key(A), "outcome": "verified"}], files=[A])
        self.run_sync()
        (self.dest / "functions" / f"fn_{A:08x}.rs").unlink()
        self.lane("r-s2", [{"address": key(A), "outcome": "deferred", "reason": "other"}], "v4")
        out = self.run_sync("--demote")
        self.assertIn("not demoted because the file is missing 1", out)
        self.assertIn(key(A), self.index(self.dest))


class TestPureHelpers(unittest.TestCase):
    def test_judged_against(self):
        for outcome in ("deferred", "failed", "failed_v4"):
            self.assertTrue(sync_rewrites.judged_against({"outcome": outcome}))
        for outcome in ("verified", "not_reached", "not yet run", None):
            self.assertFalse(sync_rewrites.judged_against({"outcome": outcome}))

    def test_note_against_keeps_latest_version(self):
        against = {}
        sync_rewrites.note_against(against, "k", "version 3", "r-1", {})
        sync_rewrites.note_against(against, "k", "version 2", "r-2", {})
        sync_rewrites.note_against(against, "k", "version 4", "r-3", {})
        sync_rewrites.note_against(against, "k", "version 4", "r-4", {})
        self.assertEqual(against["k"][:2], ("version 4", "r-3"))


class TestValidateFlag(SyncTree):
    def test_warns_without_blocking(self):
        self.lane("r-s1", [{"address": key(A), "outcome": "verified_v4", "trials": 10}], "v4", files=[A])
        out = self.run_sync("--validate")
        self.assertIn("schema warning: r-s1:", out)
        self.assertIn(key(A), self.index(self.dest))  # imported all the same

    def test_silent_by_default(self):
        self.lane("r-s1", [{"address": key(A), "outcome": "verified_v4", "trials": 10}], "v4", files=[A])
        self.assertNotIn("schema warning", self.run_sync())


if __name__ == "__main__":
    unittest.main()
