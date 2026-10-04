"""Regenerate the progress badges and the site's data files.

Usage: python scripts/update_progress.py

Reads   docs/data/progress.json    (counts only; it never holds code)
        docs/data/changelog.json   (one entry per commit, newest first)
Writes  docs/badges/*.svg          (badges referenced by README.md)
        docs/data/changelog.json   (only to fill in commit hashes)

The site pages fetch the two JSON files directly. There are no generated copies.

A changelog entry whose title equals a commit's subject line gets that commit's hash filled in.
"""

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "docs" / "data"
BADGES = ROOT / "docs" / "badges"

GREY = "#5b6470"
BLUE = "#2f77a8"
DEEP = "#124a73"
AMBER = "#8a5a00"

# Rough advance widths for 11px Verdana; textLength pins the rendered width to the estimate.
HAIR = set("ijl.,:;|!'")
NARROW = set("ftr()[] ")
WIDE = set("mwMW%")


def text_width(text):
    width = 0.0
    for ch in text:
        if ch in HAIR:
            width += 3.1
        elif ch in NARROW:
            width += 4.4
        elif ch in WIDE:
            width += 9.6
        elif ch.isupper() or ch.isdigit():
            width += 7.2
        else:
            width += 6.6
    return round(width)


def badge(label, message, color):
    lw, mw = text_width(label), text_width(message)
    left, right = lw + 12, mw + 12
    total = left + right
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{total}" height="20" role="img" '
        f'aria-label="{label}: {message}">'
        f"<title>{label}: {message}</title>"
        f'<clipPath id="r"><rect width="{total}" height="20" rx="3"/></clipPath>'
        f'<g clip-path="url(#r)">'
        f'<rect width="{left}" height="20" fill="#2a3038"/>'
        f'<rect x="{left}" width="{right}" height="20" fill="{color}"/>'
        f"</g>"
        f'<g fill="#fff" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" font-size="11" '
        f'text-anchor="middle">'
        f'<text x="{left / 2}" y="14" textLength="{lw}">{label}</text>'
        f'<text x="{left + right / 2}" y="14" textLength="{mw}">{message}</text>'
        f"</g></svg>\n"
    )


def count(n):
    return f"{n:,}"


def share(done, total):
    if total:
        return f"{count(done)} of {count(total)} ({done * 100 / total:.1f}%)"
    return f"{count(done)} functions"


def main():
    data = json.loads((DATA / "progress.json").read_text(encoding="utf-8"))
    game = data["functions"]["game"]
    stages = data["stages"]

    badges = {
        "phase": ("phase", f'{data["phase"]["index"]} {data["phase"]["name"].lower()}', AMBER),
        "functions": ("game functions", count(game) if game else "not measured yet", GREY),
        "named": ("named", share(stages["named"], game), BLUE),
        "rewritten": ("rewritten in Rust", share(stages["rewritten"], game), BLUE),
        "verified": ("verified", share(stages["verified"], game), DEEP),
        "structures": ("structures", count(data["structures"]), GREY),
        "decompiled-code": ("decompiled code in repo", "none", DEEP),
    }

    BADGES.mkdir(parents=True, exist_ok=True)
    for name, (label, message, color) in badges.items():
        (BADGES / f"{name}.svg").write_text(badge(label, message, color), encoding="utf-8", newline="\n")

    print(f"wrote {len(badges)} badges (updated {data['updated']})")

    update_changelog()


def commit_subjects():
    """Map commit subject -> full hash for this repository. Empty if there are no commits yet."""
    result = subprocess.run(
        ["git", "-C", str(ROOT), "log", "--format=%H%x09%s"],
        capture_output=True, text=True, encoding="utf-8",
    )
    subjects = {}
    if result.returncode == 0:
        for line in result.stdout.splitlines():
            full, _, subject = line.partition("\t")
            subjects.setdefault(subject, full)
    return subjects


def update_changelog():
    """Fill in commit hashes for changelog entries whose title equals a commit subject."""
    path = DATA / "changelog.json"
    entries = json.loads(path.read_text(encoding="utf-8"))
    subjects = commit_subjects()
    filled = 0
    for entry in entries:
        if not entry.get("commit") and entry["title"] in subjects:
            entry["commit"] = subjects[entry["title"]]
            filled += 1
    if filled:
        path.write_text(json.dumps(entries, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    pending = sum(1 for entry in entries if not entry.get("commit"))
    print(f"changelog: {len(entries)} entries ( {filled} hashes filled, {pending} not committed yet)")


if __name__ == "__main__":
    main()
