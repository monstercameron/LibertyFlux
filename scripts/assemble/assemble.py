"""Assemble the verified rewrites into one injectable 32-bit library.

What it makes. Under --out (default .artifacts/assembled/, which git ignores; generated code is never committed):

  Cargo.toml              a workspace of its own (not the repository's), release profile panic=abort like the checker
  shards/sNNN/            shard crates `lf-rw-sNNN`: a few hundred rewrites each, cut by address order, so no crate
                          is large (one crate of 7,381 rewrites once took a 24 GB machine down to under 1 GB of
                          commit headroom); each lists its rewrites as `lf_rewrite_32::RewriteEntry` rows
  top/                    the cdylib `lf-rewrites` (lf_rewrites.dll): depends on every shard and on crates/lf-rewrite-32,
                          exports lf_rewrite_version, lf_rewrite_init and lf_rewrite_table (see lf-rewrite-32's docs)
  manifest.json           what went in, what was left out and why, shard sizes and costs (format below)
  callee-map.template.json  every included rewrite that calls callees, with the slot ids found, for the coordinator

Which rewrites go in. Every entry of rewrites/verified/index.json is scanned, then compiled inside its shard against
the shared runtime (crates/tools/lf-checker-rt) alone, as scripts/ci/check_rewrites_build.py does: inner doc comments
and inner attributes are neutralised in the copy, `lf_k2_rt` (an older name of the shared runtime) is aliased to
it, and a file that fails to type-check is dropped and the shard checked again until it compiles. Left out, each with
its reason in the manifest: files that do not compile; files that name another lane runtime (`lf_rn94_rt` and the
like, not aliased because their helpers may differ; --alias adds a name); files whose export cannot be found or is
ambiguous; files that declare foreign functions (they could not link); a second file exporting a symbol name already
taken (exports are unmangled, so two would collide at link time).

The production runtime. Rewrites were proven in the checker, where `relocated()`/`global()` convert file addresses
through the worker's mapping base and `callee_cdecl!(id, ...)` (and the stdcall/thiscall/fastcall forms and
`callee_addr(id)`) call through the worker's stub table, slot `id`. Slot ids are local to each function's contract.
In the game:
  - the base is the game's own, set once by the loader through lf_rewrite_init(exe_base);
  - slot `id` must be the original callee's real address. That mapping is in the contracts, which are not tracked,
    so it comes from an optional callee map (--callee-map; the coordinator supplies it). In the generated copy of
    each rewrite (never in the tracked file) every callee call is redirected to a per-function resolver
    `__lf_callee(id)` built from the map; an unmapped id resolves to 0, as in the checker.
A rewrite is switchable (FLAG_SWITCHABLE; the loader hooks only those, all off at start) when it reads no
checker-only input (xmm_word, tls_slot, the CHECKER_* mirrors) and either calls no callee slot or every slot it uses
is mapped to a plain original address. Without --callee-map every rewrite that calls a callee is compiled and listed
but not switchable.

Callee map format (JSON, input):
  {"format": "lf-callee-map/1",
   "functions": {
     "0x00A01230": {                        original address of the rewritten function, as in index.json
       "callees": {
         "1": {"va": "0x00A04560"},         slot 1 calls the original function at this address
         "2": {"iat": "0x00B01000"},        slot 2 calls through this import-table slot (the pointer stored there)
         "3": {"va": "0x00A07890", "transport": "xmm0_from_stack"}
       },                                   a slot fed through a checker transport (xmm0/xmm1/eax_from_stack,
                                            noclean) cannot be called directly: the rewrite stays unswitchable
       "complete": true,                    the map lists every slot the contract declares; required when the
                                            rewrite passes slot ids through named constants (not literals)
       "expected": "558BEC"                 optional: bytes the loader requires at the original before patching
     }}}

Manifest format (JSON, output): {"format": "lf-assembled-manifest/1", "index_entries", "included", "excluded_count",
"switchable", "shard_size", "edition", "callee_map": {"path", "sha256", "functions"} or null, "aliases", "deps":
{"seconds", "peak_rss_mb"}, "shards": [{"name", "crate", "index", "from", "to", "count", "rounds", "check_seconds",
"check_peak_rss_mb", "build_seconds", "build_peak_rss_mb"}] (check figures: all rounds' seconds added up, the largest
round's peak), "rewrites": [{"address", "kind", "file", "shard",
"handle", "export", "extra_exports" (a lane's `mut_*` wrong version kept in the same file: compiled, never in the
table), "conv", "uses_callees", "callee_ids", "callee_ids_literal", "callees" (the map entry used, or
null), "checker_only", "switchable", "not_switchable": [reasons]}], "excluded": [{"address", "kind", "file", "stage"
(scan, duplicate or compile), "reason"}]}. Costs are null when not measured. Peak memory is the largest resident set
of any one process cargo ran for that step (os.wait4), so it is the compiler's peak, not the sum.

Usage:
  python scripts/assemble/assemble.py                       scan, write the crates, type-check every shard
  python scripts/assemble/assemble.py --callee-map FILE     the same, with slot ids mapped to original addresses
  python scripts/assemble/assemble.py --build               then `cargo build --release` the cdylib (needs the MSVC
                                                            linker: Windows only)
  python scripts/assemble/assemble.py --build-shards        `cargo build --release` each shard rlib (no linker
                                                            needed) and record time and memory per shard
  --shard-size N (default 300), --edition 2021|2024 (default 2024), --limit N (first N rewrites by address, for measurements), --no-check (write the
  crates only; every rewrite is listed as included unchecked), --out DIR, --index FILE, --alias NAME.
Exit status 0 when the crates were written and every shard (and the top crate) type-checks; 1 otherwise.
Needs cargo with the i686-pc-windows-msvc target. Standard library only.
"""

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TARGET = "i686-pc-windows-msvc"
DEFAULT_OUT = ROOT / ".artifacts" / "assembled"
DEFAULT_SHARD_SIZE = 300
MAX_ROUNDS = 30
PREFERRED_BASE = 0x400000
RUNTIME = "lf_checker_rt"
DEFAULT_ALIASES = ("lf_k2_rt",)
EDITION = "2024"  # the workspace edition, as scripts/ci/check_rewrites_build.py assumes; --edition overrides

CONV_CODES = {"cdecl": 0, "C": 0, "stdcall": 1, "system": 1, "thiscall": 2, "fastcall": 3}
CONV_NAMES = {"C": "cdecl", "system": "stdcall"}
FLAG_SWITCHABLE, FLAG_USES_CALLEES, FLAG_CALLEES_MAPPED, FLAG_CHECKER_ONLY = 1, 2, 4, 8
TRANSPORTS = ("xmm0_from_stack", "xmm1_from_stack", "eax_from_stack", "noclean")

EXPORT_MACRO = re.compile(r"(?<![\w$])(?:[A-Za-z_]\w*\s*::\s*)?export\s*!\s*\(\s*(cdecl|stdcall|thiscall|fastcall)\s*,"
                          r"\s*([A-Za-z_]\w*)\s*\(")
EXPORT_FN = re.compile(r"#\[\s*(?:unsafe\s*\(\s*)?no_mangle\s*\)?\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub\s+)?(?:unsafe\s+)?"
                       r"extern\s+\"(C|cdecl|stdcall|thiscall|fastcall|system)\"\s+fn\s+([A-Za-z_]\w*)")
CALLEE_MACRO = re.compile(r"(?<![\w$])(?:[A-Za-z_]\w*\s*::\s*)*callee_(cdecl|stdcall|thiscall|fastcall)\s*!")
CALLEE_ADDR = re.compile(r"(?<![\w$])(?:[A-Za-z_]\w*\s*::\s*)*callee_addr\s*\(")
CALLEE_ID_MACRO = re.compile(r"callee_(?:cdecl|stdcall|thiscall|fastcall)\s*!\s*\(\s*([^,()]+?)\s*,")
CALLEE_ID_ADDR = re.compile(r"callee_addr\s*\(\s*([^()]+?)\s*\)")
INT_LITERAL = re.compile(r"^(0x[0-9a-fA-F_]+|[0-9_]+)(?:u32|usize|i32)?$")
CHECKER_ONLY = re.compile(r"\b(xmm_word|tls_slot|CHECKER_XMM|CHECKER_TLS|CHECKER_CTABLE)\b")
FOREIGN_BLOCK = re.compile(r"\bextern\s+\"[^\"]*\"\s*\{")
RUNTIME_PATH = re.compile(r"\b(lf_\w+_rt|checker_rt|rt)\s*::")
LOCAL_ALIAS = re.compile(r"\b(?:use|extern\s+crate)\s+(\w+)\s+as\s+(\w+)\s*;")
RW_FILE = re.compile(r"(?:^|[\\/])rw[\\/]([0-9a-f]{8})\.rs$")

# The checker runtime's callee macros, redirected to the per-function resolver. Same patterns as lf-checker-rt, so a
# call that compiled against the runtime compiles here; only the slot lookup differs.
CALLEE_MACROS = """\
macro_rules! __lf_callee_cdecl {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "cdecl" fn($( lf_checker_rt::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute(__lf_callee($id) as usize) };
        f($( $arg ),*)
    }};
}
macro_rules! __lf_callee_stdcall {
    ($id:expr, $ret:ty, $($arg:expr),* $(,)?) => {{
        let f: extern "stdcall" fn($( lf_checker_rt::__ty!($arg) ),*) -> $ret =
            unsafe { core::mem::transmute(__lf_callee($id) as usize) };
        f($( $arg ),*)
    }};
}
macro_rules! __lf_callee_thiscall {
    ($id:expr, $ret:ty, $this_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "thiscall" fn(u32 $(, lf_checker_rt::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute(__lf_callee($id) as usize) };
        f($this_arg $(, $arg )*)
    }};
}
macro_rules! __lf_callee_fastcall {
    ($id:expr, $ret:ty, $ecx_arg:expr, $edx_arg:expr $(, $arg:expr)* $(,)?) => {{
        let f: extern "fastcall" fn(u32, u32 $(, lf_checker_rt::__ty!($arg) )*) -> $ret =
            unsafe { core::mem::transmute(__lf_callee($id) as usize) };
        f($ecx_arg, $edx_arg $(, $arg )*)
    }};
}
"""


def key_of(address):
    """Eight lower-case hex digits of an index address (the copy's file stem and module suffix)."""
    return f"{int(address, 16):08x}"


def strip_comments(text):
    """Text with line and block comments blanked (string and char literals kept), for classification only."""
    out, i, n = [], 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
            out.append(" ")
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            out.append(text[i:j + 1])
            i = j + 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


def parse_int(text):
    match = INT_LITERAL.match(text.strip())
    return int(match.group(1).replace("_", ""), 0) if match else None


def scan(text, aliases=DEFAULT_ALIASES):
    """Classify one rewrite's source. Returns a dict; `error` is set when the file cannot go in at all."""
    code = strip_comments(text)
    info = {"error": None, "export": None, "conv": None, "all_exports": [], "uses_callees": False, "callee_ids": [],
            "callee_ids_literal": True, "checker_only": sorted(set(CHECKER_ONLY.findall(code)))}
    exports = [(m.group(1), m.group(2)) for m in EXPORT_MACRO.finditer(code)]
    exports += [(CONV_NAMES.get(m.group(1), m.group(1)), m.group(2)) for m in EXPORT_FN.finditer(code)]
    names = sorted({name for _, name in exports})
    info["all_exports"] = names
    # A lane may keep its deliberately wrong version (`mut_*`) beside the rewrite; the rewrite is the one `rw_*`.
    rewrites = [e for e in exports if e[1].startswith("rw_")]
    if not exports:
        info["error"] = "no export found (neither export!(conv, name(...)) nor a no_mangle extern fn)"
    elif len(names) == 1:
        info["conv"], info["export"] = exports[0]
    elif len({name for _, name in rewrites}) == 1 and all(n.startswith(("rw_", "mut_")) for n in names):
        info["conv"], info["export"] = rewrites[0]
    else:
        info["error"] = "several exports: " + ", ".join(names)
    if info["error"] is None and FOREIGN_BLOCK.search(code):
        info["error"] = "declares foreign functions (extern block); they could not be resolved at link time"
    allowed = {RUNTIME, *aliases}
    # A file may rename the shared runtime locally (`use lf_checker_rt as rt;`); that name is the runtime too.
    allowed |= {m.group(2) for m in LOCAL_ALIAS.finditer(code) if m.group(1) in allowed}
    other = sorted({m.group(1) for m in RUNTIME_PATH.finditer(code)} - allowed)
    if info["error"] is None and other:
        info["error"] = f"uses lane runtime `{other[0]}`, which is not the shared runtime (not aliased)"
    ids = [m.group(1) for m in CALLEE_ID_MACRO.finditer(code)] + [m.group(1) for m in CALLEE_ID_ADDR.finditer(code)]
    info["uses_callees"] = bool(CALLEE_MACRO.search(code) or CALLEE_ADDR.search(code))
    literal = [parse_int(i) for i in ids]
    info["callee_ids"] = sorted({v for v in literal if v is not None})
    info["callee_ids_literal"] = all(v is not None for v in literal) and len(literal) > 0 or not info["uses_callees"]
    return info


def production_copy(text):
    """The text compiled in a shard: inner doc comments and inner attributes neutralised (an included file cannot
    carry them), every callee call redirected to the module's resolver."""
    text = re.sub(r"(?m)^(\s*)//!", r"\1//", text)
    text = re.sub(r"(?m)^\s*#!\[[^\]]*\]\s*$", "", text)
    text = CALLEE_MACRO.sub(lambda m: f"__lf_callee_{m.group(1)}!", text)
    return CALLEE_ADDR.sub("__lf_callee(", text)


def parse_address(value, what):
    if not isinstance(value, str) or not re.fullmatch(r"0x[0-9a-fA-F]{1,8}", value):
        raise ValueError(f"{what}: expected a hex address string, got {value!r}")
    number = int(value, 16)
    if number < PREFERRED_BASE:
        raise ValueError(f"{what}: {value} lies below the image base")
    return number


def load_callee_map(path):
    """Read and validate a callee map. Returns {function address: {"callees": {id: entry}, "complete", "expected"}}."""
    data = json.loads(Path(path).read_text(encoding="utf-8"))
    if data.get("format") != "lf-callee-map/1":
        raise ValueError(f"{path}: format must be lf-callee-map/1")
    out = {}
    for address, entry in data.get("functions", {}).items():
        function = parse_address(address, f"function {address}")
        callees = {}
        for slot, target in entry.get("callees", {}).items():
            if not re.fullmatch(r"[0-9]+", slot) or int(slot) > 255:
                raise ValueError(f"function {address}: slot id {slot!r} is not 0-255")
            kinds = [k for k in ("va", "iat") if k in target]
            if len(kinds) != 1:
                raise ValueError(f"function {address} slot {slot}: give exactly one of va or iat")
            transport = target.get("transport")
            if transport is not None and transport not in TRANSPORTS:
                raise ValueError(f"function {address} slot {slot}: unknown transport {transport!r}")
            callees[int(slot)] = {"kind": kinds[0], "address": parse_address(target[kinds[0]], f"{address}/{slot}"),
                                  "transport": transport}
        expected = entry.get("expected", "")
        if not re.fullmatch(r"(?:[0-9a-fA-F]{2}){0,16}", expected):
            raise ValueError(f"function {address}: expected must be up to 16 hex bytes")
        out[function] = {"callees": callees, "complete": bool(entry.get("complete", False)),
                         "expected": bytes.fromhex(expected)}
    return out


def switchability(info, mapped):
    """(switchable, reasons, callees used) for a scanned rewrite and its callee-map entry (or None)."""
    reasons = []
    if info["checker_only"]:
        reasons.append("reads checker-only input: " + ", ".join(info["checker_only"]))
    used = {}
    if info["uses_callees"]:
        if mapped is None:
            reasons.append("calls callee slots and the callee map has no entry for it")
        else:
            wanted = set(info["callee_ids"])
            if not info["callee_ids_literal"]:
                if mapped["complete"]:
                    wanted |= set(mapped["callees"])
                else:
                    reasons.append("passes slot ids through names and the map entry is not marked complete")
            for slot in sorted(wanted):
                target = mapped["callees"].get(slot)
                if target is None:
                    reasons.append(f"callee slot {slot} is not in the map")
                elif target["transport"]:
                    reasons.append(f"callee slot {slot} uses the checker transport {target['transport']}")
                else:
                    used[slot] = target
    return not reasons, reasons, used


def resolver(callees):
    """Rust source of one module's `__lf_callee`: slot id to the callee's address in the running game."""
    arms = []
    for slot, target in sorted(callees.items()):
        if target["transport"]:
            continue
        if target["kind"] == "va":
            arms.append(f"        {slot} => lf_checker_rt::relocated(0x{target['address']:08X}),")
        else:
            arms.append(f"        {slot} => unsafe {{ (lf_checker_rt::relocated(0x{target['address']:08X}) as *const u32)"
                        ".read_volatile() },")
    if not arms:
        return ["    #[allow(dead_code)]", "    #[inline(always)]", "    fn __lf_callee(_id: u32) -> u32 {", "        0",
                "    }"]
    return (["    #[allow(dead_code)]", "    #[inline(always)]", "    fn __lf_callee(id: u32) -> u32 {", "        match id {"]
            + ["    " + a for a in arms] + ["            _ => 0,", "        }", "    }"])


def write_if_changed(path, text):
    """Write only when the content differs, so unchanged shards keep their timestamps and stay built."""
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.is_file() or path.read_text(encoding="utf-8") != text:
        path.write_text(text, encoding="utf-8", newline="\n")


def rel(path, start):
    return Path(os.path.relpath(path, start)).as_posix()


def shard_name(index):
    return f"s{index:03d}"


def write_shard(out, index, rows, sources, callee_maps, aliases=DEFAULT_ALIASES):
    """Write one shard crate. Returns {lib.rs line: key} for attributing errors raised in lib.rs itself."""
    name = shard_name(index)
    crate_dir = out / "shards" / name
    runtime = rel(ROOT / "crates" / "tools" / "lf-checker-rt", crate_dir)
    glue = rel(ROOT / "crates" / "lf-rewrite-32", crate_dir)
    write_if_changed(crate_dir / "Cargo.toml",
                     f'[package]\nname = "lf-rw-{name}"\nversion = "0.0.0"\nedition = "{EDITION}"\npublish = false\n\n'
                     f'[lib]\npath = "src/lib.rs"\n\n[dependencies]\nlf-checker-rt = {{ path = "{runtime}" }}\n'
                     f'lf-rewrite-32 = {{ path = "{glue}" }}\n')
    span = f", {rows[0]['address']} to {rows[-1]['address']}" if rows else " (every file left out)"
    lines = [f"//! Generated by scripts/assemble/assemble.py: shard {name}, {len(rows)} rewrites{span}.",
             "//! Do not edit; regenerate. The tracked rewrites are under rewrites/verified/.",
             "#![allow(warnings, clippy::all)]", "#[macro_use]", "extern crate lf_checker_rt;"]
    lines += [f"extern crate lf_checker_rt as {alias};" for alias in sorted(set(aliases))]
    lines.append("use lf_rewrite_32::RewriteEntry;")
    lines += CALLEE_MACROS.splitlines()
    owner = {}
    wanted_files = set()
    for row in rows:
        key = row["key"]
        wanted_files.add(f"{key}.rs")
        write_if_changed(crate_dir / "src" / "rw" / f"{key}.rs", production_copy(sources[key]))
        start = len(lines) + 1
        lines.append(f"pub mod m_{key} {{")
        lines.append("    #[allow(unused_imports)]")
        lines.append("    use lf_checker_rt::*;")
        lines += resolver(callee_maps.get(key, {}))
        lines.append(f'    include!("rw/{key}.rs");')
        lines.append("}")
        for number in range(start, len(lines) + 1):
            owner[number] = key
    lines.append(f"const SHARD: u16 = {index};")
    lines.append("fn row(address: u32, detour: usize, name: &'static str, conv: u8, flags: u8, expected: &'static [u8])"
                 " -> RewriteEntry {")
    lines.append("    RewriteEntry { address, conv, flags, shard: SHARD, detour, name_ptr: name.as_ptr() as usize,"
                 " expected_ptr: if expected.is_empty() { 0 } else { expected.as_ptr() as usize },"
                 " name_len: name.len() as u32, expected_len: expected.len() as u32 }")
    lines.append("}")
    lines.append("/// This shard's replacement rows, address order.")
    lines.append("pub fn entries() -> Vec<RewriteEntry> {")
    lines.append("    vec![")
    for row in rows:
        expected = ", ".join(f"0x{b:02X}" for b in row["expected"])
        owner[len(lines) + 1] = row["key"]
        lines.append(f"        row(0x{int(row['address'], 16):08X}, m_{row['key']}::{row['export']} as *const () as usize, "
                     f"\"{row['handle']}\", {CONV_CODES[row['conv']]}, {row['flags']}, &[{expected}]),")
    lines.append("    ]")
    lines.append("}")
    write_if_changed(crate_dir / "src" / "lib.rs", "\n".join(lines) + "\n")
    # Drop copies of rewrites no longer in this shard, so a stale file can never be compiled by mistake.
    for old in (crate_dir / "src" / "rw").glob("*.rs"):
        if old.name not in wanted_files:
            old.unlink()
    return owner


def write_workspace(out, shard_count):
    members = [f'    "shards/{shard_name(i)}",' for i in range(shard_count)] + ['    "top",']
    write_if_changed(out / "Cargo.toml",
                     "# Generated by scripts/assemble/assemble.py. Not part of the repository's workspace.\n"
                     "[workspace]\nresolver = \"2\"\nmembers = [\n" + "\n".join(members) + "\n]\n\n"
                     "# Built the way the checker builds rewrite libraries.\n"
                     "[profile.release]\npanic = \"abort\"\n\n[profile.dev]\npanic = \"abort\"\n")
    top = out / "top"
    deps = [f'lf-rw-{shard_name(i)} = {{ path = "../shards/{shard_name(i)}" }}' for i in range(shard_count)]
    write_if_changed(top / "Cargo.toml",
                     '[package]\nname = "lf-rewrites"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n\n'
                     '[lib]\nname = "lf_rewrites"\npath = "src/lib.rs"\ncrate-type = ["cdylib"]\n\n[dependencies]\n'
                     f'lf-rewrite-32 = {{ path = "{rel(ROOT / "crates" / "lf-rewrite-32", top)}" }}\n' + "\n".join(deps) + "\n")
    shards = ", ".join(f"lf_rw_{shard_name(i)}::entries" for i in range(shard_count))
    write_if_changed(top / "src" / "lib.rs",
                     "//! Generated by scripts/assemble/assemble.py: the assembled rewrite library.\n"
                     "//! Exports lf_rewrite_version and lf_rewrite_init (from lf-rewrite-32) and lf_rewrite_table.\n"
                     "#![allow(warnings, clippy::all)]\n"
                     f"lf_rewrite_32::export_table!({shards});\n")


def run_measured(cmd, cwd, env):
    """Run a command; return (exit code, seconds, peak RSS in MB, stdout, stderr). The peak is the largest resident
    set of the command or of any process it waited for (os.wait4, so per step, not a running maximum); None where
    wait4 is missing (Windows)."""
    import tempfile
    start = time.monotonic()
    with tempfile.TemporaryFile("w+", encoding="utf-8") as out, tempfile.TemporaryFile("w+", encoding="utf-8") as err:
        proc = subprocess.Popen(cmd, cwd=cwd, env=env, stdout=out, stderr=err, text=True)
        peak = None
        if hasattr(os, "wait4"):
            _, status, usage = os.wait4(proc.pid, 0)
            proc.returncode = os.waitstatus_to_exitcode(status)
            peak = round(usage.ru_maxrss / 1024, 1)
        else:
            proc.wait()
        seconds = round(time.monotonic() - start, 1)
        out.seek(0)
        err.seek(0)
        return proc.returncode, seconds, peak, out.read(), err.read()


def attribute(stdout, owner):
    """{key: first error} from cargo's JSON messages, plus unattributed error texts. A span counts when it, or any
    macro expansion it came through, lies in a rewrite copy (rw/<key>.rs) or on a lib.rs line owned by a rewrite."""
    found, loose = {}, []

    def keys_of(span):
        while span:
            name = span.get("file_name", "")
            match = RW_FILE.search(name)
            if match:
                yield match.group(1)
            elif re.search(r"(?:^|[\\/])lib\.rs$", name) and span.get("line_start") in owner:
                yield owner[span["line_start"]]
            span = (span.get("expansion") or {}).get("span")

    for line in stdout.splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        message = item.get("message") if item.get("reason") == "compiler-message" else None
        if not message or message.get("level") != "error":
            continue
        spans = list(message.get("spans", []))
        for child in message.get("children", []):
            spans += child.get("spans", [])
        keys = [k for span in spans for k in keys_of(span)]
        if keys:
            found.setdefault(keys[0], message.get("message", "error"))
        elif not message.get("message", "").startswith("aborting due to"):
            loose.append(message.get("rendered") or message.get("message", "error"))
    return found, loose


def cargo_env(out):
    return dict(os.environ, CARGO_TARGET_DIR=str(out / "target"), CARGO_TERM_COLOR="never")


def check_shard(out, index, rows, sources, callee_maps, log, aliases=DEFAULT_ALIASES):
    """Type-check one shard, dropping files that fail until it compiles. Returns (kept rows, {key: error}, stats)."""
    failed, stats = {}, {"rounds": 0, "check_seconds": None, "check_peak_rss_mb": None}
    kept = list(rows)
    for _ in range(MAX_ROUNDS):
        owner = write_shard(out, index, kept, sources, callee_maps, aliases)
        if not kept:
            # An empty shard stays a valid, empty crate so shard numbers do not move.
            return kept, failed, stats
        cmd = ["cargo", "check", "--target", TARGET, "-p", f"lf-rw-{shard_name(index)}", "--message-format", "json"]
        code, seconds, peak, stdout, stderr = run_measured(cmd, out, cargo_env(out))
        stats["rounds"] += 1
        stats["check_seconds"] = round((stats["check_seconds"] or 0) + seconds, 1)
        if peak is not None:
            stats["check_peak_rss_mb"] = max(stats["check_peak_rss_mb"] or 0, peak)
        if code == 0:
            return kept, failed, stats
        new, loose = attribute(stdout, owner)
        new = {k: v for k, v in new.items() if k not in failed}
        if not new:
            raise RuntimeError(f"shard {shard_name(index)} fails without a per-file error:\n"
                               + "\n".join(loose)[-4000:] + stderr[-2000:])
        log(f"  {shard_name(index)}: dropping {len(new)} file(s) that do not compile")
        failed.update(new)
        kept = [r for r in kept if r["key"] not in failed]
    raise RuntimeError(f"shard {shard_name(index)} still failing after {MAX_ROUNDS} rounds")


def shard_rows(rows, size):
    """Address-ordered chunks of at most `size` rows."""
    ordered = sorted(rows, key=lambda r: int(r["address"], 16))
    return [ordered[i:i + size] for i in range(0, len(ordered), size)]


def plan(index, read_source, aliases=DEFAULT_ALIASES, callee_map=None):
    """Scan every index entry. Returns (candidate rows, excluded rows, sources by key)."""
    rows, excluded, sources, exporters = [], [], {}, {}
    for entry in sorted(index, key=lambda e: int(e["address"], 16)):
        key = key_of(entry["address"])
        base = {"address": entry["address"], "kind": entry.get("kind", "function"), "file": entry["file"]}
        text = read_source(entry["file"])
        if text is None:
            excluded.append(dict(base, stage="scan", reason="file missing"))
            continue
        info = scan(text, aliases)
        if info["error"]:
            excluded.append(dict(base, stage="scan", reason=info["error"]))
            continue
        taken = [name for name in info["all_exports"] if name in exporters]
        if taken:
            excluded.append(dict(base, stage="duplicate",
                                 reason=f"export {taken[0]} is already taken by {exporters[taken[0]]}"))
            continue
        exporters.update({name: entry["file"] for name in info["all_exports"]})
        mapped = (callee_map or {}).get(int(entry["address"], 16))
        switchable, reasons, used = switchability(info, mapped)
        flags = (FLAG_SWITCHABLE if switchable else 0) | (FLAG_USES_CALLEES if info["uses_callees"] else 0) \
            | (FLAG_CALLEES_MAPPED if info["uses_callees"] and switchable else 0) \
            | (FLAG_CHECKER_ONLY if info["checker_only"] else 0)
        sources[key] = text
        rows.append(dict(base, key=key, handle=f"{base['kind']}.{key}", export=info["export"],
                         extra_exports=[n for n in info["all_exports"] if n != info["export"]],
                         conv=CONV_NAMES.get(info["conv"], info["conv"]), uses_callees=info["uses_callees"],
                         callee_ids=info["callee_ids"], callee_ids_literal=info["callee_ids_literal"],
                         checker_only=info["checker_only"], switchable=switchable, not_switchable=reasons,
                         flags=flags, callees=used, expected=(mapped or {}).get("expected", b"")))
    return rows, excluded, sources


def manifest_row(row, shard):
    callees = {str(s): {t["kind"]: f"0x{t['address']:08X}"} for s, t in sorted(row["callees"].items())} or None
    return {"address": row["address"], "kind": row["kind"], "file": row["file"], "shard": shard,
            "handle": row["handle"], "export": row["export"], "extra_exports": row["extra_exports"], "conv": row["conv"],
            "uses_callees": row["uses_callees"],
            "callee_ids": row["callee_ids"], "callee_ids_literal": row["callee_ids_literal"],
            "callees": callees if row["switchable"] else None, "checker_only": row["checker_only"],
            "switchable": row["switchable"], "not_switchable": row["not_switchable"]}


def assemble(index, read_source, out, shard_size=DEFAULT_SHARD_SIZE, aliases=DEFAULT_ALIASES, callee_map=None,
             callee_map_info=None, check=True, build_shards=False, log=print, checker=None):
    """Write the crates and (unless check=False) type-check them. `checker` replaces check_shard in tests. Returns
    the manifest dict (also written to out/manifest.json)."""
    out = Path(out)
    rows, excluded, sources = plan(index, read_source, aliases, callee_map)
    real_check = checker is None
    checker = checker or check_shard
    deps = {"seconds": None, "peak_rss_mb": None}
    chunks = shard_rows(rows, shard_size)
    write_workspace(out, len(chunks))
    # Every member must exist before cargo will run at all; drop crates of shards that no longer exist.
    for index_no, chunk in enumerate(chunks):
        write_shard(out, index_no, chunk, sources, {r["key"]: r["callees"] for r in chunk}, aliases)
    for stale in (out / "shards").glob("s[0-9][0-9][0-9]"):
        if int(stale.name[1:]) >= len(chunks):
            shutil.rmtree(stale)
    if check and real_check and chunks:
        # Build the shared dependencies once, measured on their own, so the shard figures are the shards' alone.
        code, seconds, peak, _, stderr = run_measured(
            ["cargo", "check", "--target", TARGET, "-p", "lf-rewrite-32"], out, cargo_env(out))
        if code != 0:
            raise RuntimeError("the shared runtime does not build:\n" + stderr[-4000:])
        deps = {"seconds": seconds, "peak_rss_mb": peak}
    shards, kept_all = [], []
    for index_no, chunk in enumerate(chunks):
        callee_maps = {r["key"]: r["callees"] for r in chunk}
        if check:
            kept, failed, stats = checker(out, index_no, chunk, sources, callee_maps, log, aliases)
        else:
            kept, failed, stats = chunk, {}, {"rounds": 0, "check_seconds": None, "check_peak_rss_mb": None}
        for row in chunk:
            if row["key"] in failed:
                excluded.append({"address": row["address"], "kind": row["kind"], "file": row["file"],
                                 "stage": "compile", "reason": failed[row["key"]]})
        if len(kept) != len(chunk):
            write_shard(out, index_no, kept, sources, callee_maps, aliases)
        record = {"name": shard_name(index_no), "crate": f"lf-rw-{shard_name(index_no)}", "index": index_no,
                  "from": kept[0]["address"] if kept else None, "to": kept[-1]["address"] if kept else None,
                  "count": len(kept), **stats, "build_seconds": None, "build_peak_rss_mb": None}
        if build_shards and kept:
            code, seconds, peak, _, stderr = run_measured(
                ["cargo", "build", "--release", "--target", TARGET, "-p", record["crate"]], out, cargo_env(out))
            if code != 0:
                raise RuntimeError(f"release build of {record['crate']} failed:\n" + stderr[-4000:])
            record["build_seconds"], record["build_peak_rss_mb"] = seconds, peak
        shards.append(record)
        kept_all += [(row, record["name"]) for row in kept]
        log(f"{record['name']}: {len(kept)} of {len(chunk)} rewrites, check {stats['check_seconds']} s "
            f"(peak {stats['check_peak_rss_mb']} MB), build {record['build_seconds']} s "
            f"(peak {record['build_peak_rss_mb']} MB)")
    excluded.sort(key=lambda e: int(e["address"], 16))
    manifest = {"format": "lf-assembled-manifest/1", "index_entries": len(index), "included": len(kept_all),
                "excluded_count": len(excluded), "switchable": sum(1 for r, _ in kept_all if r["switchable"]),
                "shard_size": shard_size, "edition": EDITION, "callee_map": callee_map_info, "aliases": list(aliases),
                "deps": deps,
                "shards": shards, "rewrites": [manifest_row(r, s) for r, s in kept_all], "excluded": excluded}
    template = {"format": "lf-callee-map/1", "functions": {
        r["address"]: {"complete": False, "callees": {str(i): {"va": None} for i in r["callee_ids"]}}
        for r, _ in kept_all if r["uses_callees"]}}
    write_if_changed(out / "manifest.json", json.dumps(manifest, indent=1) + "\n")
    write_if_changed(out / "callee-map.template.json", json.dumps(template, indent=1) + "\n")
    return manifest


def main(argv=None):
    global EDITION
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", default=str(DEFAULT_OUT))
    parser.add_argument("--index", default=str(ROOT / "rewrites" / "verified" / "index.json"))
    parser.add_argument("--shard-size", type=int, default=DEFAULT_SHARD_SIZE)
    parser.add_argument("--callee-map")
    parser.add_argument("--alias", action="append", default=[], help="another runtime crate name to alias")
    parser.add_argument("--limit", type=int, help="only the first N rewrites by address (measurements)")
    parser.add_argument("--edition", choices=("2021", "2024"), default=EDITION,
                        help="edition of the shard crates (2021 accepts files that use `gen` as a name)")
    parser.add_argument("--no-check", action="store_true")
    parser.add_argument("--build-shards", action="store_true")
    parser.add_argument("--build", action="store_true", help="cargo build --release the cdylib (Windows)")
    args = parser.parse_args(argv)
    EDITION = args.edition
    index_path = Path(args.index)
    index = json.loads(index_path.read_text(encoding="utf-8"))
    if args.limit:
        index = sorted(index, key=lambda e: int(e["address"], 16))[:args.limit]
    callee_map, info = None, None
    if args.callee_map:
        callee_map = load_callee_map(args.callee_map)
        digest = hashlib.sha256(Path(args.callee_map).read_bytes()).hexdigest()
        info = {"path": Path(args.callee_map).name, "sha256": digest, "functions": len(callee_map)}
    base = index_path.parent

    def read_source(name):
        path = base / name
        return path.read_text(encoding="utf-8", errors="replace") if path.is_file() else None

    out = Path(args.out)
    try:
        manifest = assemble(index, read_source, out, args.shard_size, (*DEFAULT_ALIASES, *args.alias), callee_map,
                            info, check=not args.no_check, build_shards=args.build_shards)
        if not args.no_check:
            code, seconds, peak, _, stderr = run_measured(
                ["cargo", "check", "--target", TARGET, "-p", "lf-rewrites"], out, cargo_env(out))
            if code != 0:
                print(stderr[-4000:], file=sys.stderr)
                return 1
            print(f"top crate checks ({seconds} s)")
        if args.build:
            code = subprocess.run(["cargo", "build", "--release", "--target", TARGET, "-p", "lf-rewrites"],
                                  cwd=out, env=cargo_env(out)).returncode
            if code != 0:
                return 1
    except RuntimeError as error:
        print(error, file=sys.stderr)
        return 1
    print(f"{manifest['included']} of {manifest['index_entries']} rewrites assembled in {len(manifest['shards'])} "
          f"shards ({manifest['switchable']} switchable, {manifest['excluded_count']} left out); "
          f"manifest: {out / 'manifest.json'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
