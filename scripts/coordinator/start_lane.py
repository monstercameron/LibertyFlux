"""Start one lane so that it outlives whatever started it.

Usage: python start_lane.py <lane> [<effort>]

Runs run-lane.ps1 detached (its own process group, no console): stopping the
supervisor task must not kill the lanes it started. Output goes to
.artifacts/logs/lanes-out/<lane>.out and .err, as before.
"""

import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common


def start_lane(lane, effort="max"):
    root = common.find_root()
    here = common.HERE
    out_dir = common.logs_dir(root) / "lanes-out"
    out_dir.mkdir(parents=True, exist_ok=True)
    with open(out_dir / f"{lane}.out", "w") as stdout, open(out_dir / f"{lane}.err", "w") as stderr:
        subprocess.Popen(["powershell.exe", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File",
                          str(here / "run-lane.ps1"), "-Lane", lane, "-Effort", effort],
                         stdout=stdout, stderr=stderr, cwd=str(root),
                         creationflags=subprocess.CREATE_NEW_PROCESS_GROUP | subprocess.DETACHED_PROCESS)
    return lane


def main():
    lane = sys.argv[1]
    effort = sys.argv[2] if len(sys.argv) > 2 else "max"
    start_lane(lane, effort)
    print(f"started {lane} (effort {effort})")


if __name__ == "__main__":
    main()
