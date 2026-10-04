"""Tests for lint_rewrites.py on hand-written rewrite texts."""

import contextlib
import io
import json
import os
import re
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

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


MISALIGNED = CLEAN.replace("core::ptr::write_unaligned((this as *mut u8).add(COUNTER_OFF) as *mut u32, 0);",
                           "let obj = this as *mut u8;\n        *(obj.add(0xF1) as *mut u16) = 0;")


def first_finding(text, code, path="functions/fn_00401000.rs", address=ADDRESS):
    return next(f for f in lint_rewrites.lint_text(text, path, address) if f["code"] == code)


def line_of(text, fragment):
    return text[:text.index(fragment)].count("\n") + 1


class TestCodes(unittest.TestCase):
    def test_codes_list_every_code_lint_text_can_report(self):
        source = Path(lint_rewrites.__file__).read_text(encoding="utf-8")
        added = set(re.findall(r'\badd\("([a-z-]+)"', source))
        self.assertEqual(added, set(lint_rewrites.CODES))
        self.assertEqual({r["code"] for r in lint_rewrites.systemic([CLEAN])}, set(lint_rewrites.SYSTEMIC_CODES))


class TestFindingLine(unittest.TestCase):
    def line(self, text, code, **kwargs):
        return lint_rewrites.finding_line(text, first_finding(text, code, **kwargs))

    def test_file_level_and_header(self):
        self.assertIsNone(self.line("// original: 0x00401000 f\npub fn rw_00401000() -> u32 { 0 }\n", "no-export"))
        self.assertEqual(self.line("\n\n" + CLEAN.replace("0x00401000 thing", "0x0x00401000 thing"), "header"), 3)

    def test_points_at_the_offending_text(self):
        self.assertEqual(self.line(MISALIGNED, "misaligned-deref"), line_of(MISALIGNED, "*(obj.add(0xF1)"))
        partial = CLEAN.replace("    this\n});", "    unreachable!()\n});")
        self.assertEqual(self.line(partial, "partial"), line_of(partial, "unreachable!"))
        wrong = CLEAN + "export!(thiscall, mut_00401000(this: u32) -> u32 { this });\n"
        self.assertEqual(self.line(wrong, "wrong-version-tracked"), line_of(wrong, "export!(thiscall, mut_"))
        renamed = CLEAN.replace("rw_00401000", "rw_rs12_00401000")
        self.assertEqual(self.line(renamed, "export-name"), line_of(renamed, "export!("))
        literal = CLEAN.replace("    this\n});", "    // 0x00F12340 in a comment first\n    unsafe { *(0x00F12340 as *const u32) }\n});")
        self.assertEqual(self.line(literal, "image-literal"), line_of(literal, "unsafe { *(0x00F1"))
        temps = CLEAN.replace("    this\n});", "    let uVar1 = this; let iVar2 = uVar1; let local_10 = iVar2;\n    local_10\n});")
        self.assertEqual(self.line(temps, "transliteration"), line_of(temps, "let uVar1"))
        placeholder = CLEAN.replace("    this\n});", "    0 // placeholder for the real result\n});")
        self.assertEqual(self.line(placeholder, "placeholder"), line_of(placeholder, "0 // placeholder"))

    def test_floats_prefer_code_over_comments(self):
        body = "/// a is scaled.\nexport!(cdecl, rw_00401000(a: f32) -> u32 {\n    let b: f32 = a * 2.0;\n    b as i32 as u32\n});\n"
        text = "// original: 0x00401000 f\n" + body
        self.assertEqual(self.line(text, "float-order"), line_of(text, "let b: f32 = a * 2.0"))
        self.assertEqual(self.line(text, "float-to-int"), line_of(text, "b as i32"))

    def test_structure_findings(self):
        truncated = CLEAN[:CLEAN.index("    this\n")]
        self.assertEqual(self.line(truncated, "unbalanced"), line_of(truncated, "export!("))
        mismatch = CLEAN.replace("    this\n});", "    (this\n});")
        self.assertEqual(self.line(mismatch, "unbalanced"), line_of(mismatch, "});"))
        no_semicolon = CLEAN.replace("});", "})")
        found = [f for f in lint_rewrites.lint_text(no_semicolon, "f.rs", ADDRESS) if f["title"].startswith("Export macro")]
        self.assertEqual(lint_rewrites.finding_line(no_semicolon, found[0]), line_of(no_semicolon, "})"))

    def test_function_level_findings(self):
        dropped = FORWARDER.replace("callee_cdecl!(1, u32, a, b)", "callee_cdecl!(1, u32, a, 0)")
        self.assertEqual(self.line(dropped, "forwarder-drops-args"), line_of(dropped, "callee_cdecl!"))
        self.assertEqual(self.line(dropped, "unused-parameter"), line_of(dropped, "export!("))
        self.assertEqual(self.line(RESIDUE, "constant-for-callee-result"), line_of(RESIDUE, "let _ = callee_thiscall!"))


class TestGithubAnnotations(unittest.TestCase):
    def test_escaping(self):
        self.assertEqual(lint_rewrites.escape_data("50% a\nb\r"), "50%25 a%0Ab%0D")
        self.assertEqual(lint_rewrites.escape_property("a:b,c%"), "a%3Ab%2Cc%25")

    def test_annotation(self):
        finding = first_finding(MISALIGNED, "misaligned-deref")
        line = lint_rewrites.finding_line(MISALIGNED, finding)
        text = lint_rewrites.github_annotation(finding, "rewrites/verified/functions/fn_00401000.rs", line)
        self.assertTrue(text.startswith(f"::warning file=rewrites/verified/functions/fn_00401000.rs,line={line},"
                                        "title=misaligned-deref (high%2C certain)::Aligned dereference at an odd offset: "))
        self.assertNotIn("\n", text)
        self.assertNotIn("line=", lint_rewrites.github_annotation(finding, "f.rs"))


@contextlib.contextmanager
def repository(index=None, files=None):
    """A temporary repository root (LIBERTYFLUX_ROOT points at it) with an optional verified tree."""
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        folder = root / "rewrites" / "verified"
        for name, text in (files or {}).items():
            (folder / name).parent.mkdir(parents=True, exist_ok=True)
            (folder / name).write_text(text, encoding="utf-8")
        if index is not None:
            folder.mkdir(parents=True, exist_ok=True)
            (folder / "index.json").write_text(json.dumps(index), encoding="utf-8")
        with mock.patch.dict(os.environ, {"LIBERTYFLUX_ROOT": str(root)}):
            yield root


def run_main(*argv):
    out = io.StringIO()
    with contextlib.redirect_stdout(out):
        code = lint_rewrites.main(list(argv))
    return code, out.getvalue()


class TestMain(unittest.TestCase):
    INDEX = [{"address": "0x00401000", "file": "functions/fn_00401000.rs"},
             {"address": "0x00401010", "file": "functions/fn_00401010.rs"}]

    def files(self):
        return {"functions/fn_00401000.rs": CLEAN, "functions/fn_00401010.rs": MISALIGNED.replace("00401000", "00401010")}

    def test_default_summary_format_is_unchanged(self):
        with repository(self.INDEX, self.files()):
            code, out = run_main()
        self.assertEqual(code, 0)
        self.assertEqual(out, "checked 2 rewrites; 1 findings in 1 files\n"
                              "  misaligned-deref       high   certain 1\n"
                              "  systemic plain-deref: 1 files\n"
                              "  systemic debug-overflow: 2 files\n")

    def test_code_filter(self):
        with repository(self.INDEX, self.files()):
            _, out = run_main("--json", "--code", "header", "debug-overflow")
            data = json.loads(out)
            self.assertEqual((data["checked"], data["findings"]), (2, []))
            self.assertEqual([r["code"] for r in data["systemic"]], ["debug-overflow"])
            _, out = run_main("--json", "--code", "misaligned-deref")
            self.assertEqual([f["code"] for f in json.loads(out)["findings"]], ["misaligned-deref"])
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as stop:
                run_main("--code", "no-such-code")
            self.assertEqual(stop.exception.code, 2)

    def test_github_format_over_the_index(self):
        with repository(self.INDEX, self.files()):
            code, out = run_main("--format", "github")
            self.assertEqual(code, 0)
            lines = out.splitlines()
            line = line_of(MISALIGNED, "*(obj.add(0xF1)")
            self.assertTrue(lines[0].startswith(f"::warning file=rewrites/verified/functions/fn_00401010.rs,line={line},"))
            self.assertEqual([x.split("::")[1].split(" ")[0] for x in lines[1:3]], ["notice", "notice"])
            self.assertEqual(lines[-1], "checked 2 rewrites; 1 findings in 1 files")
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                run_main("--format", "github", "--json")

    def test_file_option_lints_outside_the_index(self):
        with repository() as root:  # no index at all: --file must not need one
            lane = root / ".artifacts" / "scratch" / "r-s1"
            (lane / "natives").mkdir(parents=True)
            (lane / "fn_00401000.rs").write_text(CLEAN.replace("0x00401000 thing", "0x00401010 thing"), encoding="utf-8")
            native = ("// original: 0x00401020 NATIVE\n/// Forwards script arguments.\n"
                      "export!(cdecl, rw_00401020(ctx: *const u8) -> u32 {\n    unsafe {\n"
                      "        let args = *(ctx.add(8) as *const u32) as *const u32;\n"
                      "        callee_cdecl!(1, u32, *args, *args.add(2))\n    }\n});\n")
            (lane / "natives" / "attempt.rs").write_text(native, encoding="utf-8")
            paths = [str(lane / "fn_00401000.rs"), str(lane / "natives" / "attempt.rs")]
            _, out = run_main("--json", "--file", *paths)
            data = json.loads(out)
            self.assertEqual(data["checked"], 2)
            # The name's address wins over the header's; a name without one falls back to the header.
            self.assertEqual(sorted(f["code"] for f in data["findings"]), ["args-gap", "header"])
            self.assertEqual({f["file"] for f in data["findings"]}, {p.replace("\\", "/") for p in paths})
            _, out = run_main("--format", "github", "--file", *paths)
            self.assertIn("::warning file=.artifacts/scratch/r-s1/fn_00401000.rs,line=1,title=header", out)
            _, out = run_main("--json", "--file", str(lane))  # a folder: every .rs file under it
            self.assertEqual(json.loads(out)["findings"], data["findings"])
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as stop:
                run_main("--file", str(lane / "missing.rs"))
            self.assertEqual(stop.exception.code, 2)

    def test_address_for(self):
        self.assertEqual(lint_rewrites.address_for("x/fn_00401000.rs", "// original: 0x00401010 f\n"), 0x00401000)
        self.assertEqual(lint_rewrites.address_for("x/attempt.rs", "\n// original: 0x00401010 f\n"), 0x00401010)
        self.assertIsNone(lint_rewrites.address_for("attempt.rs", "// original: 0x0x00401010 f\n"))
        self.assertEqual(codes(CLEAN.replace("0x00401000 thing", "no header"), None), [])


if __name__ == "__main__":
    unittest.main()
