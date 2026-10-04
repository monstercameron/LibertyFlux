"""Integrate finished lanes' devlog fragments, one commit per lane.

Usage: python integrate_and_commit.py [--force lane] <lane> [<lane> ...]
With no lanes given, integrates every lane that has devlog-entry.html and no integrated.txt.
"""

import datetime
import json
import os
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

ROOT = common.find_root()
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)
HERE = Path(__file__).resolve().parent
PY = sys.executable


def main():
    args = sys.argv[1:]
    forced = set()
    while "--force" in args:
        i = args.index("--force")
        forced.add(args[i + 1])
        del args[i:i + 2]
    lanes = args or sorted(
        d.name for d in SCRATCH.iterdir()
        if (d / "devlog-entry.html").exists() and not (d / "integrated.txt").exists()
        and (d / "summary.txt").exists()
    )

    for lane in lanes:
        cmd = [PY, str(ROOT / "scripts" / "integrate_devlog.py"), lane] + (["--force"] if lane in forced else [])
        result = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, encoding="utf-8")
        print(result.stdout.strip())
        if result.returncode != 0:
            continue
        heading = re.search(r"integrated as #[a-z0-9-]+: (.*?)(?: \(forced\))?$", result.stdout.strip(), re.M).group(1)
        entry = {
            "date": datetime.date.today().isoformat(),
            "title": f"Devlog: {heading}"[:120],
            "summary": f"Findings from the {lane} lane.",
            "changes": [
                f"New devlog entry: {heading}.",
                "Written by a Muse lane and checked before publishing for addresses, local paths, byte dumps and code.",
            ],
            "files": ["docs/devlog.html"],
        }
        path = COORD / f"entry-lane-{lane}.json"
        path.write_text(json.dumps(entry, indent=2), encoding="utf-8")
        env = dict(os.environ, LF_BATCH="1")
        commit = subprocess.run([PY, str(HERE / "commit_one.py"), str(path)], cwd=ROOT, capture_output=True, text=True,
                                encoding="utf-8", env=env)
        print("  " + commit.stdout.strip().splitlines()[0] if commit.stdout.strip() else "  commit failed: " + commit.stderr[:200])

    # One push for the whole batch: GitHub Pages fails builds that are triggered seconds apart.
    push = subprocess.run(["git", "push", "-q", "origin", "main"], cwd=ROOT, capture_output=True, text=True)
    print("batch pushed" if push.returncode == 0 else "PUSH FAILED: " + push.stderr.strip()[:200])


if __name__ == "__main__":
    main()
