"""Social preview image: write docs/img/social.svg and docs/img/social.png (1200 x 630) from docs/data/progress.json.

Usage: python scripts/site/social.py

Link previews (Open Graph, Twitter cards) show one picture per page. This one is drawn in the site's own
style: the dark masthead colour, the project name, one line saying what it is, and the overview page's code
map as it stands in progress.json, with the date of that file and the verified count under it. The map is the
real data of the day it is drawn, never a made-up pattern, and the caption says which day that was.

Most platforms do not accept SVG for previews, so the same drawing is also written as a PNG. The PNG is
encoded here with zlib alone (no imaging library): every shape is an axis-aligned rectangle, and the lettering
is a 5 x 7 pixel font drawn as squares, so the two files show the same picture without needing a font.

The image is a snapshot. Re-run the script when the map has changed enough to matter (once a day is plenty);
running it on every tick would commit a new binary file every five minutes.
"""

import json
import struct
import zlib
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
PROGRESS = ROOT / "docs" / "data" / "progress.json"
OUT_SVG = ROOT / "docs" / "img" / "social.svg"
OUT_PNG = ROOT / "docs" / "img" / "social.png"

WIDTH, HEIGHT = 1200, 630
MARGIN = 60
TOP = 104  # the drawing is about 420 pixels tall; this centres it in the 630-pixel frame
# The masthead's colours (site.css, light theme): asphalt behind, sodium for the one accent, and the stage ramp
# the code map uses on asphalt, lighter meaning further along.
ASPHALT = (0x2A, 0x30, 0x38)
ON_ASPHALT = (0xEC, 0xEF, 0xF2)
ON_ASPHALT_SOFT = (0xB7, 0xC0, 0xCA)
SODIUM = (0xE0, 0xA0, 0x21)
STAGES = {
    "identified": (0x4A, 0x73, 0x91),
    "named": (0x6D, 0x9F, 0xC6),
    "rewritten": (0x9C, 0xC9, 0xEA),
    "verified": (0xF0, 0xF8, 0xFF),
}
VERIFIED_RING = (0x8A, 0xA6, 0xBC)  # site.css draws rgba(18, 74, 115, 0.5) over the verified colour
# The "no game code" hatch: white at 20% and 8% over asphalt, in 3-pixel diagonal stripes.
HATCH_LIGHT = tuple(round(c * 0.8 + 255 * 0.2) for c in ASPHALT)
HATCH_DARK = tuple(round(c * 0.92 + 255 * 0.08) for c in ASPHALT)

MAP_COLUMNS = 60
CELL, GAP = 16, 2

# A 5 x 7 pixel font: each glyph is seven rows of five columns, "#" for a lit pixel.
FONT = {
    "A": [".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"],
    "B": ["####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."],
    "C": [".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."],
    "D": ["####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####."],
    "E": ["#####", "#....", "#....", "####.", "#....", "#....", "#####"],
    "F": ["#####", "#....", "#....", "####.", "#....", "#....", "#...."],
    "G": [".###.", "#...#", "#....", "#.###", "#...#", "#...#", ".####"],
    "H": ["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"],
    "I": ["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "#####"],
    "J": ["..###", "...#.", "...#.", "...#.", "...#.", "#..#.", ".##.."],
    "K": ["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"],
    "L": ["#....", "#....", "#....", "#....", "#....", "#....", "#####"],
    "M": ["#...#", "##.##", "#.#.#", "#.#.#", "#...#", "#...#", "#...#"],
    "N": ["#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#"],
    "O": [".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."],
    "P": ["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."],
    "Q": [".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#"],
    "R": ["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"],
    "S": [".####", "#....", "#....", ".###.", "....#", "....#", "####."],
    "T": ["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."],
    "U": ["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."],
    "V": ["#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#.."],
    "W": ["#...#", "#...#", "#...#", "#.#.#", "#.#.#", "#.#.#", ".#.#."],
    "X": ["#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#"],
    "Y": ["#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#.."],
    "Z": ["#####", "....#", "...#.", "..#..", ".#...", "#....", "#####"],
    "0": [".###.", "#...#", "#..##", "#.#.#", "##..#", "#...#", ".###."],
    "1": ["..#..", ".##..", "..#..", "..#..", "..#..", "..#..", ".###."],
    "2": [".###.", "#...#", "....#", "...#.", "..#..", ".#...", "#####"],
    "3": ["####.", "....#", "....#", ".###.", "....#", "....#", "####."],
    "4": ["...#.", "..##.", ".#.#.", "#..#.", "#####", "...#.", "...#."],
    "5": ["#####", "#....", "####.", "....#", "....#", "#...#", ".###."],
    "6": [".###.", "#....", "#....", "####.", "#...#", "#...#", ".###."],
    "7": ["#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#..."],
    "8": [".###.", "#...#", "#...#", ".###.", "#...#", "#...#", ".###."],
    "9": [".###.", "#...#", "#...#", ".####", "....#", "....#", ".###."],
    " ": [".....", ".....", ".....", ".....", ".....", ".....", "....."],
    ",": [".....", ".....", ".....", ".....", ".....", "..#..", ".#..."],
    ".": [".....", ".....", ".....", ".....", ".....", ".....", "..#.."],
    ":": [".....", "..#..", ".....", ".....", ".....", "..#..", "....."],
    "'": ["..#..", "..#..", ".#...", ".....", ".....", ".....", "....."],
    "-": [".....", ".....", ".....", ".###.", ".....", ".....", "....."],
}
MONTHS = ("JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC")


def text_width(text, pitch):
    """Width in pixels of text drawn with the pixel font at `pitch` pixels per font pixel."""
    return len(text) * 6 * pitch - pitch


def lettering(text, x, y, pitch, colour, gap=0):
    """Rectangles for text in the pixel font. With no gap, runs of lit pixels in a row merge into one rectangle."""
    rects = []
    for index, char in enumerate(text.upper()):
        glyph = FONT[char]
        left = x + index * 6 * pitch
        for row, line in enumerate(glyph):
            column = 0
            while column < 5:
                if line[column] != "#":
                    column += 1
                    continue
                run = 1 if gap else len(line[column:]) - len(line[column:].lstrip("#"))
                rects.append((left + column * pitch, y + row * pitch, run * pitch - gap, pitch - gap, colour))
                column += run
    return rects


def map_cells(slices, x, y):
    """Rectangles for the code map: one square per slice, shaded like the overview page's map."""
    rects, hatched = [], []
    for index, entry in enumerate(slices):
        left = x + (index % MAP_COLUMNS) * (CELL + GAP)
        top = y + (index // MAP_COLUMNS) * (CELL + GAP)
        stage = entry.get("stage")
        if stage not in STAGES:
            hatched.append((left, top, CELL, CELL))
            continue
        if stage == "verified":
            rects.append((left, top, CELL, CELL, VERIFIED_RING))
            rects.append((left + 2, top + 2, CELL - 4, CELL - 4, STAGES["verified"]))
            continue
        rects.append((left, top, CELL, CELL, STAGES[stage]))
        count, verified = entry.get("count") or 0, entry.get("verified") or 0
        if count and verified > 0:
            band = max(round(CELL * 0.12), round(CELL * verified / count))
            rects.append((left, top + CELL - band, CELL, band, STAGES["verified"]))
    return rects, hatched


def drawing(progress):
    """The picture as (rectangles, hatched squares); each rectangle is (x, y, width, height, colour)."""
    slices = progress.get("map") or []
    stages = progress.get("stages") or {}
    game = (progress.get("functions") or {}).get("game") or 0
    rects = [(0, 0, WIDTH, HEIGHT, ASPHALT)]
    rects += lettering("LibertyFlux", MARGIN, TOP, 14, ON_ASPHALT, gap=2)
    rects.append((MARGIN, TOP + 120, 112, 6, SODIUM))
    rects += lettering("GTA IV's engine, being rewritten in Rust", MARGIN, TOP + 158, 4, ON_ASPHALT_SOFT)
    rows = -(-len(slices) // MAP_COLUMNS) if slices else 0
    map_top = TOP + 236
    cells, hatched = map_cells(slices, MARGIN, map_top)
    rects += cells
    caption_top = map_top + rows * (CELL + GAP) + 22
    try:
        day = date.fromisoformat(progress.get("updated", ""))
        when = f"{day.day} {MONTHS[day.month - 1]} {day.year}"
    except ValueError:
        when = ""
    if game:
        caption = f"Code map{', ' + when if when else ''}: {stages.get('verified', 0):,} of {game:,} functions verified"
    else:
        caption = "Nothing measured yet"
    rects += lettering(caption, MARGIN, caption_top, 3, ON_ASPHALT_SOFT)
    return rects, hatched


def hatch_colour(x, y):
    return HATCH_LIGHT if (x + y) % 6 < 3 else HATCH_DARK


def to_png(rects, hatched):
    """Encode the drawing as an 8-bit RGB PNG with zlib."""
    pixels = bytearray(WIDTH * HEIGHT * 3)
    for x, y, w, h, colour in rects:
        row = bytes(colour) * w
        for line in range(max(0, y), min(HEIGHT, y + h)):
            start = (line * WIDTH + x) * 3
            pixels[start:start + w * 3] = row
    for x, y, w, h in hatched:
        for line in range(y, y + h):
            for column in range(x, x + w):
                start = (line * WIDTH + column) * 3
                pixels[start:start + 3] = bytes(hatch_colour(column, line))
    raw = b"".join(b"\x00" + bytes(pixels[line * WIDTH * 3:(line + 1) * WIDTH * 3]) for line in range(HEIGHT))

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    header = struct.pack(">IIBBBBB", WIDTH, HEIGHT, 8, 2, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b"")


def to_svg(rects, hatched, title):
    """The same drawing as SVG; the hatch is a pattern of the same diagonal stripes."""
    def hexed(colour):
        return "#%02x%02x%02x" % colour

    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}"'
           f' role="img" aria-label="{title}">',
           f"<title>{title}</title>",
           '<defs><pattern id="hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">'
           f'<rect width="6" height="6" fill="{hexed(HATCH_DARK)}"/><rect width="3" height="6" fill="{hexed(HATCH_LIGHT)}"/>'
           "</pattern></defs>"]
    background = rects[0]
    out.append(f'<rect width="{background[2]}" height="{background[3]}" fill="{hexed(background[4])}"/>')
    # Consecutive rectangles of one colour share a group; the order is kept, because later shapes paint over
    # earlier ones (a verified band over its square).
    runs = []
    for x, y, w, h, colour in rects[1:]:
        if not runs or runs[-1][0] != colour:
            runs.append((colour, []))
        runs[-1][1].append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}"/>')
    for colour, items in runs:
        out.append(f'<g fill="{hexed(colour)}">' + "".join(items) + "</g>")
    if hatched:
        out.append('<g fill="url(#hatch)">' + "".join(
            f'<rect x="{x}" y="{y}" width="{w}" height="{h}"/>' for x, y, w, h in hatched) + "</g>")
    out.append("</svg>")
    return "\n".join(out) + "\n"


def main():
    progress = json.loads(PROGRESS.read_text(encoding="utf-8"))
    rects, hatched = drawing(progress)
    title = "LibertyFlux: GTA IV's engine, being rewritten in Rust".replace("'", "&#39;")
    OUT_SVG.write_text(to_svg(rects, hatched, title), encoding="utf-8", newline="\n")
    OUT_PNG.write_bytes(to_png(rects, hatched))
    print(f"social.svg: {OUT_SVG.stat().st_size:,} bytes; social.png: {OUT_PNG.stat().st_size:,} bytes")


if __name__ == "__main__":
    main()
