from __future__ import annotations

from contextlib import contextmanager
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch


def find_repo_root(start: Path) -> Path:
    for parent in (start.resolve(), *start.resolve().parents):
        if ((parent / "AGENTS.md").is_file()
                and (parent / "scripts/checker/checker2.py").is_file()):
            return parent
    raise RuntimeError("cannot locate LibertyFlux repository root")


ROOT = find_repo_root(Path(__file__).resolve().parent)
RUNNER_PATH = Path(__file__).resolve().parent.parent / "common_stock_runner.py"
spec = importlib.util.spec_from_file_location("common_stock_build_test_subject", RUNNER_PATH)
if spec is None or spec.loader is None:
    raise RuntimeError(f"cannot load runner at {RUNNER_PATH}")
runner = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = runner
spec.loader.exec_module(runner)


@contextmanager
def workspace():
    with tempfile.TemporaryDirectory(prefix="common-stock-build-") as temp:
        root = Path(temp).resolve()
        lane_root = root / ".artifacts/scratch/q-build"
        crate = lane_root / "candidate"
        (crate / "src").mkdir(parents=True)
        manifest = crate / "Cargo.toml"
        manifest.write_text(
            '[package]\nname = "fixture-pkg"\nversion = "0.1.0"\nedition = "2021"\n'
            '[lib]\nname = "fixture_lib"\ncrate-type = ["cdylib"]\n',
            encoding="utf-8")
        (crate / "Cargo.lock").write_text("# fixture lock\n", encoding="utf-8")
        source = crate / "src/lib.rs"
        source.write_text("pub fn fixture() -> u32 { 1 }\n", encoding="utf-8")
        python = root / ".fixture-python/python.exe"
        python.parent.mkdir(parents=True)
        python.write_bytes(b"fake interpreter; not executed")
        cargo = root / ".fixture-cargo/cargo.exe"
        cargo.parent.mkdir(parents=True)
        cargo.write_bytes(b"fake cargo; never executed")
        slot = root / "scripts/coordinator/local_slot.py"
        slot.parent.mkdir(parents=True)
        slot.write_text("# slot path fixture\n", encoding="utf-8")
        with patch.object(runner, "ROOT", root), \
                patch.object(runner, "SLOT", slot):
            yield {"root": root, "lane_root": lane_root, "crate": crate,
                   "manifest": manifest, "source": source, "python": python,
                   "cargo": cargo, "slot": slot}


def success_transcript(exit_code: int = 0) -> str:
    return (
        "2026-10-08 q-build: acquired slot 1; wrapper PID 400\n"
        "2026-10-08 q-build: child PID 500\n"
        f"2026-10-08 q-build: released slot; exit {exit_code}\n"
    )


class CommonStockBuildFixtures(unittest.TestCase):
    def test_build_cli_resolves_explicit_paths_from_repo_root(self):
        with workspace() as work:
            with patch.object(runner, "run_build", return_value=0) as build:
                self.assertEqual(runner.cli([
                    "build", "--lane", "q-build", "--python", ".fixture-python/python.exe",
                    "--manifest-path", ".artifacts/scratch/q-build/candidate/Cargo.toml",
                ]), 0)
            build.assert_called_once_with(
                "q-build", work["python"], work["manifest"].resolve())

    def test_direct_build_api_resolves_relative_paths_from_root(self):
        with workspace() as work:
            manifest_arg = work["manifest"].relative_to(work["root"])
            with tempfile.TemporaryDirectory(prefix="explicit-python-") as external:
                external_python = Path(external) / "python.exe"
                external_python.write_bytes(b"explicit interpreter outside repository")
                with patch.object(runner.shutil, "which", return_value=str(work["cargo"])):
                    plan = runner.make_build_plan("q-build", external_python, manifest_arg)
                self.assertEqual(Path(plan["python"]["path"]), external_python.resolve())
            self.assertEqual(Path(plan["cargo_manifest"]), manifest_arg)

    def test_each_build_plan_uses_unique_lane_target_subdirectory(self):
        with workspace() as work:
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])):
                first = runner.make_build_plan("q-build", work["python"], work["manifest"])
                second = runner.make_build_plan("q-build", work["python"], work["manifest"])
            first_target = (runner.ROOT / first["target_dir"]).resolve()
            second_target = (runner.ROOT / second["target_dir"]).resolve()
            lane_target_root = work["root"] / ".artifacts/build/lanes/q-build"
            self.assertEqual(first_target.parent, lane_target_root)
            self.assertEqual(second_target.parent, lane_target_root)
            self.assertNotEqual(first["build_id"], second["build_id"])
            self.assertNotEqual(first_target, second_target)
            first_expected_dll = (runner.ROOT / first["expected_dll"]).resolve()
            self.assertEqual(first_expected_dll.parent.parent.parent, first_target)

    def test_missing_or_unowned_manifest_fails_before_local_slot(self):
        with workspace() as work:
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run") as launch:
                missing = work["lane_root"] / "candidate/missing/Cargo.toml"
                with self.assertRaises(FileNotFoundError):
                    runner.run_build("q-build", work["python"], missing)
                outside = work["root"] / "outside/Cargo.toml"
                outside.parent.mkdir(parents=True)
                outside.write_text("[package]\nname='outside'\n", encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "inside the lane scratch"):
                    runner.run_build("q-build", work["python"], outside)
                launch.assert_not_called()

    def test_transitive_local_path_dependencies_are_hash_pinned(self):
        with workspace() as work:
            dep = work["lane_root"] / "deps/dep"
            leaf = work["lane_root"] / "deps/leaf"
            (dep / "src").mkdir(parents=True)
            (leaf / "src").mkdir(parents=True)
            work["manifest"].write_text(
                '[package]\nname="fixture-pkg"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n'
                '[dependencies]\nfixture_dep={path="../deps/dep"}\n', encoding="utf-8")
            (dep / "Cargo.toml").write_text(
                '[package]\nname="fixture-dep"\nversion="0.1.0"\nedition="2021"\n'
                '[dependencies]\nfixture_leaf={path="../leaf"}\n', encoding="utf-8")
            (leaf / "Cargo.toml").write_text(
                '[package]\nname="fixture-leaf"\nversion="0.1.0"\nedition="2021"\n', encoding="utf-8")
            (dep / "src/lib.rs").write_text("pub fn dep() -> u32 { 1 }\n", encoding="utf-8")
            leaf_source = leaf / "src/lib.rs"
            leaf_source.write_text("pub fn leaf() -> u32 { 1 }\n", encoding="utf-8")
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])):
                before = runner.make_build_plan("q-build", work["python"], work["manifest"])
                leaf_source.write_text("pub fn leaf() -> u32 { 2 }\n", encoding="utf-8")
                after = runner.make_build_plan("q-build", work["python"], work["manifest"])
            dependency_record = before["candidate_source"]["local_path_dependencies"]
            self.assertEqual(dependency_record["status"], "complete")
            self.assertEqual(len(dependency_record["manifests"]), 2)
            self.assertNotEqual(before["candidate_source"]["sha256"], after["candidate_source"]["sha256"])

            def drift_transitive_dependency(argv, **kwargs):
                args = argv[argv.index("--") + 1:]
                if args[1] == "metadata":
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                target_dir = Path(args[args.index("--target-dir") + 1])
                output = target_dir / runner.BUILD_TARGET / "release/fixture_lib.dll"
                output.parent.mkdir(parents=True)
                output.write_bytes(b"fixture DLL built before dependency drift")
                leaf_source.write_text("pub fn leaf() -> u32 { 3 }\n", encoding="utf-8")
                return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=drift_transitive_dependency):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 3)
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "candidate_source_changed")
            self.assertIsNone(result["generation_dll"])

    def test_workspace_inherited_local_path_dependency_is_pinned(self):
        with workspace() as work:
            ws = work["lane_root"] / "workspace"
            member = ws / "member"
            dep = ws / "deps/dep"
            (member / "src").mkdir(parents=True)
            (dep / "src").mkdir(parents=True)
            workspace_manifest = ws / "Cargo.toml"
            workspace_manifest.write_text(
                '[workspace]\nmembers=["member"]\n'
                '[workspace.dependencies]\nfixture_dep={path="deps/dep"}\n', encoding="utf-8")
            member_manifest = member / "Cargo.toml"
            member_manifest.write_text(
                '[package]\nname="fixture-member"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n'
                '[dependencies]\nfixture_dep={workspace=true}\n', encoding="utf-8")
            (member / "src/lib.rs").write_text("pub fn member() {}\n", encoding="utf-8")
            (dep / "Cargo.toml").write_text(
                '[package]\nname="fixture-dep"\nversion="0.1.0"\nedition="2021"\n', encoding="utf-8")
            (dep / "src/lib.rs").write_text("pub fn dep() {}\n", encoding="utf-8")
            snapshot = runner.build_candidate_source(member_manifest)
            self.assertEqual(snapshot["status"], "complete")
            self.assertEqual(snapshot["local_path_dependencies"]["manifests"],
                             [runner.evidence_path(dep / "Cargo.toml")])
            self.assertEqual(snapshot["workspace_manifests"], [{
                "path": runner.evidence_path(workspace_manifest),
                "sha256": runner.sha256_file(workspace_manifest),
            }])

    def test_ancestor_workspace_lock_is_pinned(self):
        with workspace() as work:
            ws = work["lane_root"] / "workspace"
            member = ws / "member"
            (member / "src").mkdir(parents=True)
            workspace_manifest = ws / "Cargo.toml"
            workspace_manifest.write_text('[workspace]\nmembers=["member"]\n', encoding="utf-8")
            workspace_lock = ws / "Cargo.lock"
            workspace_lock.write_text("version = 4\n# first\n", encoding="utf-8")
            member_manifest = member / "Cargo.toml"
            member_manifest.write_text(
                '[package]\nname="fixture-member"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n', encoding="utf-8")
            (member / "src/lib.rs").write_text("pub fn member() {}\n", encoding="utf-8")
            before = runner.build_candidate_source(member_manifest)
            first_hash = runner.sha256_file(workspace_lock)
            workspace_lock.write_text("version = 4\n# second\n", encoding="utf-8")
            after = runner.build_candidate_source(member_manifest)
            self.assertEqual(before["workspace_locks"], [{
                "path": runner.evidence_path(workspace_lock),
                "present": True,
                "sha256": first_hash,
            }])
            self.assertNotEqual(before["workspace_locks"][0]["sha256"],
                                after["workspace_locks"][0]["sha256"])
            self.assertNotEqual(before["sha256"], after["sha256"])

    def test_workspace_patch_override_fails_closed_before_slot(self):
        with workspace() as work:
            ws = work["lane_root"] / "workspace"
            member = ws / "member"
            (member / "src").mkdir(parents=True)
            workspace_manifest = ws / "Cargo.toml"
            workspace_manifest.write_text(
                '[workspace]\nmembers=["member"]\n'
                '[patch.crates-io]\nfixture-override={path="../override"}\n', encoding="utf-8")
            member_manifest = member / "Cargo.toml"
            member_manifest.write_text(
                '[package]\nname="fixture-member"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n', encoding="utf-8")
            (member / "src/lib.rs").write_text("pub fn member() {}\n", encoding="utf-8")
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run") as launch:
                self.assertEqual(runner.run_build("q-build", work["python"], member_manifest), 3)
            launch.assert_not_called()
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "path_dependency_hash_unknown")
            self.assertFalse(result["cargo_started"])
            self.assertIn("[patch.*]", result["path_dependency_hash_errors"][0])

    def test_unhashable_path_dependency_writes_fail_closed_receipt_before_slot(self):
        with workspace() as work:
            work["manifest"].write_text(
                '[package]\nname="fixture-pkg"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n'
                '[dependencies]\nmissing_dep={path="../missing-dep"}\n', encoding="utf-8")
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run") as launch:
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 3)
            launch.assert_not_called()
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "path_dependency_hash_unknown")
            self.assertFalse(result["cargo_started"])
            self.assertIn("has no Cargo.toml", result["path_dependency_hash_errors"][0])

    def test_manifest_name_and_cdylib_are_preflighted(self):
        with workspace() as work:
            wrong_name = work["crate"] / "Cargo.toml.copy"
            wrong_name.write_text(work["manifest"].read_text(encoding="utf-8"), encoding="utf-8")
            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])):
                with self.assertRaisesRegex(ValueError, "named Cargo.toml"):
                    runner.make_build_plan("q-build", work["python"], wrong_name)
                work["manifest"].write_text(
                    '[package]\nname="fixture-pkg"\nversion="0.1.0"\n'
                    '[lib]\ncrate-type=["rlib"]\n', encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "cdylib"):
                    runner.make_build_plan("q-build", work["python"], work["manifest"])

    def test_fixed_local_slot_argv_root_cwd_and_hash_verified_generation(self):
        with workspace() as work:
            captured = {}

            calls = []
            def fake_local_slot(argv, **kwargs):
                split = argv.index("--")
                cargo_args = argv[split + 1:]
                calls.append(cargo_args[1])
                if cargo_args[1] == "metadata":
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                captured["argv"] = list(argv)
                captured["kwargs"] = kwargs
                target_dir = Path(cargo_args[cargo_args.index("--target-dir") + 1])
                (target_dir / runner.BUILD_TARGET / "release").mkdir(parents=True)
                (target_dir / runner.BUILD_TARGET / "release/fixture_lib.dll").write_bytes(b"fixture DLL image")
                return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=fake_local_slot):
                self.assertEqual(runner.run_build(
                    "q-build", Path(".fixture-python/python.exe"),
                    Path(".artifacts/scratch/q-build/candidate/Cargo.toml")), 0)

            self.assertEqual(calls, ["metadata", "build"])
            argv = captured["argv"]
            args = argv[argv.index("--") + 1:]
            self.assertEqual(argv[:2], [str(work["python"].resolve()), str(work["slot"].resolve())])
            self.assertEqual(argv[argv.index("--cwd") + 1], str(work["root"]))
            self.assertEqual(captured["kwargs"]["cwd"], work["root"])
            self.assertIn("--manifest-path", args)
            self.assertEqual(Path(args[args.index("--manifest-path") + 1]), work["manifest"])
            target_dir = Path(args[args.index("--target-dir") + 1])
            self.assertEqual(target_dir.parent, work["root"] / ".artifacts/build/lanes/q-build")
            self.assertTrue(target_dir.name)
            self.assertEqual(args[args.index("--jobs") + 1], "1")
            self.assertEqual(args[args.index("--target") + 1], "i686-pc-windows-msvc")
            self.assertIn("--release", args)

            results = list((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            self.assertEqual(len(results), 1)
            result = json.loads(results[0].read_text(encoding="utf-8"))
            self.assertEqual(target_dir.name, result["build_id"])
            candidate = work["root"] / result["generation_dll"]
            self.assertEqual(candidate.parent.parent, work["lane_root"] / "generations")
            self.assertTrue(result["copy_hash_verified"])
            self.assertEqual(result["status"], "built_and_preserved")
            self.assertEqual(runner.sha256_file(candidate), result["dll_sha256"])
            self.assertIsNone(result["local_slot"]["outer_launch_pid"])

    def test_cargo_failure_propagates_without_preserving_output(self):
        with workspace() as work:
            calls = []
            def failed_local_slot(argv, **kwargs):
                subcommand = argv[argv.index("--") + 2]
                calls.append(subcommand)
                if subcommand == "metadata":
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                return runner.subprocess.CompletedProcess(argv, 7, stdout=success_transcript(7))

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=failed_local_slot):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 7)
            self.assertEqual(calls, ["metadata", "build"])
            self.assertFalse((work["lane_root"] / "generations").exists())
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "cargo_failed")
            self.assertTrue(result["cargo_started"])
            self.assertEqual(result["cargo_exit_code"], 7)

    def test_slot_failure_without_child_is_not_labeled_cargo_failure(self):
        with workspace() as work:
            transcript = "2026-10-08 q-build: waiting for local resource slot\n"
            calls = []
            def slot_failed(argv, **kwargs):
                subcommand = argv[argv.index("--") + 2]
                calls.append(subcommand)
                if subcommand == "metadata":
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                return runner.subprocess.CompletedProcess(argv, 9, stdout=transcript)

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=slot_failed):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 9)
            self.assertEqual(calls, ["metadata", "build"])
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "local_slot_failed_before_cargo")
            self.assertFalse(result["cargo_started"])
            self.assertIsNone(result["cargo_exit_code"])

    def test_metadata_lock_normalization_precedes_final_build_identity(self):
        with workspace() as work:
            ws = work["lane_root"] / "workspace"
            member = ws / "member"
            (member / "src").mkdir(parents=True)
            manifest = member / "Cargo.toml"
            manifest.write_text(
                '[package]\nname="fixture-member"\nversion="0.1.0"\nedition="2021"\n'
                '[lib]\nname="fixture_lib"\ncrate-type=["cdylib"]\n', encoding="utf-8")
            (member / "src/lib.rs").write_text("pub fn member() -> u32 { 1 }\n", encoding="utf-8")
            workspace_manifest = ws / "Cargo.toml"
            workspace_manifest.write_text('[workspace]\nmembers=["member"]\n', encoding="utf-8")
            lockfile = ws / "Cargo.lock"
            initial_lock = b'version = 4\n# pre-metadata\n'
            normalized_lock = b'version = 4\n# normalized by metadata\n'
            lockfile.write_bytes(initial_lock)
            calls = []

            def metadata_then_build(argv, **kwargs):
                args = argv[argv.index("--") + 1:]
                calls.append(args[1])
                if args[1] == "metadata":
                    lockfile.write_bytes(normalized_lock)
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                target_dir = Path(args[args.index("--target-dir") + 1])
                output = target_dir / runner.BUILD_TARGET / "release/fixture_lib.dll"
                output.parent.mkdir(parents=True)
                output.write_bytes(b"fixture DLL after lock normalization")
                return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=metadata_then_build):
                self.assertEqual(runner.run_build("q-build", work["python"], manifest), 0)

            self.assertEqual(calls, ["metadata", "build"])
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            plan = json.loads((work["root"] / result["build_plan"]).read_text(encoding="utf-8"))
            expected_lock_hash = runner.hashlib.sha256(normalized_lock).hexdigest()
            self.assertEqual(plan["candidate_source"]["workspace_locks"], [{
                "path": runner.evidence_path(lockfile), "present": True,
                "sha256": expected_lock_hash,
            }])
            preparation_path = work["root"] / result["lock_preparation"]["path"]
            preparation = json.loads(preparation_path.read_text(encoding="utf-8"))
            self.assertEqual(preparation["lock_before"]["sha256"], runner.hashlib.sha256(initial_lock).hexdigest())
            self.assertEqual(preparation["lock_after"]["sha256"], expected_lock_hash)
            self.assertEqual(result["status"], "built_and_preserved")

    def test_metadata_failure_stops_before_build(self):
        with workspace() as work:
            calls = []
            def metadata_fails(argv, **kwargs):
                subcommand = argv[argv.index("--") + 2]
                calls.append(subcommand)
                return runner.subprocess.CompletedProcess(argv, 7, stdout=success_transcript(7))

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=metadata_fails):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 7)
            self.assertEqual(calls, ["metadata"])
            self.assertFalse((work["lane_root"] / "build-runs").exists())
            record_path = next((work["lane_root"] / "build-lock-preparations").glob("*/lock_preparation.json"))
            record = json.loads(record_path.read_text(encoding="utf-8"))
            self.assertEqual(record["status"], "metadata_failed")

    def test_metadata_success_without_lock_stops_before_build(self):
        with workspace() as work:
            (work["crate"] / "Cargo.lock").unlink()
            calls = []
            def metadata_does_not_create_lock(argv, **kwargs):
                calls.append(argv[argv.index("--") + 2])
                return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=metadata_does_not_create_lock):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), runner.EXIT_EVIDENCE)
            self.assertEqual(calls, ["metadata"])
            self.assertFalse((work["lane_root"] / "build-runs").exists())
            record_path = next((work["lane_root"] / "build-lock-preparations").glob("*/lock_preparation.json"))
            record = json.loads(record_path.read_text(encoding="utf-8"))
            self.assertEqual(record["status"], "lock_missing_after_metadata")

    def test_source_drift_during_build_is_rejected_before_copy(self):
        with workspace() as work:
            def drift_during_build(argv, **kwargs):
                args = argv[argv.index("--") + 1:]
                if args[1] == "metadata":
                    return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())
                target_dir = Path(args[args.index("--target-dir") + 1])
                output = target_dir / runner.BUILD_TARGET / "release/fixture_lib.dll"
                output.parent.mkdir(parents=True)
                output.write_bytes(b"built from changing source")
                work["source"].write_text("pub fn fixture() -> u32 { 2 }\n", encoding="utf-8")
                return runner.subprocess.CompletedProcess(argv, 0, stdout=success_transcript())

            with patch.object(runner.shutil, "which", return_value=str(work["cargo"])), \
                    patch.object(runner.subprocess, "run", side_effect=drift_during_build):
                self.assertEqual(runner.run_build("q-build", work["python"], work["manifest"]), 3)
            result_path = next((work["lane_root"] / "build-runs").glob("*/build_result.json"))
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "candidate_source_changed")
            self.assertIsNone(result["generation_dll"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
