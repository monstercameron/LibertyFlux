"""Tests for scan.py: the publication scan that gates rewrites entering the tree."""

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import scan

CLEAN_RUST = """\
// original: 0x10000000 pool_tick
//! Advances every live voice by one step and drops finished ones.
use lf_checker_rt::{Mem, State};

const MAX_VOICES: u32 = 64;

fn pool_tick(st: &mut State, mem: &Mem, pool: u32) -> u32 {
    let mut live = 0u32;
    let mut slot = pool;
    while live < MAX_VOICES {
        let flags = mem.read_u32(slot);
        if flags & 1 != 0 {
            mem.write_u32(slot + 4, mem.read_u32(slot + 4).wrapping_add(1));
            live += 1;
        }
        slot += 8;
    }
    live
}
"""


class TestScanText(unittest.TestCase):
    def test_clean_rust_passes(self):
        self.assertIsNone(scan.scan_text(CLEAN_RUST))

    def test_disassembly_comment_blocked(self):
        self.assertEqual(scan.scan_text(CLEAN_RUST + "// original did: mov eax, ebx\n"), "disassembly")
        self.assertEqual(scan.scan_text("/* push ebp then call 0x10001000 */"), "disassembly")

    def test_inline_assembly_blocked(self):
        self.assertEqual(scan.scan_text(CLEAN_RUST + "    unsafe { asm!(\"nop\"); }\n"), "inline assembly")
        self.assertEqual(scan.scan_text("// built with global_asm!"), "inline assembly")

    def test_byte_dump_blocked(self):
        dump = " ".join(f"{b:02x}" for b in range(16))
        self.assertEqual(scan.scan_text("// bytes: " + dump), "byte dump")
        array = ", ".join(f"0x{b:02x}" for b in range(16))
        self.assertEqual(scan.scan_text("const T: [u8; 16] = [" + array + "];"), "byte dump")

    def test_machine_path_blocked(self):
        # Samples are built from pieces so this file itself passes the
        # publication check, which forbids the very strings under test.
        drive_path = "// built at " + "C:" + "\\" + "Users" + "\\cam\\work\\x"
        self.assertEqual(scan.scan_text(drive_path), "machine path")
        self.assertEqual(scan.scan_text("// see " + "steam" + "apps" + " workshop item"), "machine path")
        self.assertEqual(scan.scan_text("// from .artifacts/scratch/r-s01"), "machine path")

    def test_plain_prose_passes(self):
        self.assertIsNone(scan.scan_text("// forwards its two arguments and returns the result"))


class TestLaneImports(unittest.TestCase):
    def test_lane_import_stripped_shared_kept(self):
        text = "use lf_rs01_rw::rt::Mem;\nuse lf_checker_rt::State;\n"
        self.assertEqual(scan.strip_lane_imports(text), "use lf_checker_rt::State;\n")


class TestCheckerVersions(unittest.TestCase):
    def test_marker_rank(self):
        with tempfile.TemporaryDirectory() as tmp:
            lists = Path(tmp)
            self.assertEqual(scan.checker_of_lane(lists, "r-s01"), "version 1")
            self.assertEqual(scan.checker_of_lane(lists, "a-F01"), "version 2")  # re-run lanes
            (lists / "r-s01.v2").write_text("x", encoding="utf-8")
            self.assertEqual(scan.checker_of_lane(lists, "r-s01"), "version 2")
            (lists / "r-s01.v3").write_text("x", encoding="utf-8")
            self.assertEqual(scan.checker_of_lane(lists, "r-s01"), "version 3")
            (lists / "r-s01.v4").write_text("x", encoding="utf-8")
            self.assertEqual(scan.checker_of_lane(lists, "r-s01"), "version 4")

    def test_rank_orders_versions(self):
        self.assertLess(scan.RANK["version 1"], scan.RANK["version 2"])
        self.assertLess(scan.RANK["version 2"], scan.RANK["version 3"])
        self.assertLess(scan.RANK["version 3"], scan.RANK["version 4"])

    def test_short_version(self):
        self.assertEqual(scan.short_version("version 4"), "v4")
        self.assertEqual(scan.short_version("version 1"), "v1")


if __name__ == "__main__":
    unittest.main()
