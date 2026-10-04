"""Build the work queue: game functions only, easiest first.

Usage:
    python queue_build.py [--inventory PATH] [--db PATH] [--fresh]
                          [--libraries PATH] [--no-libraries]
                          [--encrypted-from 0x1000 --encrypted-to 0xfb000]
                          [--subsystems PATH] [--classes PATH] [--vtables PATH]

Steps:
  1. Load the inventory (default: p0-function-estimate functions_estimate.json)
     via import_inventory semantics (existing queue state is kept).
  2. Classify every function: encrypted-range -> kind 'encrypted', inside a
     library code range from libraries.json -> 'library', runtime patterns
     (or kind hint) -> 'runtime', everything else -> 'game'.
  3. Mark non-game functions state 'excluded' (accepted rows are never
     downgraded; a re-run only excludes rows still queued/claimed).
  4. Optionally attach subsystems (JSON: [{"from":..,"to":..,"subsystem":..}])
     and RTTI classes/vtables (classes.json, vtables.json).
  5. Print counts and the difficulty of the first 5 queued functions.

Queue order is (difficulty ASC, addr ASC); difficulty comes from lfdb and is
stored per row so ORDER BY never recomputes. --fresh deletes the database
first; without it a rebuild is additive and safe.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb

# Names that mark statically linked C runtime / compiler helpers. A function
# whose inventory name starts with one of these (after stripping a leading
# '_' or '@') is kind 'runtime'. Deliberately narrow: game code must never
# be excluded by accident.
RUNTIME_PREFIXES = (
    "__", "___", "_RTC_", "_CRT_", "_SEH_", "_EH_", "_CI", "_all", "_aull",
    "_ftol", "_flt", "_except_", "_seh_", "_chkstk", "_NLG_", "@_",
    "__imp__", "??_C@", "??_7", "??_8",
)


def load_ranges(path: str | None) -> list[tuple[int, int, str]]:
    """Library code ranges as (from_rva, to_rva, libname). libraries.json
    documents its ranges as VAs (see its code_section_va), so convert."""
    if not path:
        return []
    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
    out = []
    for lib in data.get("libraries", []):
        for r in lib.get("code_ranges", []) or []:
            try:
                out.append((lfdb.to_rva(r["from"], "va"),
                            lfdb.to_rva(r["to"], "va"),
                            lib.get("name", "?")))
            except (KeyError, ValueError):
                continue
    return out


def _extra_flag(row, key: str) -> bool:
    try:
        return bool(json.loads(row["extra"] or "{}").get(key))
    except (ValueError, TypeError):
        return False


def looks_like_runtime(name: str | None) -> bool:
    if not name:
        return False
    n = name.strip().lstrip("_@")
    return n.startswith(RUNTIME_PREFIXES) or name.startswith(RUNTIME_PREFIXES)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--inventory", default=None)
    ap.add_argument("--db", default=None)
    ap.add_argument("--fresh", action="store_true",
                    help="Delete the database before building")
    ap.add_argument("--libraries", default=None,
                    help="libraries.json (default: p0-libraries)")
    ap.add_argument("--no-libraries", action="store_true",
                    help="Skip library exclusion")
    ap.add_argument("--encrypted-from", default="0x1000")
    ap.add_argument("--encrypted-to", default="0xfb000")
    ap.add_argument("--subsystems", default=None,
                    help="Subsystem range map JSON (optional)")
    ap.add_argument("--classes", default=None, help="classes.json (optional)")
    ap.add_argument("--vtables", default=None, help="vtables.json (optional)")
    ap.add_argument("--addr-form", choices=("rva", "va"), default="rva",
                    help="Address form of bare entry/address keys")
    args = ap.parse_args(argv)

    root = lfdb.repo_root()
    inv = args.inventory or str(
        root / ".artifacts" / "scratch" / "p0-function-estimate"
        / "functions_estimate.json")
    if args.fresh and args.db:
        Path(args.db).unlink(missing_ok=True)
    elif args.fresh:
        lfdb.default_db_path().unlink(missing_ok=True)

    with open(inv, "r", encoding="utf-8") as fh:
        records, skipped = lfdb.normalise_inventory(
            json.load(fh), addr_form=args.addr_form)
    conn = lfdb.connect(args.db)
    try:
        res = lfdb.import_functions(conn, records, Path(inv).name)
        enc_from = lfdb.hexint(args.encrypted_from)
        enc_to = lfdb.hexint(args.encrypted_to)
        lib_ranges = [] if args.no_libraries else load_ranges(
            args.libraries or str(root / ".artifacts" / "scratch"
                                  / "p0-libraries" / "libraries.json"))

        def lib_at(rva: int) -> str | None:
            for lo, hi, name in lib_ranges:
                if lo <= rva < hi:
                    return name
            return None

        # Re-read names/kind hints the import may have stored.
        rows = conn.execute(
            "SELECT addr, name, kind, extra FROM functions").fetchall()
        n_game = n_lib = n_rt = n_enc = 0
        conn.execute("BEGIN IMMEDIATE")
        try:
            for row in rows:
                addr = row["addr"]
                if enc_from <= addr < enc_to:
                    kind = "encrypted"
                elif (lib_at(addr) is not None
                        and row["kind"] not in ("game", "runtime")):
                    kind = "library"
                elif row["kind"] == "library" and lib_at(addr) is None:
                    kind = "game"  # range map shrank; restore to game
                elif looks_like_runtime(row["name"]) or row["kind"] == "runtime":
                    kind = "runtime"
                elif row["kind"] == "unknown" and _extra_flag(row, "library"):
                    kind = "library"  # inventory's own library flag (Ghidra)
                elif row["kind"] in ("unknown",):
                    kind = "game"
                else:
                    kind = row["kind"]
                conn.execute("UPDATE functions SET kind=? WHERE addr=?",
                             (kind, addr))
                if kind == "game":
                    n_game += 1
                else:
                    if kind == "library":
                        n_lib += 1
                    elif kind == "runtime":
                        n_rt += 1
                    else:
                        n_enc += 1
                    # Exclude unless already decided (never touch accepted).
                    conn.execute(
                        "UPDATE functions SET state='excluded' WHERE addr=?"
                        " AND state IN ('queued','claimed')", (addr,))
                    conn.execute("DELETE FROM claims WHERE addr=?", (addr,))
            conn.execute("COMMIT")
        except BaseException:
            conn.execute("ROLLBACK")
            raise

        if args.subsystems:
            apply_subsystems(conn, args.subsystems)
        if args.classes:
            import_classes(conn, args.classes)
        if args.vtables:
            import_vtables(conn, args.vtables)

        counts = lfdb.state_counts(conn)
        kinds = {r["kind"]: r["n"] for r in conn.execute(
            "SELECT kind, COUNT(*) n FROM functions GROUP BY kind")}
        head = [dict(r) for r in conn.execute(
            "SELECT addr, difficulty, size_bytes, n_insns FROM functions"
            " WHERE state='queued' ORDER BY difficulty, addr LIMIT 5")]
    finally:
        conn.close()
    print(json.dumps({"imported": res, "skipped": skipped,
                      "by_state": counts, "by_kind": kinds,
                      "queue_head": head}, indent=1))
    return 0


def apply_subsystems(conn, path: str) -> int:
    with open(path, "r", encoding="utf-8") as fh:
        ranges = json.load(fh)
    n = 0
    conn.execute("BEGIN IMMEDIATE")
    try:
        for r in ranges:
            form = r.get("form", "rva")
            lo, hi = lfdb.to_rva(r["from"], form), lfdb.to_rva(r["to"], form)
            cur = conn.execute(
                "UPDATE functions SET subsystem=? WHERE addr>=? AND addr<?",
                (r["subsystem"], lo, hi))
            n += cur.rowcount
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return n


def import_classes(conn, path: str) -> int:
    with open(path, "r", encoding="utf-8") as fh:
        classes = json.load(fh)
    n = 0
    conn.execute("BEGIN IMMEDIATE")
    try:
        for c in classes:
            conn.execute(
                "INSERT OR IGNORE INTO classes (mangled, demangled, namespace,"
                " polymorphic) VALUES (?,?,?,?)",
                (c.get("mangled"), c.get("demangled"),
                 c.get("top_level_namespace") or (c.get("namespaces") or [None])[0],
                 1 if c.get("polymorphic") else 0))
            n += 1
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return n


def import_vtables(conn, path: str) -> int:
    with open(path, "r", encoding="utf-8") as fh:
        vts = json.load(fh)
    id_by_mangled = {r["mangled"]: r["id"]
                     for r in conn.execute("SELECT id, mangled FROM classes")}
    n = 0
    conn.execute("BEGIN IMMEDIATE")
    try:
        for vt in vts:
            cid = id_by_mangled.get(vt.get("class_mangled"))
            if cid is None:
                continue
            # vtables.json: vt_rva is an RVA, vt_va and slot targets are VAs.
            vt_rva = None
            try:
                if vt.get("vt_rva") is not None:
                    vt_rva = lfdb.to_rva(vt["vt_rva"], "rva")
                elif vt.get("vt_va") is not None:
                    vt_rva = lfdb.to_rva(vt["vt_va"], "va")
            except (TypeError, ValueError):
                pass
            for slot, target in enumerate(vt.get("slots", [])):
                try:
                    addr = lfdb.to_rva(target, "va")
                except (TypeError, ValueError):
                    continue
                conn.execute(
                    "INSERT OR REPLACE INTO vtable_slots"
                    " (class_id, slot, addr, vt_rva) VALUES (?,?,?,?)",
                    (cid, slot, addr, vt_rva))
                n += 1
        conn.execute("COMMIT")
    except BaseException:
        conn.execute("ROLLBACK")
        raise
    return n


if __name__ == "__main__":
    raise SystemExit(main())
