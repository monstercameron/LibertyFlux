"""The devlog's store: one SQLite database, docs/data/devlog.sqlite, read in the browser by docs/devlog.html.

Every devlog entry is one row in `posts`. The page loads the database, lists the posts as a blog, and searches
them locally; nothing is queried on a server. Entries are added with scripts/integrate_devlog.py (lane
fragments, after the publication checks) or with this script.

    python scripts/devlog_db.py count
    python scripts/devlog_db.py list [text]          ids, dates and titles (optionally only those containing text)
    python scripts/devlog_db.py get <id>              print the entry as an <article> fragment
    python scripts/devlog_db.py put <fragment.html>   insert the entry, or replace the one with the same id
    python scripts/devlog_db.py migrate <devlog.html> build the database from the old single-page devlog

A fragment is exactly one <article class="entry" id="..."> element holding an entry-meta block with a
<time datetime="YYYY-MM-DD">, an <h2> title, and the body.
"""

import html as html_module
import re
import sqlite3
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DB = ROOT / "docs" / "data" / "devlog.sqlite"

SCHEMA = """
CREATE TABLE IF NOT EXISTS posts (
    id       TEXT PRIMARY KEY,      -- the entry's anchor, as in devlog.html#id
    seq      INTEGER NOT NULL,      -- order of publication; the newest entry has the highest number
    date     TEXT NOT NULL,         -- YYYY-MM-DD
    title    TEXT NOT NULL,
    summary  TEXT NOT NULL,         -- the opening paragraph as plain text
    html     TEXT NOT NULL,         -- the body, after the title
    text     TEXT NOT NULL,         -- title and body as plain text, for searching
    verified INTEGER NOT NULL,      -- how many findings carry each evidence label
    inferred INTEGER NOT NULL,
    unknown  INTEGER NOT NULL,
    added    TEXT NOT NULL          -- when the row was written, local time
);
CREATE TABLE IF NOT EXISTS tags (post_id TEXT NOT NULL REFERENCES posts(id) ON DELETE CASCADE, tag TEXT NOT NULL, PRIMARY KEY (post_id, tag));
CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE INDEX IF NOT EXISTS posts_seq ON posts(seq);
"""

# Topics are derived from the entry's id and title, so lanes do not have to tag anything. An entry can have
# several; one that matches none is a general finding.
TOPICS = [
    ("hourly review", r"hourly-reviews"),
    ("production batch", r"\bproduction-|production batch"),
    ("naming", r"\bnaming-|\bnam(e|es|ed|ing)\b"),
    ("checker", r"checker|verdict|wrong version|mutant|contract|proof|vacuous|\bfills?\b|trials?\b"),
    ("inventory", r"inventory|boundar|pointer-only|fragment|not functions|functions? (are|is) |entries"),
    ("operations", r"supervisor|memory|commit headroom|lanes? |operating|launcher|swarm|laptop|agent"),
    ("file formats", r"archive|texture|collision|format|reader|model|save|animation bank|audio bank|cutscene"),
    ("64-bit lift", r"64-bit|\blift"),
    ("tooling", r"dashboard|ledger|pipeline|tooling|scripts?\b|loader|capture|ghidra"),
    ("engine", r"engine|render|thread|streaming|script machine|native|audio|physics|ped|vehicle|task"),
    ("research", r"research|other projects|recomp|decomp|dlss|upscal|guide"),
]


def plain(fragment):
    text = re.sub(r"<(script|style)\b.*?</\1>", " ", fragment, flags=re.S | re.I)
    text = re.sub(r"<[^>]+>", " ", text)
    return re.sub(r"\s+", " ", html_module.unescape(text)).strip()


def parse(fragment):
    fragment = fragment.replace("\r\n", "\n").strip()
    opening = re.match(r'<article class="entry" id="([a-z0-9-]+)">', fragment)
    if not opening or not fragment.endswith("</article>") or len(re.findall(r"<article\b", fragment)) != 1:
        raise ValueError("not exactly one <article class=\"entry\" id=\"...\"> element")
    inner = fragment[opening.end():-len("</article>")].strip()
    date = re.search(r'<time datetime="(\d{4}-\d{2}-\d{2})"', inner)
    title = re.search(r"<h2>(.*?)</h2>", inner, re.S)
    if not date or not title:
        raise ValueError("entry needs a <time datetime> and an <h2> title")
    body = inner[title.end():].strip()
    body = "\n".join(line.strip() for line in body.splitlines() if line.strip())
    first = re.search(r"<p>(.*?)</p>", body, re.S)
    title_text = plain(title.group(1))
    text = title_text + " " + plain(body)
    haystack = (opening.group(1) + " " + title_text).lower()
    tags = [name for name, pattern in TOPICS if re.search(pattern, haystack)]
    return {"id": opening.group(1), "date": date.group(1), "title": title_text, "summary": plain(first.group(1)) if first else "",
            "html": body, "text": text, "tags": tags or ["finding"],
            "verified": len(re.findall(r'class="evidence verified"', body)),
            "inferred": len(re.findall(r'class="evidence">Inferred', body)),
            "unknown": len(re.findall(r'class="evidence">Unknown', body))}


def connect():
    DB.parent.mkdir(parents=True, exist_ok=True)
    connection = sqlite3.connect(DB)
    connection.execute("PRAGMA journal_mode = DELETE")
    connection.execute("PRAGMA foreign_keys = ON")
    connection.executescript(SCHEMA)
    return connection


def has(entry_id):
    with connect() as connection:
        return connection.execute("SELECT 1 FROM posts WHERE id = ?", (entry_id,)).fetchone() is not None


def put(fragment, seq=None, replace=True):
    """Insert the entry, or replace the one with the same id (keeping its place in the order)."""
    post = parse(fragment)
    connection = connect()
    try:
        existing = connection.execute("SELECT seq FROM posts WHERE id = ?", (post["id"],)).fetchone()
        if existing and not replace:
            raise ValueError(f"id {post['id']!r} already exists in the devlog")
        if seq is None:
            seq = existing[0] if existing else connection.execute("SELECT COALESCE(MAX(seq), 0) + 1 FROM posts").fetchone()[0]
        connection.execute("DELETE FROM posts WHERE id = ?", (post["id"],))
        connection.execute("INSERT INTO posts VALUES (?,?,?,?,?,?,?,?,?,?,?)",
                           (post["id"], seq, post["date"], post["title"], post["summary"], post["html"], post["text"],
                            post["verified"], post["inferred"], post["unknown"], datetime.now().strftime("%Y-%m-%d %H:%M:%S")))
        connection.executemany("INSERT INTO tags VALUES (?, ?)", [(post["id"], tag) for tag in post["tags"]])
        connection.execute("INSERT OR REPLACE INTO meta VALUES ('updated', ?)", (datetime.now().strftime("%Y-%m-%d %H:%M:%S"),))
        connection.commit()  # no VACUUM: it rewrites every page, and git then stores each version whole
    finally:
        connection.close()
    return post


def get(entry_id):
    with connect() as connection:
        row = connection.execute("SELECT id, date, title, html FROM posts WHERE id = ?", (entry_id,)).fetchone()
    if not row:
        return None
    return (f'<article class="entry" id="{row[0]}">\n<div class="entry-meta"><time datetime="{row[1]}">{row[1]}</time></div>\n'
            f"<h2>{html_module.escape(row[2], quote=False)}</h2>\n{row[3]}\n</article>")


def count():
    with connect() as connection:
        return connection.execute("SELECT COUNT(*) FROM posts").fetchone()[0]


def migrate(page):
    articles = re.findall(r'<article class="entry" id="[a-z0-9-]+">.*?</article>', page.read_text(encoding="utf-8"), re.S)
    if DB.exists():
        DB.unlink()
    total = len(articles)
    for position, article in enumerate(articles):  # the page lists the newest first
        put(article, seq=total - position)
    return total


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    if command == "count":
        print(count())
    elif command == "list":
        needle = (sys.argv[2] if len(sys.argv) > 2 else "").lower()
        with connect() as connection:
            for row in connection.execute("SELECT id, date, title FROM posts ORDER BY seq DESC"):
                if needle in (row[0] + " " + row[2]).lower():
                    print(f"{row[1]}  {row[0]}  {row[2]}")
    elif command == "get" and len(sys.argv) == 3:
        fragment = get(sys.argv[2])
        sys.exit(f"no entry {sys.argv[2]!r}") if fragment is None else sys.stdout.buffer.write(fragment.encode("utf-8") + b"\n")
    elif command == "put" and len(sys.argv) == 3:
        post = put(Path(sys.argv[2]).read_text(encoding="utf-8"))
        print(f"stored #{post['id']}: {post['title']} ({', '.join(post['tags'])})")
    elif command == "migrate" and len(sys.argv) == 3:
        print(f"migrated {migrate(Path(sys.argv[2]))} entries into {DB.relative_to(ROOT).as_posix()}")
    else:
        sys.exit(__doc__)
