from __future__ import annotations

from contextlib import contextmanager
import importlib.util
import json
import os
from pathlib import Path
import sys
import sysconfig
import tempfile
import types
import unittest
from unittest.mock import Mock, patch


def find_repo_root(start: Path) -> Path:
    for parent in (start.resolve(), *start.resolve().parents):
        if ((parent / "AGENTS.md").is_file()
                and (parent / "scripts/coordinator/common_stock_runner.py").is_file()):
            return parent
    raise RuntimeError("cannot locate LibertyFlux repository root")


ROOT = find_repo_root(Path(__file__).resolve().parent)
RUNNER_PATH = Path(__file__).resolve().parent.parent / "common_stock_runner.py"
spec = importlib.util.spec_from_file_location("common_stock_runner_test_subject", RUNNER_PATH)
if spec is None or spec.loader is None:
    raise RuntimeError(f"cannot load runner at {RUNNER_PATH}")
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
    """Build hashable stock-input fixtures without game/build/venv files."""
    with tempfile.TemporaryDirectory(prefix="common-stock-runner-") as temp:
        root = Path(temp).resolve()
        stock_paths = {}
        for role, relative in {
            "driver": "scripts/checker/checker2.py",
            "local_slot": "scripts/coordinator/local_slot.py",
            "worker_source": "crates/tools/lf-checker-worker/src/main.rs",
            "worker_manifest": "crates/tools/lf-checker-worker/Cargo.toml",
            "worker_binary": ".artifacts/build/lf-checker-worker.exe",
            "proof_source": "crates/tools/lf-checker-proofs/src/lib.rs",
            "proof_manifest": "crates/tools/lf-checker-proofs/Cargo.toml",
            "proof_binary": ".artifacts/build/lf_checker_proofs.dll",
            "original_exe": "orig/GTAIV.exe",
        }.items():
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            if role == "driver":
                path.write_bytes((ROOT / "scripts/checker/checker2.py").read_bytes())
            elif role == "original_exe":
                path.write_bytes(minimal_pe())
            else:
                path.write_bytes(("fixture input " + role).encode("ascii"))
            stock_paths[role] = path
        build = root / ".artifacts/build"
        python = root / ".fixture-python/python.exe"
        python.parent.mkdir(parents=True)
        python.write_bytes(b"fixture interpreter; never executed")
        lane_root = root / ".artifacts/scratch/q-test"
        contract_dir = lane_root / "contracts"
        candidate_root = lane_root / "candidate"
        (candidate_root / "src").mkdir(parents=True)
        contract_dir.mkdir(parents=True)
        (candidate_root / "Cargo.toml").write_text(
            "[package]\nname='fixture'\nversion='0.1.0'\nedition='2021'\n", encoding="utf-8")
        (candidate_root / "Cargo.lock").write_text("# fixture lock\n", encoding="utf-8")
        rust_source = candidate_root / "src/lib.rs"
        rust_source.write_text("pub fn fixture() -> u32 { 1 }\n", encoding="utf-8")
        candidate_dll = lane_root / "candidate.dll"
        candidate_dll.write_bytes(b"fixture candidate image; never loaded")
        contract_path = contract_dir / "fixture.json"
        contract_path.write_text(json.dumps({
            "name": "fixture", "function": "0x1000", "export": "fixture",
            "dll": str(candidate_dll), "mut_export": "fixture_mut",
        }), encoding="utf-8")
        with patch.object(runner, "ROOT", root), \
                patch.object(runner, "STOCK_PATHS", stock_paths), \
                patch.object(runner, "BUILD", build):
            yield {
                "root": root, "stock_paths": stock_paths, "python": python,
                "lane_root": lane_root, "contract_dir": contract_dir,
                "contract_path": contract_path, "candidate_root": candidate_root,
                "candidate_dll": candidate_dll, "rust_source": rust_source,
            }


def fixture_manifest(workspace: dict, run_name: str = "run"):
    root = workspace["root"]
    return runner.make_manifest(
        "run", "q-test", workspace["python"], workspace["lane_root"] / run_name,
        workspace["contract_dir"], "fixture", workspace["candidate_root"],
        workspace["candidate_dll"], 12,
    )


def positive_verdict(**overrides):
    value = {
        "passed": True, "trials": 12, "orig_ok_trials": 12, "fails": 0,
        "vacuous": False, "contract_hash": "same",
        "coverage": {"checks_missing": [], "callees_declared": [],
                     "callees_fired_on_ok": {}},
    }
    value.update(overrides)
    return value


def negative_verdict(check="ret", **overrides):
    value = {
        "passed": False, "trials": 4, "orig_ok_trials": 4, "fails": 2,
        "contract_hash": "same",
        "first_mismatch": {"checks": [{"name": check, "passed": False}]},
    }
    value.update(overrides)
    return value


class StockRunnerFixtures(unittest.TestCase):
    def test_slot_owner_can_be_intermediate_verified_ancestor(self):
        parents = {700: 600, 600: 500, 500: 400}
        observed = {pid: runner.pid_chain(pid, parents) for pid in parents}
        valid = runner.slot_ancestry_evidence(400, 500, 700, observed)
        self.assertTrue(valid["ancestry_ok"])
        self.assertEqual(valid["slot_owner_ancestor_chain_sample"], [500, 400])
        unrelated = {700: 600, 600: 900, 900: 400, 500: 400}
        observed = {pid: runner.pid_chain(pid, unrelated) for pid in unrelated}
        invalid = runner.slot_ancestry_evidence(400, 500, 700, observed)
        self.assertFalse(invalid["ancestry_ok"])

    def test_slot_release_comes_from_this_transcript(self):
        transcript = (
            "2026-10-08 lane: acquired slot 1; wrapper PID 500\n"
            "2026-10-08 lane: child PID 700\n"
            "2026-10-08 lane: released slot; exit 0\n"
        )
        evidence = runner.slot_transcript_evidence(transcript, 400, 0)
        self.assertTrue(evidence["release_verified_from_this_transcript"])
        self.assertEqual(evidence["child_pid_from_transcript"], 700)
        self.assertEqual(evidence["owner_pid_from_transcript"], 500)

    def test_empty_callees_are_na_when_pair_has_real_evidence(self):
        result = runner.assess_verdicts(
            {"callees": [], "mut_export": "fixture_mut"},
            positive_verdict(), negative_verdict("calls"),
        )
        self.assertEqual(result["callee_coverage"], "n/a")
        self.assertTrue(result["function_pair_eligible"])

    def test_no_trial_setup_is_never_credited(self):
        result = runner.assess_verdicts(
            {"callees": [], "mut_export": "fixture_mut"},
            positive_verdict(trials=0, orig_ok_trials=0),
            negative_verdict(trials=0, orig_ok_trials=0, fails=0),
        )
        self.assertEqual(result["positive_state"], "no_trial_setup")
        self.assertFalse(result["positive_nonvacuous_pass"])
        self.assertFalse(result["function_pair_eligible"])
        self.assertFalse(result["no_trial_setup_credited"])

    def test_positive_requires_all_original_trials_and_explicit_complete_checks(self):
        contract = {"callees": [], "mut_export": "fixture_mut"}
        for coverage in (
            {"checks_missing": ["calls"], "callees_declared": []},
            {"callees_declared": []},
            None,
        ):
            result = runner.assess_verdicts(
                contract,
                positive_verdict(orig_ok_trials=11, coverage=coverage)
                if coverage is None else positive_verdict(coverage=coverage),
                None,
            )
            self.assertFalse(result["positive_nonvacuous_pass"])
        result = runner.assess_verdicts(
            contract, positive_verdict(orig_ok_trials=11), None)
        self.assertFalse(result["positive_nonvacuous_pass"])

    def test_negative_requires_completed_originals_and_semantic_mismatch(self):
        contract = {"callees": [], "mut_export": "fixture_mut"}
        for check in ("termination", "no_cheat"):
            result = runner.assess_verdicts(
                contract, positive_verdict(), negative_verdict(check))
            self.assertFalse(result["negative_semantic_mismatch"])
            self.assertFalse(result["same_contract_negative_caught"])
        result = runner.assess_verdicts(
            contract, positive_verdict(), negative_verdict(orig_ok_trials=0))
        self.assertFalse(result["same_contract_negative_caught"])
        self.assertFalse(result["function_pair_eligible"])

    def test_declared_callee_requires_success_coverage(self):
        result = runner.assess_verdicts(
            {"callees": [{"id": 9}], "mut_export": "fixture_mut"},
            positive_verdict(coverage={"checks_missing": [], "callees_declared": ["9"],
                                      "callees_fired_on_ok": {"9": 0}}),
            negative_verdict("ret"),
        )
        self.assertEqual(result["callee_coverage"], "uncovered")
        self.assertFalse(result["function_pair_eligible"])

    def test_nonpositive_trial_override_rejected_without_repository_runtime(self):
        with tempfile.TemporaryDirectory() as temp:
            python = Path(temp) / "python.exe"
            python.write_bytes(b"fixture interpreter")
            with self.assertRaisesRegex(ValueError, "must be positive"):
                runner.make_manifest("selftest", "q-test", python,
                                     Path(temp) / "run", trials=0)

    def test_manifest_pins_contract_and_candidate_source_without_build_files(self):
        with fixture_workspace() as workspace:
            manifest, env = fixture_manifest(workspace)
            self.assertEqual(runner.verify_manifest(manifest, env), [])
            workspace["contract_path"].write_text(
                json.dumps({"name": "fixture", "function": "fixture",
                            "export": "fixture", "dll": str(workspace["candidate_dll"]),
                            "mut_export": "fixture_mut", "note": "changed"}),
                encoding="utf-8")
            self.assertIn("contract changed", runner.verify_manifest(manifest, env))
            workspace["rust_source"].write_text("pub fn fixture() -> u32 { 2 }\n", encoding="utf-8")
            self.assertIn("candidate source tree changed", runner.verify_manifest(manifest, env))

    def test_contract_output_name_must_match_safe_basename(self):
        with fixture_workspace() as workspace:
            workspace["contract_path"].write_text(json.dumps({
                "name": "../outside", "dll": str(workspace["candidate_dll"]),
                "mut_export": "fixture_mut",
            }), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "safe selected basename"):
                fixture_manifest(workspace, "unsafe-name")

    def test_parent_preflight_routes_contract_and_validates_before_slot(self):
        with fixture_workspace() as workspace:
            manifest, env = fixture_manifest(workspace)
            fake = types.SimpleNamespace(
                BUILD=env["LF_CHECKER_BUILD_DIR"], WORKER=env["LF_CHECKER_WORKER"],
                DLL=env["LF_CHECKER_DLL"], EXE=env["LIBERTYFLUX_ORIG_EXE"],
                OUT=env["LF_CHECKER_OUT"], HERE=str(runner.DRIVER.parent),
                validate_contract=Mock(),
            )
            with patch.object(runner, "import_stock_driver", return_value=fake):
                runner.preflight_driver_routing(manifest, env)
            self.assertEqual(Path(fake.HERE), workspace["lane_root"])
            fake.validate_contract.assert_called_once()

    def test_canonical_driver_is_loaded_unmodified_and_stock_pinned(self):
        env = runner.effective_environment({}, ROOT / ".artifacts/scratch/q-test/out")
        manifest = {"stock_inputs": {"driver": {"path": "scripts/checker/checker2.py"}}}
        driver_hash = runner.sha256_file(ROOT / "scripts/checker/checker2.py")
        with patch.dict(os.environ, env, clear=True):
            module = runner.import_stock_driver(env, manifest)
        self.assertEqual(Path(module.__file__).resolve(), (ROOT / "scripts/checker/checker2.py").resolve())
        self.assertEqual(driver_hash, runner.EXPECTED_STOCK_DRIVER_SHA256)
        self.assertEqual(Path(module.HERE).resolve(), (ROOT / "scripts/checker").resolve())

    def test_worker_start_uses_stdlib_queue_when_coordinator_dir_leads_path(self):
        env = runner.effective_environment({}, ROOT / ".artifacts/scratch/q-test/queue-out")
        manifest = {"stock_inputs": {"driver": {"path": "scripts/checker/checker2.py"}}}
        coordinator = (ROOT / "scripts/coordinator").resolve()
        previous_path = list(sys.path)
        previous_queue = sys.modules.get("queue")
        shadow = types.ModuleType("queue")
        shadow.__file__ = str(coordinator / "queue.py")
        try:
            sys.path[:] = [str(coordinator)] + [
                entry for entry in previous_path
                if Path(entry or os.curdir).resolve() != coordinator
            ]
            self.assertEqual(Path(sys.path[0]).resolve(), coordinator)
            sys.modules["queue"] = shadow
            module = runner.import_stock_driver(env, manifest)
            expected_queue = (Path(sysconfig.get_path("stdlib")) / "queue.py").resolve()
            self.assertEqual(Path(module.queue.__file__).resolve(), expected_queue)
            self.assertTrue(hasattr(module.queue, "Queue"))
            self.assertTrue(hasattr(module.queue, "Empty"))

            class FakeProcess:
                pass

            fake_thread = types.SimpleNamespace(start=Mock())
            worker = module.Worker.__new__(module.Worker)
            with patch.object(module.subprocess, "Popen", return_value=FakeProcess()), \
                    patch.object(module.threading, "Thread", return_value=fake_thread), \
                    patch.object(module.Worker, "recv", return_value={"ready": True}):
                worker.start()
            self.assertIsInstance(worker.q, module.queue.Queue)
        finally:
            sys.path[:] = previous_path
            if previous_queue is None:
                sys.modules.pop("queue", None)
            else:
                sys.modules["queue"] = previous_queue


class CargoLockPreparationFixtures(unittest.TestCase):
    CASES = json.loads((Path(__file__).parent / "fixtures" / "cargo_lock_cases.json").read_text(encoding="utf-8"))

    def assert_lock_case(self, case_name: str):
        case = self.CASES[case_name]
        with fixture_workspace() as workspace:
            source_root = workspace["candidate_root"]
            manifest = source_root / "Cargo.toml"
            manifest.write_text(
                "[package]\nname='fixture'\nversion='0.1.0'\nedition='2021'\n"
                "[lib]\ncrate-type=['cdylib']\n"
                "[dependencies]\nruntime={path='runtime'}\n"
                "[workspace]\n",
                encoding="utf-8",
            )
            dependency_root = source_root / "runtime"
            (dependency_root / "src").mkdir(parents=True)
            (dependency_root / "Cargo.toml").write_text(
                "[package]\nname='runtime'\nversion='0.1.0'\nedition='2021'\n",
                encoding="utf-8",
            )
            dependency_source = dependency_root / "src/lib.rs"
            dependency_source.write_text("pub fn stable() -> u32 { 7 }\n", encoding="utf-8")
            lockfile = source_root / "Cargo.lock"
            if lockfile.exists():
                lockfile.unlink()
            if case["initial"] is not None:
                lockfile.write_bytes(case["initial"].encode("utf-8"))
            before = runner.build_candidate_source(manifest)
            cargo = workspace["root"] / "fixture-cargo.exe"
            seen = {}

            def fake_local_slot(argv):
                seen["argv"] = argv
                self.assertEqual(argv[argv.index("--") + 1:argv.index("--") + 3],
                                 [str(cargo.resolve()), "metadata"])
                if case["resolved"] is not None:
                    lockfile.write_bytes(case["resolved"].encode("utf-8"))
                return types.SimpleNamespace(returncode=0, stdout=("2026-10-08 q-test: acquired slot 1; wrapper PID 500\n" + "2026-10-08 q-test: child PID 700\n" + "2026-10-08 q-test: released slot; exit 0\n"))

            record, transcript = runner.prepare_cargo_lock_metadata(
                "q-test", workspace["python"], cargo, manifest, invoke=fake_local_slot)
            after = runner.build_candidate_source(manifest)

            self.assertEqual(record["status"], {
                "absent": "created", "normalized": "normalized", "already_existing": "unchanged"
            }[case_name])
            self.assertEqual(record["lock_before"]["present"], case["initial"] is not None)
            self.assertTrue(record["lock_after"]["present"])
            expected = case["resolved"] if case["resolved"] is not None else case["initial"]
            self.assertEqual(record["lock_after"]["sha256"], runner.hashlib.sha256(expected.encode()).hexdigest())
            self.assertEqual(before["local_path_dependencies"], after["local_path_dependencies"])
            self.assertIn("child PID 700", transcript)
            self.assertEqual(seen["argv"][seen["argv"].index("--cwd") + 1], str(runner.ROOT.resolve()))
            self.assertIn("--offline", seen["argv"])

    def test_absent_lock_is_created_before_identity(self):
        self.assert_lock_case("absent")

    def test_existing_lock_is_normalized_before_identity(self):
        self.assert_lock_case("normalized")

    def test_already_existing_lock_stays_stable(self):
        self.assert_lock_case("already_existing")


if __name__ == "__main__":
    unittest.main(verbosity=2)
