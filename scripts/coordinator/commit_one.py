"""Commit helper: add one changelog entry, regenerate site data, stage the named files, commit.

Usage: python commit_one.py <entry.json>
entry.json: {"title": ..., "summary": ..., "changes": [...], "files": [...]}

Only the named files (plus the changelog, progress data and badges) are committed. If any step fails, the
changelog entry is rolled back and the script exits 1, so no orphan entry is left behind.
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


class StepFailed(Exception):
    """A git or regeneration step failed; the changelog entry has been rolled back."""


def run(*args, stdin=None, root=None):
    result = subprocess.run(args, cwd=root or ROOT, input=stdin, capture_output=True, text=True, encoding="utf-8")
    if result.returncode != 0:
        raise StepFailed(" ".join(args) + "\n" + result.stdout + result.stderr)
    return result.stdout.strip()


def add_entry(entries, step, date=None):
    """The changelog with the step's entry on top. An orphan left on top by an earlier failed run (same title,
    commit still null) is replaced rather than duplicated."""
    entry = {
        "date": step.get("date") or date or datetime.date.today().isoformat(),
        "title": step["title"],
        "commit": None,
        "summary": step.get("summary", ""),
        "changes": step["changes"],
    }
    rest = entries[1:] if entries and entries[0].get("title") == step["title"] and entries[0].get("commit") is None else entries
    return [entry] + list(rest)


def commit(step, root=None, push=True):
    """Add the entry, regenerate the site data, stage the step's files and commit exactly those paths.

    Atomic for the changelog: if regeneration, staging or the commit fails, changelog.json is restored byte for
    byte and unstaged, and StepFailed is raised, so a failed call leaves no orphan entry for the next call to
    stack another on. Only the named paths are committed (`git commit -- paths`); anything else that happened to
    be staged stays staged and out of this commit. Returns the short hash.
    """
    root = Path(root or ROOT)
    changelog = root / "docs" / "data" / "changelog.json"
    original = changelog.read_bytes()
    entries = add_entry(json.loads(original.decode("utf-8")), step)
    paths = [*step["files"], "docs/data/changelog.json", "docs/data/progress.json", "docs/badges"]
    message = step["title"] + "\n\n" + "\n".join("- " + c for c in step["changes"]) + "\n\n" + TRAILER + "\n"
    changelog.write_text(json.dumps(entries, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    try:
        run(sys.executable, "scripts/update_progress.py", root=root)
        run("git", "add", "--", *paths, root=root)
        staged = run("git", "diff", "--cached", "--name-only", "--", *paths, root=root).splitlines()
        run("git", "commit", "-q", "-F", "-", "--", *paths, stdin=message, root=root)
    except StepFailed:
        changelog.write_bytes(original)
        subprocess.run(["git", "reset", "-q", "--", "docs/data/changelog.json"], cwd=root, capture_output=True)
        raise
    head = run("git", "rev-parse", "--short", "HEAD", root=root)
    print(head, step["title"], f"({len(staged)} files)")
    if not push:
        return head
    if os.environ.get("LF_BATCH"):
        print("push deferred to the end of the batch")
    else:
        result = subprocess.run(["git", "push", "-q", "origin", "main"], cwd=root, capture_output=True, text=True)
        print("pushed" if result.returncode == 0 else "PUSH FAILED: " + result.stderr.strip()[:200])
    remaining = run("git", "status", "--short", root=root)
    print("unstaged remaining:\n" + remaining if remaining else "working tree clean")
    return head


def main():
    step = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    try:
        commit(step)
    except StepFailed as error:
        print("FAILED:", error)
        print("changelog entry rolled back; nothing committed")
        sys.exit(1)


if __name__ == "__main__":
    main()
