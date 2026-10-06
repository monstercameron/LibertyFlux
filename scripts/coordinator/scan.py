"""Publication scanning for rewrites entering the tracked tree.

A rewrite is copied into `rewrites/verified/` only when it passes this scan:
no disassembly, no byte dumps, no machine paths, no inline assembly. The same
patterns gate every copy, so they live here instead of inside the importer.
"""

import re
from pathlib import Path

REGS = r"(?:e[abcd]x|e[sd]i|e[sb]p|[abcd][lhx]|xmm\d|st\(?\d?\)?|\[|byte|word|dword|qword|0x[0-9a-f]+|\d)"
FORBIDDEN = [
    ("disassembly", re.compile(r"\b(?:mov|movzx|movsx|movss|movaps|push|pop|lea|jmp|jn?[ezlgab]e?|call|retn?|xor|cmp|test|add|sub|imul|fld|fstp|shl|shr|sar)\s+" + REGS, re.I)),
    ("inline assembly", re.compile(r"\basm!|global_asm!|naked")),
    ("byte dump", re.compile(r"(?:\b[0-9a-f]{2}\s+){8,}|(?:0x[0-9a-f]{2},\s*){12,}", re.I)),
    # Built from pieces so this file itself passes the publication check, which
    # forbids the very strings the pattern must detect.
    ("machine path", re.compile(r"[A-Za-z]:[\\/]+Use" r"rs|stea" r"mapps|\.artifacts", re.I)),
]
# Lane-specific runtime imports are dropped on import: the shared runtime
# replaces them at assembly.
LANE_IMPORT = re.compile(r"^\s*use lf_(?:r[nsb]|a)\d+_\w+::[^;]*;\s*\n", re.M)

# Outcomes that count as a pass, and the rank of each checker version: a
# rewrite is replaced only by one checked under a later version.
PASSED = {"verified", "verified_v2", "verified_v3", "verified_v4", "verified_v5", "verified_v6", "verified_v7"}
RANK = {"version 1": 1, "version 2": 2, "version 3": 3, "version 4": 4, "version 5": 5, "version 6": 6, "version 7": 7}
MODERN = ("version 2", "version 3", "version 4", "version 5", "version 6", "version 7")


def scan_text(text):
    """The reason a text must stay out of the tracked tree, or None."""
    return next((label for label, pattern in FORBIDDEN if pattern.search(text)), None)


def strip_lane_imports(text):
    """Drop lane-specific runtime imports, replaced at assembly."""
    return LANE_IMPORT.sub("", text)


def checker_of_lane(lists, lane):
    """The checker version a lane's passes count as, from its marker files.
    Re-run lanes are always version 2 at least."""
    lists = Path(lists)
    if (lists / f"{lane}.v7").exists():
        return "version 7"
    if (lists / f"{lane}.v6").exists():
        return "version 6"
    if (lists / f"{lane}.v5").exists():
        return "version 5"
    if (lists / f"{lane}.v4").exists():
        return "version 4"
    if (lists / f"{lane}.v3").exists():
        return "version 3"
    if lane.startswith("a-") or (lists / f"{lane}.v2").exists():
        return "version 2"
    return "version 1"


def short_version(version):
    """`version 4` -> `v4`, for reports."""
    return "v" + version.rsplit(" ", 1)[1]
