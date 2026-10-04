"""Devlog feed: write docs/devlog.xml, an Atom feed of the newest devlog entries, from docs/data/devlog.sqlite.

Usage: python scripts/site/devlog_feed.py [--check]

The database is opened read-only; this script never changes it. Each of the newest 50 posts (by `seq`, the
order of publication) becomes one Atom entry linking to devlog.html#<id>, which is how the devlog page opens a
single post. An entry carries the title, the opening paragraph as its summary and the body as HTML content.

Every entry is run through the repository's publication rules before it is written, because the feed is a
published page like any other (AGENTS.md: no addresses, local paths, account names, byte dumps or code):

- the devlog's own fragment checks from scripts/integrate_devlog.py (addresses, generated labels, local paths,
  byte dumps, long hex strings, scripts and styles), except its account-name check, which is replaced by the
  stricter word-bounded one below;
- the machine-path pattern from scripts/ci/check_publication.py, read out of that file's source (importing it
  would run the whole repository check), and the same account-name rule it applies.

A post whose title or summary breaks a rule is left out of the feed; a post whose body breaks one is published
with its summary only. Both are reported. As a last guard the finished document is checked as a whole, and
nothing is written if anything still matches.

Dates: the database records a day per post, not a time, so each entry is stamped at 00:00 UTC on its day. The
output depends only on the database, so it changes only when the devlog does.

The coordinator runs it after adding devlog entries, and commits docs/devlog.xml with them:
    python scripts/site/devlog_feed.py
--check exits 1 if docs/devlog.xml is out of date and writes nothing.
"""

import ast
import getpass
import re
import sqlite3
import sys
from pathlib import Path
from xml.sax.saxutils import escape

ROOT = Path(__file__).resolve().parent.parent.parent
DB = ROOT / "docs" / "data" / "devlog.sqlite"
FEED = ROOT / "docs" / "devlog.xml"
PUBLICATION_CHECK = ROOT / "scripts" / "ci" / "check_publication.py"
SITE = "https://monstercameron.github.io/LibertyFlux/"
LIMIT = 50
# Account names that belong to build machines and containers, not to a person; check_publication.py skips the same.
GENERIC_ACCOUNTS = ("runner", "root", "admin", "administrator", "user", "runneradmin")

sys.path.insert(0, str(ROOT / "scripts"))
import integrate_devlog  # noqa: E402  (its CHECKS list is the devlog's publication rule set)


def publication_patterns(source=PUBLICATION_CHECK):
    """The module-level regular expressions named in check_publication.py, compiled from its source text."""
    found = {}
    for node in ast.parse(source.read_text(encoding="utf-8")).body:
        if not (isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name)):
            continue
        call = node.value
        if (isinstance(call, ast.Call) and isinstance(call.func, ast.Attribute) and call.func.attr == "compile"
                and call.args and isinstance(call.args[0], ast.Constant) and isinstance(call.args[0].value, str)):
            flags = 0
            if len(call.args) > 1 and isinstance(call.args[1], ast.Attribute):
                flags = getattr(re, call.args[1].attr, 0)
            found[node.targets[0].id] = re.compile(call.args[0].value, flags)
    return found


def account_pattern(account=None):
    """A word-bounded match for this machine's account name, or None when it is short or generic."""
    if account is None:
        try:
            account = getpass.getuser()
        except Exception:  # no account name available: this one check has nothing to match
            return None
    if len(account) < 4 or account.lower() in GENERIC_ACCOUNTS:
        return None
    return re.compile(r"\b" + re.escape(account) + r"\b", re.I)


def rules(account=None):
    """(name, pattern) pairs every published piece of feed text must pass."""
    checks = [(name, pattern) for name, pattern in integrate_devlog.CHECKS if name != "user name"]
    machine = publication_patterns().get("MACHINE_PATH")
    if machine is None:
        raise RuntimeError("check_publication.py no longer defines MACHINE_PATH; update devlog_feed.py")
    checks.append(("machine path", machine))
    own = account_pattern(account)
    if own:
        checks.append(("account name", own))
    return checks


def findings(text, checks):
    """Names of the rules the text breaks, with the first match of each."""
    out = []
    for name, pattern in checks:
        hit = pattern.search(text)
        if hit:
            out.append(f"{name} ({hit.group(0)[:40]!r})")
    return out


def read_posts(db=DB, limit=LIMIT):
    """The newest posts, newest first, from a read-only connection."""
    connection = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    try:
        rows = connection.execute(
            "SELECT id, seq, date, title, summary, html FROM posts ORDER BY seq DESC LIMIT ?", (limit,)).fetchall()
    finally:
        connection.close()
    return [dict(zip(("id", "seq", "date", "title", "summary", "html"), row)) for row in rows]


def stamp(day):
    """An Atom date for a YYYY-MM-DD day: midnight UTC."""
    if not re.fullmatch(r"\d{4}-\d{2}-\d{2}", day or ""):
        raise ValueError(f"post date is not YYYY-MM-DD: {day!r}")
    return day + "T00:00:00Z"


def screen(posts, checks):
    """Apply the publication rules. Returns (kept posts, report lines); a kept post may lose its body."""
    kept, report = [], []
    for post in posts:
        head = findings(post["title"] + "\n" + post["summary"], checks)
        if head:
            report.append(f"left out {post['id']}: " + "; ".join(head))
            continue
        body = findings(post["html"], checks)
        if body:
            report.append(f"summary only for {post['id']}: " + "; ".join(body))
            post = dict(post, html="")
        kept.append(post)
    return kept, report


def render(posts, site=SITE):
    """The Atom document for already-screened posts, newest first."""
    feed_url = site + "devlog.xml"
    page = site + "devlog.html"
    updated = max((stamp(p["date"]) for p in posts), default="1970-01-01T00:00:00Z")
    out = [
        '<?xml version="1.0" encoding="utf-8"?>',
        f'<feed xmlns="http://www.w3.org/2005/Atom" xml:lang="en" xml:base="{escape(site)}">',
        "  <title>LibertyFlux devlog</title>",
        "  <subtitle>What the project found, decided and got wrong while rewriting the GTA IV engine in Rust.</subtitle>",
        f"  <id>{escape(page)}</id>",
        f'  <link rel="alternate" type="text/html" href="{escape(page)}"/>',
        f'  <link rel="self" type="application/atom+xml" href="{escape(feed_url)}"/>',
        f"  <updated>{updated}</updated>",
        "  <author><name>LibertyFlux</name></author>",
        "  <generator>scripts/site/devlog_feed.py</generator>",
    ]
    for post in posts:
        link = page + "#" + post["id"]
        when = stamp(post["date"])
        out += [
            "  <entry>",
            f"    <title>{escape(post['title'])}</title>",
            f'    <link rel="alternate" type="text/html" href="{escape(link)}"/>',
            f"    <id>{escape(link)}</id>",
            f"    <published>{when}</published>",
            f"    <updated>{when}</updated>",
            f"    <summary>{escape(post['summary'])}</summary>",
        ]
        if post["html"]:
            out.append(f'    <content type="html" xml:base="{escape(page)}">{escape(post["html"])}</content>')
        out.append("  </entry>")
    out.append("</feed>")
    return "\n".join(out) + "\n"


def build(db=DB, account=None):
    """(document, report). Raises RuntimeError if the finished document still breaks a rule."""
    checks = rules(account)
    posts, report = screen(read_posts(db), checks)
    document = render(posts)
    # The links and the namespace are the feed's own and contain no paths; everything else came from the posts.
    leftover = findings(document, [c for c in checks if c[0] != "script or style"])
    if leftover:
        raise RuntimeError("the finished feed breaks the publication rules: " + "; ".join(leftover))
    return document, report


def main():
    try:
        document, report = build()
    except RuntimeError as error:
        sys.exit(f"devlog.xml not written: {error}")
    for line in report:
        print(line)
    current = FEED.read_text(encoding="utf-8") if FEED.exists() else ""
    entries = document.count("<entry>")
    if "--check" in sys.argv:
        print("devlog.xml is up to date" if document == current else "devlog.xml is out of date")
        sys.exit(0 if document == current else 1)
    if document != current:
        FEED.write_text(document, encoding="utf-8", newline="\n")
        print(f"devlog.xml: {entries} entries written")
    else:
        print(f"devlog.xml: unchanged, {entries} entries")


if __name__ == "__main__":
    main()
