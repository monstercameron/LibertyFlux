"""Write one lane brief from the standard opening plus a task file.

Usage: python make_brief.py <lane> <task-file> [--date YYYY-MM-DD]

The task file holds the lane's task starting with "Your task:"; the tokens
{root}, {lane} and {date} in it are replaced (plain replacement, so braces
used for anything else are left alone). This replaces the old pattern of
borrowing the opening section by slicing another lane's brief file.
"""

import os
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
from briefs import render_head


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--date")]
    date = None
    for i, a in enumerate(sys.argv[1:]):
        if a == "--date":
            date = sys.argv[1:][i + 1]
        elif a.startswith("--date="):
            date = a.split("=", 1)[1]
    lane, task_file = args[0], args[1]
    root = common.find_root()
    task = Path(task_file).read_text(encoding="utf-8")
    task = task.replace("{root}", str(root)).replace("{lane}", lane).replace("{date}", date or common.today())
    out = common.briefs_dir(root) / f"{lane}.txt"
    out.parent.mkdir(parents=True, exist_ok=True)
    text = render_head(lane, str(root), date) + task
    out.write_text(text, encoding="utf-8", newline="\n")
    print(f"{lane} brief written, {len(text)} characters")


if __name__ == "__main__":
    main()
