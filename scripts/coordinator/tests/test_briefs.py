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
        self.assertEqual(notes["version"], "b6")
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


if __name__ == "__main__":
    unittest.main()
