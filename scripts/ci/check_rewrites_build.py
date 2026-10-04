"""Does every verified rewrite compile from the repository alone?

The checker compiled each rewrite inside its lane's own crate, with whatever runtime that lane had. The tree
only keeps the rewrite, so a file that leaned on a lane-only helper no longer builds, and what was proven can
no longer be rebuilt. This script includes every file in rewrites/verified/index.json as its own module of a
scratch crate that depends only on the shared runtime (crates/tools/lf-checker-rt), type-checks it for the
32-bit Windows target (no linker needed, so it runs on any host with that Rust target installed), drops the
files that fail and repeats until the rest compiles. Inner doc comments and inner attributes, which a lane's
crate root could hold but an included file cannot, are neutralised in the scratch copy only.

Usage: python scripts/ci/check_rewrites_build.py [--out PATH] [--edition 2021|2024]
Writes the per-file errors as JSON (default .artifacts/build/rewrite-check/result.json) and exits 1 if any
file fails. The scratch crate lives under .artifacts/build/rewrite-check/, which git ignores. The JSON also
counts the failures by error kind under "error_classes" (see error_class).

  --only FILE...     check only these files: index entries (relative to rewrites/verified, with or without that
                     prefix) or rewrite files on disk outside the index, each under the address in its name
                     (fn_<8 hex digits>.rs) or its `// original:` header; a folder means every .rs file under
                     it. Fast, for a lane's own output.
  --alias OLD=NEW    an experiment, repeatable: after the strict check, check the files that failed again with
                     crate OLD (a lane's runtime, such as lf_k2_rt) aliased to NEW (lf_checker_rt, the shared
                     runtime) and report how many more compile, under "alias" in the JSON. The strict result,
                     its "failed" list and the exit status are unchanged by it.
"""

import argparse
import json
import os
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / ".artifacts" / "build" / "rewrite-check"
TARGET = "i686-pc-windows-msvc"
ERROR_LINE = re.compile(r"^src[\\/]rw[\\/]([0-9a-f]{8})\.rs:\d+:\d+: (error.*)$")
SHARED_RUNTIME = "lf_checker_rt"
# What an alias may point at: the shared runtime, or a crate every Rust build has.
ALIAS_TARGETS = (SHARED_RUNTIME, "core", "alloc", "std")
CRATE_NAME = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")
FILE_ADDRESS = re.compile(r"fn_([0-9a-fA-F]{8})\.rs$")
HEADER = re.compile(r"//\s*original:\s*(0x[0-9A-Fa-f]+)\b")
VERIFIED = "rewrites/verified/"


def error_class(message):
    """A compiler error reduced to its kind, for counting: names in backticks become X and the explanation after
    the message's own colon is dropped, so `error[E0425]: cannot find function `f` in this scope: not found in
    this scope` becomes `error[E0425]: cannot find function X in this scope`. Pure."""
    text = re.sub(r"`[^`]*`", "X", message).strip()
    head, _, rest = text.partition(": ")
    return head + (": " + rest.split(": ")[0] if rest else "")


def class_counts(rows):
    """{error kind: number of files} over failed rows, most frequent first."""
    return dict(Counter(error_class(r["error"]) for r in rows).most_common())


def parse_aliases(items):
    """{old crate name: new crate name} from OLD=NEW strings, hyphens read as underscores as Cargo does. Raises
    ValueError for a malformed item, an OLD the check already builds against, or a NEW it cannot provide."""
    aliases = {}
    for item in items:
        old, sep, new = (part.strip().replace("-", "_") for part in item.partition("="))
        if not sep or not CRATE_NAME.match(old) or not CRATE_NAME.match(new):
            raise ValueError(f"bad alias {item!r}: use OLD=NEW with two crate names")
        if old in ALIAS_TARGETS:
            raise ValueError(f"bad alias {item!r}: {old} is a crate the check already builds against")
        if new not in ALIAS_TARGETS:
            raise ValueError(f"bad alias {item!r}: NEW is one of {', '.join(ALIAS_TARGETS)}")
        aliases[old] = new
    return aliases


def file_address(path):
    """The address in a rewrite file's name (fn_<8 hex digits>.rs), else in its `// original:` header, or None."""
    match = FILE_ADDRESS.search(Path(path).name)
    if match:
        return int(match.group(1), 16)
    text = Path(path).read_text(encoding="utf-8", errors="replace")
    match = HEADER.match(next((line.strip() for line in text.splitlines() if line.strip()), ""))
    return int(match.group(1), 16) if match else None


def select(index, names):
    """The entries --only names, in the order given. A name is an index entry's file (relative to
    rewrites/verified, with or without that prefix, either slash), else a rewrite file on disk, which becomes an
    entry of its own (with its path) under the address file_address finds; a folder stands for every .rs file
    under it (PowerShell does not expand wildcards for a program). Raises ValueError for a name that is neither,
    a file with no address, or two different files with one address."""
    by_file = {e["file"]: e for e in index}
    chosen, keys = [], {}
    expanded = []
    for name in names:
        expanded.extend(sorted(str(p) for p in Path(name).rglob("*.rs")) if Path(name).is_dir() else [name])
    for name in expanded:
        relative = name.replace("\\", "/")
        relative = relative[2:] if relative.startswith("./") else relative
        relative = relative[len(VERIFIED):] if relative.startswith(VERIFIED) else relative
        if relative in by_file:
            entry = by_file[relative]
        elif Path(name).is_file():
            address = file_address(name)
            if address is None:
                raise ValueError(f"{name}: no address in its name (fn_<8 hex digits>.rs) or its // original: header")
            entry = {"address": f"0x{address:08X}", "file": name.replace("\\", "/"), "path": str(Path(name).resolve())}
        else:
            raise ValueError(f"{name}: neither an entry of rewrites/verified/index.json nor a file")
        key = entry["address"][2:].lower()
        if key in keys:
            if keys[key].get("path") == entry.get("path") and keys[key]["file"] == entry["file"]:
                continue  # the same file named twice
            raise ValueError(f"{name}: another file given has the same address 0x{key.upper()}")
        keys[key] = entry
        chosen.append(entry)
    return chosen


def write_crate(entries, failed, edition, aliases=None):
    (WORK / "src" / "rw").mkdir(parents=True, exist_ok=True)
    runtime = (ROOT / "crates" / "tools" / "lf-checker-rt").as_posix()
    (WORK / "Cargo.toml").write_text(
        f'[package]\nname = "lf-rewrite-check"\nversion = "0.0.0"\nedition = "{edition}"\npublish = false\n\n'
        f'[lib]\npath = "src/lib.rs"\n\n[dependencies]\nlf-checker-rt = {{ path = "{runtime}" }}\n\n'
        "# Not part of the repository's workspace.\n[workspace]\n", encoding="utf-8")
    lines = ["#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn)]", "#[macro_use] extern crate lf_checker_rt;"]
    # An `extern crate` with `as` in the crate root puts the alias in every module's extern prelude.
    lines += [f"extern crate {new} as {old};" for old, new in sorted((aliases or {}).items())]
    for entry in entries:
        key = entry["address"][2:].lower()
        if key in failed:
            continue
        source = Path(entry["path"]) if entry.get("path") else ROOT / "rewrites" / "verified" / entry["file"]
        copy = WORK / "src" / "rw" / f"{key}.rs"
        # An included file cannot carry inner doc comments or inner attributes; lanes' crate roots could.
        text = source.read_text(encoding="utf-8", errors="replace")
        text = re.sub(r"(?m)^(\s*)//!", r"\1//", text)
        text = re.sub(r"(?m)^\s*#!\[[^\]]*\]\s*$", "", text)
        copy.write_text(text, encoding="utf-8")
        lines.append(f'pub mod m_{key} {{ #[allow(unused_imports)] use lf_checker_rt::*; include!("rw/{key}.rs"); }}')
    (WORK / "src" / "lib.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")


def run(entries, edition, rounds=30, aliases=None):
    """Type-check, dropping failing files each round. Returns {address key: first error}."""
    failed = {}
    env = dict(os.environ, CARGO_TARGET_DIR=str(WORK / "target"))
    for _ in range(rounds):
        write_crate(entries, failed, edition, aliases)
        result = subprocess.run(["cargo", "check", "--target", TARGET, "--message-format", "short"],
                                cwd=WORK, env=env, capture_output=True, text=True)
        if result.returncode == 0:
            return failed
        new = {}
        for line in result.stderr.splitlines():
            match = ERROR_LINE.match(line.strip())
            if match and match.group(1) not in failed:
                new.setdefault(match.group(1), match.group(2))
        if not new:
            raise SystemExit("cargo check failed without a per-file error:\n" + result.stderr[-4000:])
        failed.update(new)
    raise SystemExit(f"still failing after {rounds} rounds")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", default=str(WORK / "result.json"))
    parser.add_argument("--edition", default="2024", choices=("2021", "2024"),
                        help="2024 is the workspace edition the assembled library will use")
    parser.add_argument("--only", nargs="+", action="extend", metavar="FILE",
                        help="check only these files (index entries, or rewrite files on disk)")
    parser.add_argument("--alias", action="append", default=[], metavar="OLD=NEW",
                        help=f"experiment: retry the failures with crate OLD aliased to NEW ({SHARED_RUNTIME}); repeatable")
    args = parser.parse_args(argv)
    try:
        aliases = parse_aliases(args.alias)
    except ValueError as problem:
        parser.error(str(problem))
    index = json.loads((ROOT / "rewrites" / "verified" / "index.json").read_text(encoding="utf-8"))
    if args.only:
        try:
            index = select(index, args.only)
        except ValueError as problem:
            parser.error(str(problem))
    failed = run(index, args.edition)
    files = {e["address"][2:].lower(): e["file"] for e in index}
    rows = [{"file": files[key], "error": error} for key, error in sorted(failed.items())]
    result = {"checked": len(index), "edition": args.edition, "failed": rows, "error_classes": class_counts(rows)}
    if aliases:
        retry = [e for e in index if e["address"][2:].lower() in failed]
        still = run(retry, args.edition, aliases=aliases) if retry else {}
        still_rows = [{"file": files[key], "error": error} for key, error in sorted(still.items())]
        result["alias"] = {"aliases": aliases, "retried": len(retry),
                           "now_compile": sorted(files[key] for key in failed if key not in still),
                           "failed": still_rows, "error_classes": class_counts(still_rows)}
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(result, indent=1) + "\n", encoding="utf-8")
    print(f"{len(index) - len(rows)} of {len(index)} rewrites compile against the shared runtime alone ({len(rows)} do not)")
    for message, n in Counter(re.sub(r"`[^`]*`", "X", r["error"])[:100] for r in rows).most_common(10):
        print(f"  {n:5}  {message}")
    if aliases:
        named = ", ".join(f"{old}={new}" for old, new in sorted(aliases.items()))
        print(f"with the alias(es) {named}: {len(result['alias']['now_compile'])} more compile, {len(still_rows)} of the "
              f"{len(retry)} failures still fail (an experiment: the result above and the exit status stay strict)")
    return 1 if rows else 0


if __name__ == "__main__":
    sys.exit(main())
