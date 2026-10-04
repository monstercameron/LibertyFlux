"""Static checks over the verified rewrites, for what the checker cannot see.

The checker proves one rewrite at a time in a release build with its callees stubbed. These checks look for
what that proof is blind to and what will matter when the rewrites are assembled into one library and run in
the game: a debug build panics where a release build silently wraps or misreads, an export that is not the
code the checker ran, a proof the file itself says is partial.

Every check is a pattern over the text, so each finding says how sure it is: `certain` means the text alone
proves it, `likely` means a heuristic that can misfire and wants a look. Per-file findings are listed;
patterns found in most files (integers as pointers, plain dereferences) are counted once as systemic.

Structural checks look for proofs narrower than they seem: a truncated or unbalanced file (`unbalanced`, it
cannot be what the checker ran), words saying part of the code is a placeholder (`placeholder`; todo!,
unimplemented! and unreachable! stay under `partial`), a callee's answer discarded while the function returns
a constant although its doc says the original returns that answer (`constant-for-callee-result`), and
arguments that never reach the callee (`unused-parameter`, `forwarder-drops-args`, `args-gap` for script
natives). On the verified tree of 4 October 2026 they fired on 4, 3, 6, 4, 0 and 0 files.

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


# --- Structure: balance, the export body, its final expression -------------------------------------------

CLOSERS = {")": "(", "]": "[", "}": "{"}
OPENERS = {v: k for k, v in CLOSERS.items()}
CHAR_LITERAL = re.compile(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]{1,6}\}|.)|[^\\'\n])'")
RAW_STRING = re.compile(r"b?r(#*)\"")


def balance_problems(text):
    """Why the file is not a complete Rust token stream: an unterminated string or block comment, a closing
    delimiter that does not match, or delimiters still open at the end (a truncated file ends mid-item or
    mid-macro). Comments, strings, raw strings, byte strings, char literals and lifetimes are skipped. Pure;
    returns a list of sentences, empty when the file is balanced."""
    stack, i, n, line = [], 0, len(text), 1
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
        elif text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, start, i = 1, line, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    line += text[i] == "\n"
                    i += 1
            if depth:
                return [f"block comment from line {start} never ends"]
        elif c in "br" and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")) and RAW_STRING.match(text, i):
            match = RAW_STRING.match(text, i)
            end = text.find('"' + match.group(1), match.end())
            if end < 0:
                return [f"raw string from line {line} never ends"]
            line += text.count("\n", i, end)
            i = end + 1 + len(match.group(1))
        elif c == '"' or (c == "b" and text.startswith('b"', i) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))):
            start, i = line, i + (2 if c == "b" else 1)
            while i < n and text[i] != '"':
                if text[i] == "\\":
                    i += 1
                if i < n and text[i] == "\n":
                    line += 1
                i += 1
            if i >= n:
                return [f"string from line {start} never ends"]
            i += 1
        elif c == "'":
            match = CHAR_LITERAL.match(text, i)
            i = match.end() if match else i + 1  # otherwise a lifetime or a loop label
        else:
            if c in OPENERS:
                stack.append((c, line))
            elif c in CLOSERS:
                if not stack:
                    return [f"unmatched '{c}' at line {line}"]
                opener, opened = stack.pop()
                if opener != CLOSERS[c]:
                    return [f"'{opener}' opened at line {opened} is closed by '{c}' at line {line}"]
            i += 1
    if stack:
        opener, opened = stack[0]
        return [f"{len(stack)} delimiter(s) still open at the end of the file, the outermost '{opener}' from line {opened}"]
    return []


def matching(code, i):
    """Index of the delimiter closing the one at `i` (scanning forward), or None. Comments must already be
    stripped; brackets inside string literals are rare enough in rewrites to ignore."""
    opener = code[i]
    closer, depth = OPENERS[opener], 0
    for j in range(i, len(code)):
        if code[j] == opener:
            depth += 1
        elif code[j] == closer:
            depth -= 1
            if depth == 0:
                return j
    return None


def matching_back(code, j):
    """Index of the delimiter opening the one at `j` (scanning backward), or None."""
    closer = code[j]
    opener, depth = CLOSERS[closer], 0
    for i in range(j, -1, -1):
        if code[i] == closer:
            depth += 1
        elif code[i] == opener:
            depth -= 1
            if depth == 0:
                return i
    return None


def split_top(text):
    """Comma-separated items at nesting depth zero."""
    items, depth, current = [], 0, ""
    for ch in text:
        depth += ch in "([{"
        depth -= ch in ")]}"
        if ch == "," and depth == 0:
            items.append(current.strip())
            current = ""
        else:
            current += ch
    if current.strip():
        items.append(current.strip())
    return items


def export_parts(code):
    """(parameter names, body text) of the first export, from comment-stripped code; None when the export
    cannot be delimited (the balance check reports why)."""
    match = EXPORT.search(code)
    if not match:
        return None
    params_end = matching(code, match.end() - 1)
    if params_end is None:
        return None
    body_start = code.find("{", params_end)
    body_end = matching(code, body_start) if body_start >= 0 else None
    if body_end is None:
        return None
    names = []
    for item in split_top(code[match.end():params_end]):
        name = item.split(":")[0].strip()
        names.append(re.sub(r"^mut\s+", "", name))
    return names, code[body_start + 1:body_end]


def final_expression(body):
    """The expression a body evaluates to, looking through `unsafe { }` wrappers and a trailing
    `return X;`. None when the value comes from a branching expression (if/match/loop) or a nested block."""
    body = body.strip()
    for _ in range(16):
        if body.endswith("}"):
            start = matching_back(body, len(body) - 1)
            if start is None:
                return None
            before = body[:start].rstrip()
            outer = before[:-len("unsafe")].rstrip() if before.endswith("unsafe") else None
            if before == "" or (outer is not None and (outer == "" or outer[-1] in ";{}")):
                body = body[start + 1:-1].strip()
                continue
            return None
        depth, cut = 0, 0
        for i, ch in enumerate(body):
            if ch in "([{":
                depth += 1
            elif ch in ")]}":
                depth -= 1
                if depth == 0 and ch == "}":
                    cut = i + 1
            elif ch == ";" and depth == 0:
                cut = i + 1
        tail = body[cut:].strip()
        if tail:
            return tail
        statements = [s for s in re.split(r";\s*", body) if s.strip()]
        match = re.match(r"\s*return\s+(.*)$", statements[-1], re.S) if statements else None
        return match.group(1).strip() if match else ""
    return None


# --- Narrow-proof patterns the checker cannot see -------------------------------------------------------

CALLEE_CALL = re.compile(r"(?:[\w:]+::)?callee_\w+!\(")
INT_LITERAL = re.compile(r"(?:0x[0-9a-fA-F_]+|\d[\d_]*)(?:u8|u16|u32|i32|usize)?")
CONTROL_FLOW = re.compile(r"\b(?:if|match|loop|while|for)\b")
# The doc says the original's return value is a callee's answer, or what a call left in EAX/ST0.
RETURNS_CALLEE = re.compile(
    r"returns?\s+(?:value\s+is\s+)?whatever"
    r"|(?:return\s+value|exit\s+eax|eax\s+on\s+exit|eax\s+at\s+exit)\s+is\s+(?:the\s+)?(?:last\s+)?(?:[\w-]+\s+)?"
    r"(?:callee|helper|call|stub|worker)(?:'s)?\s+(?:answer|result)"
    r"|returns?\s+(?:the\s+)?(?:last\s+)?(?:[\w-]+(?:'s)?\s+)?(?:callee|helper|worker|stub|call)(?:'s)?\s+(?:answer|result|return\s+value)"
    r"|\bleaves?\s+(?:whatever|the\s+(?:last\s+)?[\w-]+\s+(?:answer|result))[^.;]{0,40}?\bin\s+(?:eax|st0)"
    r"|\bleft\s+in\s+(?:eax|st0|the\s+floating-point\s+result\s+register)"
    r"|\bas\s+a\s+placeholder\b", re.I)
NUMBER_WORDS = {"one": 1, "single": 1, "two": 2, "both": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7,
                "eight": 8, "nine": 9, "ten": 10, "eleven": 11, "twelve": 12}
FORWARDS_N = re.compile(r"\bforwards?\s+(?:the\s+|its\s+|all\s+|both\s+(?=\w+\s))?(\d+|" + "|".join(NUMBER_WORDS) + r")\s+"
                        r"(?:\w+\s+){0,3}?(?:arguments?|args?|words?|values?|handles?|parameters?|integers?|floats?|ints?|ids?|pointers?)\b", re.I)
# Script-native argument reads: args.add(N), (args + bytes), *args, args.read(), (args as *const T), args[N].
NATIVE_ARG_INDEX = re.compile(r"\bargs\.(?:add|offset)\(\s*(0x[0-9a-fA-F]+|\d+)\s*(?:as\s+\w+\s*)?\)|\bargs\[(\d+)\]")
NATIVE_ARG_BYTES = re.compile(r"\(\s*args\s*\+\s*(0x[0-9a-fA-F]+|\d+)\s*\)|\bargs\.wrapping_add\(\s*(0x[0-9a-fA-F]+|\d+)\s*\)")
NATIVE_ARG_ZERO = re.compile(r"\*\s*args\b(?!\s*[.+])|\bargs\.read(?:_unaligned)?\(\)|\(\s*args\s+as\s+\*(?:const|mut)\s+\w+\s*\)")
# Words that say the code itself is unfinished. "stub" alone is not one: rewrites call the checker's recorder
# stubs constantly. "placeholder" in a naming note (an inventory placeholder name or token) is not one either.
PLACEHOLDER = re.compile(
    r"\bTODO\b|\bFIXME\b|\bXXX\b"
    r"|\bplaceholder\b(?!\s+(?:name|token|merged|symbol|coordinate))(?<!merged\ssymbol;\splaceholder)"
    r"|\b(?:this|the)\s+(?:rewrite|function|export|body|implementation)\s+is\s+(?:a\s+|only\s+a\s+)?stub\b"
    r"|\bstub\s+(?:implementation|rewrite|body|version)\b|\bstubbed[- ]out\b|\bnot\s+(?:yet\s+)?implemented\b")
PLACEHOLDER_NAMING = re.compile(r"placeholder\s+merged\s+name|merged\s+symbol;\s*placeholder", re.I)


def discarded_calls(body):
    """Callee calls whose (non-unit) result is thrown away: `let _ = callee!(...);` or a bare statement."""
    found = []
    for match in CALLEE_CALL.finditer(body):
        close = matching(body, match.end() - 1)
        if close is None:
            continue
        args = split_top(body[match.end():close])
        if len(args) < 2 or args[1].replace(" ", "") == "()":
            continue
        lead = body[body.rfind("\n", 0, match.start()) + 1:match.start()]
        if not body[close + 1:].lstrip().startswith(";"):
            continue
        if re.fullmatch(r"\s*let\s+_\w*\s*(?::\s*[\w()]+\s*)?=\s*", lead) or re.fullmatch(r"\s*(?:unsafe\s*\{\s*)?", lead):
            found.append(args[0])
    return found


def is_literal(expr):
    return bool(INT_LITERAL.fullmatch(expr.strip()))


def doc_text(text):
    return " ".join(line.strip().lstrip("/!").strip() for line in text.splitlines() if line.strip().startswith("//"))


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
    lint_structure(text, code_only, path, add)
    temps = set(TEMPS.findall(code_only))
    if len(temps) >= 3 or len(set(REGISTER_LOCAL.findall(code_only))) >= 3:
        add("transliteration", "medium", "rule", "likely", "now", "Decompiler-style or register-named locals",
            "Names such as uVar/local_/param_ or eax/ecx as variables suggest a line-by-line transliteration; rule 1 wants a Rust rewrite. Review it.")
    return found


def lint_structure(text, code_only, path, add):
    """The checks that need the file's structure: truncation, placeholders, a callee's result replaced by a
    constant, and arguments that never reach the callee. Calibrated on the verified tree (see the tests)."""
    problems = balance_problems(text)
    if problems:
        add("unbalanced", "high", "integration", "certain", "now", "Truncated or unbalanced file",
            f"{problems[0]}. The tracked file cannot compile, so it is not the code the checker ran.")
    for match in re.finditer(r"export!\(", code_only):
        close = matching(code_only, match.end() - 1)
        if close is not None and not code_only[close + 1:].lstrip().startswith(";"):
            add("unbalanced", "high", "integration", "certain", "now", "Export macro without its closing semicolon",
                "An item-position `export!( ... )` needs a trailing `;`; the file does not compile as tracked.")
    placeholder = next((m for m in PLACEHOLDER.finditer(text)
                        if not PLACEHOLDER_NAMING.search(text[max(0, m.start() - 30):m.end() + 20])), None)
    if placeholder:
        add("placeholder", "high", "narrow-proof", "likely", "now", "The file says part of it is a placeholder",
            f"Found `{placeholder.group(0)}`: a value or path the rewrite does not really implement; say what the proof does not cover, or finish it.")
    parts = export_parts(code_only)
    if parts is None:
        return
    params, body = parts
    final = final_expression(body)
    if final and is_literal(final):
        returns = [r.strip() for r in re.findall(r"\breturn\s+([^;]+);", body)]
        dropped = discarded_calls(body)
        claim = RETURNS_CALLEE.search(doc_text(text))
        if dropped and claim and all(is_literal(r) for r in returns):
            add("constant-for-callee-result", "high", "narrow-proof", "likely", "now", "A callee's result is replaced by a constant",
                f"The doc says `{claim.group(0)}`, but the rewrite discards the answer of callee {', '.join(list(dict.fromkeys(dropped))[:4])}{' and others' if len(set(dropped)) > 4 else ''} and always returns {final}; "
                "the original's return value is not reproduced, so a caller that reads it would differ (and the contract must not compare it).")
    native = "natives/" in path.replace("\\", "/") or params == ["ctx"]
    unused = [p for p in params if p and not p.startswith("_") and not (native and p == "ctx")
              and not re.search(r"\b" + re.escape(p) + r"\b", body)]
    calls = list(CALLEE_CALL.finditer(body))
    forwarder = len(calls) == 1 and not CONTROL_FLOW.search(body)
    if unused:
        detail = f"Parameter(s) {', '.join(unused)} never used: an argument the original passes on may have been dropped"
        if forwarder:
            close = matching(body, calls[0].end() - 1)
            if close is not None and any(is_literal(a) for a in split_top(body[calls[0].end():close])[2:]):
                detail += " (the one callee call passes a literal where it might go)"
        add("unused-parameter", "medium", "narrow-proof", "likely", "now", "A parameter is never used", detail + ".")
    if forwarder:
        claim = FORWARDS_N.search(doc_text(text))
        close = matching(body, calls[0].end() - 1)
        if claim and close is not None:
            word = claim.group(1).lower()
            wanted = int(word) if word.isdigit() else NUMBER_WORDS[word]
            passed = [a for a in split_top(body[calls[0].end():close])[2:] if not is_literal(a)]
            if len(passed) < wanted:
                add("forwarder-drops-args", "high", "narrow-proof", "likely", "now", "Forwarder passes fewer arguments than its doc says",
                    f"The doc says `{claim.group(0)}` but the call passes {len(passed)} non-literal argument(s).")
    if native:
        indexes = {int(a or b, 0) for a, b in NATIVE_ARG_INDEX.findall(body)}
        indexes |= {int(a or b, 0) // 4 for a, b in NATIVE_ARG_BYTES.findall(body)}
        if NATIVE_ARG_ZERO.search(body):
            indexes.add(0)
        gaps = sorted(set(range(max(indexes) + 1)) - indexes) if indexes else []
        if gaps:
            add("args-gap", "medium", "narrow-proof", "likely", "now", "A script argument between used ones is never read",
                f"Reads script arguments {sorted(indexes)} but not {gaps}: a skipped argument, unless the native ignores it.")


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
