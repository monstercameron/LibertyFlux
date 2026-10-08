from contextlib import contextmanager
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import types
import unittest
from unittest.mock import patch

TREE = Path(__file__).resolve().parents[3]
RUNNER_PATH = TREE / "scripts/coordinator/common_stock_runner.py"
spec = importlib.util.spec_from_file_location("runner_preflight_patch_test", RUNNER_PATH)
if spec is None or spec.loader is None:
    raise RuntimeError("cannot load scratch runner")
runner = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = runner
spec.loader.exec_module(runner)


def minimal_pe() -> bytes:
    data = bytearray(0x600)
    data[:2] = b"MZ"
    pe_offset = 0x80
    data[0x3C:0x40] = pe_offset.to_bytes(4, "little")
    data[pe_offset:pe_offset + 4] = b"PE\0\0"
    coff = pe_offset + 4
    data[coff:coff + 2] = (0x14C).to_bytes(2, "little")
    data[coff + 2:coff + 4] = (2).to_bytes(2, "little")
    data[coff + 16:coff + 18] = (224).to_bytes(2, "little")
    data[coff + 18:coff + 20] = (0x0102).to_bytes(2, "little")
    opt = pe_offset + 24
    data[opt:opt + 2] = (0x10B).to_bytes(2, "little")
    data[opt + 28:opt + 32] = (0x400000).to_bytes(4, "little")
    data[opt + 56:opt + 60] = (0x3000).to_bytes(4, "little")
    data[opt + 60:opt + 64] = (0x200).to_bytes(4, "little")
    section = opt + 224
    def write_section(offset: int, name: bytes, va: int, raw: int, rawptr: int, flags: int):
        data[offset:offset + 8] = name.ljust(8, b"\0")
        data[offset + 8:offset + 12] = (0x1000).to_bytes(4, "little")
        data[offset + 12:offset + 16] = va.to_bytes(4, "little")
        data[offset + 16:offset + 20] = raw.to_bytes(4, "little")
        data[offset + 20:offset + 24] = rawptr.to_bytes(4, "little")
        data[offset + 36:offset + 40] = flags.to_bytes(4, "little")
    write_section(section, b".text", 0x1000, 0x200, 0x200, 0x60000020)
    write_section(section + 40, b".data", 0x2000, 0x200, 0x400, 0xC0000040)
    code = (b"\x55\x8b\xec" + b"\xe8\0\0\0\0" + b"\xe9\0\0\0\0"
            + b"\x0f\x84\0\0\0\0")
    data[0x200:0x200 + len(code)] = code
    return bytes(data)


@contextmanager
def fixture_workspace():
    with tempfile.TemporaryDirectory(prefix="runner-preflight-") as temp:
        root = Path(temp).resolve()
        stock = {}
        stock_rel = {
            "driver": "scripts/checker/checker2.py",
            "local_slot": "scripts/coordinator/local_slot.py",
            "worker_source": "crates/tools/lf-checker-worker/src/main.rs",
            "worker_manifest": "crates/tools/lf-checker-worker/Cargo.toml",
            "worker_binary": ".artifacts/build/lf-checker-worker.exe",
            "proof_source": "crates/tools/lf-checker-proofs/src/lib.rs",
            "proof_manifest": "crates/tools/lf-checker-proofs/Cargo.toml",
            "proof_binary": ".artifacts/build/lf_checker_proofs.dll",
            "original_exe": "orig/GTAIV.exe",
        }
        for role, relative in stock_rel.items():
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            if role == "driver":
                path.write_bytes((runner.ROOT / "scripts/checker/checker2.py").read_bytes())
            elif role == "original_exe":
                path.write_bytes(minimal_pe())
            else:
                path.write_bytes(("fixture " + role).encode("ascii"))
            stock[role] = path
        build = root / ".artifacts/build"
        python = root / ".fixture-python/python.exe"
        python.parent.mkdir(parents=True)
        python.write_bytes(b"unused fixture interpreter")
        lane_root = root / ".artifacts/scratch/q-test"
        contracts = lane_root / "contracts"
        candidate = lane_root / "candidate"
        (candidate / "src").mkdir(parents=True)
        contracts.mkdir(parents=True)
        (candidate / "Cargo.toml").write_text("[package]\nname='f'\n", encoding="utf-8")
        (candidate / "Cargo.lock").write_text("# lock\n", encoding="utf-8")
        (candidate / "src/lib.rs").write_text("pub fn f() -> u32 { 1 }\n", encoding="utf-8")
        dll = lane_root / "candidate.dll"
        dll.write_bytes(b"not loaded")
        contract = {
            "name": "fixture", "function": "0x1000", "expected_function_va": "0x401000",
            "export": "fixture", "dll": str(dll), "mut_export": "fixture_mut",
            "callees": [{"id": 1, "conv": "cdecl", "nargs": 0, "ret": "u32", "script": [0]}],
            "patches": [{"site": "0x1003", "id": 1}],
            "tailpatches": [{"site": "0x1008", "id": 1}],
            "ctailpatches": [{"site": "0x100d", "id": 1}],
        }
        contract_path = contracts / "fixture.json"
        contract_path.write_text(json.dumps(contract), encoding="utf-8")
        with patch.object(runner, "ROOT", root), \
                patch.object(runner, "STOCK_PATHS", stock), \
                patch.object(runner, "BUILD", build):
            yield {"root": root, "stock": stock, "python": python, "lane_root": lane_root,
                   "contracts": contracts, "candidate": candidate, "dll": dll,
                   "contract_path": contract_path, "contract": contract}


class PreflightGuards(unittest.TestCase):
    def test_valid_rva_and_supported_patch_opcodes_are_pinned(self):
        with fixture_workspace() as ws:
            manifest, env = runner.make_manifest(
                "run", "q-test", ws["python"], ws["lane_root"] / "valid",
                ws["contracts"], "fixture", ws["candidate"], ws["dll"], 12)
            evidence = manifest["contract"]["function_validation"]
            self.assertEqual(evidence["function_rva"], "0x1000")
            self.assertEqual(evidence["function_va"], "0x401000")
            self.assertEqual(evidence["expected_function_va"], "0x401000")
            self.assertEqual(evidence["section"], ".text")
            self.assertEqual([row["opcode"] for row in evidence["patch_sites"]],
                             ["0xe8", "0xe9", "0x0f 0x84"])
            self.assertEqual(runner.verify_manifest(manifest, env), [])

    def test_wrong_va_rva_and_patch_opcode_fail_before_launch(self):
        with fixture_workspace() as ws:
            bad_cases = [
                ("wrong expected VA", {"expected_function_va": "0x401004"},
                 "does not equal contract.function"),
                ("non-executable function RVA", {"function": "0x2000"},
                 "non-executable section"),
                ("wrong E8 patch site", {"patches": [{"site": "0x1008", "id": 1}]},
                 "expected 0xE8"),
            ]
            for label, changes, message in bad_cases:
                with self.subTest(label=label):
                    contract = dict(ws["contract"])
                    contract.update(changes)
                    ws["contract_path"].write_text(json.dumps(contract), encoding="utf-8")
                    with patch.object(runner, "launch") as launch:
                        with self.assertRaisesRegex(ValueError, message):
                            runner.make_manifest(
                                "run", "q-test", ws["python"], ws["lane_root"] / ("bad-" + label.replace(" ", "-")),
                                ws["contracts"], "fixture", ws["candidate"], ws["dll"], 12)
                        launch.assert_not_called()

    def test_missing_or_empty_script_is_rejected_before_slot_acquisition(self):
        with fixture_workspace() as ws:
            for label, callee in (("missing", {"id": 7, "conv": "cdecl", "nargs": 0, "ret": "u32"}),
                                  ("empty", {"id": 7, "conv": "cdecl", "nargs": 0, "ret": "u32", "script": []})):
                with self.subTest(label=label):
                    contract = dict(ws["contract"])
                    contract["callees"] = [callee]
                    ws["contract_path"].write_text(json.dumps(contract), encoding="utf-8")
                    argv = [
                        "run", "--lane", "q-test", "--python", str(ws["python"]),
                        "--run-dir", str(ws["lane_root"] / ("no-slot-" + label)),
                        "--contract-dir", str(ws["contracts"]), "--contract", "fixture",
                        "--candidate-source-root", str(ws["candidate"]),
                        "--candidate-dll", str(ws["dll"]), "--trials", "12",
                    ]
                    with patch.object(runner, "preflight_driver_routing") as routing, \
                            patch.object(runner, "launch") as launch:
                        with self.assertRaisesRegex(ValueError, "needs a nonempty script"):
                            runner.cli(argv)
                        routing.assert_not_called()
                        launch.assert_not_called()


    def test_optional_wscript_absent_or_null_is_accepted(self):
        with fixture_workspace() as ws:
            for label, value in (("absent", ...), ("null", None), ("valid", [[0, {"stub": 1}]])):
                with self.subTest(label=label):
                    contract = dict(ws["contract"])
                    callee = dict(contract["callees"][0])
                    if value is ...:
                        callee.pop("wscript", None)
                    else:
                        callee["wscript"] = value
                    contract["callees"] = [callee]
                    ws["contract_path"].write_text(json.dumps(contract), encoding="utf-8")
                    manifest, env = runner.make_manifest(
                        "run", "q-test", ws["python"], ws["lane_root"] / ("optional-wscript-" + label),
                        ws["contracts"], "fixture", ws["candidate"], ws["dll"], 12)
                    self.assertEqual(manifest["contract"]["function_validation"]["function_rva"], "0x1000")
                    runner.preflight_driver_routing(manifest, env)

    def test_empty_or_non_list_wscript_is_rejected_before_slot(self):
        with fixture_workspace() as ws:
            bad_values = (
                ("empty", [], r"wscript must be a nonempty list when supplied"),
                ("string", "not-a-row-list", r"wscript must be a nonempty list when supplied"),
                ("object", {"row": [0]}, r"wscript must be a nonempty list when supplied"),
                ("non-list-row", ["not-a-row"], r"wscript\[0\] must be a list"),
                ("non-word", [[0, "bad"]], r"wscript\[0\]\[1\] must be an int or object"),
            )
            for label, wscript, message in bad_values:
                with self.subTest(label=label):
                    contract = dict(ws["contract"])
                    callee = dict(contract["callees"][0])
                    callee["wscript"] = wscript
                    contract["callees"] = [callee]
                    ws["contract_path"].write_text(json.dumps(contract), encoding="utf-8")
                    argv = [
                        "run", "--lane", "q-test", "--python", str(ws["python"]),
                        "--run-dir", str(ws["lane_root"] / ("no-slot-wscript-" + label)),
                        "--contract-dir", str(ws["contracts"]), "--contract", "fixture",
                        "--candidate-source-root", str(ws["candidate"]),
                        "--candidate-dll", str(ws["dll"]), "--trials", "12",
                    ]
                    with patch.object(runner, "preflight_driver_routing") as routing, \
                            patch.object(runner, "launch") as launch:
                        with self.assertRaisesRegex(ValueError, message):
                            runner.cli(argv)
                        routing.assert_not_called()
                        launch.assert_not_called()

    def test_late_script_entry_in_later_callee_is_preflighted_before_slot(self):
        with fixture_workspace() as ws:
            bad_entries = (
                ("bad-type", [0, "not-a-word-spec"], r"callee 2 script\[1\] must be an int or object"),
                ("bad-nested-spec", [0, {"lo": "not-a-word-spec"}], r"callee 2 script\[1\]\.lo"),
            )
            for label, script, message in bad_entries:
                with self.subTest(label=label):
                    contract = dict(ws["contract"])
                    contract["callees"] = [
                        dict(contract["callees"][0]),
                        {"id": 2, "conv": "cdecl", "nargs": 0, "ret": "u32", "script": script},
                    ]
                    ws["contract_path"].write_text(json.dumps(contract), encoding="utf-8")
                    argv = [
                        "run", "--lane", "q-test", "--python", str(ws["python"]),
                        "--run-dir", str(ws["lane_root"] / ("no-slot-late-entry-" + label)),
                        "--contract-dir", str(ws["contracts"]), "--contract", "fixture",
                        "--candidate-source-root", str(ws["candidate"]),
                        "--candidate-dll", str(ws["dll"]), "--trials", "12",
                    ]
                    with patch.object(runner, "launch") as launch:
                        with self.assertRaisesRegex(ValueError, message):
                            runner.cli(argv)
                        launch.assert_not_called()



if __name__ == "__main__":
    unittest.main(verbosity=2)
