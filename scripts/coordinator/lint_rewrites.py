"""Static checks over the verified rewrites, for what the checker cannot see.

The checker proves one rewrite at a time in a release build with its callees stubbed. These checks look for
what that proof is blind to and what will matter when the rewrites are assembled into one library and run in
the game: a debug build panics where a release build silently wraps or misreads, an export that is not the
code the checker ran, a proof the file itself says is partial.

Every check is a pattern over the text, so each finding says how sure it is: `certain` means the text alone
proves it, `likely` means a heuristic that can misfire and wants a look. Per-file findings are listed;
patterns found in most files (integers as pointers, plain dereferences) are counted once as systemic.

Usage: python lint_rewrites.py                 summary
       python lint_rewrites.py --json          every finding as JSON on stdout
       python lint_rewrites.py --write PATH    write the issue log (findings plus systemic counts) to PATH
"""

import argparse
import json
import os
import re
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

SIZES = {"u16": 2, "i16": 2, "u32": 4, "i32": 4, "f32": 4, "u64": 8, "i64": 8, "f64": 8}
WIDE = r"(u16|i16|u32|i32|f32|u64|i64|f64)"
NUMBER = r"(0x[0-9a-fA-F_]+|\d[\d_]*)"

EXPORT = re.compile(r"export!\(\s*(\w+)\s*,\s*(\w+)\s*\("
                    r"|#\[(?:unsafe\()?no_mangle\)?\]\s*pub\s+(?:unsafe\s+)?extern\s+\"(\w+)\"\s+fn\s+(\w+)\s*\(")
HEADER = re.compile(r"//\s*original:\s*(0x[0-9A-Fa-f]+)\b")
WRONG_VERSION = re.compile(r"\bfn\s+mut_\w+|export!\(\s*\w+\s*,\s*(?:mut_\w+|\w+_mut)\s*\(")
PARTIAL = re.compile(r"\bunreachable!|\bstage\s*1\b|\bsubset\b|\btodo!|\bunimplemented!", re.I)
# A plain (aligned) dereference through a byte pointer or an integer address at a constant offset.
BYTE_PTR = re.compile(r"let\s+(?:mut\s+)?(\w+)\s*(?::\s*\*(?:mut|const)\s*u8)?\s*=\s*[^;]*as\s*\*(?:mut|const)\s*u8\s*;"
                      r"|(\w+)\s*:\s*\*(?:mut|const)\s*u8\b")
INT_VAR = re.compile(r"\b(\w+)\s*:\s*u32\b")
DEREF_ADD = re.compile(r"\*\s*\(\s*(\w+)\.(?:add|wrapping_add|offset)\(\s*" + NUMBER + r"\s*(?:as\s+usize)?\)\s*as\s*\*(?:mut|const)\s*" + WIDE + r"\b")
DEREF_INT = re.compile(r"\*\s*\(\s*\(?\s*(\w+)\s*(?:\+\s*|\.wrapping_add\(\s*)" + NUMBER + r"\s*\)?\s*as\s*\*(?:mut|const)\s*" + WIDE + r"\b")
PLAIN_DEREF = re.compile(r"\*\s*\(?[\w.()]+\s+as\s+\*(?:mut|const)\s+" + WIDE + r"\b|\bptr::(?:read|write)\(|\.read\(\)|\.write\(")
UNALIGNED = re.compile(r"(?:read|write)_unaligned")
FLOAT_DECL = re.compile(r"\b(\w+)\s*:\s*f(?:32|64)\b|let\s+(?:mut\s+)?(\w+)\s*=\s*f(?:32|64)::from_bits")
FLOAT_ARITH = re.compile(r"\bf(?:32|64)\b[^;\n]*[-+*/]\s*[\w(]|[-+*/]\s*[^;\n]*\bf(?:32|64)::from_bits")
CHECKER_ONLY = re.compile(r"\b(xmm_word|tls_slot|CHECKER_\w+)\b")
# The mapped image spans 0x400000 to just past 0x1100000 (code, then data); preferred base 0x400000.
IMAGE_LITERAL = re.compile(r"\b0x0*(?:[4-9a-fA-F][0-9a-fA-F]{5}|1[01][0-9a-fA-F]{5})\b")
TEMPS = re.compile(r"\b(?:uVar|iVar|fVar|local_|param_|puVar|piVar|pcVar|bVar)\w*")
REGISTER_LOCAL = re.compile(r"\blet\s+(?:mut\s+)?(e[abcd]x|e[sd]i|ebp)\b")


def offset_value(text):
    return int(text.replace("_", ""), 0)


def lint_text(text, path, address=None):
    """Findings for one rewrite. Pure. Each finding is a dict with code, severity, category, confidence,
    when, title and detail; `address` (the index's) lets the header and export name be compared to it."""
    found = []
    code_only = re.sub(r"//[^\n]*", "", text)

    def add(code, severity, category, confidence, when, title, detail):
        found.append({"file": path, "code": code, "severity": severity, "category": category, "confidence": confidence,
                      "when": when, "title": title, "detail": detail})

    exports = [(m[0] or m[2], m[1] or m[3]) for m in EXPORT.findall(text)]
    real = [name for _, name in exports if not re.match(r"(?:mut_\w+|\w+_mut)$", name)]
    if not exports:
        add("no-export", "high", "integration", "certain", "now", "No exported function",
            "The file defines no checker export, so it is not in the form the checker loads; what was run is not what is tracked.")
    elif len(real) > 1:
        add("several-exports", "medium", "integration", "certain", "post-bring-up", f"{len(real)} exports in one file",
            "Assembly expects one export per original function.")
    if WRONG_VERSION.search(text):
        add("wrong-version-tracked", "medium", "rule", "certain", "now", "A deliberately wrong version is in the tracked file",
            "Wrong versions belong with the contract, not in the verified tree; it would be assembled into the library.")
    if address is not None:
        for _, name in exports:
            match = re.match(r"rw_([0-9a-fA-F]{8})$", name)
            if name in real and not (match and int(match.group(1), 16) == address):
                add("export-name", "low", "integration", "certain", "post-bring-up", "Export not named rw_<address>",
                    f"Export `{name}`; a canonical name per address keeps the assembled library free of collisions and makes the switch table mechanical.")
        header = next((line.strip() for line in text.splitlines() if line.strip()), "")
        match = HEADER.match(header)
        if not match or int(match.group(1), 16) != address:
            add("header", "low", "quality", "certain", "now", "Header does not name the function's address", "The first line should be `// original: 0x<address> <name>`.")
    for match in PARTIAL.finditer(text):
        add("partial", "high", "narrow-proof", "likely", "now", "The file marks part of the function as not covered",
            f"Found `{match.group(0)}`: a verified rewrite that leaves branches unimplemented or proven only in part is counted as fully verified.")
        break

    byte_ptrs = {a or b for a, b in BYTE_PTR.findall(text)}
    int_vars = set(INT_VAR.findall(text))
    misaligned = []
    for pattern, bases in ((DEREF_ADD, byte_ptrs), (DEREF_INT, int_vars)):
        for match in pattern.finditer(text):
            if match.group(1) in bases and offset_value(match.group(2)) % min(SIZES[match.group(3)], 4):
                misaligned.append(f"{match.group(3)} at +{match.group(2)} from `{match.group(1)}`")
    if misaligned:
        add("misaligned-deref", "high", "ub", "certain", "post-bring-up", "Aligned dereference at an odd offset",
            "Plain dereference of " + ", ".join(sorted(set(misaligned))) + ": undefined behaviour in Rust whenever the base is aligned, and a "
            "panic (abort inside the game) in debug builds, which check alignment. Use read_unaligned/write_unaligned.")

    floats = {a or b for a, b in FLOAT_DECL.findall(text)}
    if (floats or FLOAT_ARITH.search(text)) and re.search(r"\bf(?:32|64)\b", text):
        arithmetic = FLOAT_ARITH.search(text) or any(re.search(rf"\b{re.escape(v)}\s*[-+*/]|[-+*/]\s*{re.escape(v)}\b", text) for v in floats)
        if arithmetic and "black_box" not in text:
            add("float-order", "low", "float", "likely", "lift", "Float arithmetic without pinned order",
                "Passed bit for bit in the checker's build; rule 3 asks for core::hint::black_box so another profile or a refactor cannot reassociate it.")
        casts = [v for v in floats if re.search(rf"\b{re.escape(v)}\s+as\s+(?:i|u)(?:8|16|32|64)\b", text)]
        if casts:
            add("float-to-int", "medium", "suspicious-logic", "likely", "post-bring-up", "Float converted to an integer with `as`",
                f"`as` saturates and maps NaN to 0, where x86 truncation yields 0x80000000; equal only if the inputs never overflow (variables: {', '.join(sorted(casts))}).")
    names = sorted(set(CHECKER_ONLY.findall(text)))
    if names:
        add("checker-only", "medium", "integration", "certain", "post-bring-up", "Uses a checker-only mechanism",
            f"{', '.join(names)} exist only in the checker's runtime; the assembled library needs a production equivalent.")
    # Masks such as 0x7FFFFF or 0x800000 (float mantissa and exponent bits) are not addresses.
    literals = [m.group(0) for m in IMAGE_LITERAL.finditer(code_only)
                if (v := int(m.group(0), 16)) & (v - 1) and (v + 1) & v]
    if literals and not re.search(r"\b(?:relocated|global|xbase)\b", code_only):
        add("image-literal", "medium", "rule", "likely", "now", "Image-range number used without relocated()/global()",
            f"Values such as {', '.join(sorted(set(literals))[:3])} look like original addresses; if any is used as one, it breaks once the image moves.")
    temps = set(TEMPS.findall(code_only))
    if len(temps) >= 3 or len(set(REGISTER_LOCAL.findall(code_only))) >= 3:
        add("transliteration", "medium", "rule", "likely", "now", "Decompiler-style or register-named locals",
            "Names such as uVar/local_/param_ or eax/ecx as variables suggest a line-by-line transliteration; rule 1 wants a Rust rewrite. Review it.")
    return found


def systemic(texts):
    """Patterns that run through most of the corpus, counted once instead of listed per file."""
    plain = sum(1 for t in texts if PLAIN_DEREF.search(t))
    unaligned = sum(1 for t in texts if UNALIGNED.search(t))
    return [
        {"code": "plain-deref", "files": plain, "severity": "high", "category": "ub", "when": "post-bring-up",
         "title": "Plain dereferences of game memory",
         "detail": "Aligned loads and stores through integers cast to pointers. Undefined behaviour for any address that is not aligned at run "
                   "time, and debug builds check alignment and panic. Build the injected library in release with debug assertions off until "
                   f"these are converted, or convert them to read_unaligned/write_unaligned ({unaligned} files already do)."},
        {"code": "debug-overflow", "files": len(texts), "severity": "medium", "category": "panic", "when": "post-bring-up",
         "title": "Integer overflow behaviour depends on the build profile",
         "detail": "The checker builds release, where + - * and shifts wrap as the original's do. A debug or overflow-checked build of the "
                   "same code panics instead. Set overflow-checks = false for the library profile, or convert to wrapping operations."},
    ]


def lint_tree(root):
    folder = root / "rewrites" / "verified"
    index = json.loads((folder / "index.json").read_text(encoding="utf-8"))
    findings, texts = [], []
    for entry in index:
        path = folder / entry["file"]
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        texts.append(text)
        findings.extend(lint_text(text, entry["file"], common.va(entry["address"])))
    return findings, systemic(texts), len(texts)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--write", metavar="PATH")
    args = parser.parse_args(argv)
    findings, broad, checked = lint_tree(common.find_root())
    if args.json:
        print(json.dumps({"checked": checked, "findings": findings, "systemic": broad}, indent=1))
    if args.write:
        with open(args.write, "w", encoding="utf-8", newline="\n") as fh:
            json.dump({"checked": checked, "findings": findings, "systemic": broad}, fh, indent=1)
            fh.write("\n")
    if not args.json:
        print(f"checked {checked} rewrites; {len(findings)} findings in {len({f['file'] for f in findings})} files")
        for (code, severity, confidence), n in sorted(Counter((f["code"], f["severity"], f["confidence"]) for f in findings).items()):
            print(f"  {code:22} {severity:6} {confidence:7} {n}")
        for item in broad:
            print(f"  systemic {item['code']}: {item['files']} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
