"""Tests for briefs.py: templates render completely from data, no slicing of other briefs."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import briefs

ROOT = "C:\\Repo"
LANE = "r-s99"


class TestNotes(unittest.TestCase):
    def test_version_and_order(self):
        notes = briefs.load_notes()
        self.assertRegex(notes["version"], r"^b\d+$")  # the version moves with every change to the notes
        self.assertEqual([n["id"] for n in notes["notes"]], list(range(1, len(notes["notes"]) + 1)))
        for note in notes["notes"]:
            for key in ("id", "title", "since", "text"):
                self.assertIn(key, note)
            self.assertTrue(note["text"].strip())

    def test_version_note_present(self):
        notes = briefs.load_notes()["notes"]
        current = [n for n in notes if f"brief version {briefs.brief_version()}" in n["text"]]
        self.assertEqual(len(current), 1)


class TestRendering(unittest.TestCase):
    def test_preamble(self):
        text = briefs.render_preamble(LANE, ROOT, "2026-10-04")
        self.assertIn(f"You are lane {LANE}", text)
        self.assertIn(ROOT, text)
        self.assertIn("2026-10-04", text)
        self.assertTrue(text.endswith("Your task:\n\n"))
        for token in ("{lane}", "{root}", "{date}"):
            self.assertNotIn(token, text)

    def test_head_stops_before_task_line(self):
        head = briefs.render_head(LANE, ROOT, "2026-10-04")
        self.assertNotIn("Your task:", head)
        self.assertEqual(head + "Your task:\n\n", briefs.render_preamble(LANE, ROOT, "2026-10-04"))

    def test_production_renders_fully(self):
        task = briefs.render_production(LANE, "lf_rs99_rw", "WHAT-GOES-HERE", "s99", ROOT)
        self.assertIn("WHAT-GOES-HERE", task)
        self.assertIn("lf_rs99_rw", task)
        self.assertIn(f"`brief` (write `{briefs.brief_version()}`)", task)
        self.assertIn("What earlier production lanes learned", task)
        for token in ("{root}", "{lane}", "{crate}", "{what}", "{slug}", "{brief_version}"):
            self.assertNotIn(token, text := task)
        self.assertNotIn(ROOT + "{", task)

    def test_learned_has_every_note(self):
        learned = briefs.render_learned(ROOT)
        notes = briefs.load_notes()["notes"]
        self.assertEqual(learned.count("\n- "), len(notes))
        for note in notes:
            self.assertIn(note["text"].split("\n")[0][:60], learned)

    def test_naming_renders_fully(self):
        task = briefs.render_naming(150, "C:\\Repo\\lists\\nm-01.json", ROOT, "01")
        self.assertIn("the 150 game functions", task)
        self.assertIn(ROOT, task)
        for token in ("{count}", "{list_path}", "{root}", "{slug}"):
            self.assertNotIn(token, task)

    def test_templates_carry_their_placeholders(self):
        self.assertIn("{brief_version}", briefs.template("production.txt"))
        self.assertIn("{date}", briefs.template("preamble.txt"))


class TestCrossChecks(unittest.TestCase):
    """The brief, the result schema and the scripts that read the brief must agree."""

    def test_brief_asks_for_every_field_a_pass_needs(self):
        sys.path.insert(0, str(Path(briefs.__file__).parent))
        import lane_results
        task = briefs.render_production(LANE, "lf_rs99_rw", "W", "s99", ROOT)
        results_line = next(line for line in task.splitlines() if line.startswith("- `results.json`"))
        schema = lane_results.load_schema()
        required = set(schema["required"])
        for rule in schema["allOf"]:
            required |= set(rule["then"].get("required", []))
        for field in sorted(required):
            self.assertIn(f"`{field}`", results_line, field)

    def test_dashboard_reads_the_brief_tag(self):
        import re
        server = (Path(briefs.__file__).parents[1] / "dashboard" / "server.py").read_text(encoding="utf-8")
        pattern = re.search(r're\.search\(r"(`brief` [^"]+)", text\)', server).group(1)
        task = briefs.render_production(LANE, "lf_rs99_rw", "W", "s99", ROOT)
        self.assertEqual(re.search(pattern, task).group(1), briefs.brief_version())

    def test_batch_templates_take_what_the_maker_passes(self):
        small = briefs.template("small_what.txt").format(count=40, list_path="L", subsystem="audio")
        big = briefs.template("big_what.txt").format(count=3, list_path="L", subsystem="audio", smallest=1, largest=2, total=3)
        native = briefs.template("native_what.txt").format(count=25, list_path="L")
        for text in (small, big, native):
            self.assertNotRegex(text, r"\{[a-z_]+\}")


if __name__ == "__main__":
    unittest.main()
