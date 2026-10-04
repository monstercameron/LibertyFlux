"""Tests for mutate_rewrite.py on hand-written rewrite texts."""

import contextlib
import io
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import mutate_rewrite as mr

SAMPLE = """\
// original: 0x00401000 thing_update
/// Updates the counter at +0x10 when the flag is set; returns the helper's answer.
/// "Strings" and comments such as 0x99 == 1 are never mutated.
export!(thiscall, rw_00401000(this: u32, gain: f32) -> u32 {
    const COUNT_OFF: usize = 0x10;
    let mut buf = [0u32; 8];
    let label = "a < b";
    unsafe {
        let obj = this as *mut u8;
        let scaled: f32 = core::hint::black_box(gain) * core::hint::black_box(2.0f32);
        if *obj.add(4) != 0 && unsafe { *obj.add(5) } == 1 {
            *(obj.add(COUNT_OFF) as *mut u32) = 7;
            (obj.add(0x14) as *mut u16).write(3);
        }
        let ok: u32 =
            callee_thiscall!(2, u32, this, scaled.to_bits(), buf.as_mut_ptr() as u32);
        callee_cdecl!(3, u32, this);
        ok
    }
});
"""


def sites(op, text=SAMPLE):
    return [(text[s:e], new) for s, e, new, _ in mr.candidates(text)[op]]


class TestMasking(unittest.TestCase):
    def test_comments_strings_and_chars_are_blanked(self):
        code = mr.masked(SAMPLE)
        self.assertEqual(len(code), len(SAMPLE))
        self.assertNotIn("0x99", code)
        self.assertNotIn("a < b", code)
        self.assertEqual(mr.masked("let c = '{'; let d = 1;"), "let c = ' '; let d = 1;")


class TestOperators(unittest.TestCase):
    def test_cmp_flip(self):
        self.assertEqual(sorted(sites("cmp-flip")), [("!=", "=="), ("==", "!=")])
        # generics, arrows, shifts and match arms are not comparisons
        self.assertEqual(sites("cmp-flip", "fn f(x: Vec<u32>) -> u32 { match x { _ => 1 << 2 } }"), [])

    def test_constants_skip_callee_ids_and_array_lengths(self):
        befores = [before for before, _ in sites("const-inc")]
        # callee ids 2 and 3 and the array length 8 are skipped; the 3 left is the written value
        self.assertEqual(befores, ["0x10", "0u32", "4", "0", "5", "1", "7", "0x14", "3"])
        self.assertIn(("0x10", "0x11"), sites("const-inc"))
        self.assertIn(("7", "6"), sites("const-dec"))
        self.assertIn(("0x14", "0x4"), sites("const-bitflip"))
        self.assertIn(("0u32", "1u32"), sites("const-inc"))  # the suffix is kept
        self.assertEqual(sites("const-inc", "let a = 0xFFFF_FFFF;"), [])

    def test_drop_write(self):
        self.assertEqual([b for b, _ in sites("drop-write")],
                         ["*(obj.add(COUNT_OFF) as *mut u32) = 7;", "(obj.add(0x14) as *mut u16).write(3);"])

    def test_drop_write_never_cuts_a_continued_statement(self):
        text = "fn f(p: *mut u32) {\n    let v =\n        (p as *const u32).read_unaligned();\n    let w = v.wrapping_add(1);\n}\n"
        self.assertEqual(sites("drop-write", text), [])

    def test_width_change(self):
        self.assertEqual(sorted(new for _, new in sites("width-change")),
                         ["(obj.add(0x14) as *mut u8).write(((3) as u16) as u8);",
                          "*(obj.add(COUNT_OFF) as *mut u16) = ((7) as u32) as u16;"])

    def test_swap_args(self):
        self.assertEqual(sites("swap-args"), [("this, scaled.to_bits()", "scaled.to_bits(), this")])

    def test_drop_call(self):
        self.assertEqual(sorted(sites("drop-call")), sorted([
            ("callee_thiscall!(2, u32, this, scaled.to_bits(), buf.as_mut_ptr() as u32)", "<u32>::default()"),
            ("callee_cdecl!(3, u32, this);", "")]))

    def test_invert_branch_keeps_unsafe_blocks_in_the_condition(self):
        self.assertEqual(sites("invert-branch"),
                         [("*obj.add(4) != 0 && unsafe { *obj.add(5) } == 1", "!(*obj.add(4) != 0 && unsafe { *obj.add(5) } == 1)")])
        self.assertEqual(sites("invert-branch", "fn f(x: Option<u32>) { if let Some(v) = x { } }"), [])

    def test_float_swap(self):
        self.assertEqual(sites("float-swap"), [("core::hint::black_box(gain) * core::hint::black_box(2.0f32)",
                                                "core::hint::black_box(2.0f32) * core::hint::black_box(gain)")])
        # integer arithmetic is left alone
        self.assertEqual(sites("float-swap", "fn f(a: u32, b: u32) -> u32 { let c = a * b; c }"), [])


class TestMutate(unittest.TestCase):
    def test_mutants_parse_are_renamed_and_diverse(self):
        manifest = mr.mutate(SAMPLE, "functions/fn_00401000.rs", count=10)
        self.assertIsNone(manifest["skipped"])
        self.assertEqual(manifest["export"], "rw_00401000")
        self.assertEqual(len(manifest["mutants"]), 10)
        self.assertEqual(len({m["operator"] for m in manifest["mutants"]}), 10)
        for k, m in enumerate(manifest["mutants"]):
            self.assertEqual(m["export"], f"mut_00401000_{k}")
            self.assertTrue(mr.parses(m["text"], m["export"]))
            self.assertNotIn("rw_00401000", m["text"])
            self.assertNotEqual(m["before"], m["after"])
        self.assertEqual(manifest["dropped_unparsable"], 0)

    def test_deterministic(self):
        a = mr.mutate(SAMPLE, "f.rs", count=5, seed=3)
        b = mr.mutate(SAMPLE, "f.rs", count=5, seed=3)
        self.assertEqual(a, b)

    def test_skips(self):
        self.assertIn("one export", mr.mutate("// original: 0x00401000 f\nfn f() {}\n", "f.rs")["skipped"])
        self.assertIn("address", mr.mutate(SAMPLE.replace("// original: 0x00401000 thing_update\n", ""), "f.rs")["skipped"])
        self.assertIn("unbalanced", mr.mutate(SAMPLE[:-5], "f.rs")["skipped"])
        two = SAMPLE + "export!(cdecl, rw_00401001() -> u32 { 0 });\n"
        self.assertIn("one export", mr.mutate(two, "f.rs")["skipped"])

    def test_unparsable_mutants_are_dropped(self):
        original = mr.parses
        try:
            mr.parses = lambda text, name: False
            manifest = mr.mutate(SAMPLE, "f.rs", count=3)
        finally:
            mr.parses = original
        self.assertEqual(manifest["mutants"], [])
        self.assertGreater(manifest["dropped_unparsable"], 0)

    def test_main_writes_mutants_and_manifest(self):
        with tempfile.TemporaryDirectory() as tmp:
            src = os.path.join(tmp, "fn_00401000.rs")
            with open(src, "w", encoding="utf-8") as fh:
                fh.write(SAMPLE)
            out = os.path.join(tmp, "out")
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(mr.main([src, "--out", out, "-n", "4"]), 0)
            folder = os.path.join(out, "fn_00401000")
            with open(os.path.join(folder, "manifest.json"), encoding="utf-8") as fh:
                manifest = json.load(fh)
            self.assertEqual(len(manifest["mutants"]), 4)
            self.assertNotIn("text", manifest["mutants"][0])
            for m in manifest["mutants"]:
                with open(os.path.join(folder, m["file"]), encoding="utf-8") as fh:
                    self.assertIn(m["export"], fh.read())
            with contextlib.redirect_stdout(io.StringIO()) as printed:
                mr.main([src, "--dry-run"])
            self.assertIn("1 rewrites, 8 mutants", printed.getvalue())


if __name__ == "__main__":
    unittest.main()
