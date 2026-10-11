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

HERE = Path(__file__).resolve().parent
ROOT = next((path for path in (HERE, *HERE.parents) if (path / "docs" / "data" / "progress.json").is_file()), HERE.parents[1])
LOGS = ROOT / ".artifacts" / "logs"
SCRATCH = ROOT / ".artifacts" / "scratch"
CODEX_REGISTRY = SCRATCH / "coordinator" / "codex_lanes.json"
CODEX_REGISTRY_MAX_AGE_SECONDS = 180
REGISTRY_STATES = {"running", "finished", "died", "stopped", "unknown"}
DELIVERABLES = ("summary.txt", "report.md", "devlog-entry.html")


def powershell(command):
    result = subprocess.run(
        ["powershell", "-NoProfile", "-Command", command],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
    )
    return result.stdout.strip()


def running_lanes():
    """Lane names with a live agent process of their own, and the memory those processes use.

    A Muse lane is found by the brief its process was given. A Luna lane started with the Codex command line is
    found by the file its last message goes to, which is in the lane's scratch folder (since 10 October 2026).
    """
    out = powershell(
        "Get-CimInstance Win32_Process | Where-Object { $_.Name -like 'muse-bin*' -or "
        "($_.Name -eq 'codex.exe' -and $_.CommandLine -like '*codex-last-message.txt*') } | "
        "ForEach-Object { '{0}|{1}' -f $_.WorkingSetSize, $_.CommandLine }"
    )
    lanes, memory = set(), 0
    for line in out.splitlines():
        size, _, command = line.partition("|")
        match = re.search(r"briefs\\([\w-]+)\.txt", command) or re.search(r"scratch\\([\w-]+)\\codex-last-message\.txt", command)
        if match:
            lanes.add(match.group(1))
        if size.isdigit():
            memory += int(size)
    return lanes, memory


def read_codex_registry(path=CODEX_REGISTRY, now=None):
    """Read explicit coordinator state; stale or invalid data never reports a lane as running."""
    now = datetime.datetime.now().timestamp() if now is None else now
    try:
        data = json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, ValueError):
        return {}, {"state": "unknown", "fresh": False, "age_seconds": None}
    if not isinstance(data, dict) or not isinstance(data.get("lanes"), list):
        return {}, {"state": "unknown", "fresh": False, "age_seconds": None}
    updated_at = data.get("updated_at")
    try:
        updated = datetime.datetime.fromisoformat(updated_at.replace("Z", "+00:00"))
        age = now - updated.timestamp()
    except (AttributeError, TypeError, ValueError, OverflowError):
        return {}, {"state": "unknown", "fresh": False, "age_seconds": None}
    fresh = 0 <= age <= CODEX_REGISTRY_MAX_AGE_SECONDS
    lanes = {}
    for item in data["lanes"]:
        if not isinstance(item, dict):
            continue
        lane = item.get("lane")
        if not isinstance(lane, str) or not re.fullmatch(r"[A-Za-z0-9][\w-]{0,40}", lane):
            continue
        state = item.get("state")
        if not isinstance(state, str) or state not in REGISTRY_STATES or not fresh:
            state = "unknown"
        lanes[lane] = {"state": state,
                       "model": item.get("model") if isinstance(item.get("model"), str) else None,
                       "effort": item.get("effort") if isinstance(item.get("effort"), str) else None}
    return lanes, {"state": "fresh" if fresh else ("stale" if age > CODEX_REGISTRY_MAX_AGE_SECONDS else "unknown"),
                   "fresh": fresh, "age_seconds": round(age, 1)}


def classify_lane(lane, registry_lanes, muse_live, exited, deliverables_complete, production_complete):
    """Prefer the coordinator's explicit lane state over log or deliverable heuristics."""
    if lane in registry_lanes:
        return registry_lanes[lane]["state"]
    if lane in muse_live:
        return "running"
    if exited and (deliverables_complete or production_complete):
        return "finished"
    if exited:
        return "exited-incomplete"
    return "not-running"


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

    muse_live, muse_memory = running_lanes()
    registry_lanes, registry_status = read_codex_registry(now=datetime.datetime.now().timestamp())
    rows = []
    seen = set()
    for log in sorted(LOGS.glob("*.log")):
        lane = log.stem
        if lane.startswith("ghidra"):
            continue
        seen.add(lane)
        done = [name for name in DELIVERABLES if (SCRATCH / lane / name).exists()]
        text = last_line(log)
        exited = "lane exited with code" in text
        state = classify_lane(lane, registry_lanes, muse_live, exited, len(done) == len(DELIVERABLES), production_done(lane))
        rows.append((state, lane, age_minutes(log), len(done), text[:150], registry_lanes.get(lane)))
    for lane, entry in registry_lanes.items():
        if lane in seen:
            continue
        done = sum(1 for name in DELIVERABLES if (SCRATCH / lane / name).exists())
        rows.append((entry["state"], lane, None, done, "no log file", entry))

    counts = {}
    for state, *_ in rows:
        counts[state] = counts.get(state, 0) + 1
    print("lanes: " + ", ".join(f"{n} {s}" for s, n in sorted(counts.items())) if rows else "lanes: none")
    brief = "--brief" in sys.argv
    for state, lane, age, done, text, entry in rows:
        if brief and state == "finished":
            continue
        age_text = f"log {age:4.0f} min ago" if age is not None else "log age unknown"
        model_text = ""
        if entry:
            model_text = f" | {entry.get('model') or 'model unknown'} / {entry.get('effort') or 'effort unknown'}"
        print(f"  {state:17} {lane:22} {age_text}, {done}/3 deliverables | {text[:150]}{model_text}")
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
    codex_memory = "; native Codex subagent memory unavailable per lane (no per-task OS PID)" if registry_lanes else ""
    print(f"memory: {memory}; Muse processes use {muse_memory / 1e9:.1f} GB across {len(muse_live)} lanes{codex_memory}")
    if CODEX_REGISTRY.exists():
        age_text = f", {registry_status['age_seconds']:.0f}s old" if registry_status["age_seconds"] is not None else ""
        print(f"Codex registry: {registry_status['state']}{age_text}; stale or invalid entries are never counted as running")

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
        "lanes_incomplete": counts.get("exited-incomplete", 0) + counts.get("not-running", 0) + counts.get("died", 0) + counts.get("stopped", 0) + counts.get("unknown", 0),
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
