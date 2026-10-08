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
        with patch.object(server, "result_document", return_value={}), \
                patch.object(server, "result_rows", return_value=rows):
            self.assertEqual(server.yield_of("a-G03"),
                             {"done": 2, "of": 3, "unit": "checked", "display": "2 of 3 checked"})

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


    def test_task_activity_is_bounded_and_never_counts_as_proof(self):
        data = {
            "status": "in_progress",
            "preliminary_findings": [
                {"id": f"finding-{index}", "severity": "P2",
                 "evidence": "<script>" + ("x" * 500), "effect": "review only"}
                for index in range(15)
            ],
            "recommendation": "<img src=x onerror=alert(1)>",
        }
        evidence = server.structured_result_evidence(data)
        self.assertEqual(evidence["kind"], "activity")
        self.assertEqual(evidence["done"], 0)
        self.assertEqual(evidence["of"], 0)
        self.assertEqual(evidence["tests_count"], 0)
        self.assertEqual(evidence["changes_count"], 0)
        self.assertEqual(evidence.get("verified_functions", 0), 0)
        self.assertEqual(evidence.get("partial_stages", 0), 0)
        self.assertEqual(len([row for row in evidence["rows"] if row["category"] == "Finding"]), 13)
        self.assertTrue(any(row["category"] == "Recommendation" for row in evidence["rows"]))
        self.assertTrue(all(len(row["name"]) <= 120 and len(row["detail"]) <= 260
                            for row in evidence["rows"]))
        self.assertIn("<script>", evidence["rows"][0]["detail"])
        recommendation = next(row for row in evidence["rows"]
                              if row["category"] == "Recommendation")
        self.assertIn("<img", recommendation["detail"])

        prior_findings = {"candidate_source_tree_sha256": "abc123",
                          "positive_original_ok": "1000/1000"}
        prior_record = {"status": "complete", "preliminary_findings": prior_findings,
                        "recommendation": "bounded acceptance for the exact recorded target"}
        prior = server.structured_result_evidence(prior_record)
        self.assertEqual(prior["kind"], "activity")
        self.assertIn("candidate_source_tree_sha256", prior["rows"][0]["detail"])
        self.assertEqual(prior["done"], 0)
        self.assertEqual(prior["of"], 0)
        self.assertEqual(prior_record["preliminary_findings"], prior_findings)
        html = (SERVER.parent / "index.html").read_text(encoding="utf-8")
        for field in ('esc(r.category || "")', 'esc(r.name || "")',
                      'esc(r.outcome || "")', 'esc(r.detail || "")'):
            self.assertIn(field, html)


    def test_statusless_task_notes_preserve_existing_row_proof_counts(self):
        data = {
            "preliminary_findings": {"candidate_source_tree_sha256": "abc123"},
            "recommendation": "Review-only note with <markup>.",
            "results": [
                {"name": "fn_verified", "outcome": "verified"},
                {"name": "fn_partial", "outcome": "partial_stage"},
                {"name": "fn_not_reached", "outcome": "not_reached"},
            ],
        }
        proof_rows = data["results"]
        original_data = json.loads(json.dumps(data))
        self.assertNotIn("status", data)
        self.assertIsNone(server.structured_result_evidence(data))
        with patch.object(server, "result_document", return_value=data), \
                patch.object(server, "result_rows", return_value=proof_rows):
            progress = server.yield_of("a-R01")
            detail_rows, label, item_label, result_kind = server.result_detail_rows("a-R01")

        self.assertEqual((progress["done"], progress["of"], progress["unit"]), (1, 3, "verified"))
        self.assertEqual(progress["verified_functions"], 1)
        self.assertEqual(progress["partial_functions"], 1)
        self.assertEqual(label, "Function results and task activity")
        self.assertEqual(item_label, "Result / note")
        self.assertEqual(result_kind, "legacy")
        self.assertEqual([row["name"] for row in detail_rows[:3]],
                         ["fn_verified", "fn_partial", "fn_not_reached"])
        self.assertEqual([row["outcome"] for row in detail_rows[3:]], ["recorded", "recorded"])
        self.assertTrue(any("candidate_source_tree_sha256" in row["detail"] for row in detail_rows[3:]))
        self.assertTrue(any("<markup>" in row["detail"] for row in detail_rows[3:]))
        self.assertEqual(data, original_data)
        self.assertEqual(data["results"], proof_rows)
        self.assertNotIn("status", data)

    def test_recent_finished_view_requires_a_known_finite_past_timestamp(self):
        now = 10000.0
        cutoff = now - server.RECENT_SECONDS
        self.assertTrue(server.ended_recently("finished", cutoff, now))
        self.assertTrue(server.ended_recently("finished", now, now))
        self.assertFalse(server.ended_recently("finished", cutoff - 0.01, now))
        self.assertFalse(server.ended_recently("finished", now + 0.01, now))
        self.assertFalse(server.ended_recently("finished", None, now))
        self.assertFalse(server.ended_recently("finished", "unknown", now))
        self.assertFalse(server.ended_recently("finished", float("nan"), now))
        self.assertFalse(server.ended_recently("finished", float("inf"), now))
        self.assertFalse(server.ended_recently("running", now - 1, now))
        self.assertFalse(server.ended_recently("unknown", now - 1, now))

    def test_ended_annotation_keeps_running_and_unknown_rows(self):
        now = 10000.0
        lanes = [
            {"lane": "running-old", "state": "running", "last_activity_epoch": now - 50000},
            {"lane": "finished-recent", "state": "finished", "last_activity_epoch": now - 5},
            {"lane": "finished-unknown", "state": "finished", "last_activity_epoch": None},
            {"lane": "unknown-recent", "state": "unknown", "last_activity_epoch": now - 5},
        ]
        annotated = server.annotate_ended_recent(lanes, now)
        self.assertEqual(len(annotated), 4)
        self.assertEqual([row["lane"] for row in annotated], ["running-old", "finished-recent",
                                                              "finished-unknown", "unknown-recent"])
        self.assertEqual([row["ended_recent"] for row in annotated], [False, True, False, False])
        html = (SERVER.parent / "index.html").read_text(encoding="utf-8")
        self.assertIn('if (filter === "running" && l.state !== "running") return false;', html)
        self.assertIn('if (filter === "ended" && l.ended_recent !== true) return false;', html)


if __name__ == "__main__":
    unittest.main()
