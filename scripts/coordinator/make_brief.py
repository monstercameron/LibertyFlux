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


def parse_args(argv):
    """(lane, task_file, date) from the command line; `--date VALUE` and `--date=VALUE` both work, anywhere.

    The value after a bare `--date` is consumed (it used to be left among the positional arguments, so
    `make_brief.py --date 2026-10-04 lane task.txt` took the date for the lane name)."""
    positional, date = [], None
    items = iter(argv)
    for item in items:
        if item == "--date":
            date = next(items, None)
            if date is None:
                raise SystemExit("--date needs a value (YYYY-MM-DD)")
        elif item.startswith("--date="):
            date = item.split("=", 1)[1]
        else:
            positional.append(item)
    if len(positional) != 2:
        raise SystemExit(__doc__.strip().splitlines()[2])
    return positional[0], positional[1], date


def main():
    lane, task_file, date = parse_args(sys.argv[1:])
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
