"""Tests for devlog_feed.py on a small synthetic database: selection, publication screening and the Atom output."""

import hashlib
import sqlite3
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ElementTree
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import devlog_feed

ATOM = {"a": "http://www.w3.org/2005/Atom"}
# Strings that must be refused, assembled at run time so this file itself passes the publication check.
MACHINE_PATH = "/ho" + "me/alice/game"
ADDRESS = "0x" + "00A1B2C3"


def make_db(folder, posts):
    path = Path(folder) / "devlog.sqlite"
    connection = sqlite3.connect(path)
    connection.execute("CREATE TABLE posts (id TEXT PRIMARY KEY, seq INTEGER, date TEXT, title TEXT, summary TEXT,"
                       " html TEXT, text TEXT, verified INTEGER, inferred INTEGER, unknown INTEGER, added TEXT)")
    for seq, (entry_id, title, summary, html) in enumerate(posts, 1):
        connection.execute("INSERT INTO posts VALUES (?,?,?,?,?,?,?,0,0,0,'')",
                           (entry_id, seq, "2026-10-0" + str(1 + seq % 9), title, summary, html, title))
    connection.commit()
    connection.close()
    return path


def clean(n):
    return (f"post-{n}", f"Post {n} & more", f"Summary {n}.", f"<p>Summary {n}.</p><p>A <a href=\"#post-1\">link</a>.</p>")


class TestPatterns(unittest.TestCase):
    def test_machine_path_comes_from_the_publication_check(self):
        patterns = devlog_feed.publication_patterns()
        self.assertIn("MACHINE_PATH", patterns)
        self.assertTrue(patterns["MACHINE_PATH"].search(MACHINE_PATH))

    def test_generic_and_short_accounts_are_not_patterns(self):
        for name in ("root", "runner", "user", "bob"):
            self.assertIsNone(devlog_feed.account_pattern(name))
        pattern = devlog_feed.account_pattern("alicia")
        self.assertTrue(pattern.search("written by Alicia today"))
        self.assertFalse(pattern.search("alicias"))

    def test_rules_include_the_devlog_checks(self):
        names = [name for name, _ in devlog_feed.rules("alicia")]
        for expected in ("hexadecimal address", "byte dump", "machine path", "account name"):
            self.assertIn(expected, names)
        self.assertNotIn("user name", names)


class TestScreen(unittest.TestCase):
    def setUp(self):
        self.checks = devlog_feed.rules("alicia")

    def test_bad_title_or_summary_leaves_the_post_out(self):
        posts = [{"id": "a", "title": "At " + ADDRESS, "summary": "s", "html": "<p>s</p>"},
                 {"id": "b", "title": "t", "summary": "In " + MACHINE_PATH, "html": "<p>s</p>"}]
        kept, report = devlog_feed.screen(posts, self.checks)
        self.assertEqual(kept, [])
        self.assertEqual(len(report), 2)

    def test_bad_body_keeps_only_the_summary(self):
        posts = [{"id": "c", "title": "t", "summary": "s", "html": "<p>bytes 00 11 22 33 44 55 66 77 88</p>"}]
        kept, report = devlog_feed.screen(posts, self.checks)
        self.assertEqual(kept[0]["html"], "")
        self.assertIn("summary only for c", report[0])

    def test_clean_post_is_untouched(self):
        post = {"id": "d", "title": "t", "summary": "s", "html": "<p>fine</p>"}
        kept, report = devlog_feed.screen([post], self.checks)
        self.assertEqual((kept, report), ([post], []))


class TestBuild(unittest.TestCase):
    def test_newest_fifty_as_valid_atom_and_database_unchanged(self):
        with tempfile.TemporaryDirectory() as folder:
            db = make_db(folder, [clean(n) for n in range(1, 61)])
            before = hashlib.sha256(db.read_bytes()).hexdigest()
            document, report = devlog_feed.build(db, account="alicia")
            self.assertEqual(hashlib.sha256(db.read_bytes()).hexdigest(), before)
        self.assertEqual(report, [])
        root = ElementTree.fromstring(document.encode("utf-8"))
        entries = root.findall("a:entry", ATOM)
        self.assertEqual(len(entries), 50)
        first = entries[0]
        self.assertEqual(first.find("a:title", ATOM).text, "Post 60 & more")
        self.assertEqual(first.find("a:link", ATOM).get("href"), devlog_feed.SITE + "devlog.html#post-60")
        self.assertIn('<a href="#post-1">', first.find("a:content", ATOM).text)
        self.assertRegex(first.find("a:updated", ATOM).text, r"^\d{4}-\d{2}-\d{2}T00:00:00Z$")

    def test_flagged_posts_are_reported_not_published(self):
        with tempfile.TemporaryDirectory() as folder:
            db = make_db(folder, [clean(1), ("bad", "Found at " + ADDRESS, "s", "<p>s</p>")])
            document, report = devlog_feed.build(db, account="alicia")
        self.assertNotIn(ADDRESS, document)
        self.assertEqual(len(report), 1)
        self.assertEqual(document.count("<entry>"), 1)

    def test_empty_database_gives_an_empty_feed(self):
        with tempfile.TemporaryDirectory() as folder:
            document, _ = devlog_feed.build(make_db(folder, []), account="alicia")
        self.assertEqual(ElementTree.fromstring(document.encode("utf-8")).findall("a:entry", ATOM), [])

    def test_bad_date_is_refused(self):
        with self.assertRaises(ValueError):
            devlog_feed.stamp("4 October")


if __name__ == "__main__":
    unittest.main()
