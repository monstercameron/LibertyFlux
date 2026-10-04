"""Checks over the site's pages in docs/: navigation, link-preview tags, no outside resources, the sitemap,
links into the repository that resolve, and nothing that looks like an address or a machine path.

The pages are plain HTML with no build step, so these run on the files as they are served.
"""

import re
import sys
import unittest
from html.parser import HTMLParser
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import devlog_feed
import quality

ROOT = Path(__file__).resolve().parent.parent.parent.parent
DOCS = ROOT / "docs"
SITE = "https://monstercameron.github.io/LibertyFlux/"
REPO = "https://github.com/monstercameron/LibertyFlux"
# The site's pages in navigation order; the GitHub link closes the list on every page.
NAV = ["index.html", "engine.html", "quality.html", "tools.html", "changelog.html", "devlog.html"]
NOT_FOUND = "404.html"


class Page(HTMLParser):
    """The parts of a page these checks need."""

    def __init__(self):
        super().__init__()
        self.nav, self.current, self.meta, self.links, self.resources, self.anchors = [], [], {}, [], [], []
        self.in_nav = False

    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if tag == "nav" and a.get("aria-label") == "Site":
            self.in_nav = True
        if tag == "a" and a.get("href"):
            self.anchors.append(a["href"])
            if self.in_nav:
                self.nav.append(a["href"])
                if a.get("aria-current") == "page":
                    self.current.append(a["href"])
        if tag == "meta":
            key = a.get("property") or a.get("name")
            if key:
                self.meta[key] = a.get("content", "")
        if tag == "link":
            self.links.append(a)
            if a.get("rel") in ("stylesheet", "preload", "icon"):
                self.resources.append(a.get("href", ""))
        if tag in ("script", "img", "iframe", "source", "video", "audio") and a.get("src"):
            self.resources.append(a["src"])

    def handle_endtag(self, tag):
        if tag == "nav":
            self.in_nav = False


def parse(name):
    page = Page()
    page.feed((DOCS / name).read_text(encoding="utf-8"))
    return page


def pages():
    return sorted(p.name for p in DOCS.glob("*.html"))


class TestNavigation(unittest.TestCase):
    def test_every_page_is_listed(self):
        self.assertEqual(sorted(NAV + [NOT_FOUND]), pages())

    def test_every_page_has_the_full_navigation_in_order(self):
        for name in NAV:
            page = parse(name)
            self.assertEqual(page.nav, NAV + [REPO], name)
            self.assertEqual(page.current, [name], f"{name}: aria-current must mark the page itself")

    def test_not_found_page_links_from_the_site_root(self):
        page = parse(NOT_FOUND)
        self.assertEqual(page.nav, ["/LibertyFlux/" + n for n in NAV] + [REPO])
        for href in page.resources:
            self.assertTrue(href.startswith("/LibertyFlux/"), href)


class TestPreviews(unittest.TestCase):
    def test_link_preview_tags(self):
        for name in pages():
            meta = parse(name).meta
            for key in ("og:title", "og:description", "og:image", "twitter:card", "twitter:image", "description"):
                self.assertTrue(meta.get(key), f"{name}: {key}")
            self.assertEqual(meta["og:image"], SITE + "img/social.png", name)
            if name != NOT_FOUND:
                self.assertEqual(meta.get("og:url"), SITE + name, name)

    def test_canonical_links(self):
        for name in NAV:
            canonical = [l.get("href") for l in parse(name).links if l.get("rel") == "canonical"]
            self.assertEqual(canonical, [SITE + name], name)
        self.assertEqual(parse(NOT_FOUND).meta.get("robots"), "noindex")


class TestNoOutsideResources(unittest.TestCase):
    def test_scripts_styles_fonts_and_images_come_from_the_site(self):
        for name in pages():
            for href in parse(name).resources:
                self.assertFalse(re.match(r"^(?:[a-z]+:)?//", href), f"{name} loads {href} from another host")


class TestSitemap(unittest.TestCase):
    def test_sitemap_lists_every_page_but_the_not_found_page(self):
        text = (DOCS / "sitemap.xml").read_text(encoding="utf-8")
        self.assertEqual(re.findall(r"<loc>([^<]+)</loc>", text), [SITE + n for n in NAV])

    def test_robots_points_at_the_sitemap(self):
        self.assertIn("Sitemap: " + SITE + "sitemap.xml", (DOCS / "robots.txt").read_text(encoding="utf-8"))


class TestRepositoryLinks(unittest.TestCase):
    def test_links_into_the_repository_resolve(self):
        pattern = re.compile(re.escape(REPO) + r"/(?:blob|tree)/main/([^\"#?]+)")
        for name in pages():
            for href in parse(name).anchors:
                match = pattern.match(href)
                if match:
                    self.assertTrue((ROOT / match.group(1)).exists(), f"{name} links to missing {match.group(1)}")

    def test_scripts_named_in_commands_exist(self):
        text = (DOCS / "tools.html").read_text(encoding="utf-8")
        named = set(re.findall(r"\bpython3? ((?:scripts)/[\w/]+\.py)", text))
        self.assertGreater(len(named), 10)
        for path in sorted(named):
            self.assertTrue((ROOT / path).exists(), path)

    def test_local_links_resolve(self):
        for name in pages():
            for href in parse(name).anchors:
                if re.match(r"^(?:[a-z]+:|#|/LibertyFlux/)", href):
                    continue
                target = href.split("#")[0]
                if target:
                    self.assertTrue((DOCS / target).exists(), f"{name} links to missing {target}")


class TestQualityPage(unittest.TestCase):
    def test_a_card_for_every_problem_class(self):
        text = (DOCS / "quality.html").read_text(encoding="utf-8")
        cards = re.findall(r'class="qcard" data-class="([a-z-]+)"', text)
        self.assertEqual(cards, [key for key, _, _ in quality.CLASSES])


class TestPublication(unittest.TestCase):
    def test_no_addresses_or_machine_paths_on_any_page(self):
        machine = devlog_feed.publication_patterns()["MACHINE_PATH"]
        address = re.compile(r"\b0x[0-9A-Fa-f]{5,}\b")
        files = [DOCS / n for n in pages()] + sorted(DOCS.glob("*.css")) + sorted(DOCS.glob("*.js")) + [
            DOCS / "sitemap.xml", DOCS / "robots.txt"]
        for path in files:
            text = path.read_text(encoding="utf-8")
            self.assertIsNone(address.search(text), f"{path.name}: address-like number")
            self.assertIsNone(machine.search(text), f"{path.name}: machine path")


if __name__ == "__main__":
    unittest.main()
