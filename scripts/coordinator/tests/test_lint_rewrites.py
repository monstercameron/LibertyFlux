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


FORWARDER = """\
// original: 0x00401000 thing_forward
/// Forwards both words of the pair to the shared helper.
export!(cdecl, rw_00401000(a: u32, b: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, a, b) }
});
"""

RESIDUE = """\
// original: 0x00401000 thing_notify
/// Notifies the sink. The original leaves the last helper answer in EAX.
export!(thiscall, rw_00401000(this: u32) -> u32 {
    unsafe {
        let _ = callee_thiscall!(1, u32, this);
        callee_cdecl!(2, u32, this);
    }
    0
});
"""


class TestStructure(unittest.TestCase):
    def test_balance(self):
        self.assertEqual(lint_rewrites.balance_problems(CLEAN), [])
        self.assertIn("still open", lint_rewrites.balance_problems(CLEAN[:CLEAN.index("this\n});")])[0])
        self.assertIn("closed by", lint_rewrites.balance_problems("fn f() { (] }")[0])
        self.assertIn("never ends", lint_rewrites.balance_problems('let s = "abc;\n')[0])
        # Delimiters inside strings, chars, comments, raw strings and lifetimes do not count.
        tricky = "fn f<'a>(x: &'a u8) -> char { let _ = \"({[\"; let _ = r#\"}\"#; /* ) */ // (\n '}' }"
        self.assertEqual(lint_rewrites.balance_problems(tricky), [])

    def test_unbalanced_finding(self):
        self.assertIn("unbalanced", codes(CLEAN[:CLEAN.index("    this\n")]))
        self.assertIn("unbalanced", codes(CLEAN.replace("});", "})")))

    def test_final_expression(self):
        fe = lint_rewrites.final_expression
        self.assertEqual(fe(" unsafe { let x = 1; x } "), "x")
        self.assertEqual(fe("let x = 1;\n return 0;"), "0")
        self.assertIsNone(fe("if a { 1 } else { 0 }"))
        self.assertEqual(fe("unsafe { f(); }\n 0"), "0")

    def test_constant_for_callee_result(self):
        self.assertIn("constant-for-callee-result", codes(RESIDUE))
        # Without the doc's claim, or when the answer is returned on some path, nothing is reported.
        self.assertNotIn("constant-for-callee-result", codes(RESIDUE.replace(" The original leaves the last helper answer in EAX.", "")))
        returned = RESIDUE.replace("        callee_cdecl!(2, u32, this);\n", "        if this == 0 { return callee_cdecl!(2, u32, this); }\n")
        self.assertNotIn("constant-for-callee-result", codes(returned))
        # A unit-typed callee has no answer to discard.
        unit = RESIDUE.replace("let _ = callee_thiscall!(1, u32, this);", "callee_thiscall!(1, (), this);").replace(
            "callee_cdecl!(2, u32, this);", "callee_cdecl!(2, (), this);")
        self.assertNotIn("constant-for-callee-result", codes(unit))

    def test_skipped_arguments(self):
        self.assertEqual(codes(FORWARDER), [])
        dropped = FORWARDER.replace("callee_cdecl!(1, u32, a, b)", "callee_cdecl!(1, u32, a, 0)")
        found = codes(dropped)
        self.assertIn("unused-parameter", found)
        self.assertIn("forwarder-drops-args", found)
        detail = [f["detail"] for f in lint_rewrites.lint_text(dropped, "f.rs", ADDRESS) if f["code"] == "unused-parameter"][0]
        self.assertIn("literal", detail)
        self.assertNotIn("unused-parameter", codes(FORWARDER.replace("(a: u32, b: u32)", "(a: u32, _b: u32)").replace(", a, b)", ", a, 0)")))

    def test_native_argument_gap(self):
        native = ("// original: 0x00401000 NATIVE\n/// Forwards script arguments.\n"
                  "export!(cdecl, rw_00401000(ctx: *const u8) -> u32 {\n    unsafe {\n"
                  "        let args = *(ctx.add(8) as *const u32) as *const u32;\n"
                  "        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(3))\n    }\n});\n")
        found = lint_rewrites.lint_text(native, "natives/fn_00401000.rs", ADDRESS)
        self.assertEqual([f["code"] for f in found], ["args-gap"])
        self.assertIn("[2]", found[0]["detail"])
        self.assertEqual(lint_rewrites.lint_text(native.replace("*args.add(3)", "*args.add(2)"), "natives/fn_00401000.rs", ADDRESS), [])

    def test_placeholders(self):
        self.assertIn("placeholder", codes(CLEAN.replace("/// Clears", "/// TODO: the flag path. Clears")))
        self.assertIn("placeholder", codes(CLEAN.replace("    this\n});", "    0 // placeholder for the real result\n});")))
        # Checker stubs and inventory placeholder names are not placeholders in the code.
        self.assertEqual(codes(CLEAN.replace("/// Clears", "/// The base constructor (a stubbed outgoing call) runs. Clears")), [])
        self.assertEqual(codes(CLEAN.replace("/// Clears", "/// Placeholder merged name: SOME_NATIVE. Clears")), [])
        self.assertEqual(codes(CLEAN.replace("/// Clears", "/// (merged symbol; placeholder name). Clears")), [])


class TestSystemic(unittest.TestCase):
    def test_counts(self):
        plain = "unsafe { *(p as *mut u32) = 1; }"
        rows = {r["code"]: r["files"] for r in lint_rewrites.systemic([plain, CLEAN, plain])}
        self.assertEqual(rows["plain-deref"], 2)
        self.assertEqual(rows["debug-overflow"], 3)


if __name__ == "__main__":
    unittest.main()
