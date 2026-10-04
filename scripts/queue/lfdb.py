"""Shared core for the LibertyFlux work-queue tools (lane h-queue, v1).

Usage:
    import lfdb
    conn = lfdb.connect()            # opens .artifacts/db/libertyflux.db
    lfdb.init_db(conn)               # create schema if missing (idempotent)

All queue state lives in ONE SQLite database (WAL mode, 30s busy timeout).
Every tool in this folder imports this module; the only per-tool logic is CLI
argument handling. Safe for many lanes at once: each lane uses its own
connection, transactions are short, claims use BEGIN IMMEDIATE plus a
UNIQUE constraint so a double claim fails instead of corrupting.

Canonical address form: RVA as an INTEGER (image base 0x400000, 32-bit).
Hex strings ("0x...", with or without leading zeros) are parsed on input;
integers are passed through. VAs are converted by subtracting the image base.

Schema (version 1), see SCHEMA_SQL below:
    schema_version  one row per applied migration
    functions       one row per known function (queue state lives here)
    names           name history per function (source + confidence, never deleted)
    evidence        free-form key/value facts per function (BSim scores, notes)
    classes         RTTI classes (subset needed by the queue: id, names)
    vtable_slots    (class, slot) -> function address
    claims          one row per ACTIVE claim; addr is UNIQUE (the atomicity)
    attempts        every recorded try on a function (best attempt lookup)
    verdicts        checker verdicts as received (raw JSON kept)
    deferrals       deferral history; latest active row explains a deferred state
    batches         ledger: one row per sweep with queue statistics
"""

from __future__ import annotations

import json
import os
import re
import sqlite3
import time
from datetime import datetime, timedelta, timezone
from pathlib import Path

SCHEMA_VERSION = 1
IMAGE_BASE = 0x400000
CLAIM_TTL = timedelta(hours=2)
BUSY_TIMEOUT_MS = 30000

STATES = ("queued", "claimed", "accepted", "deferred", "excluded")
KINDS = ("game", "library", "runtime", "encrypted", "stub", "thunk", "unknown")

# Fixed blocker vocabulary for defer.py. A deferral with any other tag is refused.
BLOCKER_TAGS = (
    "needs-structs",     # touches an undocumented structure
    "needs-caller",      # cannot understand without an accepted caller
    "needs-callee",      # cannot understand without an accepted callee
    "float-tolerance",   # FP diffs inside tolerance, needs in-game check
    "indirect-call",     # target of indirect call/jump unresolved
    "switch-table",      # jump table not recovered
    "encrypted-ref",     # references the encrypted first MB
    "bounds-dispute",    # function boundaries uncertain / overlap neighbour
    "thunk-chain",       # long/odd thunk chain, needs manual look
    "tool-bug",          # queue, context or checker tooling misbehaved
    "needs-manual",      # human decision needed (general)
    "other",             # none of the above; note must explain
)

ATTEMPT_CAP = 12          # max attempts ever recorded per function
NO_IMPROVE_STOP = 4       # stop after this many attempts with no score gain

# Tool-generated placeholder names are never stored as function names.
PLACEHOLDER_PREFIXES = ("FUN_", "SUB_", "LAB_", "DAT_", "thunk_FUN_",
                        "PTR_", "s_", "UNK_")


def is_placeholder_name(name: str | None) -> bool:
    return bool(name) and name.startswith(PLACEHOLDER_PREFIXES)


def repo_root() -> Path:
    """Repository root, derived from this file's location (scripts/queue/)."""
    here = Path(__file__).resolve()
    for parent in [here.parent] + list(here.parent.parents):
        if (parent / "AGENTS.md").exists():
            return parent
    # Fallback: scripts/queue/ is 2 levels below the root.
    return here.parents[2]


def default_db_path() -> Path:
    return repo_root() / ".artifacts" / "db" / "libertyflux.db"


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def hexint(value) -> int:
    """Parse an RVA/VA given as int or hex/dec string. Accepts 0x-prefixed
    hex, bare 8-digit hex as emitted by Ghidra ('00401020'), plain decimals,
    and short bare hex with a-f digits ('fb050')."""
    if isinstance(value, int):
        return value
    s = str(value).strip()
    if s.lower().startswith("0x"):
        return int(s, 16)
    if re.fullmatch(r"[0-9a-fA-F]{7,16}", s):
        return int(s, 16)
    if re.fullmatch(r"[0-9]+", s):
        return int(s, 10)
    return int(s, 16)  # short bare hex like 'fb050'; raises if not hex


def to_rva(value, form: str = "rva", image_base: int = IMAGE_BASE) -> int:
    """Normalise an address to an RVA int. The form must be DECLARED by the
    caller ('rva' or 'va'); nothing is sniffed, because on this binary a
    value like 0x401270 is a valid RVA (plain code) AND a valid VA (RVA
    0x1270) and guessing picks the wrong one. VAs have the image base
    subtracted; anything else raises."""
    n = hexint(value)
    if form == "va":
        if n < image_base:
            raise ValueError("0x%x is below the image base, not a VA" % n)
        return n - image_base
    if form == "rva":
        return n
    raise ValueError("address form must be 'rva' or 'va', got %r" % (form,))


def resolve_addr(conn: sqlite3.Connection, value) -> int:
    """Resolve a CLI/verdict address to an RVA, grounded in the database:
    exact RVA match wins, else VA-minus-base match, else the raw RVA."""
    n = hexint(value)
    if conn.execute("SELECT 1 FROM functions WHERE addr=?",
                    (n,)).fetchone():
        return n
    va = n - IMAGE_BASE
    if n >= IMAGE_BASE and conn.execute(
            "SELECT 1 FROM functions WHERE addr=?", (va,)).fetchone():
        return va
    return n


def va_of(rva: int, image_base: int = IMAGE_BASE) -> int:
    return int(rva) + image_base


def rva_hex(rva: int) -> str:
    return "0x%x" % (int(rva),)


SCHEMA_SQL = """
CREATE TABLE IF NOT EXISTS schema_version (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS functions (
    addr            INTEGER PRIMARY KEY,   -- RVA
    size_bytes      INTEGER NOT NULL DEFAULT 0,
    n_insns         INTEGER NOT NULL DEFAULT 0,
    branches        INTEGER NOT NULL DEFAULT 0,
    calls_direct    INTEGER NOT NULL DEFAULT 0,
    calls_indirect  INTEGER NOT NULL DEFAULT 0,
    switches        INTEGER NOT NULL DEFAULT 0,
    fp_insns        INTEGER NOT NULL DEFAULT 0,
    sha1_masked     TEXT,
    kind            TEXT NOT NULL DEFAULT 'unknown',
    subsystem       TEXT,
    name            TEXT,                  -- current best name (history in names)
    name_confidence TEXT,
    state           TEXT NOT NULL DEFAULT 'queued',
    difficulty      REAL NOT NULL DEFAULT 0,
    source          TEXT,                  -- inventory that last touched this row
    extra           TEXT                   -- JSON: thunk/cc/origin/anything else
);
CREATE INDEX IF NOT EXISTS idx_functions_state_diff
    ON functions (state, difficulty, addr);
CREATE INDEX IF NOT EXISTS idx_functions_kind ON functions (kind);
CREATE INDEX IF NOT EXISTS idx_functions_subsystem ON functions (subsystem);
CREATE TABLE IF NOT EXISTS names (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    addr        INTEGER NOT NULL REFERENCES functions (addr),
    name        TEXT NOT NULL,
    source      TEXT NOT NULL,
    confidence  TEXT NOT NULL,
    created_at  TEXT NOT NULL,
    superseded  INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_names_addr ON names (addr);
CREATE TABLE IF NOT EXISTS evidence (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    addr        INTEGER NOT NULL REFERENCES functions (addr),
    kind        TEXT NOT NULL,
    value       TEXT NOT NULL,
    source      TEXT NOT NULL,
    created_at  TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_evidence_addr ON evidence (addr);
CREATE TABLE IF NOT EXISTS classes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    mangled     TEXT UNIQUE NOT NULL,
    demangled   TEXT NOT NULL,
    namespace   TEXT,
    polymorphic INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS vtable_slots (
    class_id    INTEGER NOT NULL REFERENCES classes (id),
    slot        INTEGER NOT NULL,
    addr        INTEGER NOT NULL,          -- RVA of the slot target
    vt_rva      INTEGER,
    PRIMARY KEY (class_id, slot)
);
CREATE INDEX IF NOT EXISTS idx_vtable_addr ON vtable_slots (addr);
CREATE TABLE IF NOT EXISTS claims (
    addr        INTEGER PRIMARY KEY REFERENCES functions (addr),
    lane        TEXT NOT NULL,
    claimed_at  TEXT NOT NULL,
    expires_at  TEXT NOT NULL,
    renewed_at  TEXT
);
CREATE TABLE IF NOT EXISTS attempts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    addr        INTEGER NOT NULL REFERENCES functions (addr),
    lane        TEXT NOT NULL,
    attempt_no  INTEGER NOT NULL,
    created_at  TEXT NOT NULL,
    verdict     TEXT,                      -- 'pass' / 'fail' / NULL
    score       REAL,                      -- higher is better, NULL if unscored
    path        TEXT,                      -- rewrite file under .artifacts
    note        TEXT
);
CREATE INDEX IF NOT EXISTS idx_attempts_addr ON attempts (addr);
CREATE TABLE IF NOT EXISTS verdicts (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    addr            INTEGER NOT NULL REFERENCES functions (addr),
    lane            TEXT NOT NULL,
    passed          INTEGER NOT NULL,
    inputs_tested   INTEGER NOT NULL DEFAULT 0,
    comparisons     INTEGER NOT NULL DEFAULT 0,
    detail          TEXT NOT NULL,         -- raw verdict JSON
    path            TEXT,
    created_at      TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_verdicts_addr ON verdicts (addr);
CREATE TABLE IF NOT EXISTS deferrals (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    addr        INTEGER NOT NULL REFERENCES functions (addr),
    lane        TEXT NOT NULL,
    blocker     TEXT NOT NULL,
    note        TEXT NOT NULL,
    created_at  TEXT NOT NULL,
    active      INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_deferrals_addr ON deferrals (addr);
CREATE TABLE IF NOT EXISTS batches (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at      TEXT NOT NULL,
    ended_at        TEXT NOT NULL,
    released_stale  INTEGER NOT NULL DEFAULT 0,
    n_queued        INTEGER NOT NULL DEFAULT 0,
    n_claimed       INTEGER NOT NULL DEFAULT 0,
    n_accepted      INTEGER NOT NULL DEFAULT 0,
    n_deferred      INTEGER NOT NULL DEFAULT 0,
    n_excluded      INTEGER NOT NULL DEFAULT 0,
    note            TEXT,
    detail          TEXT                   -- JSON: deferral reasons, lanes, cost
);
"""

MIGRATIONS = {
    # version: list of SQL statements applied in order inside one transaction.
    # v1 is the base schema above; future versions append ALTER TABLEs here.
}


def connect(db_path: str | os.PathLike | None = None) -> sqlite3.Connection:
    """Open the queue database with WAL mode and a 30s busy timeout."""
    path = Path(db_path) if db_path else default_db_path()
    path.parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(str(path), timeout=BUSY_TIMEOUT_MS / 1000.0,
                           isolation_level=None)  # autocommit; we BEGIN by hand
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA busy_timeout=%d" % BUSY_TIMEOUT_MS)
    conn.execute("PRAGMA foreign_keys=ON")
    conn.execute("PRAGMA synchronous=NORMAL")
    init_db(conn)
    return conn


def db_version(conn: sqlite3.Connection) -> int:
    try:
        row = conn.execute("SELECT MAX(version) AS v FROM schema_version").fetchone()
    except sqlite3.OperationalError:
        return 0
    return row["v"] if row and row["v"] else 0


def init_db(conn: sqlite3.Connection) -> int:
    """Create the schema if missing and apply pending migrations. Returns version."""
    # executescript() commits implicitly, so schema first, versioning after.
    conn.executescript(SCHEMA_SQL)
    conn.execute("BEGIN IMMEDIATE")
    try:
        have = db_version(conn)
        if have < 1:
            conn.execute(
                "INSERT OR IGNORE INTO schema_version (version, applied_at) VALUES (1, ?)",
                (now_iso(),))
            have = 1
        for ver in sorted(MIGRATIONS):
            if ver > have:
                for stmt in MIGRATIONS[ver]:
                    conn.execute(stmt)
                conn.execute(
                    "INSERT INTO schema_version (version, applied_at) VALUES (?, ?)",
                    (ver, now_iso()))
                have = ver
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return db_version(conn)


# --------------------------------------------------------------------------
# Difficulty: easiest first. Weights are documented so a later lane can tune
# them against measured attempt counts; order must stay deterministic.
# --------------------------------------------------------------------------

def difficulty(n_insns=0, branches=0, calls=0, switches=0, fp=0) -> float:
    return (float(n_insns or 0) + 8.0 * float(branches or 0)
            + 4.0 * float(calls or 0) + 25.0 * float(switches or 0)
            + 6.0 * float(fp or 0))


# --------------------------------------------------------------------------
# Inventory normalisation. Accepts the p0 estimate format AND the richer
# f-function-inventory / Ghidra format (entry, name, size, insn count,
# calling convention, thunk and library flags). Unknown keys are kept in
# 'extra' so nothing is lost when the richer inventory replaces the estimate.
# --------------------------------------------------------------------------

def _first(d: dict, *keys, default=None):
    for k in keys:
        if k in d and d[k] is not None:
            return d[k]
    return default


def normalise_record(raw: dict, addr_form: str = "rva") -> dict:
    """Normalise one inventory record. Keys whose name says RVA (start_rva,
    entry_rva, rva, addr) are RVAs; keys saying VA (*_va, va) are VAs; the
    bare keys 'entry'/'address' follow addr_form (Ghidra exports: 'va')."""
    for key in ("start_rva", "entry_rva", "rva", "addr"):
        if raw.get(key) is not None:
            return _normalise_with_rva(raw, to_rva(raw[key], "rva"))
    for key in ("entry_va", "address_va", "va"):
        if raw.get(key) is not None:
            return _normalise_with_rva(raw, to_rva(raw[key], "va"))
    entry = _first(raw, "entry", "address")
    if entry is None:
        raise ValueError("inventory record has no address: %r" % (raw,))
    return _normalise_with_rva(raw, to_rva(entry, addr_form))


def _normalise_with_rva(raw: dict, rva: int) -> dict:
    size = int(_first(raw, "size_bytes", "size", "length", default=0) or 0)
    n_insns = int(_first(raw, "n_insns", "insns", "instruction_count",
                         "instructions", "ninsns", default=0) or 0)
    branches = int(_first(raw, "branches", "n_branches", default=0) or 0)
    calls_d = int(_first(raw, "calls_direct", "calls", "n_calls", default=0) or 0)
    calls_i = int(_first(raw, "calls_indirect", "indirect_calls", default=0) or 0)
    sw = _first(raw, "switches", "n_switches", "switch_count", default=0) or 0
    switches = len(sw) if isinstance(sw, list) else int(sw)
    fp = int(_first(raw, "x87_insns", default=0) or 0) + int(
        _first(raw, "sse_insns", default=0) or 0) + int(
        _first(raw, "fp_insns", "float_insns", default=0) or 0)
    known = {"start_rva", "entry", "entry_rva", "entry_va", "address",
             "address_va", "va", "rva", "addr",
             "size_bytes", "size", "length", "n_insns", "insns",
             "instruction_count", "ninsns", "branches", "n_branches",
             "calls_direct", "calls", "n_calls", "calls_indirect",
             "indirect_calls", "switches", "n_switches", "switch_count",
             "x87_insns", "sse_insns", "fp_insns", "float_insns",
             "sha1_masked", "name", "subsystem", "cc", "calling_convention",
             "thunk", "library", "kind", "origin"}
    extra = {k: v for k, v in raw.items() if k not in known}
    for k in ("cc", "calling_convention", "thunk", "is_thunk", "origin",
              "callers", "callees", "section"):
        if k in raw and raw[k] is not None:
            extra[k] = raw[k]
    lib_flag = bool(_first(raw, "library", "is_library", default=False))
    extra["library"] = lib_flag
    return {
        "rva": rva,
        "size_bytes": size,
        "n_insns": n_insns,
        "branches": branches,
        "calls_direct": calls_d,
        "calls_indirect": calls_i,
        "switches": switches,
        "fp_insns": fp,
        "sha1_masked": _first(raw, "sha1_masked", "sha1", "hash"),
        "name": _first(raw, "name"),
        "subsystem": _first(raw, "subsystem"),
        "kind_hint": _first(raw, "kind"),
        "library": lib_flag,
        "thunk": bool(_first(raw, "thunk", "is_thunk", default=False)),
        "extra": extra,
    }


def normalise_inventory(data, addr_form: str = "rva"):
    """Returns (records, skipped): records with an unparsable address
    (Ghidra 'EXTERNAL:...' placeholders) or is_external=true are skipped,
    because they have no code to queue. The count is reported, never silent."""
    if isinstance(data, dict):
        for key in ("functions", "functions_text", "items", "rows"):
            if isinstance(data.get(key), list):
                data = data[key]
                break
        else:
            raise ValueError("inventory JSON object has no function list")
    records, skipped = [], 0
    for r in data:
        if isinstance(r, dict) and r.get("is_external"):
            skipped += 1
            continue
        try:
            records.append(normalise_record(r, addr_form))
        except (ValueError, KeyError, TypeError):
            skipped += 1
    return records, skipped


def import_functions(conn: sqlite3.Connection, records: list[dict],
                     source: str) -> dict:
    """Upsert inventory records WITHOUT losing queue state.

    New addresses are inserted as queued/unknown; existing rows keep their
    state, name, kind and subsystem and only get fresh metrics. Returns
    {'inserted': n, 'updated': m}.
    """
    inserted = updated = 0
    conn.execute("BEGIN IMMEDIATE")
    try:
        for r in records:
            if r["n_insns"] == 0 and r["size_bytes"] == 0:
                # Unmeasurable (usually a seed that is not code): sort last
                # so lanes never trip over it first; still servable.
                diff = 1e9
            else:
                diff = difficulty(r["n_insns"], r["branches"],
                                  r["calls_direct"] + r["calls_indirect"],
                                  r["switches"], r["fp_insns"])
            cur = conn.execute(
                "SELECT addr FROM functions WHERE addr=?", (r["rva"],)).fetchone()
            if cur is None:
                kind = "unknown"
                if r["kind_hint"] in KINDS:
                    kind = r["kind_hint"]
                elif r["library"]:
                    kind = "library"
                conn.execute(
                    "INSERT INTO functions (addr, size_bytes, n_insns, branches,"
                    " calls_direct, calls_indirect, switches, fp_insns,"
                    " sha1_masked, kind, subsystem, state, difficulty, source, extra)"
                    " VALUES (?,?,?,?,?,?,?,?,?,?,?, 'queued',?,?,?)",
                    (r["rva"], r["size_bytes"], r["n_insns"], r["branches"],
                     r["calls_direct"], r["calls_indirect"], r["switches"],
                     r["fp_insns"], r["sha1_masked"], kind, r["subsystem"],
                     diff, source, json.dumps(r["extra"], sort_keys=True)))
                inserted += 1
                if r["name"] and not is_placeholder_name(r["name"]):
                    set_name(conn, r["rva"], r["name"], source, "inventory")
            else:
                conn.execute(
                    "UPDATE functions SET size_bytes=?, n_insns=?, branches=?,"
                    " calls_direct=?, calls_indirect=?, switches=?, fp_insns=?,"
                    " sha1_masked=COALESCE(?, sha1_masked), difficulty=?,"
                    " source=?, extra=? WHERE addr=?",
                    (r["size_bytes"], r["n_insns"], r["branches"],
                     r["calls_direct"], r["calls_indirect"], r["switches"],
                     r["fp_insns"], r["sha1_masked"], diff, source,
                     json.dumps(r["extra"], sort_keys=True), r["rva"]))
                updated += 1
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return {"inserted": inserted, "updated": updated}


def set_name(conn: sqlite3.Connection, addr: int, name: str, source: str,
             confidence: str) -> None:
    """Append to name history and make it current. Caller owns the transaction."""
    conn.execute("UPDATE names SET superseded=1 WHERE addr=? AND superseded=0",
                 (addr,))
    conn.execute(
        "INSERT INTO names (addr, name, source, confidence, created_at)"
        " VALUES (?,?,?,?,?)", (addr, name, source, confidence, now_iso()))
    conn.execute("UPDATE functions SET name=?, name_confidence=? WHERE addr=?",
                 (name, confidence, addr))


# --------------------------------------------------------------------------
# Claims: the atomic core. One transaction: drop expired claims, pick the N
# easiest unclaimed rows, insert claim rows. The UNIQUE(addr) constraint is
# the backstop: a concurrent winner makes the loser fail, never double-claim.
# --------------------------------------------------------------------------

def _expiry_iso() -> str:
    return (datetime.now(timezone.utc) + CLAIM_TTL).isoformat(timespec="seconds")


def release_expired(conn: sqlite3.Connection, in_txn: bool = False) -> int:
    """Delete expired claims, return queued state to their functions."""
    if not in_txn:
        conn.execute("BEGIN IMMEDIATE")
    try:
        rows = conn.execute(
            "SELECT addr FROM claims WHERE expires_at <= ?", (now_iso(),)).fetchall()
        for (addr,) in rows:
            conn.execute("DELETE FROM claims WHERE addr=?", (addr,))
            conn.execute(
                "UPDATE functions SET state='queued'"
                " WHERE addr=? AND state='claimed'", (addr,))
        if not in_txn:
            conn.execute("COMMIT")
    except BaseException:
        if not in_txn:
            conn.execute("ROLLBACK")
        raise
    return len(rows)


def claim_next(conn: sqlite3.Connection, lane: str, n: int = 1,
               subsystem: str | None = None,
               class_name: str | None = None) -> list[dict]:
    """Claim the next N easiest functions for a lane. Returns row dicts."""
    if not lane:
        raise ValueError("lane id is required")
    if n < 1:
        raise ValueError("n must be >= 1")
    conn.execute("BEGIN IMMEDIATE")
    try:
        release_expired(conn, in_txn=True)
        sql = ("SELECT f.addr, f.size_bytes, f.n_insns, f.difficulty, f.name,"
               " f.subsystem FROM functions f WHERE f.state='queued'")
        args: list = []
        if subsystem:
            sql += " AND f.subsystem=?"
            args.append(subsystem)
        if class_name:
            sql += (" AND f.addr IN (SELECT v.addr FROM vtable_slots v"
                    " JOIN classes c ON c.id=v.class_id"
                    " WHERE c.demangled=? OR c.mangled=?)")
            args += [class_name, class_name]
        sql += " ORDER BY f.difficulty ASC, f.addr ASC LIMIT ?"
        args.append(n)
        rows = [dict(r) for r in conn.execute(sql, args).fetchall()]
        exp = _expiry_iso()
        claimed_at = now_iso()
        for r in rows:
            conn.execute(
                "INSERT INTO claims (addr, lane, claimed_at, expires_at)"
                " VALUES (?,?,?,?)",
                (r["addr"], lane, claimed_at, exp))
            conn.execute("UPDATE functions SET state='claimed' WHERE addr=?",
                         (r["addr"],))
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return rows


def renew_claim(conn: sqlite3.Connection, lane: str, addr: int) -> dict:
    conn.execute("BEGIN IMMEDIATE")
    try:
        row = conn.execute("SELECT * FROM claims WHERE addr=?",
                           (addr,)).fetchone()
        if row is None:
            raise ValueError("no active claim on %s" % rva_hex(addr))
        if row["lane"] != lane:
            raise ValueError("claim on %s belongs to lane %s"
                             % (rva_hex(addr), row["lane"]))
        exp = _expiry_iso()
        conn.execute("UPDATE claims SET expires_at=?, renewed_at=? WHERE addr=?",
                     (exp, now_iso(), addr))
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return {"addr": addr, "lane": lane, "expires_at": exp}


# --------------------------------------------------------------------------
# Verdicts: the JSON format the checker must produce.
#   {"function": "0x<rva hex>", "passed": true, "inputs_tested": N,
#    "comparisons": [{"name": "...", "passed": true}], "checker_version": "..."}
# --------------------------------------------------------------------------

def load_verdict(path: str | os.PathLike) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def validate_verdict(verdict: dict, addr: int) -> dict:
    """Raise ValueError unless the verdict passes for this address. Returns
    a normalised summary {passed, inputs_tested, comparisons}."""
    if not isinstance(verdict, dict):
        raise ValueError("verdict must be a JSON object")
    if "function" not in verdict:
        raise ValueError("verdict is missing 'function'")
    got = hexint(verdict["function"])
    if got != int(addr) and got != int(addr) + IMAGE_BASE:
        raise ValueError("verdict is for %s, not %s"
                         % (verdict["function"], rva_hex(addr)))
    if verdict.get("passed") is not True:
        raise ValueError("verdict did not pass")
    inputs = verdict.get("inputs_tested", 0)
    if not isinstance(inputs, int) or inputs <= 0:
        raise ValueError("verdict tested no inputs")
    comps = verdict.get("comparisons", [])
    if not isinstance(comps, list) or not comps:
        raise ValueError("verdict has no comparisons")
    for c in comps:
        if not isinstance(c, dict) or c.get("passed") is not True:
            raise ValueError("verdict has a failing comparison: %r" % (c,))
    return {"passed": True, "inputs_tested": inputs, "comparisons": len(comps)}


def record_attempt(conn: sqlite3.Connection, addr: int, lane: str,
                   verdict: str | None = None, score: float | None = None,
                   path: str | None = None, note: str | None = None,
                   in_txn: bool = False) -> int:
    """Append an attempt row; enforces the attempt cap and no-improvement stop.
    Returns the attempt number. Raises ValueError when the budget is spent."""
    if not in_txn:
        conn.execute("BEGIN IMMEDIATE")
    try:
        rows = conn.execute(
            "SELECT score FROM attempts WHERE addr=? ORDER BY attempt_no",
            (addr,)).fetchall()
        if len(rows) >= ATTEMPT_CAP:
            raise ValueError("attempt cap (%d) reached for %s"
                             % (ATTEMPT_CAP, rva_hex(addr)))
        scored = [r["score"] for r in rows if r["score"] is not None]
        if score is not None and len(scored) >= NO_IMPROVE_STOP:
            best = max(scored)
            tail = scored[-NO_IMPROVE_STOP:]
            if all(s <= best for s in tail) and score <= best:
                raise ValueError("no improvement in %d attempts for %s"
                                 % (NO_IMPROVE_STOP, rva_hex(addr)))
        attempt_no = len(rows) + 1
        conn.execute(
            "INSERT INTO attempts (addr, lane, attempt_no, created_at, verdict,"
            " score, path, note) VALUES (?,?,?,?,?,?,?,?)",
            (addr, lane, attempt_no, now_iso(), verdict, score, path, note))
        if not in_txn:
            conn.execute("COMMIT")
    except BaseException:
        if not in_txn:
            conn.execute("ROLLBACK")
        raise
    return attempt_no


def best_attempt(conn: sqlite3.Connection, addr: int) -> dict | None:
    row = conn.execute(
        "SELECT * FROM attempts WHERE addr=? ORDER BY"
        " (score IS NULL), score DESC, attempt_no DESC LIMIT 1",
        (addr,)).fetchone()
    return dict(row) if row else None


def accept_function(conn: sqlite3.Connection, lane: str, addr: int,
                    verdict: dict, verdict_path: str | None = None) -> dict:
    summary = validate_verdict(verdict, addr)
    conn.execute("BEGIN IMMEDIATE")
    try:
        claim = conn.execute("SELECT lane FROM claims WHERE addr=?",
                             (addr,)).fetchone()
        if claim is None:
            raise ValueError("no active claim on %s" % rva_hex(addr))
        if claim["lane"] != lane:
            raise ValueError("claim on %s belongs to lane %s"
                             % (rva_hex(addr), claim["lane"]))
        conn.execute(
            "INSERT INTO verdicts (addr, lane, passed, inputs_tested,"
            " comparisons, detail, path, created_at)"
            " VALUES (?,?,?,?,?,?,?,?)",
            (addr, lane, 1, summary["inputs_tested"], summary["comparisons"],
             json.dumps(verdict, sort_keys=True), verdict_path, now_iso()))
        record_attempt(conn, addr, lane, verdict="pass", path=verdict_path,
                       note="accepted", in_txn=True)
        conn.execute("DELETE FROM claims WHERE addr=?", (addr,))
        conn.execute("UPDATE deferrals SET active=0 WHERE addr=?", (addr,))
        conn.execute("UPDATE functions SET state='accepted' WHERE addr=?", (addr,))
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return {"addr": addr, "state": "accepted", **summary}


def defer_function(conn: sqlite3.Connection, lane: str, addr: int,
                   blocker: str, note: str) -> dict:
    if blocker not in BLOCKER_TAGS:
        raise ValueError("unknown blocker tag %r (pick one of: %s)"
                         % (blocker, ", ".join(BLOCKER_TAGS)))
    if not note or not note.strip():
        raise ValueError("a note is required with a deferral")
    conn.execute("BEGIN IMMEDIATE")
    try:
        claim = conn.execute("SELECT lane FROM claims WHERE addr=?",
                             (addr,)).fetchone()
        if claim is None:
            raise ValueError("no active claim on %s" % rva_hex(addr))
        if claim["lane"] != lane:
            raise ValueError("claim on %s belongs to lane %s"
                             % (rva_hex(addr), claim["lane"]))
        conn.execute("UPDATE deferrals SET active=0 WHERE addr=?", (addr,))
        conn.execute(
            "INSERT INTO deferrals (addr, lane, blocker, note, created_at)"
            " VALUES (?,?,?,?,?)", (addr, lane, blocker, note.strip(), now_iso()))
        conn.execute("DELETE FROM claims WHERE addr=?", (addr,))
        conn.execute("UPDATE functions SET state='deferred' WHERE addr=?", (addr,))
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return {"addr": addr, "state": "deferred", "blocker": blocker}


def state_counts(conn: sqlite3.Connection) -> dict:
    rows = conn.execute(
        "SELECT state, COUNT(*) AS n FROM functions GROUP BY state").fetchall()
    counts = {s: 0 for s in STATES}
    for r in rows:
        counts[r["state"]] = r["n"]
    return counts


def find_similar(addr: int, conn: sqlite3.Connection, limit: int = 5) -> list[dict]:
    """Most similar already-accepted functions. STUB: always returns [].

    Interface (stable): returns at most `limit` dicts with keys
    addr, name, score (0..1, higher is more similar) and source_path
    (the accepted Rust file under .artifacts). The real implementation
    plugs Ghidra BSim (or a masked-bytes family match on sha1_masked)
    in here without touching callers.
    """
    _ = (addr, conn, limit)
    return []


def class_slots_for(conn: sqlite3.Connection, addr: int) -> list[dict]:
    rows = conn.execute(
        "SELECT c.mangled, c.demangled, c.namespace, v.slot, v.vt_rva"
        " FROM vtable_slots v JOIN classes c ON c.id=v.class_id"
        " WHERE v.addr=? ORDER BY c.demangled, v.slot", (addr,)).fetchall()
    return [dict(r) for r in rows]
