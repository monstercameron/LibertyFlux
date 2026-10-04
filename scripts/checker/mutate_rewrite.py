"""mutate_rewrite.py: generate deliberately wrong versions of a rewrite, to find contracts that cannot see a change.

Rule 3 asks for a wrong version of every function, run through the same contract, which must fail. One
hand-written wrong version shows the contract sees one change. This script writes many, each with one
Rust-level change chosen to reveal a weak contract, so a run through the checker on the laptop (local check:
it needs the original executable) maps what each contract can and cannot see. A mutant that passes is a
blind spot: widen the contract, or record the function as narrowed or deferred.

Mutation operators (one change per mutant):
  cmp-flip        a comparison operator replaced by its negation (== and !=, < and >=, > and <=)
  const-inc       an integer literal plus one
  const-dec       an integer literal minus one (nonzero literals only)
  const-bitflip   an integer literal with one bit flipped (its highest set bit, or bit 1 below 4)
  drop-write      one memory write statement deleted (*p = v; *p |= v; .write(..); write_unaligned(..); store_*(..))
  swap-args       two neighbouring arguments of a callee call swapped
  drop-call       a callee call removed (deleted when its answer is unused, else replaced by the type's default)
  width-change    a write's width changed (u32 to u16, u16 to u8, u8 to u16, signed alike) with the value cast to it
  invert-branch   an if condition negated
  float-swap      the operands of a float + or * swapped (same value except the NaN payload, which x87/SSE
                  take from a fixed operand: the bit-exact rule of rule 3)

Skipped on purpose: callee ids (an undeclared id faults, which any contract catches), array lengths (usually
an equivalent mutant), literals in comments and strings, and literals whose change would not compile
(overflow past 32 bits). Every mutant is checked to parse (balanced delimiters, exactly one export, named
mut_<address>_<k>); one that does not is dropped and counted. Compilation is not checked here (local check).

Usage (from the repository root):
  python3 scripts/checker/mutate_rewrite.py FILE [FILE ...] --out DIR [-n 8] [--seed 0]
      writes DIR/<file stem>/mut_<address>_<k>.rs and DIR/<file stem>/manifest.json
  python3 scripts/checker/mutate_rewrite.py FILE ... --dry-run [--json]
      counts candidates and mutants per file without writing
Output stays under .artifacts/ by convention (rule: all build output and scratch work go there).
"""

import argparse
import json
import os
import random
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "coordinator"))
from lint_rewrites import CHAR_LITERAL, EXPORT, balance_problems, matching  # noqa: E402

OPERATORS = ("cmp-flip", "const-inc", "const-dec", "const-bitflip", "drop-write", "swap-args", "drop-call",
             "width-change", "invert-branch", "float-swap")
# The order mutants are drawn in, one operator per turn: the changes a weak contract most often misses
# (a dropped write or call, swapped or narrowed arguments, a flipped branch) before literal tweaks.
PICK_ORDER = ("drop-write", "swap-args", "drop-call", "width-change", "invert-branch", "cmp-flip", "float-swap",
              "const-inc", "const-bitflip", "const-dec")
DEFAULT_MUTANTS = 8
U32_MAX = 0xFFFF_FFFF

CMP_FLIP = {"==": "!=", "!=": "==", "<": ">=", ">=": "<", ">": "<=", "<=": ">"}
CMP = re.compile(r"(?<=\s)(==|!=|<=|>=|<|>)(?=\s)")
INT = re.compile(r"(?<![\w.])(0x[0-9a-fA-F_]+|\d[\d_]*)(u8|u16|u32|u64|usize|i8|i16|i32|i64|isize)?(?![\w.])")
CALLEE = re.compile(r"(?:[\w:]+::)?callee_\w+!\(")
HEADER = re.compile(r"//\s*original:\s*(0x[0-9A-Fa-f]+)")
FILE_ADDRESS = re.compile(r"fn_([0-9a-fA-F]{8})\.rs$")
WIDER_NARROWER = {"u32": "u16", "u16": "u8", "u8": "u16", "i32": "i16", "i16": "i8", "i8": "i16"}
FLOAT_VAR = re.compile(r"\b(\w+)\s*:\s*f(?:32|64)\b|let\s+(?:mut\s+)?(\w+)\s*=\s*(?:f(?:32|64)::from_bits|[^;\n]*\bas\s+f(?:32|64)\b|-?\d+\.\d+)")
CALL_ARGS = r"\((?:[^()]|\([^()]*\))*\)"  # one level of nested parentheses
OPERAND = (r"(?:(?:core::hint::)?black_box" + CALL_ARGS + r"|f(?:32|64)::from_bits" + CALL_ARGS
           + r"|[A-Za-z_][\w.]*(?:" + CALL_ARGS + r")?|\d+\.\d+(?:f32|f64)?)")
FLOAT_BINOP = re.compile(r"(" + OPERAND + r")(\s*)([+*])(?!=)(\s*)(" + OPERAND + r")")


def masked(text):
    """`text` with comments, string contents and char literals blanked to spaces (newlines kept), so
    patterns match only code and every offset still points into the original."""
    out = list(text)
    i, n = 0, len(text)

    def blank(a, b):
        for k in range(a, b):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            blank(i, j)
            i = j
        elif text[i] == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            blank(i + 1, min(j, n))
            i = j + 1
        elif text[i] == "'" and CHAR_LITERAL.match(text, i):
            end = CHAR_LITERAL.match(text, i).end()
            blank(i + 1, end - 1)
            i = end
        else:
            i += 1
    return "".join(out)


def address_of(text, path):
    match = HEADER.search(text)
    if match:
        return int(match.group(1), 16)
    match = FILE_ADDRESS.search(os.path.basename(path))
    return int(match.group(1), 16) if match else None


def statement_end(code, start):
    """Index just past the `;` ending the statement that starts at `start` (depth-aware), or None when a
    closing brace of the enclosing block comes first."""
    depth = 0
    for j in range(start, len(code)):
        ch = code[j]
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
            if depth < 0:
                return None
        elif ch == ";" and depth == 0:
            return j + 1
    return None


def format_like(original, value, suffix):
    """`value` written in the style of the literal `original` (hex case and digit grouping kept loosely)."""
    if original.lower().startswith("0x"):
        digits = original[2:].replace("_", "")
        body = format(value, "X" if any(c.isupper() for c in digits) else "x")
        return "0x" + body + (suffix or "")
    return str(value) + (suffix or "")


# --- candidate sites: (operator, start, end, replacement, note) on the original text -----------------------

def cmp_sites(code):
    for m in CMP.finditer(code):
        yield ("cmp-flip", m.start(), m.end(), CMP_FLIP[m.group(1)], m.group(1))


def const_sites(code):
    for m in INT.finditer(code):
        before = code[max(0, m.start() - 80):m.start()]
        if re.search(r"callee_\w+!\(\s*$", before) or (re.match(r"\s*\]", code[m.end():]) and re.search(r";\s*$", before)):
            continue  # a callee id, or an array length
        if re.search(r"#\[[^\]\n]*$", before):
            continue  # inside an attribute
        literal, suffix = m.group(1), m.group(2)
        value = int(literal.replace("_", ""), 0)
        wide = suffix in ("u64", "i64", "usize", "isize")
        if value < U32_MAX or wide:
            yield ("const-inc", m.start(), m.end(), format_like(literal, value + 1, suffix), m.group(0))
        if value > 0:
            yield ("const-dec", m.start(), m.end(), format_like(literal, value - 1, suffix), m.group(0))
        bit = value.bit_length() - 1 if value >= 4 else 1
        flipped = value ^ (1 << bit)
        if flipped <= U32_MAX or wide:
            yield ("const-bitflip", m.start(), m.end(), format_like(literal, flipped, suffix), m.group(0))


def statement_starts(code):
    """Offsets where a statement can begin: the first code after `;`, `{` or `}` (and the file start).
    A line that continues an expression from the line before is never one, which keeps a deletion from
    cutting a statement in half."""
    starts = []
    for m in re.finditer(r"(?:^|[;{}])\s*", code):
        if m.end() < len(code):
            starts.append(m.end())
    return starts


# Helper names that write memory in rewrites (store_u32, wr32, write_bytes, put_word, ...); not wrapping_*.
WRITER_CALL = re.compile(r"(?:(?:core::)?ptr::write(?:_unaligned|_volatile)?|store\w*|write\w*|put\w*|wr(?:8|16|32|64|_\w+)?)\(")
ASSIGN_OP = re.compile(r"(?<![=!<>|&^+\-*/%])(?:[|&^+\-*/%]|<<|>>)?=(?!=)")


def top_level(stmt, pattern):
    """The first match of `pattern` at bracket depth zero in `stmt`, or None."""
    depth = 0
    for i, ch in enumerate(stmt):
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif depth == 0:
            m = pattern.match(stmt, i)
            if m:
                return m
    return None


def write_statements(code):
    """(start, end) of whole statements that write memory: `*place = v;` (or a compound assignment),
    `(place as *mut T).write(v);` and its unaligned/volatile forms, `ptr::write(p, v);`, and a statement
    that is one call to a writer helper."""
    for start in statement_starts(code):
        head = code[start:start + 16]
        if re.match(r"(?:let|fn|pub|return|if|else|match|while|for|loop|use|const|static|struct|impl|mod|#)\b", head):
            continue
        end = statement_end(code, start)
        if end is None:
            continue
        stmt = code[start:end]
        if stmt.startswith("*"):
            if top_level(stmt, ASSIGN_OP):
                yield start, end
        elif stmt.startswith("("):
            close = matching(stmt, 0)
            rest = stmt[close + 1:] if close is not None else ""
            call = re.match(r"\s*\.write(?:_unaligned|_volatile)?\(", rest)
            if call and matching(rest, call.end() - 1) == len(rest.rstrip()) - 2:
                yield start, end
        else:
            call = WRITER_CALL.match(stmt)
            if call and matching(stmt, call.end() - 1) == len(stmt.rstrip()) - 2:
                yield start, end


def drop_write_sites(code):
    for start, end in write_statements(code):
        yield ("drop-write", start, end, "", code[start:end])


def width_sites(code):
    for start, end in write_statements(code):
        stmt = code[start:end]
        m = re.match(r"\*\s*\(", stmt)
        if m:  # *(E as *mut T) = V;
            close = matching(stmt, m.end() - 1)
            if close is None:
                continue
            target = re.search(r"\bas\s*\*mut\s+(u8|u16|u32|i8|i16|i32)\s*$", stmt[m.end():close])
            assign = re.match(r"\s*=\s*", stmt[close + 1:])
            if target and assign:
                narrow = WIDER_NARROWER[target.group(1)]
                value = stmt[close + 1 + assign.end():-1].strip()
                new = (stmt[:m.end() + target.start(1)] + narrow + stmt[m.end() + target.end(1):close + 1]
                       + f" = (({value}) as {target.group(1)}) as {narrow};")
                yield ("width-change", start, end, new, f"{target.group(1)} -> {narrow}")
            continue
        m = re.match(r"\(", stmt)
        if m:  # (E as *mut T).write(V);  .write_unaligned(V)
            close = matching(stmt, 0)
            if close is None:
                continue
            target = re.search(r"\bas\s*\*mut\s+(u8|u16|u32|i8|i16|i32)\s*$", stmt[1:close])
            call = re.match(r"\.(write(?:_unaligned|_volatile)?)\(", stmt[close + 1:])
            if target and call:
                open_at = close + 1 + call.end() - 1
                end_call = matching(stmt, open_at)
                if end_call is None or stmt[end_call + 1:].strip() != ";":
                    continue
                narrow = WIDER_NARROWER[target.group(1)]
                value = stmt[open_at + 1:end_call]
                new = (stmt[:1 + target.start(1)] + narrow + stmt[1 + target.end(1):open_at + 1]
                       + f"(({value}) as {target.group(1)}) as {narrow})" + ";")
                yield ("width-change", start, end, new, f"{target.group(1)} -> {narrow}")


def callee_calls(code):
    """(start, open paren, close paren, args, arg spans) of each callee call."""
    for m in CALLEE.finditer(code):
        close = matching(code, m.end() - 1)
        if close is None:
            continue
        spans, depth, item_start = [], 0, m.end()
        for j in range(m.end(), close + 1):
            ch = code[j]
            if ch in "([{":
                depth += 1
            elif ch in ")]}":
                if j == close and depth == 0:
                    spans.append((item_start, j))
                    break
                depth -= 1
            elif ch == "," and depth == 0:
                spans.append((item_start, j))
                item_start = j + 1
        spans = [(a + len(code[a:b]) - len(code[a:b].lstrip()), b - (len(code[a:b]) - len(code[a:b].rstrip())))
                 for a, b in spans if code[a:b].strip()]
        yield m.start(), m.end() - 1, close, [code[a:b] for a, b in spans], spans


def swap_sites(code):
    for _, _, _, args, spans in callee_calls(code):
        for i in range(2, len(args) - 1):
            if args[i] != args[i + 1]:
                (a0, a1), (b0, b1) = spans[i], spans[i + 1]
                new = code[b0:b1] + code[a1:b0] + code[a0:a1]
                yield ("swap-args", a0, b1, new, f"{args[i]} <-> {args[i + 1]}")
                break


def drop_call_sites(code):
    starts = statement_starts(code)
    for start, _, close, args, _ in callee_calls(code):
        if len(args) < 2:
            continue
        statement = next((s for s in reversed(starts) if s <= start), None)
        lead = code[statement:start] if statement is not None else None
        after = code[close + 1:]
        if lead is not None and re.fullmatch(r"(?:let\s+_\w*\s*(?::\s*[\w()]+\s*)?=\s*)?", lead) and after.lstrip().startswith(";"):
            end = close + 1 + (len(after) - len(after.lstrip())) + 1
            yield ("drop-call", statement, end, "", code[start:close + 1])
        else:
            ret = args[1].replace(" ", "")
            if ret == "_":
                continue
            yield ("drop-call", start, close + 1, f"<{ret}>::default()" if ret != "()" else "()", code[start:close + 1])


def invert_sites(code):
    for m in re.finditer(r"\bif\s+(?!let\b)", code):
        depth = 0
        for j in range(m.end(), len(code)):
            ch = code[j]
            if ch in "([" or (ch == "{" and re.search(r"\bunsafe\s*$", code[m.end():j])):
                depth += 1  # an `unsafe { .. }` block inside the condition is part of it
            elif ch in ")]" or (ch == "}" and depth > 0):
                depth -= 1
            elif ch == "{" and depth == 0:
                cond = code[m.end():j].rstrip()
                if cond:
                    yield ("invert-branch", m.end(), m.end() + len(cond), f"!({cond})", cond)
                break
            elif ch in ";}" and depth == 0:
                break


def float_sites(code):
    floats = {a or b for a, b in FLOAT_VAR.findall(code)}
    for m in FLOAT_BINOP.finditer(code):
        left, op, right = m.group(1), m.group(3), m.group(5)
        floaty = lambda s: (s.split("(")[0].split(".")[0] in floats or "black_box" in s or "from_bits" in s
                            or re.fullmatch(r"\d+\.\d+(?:f32|f64)?", s))
        if not (floaty(left) or floaty(right)) or left == right:
            continue
        before = code[:m.start()].rstrip()
        after = code[m.end():].lstrip()
        if not (before.endswith(("(", "=", ",", "{")) or re.search(r"\breturn$", before)) or not after[:1] in (")", ";", ",", "}"):
            continue
        if before.endswith(("==", "!=", "<=", ">=")):
            continue
        yield ("float-swap", m.start(), m.end(), right + m.group(2) + op + m.group(4) + left, f"{left} {op} {right}")


SITE_FINDERS = (cmp_sites, const_sites, drop_write_sites, swap_sites, drop_call_sites, width_sites, invert_sites, float_sites)


def candidates(text):
    """Every candidate mutation of `text`, grouped by operator, in source order."""
    code = masked(text)
    by_op = {op: [] for op in OPERATORS}
    for finder in SITE_FINDERS:
        for op, start, end, new, note in finder(code):
            if text[start:end] != new:
                by_op[op].append((start, end, new, note))
    return by_op


def export_name(code):
    names = [(m.group(2) or m.group(4)) for m in EXPORT.finditer(code)]
    real = [n for n in names if not re.match(r"(?:mut_\w+|\w+_mut)$", n)]
    return real, names


def apply(text, start, end, new, old_name, new_name):
    """The mutated text with the export renamed (whole-word, code positions only)."""
    mutated = text[:start] + new + text[end:]
    code = masked(mutated)
    parts, last = [], 0
    for m in re.finditer(r"\b" + re.escape(old_name) + r"\b", code):
        parts.append(mutated[last:m.start()] + new_name)
        last = m.end()
    parts.append(mutated[last:])
    return "".join(parts)


def parses(text, new_name):
    """True when the mutant is balanced and defines exactly one export, the renamed one."""
    if balance_problems(text):
        return False
    _, names = export_name(masked(text))
    return names == [new_name]


def line_col(text, offset):
    line = text.count("\n", 0, offset) + 1
    return line, offset - (text.rfind("\n", 0, offset) + 1) + 1


def mutate(text, path, count=DEFAULT_MUTANTS, seed=0):
    """Up to `count` parsing mutants of one rewrite, operators taken in turn so the set is diverse.
    Returns a manifest dict; each mutant entry carries its text under "text"."""
    address = address_of(text, path)
    code = masked(text)
    real, names = export_name(code)
    manifest = {"source": path, "address": None if address is None else f"0x{address:08X}", "export": None,
                "mutants": [], "candidates": {}, "dropped_unparsable": 0, "skipped": None}
    if address is None:
        manifest["skipped"] = "no address (no `// original:` header and no fn_<address>.rs name)"
        return manifest
    if len(real) != 1 or len(names) != 1:
        manifest["skipped"] = f"needs exactly one export, found {names}"
        return manifest
    if balance_problems(text):
        manifest["skipped"] = "the source itself is unbalanced"
        return manifest
    old_name = real[0]
    manifest["export"] = old_name
    by_op = candidates(text)
    manifest["candidates"] = {op: len(sites) for op, sites in by_op.items()}
    rng = random.Random((seed << 32) ^ address)
    queues = {op: rng.sample(sites, len(sites)) for op, sites in by_op.items()}
    k = 0
    while len(manifest["mutants"]) < count and any(queues.values()):
        for op in PICK_ORDER:
            if len(manifest["mutants"]) >= count:
                break
            while queues[op]:
                start, end, new, note = queues[op].pop()
                new_name = f"mut_{address:08x}_{k}"
                mutated = apply(text, start, end, new, old_name, new_name)
                if not parses(mutated, new_name):
                    manifest["dropped_unparsable"] += 1
                    continue
                line, col = line_col(text, start)
                manifest["mutants"].append({"export": new_name, "file": f"{new_name}.rs", "operator": op,
                                            "line": line, "column": col, "before": text[start:end],
                                            "after": new, "note": note, "text": mutated})
                k += 1
                break
    return manifest


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("files", nargs="+")
    parser.add_argument("--out", help="output folder (one subfolder per rewrite)")
    parser.add_argument("-n", type=int, default=DEFAULT_MUTANTS, help="mutants per rewrite (default %(default)s)")
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--dry-run", action="store_true", help="count, write nothing")
    parser.add_argument("--json", action="store_true", help="print the manifests (without mutant texts) as JSON")
    args = parser.parse_args(argv)
    if not args.dry_run and not args.out:
        parser.error("--out is required unless --dry-run")
    summary = []
    for path in args.files:
        with open(path, encoding="utf-8", errors="replace") as fh:
            text = fh.read()
        manifest = mutate(text, path.replace(os.sep, "/"), args.n, args.seed)
        if not args.dry_run and manifest["mutants"]:
            folder = os.path.join(args.out, os.path.splitext(os.path.basename(path))[0])
            os.makedirs(folder, exist_ok=True)
            for m in manifest["mutants"]:
                with open(os.path.join(folder, m["file"]), "w", encoding="utf-8", newline="\n") as fh:
                    fh.write(m["text"])
            with open(os.path.join(folder, "manifest.json"), "w", encoding="utf-8", newline="\n") as fh:
                json.dump(strip_texts(manifest), fh, indent=1)
                fh.write("\n")
        summary.append(strip_texts(manifest))
    if args.json:
        print(json.dumps(summary, indent=1))
    else:
        produced = [len(m["mutants"]) for m in summary]
        ops = {}
        for m in summary:
            for mut in m["mutants"]:
                ops[mut["operator"]] = ops.get(mut["operator"], 0) + 1
        skipped = [m for m in summary if m["skipped"]]
        print(f"{len(summary)} rewrites, {sum(produced)} mutants ({min(produced, default=0)}-{max(produced, default=0)} per file), "
              f"{sum(m['dropped_unparsable'] for m in summary)} dropped as unparsable, {len(skipped)} skipped")
        for op in OPERATORS:
            print(f"  {op:14} {ops.get(op, 0)}")
        for m in skipped:
            print(f"  skipped {m['source']}: {m['skipped']}")
    return 0


def strip_texts(manifest):
    out = dict(manifest)
    out["mutants"] = [{k: v for k, v in m.items() if k != "text"} for m in manifest["mutants"]]
    return out


if __name__ == "__main__":
    sys.exit(main())
