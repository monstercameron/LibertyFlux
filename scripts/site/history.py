"""Progress over time: write docs/data/history.json, the series behind the overview page's progress chart.

Usage: python scripts/site/history.py [--check]

Every committed version of docs/data/progress.json is one recorded state of the project. This script walks
them in git history (`git log -- docs/data/progress.json`, then `git show <hash>:docs/data/progress.json`)
and keeps, for each, the commit time and the four counts the chart draws:

    {"t": "2026-10-04T15:46:51Z", "game": 37413, "named": 14690, "rewritten": 7884, "verified": 7655}

`t` is the commit time in UTC. `game` is functions.game (the denominator of every percentage); `named`,
`rewritten` and `verified` are the cumulative stage counts. A version without functions.game is skipped:
nothing had been counted yet, so there is nothing to plot.

The clone may be shallow and only reach back a few dozen commits, so the points already in history.json are
read first and merged: a point is never dropped because its commit is no longer reachable. Points are keyed
by time; a point for a time already present is replaced by the value from git, which is the source.

When the progress.json on disk differs from the committed one (the tick has just written it and is about to
commit it), its counts are added as one more point, stamped with the file's modification time and marked
"tree": true. Such a point is provisional: the next run drops every marked point and takes the committed
version from git instead, so the file never keeps two points for one state.

Runs of identical counts are thinned to their first and last point, so a quiet hour costs two points rather
than twelve, and the chart still shows when the run began and ended.

The output is written only when it changed. --check exits 1 if history.json is out of date and writes nothing.

The coordinator runs it on every tick and commits docs/data/history.json in the same commit as progress.json:
    python scripts/site/history.py
(In scripts/status.py: run it after progress.json is written, and add docs/data/history.json to the paths the
progress commit stages. Run before that commit it records the new state as a provisional point; run after it,
it reads the state from git. Both give the same file once the next tick has run.)
"""

import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
PROGRESS = "docs/data/progress.json"
HISTORY = ROOT / "docs" / "data" / "history.json"
# The counts kept per point, in the order the file lists them.
FIELDS = ("game", "named", "rewritten", "verified")


def git(*args):
    """Run git in the repository and return its standard output as text."""
    return subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True, check=True).stdout


def to_utc(stamp):
    """Normalise an ISO 8601 time with an offset to UTC with a trailing Z, to the second."""
    moment = datetime.fromisoformat(stamp.replace("Z", "+00:00"))
    if moment.tzinfo is None:
        raise ValueError(f"time without a zone: {stamp!r}")
    return moment.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def point(stamp, progress):
    """The chart's point for one version of progress.json, or None when no functions were counted yet."""
    functions = progress.get("functions") or {}
    stages = progress.get("stages") or {}
    game = functions.get("game")
    if not isinstance(game, int) or game <= 0:
        return None
    entry = {"t": to_utc(stamp), "game": game}
    for name in FIELDS[1:]:
        value = stages.get(name)
        entry[name] = value if isinstance(value, int) and value >= 0 else 0
    return entry


def from_git():
    """One point per commit that changed progress.json and still parses, oldest first."""
    points = []
    for line in git("log", "--format=%H %cI", "--", PROGRESS).splitlines():
        commit, _, stamp = line.partition(" ")
        try:
            progress = json.loads(git("show", f"{commit}:{PROGRESS}"))
        except (subprocess.CalledProcessError, json.JSONDecodeError):
            continue  # deleted or unparsable in that commit: not a recorded state
        entry = point(stamp, progress)
        if entry:
            points.append(entry)
    return points[::-1]  # git log lists the newest first


def from_tree():
    """A provisional point for progress.json as it is on disk, or None when it matches the committed version."""
    path = ROOT / PROGRESS
    try:
        current = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    try:
        committed = json.loads(git("show", f"HEAD:{PROGRESS}"))
    except (subprocess.CalledProcessError, json.JSONDecodeError):
        committed = None
    if current == committed:
        return None
    entry = point(datetime.fromtimestamp(path.stat().st_mtime, timezone.utc).isoformat(), current)
    if entry:
        entry["tree"] = True
    return entry


def combine(existing, committed, provisional=None):
    """The file's points: earlier points without the provisional ones, the committed points, and the new
    provisional point if there is one, merged by time and thinned."""
    kept = [p for p in existing if not (isinstance(p, dict) and p.get("tree"))]
    return thin(merge(kept, committed + ([provisional] if provisional else [])))


def merge(existing, fresh):
    """Union of two point lists keyed by time; fresh points win. Returned sorted by time."""
    by_time = {p["t"]: p for p in existing if isinstance(p, dict) and "t" in p}
    by_time.update({p["t"]: p for p in fresh})
    return [by_time[t] for t in sorted(by_time)]


def counts(entry):
    return tuple(entry.get(name) for name in FIELDS)


def thin(points):
    """Keep only the first and last point of every run of identical counts."""
    kept = []
    for index, entry in enumerate(points):
        same_as_before = index > 0 and counts(points[index - 1]) == counts(entry)
        same_as_after = index + 1 < len(points) and counts(points[index + 1]) == counts(entry)
        if not (same_as_before and same_as_after):
            kept.append(entry)
    return kept


def read_existing(path):
    """Points already recorded in history.json, or none when the file is missing or unreadable."""
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return []
    points = data.get("points") if isinstance(data, dict) else None
    return points if isinstance(points, list) else []


def render(points):
    """The file's text: one point per line, so a tick's diff is one or two lines."""
    def line(p):
        entry = {k: p[k] for k in ("t",) + FIELDS}
        if p.get("tree"):
            entry["tree"] = True
        return "    " + json.dumps(entry, separators=(", ", ": "))

    lines = ",\n".join(line(p) for p in points)
    return ('{\n  "source": "git history of docs/data/progress.json; written by scripts/site/history.py",\n'
            '  "fields": {"t": "commit time, UTC", "game": "functions.game", "named": "stages.named",'
            ' "rewritten": "stages.rewritten", "verified": "stages.verified",'
            ' "tree": "present when the state was not committed yet; replaced by its commit on the next run"},\n'
            '  "points": [\n' + lines + ("\n" if lines else "") + "  ]\n}\n")


def main():
    points = combine(read_existing(HISTORY), from_git(), from_tree())
    text = render(points)
    current = HISTORY.read_text(encoding="utf-8") if HISTORY.exists() else ""
    if "--check" in sys.argv:
        print("history.json is up to date" if text == current else "history.json is out of date")
        sys.exit(0 if text == current else 1)
    if text != current:
        HISTORY.write_text(text, encoding="utf-8", newline="\n")
        print(f"history.json: {len(points)} points written")
    else:
        print(f"history.json: unchanged, {len(points)} points")


if __name__ == "__main__":
    main()
