"""Write the per-function context packet for a lane.

Usage:
    python context.py --addr 0xRVA --lane LANE [--db PATH] [--exe PATH]
                      [--imports PATH] [--names PATH] [--out DIR]
                      [--max-insns N]

Writes DIR/0x<rva>.json (default DIR: .artifacts/scratch/<lane>/context/)
with: the queue row (size, metrics, difficulty, name, subsystem), the
capstone disassembly of the function bytes (private material, stays under
.artifacts), resolved references (direct callees with names where known,
import calls via the IAT map, data/string references with names where
known), the RTTI class and virtual slot if it is a virtual method, the most
similar accepted functions (stubbed: [] with a stable interface, see
lfdb.find_similar), name history, and the best earlier attempt if any.

--exe defaults to orig/GTAIV.exe. --imports is imports.json (uses the 'iat'
field per import). --names is name_hints.json (string_address -> hint).
--max-insns caps disassembly (default 4000). Without --exe bytes
(--no-disasm) the packet is still written, with an empty listing, so the
rest of the pipeline stays testable without the binary.
"""

from __future__ import annotations

import argparse
import json
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import lfdb

IMAGE_BASE = lfdb.IMAGE_BASE


# --------------------------------------------------------------------------
# Minimal PE section reader (stdlib only): enough to map RVA -> file offset.
# --------------------------------------------------------------------------

def pe_sections(exe_path: str) -> list[dict]:
    with open(exe_path, "rb") as fh:
        dos = fh.read(64)
        if dos[:2] != b"MZ":
            raise ValueError("not a PE file: %s" % exe_path)
        (pe_off,) = struct.unpack_from("<I", dos, 0x3C)
        fh.seek(pe_off)
        if fh.read(4) != b"PE\0\0":
            raise ValueError("bad PE signature: %s" % exe_path)
        coff = fh.read(20)
        (_, n_sec, _, _, _, opt_size, _) = struct.unpack("<HHIIIHH", coff)
        fh.seek(pe_off + 4 + 20 + opt_size)
        sections = []
        for _ in range(n_sec):
            raw = fh.read(40)
            name = raw[:8].rstrip(b"\0").decode("ascii", "replace")
            vsize, vaddr, rawsize, rawptr = struct.unpack_from("<IIII", raw, 8)
            flags = struct.unpack_from("<I", raw, 36)[0]
            sections.append({"name": name, "vaddr": vaddr, "vsize": vsize,
                             "rawptr": rawptr, "rawsize": rawsize,
                             "exec": bool(flags & 0x20000000)})
        return sections


def rva_to_offset(sections: list[dict], rva: int) -> int | None:
    for s in sections:
        span = max(s["vsize"], s["rawsize"])
        if s["vaddr"] <= rva < s["vaddr"] + span:
            off = s["rawptr"] + (rva - s["vaddr"])
            if off < s["rawptr"] + s["rawsize"]:
                return off
            return None
    return None


def read_bytes(exe_path: str, sections: list[dict], rva: int, size: int) -> bytes:
    off = rva_to_offset(sections, rva)
    if off is None:
        raise ValueError("RVA %s not in any section" % lfdb.rva_hex(rva))
    with open(exe_path, "rb") as fh:
        fh.seek(off)
        return fh.read(size)


def section_of(sections: list[dict], rva: int) -> str | None:
    for s in sections:
        if s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rawsize"]):
            return s["name"]
    return None


# --------------------------------------------------------------------------
# Disassembly (capstone, loaded lazily so --no-disasm and the test suite
# work without it).
# --------------------------------------------------------------------------

def disassemble(code: bytes, rva: int, max_insns: int) -> list[dict]:
    try:
        from capstone import Cs, CS_ARCH_X86, CS_MODE_32
    except ImportError:
        raise RuntimeError("capstone is not installed; use --no-disasm")
    md = Cs(CS_ARCH_X86, CS_MODE_32)
    md.detail = True
    out = []
    for insn in md.disasm(code, IMAGE_BASE + rva):
        ops = []
        try:
            for op in insn.operands:
                if op.type == 2:  # X86_OP_IMM
                    ops.append({"type": "imm", "value": op.imm & 0xFFFFFFFF})
                elif op.type == 3:  # X86_OP_MEM
                    m = op.mem
                    ops.append({"type": "mem", "base": m.base, "index": m.index,
                                "scale": m.scale, "disp": m.disp & 0xFFFFFFFF})
        except Exception:
            pass
        out.append({"addr": "0x%x" % insn.address, "size": insn.size,
                    "mnemonic": insn.mnemonic, "op_str": insn.op_str,
                    "operands": ops})
        if len(out) >= max_insns:
            break
    return out


def resolve_refs(conn, sections, insns: list[dict], rva: int,
                 iat_map: dict[int, str], str_map: dict[int, str]) -> dict:
    """Classify immediates/displacements: callee, import, string or data."""
    callees: dict[int, dict] = {}
    imports: dict[int, dict] = {}
    data: dict[int, dict] = {}
    func_addrs = {r["addr"] for r in conn.execute("SELECT addr FROM functions")}
    names = {r["addr"]: r["name"] for r in
             conn.execute("SELECT addr, name FROM functions WHERE name IS NOT NULL")}
    for ins in insns:
        vals = []
        for op in ins["operands"]:
            if op["type"] == "imm":
                vals.append(op["value"])
            elif op["type"] == "mem" and op["base"] == 0 and op["index"] == 0:
                vals.append(op["disp"])
        # E8 direct call: target = next_ip + rel32
        if ins["mnemonic"] == "call" and ins["operands"] and \
                ins["operands"][0]["type"] == "imm":
            nxt = int(ins["addr"], 16) + ins["size"]
            rel = ins["operands"][0]["value"]
            if rel & 0x80000000:
                rel -= 0x100000000
            vals.append((nxt + rel) & 0xFFFFFFFF)
        for v in vals:
            if v in iat_map:
                imports[v] = {"iat": "0x%x" % v, "import": iat_map[v]}
                continue
            if IMAGE_BASE <= v < IMAGE_BASE + 0x2000000:
                tgt = v - IMAGE_BASE
                if tgt in func_addrs:
                    callees[tgt] = {"addr": lfdb.rva_hex(tgt),
                                    "name": names.get(tgt)}
                elif tgt in str_map:
                    data[tgt] = {"addr": lfdb.rva_hex(tgt), "kind": "string",
                                 "hint": str_map[tgt]}
                else:
                    sec = section_of(sections, tgt)
                    if sec:
                        data[tgt] = {"addr": lfdb.rva_hex(tgt), "kind": "data",
                                     "section": sec}
    return {"callees": sorted(callees.values(), key=lambda d: d["addr"]),
            "imports": sorted(imports.values(), key=lambda d: d["iat"]),
            "data": sorted(data.values(), key=lambda d: d["addr"])}


def load_iat_map(path: str | None) -> dict[int, str]:
    if not path:
        return {}
    with open(path, "r", encoding="utf-8") as fh:
        imports = json.load(fh)
    out = {}
    for imp in imports:
        iat = imp.get("iat")
        if not iat:
            continue
        try:
            out[int(iat, 16)] = "%s!%s" % (imp.get("dll", "?"),
                                           imp.get("function", "?"))
        except (ValueError, TypeError):
            continue
    return out


def load_str_map(path: str | None) -> dict[int, str]:
    if not path:
        return {}
    with open(path, "r", encoding="utf-8") as fh:
        hints = json.load(fh)
    out = {}
    for h in hints:
        try:
            va = int(h["string_address"], 16)
            if va < IMAGE_BASE:  # already an RVA in some exports
                rva = va
            else:
                rva = va - IMAGE_BASE
        except (KeyError, ValueError, TypeError):
            continue
        text = h.get("string_text", "")
        if len(text) > 120:
            text = text[:120] + "..."
        out[rva] = "%s [%s/%s]" % (
            text, h.get("suggested_name", "?"), h.get("confidence", "?"))
    return out


def build_packet(conn, addr: int, exe: str | None, iat_map, str_map,
                 max_insns: int) -> dict:
    row = conn.execute("SELECT * FROM functions WHERE addr=?", (addr,)).fetchone()
    if row is None:
        raise ValueError("unknown function %s" % lfdb.rva_hex(addr))
    row = dict(row)
    packet = {
        "addr": lfdb.rva_hex(addr),
        "function": {k: row[k] for k in (
            "size_bytes", "n_insns", "branches", "calls_direct",
            "calls_indirect", "switches", "fp_insns", "sha1_masked", "kind",
            "subsystem", "name", "name_confidence", "state", "difficulty",
            "source")},
        "disassembly": [],
        "disasm_truncated": False,
        "references": {"callees": [], "imports": [], "data": []},
        "class_slots": lfdb.class_slots_for(conn, addr),
        "similar_accepted": lfdb.find_similar(addr, conn),
        "name_history": [dict(r) for r in conn.execute(
            "SELECT name, source, confidence, created_at, superseded"
            " FROM names WHERE addr=? ORDER BY id", (addr,))],
        "best_attempt": lfdb.best_attempt(conn, addr),
        "sources": {"exe": bool(exe), "imports": bool(iat_map),
                    "names": bool(str_map)},
    }
    if exe:
        sections = pe_sections(exe)
        size = max(row["size_bytes"] or 0, 1)
        code = read_bytes(exe, sections, addr, min(size, 1 << 20))
        insns = disassemble(code, addr, max_insns)
        packet["disassembly"] = insns
        packet["disasm_truncated"] = len(insns) >= max_insns
        packet["references"] = resolve_refs(conn, sections, insns, addr,
                                            iat_map, str_map)
    return packet


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--addr", required=True)
    ap.add_argument("--lane", required=True)
    ap.add_argument("--db", default=None)
    ap.add_argument("--exe", default=None)
    ap.add_argument("--no-disasm", action="store_true")
    ap.add_argument("--imports", default=None)
    ap.add_argument("--names", default=None)
    ap.add_argument("--out", default=None)
    ap.add_argument("--max-insns", type=int, default=4000)
    args = ap.parse_args(argv)
    root = lfdb.repo_root()
    exe = None if args.no_disasm else (
        args.exe or str(root / "orig" / "GTAIV.exe"))
    outdir = Path(args.out) if args.out else (
        root / ".artifacts" / "scratch" / args.lane / "context")
    outdir.mkdir(parents=True, exist_ok=True)
    conn = lfdb.connect(args.db)
    try:
        addr = lfdb.resolve_addr(conn, args.addr)
        packet = build_packet(conn, addr, exe, load_iat_map(args.imports),
                              load_str_map(args.names), args.max_insns)
    finally:
        conn.close()
    dest = outdir / ("%s.json" % lfdb.rva_hex(addr))
    with open(dest, "w", encoding="utf-8") as fh:
        json.dump(packet, fh, indent=1, sort_keys=True)
    print(str(dest))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
