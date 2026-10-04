"""Coordinator tick: integrate finished lanes, show their summaries, update and push progress, print brief status."""

import os
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common

HERE = Path(__file__).resolve().parent
ROOT = common.find_root()
PY = sys.executable


def integrated_lanes(out):
    """Lanes integrate_and_commit.py reports as integrated, in its order."""
    return re.findall(r"^([\w-]+): integrated as", out, re.M)


def failed_lanes(out):
    """Lanes integrate_and_commit.py could not integrate."""
    return re.findall(r"^([\w-]+): NOT integrated", out, re.M)


def summary_head(text, limit):
    """The first `limit` non-blank lines of a lane's summary, each cut to 200 characters and indented."""
    return ["  " + line[:200] for line in text.splitlines() if line.strip()][:limit]


def sync_report(text):
    """The lines of sync_rewrites.py's output worth showing on a tick."""
    return [line for line in text.splitlines()
            if line.startswith(("added", "left out", "FAILED", "demoted", "schema warning")) or "Rewrites:" in line]


def main():
    out = subprocess.run([PY, str(HERE / "integrate_and_commit.py")], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    print(out.strip() or "no new lane entries")
    for lane in integrated_lanes(out):
        summary = common.scratch_dir(ROOT) / lane / "summary.txt"
        if summary.exists():
            print(f"=== {lane}")
            limit = int(sys.argv[1]) if len(sys.argv) > 1 else 4
            for line in summary_head(summary.read_text(encoding="utf-8", errors="replace"), limit):
                print(line)
    for lane in failed_lanes(out):
        print(f"!!! {lane} needs attention")
    # Watchdog: the checker driver's memory grows without bound on long runs (one reached 8.9 GB on 2026-10-04).
    # Stop any checker driver process above 4 GB before it takes the machine down, and say so.
    watchdog = subprocess.run(["powershell", "-NoProfile", "-Command",
        "Get-CimInstance Win32_Process -Filter \"Name='python.exe'\" | Where-Object { $_.PrivatePageCount -gt 4GB -and "
        "$_.CommandLine -match 'check|run\\.py|rerun' } | ForEach-Object { taskkill /PID $_.ProcessId /T /F | Out-Null; "
        "'WATCHDOG stopped python pid {0} at {1:N1} GB: {2}' -f $_.ProcessId, ($_.PrivatePageCount / 1GB), "
        "$_.CommandLine.Substring(0, [Math]::Min(120, $_.CommandLine.Length)) }"], capture_output=True, text=True)
    if watchdog.stdout.strip():
        print(watchdog.stdout.strip())
    # New rewrites that passed the checker go into the repository on every tick; the push happens with the progress commit.
    sync = subprocess.run([PY, str(HERE / "sync_rewrites.py"), "--commit"], cwd=ROOT, capture_output=True, text=True,
                          encoding="utf-8", env=dict(os.environ, LF_BATCH="1"))
    print("\n".join(sync_report(sync.stdout + sync.stderr)))
    # Lanes that died without results give their batches back to the queue (2026-10-04: until this ran on every tick,
    # dead lanes' functions stayed reserved until the coordinator noticed).
    released = subprocess.run([PY, str(HERE / "release_failed.py")], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout.strip()
    if released and not released.endswith("none"):
        print(released)
    counts = subprocess.run([PY, str(HERE / "apply_counts.py")], cwd=ROOT, capture_output=True, text=True, encoding="utf-8")
    print((counts.stdout + counts.stderr).strip())
    status = subprocess.run([PY, str(ROOT / "scripts" / "status.py"), "--brief", "--write", "--commit"],
                            cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    print(status)


if __name__ == "__main__":
    main()
