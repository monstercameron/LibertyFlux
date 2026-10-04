"""Insert a lane's devlog fragment into docs/devlog.html after checking it is safe to publish.

Usage: python scripts/integrate_devlog.py <lane> [--force]

Reads   .artifacts/scratch/<lane>/devlog-entry.html   (one <article class="entry" id="..."> element)
Edits   docs/devlog.html: adds the article at the top of the entries and a link in the contents.
Writes  .artifacts/scratch/<lane>/integrated.txt on success.

The fragment is refused if it contains anything the devlog must not publish: addresses, local
paths, user names, byte dumps, long hex strings, scripts, inline styles, or more than one article.
Use --force only after reading the fragment and deciding a flagged item is harmless.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DEVLOG = ROOT / "docs" / "devlog.html"

CHECKS = [
    ("hexadecimal address", re.compile(r"\b0x[0-9A-Fa-f]{5,}\b")),
    ("auto-generated function label", re.compile(r"\b(?:sub|FUN|loc|dword|off)_[0-9A-Fa-f]{5,}\b")),
    ("local path", re.compile(r"\b[A-Za-z]:\\")),
    ("user name", re.compile(r"mreca", re.I)),
    ("byte dump", re.compile(r"(?:\b[0-9A-Fa-f]{2}\s){8,}")),
    ("long hex string", re.compile(r"\b[0-9A-Fa-f]{32,}\b")),
    ("script or style", re.compile(r"<script|<style|\sstyle=|<pre|<img|<iframe", re.I)),
]


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    force = "--force" in sys.argv
    if len(args) != 1:
        sys.exit(__doc__)
    lane = args[0]
    fragment_path = ROOT / ".artifacts" / "scratch" / lane / "devlog-entry.html"
    if not fragment_path.exists():
        sys.exit(f"{lane}: no devlog-entry.html")
    fragment = fragment_path.read_text(encoding="utf-8", errors="replace").replace("\r\n", "\n").strip()

    articles = re.findall(r"<article\b", fragment)
    opening = re.search(r'<article class="entry" id="([a-z0-9-]+)">', fragment)
    title = re.search(r"<h2>(.*?)</h2>", fragment, re.S)
    problems = []
    if len(articles) != 1 or not opening or not fragment.endswith("</article>"):
        problems.append("fragment is not exactly one <article class=\"entry\" id=\"...\"> element")
    if not title:
        problems.append("no <h2> title")
    for name, pattern in CHECKS:
        hits = pattern.findall(fragment)
        if hits:
            problems.append(f"{name}: {len(hits)} found, e.g. {hits[0][:40]!r}")
    for cell in re.finditer(r"<tr\b[^>]*>(.*?)</tr>", fragment, re.S):
        cells = re.findall(r"<td\b([^>]*)>", cell.group(1))
        if any("data-label" not in attrs for attrs in cells[1:]):
            problems.append("a table cell after the first in its row has no data-label")
            break

    devlog = DEVLOG.read_text(encoding="utf-8")
    if opening and f'id="{opening.group(1)}"' in devlog:
        problems.append(f"id {opening.group(1)!r} already exists in the devlog")

    if problems and not force:
        print(f"{lane}: NOT integrated")
        for problem in problems:
            print("  -", problem)
        sys.exit(2)

    entry_id = opening.group(1)
    heading = re.sub(r"<[^>]+>", "", title.group(1)).strip()
    indented = "\n".join(("      " + line if line.strip() else line) for line in fragment.splitlines())
    marker = '    <div class="entries">\n'
    toc_marker = '    <ol class="toc">\n'
    if marker not in devlog or toc_marker not in devlog:
        sys.exit("devlog.html structure not recognised")
    devlog = devlog.replace(marker, marker + "\n" + indented + "\n", 1)
    devlog = devlog.replace(toc_marker, toc_marker + f'      <li><a href="#{entry_id}">{heading}</a></li>\n', 1)
    DEVLOG.write_text(devlog, encoding="utf-8", newline="\n")
    (fragment_path.parent / "integrated.txt").write_text(entry_id + "\n", encoding="utf-8")
    print(f"{lane}: integrated as #{entry_id}: {heading}" + (" (forced)" if problems else ""))


if __name__ == "__main__":
    main()
