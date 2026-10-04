"""Give back the batches of production lanes that stopped without results, so new lanes can take them.

A lane's batch list is moved to lists/released/ when status.py reports the lane as exited-incomplete or
not-running and it has no results.json. Lanes that are still running are never touched, and neither is
anything launched in the last 15 minutes: a lane that was launched a moment ago has no process yet and
looks dead. Runs on every tick, so a dead lane's batch is released automatically.
"""

import os
import re
import shutil
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

# A batch is never released within this long of its launch.
RELEASE_GRACE_MINUTES = 15


def main():
    root = common.find_root()
    lists = common.lists_dir(root)
    out = subprocess.run([sys.executable, str(root / "scripts" / "status.py"), "--brief"], cwd=root,
                         capture_output=True, text=True, encoding="utf-8").stdout
    released = []
    for state, lane in re.findall(r"^\s+(exited-incomplete|not-running)\s+(r-[nsb]\d+)\s", out, re.M):
        batch = lists / f"{lane}.json"
        if not batch.exists() or (common.scratch_dir(root) / lane / "results.json").exists():
            continue
        log = common.logs_dir(root) / f"{lane}.log"
        newest = max(batch.stat().st_mtime, log.stat().st_mtime if log.exists() else 0)
        if time.time() - newest < RELEASE_GRACE_MINUTES * 60:
            continue
        (lists / "released").mkdir(exist_ok=True)
        shutil.move(str(batch), str(lists / "released" / f"{lane}.json"))
        released.append(f"{lane} ({state})")
    print(f"released {len(released)} batches:", ", ".join(released) or "none")


if __name__ == "__main__":
    main()
