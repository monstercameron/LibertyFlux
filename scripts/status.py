"""Print the facts for a project status update.

Usage: python scripts/status.py [--brief] [--write [--commit]]

Reads lane logs and deliverables under .artifacts/, the running process list, memory, the Ghidra
analysis log, the progress file and git. Prints plain text. Windows only (uses PowerShell for the
process list and memory).

--brief   list only lanes that are still running or that exited without their deliverables.
--write   also record this tick in docs/data/progress.json (timestamps and lane activity) and
          regenerate the site data and badges.
--commit  with --write, commit the progress files if they changed, with a subject starting
          "Progress:". Progress commits need no changelog entry.
"""

import datetime
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOGS = ROOT / ".artifacts" / "logs"
SCRATCH = ROOT / ".artifacts" / "scratch"
DELIVERABLES = ("summary.txt", "report.md", "devlog-entry.html")


def powershell(command):
    result = subprocess.run(
        ["powershell", "-NoProfile", "-Command", command],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
    )
    return result.stdout.strip()


def running_lanes():
    """Lane names with a live Muse process, and the memory those processes use."""
    out = powershell(
        "Get-CimInstance Win32_Process | Where-Object { $_.Name -like 'muse-bin*' } | "
        "ForEach-Object { '{0}|{1}' -f $_.WorkingSetSize, $_.CommandLine }"
    )
    lanes, memory = set(), 0
    for line in out.splitlines():
        size, _, command = line.partition("|")
        match = re.search(r"briefs\\([\w-]+)\.txt", command)
        if match:
            lanes.add(match.group(1))
        if size.isdigit():
            memory += int(size)
    return lanes, memory


def last_line(path):
    try:
        lines = [l for l in path.read_text(encoding="utf-8", errors="replace").splitlines() if l.strip()]
        return lines[-1] if lines else ""
    except OSError:
        return ""


def age_minutes(path):
    return (datetime.datetime.now().timestamp() - path.stat().st_mtime) / 60


def production_done(lane):
    """Production lanes write results.json and a summary; a devlog entry is optional for them."""
    folder = SCRATCH / lane
    return (folder / "results.json").exists() and (folder / "summary.txt").exists()


def main():
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    progress = json.loads((ROOT / "docs" / "data" / "progress.json").read_text(encoding="utf-8"))
    print(f"status at {now}")
    print(f"phase {progress['phase']['index']} {progress['phase']['name']}; "
          f"functions game={progress['functions']['game']} stages={progress['stages']} "
          f"structures={progress['structures']} symbols={progress['symbols']}")

    live, muse_memory = running_lanes()
    rows = []
    for log in sorted(LOGS.glob("*.log")):
        lane = log.stem
        if lane.startswith("ghidra"):
            continue
        done = [name for name in DELIVERABLES if (SCRATCH / lane / name).exists()]
        text = last_line(log)
        exited = "lane exited with code" in text
        if lane in live:
            state = "running"
        elif exited and (len(done) == len(DELIVERABLES) or production_done(lane)):
            state = "finished"
        elif exited:
            state = "exited-incomplete"
        else:
            state = "not-running"
        rows.append((state, lane, age_minutes(log), len(done), text[:150]))

    counts = {}
    for state, *_ in rows:
        counts[state] = counts.get(state, 0) + 1
    print("lanes: " + ", ".join(f"{n} {s}" for s, n in sorted(counts.items())) if rows else "lanes: none")
    brief = "--brief" in sys.argv
    for state, lane, age, done, text in rows:
        if brief and state == "finished":
            continue
        print(f"  {state:17} {lane:22} log {age:4.0f} min ago, {done}/3 deliverables | {text}")
    pending = sorted(
        d.name for d in SCRATCH.iterdir()
        if (d / "devlog-entry.html").exists() and (d / "summary.txt").exists() and not (d / "integrated.txt").exists()
    ) if SCRATCH.exists() else []
    print("finished but not yet in the devlog: " + (", ".join(pending) or "none"))

    ghidra_log = LOGS / "ghidra-analysis.log"
    if ghidra_log.exists():
        ghidra_running = "java" in powershell(
            "Get-CimInstance Win32_Process | Where-Object { $_.Name -like 'java*' -and "
            "$_.CommandLine -like '*AnalyzeHeadless*' } | Select-Object -First 1 -ExpandProperty Name")
        print(f"ghidra: {'running' if ghidra_running else 'not running'}, log {age_minutes(ghidra_log):.0f} min ago, "
              f"{ghidra_log.stat().st_size // 1024} KB | {last_line(ghidra_log)[:160]}")
        project = ROOT / ".artifacts" / "cache" / "ghidra"
        if project.exists():
            size = sum(f.stat().st_size for f in project.rglob("*") if f.is_file())
            print(f"ghidra project on disk: {size / 1e6:.0f} MB")

    memory = powershell(
        "$o = Get-CimInstance Win32_OperatingSystem; "
        "'{0:N1} GB free of {1:N1} GB' -f ($o.FreePhysicalMemory/1MB), ($o.TotalVisibleMemorySize/1MB)")
    print(f"memory: {memory}; muse processes use {muse_memory / 1e9:.1f} GB across {len(live)} lanes")

    git = subprocess.run(["git", "-C", str(ROOT), "status", "--short"], capture_output=True, text=True).stdout
    ahead = subprocess.run(["git", "-C", str(ROOT), "rev-list", "--count", "origin/main..HEAD"],
                           capture_output=True, text=True).stdout.strip()
    print(f"git: {len(git.splitlines())} uncommitted paths, {ahead or '?'} commits not pushed")

    if "--write" in sys.argv:
        write_progress(progress, counts)


def write_progress(progress, counts):
    """Record this tick in docs/data/progress.json, regenerate the site data, and commit it.

    Only the activity block and the timestamps are written here. Measured numbers are set by the
    coordinator from lane results, never by this function.
    """
    import sqlite3
    database = ROOT / "docs" / "data" / "devlog.sqlite"
    devlog_entries = 0
    if database.exists():
        with sqlite3.connect(database) as connection:
            devlog_entries = connection.execute("SELECT COUNT(*) FROM posts").fetchone()[0]
    now = datetime.datetime.now()
    progress["updated"] = now.strftime("%Y-%m-%d")
    progress["updated_at"] = now.strftime("%Y-%m-%d %H:%M")
    progress["activity"] = {
        "lanes_running": counts.get("running", 0),
        "lanes_finished": counts.get("finished", 0),
        "lanes_incomplete": counts.get("exited-incomplete", 0) + counts.get("not-running", 0),
        "devlog_entries": devlog_entries,
    }
    path = ROOT / "docs" / "data" / "progress.json"
    path.write_text(json.dumps(progress, indent=2) + "\n", encoding="utf-8", newline="\n")
    subprocess.run([sys.executable, str(ROOT / "scripts" / "update_progress.py")], cwd=ROOT, capture_output=True)
    # The site's derived data, regenerated from what was just written so the pages never lag the counts: the
    # progress chart's series, the quality page's numbers and the devlog's feed. Each writes only on a change;
    # a generator that fails leaves its file as it was and is reported, and the tick goes on.
    for generator in ("history.py", "quality.py", "devlog_feed.py"):
        script = ROOT / "scripts" / "site" / generator
        if script.exists():
            made = subprocess.run([sys.executable, str(script)], cwd=ROOT, capture_output=True, text=True)
            if made.returncode != 0:
                print(f"site generator {generator} failed: " + (made.stderr.strip().splitlines() or ["no message"])[-1][:160])
    if "--commit" in sys.argv:
        # changelog.json is included because regenerating fills in the hash of the newest entry.
        paths = ["docs/data/progress.json", "docs/data/changelog.json", "docs/badges",
                 "docs/data/history.json", "docs/data/quality.json", "docs/devlog.xml"]
        subprocess.run(["git", "-C", str(ROOT), "add", "--"] + paths)
        staged = subprocess.run(["git", "-C", str(ROOT), "diff", "--cached", "--quiet", "--"] + paths).returncode
        if staged:
            a = progress["activity"]
            subject = (f"Progress: phase {progress['phase']['index']}, {a['lanes_running']} lanes running, "
                       f"{a['lanes_finished']} finished, {a['devlog_entries']} devlog entries")
            subprocess.run(["git", "-C", str(ROOT), "commit", "-q", "-m", subject, "--"] + paths)
            push = subprocess.run(["git", "-C", str(ROOT), "push", "-q", "origin", "main"],
                                  capture_output=True, text=True)
            print("progress committed: " + subject + (" (pushed)" if push.returncode == 0 else
                                                      " (PUSH FAILED: " + push.stderr.strip()[:120] + ")"))
        else:
            print("progress unchanged, nothing committed")


if __name__ == "__main__":
    main()
