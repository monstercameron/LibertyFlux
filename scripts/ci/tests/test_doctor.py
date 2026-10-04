"""Tests for scripts/doctor.py: each check with fake command results, the notes and hooks checks in throwaway
repositories, and that no machine path or variable value reaches the output."""

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

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
import doctor  # noqa: E402

TOOLCHAIN = '[toolchain]\nchannel = "1.96.1"\ncomponents = ["clippy"]\ntargets = [\n  "x86_64-pc-windows-msvc",\n  "i686-pc-windows-msvc",\n]\n'
# A machine path, assembled at run time so this file itself passes the publication check.
MACHINE_PATH = "/ho" + "me/alice/work/LibertyFlux"
HAVE_GIT = subprocess.run(["git", "--version"], capture_output=True).returncode == 0


class FakeRun:
    """A command runner answering from a table keyed by the command's first words; None means not found."""

    def __init__(self, answers):
        self.answers = answers

    def __call__(self, command, cwd):
        for words in range(len(command), 0, -1):
            key = tuple(command[:words])
            if key in self.answers:
                answer = self.answers[key]
                return answer(command) if callable(answer) else answer
        return None


def rust_answers(root, rustc="rustc 1.96.1 (31fca3adb 2026-06-26)", libdirs=None, metadata=None):
    libdirs = libdirs or {}
    return {("git", "--version"): (0, "git version 2.43.0\n", ""),
            ("git", "rev-parse", "--is-inside-work-tree"): (0, "true\n", ""),
            ("cargo", "--version"): (0, "cargo 1.96.1 (356927216 2026-06-26)\n", ""),
            ("rustc", "--version"): (0, rustc + "\n", ""),
            ("rustc", "--print", "target-libdir"): lambda c: (0, libdirs.get(c[-1], "/nowhere") + "\n", ""),
            ("cargo", "metadata"): metadata or (0, json.dumps({"workspace_members": ["a", "b"],
                                                               "target_directory": str(Path(root) / ".artifacts" / "build" / "cargo")}), "")}


class TestScrub(unittest.TestCase):
    def test_paths_and_names_are_taken_out(self):
        root, home = Path("/srv/checkouts/LibertyFlux"), Path("/srv/people/someone")
        text = f"failed to read {root}/Cargo.toml and {home}/.cargo/config and {MACHINE_PATH}/x and C:" + "\\Users\\bob\\y"
        cleaned = doctor.scrub(text, root, home)
        self.assertEqual(cleaned, "failed to read <repo>/Cargo.toml and ~/.cargo/config and <path> and <path>")
        self.assertEqual(doctor.scrub("a/b /c", None, Path("/")), "a/b /c")  # a bare root folder is never replaced


class TestChecks(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "rust-toolchain.toml").write_text(TOOLCHAIN, encoding="utf-8")
        self.libdirs = {}
        for target in ("x86_64-pc-windows-msvc", "i686-pc-windows-msvc"):
            (self.root / "sysroot" / target).mkdir(parents=True)
            self.libdirs[target] = str(self.root / "sysroot" / target)

    def tearDown(self):
        self.tmp.cleanup()

    def test_python(self):
        self.assertEqual(doctor.check_python((3, 11, 4)), ("PASS", "Python 3.11.4"))
        self.assertEqual(doctor.check_python((3, 9, 1))[0], "WARN")
        self.assertEqual(doctor.check_python((3, 7, 0))[0], "FAIL")

    def test_rust_toolchain_targets_workspace_all_pass(self):
        run = FakeRun(rust_answers(self.root, libdirs=self.libdirs))
        status, _, version = doctor.check_rust(self.root, run)
        self.assertEqual((status, version), ("PASS", "1.96.1"))
        self.assertEqual(doctor.check_toolchain(self.root, version), ("PASS", "rustc 1.96.1, as rust-toolchain.toml pins"))
        self.assertEqual(doctor.check_targets(self.root, run, version),
                         ("PASS", "installed: x86_64-pc-windows-msvc, i686-pc-windows-msvc"))
        self.assertEqual(doctor.check_workspace(self.root, run, version),
                         ("PASS", "2 workspace members; build output goes to .artifacts/build/cargo"))

    def test_mismatch_missing_target_and_outside_output(self):
        libdirs = {"x86_64-pc-windows-msvc": self.libdirs["x86_64-pc-windows-msvc"]}
        outside = (0, json.dumps({"workspace_members": [], "target_directory": MACHINE_PATH + "/target"}), "")
        run = FakeRun(rust_answers(self.root, rustc="rustc 1.97.0 (abc 2026-08-01)", libdirs=libdirs, metadata=outside))
        _, _, version = doctor.check_rust(self.root, run)
        status, detail = doctor.check_toolchain(self.root, version)
        self.assertEqual(status, "WARN")
        self.assertIn("rustc is 1.97.0 but rust-toolchain.toml pins 1.96.1", detail)
        status, detail = doctor.check_targets(self.root, run, version)
        self.assertEqual(status, "WARN")
        self.assertTrue(detail.startswith("missing: i686-pc-windows-msvc (i686-pc-windows-msvc builds the 32-bit tools"))
        self.assertTrue(detail.endswith("rustup target add i686-pc-windows-msvc"))
        status, detail = doctor.check_workspace(self.root, run, version)
        self.assertEqual(status, "WARN")
        self.assertNotIn("alice", detail)

    def test_no_rust_fails_once_and_skips_the_rest(self):
        run = FakeRun({("git", "--version"): (0, "git version 2.43.0\n", "")})
        status, detail, version = doctor.check_rust(self.root, run)
        self.assertEqual((status, version), ("FAIL", None))
        self.assertIn("cargo is not on PATH", detail)
        self.assertEqual(doctor.check_toolchain(self.root, None)[0], "WARN")
        self.assertEqual(doctor.check_targets(self.root, run, None), ("WARN", "not checked: rustc did not run"))
        self.assertEqual(doctor.check_workspace(self.root, run, None), ("WARN", "not checked: cargo did not run"))

    def test_failing_tools_are_scrubbed(self):
        answers = rust_answers(self.root)
        answers[("rustc", "--version")] = (1, "", f"error: toolchain '1.96.1' is not installed\nsee {self.root}/rust-toolchain.toml")
        status, detail, _ = doctor.check_rust(self.root, FakeRun(answers))
        self.assertEqual((status, detail), ("FAIL", "rustc --version failed: error: toolchain '1.96.1' is not installed"))
        answers = rust_answers(self.root, metadata=(101, "", f"error: failed to parse manifest at `{MACHINE_PATH}/Cargo.toml`"))
        status, detail = doctor.check_workspace(self.root, FakeRun(answers), "1.96.1")
        self.assertEqual((status, detail), ("FAIL", "cargo metadata failed: error: failed to parse manifest at `<path>`"))

    def test_variables_never_show_their_values(self):
        folder = self.root / "game"
        folder.mkdir()
        exe = self.root / "game.exe"
        exe.write_bytes(b"")
        env = {"LIBERTYFLUX_GAME_DIR": str(folder), "LIBERTYFLUX_ORIG_EXE": str(self.root / "missing.exe")}
        self.assertEqual(doctor.check_variable(env, "LIBERTYFLUX_GAME_DIR", "dir", "p"),
                         ("PASS", "LIBERTYFLUX_GAME_DIR is set; the folder it names exists"))
        self.assertEqual(doctor.check_variable(env, "LIBERTYFLUX_ORIG_EXE", "file", "p"),
                         ("WARN", "LIBERTYFLUX_ORIG_EXE is set, but there is no file where it points"))
        self.assertEqual(doctor.check_variable({}, "LIBERTYFLUX_ORIG_EXE", "file", "p", default="orig/GTAIV.exe", root=self.root),
                         ("WARN", "LIBERTYFLUX_ORIG_EXE is not set and orig/GTAIV.exe is absent (optional: p)"))
        (self.root / "orig").mkdir()
        (self.root / "orig" / "GTAIV.exe").write_bytes(b"")
        self.assertEqual(doctor.check_variable({}, "LIBERTYFLUX_ORIG_EXE", "file", "p", default="orig/GTAIV.exe", root=self.root),
                         ("PASS", "LIBERTYFLUX_ORIG_EXE is not set; the default orig/GTAIV.exe is present"))
        java = self.root / "jdk" / "bin"
        java.mkdir(parents=True)
        self.assertEqual(doctor.check_java({"JAVA_HOME": str(self.root / "jdk")})[0], "WARN")
        (java / ("java.exe" if os.name == "nt" else "java")).write_bytes(b"")
        self.assertEqual(doctor.check_java({"JAVA_HOME": str(self.root / "jdk")}), ("PASS", "JAVA_HOME is set; bin/java is there"))
        self.assertEqual(doctor.check_java({})[0], "WARN")

    def test_collect_output_holds_no_machine_path(self):
        env = {"LIBERTYFLUX_GAME_DIR": str(self.root), "JAVA_HOME": MACHINE_PATH, "LIBERTYFLUX_ORIG_EXE": str(self.root / "x")}
        results = doctor.collect(self.root, env, FakeRun(rust_answers(self.root, libdirs=self.libdirs)), (3, 11, 0))
        self.assertEqual([name for name, _, _ in results],
                         ["python", "git", "rust", "toolchain", "targets", "workspace", "game-dir", "orig-exe",
                          "java-home", "ghidra", "hooks", "notes"])
        text = json.dumps(results)
        for secret in (str(self.root), self.root.as_posix(), MACHINE_PATH, "alice"):
            self.assertNotIn(secret, text)


@unittest.skipUnless(HAVE_GIT, "git not available")
class TestRepositoryChecks(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name).resolve()
        for args in (["init", "-q"], ["config", "user.email", "test@example.invalid"], ["config", "user.name", "test"]):
            subprocess.run(["git", *args], cwd=self.root, check=True, capture_output=True)
        hooks = self.root / "scripts" / "hooks"
        hooks.mkdir(parents=True)
        (hooks / "pre-commit").write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")

    def tearDown(self):
        self.tmp.cleanup()

    def test_notes(self):
        run = doctor.run_command
        self.assertEqual(doctor.check_notes(self.root, run)[0], "WARN")  # absent
        (self.root / "notes.md").write_text("private\n", encoding="utf-8")
        status, detail = doctor.check_notes(self.root, run)
        self.assertEqual(status, "WARN")
        self.assertIn("git does not ignore it", detail)
        (self.root / ".gitignore").write_text("/notes.md\n", encoding="utf-8")
        self.assertEqual(doctor.check_notes(self.root, run), ("PASS", "notes.md is present, untracked and ignored by git"))
        subprocess.run(["git", "add", "-f", "notes.md"], cwd=self.root, check=True, capture_output=True)
        self.assertEqual(doctor.check_notes(self.root, run)[0], "FAIL")

    def test_hooks(self):
        status, detail = doctor.check_hooks(self.root)
        self.assertEqual((status, detail.split(" (")[0]), ("WARN", "pre-commit missing"))
        self.assertEqual(doctor.install.install(self.root, "pre-commit")[0], 0)
        self.assertEqual(doctor.check_hooks(self.root), ("PASS", "installed: pre-commit"))


class TestMain(unittest.TestCase):
    RESULTS = [("python", "PASS", "Python 3.11.0"), ("hooks", "WARN", "pre-commit missing"), ("notes", "FAIL", "tracked")]

    def run_main(self, *argv, results=RESULTS):
        out = io.StringIO()
        with mock.patch.object(doctor, "collect", return_value=list(results)), \
                mock.patch.object(doctor, "find_root", return_value=Path(".")), contextlib.redirect_stdout(out):
            code = doctor.main(list(argv))
        return code, out.getvalue()

    def test_text_and_exit_status(self):
        code, out = self.run_main()
        self.assertEqual(code, 1)
        self.assertEqual(out.splitlines(), ["PASS  python     Python 3.11.0", "WARN  hooks      pre-commit missing",
                                            "FAIL  notes      tracked", "summary: 1 passed, 1 warnings, 1 failures"])
        self.assertEqual(self.run_main(results=self.RESULTS[:2])[0], 0)  # warnings alone do not fail

    def test_json(self):
        code, out = self.run_main("--json")
        data = json.loads(out)
        self.assertEqual(code, 1)
        self.assertEqual(data["summary"], {"passed": 1, "warnings": 1, "failures": 1})
        self.assertEqual(data["checks"][1], {"name": "hooks", "status": "WARN", "detail": "pre-commit missing"})


if __name__ == "__main__":
    unittest.main()
