"""Tests for assemble.py on synthetic rewrites (synthetic addresses; no game data).

Run from the repository root: python -m unittest discover -s scripts/assemble/tests
The one test that runs cargo is skipped unless LF_ASSEMBLE_CARGO=1 (it needs the i686-pc-windows-msvc target).
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import assemble

BASE = 0x10000000


def addr(i):
    return f"0x{BASE + i * 0x10:08X}"


def key(i):
    return f"{BASE + i * 0x10:08x}"


def entry(i, kind="function"):
    return {"address": addr(i), "name": f"f{i}", "kind": kind, "file": f"{kind}s/fn_{key(i)}.rs", "trials": 1000,
            "checker": "version 4", "lane": "t-1"}


def simple(i, body="a.wrapping_add(1)", conv="cdecl"):
    return (f"// original: {addr(i)} f{i}\n/// Adds one.\n"
            f"lf_checker_rt::export!({conv}, rw_{key(i)}(a: u32) -> u32 {{\n    {body}\n}});\n")


def calling(i, slot="1"):
    return (f"// original: {addr(i)} f{i}\nuse lf_checker_rt::{{callee_cdecl, export}};\n"
            f"export!(cdecl, rw_{key(i)}(a: u32) -> u32 {{\n    callee_cdecl!({slot}, u32, a)\n}});\n")


class TestScan(unittest.TestCase):
    def test_export_macro_qualified_or_not(self):
        info = assemble.scan(simple(1, conv="thiscall"))
        self.assertEqual((info["error"], info["export"], info["conv"]), (None, f"rw_{key(1)}", "thiscall"))
        info = assemble.scan("lf_k2_rt::export!(fastcall, rw_x(a: u32, b: u32) -> u32 { a });")
        self.assertEqual((info["error"], info["export"], info["conv"]), (None, "rw_x", "fastcall"))

    def test_no_mangle_function_form(self):
        info = assemble.scan('#[unsafe(no_mangle)]\npub extern "C" fn rw_y(a: u32) -> u32 { a }')
        self.assertEqual((info["export"], info["conv"]), ("rw_y", "cdecl"))
        info = assemble.scan('#[unsafe(no_mangle)]\npub extern "system" fn rw_z() -> u32 { 0 }')
        self.assertEqual((info["export"], info["conv"]), ("rw_z", "stdcall"))

    def test_missing_or_ambiguous_export(self):
        self.assertIn("no export", assemble.scan("fn helper() {}")["error"])
        two = simple(1) + "export!(cdecl, rw_other() -> u32 { 0 });\n"
        self.assertIn("several exports", assemble.scan(two)["error"])

    def test_wrong_version_beside_the_rewrite(self):
        text = simple(1) + "export!(cdecl, mut_x(a: u32) -> u32 { a });\n"
        info = assemble.scan(text)
        self.assertEqual((info["error"], info["export"]), (None, f"rw_{key(1)}"))
        self.assertEqual(info["all_exports"], ["mut_x", f"rw_{key(1)}"])
        single = assemble.scan("export!(cdecl, rs9_x(a: u32) -> u32 { a });")
        self.assertEqual((single["error"], single["export"]), (None, "rs9_x"))
        other = assemble.scan(simple(1) + "export!(cdecl, helper(a: u32) -> u32 { a });\n")
        self.assertIn("several exports", other["error"])
        two_rw = simple(1) + "export!(cdecl, rw_y(a: u32) -> u32 { a });\n" + "export!(cdecl, mut_x() -> u32 { 0 });\n"
        self.assertIn("several exports", assemble.scan(two_rw)["error"])

    def test_foreign_block_cannot_link(self):
        text = simple(1) + 'unsafe extern "C" { fn missing(); }\n'
        self.assertIn("foreign functions", assemble.scan(text)["error"])

    def test_lane_runtimes(self):
        text = "lf_rn94_rt::export!(cdecl, rw_a(c: u32) -> u32 { c });"
        self.assertIn("lf_rn94_rt", assemble.scan(text)["error"])
        self.assertIsNone(assemble.scan(text, aliases=("lf_k2_rt", "lf_rn94_rt"))["error"])
        self.assertIsNone(assemble.scan("lf_k2_rt::export!(cdecl, rw_a(c: u32) -> u32 { c });")["error"])
        local = "use lf_checker_rt as rt;\nrt::export!(cdecl, rw_a(c: u32) -> u32 { rt::relocated(c) });"
        self.assertIsNone(assemble.scan(local)["error"])

    def test_callee_ids(self):
        info = assemble.scan(calling(1, "3") + "fn g() -> u32 { lf_k2_rt::callee_addr(0x7) }\n")
        self.assertTrue(info["uses_callees"])
        self.assertEqual(info["callee_ids"], [3, 7])
        self.assertTrue(info["callee_ids_literal"])
        named = assemble.scan(calling(1, "SLOT_BASE"))
        self.assertTrue(named["uses_callees"])
        self.assertFalse(named["callee_ids_literal"])
        self.assertEqual(named["callee_ids"], [])

    def test_comments_do_not_count(self):
        text = simple(1) + "// was: callee_cdecl!(4, u32, a)\n/* callee_addr(5) */\n"
        info = assemble.scan(text)
        self.assertFalse(info["uses_callees"])
        self.assertTrue(info["callee_ids_literal"])

    def test_checker_only_inputs(self):
        info = assemble.scan(simple(1, body="lf_checker_rt::xmm_word(0, 0)"))
        self.assertEqual(info["checker_only"], ["xmm_word"])
        self.assertEqual(assemble.scan(simple(1))["checker_only"], [])


class TestProductionCopy(unittest.TestCase):
    def test_callees_are_redirected(self):
        text = ("//! inner doc\n#![allow(dead_code)]\nuse lf_k2_rt::{callee_addr, callee_thiscall};\n"
                "fn f(t: u32) -> u32 { lf_checker_rt::callee_thiscall!(2, u32, t); callee_cdecl !(1, u32, t);"
                " lf_k2_rt::callee_addr(4) + callee_addr(5) }\n")
        copy = assemble.production_copy(text)
        self.assertIn("// inner doc", copy)
        self.assertNotIn("#![", copy)
        self.assertIn("use lf_k2_rt::{callee_addr, callee_thiscall};", copy)
        self.assertIn("__lf_callee_thiscall!(2, u32, t)", copy)
        self.assertIn("__lf_callee_cdecl!(1, u32, t)", copy)
        self.assertIn("__lf_callee(4) + __lf_callee(5)", copy)
        self.assertNotIn("lf_checker_rt::callee", copy)

    def test_macros_match_the_runtime_patterns(self):
        runtime = (assemble.ROOT / "crates" / "tools" / "lf-checker-rt" / "src" / "lib.rs").read_text(encoding="utf-8")
        for conv in ("cdecl", "stdcall", "thiscall", "fastcall"):
            start = runtime.index(f"macro_rules! callee_{conv} {{")
            pattern = runtime[runtime.index("(", start):runtime.index("=>", start)].strip()
            mine = assemble.CALLEE_MACROS
            at = mine.index(f"macro_rules! __lf_callee_{conv} {{")
            self.assertEqual(mine[mine.index("(", at):mine.index("=>", at)].strip(), pattern, conv)


class TestCalleeMap(unittest.TestCase):
    def write(self, data):
        handle = tempfile.NamedTemporaryFile("w", suffix=".json", delete=False, encoding="utf-8")
        json.dump(data, handle)
        handle.close()
        self.addCleanup(os.unlink, handle.name)
        return handle.name

    def test_valid_map(self):
        path = self.write({"format": "lf-callee-map/1", "functions": {addr(1): {
            "callees": {"1": {"va": "0x10400000"}, "2": {"iat": "0x10800004"},
                        "3": {"va": "0x10400010", "transport": "xmm0_from_stack"}},
            "complete": True, "expected": "558BEC"}}})
        loaded = assemble.load_callee_map(path)[BASE + 0x10]
        self.assertEqual(loaded["callees"][1], {"kind": "va", "address": 0x10400000, "transport": None})
        self.assertEqual(loaded["callees"][2]["kind"], "iat")
        self.assertEqual(loaded["callees"][3]["transport"], "xmm0_from_stack")
        self.assertTrue(loaded["complete"])
        self.assertEqual(loaded["expected"], bytes([0x55, 0x8B, 0xEC]))

    def test_invalid_maps(self):
        bad = [
            {"format": "other", "functions": {}},
            {"format": "lf-callee-map/1", "functions": {"12": {}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {"x": {"va": "0x10400000"}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {"300": {"va": "0x10400000"}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {"1": {}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {
                "1": {"va": "0x10400000", "iat": "0x10400000"}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {"1": {"va": "0x1000"}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"callees": {
                "1": {"va": "0x10400000", "transport": "teleport"}}}}},
            {"format": "lf-callee-map/1", "functions": {addr(1): {"expected": "5"}}},
        ]
        for data in bad:
            with self.assertRaises(ValueError, msg=json.dumps(data)):
                assemble.load_callee_map(self.write(data))


class TestSwitchability(unittest.TestCase):
    def mapped(self, callees, complete=False):
        return {"callees": {s: {"kind": "va", "address": 0x10400000 + s, "transport": t} for s, t in callees.items()},
                "complete": complete, "expected": b""}

    def test_rules(self):
        plain = assemble.scan(simple(1))
        self.assertEqual(assemble.switchability(plain, None), (True, [], {}))
        calls = assemble.scan(calling(1, "1"))
        ok, reasons, _ = assemble.switchability(calls, None)
        self.assertFalse(ok)
        self.assertIn("no entry", reasons[0])
        ok, reasons, used = assemble.switchability(calls, self.mapped({1: None}))
        self.assertTrue(ok)
        self.assertEqual(list(used), [1])
        ok, reasons, _ = assemble.switchability(calls, self.mapped({2: None}))
        self.assertEqual((ok, reasons), (False, ["callee slot 1 is not in the map"]))
        ok, reasons, _ = assemble.switchability(calls, self.mapped({1: "noclean"}))
        self.assertIn("noclean", reasons[0])

    def test_named_slots_need_a_complete_entry(self):
        named = assemble.scan(calling(1, "SLOT"))
        ok, reasons, _ = assemble.switchability(named, self.mapped({1: None}))
        self.assertFalse(ok)
        self.assertIn("not marked complete", reasons[0])
        ok, _, used = assemble.switchability(named, self.mapped({1: None, 2: None}, complete=True))
        self.assertTrue(ok)
        self.assertEqual(sorted(used), [1, 2])
        ok, reasons, _ = assemble.switchability(named, self.mapped({1: None, 2: "eax_from_stack"}, complete=True))
        self.assertFalse(ok)

    def test_checker_only_is_never_switchable(self):
        info = assemble.scan(simple(1, body="lf_checker_rt::tls_slot(1)"))
        ok, reasons, _ = assemble.switchability(info, None)
        self.assertFalse(ok)
        self.assertIn("tls_slot", reasons[0])


class TestResolver(unittest.TestCase):
    def test_arms(self):
        source = "\n".join(assemble.resolver({
            1: {"kind": "va", "address": 0x10400000, "transport": None},
            2: {"kind": "iat", "address": 0x10800004, "transport": None},
            3: {"kind": "va", "address": 0x10400010, "transport": "noclean"}}))
        self.assertIn("1 => lf_checker_rt::relocated(0x10400000),", source)
        self.assertIn("2 => unsafe { (lf_checker_rt::relocated(0x10800004) as *const u32).read_volatile() },", source)
        self.assertNotIn("3 =>", source)
        self.assertIn("_ => 0,", source)

    def test_empty_resolver_returns_zero(self):
        source = "\n".join(assemble.resolver({}))
        self.assertIn("fn __lf_callee(_id: u32) -> u32 {", source)


class TestAttribute(unittest.TestCase):
    def message(self, spans, text="mismatched types", children=()):
        return json.dumps({"reason": "compiler-message", "message": {
            "level": "error", "message": text, "rendered": text, "spans": spans,
            "children": [{"spans": c} for c in children]}})

    def test_spans_and_expansions(self):
        direct = self.message([{"file_name": f"src/rw/{key(1)}.rs", "line_start": 3}])
        via_macro = self.message([{"file_name": "src/lib.rs", "line_start": 9, "expansion": {
            "span": {"file_name": f"src\\rw\\{key(2)}.rs", "line_start": 4}}}])
        lib_line = self.message([{"file_name": "src/lib.rs", "line_start": 40}], text="no function rw_x")
        child = self.message([], children=[[{"file_name": f"src/rw/{key(4)}.rs", "line_start": 1}]])
        loose = self.message([{"file_name": "src/lib.rs", "line_start": 2}], text="odd")
        noise = json.dumps({"reason": "compiler-artifact"}) + "\nnot json"
        found, unattributed = assemble.attribute("\n".join([direct, via_macro, lib_line, child, loose, noise]),
                                                 {40: key(3)})
        self.assertEqual(found, {key(1): "mismatched types", key(2): "mismatched types",
                                 key(3): "no function rw_x", key(4): "mismatched types"})
        self.assertEqual(unattributed, ["odd"])

    def test_warnings_are_ignored(self):
        warning = json.dumps({"reason": "compiler-message", "message": {
            "level": "warning", "message": "unused", "spans": [{"file_name": f"src/rw/{key(1)}.rs"}]}})
        self.assertEqual(assemble.attribute(warning, {}), ({}, []))


class TestSharding(unittest.TestCase):
    def test_address_order_chunks(self):
        rows = [{"address": addr(i)} for i in (5, 1, 4, 2, 3)]
        chunks = assemble.shard_rows(rows, 2)
        self.assertEqual([[r["address"] for r in c] for c in chunks],
                         [[addr(1), addr(2)], [addr(3), addr(4)], [addr(5)]])
        self.assertEqual(assemble.shard_rows([], 3), [])


class TestAssemble(unittest.TestCase):
    """The whole generator with a fake type-checker: no cargo involved."""

    def setUp(self):
        self.out = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, self.out, ignore_errors=True)
        self.files = {
            entry(1)["file"]: simple(1),
            entry(2)["file"]: calling(2, "1"),
            entry(3)["file"]: simple(3, body="lf_checker_rt::xmm_word(1, 0)"),
            entry(4)["file"]: "lf_rn94_rt::export!(cdecl, rw_x(c: u32) -> u32 { c });",
            entry(5)["file"]: simple(5).replace(f"rw_{key(5)}", f"rw_{key(1)}"),
            entry(6)["file"]: simple(6, body="does_not_compile()"),
            entry(8, "native")["file"]: simple(8) + "export!(cdecl, mut_shared() -> u32 { 0 });\n",
            entry(9)["file"]: simple(9) + "export!(cdecl, mut_shared() -> u32 { 1 });\n",
        }
        self.index = [entry(i) for i in range(1, 8)] + [entry(8, "native"), entry(9)]

    def read(self, name):
        return self.files.get(name)

    def fake_checker(self, out, index, rows, sources, callee_maps, log, aliases):
        failed = {r["key"]: "cannot find function `does_not_compile`" for r in rows if "does_not_compile" in
                  sources[r["key"]]}
        kept = [r for r in rows if r["key"] not in failed]
        assemble.write_shard(out, index, kept, sources, callee_maps, aliases)
        return kept, failed, {"rounds": 1 + bool(failed), "check_seconds": 0.1, "check_peak_rss_mb": 1.0}

    def run_assemble(self, **kw):
        return assemble.assemble(self.index, self.read, self.out, shard_size=2, checker=self.fake_checker,
                                 log=lambda _m: None, **kw)

    def test_manifest_accounts_for_every_entry(self):
        m = self.run_assemble()
        self.assertEqual(m["index_entries"], 9)
        self.assertEqual(m["included"] + m["excluded_count"], 9)
        reasons = {e["address"]: (e["stage"], e["reason"]) for e in m["excluded"]}
        self.assertEqual(reasons[addr(4)][0], "scan")
        self.assertIn("lf_rn94_rt", reasons[addr(4)][1])
        self.assertEqual(reasons[addr(5)][0], "duplicate")
        self.assertEqual(reasons[addr(6)], ("compile", "cannot find function `does_not_compile`"))
        self.assertEqual((reasons[addr(7)][0], reasons[addr(7)][1]), ("scan", "file missing"))
        self.assertEqual(reasons[addr(9)][0], "duplicate")
        self.assertIn("mut_shared", reasons[addr(9)][1])
        rows = {r["address"]: r for r in m["rewrites"]}
        self.assertEqual(sorted(rows), [addr(1), addr(2), addr(3), addr(8)])
        self.assertTrue(rows[addr(1)]["switchable"])
        self.assertFalse(rows[addr(2)]["switchable"])
        self.assertFalse(rows[addr(3)]["switchable"])
        self.assertEqual(rows[addr(8)]["handle"], f"native.{key(8)}")
        self.assertEqual(rows[addr(8)]["extra_exports"], ["mut_shared"])
        self.assertEqual(m["switchable"], 2)
        self.assertEqual(json.loads((self.out / "manifest.json").read_text()), m)

    def test_crates_are_written(self):
        m = self.run_assemble(check=True)
        # Five rows survive the scan (1, 2, 3, 6, 8); two per shard. Row 6 fails the check in shard s001.
        self.assertEqual([(s["name"], s["count"]) for s in m["shards"]], [("s000", 2), ("s001", 1), ("s002", 1)])
        workspace = (self.out / "Cargo.toml").read_text()
        self.assertIn('"shards/s000",', workspace)
        self.assertIn('panic = "abort"', workspace)
        lib = (self.out / "shards" / "s000" / "src" / "lib.rs").read_text()
        self.assertIn(f"pub mod m_{key(1)} {{", lib)
        self.assertIn(f'include!("rw/{key(1)}.rs");', lib)
        self.assertIn("extern crate lf_checker_rt as lf_k2_rt;", lib)
        self.assertIn(f'row(0x{BASE + 0x10:08X}, m_{key(1)}::rw_{key(1)} as *const () as usize, "function.{key(1)}", 0, 1, &[]),', lib)
        copy = (self.out / "shards" / "s000" / "src" / "rw" / f"{key(2)}.rs").read_text()
        self.assertIn("__lf_callee_cdecl!(1, u32, a)", copy)
        top = (self.out / "top" / "src" / "lib.rs").read_text()
        self.assertIn("lf_rewrite_32::export_table!(lf_rw_s000::entries, lf_rw_s001::entries, lf_rw_s002::entries);",
                      top)
        template = json.loads((self.out / "callee-map.template.json").read_text())
        self.assertEqual(template["functions"], {addr(2): {"complete": False, "callees": {"1": {"va": None}}}})

    def test_callee_map_makes_a_rewrite_switchable(self):
        cmap = {BASE + 0x20: {"callees": {1: {"kind": "va", "address": 0x10400000, "transport": None}},
                              "complete": False, "expected": b"\x55\x8b"}}
        m = self.run_assemble(callee_map=cmap, callee_map_info={"path": "map.json", "sha256": "0", "functions": 1})
        row = next(r for r in m["rewrites"] if r["address"] == addr(2))
        self.assertTrue(row["switchable"])
        self.assertEqual(row["callees"], {"1": {"va": "0x10400000"}})
        lib = (self.out / "shards" / "s000" / "src" / "lib.rs").read_text()
        self.assertIn("1 => lf_checker_rt::relocated(0x10400000),", lib)
        self.assertIn(", 0, 7, &[0x55, 0x8B]),", lib)

    def test_rerun_removes_stale_copies_and_keeps_unchanged_files(self):
        self.run_assemble()
        lib = self.out / "shards" / "s000" / "src" / "lib.rs"
        before = lib.stat().st_mtime_ns
        self.run_assemble()
        self.assertEqual(lib.stat().st_mtime_ns, before)
        del self.files[entry(2)["file"]]
        self.run_assemble()
        self.assertFalse((self.out / "shards" / "s000" / "src" / "rw" / f"{key(2)}.rs").exists())

    def test_without_check_everything_scanned_is_listed(self):
        m = assemble.assemble(self.index, self.read, self.out, shard_size=3, check=False, log=lambda _m: None)
        self.assertEqual(m["included"], 5)
        self.assertTrue(all(s["check_seconds"] is None for s in m["shards"]))


@unittest.skipUnless(os.environ.get("LF_ASSEMBLE_CARGO") == "1", "set LF_ASSEMBLE_CARGO=1 to run cargo")
class TestWithCargo(unittest.TestCase):
    """Real type-check of three synthetic rewrites, one of which does not compile."""

    def test_check_drops_the_broken_file(self):
        out = assemble.ROOT / ".artifacts" / "build" / "assemble-test"
        files = {entry(1)["file"]: simple(1, conv="thiscall"), entry(2)["file"]: calling(2, "1"),
                 entry(3)["file"]: simple(3, body="does_not_compile()")}
        m = assemble.assemble([entry(i) for i in (1, 2, 3)], files.get, out, shard_size=2, log=lambda _m: None)
        self.assertEqual(m["included"], 2)
        self.assertEqual([(e["address"], e["stage"]) for e in m["excluded"]], [(addr(3), "compile")])
        result = subprocess.run(["cargo", "check", "--target", assemble.TARGET, "-p", "lf-rewrites"], cwd=out,
                                env=assemble.cargo_env(out), capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr[-3000:])


if __name__ == "__main__":
    unittest.main()
