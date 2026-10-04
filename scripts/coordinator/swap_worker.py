"""Put a newly built checker worker where lanes look for it. A running executable can be renamed on Windows but
not overwritten, so the old one is renamed aside (running workers keep using it) and the new one copied in.

Usage: python swap_worker.py <build-folder-name> <label-for-the-old-one>
"""

import os
import shutil
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

ROOT = common.find_root()
REL = Path("i686-pc-windows-msvc") / "release" / "lf-checker-worker.exe"


def swap(new, live, label):
    """Rename the live worker aside as lf-checker-worker.<label>.exe and copy `new` into its place.

    Refuses (returns None, touching nothing) when the new build is missing or the aside name is already taken,
    so an older kept worker is never overwritten; an absent live worker is simply installed. Returns the aside
    path on success."""
    new, live = Path(new), Path(live)
    aside = live.with_name(f"lf-checker-worker.{label}.exe")
    if not new.exists() or aside.exists():
        return None
    if live.exists():
        live.rename(aside)
    shutil.copy2(new, live)
    return aside


def main():
    new = ROOT / ".artifacts" / "build" / sys.argv[1] / REL
    live = ROOT / ".artifacts" / "build" / "cargo-tools" / REL
    aside = live.with_name(f"lf-checker-worker.{sys.argv[2]}.exe")
    assert new.exists(), new
    if swap(new, live, sys.argv[2]) is None:
        print("aside name already used:", aside.name)
        sys.exit(1)
    print(f"old worker kept as {aside.name}; live worker is now {live.stat().st_size} bytes")


if __name__ == "__main__":
    main()
