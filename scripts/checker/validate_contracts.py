"""validate_contracts.py: check checker contracts against contract_schema.json before they are run.

The driver (checker2.py) and the worker read contract keys with defaults and ignore every key they do not
know, so a misspelt key ("global" for "globals", "check" for "checks") drops an observation without a word,
and a value of the wrong type can be read as its default (an `fp_tol` given as a number is read as zero
tolerance; a callee id of 256 lands past the 256-entry callee table). This script makes those mistakes loud.

It reports three kinds of finding for each contract:
  errors    the contract is malformed: an unknown key, a wrong type, a value out of range, a reference to
            a callee or heap segment that is not declared, a limit the worker would reject at setup.
            A contract with an error should not be run.
  warnings  narrowing: settings that make the proof cover less than the whole function, listed by name
            (rule 3 asks that every one is stated in the function's result). A narrowed contract is
            valid; the warning is the list of what its proof does not cover.
  notes     facts worth a look that neither break nor narrow the contract.

Schema: contract_schema.json beside this file. Only the subset of JSON Schema the file uses is implemented
here (standard library only; no jsonschema package): type, enum, const, properties, required,
additionalProperties, patternProperties, propertyNames, items, minItems, maxItems, minLength, minimum,
maximum, multipleOf, pattern, anyOf, oneOf, $ref to "#/definitions/...", plus the extension keyword
x-discriminator (a oneOf over objects picked by one key's value, for clear messages). Keys marked x-pending
are accepted loosely; they belong to checker extensions still being written.

Limits that live in the worker (snapshot words per callee, out-param words per callee, logged arguments
per callee, answer-sequence steps, call-log cap) are read from the worker's source each run, so a worker
change to a cap moves the validator with it; the defaults below apply when the source is absent.

Usage (from the repository root):
  python3 scripts/checker/validate_contracts.py                  every scripts/checker/contracts/*.json
  python3 scripts/checker/validate_contracts.py PATH [PATH ...]  files, or folders of *.json
  python3 scripts/checker/validate_contracts.py --json ...       findings as JSON on stdout
  python3 scripts/checker/validate_contracts.py --quiet ...      only contracts with errors
Exit status 1 when any contract has an error, 0 otherwise (warnings never fail).
"""

import argparse
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCHEMA_PATH = os.path.join(HERE, "contract_schema.json")
WORKER_SOURCE = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                             "crates", "tools", "lf-checker-worker", "src", "main.rs")
CONTRACTS_DIR = os.path.join(HERE, "contracts")

# Worker limits (crates/tools/lf-checker-worker/src/main.rs), used when the source cannot be read.
DEFAULT_LIMITS = {
    "SNAP_MAXW": 64,        # snapshot words per callee (snap::SNAP_CAP_WORDS since v5; 8 before)
    "WRITEW_PER_ID": 16,    # out-param words per callee
    "LOG_MAXW": 40,         # stack arguments logged per call
    "SEQ_MAX": 16,          # per-call answer steps per callee
    "LOG_HARD_MAX": 1024,   # call-log cap ceiling
    "HEAP_USE": 0xF0000,    # usable trial heap bytes
    "SNAP_AT_LIMIT": 0x10000,  # largest |at| of an offset snapshot (v5)
    "X87_MAX_ENTRIES": 8,   # x87 entry values (v5)
}
CALLEE_TABLE_ENTRIES = 256  # the callee table is 1024 bytes of 4-byte stub addresses
TLS_SLOTS = 64
DEFAULT_MIN_ORIG_OK_SHARE = 0.10
FEW_TRIALS = 100

# Checks that default to on in the worker; switching one off narrows the proof.
BEHAVIOUR_CHECKS = ("esp", "heap", "stack", "globals", "calls", "undeclared", "fault", "x87_state")
# Registers a callee's call comparison includes by default, by callee convention.
DEFAULT_CALL_REGS = {"thiscall": ("ecx",), "fastcall": ("ecx", "edx")}


def worker_limits(source_path=WORKER_SOURCE):
    """The worker's caps, read from `const NAME: usize = VALUE;` lines of its source; defaults otherwise.
    A cap defined as another constant (`const SNAP_MAXW: usize = snap::SNAP_CAP_WORDS;`) is resolved
    through the literal constants of every .rs file beside the main source."""
    limits = dict(DEFAULT_LIMITS)
    try:
        with open(source_path, encoding="utf-8") as fh:
            text = fh.read()
    except OSError:
        return limits
    literals = {}
    folder = os.path.dirname(source_path)
    for name in sorted(os.listdir(folder)) if os.path.isdir(folder) else []:
        if name.endswith(".rs"):
            try:
                with open(os.path.join(folder, name), encoding="utf-8") as fh:
                    other = fh.read()
            except OSError:
                continue
            for m in CONST_LINE.finditer(other):
                value = literal_value(m.group(2))
                if value is not None:
                    literals.setdefault(m.group(1), value)
    defined = {m.group(1): m.group(2) for m in CONST_LINE.finditer(text)}
    for name in limits:
        expr = defined.get(name)
        if expr is None:
            expr = name if name in literals else None
        if expr is None:
            continue
        value = literal_value(expr)
        if value is None:
            value = literals.get(expr.split("::")[-1].strip())
        if value is not None:
            limits[name] = value
    return limits


CONST_LINE = re.compile(r"\bconst\s+([A-Z][A-Z0-9_]*)\s*:\s*(?:usize|u32|i64|i32)\s*=\s*([^;]+);")


def literal_value(expr):
    """The integer a Rust literal expression spells, or None."""
    expr = expr.strip()
    if re.fullmatch(r"(0x[0-9A-Fa-f_]+|[0-9_]+)", expr):
        return int(expr.replace("_", ""), 0)
    return None


def load_schema(path=SCHEMA_PATH):
    with open(path, encoding="utf-8") as fh:
        return json.load(fh)


# --------------------------------------------------------------------------------------------------------
# The JSON Schema subset
# --------------------------------------------------------------------------------------------------------

TYPES = {
    "object": lambda v: isinstance(v, dict),
    "array": lambda v: isinstance(v, list),
    "string": lambda v: isinstance(v, str),
    "integer": lambda v: isinstance(v, int) and not isinstance(v, bool),
    "number": lambda v: isinstance(v, (int, float)) and not isinstance(v, bool),
    "boolean": lambda v: isinstance(v, bool),
    "null": lambda v: v is None,
}


def json_path(parts):
    out = "$"
    for p in parts:
        out += f"[{p}]" if isinstance(p, int) else f".{p}"
    return out


class SchemaValidator:
    """Validates an instance against the schema subset. Errors are (code, path, message) triples."""

    def __init__(self, schema):
        self.root = schema

    def resolve(self, schema):
        while "$ref" in schema:
            ref = schema["$ref"]
            if not ref.startswith("#/"):
                raise ValueError(f"unsupported $ref {ref}")
            node = self.root
            for part in ref[2:].split("/"):
                node = node[part]
            schema = node
        return schema

    def validate(self, instance, schema=None, path=()):
        schema = self.resolve(self.root if schema is None else schema)
        errors = []
        err = lambda code, msg, p=path: errors.append((code, json_path(p), msg))

        if "type" in schema:
            wanted = schema["type"] if isinstance(schema["type"], list) else [schema["type"]]
            if not any(TYPES[t](instance) for t in wanted):
                err("type", f"expected {' or '.join(wanted)}, got {type(instance).__name__} {short(instance)}")
                return errors
        if "const" in schema and not same_value(instance, schema["const"]):
            err("enum", f"expected {json.dumps(schema['const'])}, got {short(instance)}")
            return errors
        if "enum" in schema and not any(same_value(instance, e) for e in schema["enum"]):
            err("enum", f"{short(instance)} is not one of {', '.join(json.dumps(e) for e in schema['enum'])}")
            return errors
        if TYPES["number"](instance):
            if "minimum" in schema and instance < schema["minimum"]:
                err("range", f"{instance} is below the minimum {schema['minimum']}")
            if "maximum" in schema and instance > schema["maximum"]:
                err("range", f"{instance} is above the maximum {schema['maximum']}")
            if "multipleOf" in schema and instance % schema["multipleOf"]:
                err("range", f"{instance} is not a multiple of {schema['multipleOf']}")
        if isinstance(instance, str):
            if "minLength" in schema and len(instance) < schema["minLength"]:
                err("range", "string is too short")
            if "pattern" in schema and not re.search(schema["pattern"], instance):
                err("pattern", f"{short(instance)} does not match {schema['pattern']}")
        if isinstance(instance, list):
            if "minItems" in schema and len(instance) < schema["minItems"]:
                err("range", f"{len(instance)} items, at least {schema['minItems']} required")
            if "maxItems" in schema and len(instance) > schema["maxItems"]:
                err("range", f"{len(instance)} items, at most {schema['maxItems']} allowed")
            if "items" in schema:
                for i, item in enumerate(instance):
                    errors.extend(self.validate(item, schema["items"], path + (i,)))
        if isinstance(instance, dict):
            errors.extend(self.validate_object(instance, schema, path))
        if "anyOf" in schema:
            errors.extend(self.validate_alternatives(instance, schema["anyOf"], path, exactly_one=False))
        if "oneOf" in schema:
            if "x-discriminator" in schema and isinstance(instance, dict):
                errors.extend(self.validate_discriminated(instance, schema, path))
            else:
                errors.extend(self.validate_alternatives(instance, schema["oneOf"], path, exactly_one=True))
        return errors

    def validate_object(self, instance, schema, path):
        errors = []
        props = schema.get("properties", {})
        patterns = schema.get("patternProperties", {})
        for key in schema.get("required", []):
            if key not in instance:
                errors.append(("required", json_path(path), f"missing required key \"{key}\""))
        names = schema.get("propertyNames")
        for key, value in instance.items():
            if names is not None:
                errors.extend((code, json_path(path + (key,)), "key " + msg)
                              for code, _, msg in self.validate(key, names, path + (key,)))
            if key in props:
                errors.extend(self.validate(value, props[key], path + (key,)))
                continue
            matched = [p for p in patterns if re.search(p, key)]
            for p in matched:
                errors.extend(self.validate(value, patterns[p], path + (key,)))
            if matched:
                continue
            extra = schema.get("additionalProperties", True)
            if extra is False:
                hint = close_key(key, props)
                errors.append(("unknown-key", json_path(path + (key,)),
                               f"unknown key \"{key}\"" + (f" (did you mean \"{hint}\"?)" if hint else "")
                               + "; the checker ignores it"))
            elif isinstance(extra, dict):
                errors.extend(self.validate(value, extra, path + (key,)))
        return errors

    def validate_alternatives(self, instance, branches, path, exactly_one):
        results = [self.validate(instance, b, path) for b in branches]
        passing = sum(1 for r in results if not r)
        if passing == 1 or (passing > 1 and not exactly_one):
            return []
        if passing > 1:
            return [("form", json_path(path), f"{short(instance)} matches {passing} alternatives, exactly one allowed")]
        root = json_path(path)

        def rank(r):
            shallow = [e for e in r if e[1] == root and e[0] in ("type", "required", "enum")]
            return (len(shallow), len(r))

        best = min(results, key=rank)
        if rank(best)[0]:
            message = f"{short(instance)} matches none of the allowed forms"
            if isinstance(instance, dict):
                known = set()
                for b in branches:
                    known.update(self.resolve(b).get("properties", {}))
                unknown = sorted(k for k in instance if k not in known)
                if unknown:
                    message += "; unknown key " + ", ".join(
                        f"\"{k}\"" + (f" (did you mean \"{close_key(k, known)}\"?)" if close_key(k, known) else "")
                        for k in unknown)
            return [("form", root, message)]
        return best

    def validate_discriminated(self, instance, schema, path):
        key = schema["x-discriminator"]
        value = instance.get(key)
        allowed = []
        for branch in schema["oneOf"]:
            branch = self.resolve(branch)
            spec = branch.get("properties", {}).get(key, {})
            options = spec.get("enum", [spec["const"]] if "const" in spec else [])
            allowed.extend(options)
            if value in options:
                return self.validate(instance, branch, path)
        return [("enum", json_path(path + (key,)),
                 f"{short(value)} is not one of {', '.join(json.dumps(a) for a in allowed)}")]


def same_value(a, b):
    """JSON equality: true is not 1 and 1.0 equals 1."""
    if isinstance(a, bool) or isinstance(b, bool):
        return isinstance(a, bool) and isinstance(b, bool) and a == b
    return a == b


def short(value):
    text = json.dumps(value)
    return text if len(text) <= 40 else text[:37] + "..."


def close_key(key, known):
    """A declared key within one edit (or a plural/singular slip) of `key`, as a typo hint."""
    for k in known:
        if k.rstrip("s") == key.rstrip("s") or edit_distance_le1(k, key):
            return k
    return None


def edit_distance_le1(a, b):
    if abs(len(a) - len(b)) > 1 or a == b:
        return False
    if len(a) == len(b):
        return sum(x != y for x, y in zip(a, b)) == 1 or any(
            a[:i] + a[i + 1] + a[i] + a[i + 2:] == b for i in range(len(a) - 1))
    if len(a) > len(b):
        a, b = b, a
    return any(b[:i] + b[i + 1:] == a for i in range(len(b)))


# --------------------------------------------------------------------------------------------------------
# Cross-references and worker limits (what a schema cannot say)
# --------------------------------------------------------------------------------------------------------

def to_int(value):
    """An integer from an int or a hex/decimal string; None when it is neither."""
    if isinstance(value, bool):
        return None
    if isinstance(value, int):
        return value
    if isinstance(value, str):
        try:
            return int(value, 0)
        except ValueError:
            return None
    return None


def word_specs(spec, path):
    """Every nested word spec under `spec` (cycle/rot recurse), with its path."""
    yield spec, path
    if isinstance(spec, dict):
        for key in ("cycle", "rot"):
            if isinstance(spec.get(key), list):
                for i, inner in enumerate(spec[key]):
                    yield from word_specs(inner, path + (key, i))
        for key in ("lo", "hi"):
            if key in spec:
                yield from word_specs(spec[key], path + (key,))


def all_word_specs(contract):
    """Word specs from every place the driver resolves them: segment words and pins, legacy pokes and
    heap scripts, global values, callee scripts, write scripts and answer sequences."""
    for si, seg in enumerate(as_list(contract.get("heapsegs"))):
        if not isinstance(seg, dict):
            continue
        for wi, w in enumerate(as_list(seg.get("words"))):
            yield from word_specs(w, ("heapsegs", si, "words", wi))
        for pi, pin in enumerate(as_list(seg.get("pin"))):
            if isinstance(pin, dict):
                for vi, v in enumerate(as_list(pin.get("vals"))):
                    yield from word_specs(v, ("heapsegs", si, "pin", pi, "vals", vi))
        for vals in as_list(seg.get("pinned")):  # v7b: q-08 alias [[word,[vals]]], resolved by the driver
            if isinstance(vals, list) and len(vals) == 2:
                for vi, v in enumerate(as_list(vals[1])):
                    yield from word_specs(v, ("heapsegs", si, "pinned", vi))
    for pi, poke in enumerate(as_list(contract.get("pokes"))):
        if isinstance(poke, dict):
            for vi, v in enumerate(as_list(poke.get("vals"))):
                yield from word_specs(v, ("pokes", pi, "vals", vi))
    for hi, sc in enumerate(as_list(contract.get("heap_scripts"))):  # v7b: q-02 alias, resolved by the driver
        if isinstance(sc, dict):
            for vi, v in enumerate(as_list(sc.get("cycle"))):
                yield from word_specs(v, ("heap_scripts", hi, "cycle", vi))
    for gi, g in enumerate(as_list(contract.get("globals_values"))):
        if isinstance(g, dict):
            for wi, w in enumerate(as_list(g.get("words"))):
                yield from word_specs(w, ("globals_values", gi, "words", wi))
    for ci, c in enumerate(as_list(contract.get("callees"))):
        if not isinstance(c, dict):
            continue
        if isinstance(c.get("script"), list):
            for i, s in enumerate(c["script"]):
                yield from word_specs(s, ("callees", ci, "script", i))
        for i, s in enumerate(as_list(c.get("seq"))):
            yield from word_specs(s, ("callees", ci, "seq", i))
        for ri, row in enumerate(as_list(c.get("wscript"))):
            for i, s in enumerate(as_list(row)):
                yield from word_specs(s, ("callees", ci, "wscript", ri, i))


def as_list(value):
    return value if isinstance(value, list) else []


# v7b: the driver's primary word-spec keys (checker2.py WORD_SPEC_KEYS).
# A dict with none of these resolves to random bits exactly as on stock
# (recorded in the verdict's features), refused only with strict: true.
WORD_SPEC_KEYS = frozenset(("stub", "heap", "heap_off", "int", "small",
                             "cycle", "rot", "float", "null", "any"))


def unresolved_word_specs(contract):
    """(path, keys) for every word spec the driver resolves to random bits
    by stock fallback: a dict with no known key, mirroring the driver's
    non-strict walk (a script/seq entry {"double":...} is consumed by the
    caller and {"lo":...} judges its inners as plain specs; anything under
    an already-unresolved dict resolves as part of it)."""
    out = []
    skipped = set()
    for spec, path in all_word_specs(contract):
        if any(path[:i] in skipped for i in range(1, len(path))):
            continue
        if not isinstance(spec, dict):
            continue
        if len(path) == 4 and path[0] == "callees" and path[2] in ("script", "seq") \
                and ("double" in spec or "lo" in spec):
            continue  # a script/seq wrapper, resolved by the caller
        if not any(k in spec for k in WORD_SPEC_KEYS):
            out.append((path, sorted(spec)))
            skipped.add(path)
    return out


def semantic_errors(contract, limits):
    """Cross-reference and limit errors. Assumes nothing about types (the schema pass reports those), so
    every access is guarded."""
    errors = []
    err = lambda path, msg, code="reference": errors.append((code, json_path(path), msg))
    callees = [c for c in as_list(contract.get("callees")) if isinstance(c, dict)]
    by_id = {}
    for ci, c in enumerate(callees):
        cid = c.get("id")
        if not isinstance(cid, int) or isinstance(cid, bool):
            continue
        if cid in by_id:
            err(("callees", ci, "id"), f"callee id {cid} declared twice")
        by_id[cid] = c
    declared = set(by_id)
    segs = as_list(contract.get("heapsegs"))

    def callee_ref(value, path, what):
        cid = to_int(value)
        if cid is None:
            return
        if not 0 <= cid < CALLEE_TABLE_ENTRIES:
            err(path, f"{what} {cid} is outside the {CALLEE_TABLE_ENTRIES}-entry callee table", "range")
        elif cid not in declared:
            err(path, f"{what} {cid} is not a declared callee")

    def seg_ref(value, path):
        if isinstance(value, int) and not isinstance(value, bool) and not 0 <= value < len(segs):
            err(path, f"heap segment {value} does not exist ({len(segs)} declared)")

    for key in ("patches", "tailpatches", "ctailpatches", "iat"):
        for i, p in enumerate(as_list(contract.get(key))):
            if isinstance(p, dict) and "id" in p:
                callee_ref(p["id"], (key, i, "id"), "callee id")
    for i, cid in enumerate(as_list(contract.get("coverage_exempt_callees"))):
        callee_ref(cid, ("coverage_exempt_callees", i), "exempt callee")
    checks = contract.get("checks") if isinstance(contract.get("checks"), dict) else {}
    for key in ("call_regs", "call_skip", "call_mask", "call_low8"):
        table = checks.get(key)
        if isinstance(table, dict):
            for cid in table:
                callee_ref(cid, ("checks", key, cid), f"{key} callee")
    for spec, path in all_word_specs(contract):
        if isinstance(spec, dict) and "stub" in spec:
            callee_ref(spec["stub"], path + ("stub",), "stub id")
        if isinstance(spec, dict) and "heap" in spec and "plus" in spec:
            seg_ref(spec["heap"], path + ("heap",))

    # heap segments: register and stack pointers, links, tls, global heap pointers, legacy aliases
    for i, r in enumerate(as_list(contract.get("regs"))):
        if isinstance(r, dict) and "heap" in r:
            seg_ref(r["heap"], ("regs", i, "heap"))
    for i, s in enumerate(as_list(contract.get("stack"))):
        if isinstance(s, dict) and "seg" in s:
            seg_ref(s["seg"], ("stack", i, "seg"))
        if isinstance(s, dict) and s.get("kind") == "srange":
            lo, hi = s.get("lo"), s.get("hi")
            if isinstance(lo, int) and isinstance(hi, int) and hi < lo:
                err(("stack", i), f"srange hi {hi} is below lo {lo}", "range")
    for i, t in enumerate(as_list(contract.get("tls"))):
        if not isinstance(t, dict):
            continue
        if "slot" in t and "slot_rva" in t:
            err(("tls", i), "give slot or slot_rva, not both (slot_rva wins)", "form")
        if "int" not in t and "seg" not in t:
            err(("tls", i), "needs int or seg", "form")
        if "seg" in t:
            seg_ref(t["seg"], ("tls", i, "seg"))
    for i, g in enumerate(as_list(contract.get("globals_fill_spec"))):
        if isinstance(g, dict):
            forms = [k for k in ("heap_ptr", "cycle", "words") if k in g]
            if len(forms) != 1:
                err(("globals_fill_spec", i), "needs exactly one of heap_ptr, cycle, words", "form")
            if isinstance(g.get("heap_ptr"), dict):
                seg_ref(g["heap_ptr"].get("seg"), ("globals_fill_spec", i, "heap_ptr", "seg"))
    for key in ("pokes", "heap_scripts"):
        for i, p in enumerate(as_list(contract.get(key))):
            if isinstance(p, dict):
                seg_ref(p.get("seg"), (key, i, "seg"))
    for si, seg in enumerate(segs):
        if not isinstance(seg, dict):
            continue
        off, size = seg.get("off"), seg.get("size")
        if isinstance(off, int) and isinstance(size, int) and off + size > limits["HEAP_USE"]:
            err(("heapsegs", si), f"segment ends at 0x{off + size:x}, past the usable heap 0x{limits['HEAP_USE']:x}", "range")
        if isinstance(size, int) and isinstance(seg.get("words"), list) and len(seg["words"]) != size // 4:
            err(("heapsegs", si, "words"), f"{len(seg['words'])} word specs for a {size}-byte segment ({size // 4} needed)", "range")
        for li, link in enumerate(as_list(seg.get("links"))):
            if isinstance(link, dict):
                seg_ref(link.get("to_seg"), ("heapsegs", si, "links", li, "to_seg"))
                at = link.get("at")
                if isinstance(at, int) and isinstance(size, int) and at + 4 > size:
                    err(("heapsegs", si, "links", li, "at"), f"link at {at} is outside the {size}-byte segment", "range")
        for pi, pin in enumerate(as_list(seg.get("pin"))):
            if isinstance(pin, dict) and isinstance(pin.get("at"), int) and isinstance(size, int):
                width = pin.get("size", 4) if pin.get("size") in (1, 2, 4) else 4
                if pin["at"] + width > size or (pin["at"] % 4) + width > 4:
                    err(("heapsegs", si, "pin", pi, "at"), f"pin of {width} bytes at {pin['at']} does not fit one word of the {size}-byte segment", "range")
        for wi in as_list(seg.get("floats_at")):
            if isinstance(wi, int) and isinstance(size, int) and wi >= size // 4:
                err(("heapsegs", si, "floats_at"), f"word index {wi} is outside the segment", "range")

    # callee limits the worker enforces at setup (or silently misbehaves past)
    for ci, c in enumerate(callees):
        nargs = c.get("nargs", 0) if isinstance(c.get("nargs", 0), int) else 0
        if nargs > limits["LOG_MAXW"]:
            err(("callees", ci, "nargs"), f"nargs {nargs} exceeds the worker's logged-argument window {limits['LOG_MAXW']}", "range")
        snap_words = sum(s.get("n", 0) for s in as_list(c.get("snap")) if isinstance(s, dict) and isinstance(s.get("n"), int))
        if snap_words > limits["SNAP_MAXW"]:
            err(("callees", ci, "snap"), f"{snap_words} snapshot words exceed the worker's cap of {limits['SNAP_MAXW']} per callee", "range")
        for wi, w in enumerate(as_list(c.get("writes"))):
            if not isinstance(w, dict):
                continue
            if ("arg" in w) == ("reg" in w):
                err(("callees", ci, "writes", wi), "give exactly one of arg or reg", "form")
            at, n = w.get("at", 0), w.get("n", 0)
            if isinstance(at, int) and isinstance(n, int) and at + n > limits["WRITEW_PER_ID"]:
                err(("callees", ci, "writes", wi), f"words {at}..{at + n} overflow the {limits['WRITEW_PER_ID']}-word out-param buffer", "range")
            if isinstance(w.get("arg"), int) and w["arg"] >= max(nargs, 1) and nargs:
                err(("callees", ci, "writes", wi, "arg"), f"argument {w['arg']} is past nargs {nargs}", "range")
        if c.get("writes") and not c.get("wscript"):
            err(("callees", ci), "writes declared without a wscript to supply the words", "form")
        for si, s in enumerate(as_list(c.get("snap"))):
            if isinstance(s, dict) and s.get("kind", "arg") == "arg" and isinstance(s.get("idx"), int) and nargs and s["idx"] >= nargs:
                err(("callees", ci, "snap", si, "idx"), f"snapshot of argument {s['idx']} is past nargs {nargs}", "range")
        for si, s in enumerate(as_list(c.get("snap"))):
            if isinstance(s, dict) and isinstance(s.get("at"), int) and abs(s["at"]) > limits["SNAP_AT_LIMIT"]:
                err(("callees", ci, "snap", si, "at"), f"offset {s['at']} is beyond the worker's {limits['SNAP_AT_LIMIT']}-byte limit", "range")
        if isinstance(c.get("seq"), list) and len(c["seq"]) > limits["SEQ_MAX"]:
            err(("callees", ci, "seq"), f"{len(c['seq'])} answer steps exceed {limits['SEQ_MAX']}", "range")
        for key in ("xmm0_from_stack", "xmm1_from_stack", "eax_from_stack"):
            if isinstance(c.get(key), int) and nargs and c[key] >= nargs:
                err(("callees", ci, key), f"stack argument {c[key]} is past nargs {nargs}", "range")
        if isinstance(c.get("xmm_from_stack"), dict):
            for reg, idx in c["xmm_from_stack"].items():
                if isinstance(idx, int) and nargs and idx >= nargs:
                    err(("callees", ci, "xmm_from_stack", reg), f"stack argument {idx} is past nargs {nargs}", "range")
        if isinstance(c.get("xmm_from_stack64"), dict):
            logged = set(c.get("logxmm_regs") or [])
            if c.get("logxmm"):
                logged.add(0)
            if c.get("logxmm1"):
                logged.add(1)
            narrow = set(c.get("xmm_from_stack") or {})
            if c.get("xmm0_from_stack") is not None:
                narrow.add("0")
            if c.get("xmm1_from_stack") is not None:
                narrow.add("1")
            for reg, idx in c["xmm_from_stack64"].items():
                if isinstance(idx, int) and nargs and idx + 1 >= nargs:
                    err(("callees", ci, "xmm_from_stack64", reg), f"stack arguments {idx} and {idx + 1} reach past nargs {nargs}", "range")
                if str(reg) in narrow:
                    err(("callees", ci, "xmm_from_stack64", reg), "the same register has a 4-byte transport too (the worker rejects it: pick one)", "form")
                if to_int(reg) is not None and to_int(reg) not in logged:
                    err(("callees", ci, "xmm_from_stack64", reg), "transported register is not logged (the worker rejects it: the argument would be uncompared)", "form")
        cid = c.get("id")
        regs = checks.get("call_regs", {}).get(str(cid)) if isinstance(checks.get("call_regs"), dict) else None
        wants_eax = isinstance(regs, list) and "eax" in regs
        if wants_eax and "eax_from_stack" not in c:
            err(("checks", "call_regs", str(cid)), "selects eax without the callee's eax_from_stack transport (the worker rejects it)", "form")
        if "eax_from_stack" in c and not wants_eax:
            err(("callees", ci, "eax_from_stack"), "eax transport without call_regs selecting eax (the worker rejects it: the argument would be uncompared)", "form")
        if isinstance(c.get("logxmm64_regs"), list):
            logged = set(c.get("logxmm_regs") or [])
            if c.get("logxmm"):
                logged.add(0)
            if c.get("logxmm1"):
                logged.add(1)
            for reg in c["logxmm64_regs"]:
                if isinstance(reg, int) and reg not in logged:
                    err(("callees", ci, "logxmm64_regs"), f"xmm{reg} is narrowed without being logged (the worker rejects it)", "form")
        # v7: the 4-byte narrowing, the ST0 call argument and its transport
        if isinstance(c.get("logxmm32_regs"), list):
            logged = set(c.get("logxmm_regs") or [])
            if c.get("logxmm"):
                logged.add(0)
            if c.get("logxmm1"):
                logged.add(1)
            narrow64 = set(c.get("logxmm64_regs") or [])
            for reg in c["logxmm32_regs"]:
                if isinstance(reg, int) and reg not in logged:
                    err(("callees", ci, "logxmm32_regs"), f"xmm{reg} is narrowed without being logged (the worker rejects it)", "form")
                if isinstance(reg, int) and reg in narrow64:
                    err(("callees", ci, "logxmm32_regs"), f"xmm{reg} is narrowed to 8 bytes and 4 bytes (the worker rejects it: pick one)", "form")
        tport = c.get("st0_from_stack")
        if isinstance(tport, int) and not isinstance(tport, bool):
            if c.get("logst0") is None:
                err(("callees", ci, "st0_from_stack"), "ST0 transport without logst0 (the worker rejects it: the argument would be uncompared)", "form")
            need = tport + 1 if c.get("logst0") == "f64" else tport
            if nargs and need >= nargs:
                err(("callees", ci, "st0_from_stack"), f"stack argument {tport} is past nargs {nargs}", "range")
    if isinstance(checks.get("call_mask"), dict):
        for cid, per in checks["call_mask"].items():
            if isinstance(per, dict):
                for idx, mask in per.items():
                    if to_int(mask) is not None and to_int(mask) & 0xFFFFFFFF == 0:
                        err(("checks", "call_mask", cid, idx), "a zero mask compares nothing (the driver rejects it)", "range")
    for key in ("call_skip", "call_low8"):
        table = checks.get(key)
        if isinstance(table, dict):
            for cid, idxs in table.items():
                c = by_id.get(to_int(cid))
                nargs = c.get("nargs", 0) if c else None
                for idx in as_list(idxs):
                    if isinstance(nargs, int) and isinstance(idx, int) and idx >= nargs:
                        err(("checks", key, cid), f"argument {idx} is past callee {cid}'s nargs {nargs} (has no effect: likely a typo)", "range")
    x87 = contract.get("x87")
    if isinstance(x87, list):
        if len(x87) > limits["X87_MAX_ENTRIES"]:
            err(("x87",), f"{len(x87)} x87 entry values; the FPU stack holds {limits['X87_MAX_ENTRIES']}", "range")
        if x87 and checks.get("x87_state") is False:
            err(("checks", "x87_state"), "x87 entry values need the x87 state check (the worker refuses false)", "form")
    if contract.get("export") is not None and contract.get("export") == contract.get("mut_export"):
        err(("mut_export",), "the wrong version is the same export as the correct one", "form")
    # v7: the digest folds the conv-default stream only.
    if contract.get("log_digest") is True:
        for key in ("call_regs", "call_skip", "call_mask"):
            if key in checks:
                err(("checks", key), f"log_digest cannot combine with {key} (the driver refuses it)", "form")
    return errors


# --------------------------------------------------------------------------------------------------------
# Narrowing: what a proof under this contract does not cover, by name
# --------------------------------------------------------------------------------------------------------

def sorted_by_id(table):
    """A {callee id: value} table's items in numeric id order (ids are JSON object keys, so strings)."""
    if not isinstance(table, dict):
        return []
    return sorted(table.items(), key=lambda kv: (to_int(kv[0]) is None, to_int(kv[0]) or 0, str(kv[0])))


def narrowing(contract):
    """Settings that narrow the comparison, as (name, detail) pairs. The names are stable so results files
    and the proof record can list them."""
    out = []
    add = lambda name, detail: out.append((name, detail))
    checks = contract.get("checks") if isinstance(contract.get("checks"), dict) else {}
    for name in BEHAVIOUR_CHECKS:
        if checks.get(name, True) is False:
            add(f"check-off:{name}", f"checks.{name} is false: that observation is not compared")
    ret = checks.get("ret", "eax")
    if ret == "none":
        add("ret-none", "checks.ret is none: the return value is not compared")
    elif ret in ("al", "ax"):
        add(f"ret-{ret}", f"checks.ret is {ret}: only the low {8 if ret == 'al' else 16} bits of eax are compared")
    elif isinstance(ret, bool):
        add("ret-boolean", f"checks.ret is {str(ret).lower()}: the return register (eax) is compared exactly as on "
            "stock, recorded as features.ret_boolean (refused with strict: true); new contracts should name a "
            "channel and set strict: true")
    legacy_words = unresolved_word_specs(contract)
    if legacy_words:
        shown = "; ".join(f"{json_path(p)}: {','.join(k)}" for p, k in legacy_words[:3])
        more = f" (+{len(legacy_words) - 3} more)" if len(legacy_words) > 3 else ""
        add("legacy-word-spec", f"{len(legacy_words)} word spec{'s' if len(legacy_words) != 1 else ''} "
            f"{'hold' if len(legacy_words) != 1 else 'holds'} keys the checker does not know ({shown}{more}): "
            "each resolves to random bits exactly as on stock, recorded in features.unresolved_word_specs (refused "
            "with strict: true); new contracts should fix the keys and set strict: true")
    if "ret" in contract and contract["ret"] != ret:
        add("ret-unread", f"top-level ret is {contract['ret']!r} but checks.ret ({ret!r}) is what the checker compares")
    if checks.get("fulldata", True) is False:
        add("fulldata-off", "checks.fulldata is false: writes to data outside the declared globals are not seen")
    tol = checks.get("fp_tol")
    if isinstance(tol, str) and tol.strip():
        parts = [p.strip() for p in tol.split(",")]
        try:
            nonzero = any(float(p or 0) != 0 for p in parts)
        except ValueError:
            nonzero = True
        if nonzero:
            add("fp-tolerance", f"checks.fp_tol is {tol!r}: floating-point results may differ (rule 3 wants bit-exact)")
    for cid, idxs in sorted_by_id(checks.get("call_skip")):
        add(f"call-skip:{cid}", f"callee {cid} stack arguments {idxs} are not compared")
    callees = {c.get("id"): c for c in as_list(contract.get("callees")) if isinstance(c, dict)}
    call_regs = checks.get("call_regs") if isinstance(checks.get("call_regs"), dict) else {}
    for cid, regs in sorted_by_id(call_regs):
        callee = callees.get(to_int(cid)) or {}
        dropped = [r for r in DEFAULT_CALL_REGS.get(callee.get("conv", "cdecl"), ()) if r not in as_list(regs)]
        if dropped:
            add(f"call-regs:{cid}", f"callee {cid} ({callee.get('conv')}) register arguments {dropped} are not compared")
    for key in ("call_mask", "call_low8"):
        table = checks.get(key)
        if isinstance(table, dict):
            for cid, per in sorted_by_id(table):
                idxs = sorted(per) if isinstance(per, dict) else per
                add(f"call-mask:{cid}", f"callee {cid} arguments {idxs} are compared through a mask ({key})")
    for cid, callee in sorted((k, v) for k, v in callees.items() if isinstance(k, int)):
        transports = [k for k in ("xmm0_from_stack", "xmm1_from_stack", "eax_from_stack", "xmm_from_stack", "xmm_from_stack64", "st0_from_stack") if k in callee]
        if transports and callee.get("nargs", 0) > 1:
            add(f"transport-skips-args:{cid}", f"callee {cid} uses {', '.join(transports)}: its {callee.get('nargs')} stack arguments are not compared, only the transported register")
        cmp64 = callee.get("logxmm64_regs")
        if isinstance(cmp64, list) and cmp64:
            add(f"xmm-cmp64:{cid}", f"callee {cid} vector registers {sorted(cmp64)} compare only their low 8 bytes (the upper halves are not compared)")
        cmp32 = callee.get("logxmm32_regs")
        if isinstance(cmp32, list) and cmp32:
            add(f"xmm-cmp32:{cid}", f"callee {cid} vector registers {sorted(cmp32)} compare only their low 4 bytes (the upper 12 are not compared)")
    if contract.get("log_digest") is True:
        add("log-digest", "past-cap calls compare as a digest of callee id, default registers and stack words only (no snapshots, vector registers, ST0 or out-params past the cap)")
    if "mut_export" not in contract:
        add("no-mut-export", "no wrong version is declared: nothing shows the contract can see a change")
    share = contract.get("min_orig_ok_share")
    if isinstance(share, (int, float)) and not isinstance(share, bool) and share < DEFAULT_MIN_ORIG_OK_SHARE:
        add("min-orig-ok-share" + ("-zero" if share == 0 else "-lowered"),
            f"min_orig_ok_share is {share}: the vacuity rule {'is off' if share == 0 else 'is relaxed'} (default {DEFAULT_MIN_ORIG_OK_SHARE})")
    for cid in as_list(contract.get("coverage_exempt_callees")):
        add(f"coverage-exempt:{cid}", f"callee {cid} need never fire for the verdict to pass")
    if contract.get("allow_code_pointers") is True:
        add("allow-code-pointers", "random draws may point into code pages (audit mode)")
    trials = contract.get("trials")
    if isinstance(trials, int) and not isinstance(trials, bool) and trials < FEW_TRIALS:
        add("few-trials", f"{trials} trials (fewer than {FEW_TRIALS})")
    return out


def notes_for(contract):
    """Neither errors nor narrowing: facts worth a look."""
    out = []
    if "dll" in contract:
        out.append(("dll-override", f"runs {contract['dll']}, not the proof DLL; skipped when it is absent"))
    if any(k in contract for k in ("pokes", "heap_scripts")) or any(
            isinstance(s, dict) and "pinned" in s for s in as_list(contract.get("heapsegs"))):
        out.append(("legacy-alias", "uses a legacy alias (pokes, heap_scripts or pinned); prefer seg-level pin"))
    for key in ("x87", "abs_shadow", "fn_selftest"):
        if key in contract:
            out.append(("pending-field", f"{key} belongs to a checker extension in progress; only its presence is checked"))
    return out


def validate(contract, schema=None, limits=None):
    """Validate one contract. Returns {"errors": [...], "warnings": [...], "notes": [...]}; errors are
    {code, path, message}, warnings and notes {name, detail}."""
    schema = load_schema() if schema is None else schema
    limits = worker_limits() if limits is None else limits
    errors = SchemaValidator(schema).validate(contract)
    if isinstance(contract, dict):
        errors += semantic_errors(contract, limits)
        warnings, notes = narrowing(contract), notes_for(contract)
    else:
        warnings, notes = [], []
    unique = list(dict.fromkeys(errors))
    return {"errors": [dict(code=c, path=p, message=m) for c, p, m in unique],
            "warnings": [dict(name=n, detail=d) for n, d in warnings],
            "notes": [dict(name=n, detail=d) for n, d in notes]}


def contract_files(paths):
    out = []
    for p in paths:
        if os.path.isdir(p):
            out.extend(sorted(os.path.join(p, f) for f in os.listdir(p) if f.endswith(".json")))
        else:
            out.append(p)
    return out


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("paths", nargs="*", help="contract files or folders (default: the tracked contracts)")
    parser.add_argument("--json", action="store_true", help="print every result as JSON")
    parser.add_argument("--quiet", action="store_true", help="list only contracts with errors")
    args = parser.parse_args(argv)
    schema, limits = load_schema(), worker_limits()
    results = {}
    for path in contract_files(args.paths or [CONTRACTS_DIR]):
        try:
            with open(path, encoding="utf-8") as fh:
                contract = json.load(fh)
        except (OSError, ValueError) as exc:
            results[path] = {"errors": [dict(code="unreadable", path="$", message=str(exc))], "warnings": [], "notes": []}
            continue
        results[path] = validate(contract, schema, limits)
    failed = [p for p, r in results.items() if r["errors"]]
    if args.json:
        print(json.dumps({"limits": limits, "contracts": results}, indent=1))
    else:
        for path, r in results.items():
            if args.quiet and not r["errors"]:
                continue
            status = "ERROR" if r["errors"] else "ok"
            names = ", ".join(w["name"] for w in r["warnings"])
            print(f"{status:5} {os.path.relpath(path)}" + (f"  narrowing: {names}" if names else ""))
            for e in r["errors"]:
                print(f"      {e['code']}: {e['path']}: {e['message']}")
        counts = {}
        for r in results.values():
            for w in r["warnings"]:
                base = w["name"].split(":")[0]
                counts[base] = counts.get(base, 0) + 1
        print(f"{len(results)} contracts, {len(failed)} with errors, "
              f"{sum(1 for r in results.values() if r['warnings'])} with narrowing")
        for name, n in sorted(counts.items()):
            print(f"  {name}: {n}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
