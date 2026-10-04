"""Tests for make_brief.py: argument parsing and the written brief."""

import contextlib
import io
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import make_brief


class TestParseArgs(unittest.TestCase):
    def test_plain(self):
        self.assertEqual(make_brief.parse_args(["x-1", "task.txt"]), ("x-1", "task.txt", None))

    def test_date_with_space_after(self):
        self.assertEqual(make_brief.parse_args(["x-1", "task.txt", "--date", "2026-10-04"]), ("x-1", "task.txt", "2026-10-04"))

    def test_date_with_space_before_positionals(self):
        # The case that used to take the date for the lane name.
        self.assertEqual(make_brief.parse_args(["--date", "2026-10-04", "x-1", "task.txt"]), ("x-1", "task.txt", "2026-10-04"))

    def test_date_with_equals(self):
        self.assertEqual(make_brief.parse_args(["--date=2026-10-04", "x-1", "task.txt"]), ("x-1", "task.txt", "2026-10-04"))

    def test_missing_value_and_wrong_count(self):
        with self.assertRaises(SystemExit):
            make_brief.parse_args(["x-1", "task.txt", "--date"])
        with self.assertRaises(SystemExit):
            make_brief.parse_args(["x-1"])
        with self.assertRaises(SystemExit):
            make_brief.parse_args(["x-1", "task.txt", "extra"])


class TestMain(unittest.TestCase):
    def test_writes_brief_with_date(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            task = tmp / "task.txt"
            task.write_text("Your task:\n\nlane {lane} on {date}\n", encoding="utf-8")
            env = {"LIBERTYFLUX_ROOT": str(tmp), "LIBERTYFLUX_BRIEFS_DIR": str(tmp / "briefs")}
            with mock.patch.dict(os.environ, env), mock.patch.object(sys, "argv", ["make_brief.py", "--date", "2026-01-02", "x-7", str(task)]), \
                    contextlib.redirect_stdout(io.StringIO()):
                make_brief.main()
            text = (tmp / "briefs" / "x-7.txt").read_text(encoding="utf-8")
        self.assertTrue(text.endswith("Your task:\n\nlane x-7 on 2026-01-02\n"))
        self.assertIn("2026-01-02", text.split("Your task:")[0])


if __name__ == "__main__":
    unittest.main()
