"""Tests for validate_contracts.py: the schema subset, the cross-reference checks and the narrowing list."""

import copy
import glob
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import validate_contracts as vc

SCHEMA = vc.load_schema()
LIMITS = dict(vc.DEFAULT_LIMITS)

# A small contract in the tracked style: one thiscall callee with a snapshot, every check on.
BASE = {
    "name": "t1", "function": "0x1000", "conv": "cdecl", "export": "rw_t1", "mut_export": "mut_t1",
    "trials": 200, "seed": 1, "timeout_ms": 10000, "ret": "eax",
    "regs": [{"any": True}, {"heap": 0, "plus": 0}, {"any": True}, {"any": True}, {"any": True},
             {"any": True}, {"any": True}],
    "stack": [{"kind": "int"}, {"kind": "ptr", "seg": 0}],
    "heapsegs": [{"off": 4096, "size": 16, "fill": "random", "pin": [{"at": 4, "vals": [{"stub": 1}]}]}],
    "globals": [], "globals_fill": "pristine",
    "callees": [{"id": 1, "conv": "thiscall", "nargs": 2, "ret": "u32", "script": "edges",
                 "snap": [{"kind": "arg", "idx": 0, "n": 4}]}],
    "patches": [{"site": "0x1010", "id": 1}], "iat": [],
    "checks": {"ret": "eax", "esp": True, "heap": True, "stack": True, "globals": True, "calls": True,
               "undeclared": True, "fulldata": True, "fp_tol": "0,0"},
}


def run(contract):
    return vc.validate(contract, SCHEMA, LIMITS)


def error_codes(contract):
    return sorted((e["code"], e["path"]) for e in run(contract)["errors"])


def warning_names(contract):
    return sorted(w["name"] for w in run(contract)["warnings"])


def changed(**edits):
    """BASE with dotted-path edits applied; a value of DELETE removes the key."""
    contract = copy.deepcopy(BASE)
    for dotted, value in edits.items():
        parts = dotted.split("__")
        node = contract
        for part in parts[:-1]:
            node = node[int(part)] if isinstance(node, list) else node[part]
        last = parts[-1]
        if value is DELETE:
            del node[last]
        elif isinstance(node, list):
            node[int(last)] = value
        else:
            node[last] = value
    return contract


DELETE = object()


class TestClean(unittest.TestCase):
    def test_base_is_clean(self):
        result = run(BASE)
        self.assertEqual(result["errors"], [])
        self.assertEqual(result["warnings"], [])

    def test_tracked_contracts_all_accepted(self):
        paths = sorted(glob.glob(str(HERE.parent / "contracts" / "*.json")))
        self.assertGreaterEqual(len(paths), 38)
        for path in paths:
            with open(path, encoding="utf-8") as fh:
                result = vc.validate(json.load(fh), SCHEMA, vc.worker_limits())
            self.assertEqual(result["errors"], [], os.path.basename(path))

    def test_pending_extension_fields_accepted(self):
        contract = changed(x87=[{"lo": 0, "hi": 0, "exp": 0x3FFF}])
        contract["xmm"] = {"2": [1, 2, 3, 4], "7": ["float", 0]}
        contract["callees"][0]["snap"] = [{"kind": "arg", "idx": 0, "n": 2, "at": 0x40}]
        self.assertEqual(error_codes(contract), [])
        self.assertIn("pending-field", [n["name"] for n in run(contract)["notes"]])


class TestSchemaErrors(unittest.TestCase):
    def test_unknown_top_level_key_with_hint(self):
        contract = changed(globals=DELETE)
        contract["global"] = []
        errors = run(contract)["errors"]
        self.assertEqual([(e["code"], e["path"]) for e in errors], [("unknown-key", "$.global")])
        self.assertIn('did you mean "globals"', errors[0]["message"])

    def test_unknown_nested_keys(self):
        contract = changed(checks__heep=True)
        contract["callees"][0]["snaps"] = []
        self.assertEqual(error_codes(contract), [("unknown-key", "$.callees[0].snaps"), ("unknown-key", "$.checks.heep")])

    def test_unknown_key_inside_alternatives_is_named(self):
        errors = run(changed(regs__0={"hep": 0}))["errors"]
        self.assertEqual(len(errors), 1)
        self.assertIn('unknown key "hep"', errors[0]["message"])

    def test_underscore_annotations_allowed(self):
        self.assertEqual(error_codes(changed(_why="documented")), [])

    def test_wrong_types(self):
        self.assertEqual(error_codes(changed(checks__fp_tol=0)), [("type", "$.checks.fp_tol")])
        self.assertEqual(error_codes(changed(checks__heap=1)), [("type", "$.checks.heap")])
        self.assertEqual(error_codes(changed(trials="200")), [("type", "$.trials")])
        self.assertEqual(error_codes(changed(function=4096)), [("type", "$.function")])

    def test_missing_required(self):
        self.assertEqual(error_codes(changed(checks=DELETE)), [("required", "$")])

    def test_ranges(self):
        self.assertIn(("range", "$.callees[0].id"), error_codes(changed(callees__0__id=256)))
        self.assertEqual(error_codes(changed(log_max=2048)), [("range", "$.log_max")])
        self.assertEqual(error_codes(changed(regs=BASE["regs"][:6])), [("range", "$.regs")])
        contract = copy.deepcopy(BASE)
        contract["tls"] = [{"slot": 64, "int": 0}]
        self.assertEqual(error_codes(contract), [("range", "$.tls[0].slot")])
        self.assertEqual(error_codes(changed(heapsegs__0__size=18)), [("range", "$.heapsegs[0].size")])

    def test_stack_kinds(self):
        self.assertEqual(error_codes(changed(stack__0={"kind": "ptrr"})), [("enum", "$.stack[0].kind")])
        self.assertEqual(error_codes(changed(stack__0={"kind": "ptr"})), [("required", "$.stack[0]")])
        self.assertEqual(error_codes(changed(stack__0={"kind": "int", "max": 3})), [("unknown-key", "$.stack[0].max")])
        self.assertEqual(error_codes(changed(stack__0={"kind": "srange", "lo": -4, "hi": 4})), [])

    def test_enums(self):
        self.assertEqual(error_codes(changed(checks__ret="eaxx")), [("enum", "$.checks.ret")])
        self.assertEqual(error_codes(changed(callees__0__ret="f32")), [("enum", "$.callees[0].ret")])


class TestSemanticErrors(unittest.TestCase):
    def test_undeclared_callee_references(self):
        contract = changed(patches__0__id=2)
        contract["checks"]["call_skip"] = {"3": [0]}
        contract["heapsegs"][0]["pin"][0]["vals"] = [{"stub": 4}]
        self.assertEqual(error_codes(contract), [("reference", "$.checks.call_skip.3"),
                                                 ("reference", "$.heapsegs[0].pin[0].vals[0].stub"),
                                                 ("reference", "$.patches[0].id")])

    def test_duplicate_callee(self):
        contract = copy.deepcopy(BASE)
        contract["callees"].append(copy.deepcopy(contract["callees"][0]))
        self.assertEqual(error_codes(contract), [("reference", "$.callees[1].id")])

    def test_missing_heap_segment(self):
        self.assertEqual(error_codes(changed(stack__1={"kind": "ptr", "seg": 3})), [("reference", "$.stack[1].seg")])

    def test_snapshot_cap_follows_the_worker_limit(self):
        contract = changed(callees__0__snap=[{"kind": "arg", "idx": 0, "n": 6}, {"kind": "ecx", "n": 3}])
        self.assertEqual(error_codes(contract), [("range", "$.callees[0].snap")])
        self.assertEqual(vc.validate(contract, SCHEMA, dict(LIMITS, SNAP_MAXW=32))["errors"], [])

    def test_worker_limits_are_read_from_source(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = os.path.join(tmp, "main.rs")
            with open(path, "w", encoding="utf-8") as fh:
                fh.write("const SNAP_MAXW: usize = 64;\nconst HEAP_USE: usize = 0x000F_0000;\n")
            limits = vc.worker_limits(path)
            self.assertEqual(limits["SNAP_MAXW"], 64)
            self.assertEqual(limits["HEAP_USE"], 0xF0000)
            self.assertEqual(limits["LOG_MAXW"], 40)
            self.assertEqual(vc.worker_limits(os.path.join(tmp, "absent.rs")), vc.DEFAULT_LIMITS)

    def test_limits_defined_through_another_constant(self):
        with tempfile.TemporaryDirectory() as tmp:
            with open(os.path.join(tmp, "main.rs"), "w", encoding="utf-8") as fh:
                fh.write("mod snap;\nconst SNAP_MAXW: usize = snap::SNAP_CAP_WORDS;\n")
            with open(os.path.join(tmp, "snap.rs"), "w", encoding="utf-8") as fh:
                fh.write("pub const SNAP_CAP_WORDS: usize = 64;\npub const SNAP_AT_LIMIT: i64 = 0x1_0000;\n")
            limits = vc.worker_limits(os.path.join(tmp, "main.rs"))
            self.assertEqual((limits["SNAP_MAXW"], limits["SNAP_AT_LIMIT"]), (64, 0x10000))

    def test_v5_extension_fields(self):
        contract = changed(callees__0__logxmm_regs=[2, 7], callees__0__xmm_from_stack={"3": 1})
        contract["callees"][0]["snap"] = [{"kind": "ecx", "n": 2, "at": -8}]
        contract["abs_shadow"] = True
        self.assertEqual(error_codes(contract), [])
        self.assertEqual(error_codes(changed(callees__0__logxmm_regs=[8])), [("range", "$.callees[0].logxmm_regs[0]")])
        far = changed(callees__0__snap=[{"kind": "arg", "idx": 0, "n": 1, "at": 0x20000}])
        self.assertEqual(error_codes(far), [("range", "$.callees[0].snap[0].at")])
        x87 = changed(x87=[[0, 0x80000000, 0x3FFF]], checks__x87_state=False)
        self.assertEqual(error_codes(x87), [("form", "$.checks.x87_state")])
        self.assertEqual(error_codes(changed(x87=[[0, 0, 0]] * 9)), [("range", "$.x87")])

    def test_heap_bounds_and_words(self):
        self.assertEqual(error_codes(changed(heapsegs__0__off=0xEFFF8)), [("range", "$.heapsegs[0]")])
        contract = changed(heapsegs__0__words=[0, 0, 0])
        self.assertEqual(error_codes(contract), [("range", "$.heapsegs[0].words")])

    def test_writes_overflow_and_zero_mask(self):
        contract = changed(callees__0__writes=[{"arg": 0, "at": 12, "n": 8}], callees__0__wscript=[[0]])
        self.assertEqual(error_codes(contract), [("range", "$.callees[0].writes[0]")])
        contract = changed(checks__call_mask={"1": {"0": "0x0"}})
        self.assertIn(("range", "$.checks.call_mask.1.0"), error_codes(contract))

    def test_eax_transport_pairing(self):
        contract = changed(callees__0__eax_from_stack=0)
        self.assertEqual(error_codes(contract), [("form", "$.callees[0].eax_from_stack")])
        contract["checks"]["call_regs"] = {"1": ["ecx", "eax"]}
        self.assertEqual(error_codes(contract), [])

    def test_skip_index_past_nargs(self):
        self.assertEqual(error_codes(changed(checks__call_skip={"1": [5]})), [("range", "$.checks.call_skip.1")])

    def test_mutant_same_as_export(self):
        self.assertEqual(error_codes(changed(mut_export="rw_t1")), [("form", "$.mut_export")])


class TestNarrowing(unittest.TestCase):
    def test_each_setting_is_named(self):
        cases = {
            "check-off:stack": changed(checks__stack=False),
            "check-off:fault": changed(checks__fault=False),
            "ret-none": changed(checks__ret="none", ret="none"),
            "ret-unread": changed(ret="al"),
            "fulldata-off": changed(checks__fulldata=False),
            "fp-tolerance": changed(checks__fp_tol="1e-6,0"),
            "call-skip:1": changed(checks__call_skip={"1": [1]}),
            "call-regs:1": changed(checks__call_regs={"1": []}),
            "call-mask:1": changed(checks__call_low8={"1": [0]}),
            "no-mut-export": changed(mut_export=DELETE),
            "min-orig-ok-share-zero": changed(min_orig_ok_share=0),
            "coverage-exempt:1": changed(coverage_exempt_callees=[1]),
            "few-trials": changed(trials=20),
        }
        for name, contract in cases.items():
            self.assertIn(name, warning_names(contract), name)
            self.assertEqual(run(contract)["errors"], [], name)

    def test_widening_call_regs_is_not_narrowing(self):
        self.assertEqual(warning_names(changed(checks__call_regs={"1": ["ecx", "edx"]})), [])

    def test_zero_tolerance_spellings(self):
        for tol in ("0,0", "0", "0.0, 0.0", ""):
            self.assertEqual(warning_names(changed(checks__fp_tol=tol)), [], tol)


class TestMain(unittest.TestCase):
    def test_exit_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            good, bad = os.path.join(tmp, "good.json"), os.path.join(tmp, "bad.json")
            with open(good, "w", encoding="utf-8") as fh:
                json.dump(BASE, fh)
            with open(bad, "w", encoding="utf-8") as fh:
                json.dump(changed(checks__heep=True), fh)
            import contextlib
            import io
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(vc.main([good]), 0)
                self.assertEqual(vc.main([tmp, "--json"]), 1)


if __name__ == "__main__":
    unittest.main()
