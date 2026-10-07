import importlib.util
import json
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path

STATUS = Path(__file__).resolve().parents[2] / "status.py"
spec = importlib.util.spec_from_file_location("lf_status_codex_test", STATUS)
status = importlib.util.module_from_spec(spec)
spec.loader.exec_module(status)
REPO_ROOT = next((path for path in Path(__file__).resolve().parents if (path / "docs" / "data" / "progress.json").is_file()), Path(__file__).resolve().parents[4])
SCRATCH_ROOT = REPO_ROOT / ".artifacts" / "scratch" / "luna-dashboard"
SCRATCH_ROOT.mkdir(parents=True, exist_ok=True)


class TestStatusCodexRegistry(unittest.TestCase):
    def write_registry(self, root, updated_at, lanes):
        path = root / "codex_lanes.json"
        path.write_text(json.dumps({"updated_at": updated_at, "lanes": lanes}), encoding="utf-8")
        return path

    def test_fresh_running_codex_lane_counts_without_a_muse_process(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            path = self.write_registry(Path(tmp), updated.isoformat(), [{"lane": "a-G03", "state": "running"}])
            lanes, _ = status.read_codex_registry(path, now=updated.timestamp() + 20)
        self.assertEqual(status.classify_lane("a-G03", lanes, set(), False, False, False), "running")

    def test_stale_running_lane_is_unknown_even_with_a_live_muse_match(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            path = self.write_registry(Path(tmp), updated.isoformat(), [{"lane": "a-G03", "state": "running"}])
            lanes, registry = status.read_codex_registry(path, now=updated.timestamp() + 181)
        self.assertEqual(registry["state"], "stale")
        self.assertEqual(status.classify_lane("a-G03", lanes, {"a-G03"}, False, True, True), "unknown")

    def test_finished_registry_state_is_preserved(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            path = self.write_registry(Path(tmp), updated.isoformat(), [{"lane": "a-G03", "state": "finished"}])
            lanes, _ = status.read_codex_registry(path, now=updated.timestamp() + 20)
        self.assertEqual(status.classify_lane("a-G03", lanes, {"a-G03"}, False, False, False), "finished")

    def test_invalid_registry_does_not_supply_a_running_lane(self):
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            root = Path(tmp)
            path = root / "invalid.json"
            path.write_text("not json", encoding="utf-8")
            lanes, registry = status.read_codex_registry(path, now=0)
            self.assertEqual(registry["state"], "unknown")
            self.assertEqual(lanes, {})
            updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
            path = self.write_registry(root, updated.isoformat(), [{"lane": "a-G03", "state": {"bad": "running"}}])
            lanes, registry = status.read_codex_registry(path, now=updated.timestamp() + 20)
        self.assertEqual(registry["state"], "fresh")
        self.assertEqual(lanes["a-G03"]["state"], "unknown")
        self.assertEqual(status.classify_lane("a-G03", lanes, set(), False, True, True), "unknown")


if __name__ == "__main__":
    unittest.main()