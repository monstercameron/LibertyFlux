"""Contributor environment check: what a checkout of this repository needs, and the optional tools it can use.

Usage: python scripts/doctor.py [--json]

One PASS, WARN or FAIL line per check, then a summary; the exit status is 1 when any check fails. A FAIL is
something the tracked scripts or crates cannot work without; a WARN is optional (the game, local tools) or worth
fixing. The checks, in order:
  python      the interpreter running this (CI uses 3.11)
  git         git on PATH, and this folder inside a checkout
  rust        cargo and rustc run from the repository folder
  toolchain   rustc is the version rust-toolchain.toml pins
  targets     the pinned targets are installed (i686-pc-windows-msvc builds the 32-bit tools)
  workspace   `cargo metadata` reads the workspace (nothing is built) and build output goes under .artifacts/
  game-dir, orig-exe, java-home
              LIBERTYFLUX_GAME_DIR, LIBERTYFLUX_ORIG_EXE (default orig/GTAIV.exe) and JAVA_HOME: whether each is
              set and whether what it names exists, never its value
  ghidra      tools/ghidra, used by the inventory scripts
  hooks       the git hooks from scripts/hooks are installed
  notes       notes.md is present and ignored by git, and never tracked

Machine paths never appear in the output, so it can be pasted into an issue or a log: paths are shown from the
repository root, variables only as set or not set, and anything a tool prints passes through scrub(). Nothing is
installed or downloaded (rustup's automatic toolchain install is switched off for these calls, cargo runs
offline). Works on Windows and Linux, standard library only.
"""

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE / "ci"))
sys.path.insert(0, str(HERE / "hooks"))
from check_publication import ACCOUNT, MACHINE_PATH  # noqa: E402  (what counts as a machine path)
import install  # noqa: E402  (hook_state: is the hook git runs this repository's?)

PASS, WARN, FAIL = "PASS", "WARN", "FAIL"
CI_PYTHON = (3, 11)
MIN_PYTHON = (3, 8)
WIN32_TARGET = "i686-pc-windows-msvc"
TIMEOUT = 120  # seconds for any one command; cargo metadata on a cold cache is the slowest
MACHINE_PATH_RUN = re.compile("(?:" + MACHINE_PATH.pattern + r")[^\s'\"`]*")


def run_command(command, cwd):
    """(exit status, stdout, stderr) of a command, or None when it cannot be started or times out. rustup is
    told not to install a missing toolchain for it."""
    env = dict(os.environ, RUSTUP_AUTO_INSTALL="0")
    try:
        done = subprocess.run(command, cwd=cwd, capture_output=True, text=True, timeout=TIMEOUT, env=env)
    except (OSError, subprocess.TimeoutExpired):
        return None
    return done.returncode, done.stdout, done.stderr


def scrub(text, root=None, home=None):
    """Text with machine paths taken out: the repository folder becomes <repo>, the home folder ~, the account name
    <user>, and anything else the publication check calls a machine path <path>."""
    text = str(text)
    for folder, mark in ((root, "<repo>"), (home or Path.home(), "~")):
        if folder and len(str(folder)) > 3:  # never a bare drive or /
            for form in sorted({str(folder), Path(folder).as_posix()}, key=len, reverse=True):
                text = text.replace(form, mark)
    text = MACHINE_PATH_RUN.sub("<path>", text)
    return ACCOUNT.sub("<user>", text) if ACCOUNT else text


def first_line(text):
    return next((line.strip() for line in str(text).splitlines() if line.strip()), "no message")


def check_python(version):
    shown = ".".join(str(part) for part in version[:3])
    if tuple(version[:2]) >= CI_PYTHON:
        return PASS, f"Python {shown}"
    if tuple(version[:2]) >= MIN_PYTHON:
        return WARN, f"Python {shown}: the scripts are tested on 3.11, the version CI runs"
    return FAIL, f"Python {shown}: the scripts need 3.8 or later (CI runs 3.11)"


def check_git(root, run):
    found = run(["git", "--version"], root)
    if found is None or found[0]:
        return FAIL, "git is not on PATH"
    inside = run(["git", "rev-parse", "--is-inside-work-tree"], root)
    if inside is None or inside[0] or inside[1].strip() != "true":
        return FAIL, f"{found[1].strip()}, but this folder is not a git checkout"
    return PASS, f"{found[1].strip()}; this is a git checkout"


def read_toolchain(root):
    """(channel, targets) pinned by rust-toolchain.toml; (None, []) without one."""
    path = Path(root) / "rust-toolchain.toml"
    if not path.is_file():
        return None, []
    text = path.read_text(encoding="utf-8")
    channel = re.search(r'^\s*channel\s*=\s*"([^"]+)"', text, re.M)
    targets = re.search(r"^\s*targets\s*=\s*\[(.*?)\]", text, re.M | re.S)
    return (channel.group(1) if channel else None), (re.findall(r'"([^"]+)"', targets.group(1)) if targets else [])


def check_rust(root, run):
    """cargo and rustc, run where the toolchain pin applies. Returns (status, detail, rustc's version or None)."""
    versions = []
    for tool in ("cargo", "rustc"):
        found = run([tool, "--version"], root)
        if found is None:
            return FAIL, f"{tool} is not on PATH (Rust is installed with rustup: https://rustup.rs)", None
        if found[0]:
            return FAIL, f"{tool} --version failed: {scrub(first_line(found[2]), root)}", None
        versions.append(found[1].strip())
    match = re.search(r"\b(\d+\.\d+\.\d+)", versions[1])
    return PASS, "; ".join(versions), match.group(1) if match else None


def check_toolchain(root, rustc_version):
    channel, _ = read_toolchain(root)
    if channel is None:
        return WARN, "rust-toolchain.toml names no channel"
    if rustc_version is None:
        return WARN, f"not checked: rustc did not run (the pin is {channel}; rustup toolchain install {channel})"
    if not re.fullmatch(r"\d+\.\d+(?:\.\d+)?", channel):
        return PASS, f"rust-toolchain.toml pins the {channel} channel; rustc is {rustc_version}"
    if rustc_version == channel or rustc_version.startswith(channel + "."):
        return PASS, f"rustc {rustc_version}, as rust-toolchain.toml pins"
    return WARN, (f"rustc is {rustc_version} but rust-toolchain.toml pins {channel}; rustup applies the pin "
                  f"(rustup toolchain install {channel})")


def check_targets(root, run, rustc_version):
    _, targets = read_toolchain(root)
    if rustc_version is None:
        return WARN, "not checked: rustc did not run"
    if not targets:
        return PASS, "rust-toolchain.toml pins no extra targets"
    missing = []
    for target in targets:
        found = run(["rustc", "--print", "target-libdir", "--target", target], root)
        if found is None or found[0] or not Path(found[1].strip()).is_dir():
            missing.append(target)
    if not missing:
        return PASS, "installed: " + ", ".join(targets)
    why = f" ({WIN32_TARGET} builds the 32-bit tools, the checker's worker and the rewrite build check)" if WIN32_TARGET in missing else ""
    return WARN, f"missing: {', '.join(missing)}{why}; rustup target add {' '.join(missing)}"


def check_workspace(root, run, rustc_version):
    if rustc_version is None:
        return WARN, "not checked: cargo did not run"
    found = run(["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"], root)
    if found is None:
        return FAIL, "cargo metadata did not finish"
    if found[0]:
        return FAIL, f"cargo metadata failed: {scrub(first_line(found[2]), root)}"
    try:
        meta = json.loads(found[1])
    except ValueError:
        return FAIL, "cargo metadata printed something that is not JSON"
    members = len(meta.get("workspace_members", []))
    try:
        output = Path(os.path.relpath(Path(meta.get("target_directory", "")).resolve(), Path(root).resolve())).as_posix()
    except ValueError:  # another drive on Windows
        output = ".."
    if output == ".artifacts" or output.startswith(".artifacts/"):
        return PASS, f"{members} workspace members; build output goes to {output}"
    return WARN, (f"{members} workspace members, but build output goes outside .artifacts/ "
                  "(CARGO_TARGET_DIR set, or a cargo configuration overriding .cargo/config.toml?)")


def check_variable(env, name, kind, purpose, default=None, root=None):
    """Whether an environment variable is set and whether what it names exists, never its value. `kind` is
    "dir" or "file"; `default` (a path from the repository root) is what the scripts use when it is not set."""
    value = env.get(name)
    exists = Path.is_dir if kind == "dir" else Path.is_file
    noun = "folder" if kind == "dir" else "file"
    if value:
        return (PASS, f"{name} is set; the {noun} it names exists") if exists(Path(value)) else \
            (WARN, f"{name} is set, but there is no {noun} where it points")
    if default and root and exists(Path(root) / default):
        return PASS, f"{name} is not set; the default {default} is present"
    return WARN, f"{name} is not set{' and ' + default + ' is absent' if default else ''} (optional: {purpose})"


def check_java(env):
    value = env.get("JAVA_HOME")
    if not value:
        return WARN, "JAVA_HOME is not set (optional: Ghidra needs it, pointing at tools/jdk)"
    java = Path(value) / "bin" / ("java.exe" if os.name == "nt" else "java")
    if java.is_file():
        return PASS, "JAVA_HOME is set; bin/java is there"
    return WARN, "JAVA_HOME is set, but there is no bin/java under it"


def check_ghidra(root):
    if (Path(root) / "tools" / "ghidra").is_dir():
        return PASS, "tools/ghidra is present"
    return WARN, "tools/ghidra is absent (optional: the inventory scripts in scripts/ghidra need it)"


def check_hooks(root):
    try:
        states = {name: install.hook_state(root, name) for name in install.HOOKS}
    except (OSError, subprocess.CalledProcessError):
        return WARN, "not checked: git could not name the hooks folder"
    if all(state == "installed" for state in states.values()):
        return PASS, "installed: " + ", ".join(states)
    listed = ", ".join(f"{name} {state}" for name, state in states.items() if state != "installed")
    return WARN, f"{listed} (python scripts/hooks/install.py; a different hook is kept unless --force)"


def check_notes(root, run):
    present = (Path(root) / "notes.md").is_file()
    tracked = run(["git", "ls-files", "--error-unmatch", "notes.md"], root)
    if tracked is not None and tracked[0] == 0:
        return FAIL, "notes.md is tracked: it must never be committed (AGENTS.md rule 5); git rm --cached notes.md"
    if not present:
        return WARN, "notes.md is absent (optional: the owner's private notes, local paths and tool locations)"
    ignored = run(["git", "check-ignore", "-q", "notes.md"], root)
    if ignored is not None and ignored[0] == 0:
        return PASS, "notes.md is present, untracked and ignored by git"
    return WARN, "notes.md is present and untracked, but git does not ignore it, so `git add` could take it"


def collect(root, env=None, run=run_command, version=None):
    """Every check, in order, as (name, status, detail) with each detail scrubbed of machine paths."""
    env = os.environ if env is None else env
    results = [("python", *check_python(version or sys.version_info)), ("git", *check_git(root, run))]
    status, detail, rustc_version = check_rust(root, run)
    results.append(("rust", status, detail))
    results.append(("toolchain", *check_toolchain(root, rustc_version)))
    results.append(("targets", *check_targets(root, run, rustc_version)))
    results.append(("workspace", *check_workspace(root, run, rustc_version)))
    results.append(("game-dir", *check_variable(env, "LIBERTYFLUX_GAME_DIR", "dir",
                                                "your own copy of the game, read-only, for the format tools and captures")))
    results.append(("orig-exe", *check_variable(env, "LIBERTYFLUX_ORIG_EXE", "file",
                                                "the checker and the Ghidra scripts read the original executable",
                                                default="orig/GTAIV.exe", root=root)))
    results.append(("java-home", *check_java(env)))
    results.append(("ghidra", *check_ghidra(root)))
    results.append(("hooks", *check_hooks(root)))
    results.append(("notes", *check_notes(root, run)))
    return [(name, status, scrub(detail, root)) for name, status, detail in results]


def find_root(run=run_command):
    """The repository root from git, or else the folder above scripts/."""
    found = run(["git", "rev-parse", "--show-toplevel"], HERE)
    if found is not None and found[0] == 0 and found[1].strip():
        return Path(found[1].strip())
    return HERE.parent


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--json", action="store_true", help="print the checks and the summary as JSON")
    args = parser.parse_args(argv)
    results = collect(find_root())
    counts = {status: sum(1 for _, s, _ in results if s == status) for status in (PASS, WARN, FAIL)}
    summary = {"passed": counts[PASS], "warnings": counts[WARN], "failures": counts[FAIL]}
    if args.json:
        print(json.dumps({"checks": [{"name": n, "status": s, "detail": d} for n, s, d in results], "summary": summary}, indent=1))
    else:
        for name, status, detail in results:
            print(f"{status}  {name:<10} {detail}")
        print(f"summary: {counts[PASS]} passed, {counts[WARN]} warnings, {counts[FAIL]} failures")
    return 1 if counts[FAIL] else 0


if __name__ == "__main__":
    sys.exit(main())
