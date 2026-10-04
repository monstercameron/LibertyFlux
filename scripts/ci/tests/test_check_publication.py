"""Tests for check_publication.py in throwaway git repositories: the full check, --files and --staged."""

import contextlib
import io
import os
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import check_publication as publication

# Strings that must be refused, assembled at run time so this file itself passes the publication check.
MACHINE_PATH = "/ho" + "me/alice/game"
WINDOWS_PATH = "C:" + "\\Users\\alice\\"
GAME_FOLDER = "steam" + "apps/common"
INLINE_ASM = "core::arch::" + "asm!(\"nop\");"


def git(root, *args, **kwargs):
    return subprocess.run(["git", *args], cwd=root, capture_output=True, text=True, check=True, **kwargs).stdout


@contextlib.contextmanager
def inside(folder):
    """Run with `folder` as the current folder, as a hook or a person would."""
    old = os.getcwd()
    os.chdir(folder)
    try:
        yield
    finally:
        os.chdir(old)


class TestCheckFile(unittest.TestCase):
    def check(self, name, text="", size=None):
        return publication.check_file(name, len(text) if size is None else size, lambda: text)

    def test_names(self):
        self.assertEqual(self.check("notes.md"), ["notes.md: must not be tracked"])
        self.assertEqual(self.check("orig/GTAIV.exe"), ["orig/GTAIV.exe: must not be tracked"])
        self.assertEqual(self.check(".artifacts/logs/a.txt"), [".artifacts/logs/a.txt: must not be tracked"])
        self.assertEqual(self.check("tools/x.DLL"), ["tools/x.DLL: binary or game file type must not be tracked"])
        self.assertEqual(publication.check_file("gone.txt", None, lambda: self.fail("read a missing file")), [])

    def test_text_rules(self):
        self.assertEqual(self.check("a.md", "fine\nsee " + MACHINE_PATH + "\n" + WINDOWS_PATH + "\n"),
                         ["a.md:2: machine path or account name"])  # one finding per file, at the first line
        self.assertEqual(self.check("b.json", '"' + GAME_FOLDER + '"'), ["b.json:1: machine path or account name"])
        self.assertEqual(self.check("crates/lf-core/src/a.rs", INLINE_ASM), ["crates/lf-core/src/a.rs: inline assembly in a rewrite or engine crate"])
        self.assertEqual(self.check("crates/tools/lf-hook/src/a.rs", INLINE_ASM), [])
        self.assertEqual(self.check("scripts/ci/check_publication.py", MACHINE_PATH), [])
        self.assertEqual(self.check("picture.png", MACHINE_PATH), [])  # not a text type

    def test_sizes(self):
        big = 6 * 1024 * 1024
        self.assertEqual(self.check("data.bin", size=big), ["data.bin: larger than 5 MB"])
        self.assertEqual(self.check(publication.DEVLOG, size=big), [])
        self.assertEqual(self.check(publication.DEVLOG, size=41 * 1024 * 1024), [f"{publication.DEVLOG}: larger than 40 MB"])


def make_devlog(path, text):
    connection = sqlite3.connect(path)
    connection.execute("CREATE TABLE posts (id TEXT, title TEXT, text TEXT)")
    connection.execute("INSERT INTO posts VALUES ('entry-a', 'A title', ?)", (text,))
    connection.commit()
    connection.close()


@unittest.skipUnless(subprocess.run(["git", "--version"], capture_output=True).returncode == 0, "git not available")
class TestModes(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name).resolve()
        git(self.root, "init", "-q")
        git(self.root, "config", "user.email", "test@example.invalid")
        git(self.root, "config", "user.name", "test")
        git(self.root, "config", "commit.gpgsign", "false")
        self.write("README.md", "clean\n")
        self.write("rewrites/verified/functions/fn_00401000.rs", "// original: 0x00401000 f\n")
        (self.root / "docs" / "data").mkdir(parents=True)
        make_devlog(self.root / publication.DEVLOG, "A clean entry.")
        git(self.root, "add", ".")
        git(self.root, "commit", "-q", "-m", "first")
        self.patch = mock.patch.object(publication, "repo_root", return_value=self.root)
        self.patch.start()

    def tearDown(self):
        self.patch.stop()
        self.tmp.cleanup()

    def write(self, name, text):
        (self.root / name).parent.mkdir(parents=True, exist_ok=True)
        (self.root / name).write_text(text, encoding="utf-8", newline="\n")

    def run_main(self, *argv):
        out = io.StringIO()
        with inside(self.root), contextlib.redirect_stdout(out):
            code = publication.main(list(argv))
        return code, out.getvalue().splitlines()

    def test_full_check_is_the_default(self):
        self.assertEqual(self.run_main(), (0, ["checked 3 tracked files", "publication check passed"]))
        self.write("notes.md", "private\n")
        self.write("docs/page.html", "<p>" + MACHINE_PATH + "</p>\n")
        git(self.root, "add", "-f", "notes.md", "docs/page.html")
        code, lines = self.run_main()
        self.assertEqual(code, 1)
        self.assertEqual(lines, ["checked 5 tracked files", "2 finding(s):", "  docs/page.html:1: machine path or account name",
                                 "  notes.md: must not be tracked"])

    def test_devlog_entries_are_checked(self):
        (self.root / publication.DEVLOG).unlink()
        make_devlog(self.root / publication.DEVLOG, "It lives at " + "0x0040" + "1000 in the image.")
        code, lines = self.run_main()
        self.assertEqual((code, lines[-1]), (1, f"  {publication.DEVLOG}: entry entry-a: game function address"))
        code, lines = self.run_main("--files", "README.md")
        self.assertEqual((code, lines), (0, ["checked 1 given files", "publication check passed"]))
        code, lines = self.run_main("--files", publication.DEVLOG)
        self.assertEqual(code, 1)

    def test_files_checks_only_those(self):
        self.write("docs/page.html", MACHINE_PATH + "\n")  # untracked, but named
        self.write("docs/other.html", MACHINE_PATH + "\n")  # neither tracked nor named
        code, lines = self.run_main("--files", "README.md", "docs/page.html", "orig/GTAIV.exe")
        self.assertEqual(code, 1)
        self.assertEqual(lines, ["checked 3 given files", "2 finding(s):", "  docs/page.html:1: machine path or account name",
                                 "  orig/GTAIV.exe: must not be tracked"])
        with inside(self.root / "docs"), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(publication.main(["--files", "../README.md"]), 0)  # relative to the current folder
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as stop:
            self.run_main("--files", str(Path(self.tmp.name).parent))
        self.assertEqual(stop.exception.code, 2)

    def test_staged_reads_the_index(self):
        self.assertEqual(self.run_main("--staged"), (0, ["checked 0 staged files", "publication check passed"]))
        self.write("docs/page.html", MACHINE_PATH + "\n")
        git(self.root, "add", "docs/page.html")
        self.write("docs/page.html", "cleaned in the working tree, not staged\n")
        code, lines = self.run_main("--staged")
        self.assertEqual((code, lines[0], lines[-1]), (1, "checked 1 staged files", "  docs/page.html:1: machine path or account name"))
        self.assertEqual(self.run_main("--files", "docs/page.html")[0], 0)  # the disk copy is clean
        self.assertEqual(self.run_main("--staged", "--files", "README.md")[0], 0)
        self.assertEqual(self.run_main("--staged", "--files", "docs/page.html")[0], 1)
        git(self.root, "rm", "-q", "-f", "--cached", "docs/page.html")
        git(self.root, "rm", "-q", "README.md")  # a staged deletion is not checked
        self.write("notes.md", "private\n")
        git(self.root, "add", "-f", "notes.md")
        self.assertEqual(self.run_main("--staged"), (1, ["checked 1 staged files", "1 finding(s):", "  notes.md: must not be tracked"]))

    def test_staged_devlog_and_binary_content(self):
        (self.root / publication.DEVLOG).unlink()
        make_devlog(self.root / publication.DEVLOG, "push " + "dword ptr [eax+4] then return")
        git(self.root, "add", publication.DEVLOG)
        code, lines = self.run_main("--staged")
        self.assertEqual((code, lines[-1]), (1, f"  {publication.DEVLOG}: entry entry-a: disassembly"))
        blobs = publication.staged_blobs(self.root, [publication.DEVLOG, "not/in/the/index.txt"])
        self.assertEqual(list(blobs), [publication.DEVLOG])
        self.assertEqual(blobs[publication.DEVLOG], (self.root / publication.DEVLOG).read_bytes())


if __name__ == "__main__":
    unittest.main()
