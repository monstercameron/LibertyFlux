from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from unittest.mock import patch
from datetime import datetime, timezone
from pathlib import Path

SERVER = Path(__file__).resolve().parents[2] / "dashboard" / "server.py"
spec = importlib.util.spec_from_file_location("lf_dashboard_codex_test", SERVER)
server = importlib.util.module_from_spec(spec)
spec.loader.exec_module(server)
REPO_ROOT = next((path for path in Path(__file__).resolve().parents if (path / "docs" / "data" / "progress.json").is_file()), Path(__file__).resolve().parents[4])
SCRATCH_ROOT = REPO_ROOT / ".artifacts" / "scratch" / "luna-dashboard"
SCRATCH_ROOT.mkdir(parents=True, exist_ok=True)


class TestCodexRegistry(unittest.TestCase):
    def test_regression_progress_does_not_claim_positive_replays_as_verified(self):
        rows = [{"outcome": "deferred", "settled": "holds", "mutant_caught": None},
                {"outcome": "deferred", "settled": "fails"},
                {"outcome": "not_reached"}]
        with patch.object(server, "result_rows", return_value=rows):
            self.assertEqual(server.yield_of("a-G03"), {"done": 2, "of": 3, "unit": "checked"})

    def write_registry(self, root, updated_at, lanes):
        path = root / "codex_lanes.json"
        path.write_text(json.dumps({"updated_at": updated_at, "lanes": lanes}), encoding="utf-8")
        return path

    def test_fresh_finished_state_is_explicit(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            path = self.write_registry(Path(tmp), updated.isoformat(), [
                {"lane": "a-G03", "state": "finished", "model": "gpt-6-luna", "effort": "max"}
            ])
            lanes, status = server.read_codex_registry(path, now=updated.timestamp() + 30)
        self.assertTrue(status["fresh"])
        self.assertEqual(lanes["a-G03"]["state"], "finished")
        self.assertEqual((lanes["a-G03"]["model"], lanes["a-G03"]["effort"]), ("gpt-6-luna", "max"))

    def test_stale_running_registry_becomes_unknown(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            path = self.write_registry(Path(tmp), updated.isoformat(), [
                {"lane": "a-G03", "state": "running", "model": "gpt-6-luna"}
            ])
            lanes, status = server.read_codex_registry(path, now=updated.timestamp() + 181)
        self.assertFalse(status["fresh"])
        self.assertEqual(status["state"], "stale")
        self.assertEqual(lanes["a-G03"]["state"], "unknown")

    def test_invalid_json_and_timestamp_never_return_a_live_lane(self):
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            root = Path(tmp)
            malformed = root / "bad.json"
            malformed.write_text("{", encoding="utf-8")
            lanes, status = server.read_codex_registry(malformed, now=0)
            self.assertEqual(lanes, {})
            self.assertEqual(status["state"], "unknown")
            path = self.write_registry(root, "yesterday-ish", [{"lane": "a-G03", "state": "running"}])
            lanes, status = server.read_codex_registry(path, now=0)
            self.assertEqual(lanes, {})
            self.assertEqual(status["state"], "unknown")
            updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
            path = self.write_registry(root, updated.isoformat(), [{"lane": "a-G03", "state": ["running"]}])
            lanes, status = server.read_codex_registry(path, now=updated.timestamp() + 20)
        self.assertEqual(lanes["a-G03"]["state"], "unknown")
        self.assertTrue(status["fresh"])

    def test_history_merges_only_fresh_running_registry_lanes(self):
        muse = {"muse-lane": {"mem_mb": 100}}
        codex = {"a-G03": {"state": "running", "source": "codex"},
                 "a-G04": {"state": "unknown", "source": "codex"}}
        active = server.history_lanes(muse, codex, registry_fresh=True)
        self.assertEqual(set(active), {"muse-lane", "a-G03"})
        stale = server.history_lanes(muse, codex, registry_fresh=False)
        self.assertEqual(set(stale), {"muse-lane"})
        row = server.history_sample(1000, active, {}, None)
        self.assertEqual(row["lanes"], 2)
        self.assertIsNone(row["agent_gb"])

    def test_stale_registry_and_summary_do_not_create_a_live_lane(self):
        updated = datetime(2026, 10, 7, 12, 0, tzinfo=timezone.utc)
        with tempfile.TemporaryDirectory(dir=SCRATCH_ROOT) as tmp:
            root = Path(tmp)
            path = self.write_registry(root, updated.isoformat(), [
                {"lane": "a-G03", "state": "running", "model": "gpt-6-luna", "effort": "max"}
            ])
            lanes, _ = server.read_codex_registry(path, now=updated.timestamp() + 181)
            (root / "a-G03").mkdir()
            (root / "a-G03" / "summary.txt").write_text("finished summary", encoding="utf-8")
            old = server.SCRATCH, server.LOGS, server.LISTS, server.BRIEFS
            server.SCRATCH, server.LOGS, server.LISTS, server.BRIEFS = root, root, root, root
            try:
                row = server.lane_row("a-G03", {"a-G03": {"pid": 123}}, {}, lanes["a-G03"])
            finally:
                server.SCRATCH, server.LOGS, server.LISTS, server.BRIEFS = old
        self.assertEqual(row["state"], "unknown")
        self.assertIsNone(row["agent_memory_mb"])
        self.assertEqual(row["source"], "codex")


if __name__ == "__main__":
    unittest.main()
