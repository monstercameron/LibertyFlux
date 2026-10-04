"""Tests for scripts/hooks: the installer, and the pre-commit hook run by real commits in throwaway repositories.

The repository's own scripts are copied into each throwaway repository; ledger.py is replaced by a stub that
records that it ran and exits with the status the test asks for."""

import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(SCRIPTS / "hooks"))
import install  # noqa: E402

# Strings that must be refused, assembled at run time so this file itself passes the publication check.
MACHINE_PATH = "/ho" + "me/alice/game"
LEDGER_STUB = """import os, sys
open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "ran"), "w").close()
print("ledger stub")
sys.exit(int(os.environ.get("STUB_LEDGER_EXIT", "0")))
"""
HAVE_GIT = subprocess.run(["git", "--version"], capture_output=True).returncode == 0


def git(root, *args, check=True, env=None):
    return subprocess.run(["git", *args], cwd=root, capture_output=True, text=True, check=check, env=env)


class Repository:
    """A throwaway repository holding copies of the hook, its installer and the publication check."""

    def __init__(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name).resolve()
        git(self.root, "init", "-q")
        git(self.root, "config", "user.email", "test@example.invalid")
        git(self.root, "config", "user.name", "test")
        git(self.root, "config", "commit.gpgsign", "false")
        for relative in ("hooks/pre-commit", "hooks/install.py", "ci/check_publication.py"):
            (self.root / "scripts" / relative).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(SCRIPTS / relative, self.root / "scripts" / relative)
        (self.root / "scripts" / "coordinator").mkdir()
        (self.root / "scripts" / "coordinator" / "ledger.py").write_text(LEDGER_STUB, encoding="utf-8")
        self.write("README.md", "clean\n")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "--no-verify", "-m", "first")

    def write(self, name, text):
        (self.root / name).parent.mkdir(parents=True, exist_ok=True)
        (self.root / name).write_text(text, encoding="utf-8", newline="\n")

    def commit(self, message="change", **env):
        """git commit with the hook in place; returns the completed process."""
        environment = dict(os.environ, LIBERTYFLUX_PYTHON=sys.executable, **env)
        return git(self.root, "commit", "-q", "-m", message, check=False, env=environment)

    def ledger_ran(self):
        marker = self.root / "scripts" / "coordinator" / "ran"
        ran = marker.exists()
        if ran:
            marker.unlink()
        return ran

    def close(self):
        self.tmp.cleanup()


@unittest.skipUnless(HAVE_GIT, "git not available")
class TestInstaller(unittest.TestCase):
    def setUp(self):
        self.repo = Repository()
        self.root = self.repo.root
        self.hook = self.root / ".git" / "hooks" / "pre-commit"

    def tearDown(self):
        self.repo.close()

    def test_copy_then_already_installed(self):
        self.assertEqual(install.hook_state(self.root), "missing")
        code, message = install.install(self.root, "pre-commit")
        self.assertEqual((code, message), (0, "pre-commit: copied scripts/hooks/pre-commit to .git/hooks/pre-commit"))
        self.assertEqual(install.hook_state(self.root), "installed")
        if os.name != "nt":
            self.assertTrue(os.access(self.hook, os.X_OK))
        self.assertEqual(install.install(self.root, "pre-commit"), (0, "pre-commit: already installed as .git/hooks/pre-commit"))

    def test_a_different_hook_needs_force_and_is_kept(self):
        self.hook.write_text("#!/bin/sh\necho mine\n", encoding="utf-8")
        code, message = install.install(self.root, "pre-commit")
        self.assertEqual(code, 1)
        self.assertIn("is a different hook; left as it is", message)
        self.assertEqual(self.hook.read_text(encoding="utf-8"), "#!/bin/sh\necho mine\n")
        code, message = install.install(self.root, "pre-commit", force=True)
        self.assertEqual(code, 0)
        self.assertTrue(message.endswith("the previous hook is kept as .git/hooks/pre-commit.bak"))
        self.assertEqual((self.hook.parent / "pre-commit.bak").read_text(encoding="utf-8"), "#!/bin/sh\necho mine\n")
        self.assertEqual(install.hook_state(self.root), "installed")

    def test_line_ends_are_normalised(self):
        source = self.root / "scripts" / "hooks" / "pre-commit"
        source.write_bytes(source.read_bytes().replace(b"\n", b"\r\n"))
        self.assertEqual(install.install(self.root, "pre-commit")[0], 0)
        self.assertNotIn(b"\r\n", self.hook.read_bytes())
        self.assertEqual(install.hook_state(self.root), "installed")  # line ends aside, it is the same hook
        code, message = install.install(self.root, "pre-commit", link=True)
        self.assertEqual(code, 1)
        self.assertIn("CRLF", message)

    def test_hooks_path_setting_is_followed(self):
        git(self.root, "config", "core.hooksPath", "githooks")
        code, message = install.install(self.root, "pre-commit")
        self.assertEqual((code, message), (0, "pre-commit: copied scripts/hooks/pre-commit to githooks/pre-commit"))
        self.assertTrue((self.root / "githooks" / "pre-commit").is_file())
        self.assertFalse(self.hook.exists())

    def test_link(self):
        code, message = install.install(self.root, "pre-commit", link=True)
        if code and "symbolic link" in message:
            self.skipTest("symbolic links are not available here")
        self.assertEqual((code, message), (0, "pre-commit: linked .git/hooks/pre-commit to scripts/hooks/pre-commit"))
        self.assertTrue(self.hook.is_symlink())
        self.assertEqual(install.hook_state(self.root), "installed")

    def test_main_check_and_install(self):
        script = [sys.executable, str(self.root / "scripts" / "hooks" / "install.py")]
        checked = subprocess.run(script + ["--check"], cwd=self.root, capture_output=True, text=True)
        self.assertEqual((checked.returncode, checked.stdout), (1, "pre-commit: missing\n"))
        done = subprocess.run(script, cwd=self.root / "scripts", capture_output=True, text=True)
        self.assertEqual((done.returncode, done.stdout), (0, "pre-commit: copied scripts/hooks/pre-commit to .git/hooks/pre-commit\n"))
        checked = subprocess.run(script + ["--check"], cwd=self.root, capture_output=True, text=True)
        self.assertEqual((checked.returncode, checked.stdout), (0, "pre-commit: installed\n"))


@unittest.skipUnless(HAVE_GIT, "git not available")
class TestPreCommitHook(unittest.TestCase):
    def setUp(self):
        self.repo = Repository()
        self.assertEqual(install.install(self.repo.root, "pre-commit")[0], 0)

    def tearDown(self):
        self.repo.close()

    def test_clean_commit_passes_quietly(self):
        self.repo.write("docs/page.html", "<p>fine</p>\n")
        git(self.repo.root, "add", "docs/page.html")
        done = self.repo.commit()
        self.assertEqual((done.returncode, done.stdout, done.stderr), (0, "", ""))
        self.assertFalse(self.repo.ledger_ran())  # nothing under rewrites/ was staged

    def test_forbidden_paths_are_refused(self):
        for name in ("notes.md", "orig/game.txt", ".artifacts/logs/lane.txt"):
            self.repo.write(name, "x\n")
            git(self.repo.root, "add", "-f", name)
            done = self.repo.commit()
            self.assertEqual(done.returncode, 1, name)
            self.assertIn("must never be committed", done.stderr)
            self.assertIn("  " + name, done.stderr)
            git(self.repo.root, "rm", "-q", "--cached", name)

    def test_publication_findings_block_the_commit(self):
        self.repo.write("docs/page.html", MACHINE_PATH + "\n")
        git(self.repo.root, "add", "docs/page.html")
        done = self.repo.commit()
        self.assertEqual(done.returncode, 1)
        self.assertIn("docs/page.html:1: machine path or account name", done.stderr)
        self.assertIn("the publication check failed", done.stderr)
        self.assertEqual(git(self.repo.root, "rev-list", "--count", "HEAD").stdout.strip(), "1")

    def test_ledger_runs_when_rewrites_are_staged(self):
        self.repo.write("rewrites/verified/index.json", "[]\n")
        git(self.repo.root, "add", "rewrites")
        done = self.repo.commit(STUB_LEDGER_EXIT="1")
        self.assertEqual(done.returncode, 1)
        self.assertTrue(self.repo.ledger_ran())
        self.assertIn("ledger stub", done.stderr)
        self.assertIn("the rewrite ledger has errors", done.stderr)
        done = self.repo.commit(STUB_LEDGER_EXIT="0")
        self.assertEqual((done.returncode, done.stderr), (0, ""))
        self.assertTrue(self.repo.ledger_ran())

    def test_no_python_fails_closed(self):
        tools = {name: shutil.which(name) for name in ("sh", "git", "grep", "sed")}
        if os.name == "nt" or not all(tools.values()):
            self.skipTest("needs sh, git, grep and sed, and symbolic links to put them on a PATH without Python")
        self.repo.write("docs/page.html", "<p>fine</p>\n")
        git(self.repo.root, "add", "docs/page.html")
        with tempfile.TemporaryDirectory() as bin_dir:
            for name in ("git", "grep", "sed"):
                os.symlink(tools[name], os.path.join(bin_dir, name))
            env = dict(os.environ, LIBERTYFLUX_PYTHON="", PATH=bin_dir)
            done = subprocess.run([tools["sh"], str(self.repo.root / ".git" / "hooks" / "pre-commit")], cwd=self.repo.root,
                                  capture_output=True, text=True, env=env)
        self.assertEqual(done.returncode, 1)
        self.assertIn("no Python 3.8 or later found", done.stderr)


if __name__ == "__main__":
    unittest.main()
