"""Tests for common.py: addresses, result shapes, names, the batch lock, paths."""

import json
import os
import re
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import common


class TestVa(unittest.TestCase):
    def test_hex_string(self):
        self.assertEqual(common.va("0x10001000"), 0x10001000)

    def test_hex_without_prefix(self):
        self.assertEqual(common.va("10001000"), 0x10001000)

    def test_int_passes_through(self):
        self.assertEqual(common.va(0x1234), 0x1234)

    def test_uppercase(self):
        self.assertEqual(common.va("0x1000ABCD"), 0x1000ABCD)


class TestLoadRows(unittest.TestCase):
    def test_bare_list(self):
        rows = [{"address": "0x1"}]
        self.assertEqual(common.load_rows(rows, "functions", "results"), rows)

    def test_functions_key(self):
        rows = [{"address": "0x1"}]
        self.assertEqual(common.load_rows({"functions": rows}, "functions", "results"), rows)

    def test_results_key(self):
        rows = [{"address": "0x1"}]
        self.assertEqual(common.load_rows({"results": rows}, "functions", "results"), rows)

    def test_names_key(self):
        rows = [{"address": "0x1"}]
        self.assertEqual(common.load_rows({"names": rows}, "names"), rows)

    def test_dict_without_known_keys(self):
        self.assertEqual(common.load_rows({"something": 1}, "functions", "results"), [])

    def test_neither_list_nor_dict(self):
        self.assertEqual(common.load_rows(42, "functions", "results"), [])


class TestReadResultsRows(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)

    def tearDown(self):
        self.tmp.cleanup()

    def write(self, name, obj):
        path = self.dir / name
        path.write_text(json.dumps(obj), encoding="utf-8")
        return path

    def test_each_shape(self):
        rows = [{"address": "0x10001000", "outcome": "verified"}]
        for obj in (rows, {"functions": rows}, {"results": rows}):
            with self.subTest(obj=obj):
                self.assertEqual(common.read_results_rows(self.write("r.json", obj)), rows)

    def test_non_dict_rows_skipped(self):
        self.assertEqual(common.read_results_rows(self.write("r.json", ["junk", 42, {"address": "0x1"}])),
                         [{"address": "0x1"}])

    def test_unreadable_is_empty(self):
        bad = self.dir / "bad.json"
        bad.write_text("{not json", encoding="utf-8")
        self.assertEqual(common.read_results_rows(bad), [])

    def test_missing_is_empty(self):
        self.assertEqual(common.read_results_rows(self.dir / "nope.json"), [])


class TestPlaceholderNames(unittest.TestCase):
    def test_placeholders(self):
        for label in ["Class::vf12", "Foo::vfunc3", "fn_00401000", "sub_401000", "FUN_00401000",
                      "unnamed", "unknown_helper", "", "helper_a1b2c3d4", "foo_static_init12"]:
            with self.subTest(label=label):
                self.assertTrue(common.is_placeholder_name(label), label)

    def test_meaningful(self):
        for label in ["audio_voice_tick", "pool_append_converted_string", "CTaskSimpleDie::make_abortable",
                      "strcmp", "malloc", "render_phase_submit"]:
            with self.subTest(label=label):
                self.assertFalse(common.is_placeholder_name(label), label)


class TestBatchLock(unittest.TestCase):
    @unittest.skipUnless(sys.platform == "win32", "the batch lock uses msvcrt")
    def test_exclusive(self):
        with tempfile.TemporaryDirectory() as tmp:
            with common.batch_lock(tmp):
                with self.assertRaises(SystemExit):
                    with common.batch_lock(tmp, tries=2, wait=0.05):
                        pass

    @unittest.skipUnless(sys.platform == "win32", "the batch lock uses msvcrt")
    def test_released_after_use(self):
        with tempfile.TemporaryDirectory() as tmp:
            with common.batch_lock(tmp):
                pass
            with common.batch_lock(tmp, tries=2, wait=0.05):
                pass


class TestPaths(unittest.TestCase):
    def test_root_override(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.environ["LIBERTYFLUX_ROOT"] = tmp
            try:
                self.assertEqual(common.find_root(), Path(tmp))
            finally:
                del os.environ["LIBERTYFLUX_ROOT"]

    def test_root_is_a_directory(self):
        self.assertTrue(common.find_root().is_dir())

    def test_coord_layout(self):
        root = Path("/repo")
        self.assertEqual(common.scratch_dir(root), root / ".artifacts" / "scratch")
        self.assertEqual(common.coord_dir(root), root / ".artifacts" / "scratch" / "coordinator")
        self.assertEqual(common.lists_dir(root), root / ".artifacts" / "scratch" / "coordinator" / "lists")

    def test_lists_dir_override(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.environ["LIBERTYFLUX_LISTS_DIR"] = tmp
            try:
                self.assertEqual(common.lists_dir(), Path(tmp))
            finally:
                del os.environ["LIBERTYFLUX_LISTS_DIR"]

    def test_orig_exe(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.environ["LIBERTYFLUX_ORIG_EXE"] = str(Path(tmp) / "game.exe")
            try:
                self.assertEqual(common.orig_exe(), Path(tmp) / "game.exe")
            finally:
                del os.environ["LIBERTYFLUX_ORIG_EXE"]
        self.assertEqual(common.orig_exe(Path("/repo")), Path("/repo/orig/GTAIV.exe"))

    def test_game_dir_unset_is_none(self):
        os.environ.pop("LIBERTYFLUX_GAME_DIR", None)
        self.assertIsNone(common.game_dir())

    def test_clock_formats(self):
        self.assertRegex(common.now_stamp(), r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$")
        self.assertRegex(common.today(), r"^\d{4}-\d{2}-\d{2}$")


if __name__ == "__main__":
    unittest.main()
