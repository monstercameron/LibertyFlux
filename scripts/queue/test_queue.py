"""Unit tests for the work-queue tools. Run from this folder:

    python -m unittest test_queue -v

Uses throwaway databases in a temp dir; the real queue database is never
touched. Tests that need the game binary or capstone are skipped when they
are unavailable (the --no-disasm path is always tested).
"""

from __future__ import annotations

import json
import sqlite3
import tempfile
import threading
import unittest
from pathlib import Path

import lfdb

TOOLS = Path(__file__).resolve().parent


def sample_records(n=30):
    recs = []
    for i in range(n):
        recs.append({
            "rva": 0x100000 + i * 0x40,
            "size_bytes": 16 + (i % 7) * 32,
            "n_insns": 5 + (i % 11) * 7,
            "branches": i % 4,
            "calls_direct": i % 3,
            "calls_indirect": i % 2,
            "switches": 1 if i % 9 == 0 else 0,
            "fp_insns": i % 3,
            "sha1_masked": None,
            "name": None,
            "subsystem": "net" if i % 2 == 0 else None,
            "kind_hint": None,
            "library": False,
            "thunk": False,
            "extra": {},
        })
    return recs


class DbTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.db = str(Path(self.tmp.name) / "q.db")
        self.conn = lfdb.connect(self.db)

    def tearDown(self):
        self.conn.close()
        self.tmp.cleanup()


class SchemaTest(DbTest):
    def test_version_and_tables(self):
        self.assertEqual(lfdb.db_version(self.conn), 1)
        tables = {r["name"] for r in self.conn.execute(
            "SELECT name FROM sqlite_master WHERE type='table'")}
        for t in ("functions", "names", "evidence", "classes", "vtable_slots",
                  "claims", "attempts", "verdicts", "deferrals", "batches",
                  "schema_version"):
            self.assertIn(t, tables)

    def test_wal_and_timeout(self):
        mode = self.conn.execute("PRAGMA journal_mode").fetchone()[0]
        self.assertEqual(mode.lower(), "wal")
        to = self.conn.execute("PRAGMA busy_timeout").fetchone()[0]
        self.assertGreaterEqual(to, 30000)

    def test_init_idempotent(self):
        self.assertEqual(lfdb.init_db(self.conn), 1)


class ImportTest(DbTest):
    def test_import_and_idempotent_reimport(self):
        r1 = lfdb.import_functions(self.conn, sample_records(10), "est")
        self.assertEqual(r1, {"inserted": 10, "updated": 0})
        r2 = lfdb.import_functions(self.conn, sample_records(10), "rich")
        self.assertEqual(r2, {"inserted": 0, "updated": 10})

    def test_reimport_keeps_state_and_name(self):
        lfdb.import_functions(self.conn, sample_records(5), "est")
        addr = 0x100000
        lfdb.set_name(self.conn, addr, "Player_Update", "rtti", "high")
        self.conn.execute("BEGIN IMMEDIATE")
        self.conn.execute(
            "UPDATE functions SET state='accepted' WHERE addr=?", (addr,))
        self.conn.execute("COMMIT")
        lfdb.import_functions(self.conn, sample_records(5), "rich")
        row = self.conn.execute("SELECT state, name FROM functions WHERE addr=?",
                                (addr,)).fetchone()
        self.assertEqual(row["state"], "accepted")
        self.assertEqual(row["name"], "Player_Update")

    def test_normalise_both_formats(self):
        est = {"start_rva": "0xfb050", "size_bytes": 108, "n_insns": 35,
               "branches": 2, "calls_direct": 4, "calls_indirect": 0,
               "switches": [], "x87_insns": 0, "sse_insns": 1}
        ghidra = {"entry": "0x50b050", "name": "FUN_50b050", "size": 108,
                  "instruction_count": 35, "cc": "__cdecl", "thunk": False,
                  "library": True}
        a = lfdb.normalise_record(est)
        # Bare 'entry' follows addr_form: ghidra exports need 'va'.
        b = lfdb.normalise_record(ghidra, addr_form="va")
        b_rva = lfdb.normalise_record(ghidra, addr_form="rva")
        self.assertEqual(a["rva"], 0xFB050)
        self.assertEqual(b["rva"], 0x10B050)
        self.assertEqual(b_rva["rva"], 0x50B050)
        # Keys that name their form win over addr_form either way.
        self.assertEqual(
            lfdb.normalise_record({"start_rva": "0x400310"},
                                  addr_form="va")["rva"], 0x400310)
        self.assertEqual(
            lfdb.normalise_record({"entry_va": "0x440310"})["rva"], 0x40310)
        self.assertEqual(a["fp_insns"], 1)
        lst, skipped = lfdb.normalise_inventory(
            {"functions": [est, ghidra,
                           {"entry": "EXTERNAL:00000001", "is_external": True},
                           {"no_address": True}]})
        self.assertEqual(len(lst), 2)
        self.assertEqual(skipped, 2)

    def test_ghidra_bare_hex_and_aliases(self):
        rec = lfdb.normalise_record(
            {"entry": "00401020", "name": "FUN_00401020", "size": 150,
             "instructions": 59, "calling_convention": "unknown",
             "is_thunk": False, "is_library": True, "section": ".text"},
            addr_form="va")
        self.assertEqual(rec["rva"], 0x1020)
        self.assertEqual(rec["n_insns"], 59)
        self.assertTrue(rec["library"])
        self.assertEqual(rec["extra"]["section"], ".text")
        self.assertEqual(lfdb.hexint("00401020"), 0x401020)
        self.assertEqual(lfdb.hexint("1234"), 1234)
        self.assertEqual(lfdb.hexint("fb050"), 0xFB050)

    def test_placeholder_names_not_stored(self):
        recs = sample_records(2)
        recs[0]["name"] = "FUN_00401020"
        recs[1]["name"] = "rage__Update"
        lfdb.import_functions(self.conn, recs, "ghidra")
        names = {r["addr"]: r["name"] for r in self.conn.execute(
            "SELECT addr, name FROM functions")}
        self.assertIsNone(names[0x100000])
        self.assertEqual(names[0x100040], "rage__Update")

    def test_library_flag_sets_kind(self):
        recs = sample_records(2)
        recs[0]["library"] = True
        lfdb.import_functions(self.conn, recs, "ghidra")
        kinds = {r["addr"]: r["kind"] for r in self.conn.execute(
            "SELECT addr, kind FROM functions")}
        self.assertEqual(kinds[0x100000], "library")
        self.assertEqual(kinds[0x100040], "unknown")


class ClaimTest(DbTest):
    def test_easiest_first_order(self):
        lfdb.import_functions(self.conn, sample_records(20), "est")
        got = lfdb.claim_next(self.conn, "lane-a", n=20)
        diffs = [r["difficulty"] for r in got]
        self.assertEqual(diffs, sorted(diffs))

    def test_no_double_claim_sequential(self):
        lfdb.import_functions(self.conn, sample_records(5), "est")
        a = lfdb.claim_next(self.conn, "lane-a", n=5)
        b = lfdb.claim_next(self.conn, "lane-b", n=5)
        self.assertEqual(len(a), 5)
        self.assertEqual(b, [])

    def test_no_double_claim_threads(self):
        lfdb.import_functions(self.conn, sample_records(60), "est")
        won: set[int] = set()
        lock = threading.Lock()

        def grab(lane):
            c = lfdb.connect(self.db)
            try:
                for r in lfdb.claim_next(c, lane, n=60):
                    with lock:
                        self.assertNotIn(r["addr"], won)
                        won.add(r["addr"])
            finally:
                c.close()

        threads = [threading.Thread(target=grab, args=("t%d" % i,))
                   for i in range(6)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        self.assertEqual(len(won), 60)

    def test_expiry_and_renew(self):
        lfdb.import_functions(self.conn, sample_records(2), "est")
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        # Foreign lane cannot renew.
        with self.assertRaises(ValueError):
            lfdb.renew_claim(self.conn, "lane-b", one["addr"])
        lfdb.renew_claim(self.conn, "lane-a", one["addr"])
        # Force expiry, sweep releases, rival can claim.
        self.conn.execute("BEGIN IMMEDIATE")
        self.conn.execute(
            "UPDATE claims SET expires_at='2000-01-01T00:00:00+00:00'")
        self.conn.execute("COMMIT")
        self.assertEqual(lfdb.release_expired(self.conn), 1)
        [again] = lfdb.claim_next(self.conn, "lane-b", n=1)
        self.assertEqual(again["addr"], one["addr"])

    def test_subsystem_filter(self):
        lfdb.import_functions(self.conn, sample_records(10), "est")
        got = lfdb.claim_next(self.conn, "lane-a", n=10, subsystem="net")
        self.assertEqual(len(got), 5)
        rest = lfdb.claim_next(self.conn, "lane-b", n=10)
        self.assertEqual(len(rest), 5)


class AcceptDeferTest(DbTest):
    def setUp(self):
        super().setUp()
        lfdb.import_functions(self.conn, sample_records(4), "est")

    def verdict(self, addr, **kw):
        v = {"function": lfdb.rva_hex(addr), "passed": True,
             "inputs_tested": 3,
             "comparisons": [{"name": "emu", "passed": True}],
             "checker_version": "test"}
        v.update(kw)
        return v

    def test_accept_happy_path(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        res = lfdb.accept_function(self.conn, "lane-a", one["addr"],
                                   self.verdict(one["addr"]))
        self.assertEqual(res["state"], "accepted")
        row = self.conn.execute("SELECT state FROM functions WHERE addr=?",
                                (one["addr"],)).fetchone()
        self.assertEqual(row["state"], "accepted")

    def test_accept_refuses_bad_verdicts(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        addr = one["addr"]
        for bad in (self.verdict(addr, passed=False),
                    self.verdict(addr, inputs_tested=0),
                    self.verdict(addr, comparisons=[]),
                    self.verdict(addr, comparisons=[{"name": "x"}]),
                    self.verdict(addr + 0x40),  # wrong function
                    {"passed": True}):
            with self.assertRaises(ValueError, msg=bad):
                lfdb.accept_function(self.conn, "lane-a", addr, bad)
        # Claim survives the refusals.
        row = self.conn.execute("SELECT state FROM functions WHERE addr=?",
                                (addr,)).fetchone()
        self.assertEqual(row["state"], "claimed")

    def test_accept_needs_own_claim(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        with self.assertRaises(ValueError):
            lfdb.accept_function(self.conn, "lane-b", one["addr"],
                                 self.verdict(one["addr"]))

    def test_defer_happy_path_and_blockers(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        res = lfdb.defer_function(self.conn, "lane-a", one["addr"],
                                  "needs-structs", "touches unknown pool")
        self.assertEqual(res["blocker"], "needs-structs")
        with self.assertRaises(ValueError):
            lfdb.defer_function(self.conn, "lane-a", one["addr"], "nope", "x")
        [two] = lfdb.claim_next(self.conn, "lane-a", n=1)
        with self.assertRaises(ValueError):
            lfdb.defer_function(self.conn, "lane-a", two["addr"],
                                "needs-manual", "  ")

    def test_attempt_cap(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        for i in range(lfdb.ATTEMPT_CAP):
            lfdb.record_attempt(self.conn, one["addr"], "lane-a", score=None)
        with self.assertRaises(ValueError):
            lfdb.record_attempt(self.conn, one["addr"], "lane-a")

    def test_no_improvement_stop(self):
        [one] = lfdb.claim_next(self.conn, "lane-a", n=1)
        lfdb.record_attempt(self.conn, one["addr"], "lane-a", score=0.9)
        for _ in range(lfdb.NO_IMPROVE_STOP - 1):
            lfdb.record_attempt(self.conn, one["addr"], "lane-a", score=0.5)
        with self.assertRaises(ValueError):
            lfdb.record_attempt(self.conn, one["addr"], "lane-a", score=0.4)
        # An improvement is still allowed.
        lfdb.record_attempt(self.conn, one["addr"], "lane-a", score=0.95)


class StatsExportTest(DbTest):
    def test_empty_functions_sort_last(self):
        recs = sample_records(5)
        recs.append({"rva": 0x200000, "size_bytes": 0, "n_insns": 0,
                     "branches": 0, "calls_direct": 0, "calls_indirect": 0,
                     "switches": 0, "fp_insns": 0, "sha1_masked": None,
                     "name": None, "subsystem": None, "kind_hint": None,
                     "library": False, "thunk": False, "extra": {}})
        lfdb.import_functions(self.conn, recs, "est")
        got = lfdb.claim_next(self.conn, "lane-a", n=6)
        self.assertEqual(got[-1]["addr"], 0x200000)

    def test_counts_add_up(self):
        lfdb.import_functions(self.conn, sample_records(6), "est")
        counts = lfdb.state_counts(self.conn)
        self.assertEqual(sum(counts.values()), 6)
        self.assertEqual(set(counts), set(lfdb.STATES))


if __name__ == "__main__":
    unittest.main()
