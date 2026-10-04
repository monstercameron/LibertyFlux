"""Shared helpers for the coordinator's operating scripts.

Every script in this folder resolves the repository root, parses addresses,
reads lanes' result files and takes the batch lock through this module, so the
behaviour exists once instead of once per script.

Paths: the repository root is found from the environment (`LIBERTYFLUX_ROOT`,
a test hook), from `git rev-parse`, or from this file's own location
(`scripts/coordinator/` in the tracked tree). The coordinator's working state
(briefs, batch lists, markers) lives untracked under
`.artifacts/scratch/coordinator/` and is reached through `coord_dir()`.
The game folder is only ever taken from `LIBERTYFLUX_GAME_DIR` and the
original executable from `LIBERTYFLUX_ORIG_EXE` (defaulting to
`orig/GTAIV.exe`); no machine path is written in any script.
"""

import contextlib
import datetime
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent

ROOT_ENV = "LIBERTYFLUX_ROOT"
GAME_DIR_ENV = "LIBERTYFLUX_GAME_DIR"
ORIG_EXE_ENV = "LIBERTYFLUX_ORIG_EXE"
BRIEFS_DIR_ENV = "LIBERTYFLUX_BRIEFS_DIR"
LISTS_DIR_ENV = "LIBERTYFLUX_LISTS_DIR"

# A lane's name for a function counts when it describes the function. Slot
# labels (Class::vf12), address-based placeholders and empty names do not.
# (This is the pattern the published counts use; a survey script once carried
# a slightly different one, which is why the pattern lives here now.)
PLACEHOLDER_NAME = re.compile(
    r"::vf\d+$|::vfunc_?\d+$|_static_init_?\d*$|^(fn|sub|fun|func|loc|unk|thunk|nullsub)_"
    r"|^FUN_|unnamed|unknown|^$|_[0-9a-f]{6,8}$",
    re.I,
)


def find_root():
    """The repository root: test hook, then git, then this file's location."""
    override = os.environ.get(ROOT_ENV)
    if override:
        return Path(override)
    try:
        out = subprocess.run(
            ["git", "rev-parse", "--show-toplevel"],
            capture_output=True, text=True, check=True,
        )
        return Path(out.stdout.strip())
    except (OSError, subprocess.CalledProcessError):
        return HERE.parent.parent


def scratch_dir(root=None):
    return (root or find_root()) / ".artifacts" / "scratch"


def coord_dir(root=None):
    """The coordinator's untracked working folder (briefs, lists, markers)."""
    return scratch_dir(root) / "coordinator"


def lists_dir(root=None):
    override = os.environ.get(LISTS_DIR_ENV)
    if override:
        return Path(override)
    return coord_dir(root) / "lists"


def briefs_dir(root=None):
    override = os.environ.get(BRIEFS_DIR_ENV)
    if override:
        return Path(override)
    return coord_dir(root) / "briefs"


def logs_dir(root=None):
    return (root or find_root()) / ".artifacts" / "logs"


def templates_dir():
    """Brief templates tracked beside these scripts."""
    return HERE / "briefs"


def va(value):
    """An address as an int, whether the file wrote a hex string or a number."""
    return int(value, 16) if isinstance(value, str) else int(value)


def is_placeholder_name(label):
    """True when a lane's proposed name is a slot label, placeholder or empty."""
    return bool(PLACEHOLDER_NAME.search(str(label or "")))


def load_rows(data, *keys):
    """A lane-written list in any of its shapes: a bare list, or a dict holding
    the list under one of `keys` (lanes write `functions`, `results` or `names`)."""
    if isinstance(data, list):
        return data
    if isinstance(data, dict):
        for key in keys:
            if isinstance(data.get(key), list):
                return data[key]
    return []


def read_results_rows(path):
    """One lane's result rows as a list of dicts, whatever shape it wrote.

    Returns [] when the file is missing or unreadable, so callers that scan
    hundreds of lane folders need no per-file error handling.
    """
    try:
        data = json.loads(Path(path).read_text(encoding="utf-8"))
    except (ValueError, OSError):
        return []
    return [row for row in load_rows(data, "functions", "results") if isinstance(row, dict)]


def iter_lane_results(scratch, pattern="r-*", require_summary=True):
    """(lane, rows) for every lane folder matching `pattern` (e.g. `r-*`).

    Lanes without `summary.txt` are still running and are skipped, as are
    folders whose results cannot be read.
    """
    scratch = Path(scratch)
    for results in sorted(scratch.glob(f"{pattern}/results.json")):
        lane = results.parent.name
        if require_summary and not (results.parent / "summary.txt").exists():
            continue
        yield lane, read_results_rows(results)


@contextlib.contextmanager
def batch_lock(lists, tries=600, wait=0.5):
    """The exclusive batch lock: brief makers run at the same time and must
    never hand one function to two lanes. Released automatically."""
    import msvcrt

    path = Path(lists)
    path.mkdir(parents=True, exist_ok=True)
    with open(path / ".lock", "a+") as guard:
        for _ in range(tries):
            try:
                guard.seek(0)
                msvcrt.locking(guard.fileno(), msvcrt.LK_NBLCK, 1)
                break
            except OSError:
                time.sleep(wait)
        else:
            raise SystemExit("could not get the batch lock")
        try:
            yield
        finally:
            try:
                guard.seek(0)
                msvcrt.locking(guard.fileno(), msvcrt.LK_UNLCK, 1)
            except OSError:
                pass


def now_stamp():
    """The current time, from the clock, for every timestamp scripts write."""
    return datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def today():
    """Today's date, from the clock."""
    return datetime.date.today().isoformat()


def orig_exe(root=None):
    """The original executable: `LIBERTYFLUX_ORIG_EXE`, else `orig/GTAIV.exe`."""
    override = os.environ.get(ORIG_EXE_ENV)
    if override:
        return Path(override)
    return (root or find_root()) / "orig" / "GTAIV.exe"


def game_dir():
    """The installed game folder, strictly read-only, or None when unset."""
    override = os.environ.get(GAME_DIR_ENV)
    return Path(override) if override else None


def ensure_importable():
    """Make `import common` (and its siblings) work from any driver in this folder."""
    if str(HERE) not in sys.path:
        sys.path.insert(0, str(HERE))
