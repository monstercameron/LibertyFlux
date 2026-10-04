"""Publication check: fail if a tracked file breaks the repository's publication rules (AGENTS.md rules 1 and 5).

The repository is public and every commit is pushed, so these are checked by machine on every push:

- nothing under orig/ or .artifacts/ is tracked, and notes.md is not tracked;
- no game or binary files (executables, libraries, game archives) are tracked;
- no tracked text file names a path on someone's machine or a user account;
- no inline assembly in rewrites or engine crates (a rewrite is Rust, not transcribed machine code);
- no tracked file is larger than 5 MB.

Run from anywhere inside the repository: python scripts/ci/check_publication.py
Exit status 0 when clean, 1 with a list of findings otherwise.
"""

import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip())
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

files = subprocess.run(["git", "-C", str(ROOT), "ls-files", "-z"], capture_output=True, check=True).stdout.decode("utf-8").split("\0")
findings = []
for name in filter(None, files):
    path = ROOT / name
    if name.startswith(FORBIDDEN_PREFIXES) or name in FORBIDDEN_FILES:
        findings.append(f"{name}: must not be tracked")
        continue
    if name.lower().endswith(BINARY_SUFFIXES):
        findings.append(f"{name}: binary or game file type must not be tracked")
        continue
    if not path.is_file():
        continue
    if path.stat().st_size > MAX_BYTES:
        findings.append(f"{name}: larger than 5 MB")
    if name == "scripts/ci/check_publication.py" or not name.lower().endswith(TEXT_SUFFIXES):
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    for number, line in enumerate(text.splitlines(), 1):
        if MACHINE_PATH.search(line) or (ACCOUNT and ACCOUNT.search(line)):
            findings.append(f"{name}:{number}: machine path or account name")
            break
    if name.endswith(".rs") and name.startswith(ASM_SCOPES) and not name.startswith(ASM_EXEMPT) and INLINE_ASM.search(text):
        findings.append(f"{name}: inline assembly in a rewrite or engine crate")

print(f"checked {len(files) - 1} tracked files")
if findings:
    print(f"{len(findings)} finding(s):")
    for finding in findings:
        print("  " + finding)
    sys.exit(1)
print("publication check passed")
