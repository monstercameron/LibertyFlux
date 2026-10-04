"""Commit helper: add one changelog entry, regenerate site data, stage the named files, commit.

Usage: python commit_one.py <entry.json>
entry.json: {"title": ..., "summary": ..., "changes": [...], "files": [...]}
"""

import datetime
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

ROOT = common.find_root()
CHANGELOG = ROOT / "docs" / "data" / "changelog.json"
TRAILER = "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"


def run(*args, stdin=None):
    result = subprocess.run(args, cwd=ROOT, input=stdin, capture_output=True, text=True, encoding="utf-8")
    if result.returncode != 0:
        print("FAILED:", " ".join(args), result.stdout, result.stderr)
        sys.exit(1)
    return result.stdout.strip()


def main():
    step = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    entries = json.loads(CHANGELOG.read_text(encoding="utf-8"))
    entries.insert(0, {
        "date": step.get("date") or datetime.date.today().isoformat(),
        "title": step["title"],
        "commit": None,
        "summary": step.get("summary", ""),
        "changes": step["changes"],
    })
    CHANGELOG.write_text(json.dumps(entries, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    run(sys.executable, "scripts/update_progress.py")
    run("git", "add", "--", *step["files"], "docs/data/changelog.json", "docs/data/progress.json", "docs/badges")
    staged = run("git", "diff", "--cached", "--name-only").splitlines()
    message = step["title"] + "\n\n" + "\n".join("- " + c for c in step["changes"]) + "\n\n" + TRAILER + "\n"
    run("git", "commit", "-q", "-F", "-", stdin=message)
    print(run("git", "rev-parse", "--short", "HEAD"), step["title"], f"({len(staged)} files)")
    if os.environ.get("LF_BATCH"):
        print("push deferred to the end of the batch")
    else:
        push = subprocess.run(["git", "push", "-q", "origin", "main"], cwd=ROOT, capture_output=True, text=True)
        print("pushed" if push.returncode == 0 else "PUSH FAILED: " + push.stderr.strip()[:200])
    remaining = run("git", "status", "--short")
    print("unstaged remaining:\n" + remaining if remaining else "working tree clean")


if __name__ == "__main__":
    main()
