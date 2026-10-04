"""Tests for social.py: the PNG encoder, the pixel font and the drawing from progress data."""

import struct
import sys
import unittest
import xml.etree.ElementTree as ElementTree
import zlib
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import social


def chunks(png):
    """(kind, data) for every chunk, checking each CRC on the way."""
    out, at = [], 8
    while at < len(png):
        length = struct.unpack(">I", png[at:at + 4])[0]
        kind, data = png[at + 4:at + 8], png[at + 8:at + 8 + length]
        crc = struct.unpack(">I", png[at + 8 + length:at + 12 + length])[0]
        assert crc == zlib.crc32(kind + data) & 0xFFFFFFFF, kind
        out.append((kind, data))
        at += 12 + length
    return out


PROGRESS = {"updated": "2026-10-04", "functions": {"game": 1000}, "stages": {"verified": 250},
            "map": [{"stage": "unmeasured", "count": 0}, {"stage": "named", "count": 10, "verified": 5},
                    {"stage": "verified", "count": 4, "verified": 4}] * 40}


class TestFont(unittest.TestCase):
    def test_every_glyph_is_five_by_seven(self):
        for char, glyph in social.FONT.items():
            self.assertEqual(len(glyph), 7, char)
            self.assertTrue(all(len(row) == 5 and set(row) <= {"#", "."} for row in glyph), char)

    def test_runs_merge_without_a_gap(self):
        merged = social.lettering("T", 0, 0, 2, (1, 2, 3))
        squares = social.lettering("T", 0, 0, 2, (1, 2, 3), gap=1)
        self.assertEqual(merged[0][:4], (0, 0, 10, 2))  # the top bar of a T is one rectangle
        self.assertEqual(len(squares), 5 + 6)

    def test_captions_fit_the_frame(self):
        rects, _ = social.drawing(PROGRESS)
        for x, y, w, h, _ in rects:
            self.assertLessEqual(x + w, social.WIDTH)
            self.assertLessEqual(y + h, social.HEIGHT)


class TestOutput(unittest.TestCase):
    def test_png_is_valid_and_full_size(self):
        png = social.to_png(*social.drawing(PROGRESS))
        self.assertEqual(png[:8], b"\x89PNG\r\n\x1a\n")
        parts = chunks(png)
        self.assertEqual([kind for kind, _ in parts], [b"IHDR", b"IDAT", b"IEND"])
        width, height, depth, colour = struct.unpack(">IIBB", parts[0][1][:10])
        self.assertEqual((width, height, depth, colour), (1200, 630, 8, 2))
        raw = zlib.decompress(parts[1][1])
        self.assertEqual(len(raw), height * (1 + width * 3))
        self.assertEqual(raw[1:4], bytes(social.ASPHALT))  # the corner is the masthead colour

    def test_svg_parses(self):
        root = ElementTree.fromstring(social.to_svg(*social.drawing(PROGRESS), "t"))
        self.assertEqual((root.get("width"), root.get("height")), ("1200", "630"))

    def test_nothing_measured_still_draws(self):
        rects, hatched = social.drawing({})
        self.assertEqual(hatched, [])
        self.assertGreater(len(rects), 1)


if __name__ == "__main__":
    unittest.main()
