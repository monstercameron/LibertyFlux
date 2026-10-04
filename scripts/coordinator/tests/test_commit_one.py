"""Tests for commit_one.py in a throwaway git repository: the changelog entry and the commit are atomic."""

import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import commit_one

STUB_UPDATE = "import sys\nsys.exit(0)\n"


def git(root, *args):
    return subprocess.run(["git", *args], cwd=root, capture_output=True, text=True, check=True).stdout


class TestAddEntry(unittest.TestCase):
    def step(self, title="T"):
        return {"title": title, "changes": ["c"], "files": []}

    def test_on_top(self):
        entries = commit_one.add_entry([{"title": "old", "commit": "abc"}], self.step(), date="2026-10-04")
        self.assertEqual([e["title"] for e in entries], ["T", "old"])
        self.assertEqual(entries[0], {"date": "2026-10-04", "title": "T", "commit": None, "summary": "", "changes": ["c"]})

    def test_orphan_replaced_not_duplicated(self):
        entries = commit_one.add_entry([{"title": "T", "commit": None}, {"title": "old", "commit": "abc"}], self.step())
        self.assertEqual([e["title"] for e in entries], ["T", "old"])

    def test_committed_entry_with_same_title_kept(self):
        entries = commit_one.add_entry([{"title": "T", "commit": "abc"}], self.step())
        self.assertEqual([e["title"] for e in entries], ["T", "T"])


@unittest.skipUnless(subprocess.run(["git", "--version"], capture_output=True).returncode == 0, "git not available")
class TestCommit(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "docs" / "data").mkdir(parents=True)
        (self.root / "docs" / "badges").mkdir()
        (self.root / "scripts").mkdir()
        (self.root / "docs" / "data" / "changelog.json").write_text(json.dumps([{"title": "first", "commit": None}], indent=2) + "\n")
        (self.root / "docs" / "data" / "progress.json").write_text("{}\n")
        (self.root / "docs" / "badges" / "b.svg").write_text("<svg/>\n")
        (self.root / "scripts" / "update_progress.py").write_text(STUB_UPDATE)
        git(self.root, "init", "-q")
        git(self.root, "config", "user.email", "test@example.invalid")
        git(self.root, "config", "user.name", "test")
        git(self.root, "config", "commit.gpgsign", "false")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "first")
        (self.root / "work.txt").write_text("new work\n")
        self.step = {"title": "Area: one change", "summary": "s", "changes": ["did it"], "files": ["work.txt"]}

    def tearDown(self):
        self.tmp.cleanup()

    def quiet(self, step):
        with contextlib.redirect_stdout(io.StringIO()):
            return commit_one.commit(step, self.root, push=False)

    def changelog(self):
        return (self.root / "docs" / "data" / "changelog.json").read_bytes()

    def test_success_commits_only_named_paths(self):
        (self.root / "other.txt").write_text("staged by someone else\n")
        git(self.root, "add", "other.txt")
        self.quiet(self.step)
        self.assertEqual(git(self.root, "log", "-1", "--format=%s").strip(), "Area: one change")
        files = git(self.root, "show", "--name-only", "--format=", "HEAD").split()
        self.assertEqual(sorted(files), ["docs/data/changelog.json", "work.txt"])
        self.assertIn("A  other.txt", git(self.root, "status", "--short"))
        top = json.loads(self.changelog())[0]
        self.assertEqual((top["title"], top["commit"]), ("Area: one change", None))

    def test_directory_with_new_and_deleted_files(self):
        folder = self.root / "rewrites" / "verified"
        folder.mkdir(parents=True)
        (folder / "old.rs").write_text("old\n")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "tree")
        (folder / "old.rs").unlink()
        (folder / "new.rs").write_text("new\n")
        self.quiet(dict(self.step, files=["rewrites/verified"]))
        tree = git(self.root, "ls-tree", "-r", "--name-only", "HEAD").split()
        self.assertIn("rewrites/verified/new.rs", tree)
        self.assertNotIn("rewrites/verified/old.rs", tree)
        self.assertEqual(git(self.root, "status", "--short", "--", "rewrites"), "")

    def test_failed_commit_rolls_back_changelog(self):
        before = self.changelog()
        hook = self.root / ".git" / "hooks" / "pre-commit"
        hook.write_text("#!/bin/sh\nexit 1\n")
        hook.chmod(0o755)
        if os.name == "nt":
            self.skipTest("hook execution differs on Windows")
        with self.assertRaises(commit_one.StepFailed):
            self.quiet(self.step)
        self.assertEqual(self.changelog(), before)
        self.assertNotIn("changelog.json", git(self.root, "diff", "--cached", "--name-only"))
        self.assertEqual(git(self.root, "log", "--format=%s").split("\n")[0], "first")
        # A second attempt after the cause is fixed adds exactly one entry.
        hook.unlink()
        self.quiet(self.step)
        titles = [e["title"] for e in json.loads(self.changelog())]
        self.assertEqual(titles, ["Area: one change", "first"])

    def test_failed_regeneration_rolls_back(self):
        before = self.changelog()
        (self.root / "scripts" / "update_progress.py").write_text("import sys\nsys.exit(3)\n")
        with self.assertRaises(commit_one.StepFailed):
            self.quiet(self.step)
        self.assertEqual(self.changelog(), before)

    def test_missing_file_rolls_back(self):
        before = self.changelog()
        step = dict(self.step, files=["does-not-exist.txt"])
        with self.assertRaises(commit_one.StepFailed):
            self.quiet(step)
        self.assertEqual(self.changelog(), before)


if __name__ == "__main__":
    unittest.main()
