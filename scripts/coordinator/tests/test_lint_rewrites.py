"""Tests for lint_rewrites.py on hand-written rewrite texts."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import lint_rewrites

ADDRESS = 0x00401000

CLEAN = """\
// original: 0x00401000 thing_reset
/// Clears the counter word and returns `this`.
export!(thiscall, rw_00401000(this: u32) -> u32 {
    const COUNTER_OFF: usize = 0x10;
    unsafe {
        core::ptr::write_unaligned((this as *mut u8).add(COUNTER_OFF) as *mut u32, 0);
    }
    this
});
"""


def codes(text, address=ADDRESS):
    return sorted(f["code"] for f in lint_rewrites.lint_text(text, "functions/fn_00401000.rs", address))


class TestLintText(unittest.TestCase):
    def test_clean(self):
        self.assertEqual(codes(CLEAN), [])

    def test_header_and_export_name(self):
        text = CLEAN.replace("0x00401000 thing", "0x0x00401000 thing").replace("rw_00401000", "rw_rs12_00401000")
        self.assertEqual(codes(text), ["export-name", "header"])

    def test_hand_declared_export_counts(self):
        text = CLEAN.replace('export!(thiscall, rw_00401000(this: u32) -> u32 {',
                             '#[unsafe(no_mangle)]\npub unsafe extern "thiscall" fn rw_00401000(this: u32) -> u32 {').replace("});", "}")
        self.assertEqual(codes(text), [])

    def test_no_export_and_wrong_version(self):
        self.assertIn("no-export", codes("// original: 0x00401000 f\npub fn rw_00401000() -> u32 { 0 }\n"))
        text = CLEAN + "export!(thiscall, mut_00401000(this: u32) -> u32 { this });\n"
        self.assertEqual(codes(text), ["wrong-version-tracked"])

    def test_partial(self):
        self.assertIn("partial", codes(CLEAN.replace("this\n});", "unreachable!()\n});")))
        self.assertIn("partial", codes(CLEAN.replace("/// Clears", "/// STAGE 1 SUBSET. Clears")))

    def test_misaligned_deref(self):
        text = CLEAN.replace("core::ptr::write_unaligned((this as *mut u8).add(COUNTER_OFF) as *mut u32, 0);",
                             "let obj = this as *mut u8;\n        *(obj.add(0xF1) as *mut u16) = 0;\n        *(obj.add(0x10) as *mut u32) = 0;")
        found = [f for f in lint_rewrites.lint_text(text, "f.rs", ADDRESS) if f["code"] == "misaligned-deref"]
        self.assertEqual(len(found), 1)
        self.assertIn("+0xF1", found[0]["detail"])
        self.assertNotIn("+0x10", found[0]["detail"])

    def test_misaligned_through_integer_address(self):
        text = CLEAN.replace("core::ptr::write_unaligned((this as *mut u8).add(COUNTER_OFF) as *mut u32, 0);",
                             "*((this + 0x209) as *mut u32) = 0;")
        self.assertIn("misaligned-deref", codes(text))

    def test_scaled_pointer_is_not_misaligned(self):
        text = CLEAN.replace("core::ptr::write_unaligned((this as *mut u8).add(COUNTER_OFF) as *mut u32, 0);",
                             "let words = this as *mut u32;\n        *(words.add(3) as *mut u32) = 0;")
        self.assertEqual(codes(text), [])

    def test_floats(self):
        body = "let x: f32 = f32::from_bits(this);\n    let y = x * 2.0 + 1.0;\n    y as i32 as u32"
        text = CLEAN.replace("    this\n});", "    " + body + "\n});")
        self.assertEqual(codes(text), ["float-order"])
        text = CLEAN.replace("    this\n});", "    let x: f32 = f32::from_bits(this);\n    x as i32 as u32\n});")
        self.assertEqual(codes(text), ["float-to-int"])
        pinned = CLEAN.replace("    this\n});", "    " + body.replace("x * 2.0", "core::hint::black_box(x) * 2.0") + "\n});")
        self.assertEqual(codes(pinned), [])

    def test_checker_only_and_literals(self):
        self.assertIn("checker-only", codes(CLEAN.replace("    this\n});", "    xmm_word(0, 0)\n});")))
        self.assertIn("image-literal", codes(CLEAN.replace("    this\n});", "    unsafe { *(0x00F12340 as *const u32) }\n});")))
        # Masks are not addresses, and literals in comments do not count.
        self.assertEqual(codes(CLEAN.replace("    this\n});", "    this & 0x007FFFFF // was 0x00F12340\n});")), [])

    def test_transliteration(self):
        text = CLEAN.replace("    this\n});", "    let uVar1 = this; let iVar2 = uVar1; let local_10 = iVar2;\n    local_10\n});")
        self.assertIn("transliteration", codes(text))


class TestSystemic(unittest.TestCase):
    def test_counts(self):
        plain = "unsafe { *(p as *mut u32) = 1; }"
        rows = {r["code"]: r["files"] for r in lint_rewrites.systemic([plain, CLEAN, plain])}
        self.assertEqual(rows["plain-deref"], 2)
        self.assertEqual(rows["debug-overflow"], 3)


if __name__ == "__main__":
    unittest.main()
