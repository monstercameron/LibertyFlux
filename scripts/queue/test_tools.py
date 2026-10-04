"""Unit tests for the queue tools around lfdb: the PE reader and map loaders in context.py, the helpers in
queue_build.py, the deterministic export, the progress shape in stats.py and the sweep ledger.

Run from this folder (python -m unittest test_tools) or from the repository root
(python -m unittest scripts/queue/test_tools.py). Synthetic inputs only; no game file is read.
"""

from __future__ import annotations

import contextlib
import io
import json
import struct
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import context
import export
import lfdb
import queue_build
import stats
import sweep
from test_queue import sample_records

EXEC = 0x20000000


def tiny_pe(sections):
    """A minimal PE image: DOS stub, signature, COFF header and section table, then raw section data.
    `sections` is [(name, vaddr, vsize, raw bytes, flags)]."""
    header_end = 64 + 4 + 20 + 40 * len(sections)
    rawptr = (header_end + 0x1FF) & ~0x1FF
    table, body = b"", b""
    for name, vaddr, vsize, raw, flags in sections:
        table += struct.pack("<8sIIIIIIHHI", name.encode(), vsize, vaddr, len(raw), rawptr + len(body), 0, 0, 0, 0, flags)
        body += raw
    dos = b"MZ" + b"\0" * 58 + struct.pack("<I", 64)
    coff = struct.pack("<HHIIIHH", 0x14C, len(sections), 0, 0, 0, 0, 0)
    image = dos + b"PE\0\0" + coff + table
    return image + b"\0" * (rawptr - len(image)) + body


class PeReaderTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.exe = Path(self.tmp.name) / "tiny.exe"
        self.exe.write_bytes(tiny_pe([(".text", 0x1000, 0x400, bytes(range(256)), EXEC),
                                      (".data", 0x2000, 0x100, b"\x11" * 16, 0)]))
        self.sections = context.pe_sections(str(self.exe))

    def tearDown(self):
        self.tmp.cleanup()

    def test_sections(self):
        self.assertEqual([(s["name"], s["vaddr"], s["exec"]) for s in self.sections],
                         [(".text", 0x1000, True), (".data", 0x2000, False)])

    def test_rva_mapping(self):
        text = self.sections[0]
        self.assertEqual(context.rva_to_offset(self.sections, 0x1010), text["rawptr"] + 0x10)
        self.assertIsNone(context.rva_to_offset(self.sections, 0x1200))  # inside vsize, past the raw bytes
        self.assertIsNone(context.rva_to_offset(self.sections, 0x5000))
        self.assertEqual(context.section_of(self.sections, 0x2004), ".data")
        self.assertEqual(context.section_of(self.sections, 0x1300), ".text")  # by virtual size
        self.assertIsNone(context.section_of(self.sections, 0x9000))

    def test_read_bytes(self):
        self.assertEqual(context.read_bytes(str(self.exe), self.sections, 0x1004, 4), bytes([4, 5, 6, 7]))
        with self.assertRaises(ValueError):
            context.read_bytes(str(self.exe), self.sections, 0x9000, 4)

    def test_not_a_pe(self):
        bad = Path(self.tmp.name) / "bad.exe"
        bad.write_bytes(b"ZM" + b"\0" * 100)
        with self.assertRaises(ValueError):
            context.pe_sections(str(bad))


class MapLoaderTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)

    def tearDown(self):
        self.tmp.cleanup()

    def test_iat_map(self):
        path = self.dir / "imports.json"
        path.write_text(json.dumps([{"iat": "0x1000", "dll": "k.dll", "function": "F"}, {"iat": None}, {"iat": "zz"}]))
        self.assertEqual(context.load_iat_map(str(path)), {0x1000: "k.dll!F"})
        self.assertEqual(context.load_iat_map(None), {})

    def test_str_map_va_rva_and_truncation(self):
        path = self.dir / "strings.json"
        path.write_text(json.dumps([
            {"string_address": "0x00401000", "string_text": "va", "suggested_name": "n", "confidence": "high"},
            {"string_address": "0x2000", "string_text": "x" * 130},
            {"string_address": "zz"}, {"string_text": "no address"}]))
        got = context.load_str_map(str(path))
        self.assertEqual(got[0x1000], "va [n/high]")
        self.assertTrue(got[0x2000].startswith("x" * 120 + "..."))
        self.assertEqual(len(got), 2)


class QueueBuildHelpersTest(unittest.TestCase):
    def test_load_ranges_converts_vas(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "libraries.json"
            path.write_text(json.dumps({"libraries": [
                {"name": "zlib", "code_ranges": [{"from": "0x00500000", "to": "0x00501000"}, {"from": "0x10"}]},
                {"name": "empty", "code_ranges": None}]}))
            self.assertEqual(queue_build.load_ranges(str(path)), [(0x100000, 0x101000, "zlib")])
        self.assertEqual(queue_build.load_ranges(None), [])

    def test_looks_like_runtime(self):
        for name in ("__security_check_cookie", "_RTC_CheckEsp", "_ftol2", "??_7Foo@@6B@", "@_EH4_Local"):
            self.assertTrue(queue_build.looks_like_runtime(name), name)
        for name in (None, "", "CPed::Process", "audio_tick"):
            self.assertFalse(queue_build.looks_like_runtime(name), name)


class DbToolsTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.db = str(self.dir / "q.db")
        conn = lfdb.connect(self.db)
        lfdb.import_functions(conn, sample_records(12), "est")
        conn.close()

    def tearDown(self):
        self.tmp.cleanup()

    def quiet(self, func, argv):
        with contextlib.redirect_stdout(io.StringIO()):
            return func(argv)

    def test_export_is_deterministic(self):
        first, second = self.dir / "a", self.dir / "b"
        self.assertEqual(self.quiet(export.main, ["--db", self.db, "--out", str(first)]), 0)
        self.quiet(export.main, ["--db", self.db, "--out", str(second)])
        for name in ("functions.json", "batches.json", "deferral_reasons.json"):
            self.assertEqual((first / name).read_bytes(), (second / name).read_bytes(), name)
        functions = json.loads((first / "functions.json").read_text())
        self.assertEqual(len(functions), 12)
        addresses = [int(f["address"], 16) for f in functions]
        self.assertEqual(addresses, sorted(addresses))
        self.assertNotIn(b"\r\n", (first / "functions.json").read_bytes())

    def test_progress_shape_adds_up(self):
        conn = lfdb.connect(self.db)
        try:
            shape = stats.progress_shape(conn)
        finally:
            conn.close()
        f, s = shape["functions"], shape["stages"]
        self.assertEqual(f["total"], 12)
        self.assertEqual(f["game"], f["total"] - f["library"])
        self.assertEqual(s["identified"], f["game"])
        self.assertLessEqual(s["verified"], s["rewritten"])
        self.assertLessEqual(s["named"], s["identified"])

    def test_sweep_records_a_batch_and_ledger_line(self):
        self.assertEqual(self.quiet(sweep.main, ["--db", self.db, "--note", "test"]), 0)
        lines = (self.dir / "ledger.jsonl").read_text().splitlines()
        record = json.loads(lines[-1])
        self.assertEqual((record["note"], record["released_stale"]), ("test", 0))
        self.assertEqual(sum(record["counts"].values()), 12)


if __name__ == "__main__":
    unittest.main()
