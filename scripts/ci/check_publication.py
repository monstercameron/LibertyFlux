"""Publication check: fail if a tracked file breaks the repository's publication rules (AGENTS.md rules 1 and 5).

The repository is public and every commit is pushed, so these are checked by machine on every push:

- nothing under orig/ or .artifacts/ is tracked, and notes.md is not tracked;
- no game or binary files (executables, libraries, game archives) are tracked;
- no tracked text file names a path on someone's machine or a user account;
- no inline assembly in rewrites or engine crates (a rewrite is Rust, not transcribed machine code);
- no tracked file is larger than 5 MB;
- no devlog entry (in the database) holds a machine path, a game function address, disassembly or a byte dump.

Run from anywhere inside the repository: python scripts/ci/check_publication.py
Exit status 0 when clean, 1 with a list of findings otherwise.

Narrower runs, for a pre-commit hook (scripts/hooks/pre-commit) or a quick look at a few files:
  --files PATH...   check only these files (paths from the current folder), as they are on disk; the devlog
                    database is checked when it is one of them
  --staged          check what the next commit will hold: the files staged for it (added, copied, modified or
                    renamed), read from the index rather than the disk, so a partly staged file is checked as
                    staged; with --files, those files as staged
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

MAX_BYTES = 5 * 1024 * 1024
FORBIDDEN_PREFIXES = ("orig/", ".artifacts/")
FORBIDDEN_FILES = ("notes.md",)
BINARY_SUFFIXES = (".exe", ".dll", ".pdb", ".lib", ".obj", ".rpf", ".img", ".wad", ".sco", ".wtd", ".wdr", ".wft", ".wdd")
TEXT_SUFFIXES = (".rs", ".py", ".md", ".toml", ".json", ".html", ".css", ".js", ".yml", ".yaml", ".txt", ".ps1", ".svg", ".lock", ".c", ".h")
MACHINE_PATH = re.compile(r"[A-Za-z]:[\\/]+Users[\\/]|/home/[a-z][\w.-]*/|/Users/[A-Za-z][\w.-]*/|steamapps")
INLINE_ASM = re.compile(r"\b(?:global_)?asm!\s*\(|#\[\s*naked\s*\]|#\[\s*unsafe\s*\(\s*naked\s*\)\s*\]")
# The name of the account running the check, when there is a personal one (not on a build machine).
try:
    import getpass
    _account = getpass.getuser()
except Exception:
    _account = ""
ACCOUNT = re.compile(r"\b" + re.escape(_account) + r"\b", re.I) if len(_account) >= 4 and _account.lower() not in (
    "runner", "root", "admin", "administrator", "user", "runneradmin") else None
ASM_SCOPES = ("rewrites/", "crates/")
ASM_EXEMPT = ("crates/tools/",)  # tooling may need it; rewrites and engine code may not
DEVLOG = "docs/data/devlog.sqlite"
SELF = "scripts/ci/check_publication.py"  # holds the patterns themselves, so its text is not scanned


def repo_root():
    """The repository's top folder, from git."""
    return Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip())


def check_file(name, size, read_text):
    """Findings for one file, by its path from the repository root. `size` is its length in bytes, or None when
    there is no such file to read (then only its name is checked); `read_text` returns its text when called."""
    if name.startswith(FORBIDDEN_PREFIXES) or name in FORBIDDEN_FILES:
        return [f"{name}: must not be tracked"]
    if name.lower().endswith(BINARY_SUFFIXES):
        return [f"{name}: binary or game file type must not be tracked"]
    if size is None:
        return []
    findings = []
    # The devlog's database holds every entry and grows with the project; its content is checked entry by
    # entry when it is written (scripts/integrate_devlog.py), so it gets a larger allowance.
    limit = 40 * 1024 * 1024 if name == DEVLOG else MAX_BYTES
    if size > limit:
        findings.append(f"{name}: larger than {limit // (1024 * 1024)} MB")
    if name == SELF or not name.lower().endswith(TEXT_SUFFIXES):
        return findings
    text = read_text()
    for number, line in enumerate(text.splitlines(), 1):
        if MACHINE_PATH.search(line) or (ACCOUNT and ACCOUNT.search(line)):
            findings.append(f"{name}:{number}: machine path or account name")
            break
    if name.endswith(".rs") and name.startswith(ASM_SCOPES) and not name.startswith(ASM_EXEMPT) and INLINE_ASM.search(text):
        findings.append(f"{name}: inline assembly in a rewrite or engine crate")
    return findings


def check_devlog(path):
    """The devlog database is binary, so the line scan above never reads it. Check every entry's text: no machine
    path or account name, no game function addresses, no disassembly, no byte dumps (AGENTS.md: neither site page
    may contain decompiled code, disassembly, game function addresses or game data). The disassembly pattern needs
    an operand form (`mov eax,`, `dword ptr [`) so that prose such as "call checks" does not match."""
    import sqlite3

    register = r"(?:e[abcd]x|e[sd]i|e[sb]p|[abcd][lhx]|xmm[0-7]|st\([0-7]\))"
    rules = (
        ("machine path or account name", MACHINE_PATH),
        ("game function address", re.compile(r"\b0x00[4-9a-fA-F][0-9a-fA-F]{5}\b|\b(?:sub|FUN)_00[4-9a-fA-F][0-9a-fA-F]{5}\b")),
        ("disassembly", re.compile(r"\b(?:mov|movzx|movsx|movss|movaps|lea|push|pop|call|cmp|test|xor|and|or|add|sub|imul|jmp|fld|fstp|shl|shr|sar)\s+(?:"
                                   + register + r"\s*,|(?:byte|word|dword|qword)\s+ptr\s*\[|\[" + register + ")", re.I)),
        ("byte dump", re.compile(r"(?:\b[0-9a-f]{2}\s+){12,}", re.I)),
    )
    found = []
    connection = sqlite3.connect(f"file:{path.as_posix()}?mode=ro", uri=True)
    try:
        for entry, title, body in connection.execute("SELECT id, title, text FROM posts"):
            for label, pattern in rules + ((("machine path or account name", ACCOUNT),) if ACCOUNT else ()):
                if pattern.search(title or "") or pattern.search(body or ""):
                    found.append(f"{DEVLOG}: entry {entry}: {label}")
    finally:
        connection.close()
    return found


def check_tracked(root):
    """The full check over every tracked file, as the pipeline runs it. Returns (git's file list, findings); the
    list ends with an empty name, as `git ls-files -z` output splits."""
    files = subprocess.run(["git", "-C", str(root), "ls-files", "-z"], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
    findings = []
    for name in filter(None, files):
        path = root / name
        findings.extend(check_file(name, path.stat().st_size if path.is_file() else None,
                                   lambda: path.read_text(encoding="utf-8", errors="replace")))
    devlog = root / DEVLOG
    if DEVLOG in files and devlog.is_file():
        findings.extend(check_devlog(devlog))
    return files, findings


def repository_names(root, paths):
    """Paths given on the command line (from the current folder, or absolute) as paths from the repository root,
    with forward slashes. Raises ValueError for one outside the repository."""
    names = []
    for given in paths:
        # The folder is resolved (git names the root by its real path) but not the file, which may be a link.
        absolute = Path(given).absolute()
        try:
            relative = Path(os.path.relpath(absolute.parent.resolve() / absolute.name, root.resolve())).as_posix()
        except ValueError:  # another drive on Windows
            relative = ".."
        if relative == ".." or relative.startswith("../"):
            raise ValueError(f"{given}: outside the repository")
        names.append(relative)
    return names


def check_on_disk(root, names):
    """Findings for the named files as they are on disk; the devlog database is checked when it is named."""
    findings = []
    for name in names:
        path = root / name
        findings.extend(check_file(name, path.stat().st_size if path.is_file() else None,
                                   lambda: path.read_text(encoding="utf-8", errors="replace")))
    if DEVLOG in names and (root / DEVLOG).is_file():
        findings.extend(check_devlog(root / DEVLOG))
    return findings


def staged_names(root):
    """The files staged for the next commit, except deletions: added, copied, modified or renamed."""
    out = subprocess.run(["git", "-C", str(root), "diff", "--cached", "--name-only", "-z", "--diff-filter=ACMR"],
                         capture_output=True, check=True).stdout.decode("utf-8")
    return [name for name in out.split("\0") if name]


def staged_blobs(root, names):
    """{name: bytes} of each name's content in the index, read in one `git cat-file --batch` call; a name the
    index does not hold is left out."""
    if not names:
        return {}
    request = "".join(f":{name}\n" for name in names).encode("utf-8")
    out = subprocess.run(["git", "-C", str(root), "cat-file", "--batch"], input=request, capture_output=True, check=True).stdout
    blobs, position = {}, 0
    for name in names:
        end = out.index(b"\n", position)
        header = out[position:end].split()
        position = end + 1
        if len(header) == 3 and header[1] == b"blob":  # "<object id> blob <size>", then the content and a newline
            size = int(header[2])
            blobs[name] = out[position:position + size]
            position += size + 1
    return blobs


def check_staged(root, names):
    """Findings for the named files as the index holds them (what the next commit will contain)."""
    blobs = staged_blobs(root, names)
    findings = []
    for name in names:
        blob = blobs.get(name)
        findings.extend(check_file(name, None if blob is None else len(blob),
                                   lambda: blob.decode("utf-8", errors="replace")))
    if DEVLOG in blobs:
        # SQLite reads a file, so the staged database is checked from a temporary copy.
        handle, copy = tempfile.mkstemp(suffix=".sqlite")
        try:
            with os.fdopen(handle, "wb") as fh:
                fh.write(blobs[DEVLOG])
            findings.extend(check_devlog(Path(copy)))
        finally:
            os.unlink(copy)
    return findings


def report(checked, findings):
    """Print the result the way the pipeline reads it; returns the exit status."""
    print(checked)
    if findings:
        print(f"{len(findings)} finding(s):")
        for finding in findings:
            print("  " + finding)
        return 1
    print("publication check passed")
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--files", nargs="+", action="extend", metavar="PATH",
                        help="check only these files (paths from the current folder)")
    parser.add_argument("--staged", action="store_true",
                        help="check the files staged for commit (or those of --files), as the index holds them")
    args = parser.parse_args(argv)
    root = repo_root()
    if args.files is None and not args.staged:
        files, findings = check_tracked(root)
        return report(f"checked {len(files) - 1} tracked files", findings)
    try:
        names = repository_names(root, args.files) if args.files is not None else staged_names(root)
    except ValueError as problem:
        parser.error(str(problem))
    if args.staged:
        return report(f"checked {len(names)} staged files", check_staged(root, names))
    return report(f"checked {len(names)} given files", check_on_disk(root, names))


if __name__ == "__main__":
    sys.exit(main())
