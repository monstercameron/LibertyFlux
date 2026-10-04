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
file fails. The scratch crate lives under .artifacts/build/rewrite-check/, which git ignores.
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


def write_crate(entries, failed, edition):
    (WORK / "src" / "rw").mkdir(parents=True, exist_ok=True)
    runtime = (ROOT / "crates" / "tools" / "lf-checker-rt").as_posix()
    (WORK / "Cargo.toml").write_text(
        f'[package]\nname = "lf-rewrite-check"\nversion = "0.0.0"\nedition = "{edition}"\npublish = false\n\n'
        f'[lib]\npath = "src/lib.rs"\n\n[dependencies]\nlf-checker-rt = {{ path = "{runtime}" }}\n\n'
        "# Not part of the repository's workspace.\n[workspace]\n", encoding="utf-8")
    lines = ["#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn)]", "#[macro_use] extern crate lf_checker_rt;"]
    for entry in entries:
        key = entry["address"][2:].lower()
        if key in failed:
            continue
        source = ROOT / "rewrites" / "verified" / entry["file"]
        copy = WORK / "src" / "rw" / f"{key}.rs"
        # An included file cannot carry inner doc comments or inner attributes; lanes' crate roots could.
        text = source.read_text(encoding="utf-8", errors="replace")
        text = re.sub(r"(?m)^(\s*)//!", r"\1//", text)
        text = re.sub(r"(?m)^\s*#!\[[^\]]*\]\s*$", "", text)
        copy.write_text(text, encoding="utf-8")
        lines.append(f'pub mod m_{key} {{ #[allow(unused_imports)] use lf_checker_rt::*; include!("rw/{key}.rs"); }}')
    (WORK / "src" / "lib.rs").write_text("\n".join(lines) + "\n", encoding="utf-8")


def run(entries, edition, rounds=30):
    """Type-check, dropping failing files each round. Returns {address key: first error}."""
    failed = {}
    env = dict(os.environ, CARGO_TARGET_DIR=str(WORK / "target"))
    for _ in range(rounds):
        write_crate(entries, failed, edition)
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
    args = parser.parse_args(argv)
    index = json.loads((ROOT / "rewrites" / "verified" / "index.json").read_text(encoding="utf-8"))
    failed = run(index, args.edition)
    files = {e["address"][2:].lower(): e["file"] for e in index}
    rows = [{"file": files[key], "error": error} for key, error in sorted(failed.items())]
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps({"checked": len(index), "edition": args.edition, "failed": rows}, indent=1) + "\n", encoding="utf-8")
    print(f"{len(index) - len(rows)} of {len(index)} rewrites compile against the shared runtime alone ({len(rows)} do not)")
    for message, n in Counter(re.sub(r"`[^`]*`", "X", r["error"])[:100] for r in rows).most_common(10):
        print(f"  {n:5}  {message}")
    return 1 if rows else 0


if __name__ == "__main__":
    sys.exit(main())
