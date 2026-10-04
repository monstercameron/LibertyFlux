"""Tests for check_rewrites_build.py: error kinds, aliases, --only selection, the scratch crate, and (where cargo
and the 32-bit Windows target are installed) one real run over three synthetic rewrites."""

import contextlib
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import check_rewrites_build as build

INDEX = [{"address": "0x00401000", "file": "functions/fn_00401000.rs"},
         {"address": "0x00401010", "file": "natives/fn_00401010.rs"}]

COMPILES = "// original: 0x00401000 f\n//! Inner doc a lane's crate root could hold.\nexport!(cdecl, rw_00401000(a: u32) -> u32 { a });\n"
LANE_RUNTIME = ("// original: 0x00401010 f\nuse lf_lane_rt::{export};\n"
                "export!(cdecl, rw_00401010(a: u32) -> u32 { a.wrapping_add(1) });\n")
UNDEFINED = "// original: 0x00401020 f\nexport!(cdecl, rw_00401020(a: u32) -> u32 { helper_that_is_nowhere(a) });\n"


class TestErrorClasses(unittest.TestCase):
    def test_error_class(self):
        self.assertEqual(build.error_class("error[E0425]: cannot find function `f` in this scope: not found in this scope"),
                         "error[E0425]: cannot find function X in this scope")
        self.assertEqual(build.error_class("error: expected identifier, found reserved keyword `gen`: expected identifier"),
                         "error: expected identifier, found reserved keyword X")
        self.assertEqual(build.error_class("error: this file contains an unclosed delimiter"),
                         "error: this file contains an unclosed delimiter")

    def test_counts_most_frequent_first(self):
        rows = [{"error": "error[E0432]: unresolved import `a`"}, {"error": "error[E0425]: cannot find value `x` in this scope"},
                {"error": "error[E0432]: unresolved import `b`: use of unresolved module"}]
        self.assertEqual(list(build.class_counts(rows).items()),
                         [("error[E0432]: unresolved import X", 2), ("error[E0425]: cannot find value X in this scope", 1)])


class TestAliases(unittest.TestCase):
    def test_parse(self):
        self.assertEqual(build.parse_aliases(["lf-k2-rt=lf-checker-rt", "rt = core"]), {"lf_k2_rt": "lf_checker_rt", "rt": "core"})
        self.assertEqual(build.parse_aliases([]), {})
        for bad in ("lf_k2_rt", "=lf_checker_rt", "lf_k2_rt=", "lf_checker_rt=core", "lf_k2_rt=serde", "a b=core"):
            with self.assertRaises(ValueError, msg=bad):
                build.parse_aliases([bad])


class TestSelect(unittest.TestCase):
    def test_index_entries_in_any_spelling(self):
        chosen = build.select(INDEX, ["rewrites/verified/natives/fn_00401010.rs", ".\\functions\\fn_00401000.rs",
                                      "natives/fn_00401010.rs"])
        self.assertEqual([e["file"] for e in chosen], ["natives/fn_00401010.rs", "functions/fn_00401000.rs"])
        self.assertNotIn("path", chosen[0])

    def test_files_outside_the_index(self):
        with tempfile.TemporaryDirectory() as tmp:
            named = Path(tmp) / "fn_00401020.rs"
            named.write_text(UNDEFINED, encoding="utf-8")
            headed = Path(tmp) / "attempt.rs"
            headed.write_text(UNDEFINED.replace("0x00401020", "0x00401030"), encoding="utf-8")
            nameless = Path(tmp) / "draft.rs"
            nameless.write_text("export!(cdecl, f() -> u32 { 0 });\n", encoding="utf-8")
            chosen = build.select(INDEX, [str(named), str(headed)])
            self.assertEqual([e["address"] for e in chosen], ["0x00401020", "0x00401030"])
            self.assertEqual(chosen[0]["path"], str(named.resolve()))
            self.assertEqual(len(build.select(INDEX, [str(named), str(named)])), 1)
            lane = Path(tmp) / "lane"
            (lane / "sub").mkdir(parents=True)
            shutil.copy2(named, lane / "fn_00401020.rs")
            shutil.copy2(headed, lane / "sub" / "attempt.rs")
            self.assertEqual([e["address"] for e in build.select(INDEX, [str(lane)])], ["0x00401020", "0x00401030"])
            clash = Path(tmp) / "fn_00401000.rs"
            clash.write_text(COMPILES, encoding="utf-8")
            for names in ([str(nameless)], [str(Path(tmp) / "missing.rs")], ["functions/fn_00401000.rs", str(clash)]):
                with self.assertRaises(ValueError, msg=names):
                    build.select(INDEX, names)


class TestScratchCrate(unittest.TestCase):
    def test_aliases_and_outside_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            root, work = Path(tmp) / "repo", Path(tmp) / "work"
            (root / "rewrites" / "verified" / "functions").mkdir(parents=True)
            (root / "rewrites" / "verified" / "functions" / "fn_00401000.rs").write_text(COMPILES, encoding="utf-8")
            outside = Path(tmp) / "fn_00401010.rs"
            outside.write_text(LANE_RUNTIME, encoding="utf-8")
            entries = [INDEX[0], {"address": "0x00401010", "file": "fn_00401010.rs", "path": str(outside)}]
            with mock.patch.object(build, "ROOT", root), mock.patch.object(build, "WORK", work):
                build.write_crate(entries, {}, "2024", {"lf_lane_rt": "lf_checker_rt"})
                lib = (work / "src" / "lib.rs").read_text(encoding="utf-8").splitlines()
                self.assertEqual(lib[2], "extern crate lf_checker_rt as lf_lane_rt;")
                self.assertEqual(len([line for line in lib if line.startswith("pub mod m_")]), 2)
                self.assertIn("use lf_lane_rt::{export};", (work / "src" / "rw" / "00401010.rs").read_text(encoding="utf-8"))
                copied = (work / "src" / "rw" / "00401000.rs").read_text(encoding="utf-8")
                self.assertNotIn("//!", copied)  # inner doc comments are neutralised in the copy only
                build.write_crate(entries, {"00401010": "error"}, "2024")
                lib = (work / "src" / "lib.rs").read_text(encoding="utf-8")
                self.assertNotIn("extern crate lf_checker_rt as", lib)
                self.assertNotIn("m_00401010", lib)


def toolchain_env():
    """The environment for a cargo run outside the repository: the repository's rust-toolchain.toml does not apply
    there, so its channel is named in RUSTUP_TOOLCHAIN (as the Rewrites workflow does), unless one is set already."""
    match = re.search(r'^channel\s*=\s*"([^"]+)"', (build.ROOT / "rust-toolchain.toml").read_text(encoding="utf-8"), re.M)
    pinned = os.environ.get("RUSTUP_TOOLCHAIN") or (match.group(1) if match else "")
    return {"RUSTUP_TOOLCHAIN": pinned} if pinned else {}


def i686_installed():
    """True when cargo is on PATH and the pinned toolchain has the 32-bit Windows target's standard library."""
    if not shutil.which("cargo") or not shutil.which("rustc"):
        return False
    found = subprocess.run(["rustc", "--print", "target-libdir", "--target", build.TARGET], cwd=tempfile.gettempdir(),
                           env=dict(os.environ, **toolchain_env()), capture_output=True, text=True)
    return found.returncode == 0 and Path(found.stdout.strip()).is_dir()


@unittest.skipUnless(i686_installed(), "needs cargo and the i686-pc-windows-msvc target")
class TestRealRun(unittest.TestCase):
    """One real type-check in a temporary work folder: the shared runtime is built once for it (a few seconds)."""

    def test_strict_result_alias_experiment_and_error_classes(self):
        with tempfile.TemporaryDirectory() as tmp:
            files = {"fn_00401000.rs": COMPILES, "fn_00401010.rs": LANE_RUNTIME, "fn_00401020.rs": UNDEFINED}
            for name, text in files.items():
                (Path(tmp) / name).write_text(text, encoding="utf-8")
            out = Path(tmp) / "result.json"
            argv = ["--out", str(out), "--alias", "lf_lane_rt=lf_checker_rt", "--only", *(str(Path(tmp) / n) for n in files)]
            printed = io.StringIO()
            with mock.patch.object(build, "WORK", Path(tmp) / "work"), mock.patch.dict(os.environ, toolchain_env()), \
                    contextlib.redirect_stdout(printed):
                code = build.main(argv)
            self.assertEqual(code, 1)  # strict: two files fail, whatever the alias does
            result = json.loads(out.read_text(encoding="utf-8"))
            self.assertEqual(result["checked"], 3)
            self.assertEqual([Path(r["file"]).name for r in result["failed"]], ["fn_00401010.rs", "fn_00401020.rs"])
            self.assertEqual(set(result["error_classes"]), {"error[E0432]: unresolved import X",
                                                            "error[E0425]: cannot find function X in this scope"})
            alias = result["alias"]
            self.assertEqual((alias["aliases"], alias["retried"]), ({"lf_lane_rt": "lf_checker_rt"}, 2))
            self.assertEqual([Path(f).name for f in alias["now_compile"]], ["fn_00401010.rs"])
            self.assertEqual([Path(r["file"]).name for r in alias["failed"]], ["fn_00401020.rs"])
            lines = printed.getvalue().splitlines()
            self.assertEqual(lines[0], "1 of 3 rewrites compile against the shared runtime alone (2 do not)")
            self.assertTrue(lines[-1].startswith("with the alias(es) lf_lane_rt=lf_checker_rt: 1 more compile, 1 of the 2"))


if __name__ == "__main__":
    unittest.main()
