"""Local dashboard for watching the lane swarm without a model in the loop.

Serves one page and a small read-only JSON API on 127.0.0.1. It reads what is already on disk under
.artifacts/ (lane logs, briefs, results, the supervisor's log and settings), the site's progress data, and
the process list. It writes nothing and starts nothing.

    python scripts/dashboard/server.py [port]        (default 8772)

It binds to 127.0.0.1 only: briefs and logs contain machine paths and addresses and are not for publication.
"""

import ctypes
import json
import os
import re
import subprocess
import sys
import threading
import time
from datetime import datetime
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ART = ROOT / ".artifacts"
LOGS = ART / "logs"
SCRATCH = ART / "scratch"
COORD = SCRATCH / "coordinator"
BRIEFS = COORD / "briefs"
LISTS = COORD / "lists"
LANE_NAME = re.compile(r"^[A-Za-z0-9][\w-]{0,40}$")
RECENT_SECONDS = 3 * 3600

_processes = {"at": 0.0, "lanes": {}, "error": None}
_lock = threading.Lock()


class MemoryStatus(ctypes.Structure):
    _fields_ = [("dwLength", ctypes.c_ulong), ("dwMemoryLoad", ctypes.c_ulong), ("ullTotalPhys", ctypes.c_ulonglong),
                ("ullAvailPhys", ctypes.c_ulonglong), ("ullTotalPageFile", ctypes.c_ulonglong),
                ("ullAvailPageFile", ctypes.c_ulonglong), ("ullTotalVirtual", ctypes.c_ulonglong),
                ("ullAvailVirtual", ctypes.c_ulonglong), ("ullAvailExtendedVirtual", ctypes.c_ulonglong)]


def memory():
    if os.name != "nt":
        return None
    status = MemoryStatus()
    status.dwLength = ctypes.sizeof(MemoryStatus)
    ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(status))
    gb = 1024 ** 3
    return {"total_gb": round(status.ullTotalPhys / gb, 1), "free_gb": round(status.ullAvailPhys / gb, 1),
            "commit_limit_gb": round(status.ullTotalPageFile / gb, 1), "commit_headroom_gb": round(status.ullAvailPageFile / gb, 1)}


def refresh_processes():
    """Which lanes have a live agent process, with its start time and reasoning effort (from its command line)."""
    command = ("Get-CimInstance Win32_Process -Filter \"Name like 'muse%'\" | ForEach-Object { "
               "[pscustomobject]@{ pid = $_.ProcessId; started = $_.CreationDate.ToString('s'); cmd = $_.CommandLine } } | ConvertTo-Json -Compress")
    try:
        out = subprocess.run(["powershell", "-NoProfile", "-Command", command], capture_output=True, text=True, timeout=40).stdout.strip()
        rows = json.loads(out) if out else []
        rows = rows if isinstance(rows, list) else [rows]
        lanes = {}
        for row in rows:
            cmd = row.get("cmd") or ""
            brief = re.search(r"briefs[\\/]([\w-]+)\.txt", cmd)
            if not brief:
                continue
            effort = re.search(r"--reasoning-effort\s+(\S+)", cmd)
            model = re.search(r"--model\s+(\S+)", cmd)
            lanes[brief.group(1)] = {"pid": row.get("pid"), "started": row.get("started"),
                                     "effort": effort.group(1) if effort else None, "model": model.group(1) if model else None}
        with _lock:
            _processes.update(at=time.time(), lanes=lanes, error=None)
    except Exception as error:  # the page says so instead of showing stale data as live
        with _lock:
            _processes.update(at=time.time(), error=str(error))


def process_loop():
    while True:
        refresh_processes()
        time.sleep(20)


_rows = {}


def cached_row(lane, live, experiments):
    """A lane's row changes only when its log, results or process state change; reading 170 lanes' files on
    every poll took almost two seconds."""
    def mtime(path):
        try:
            return path.stat().st_mtime
        except OSError:
            return 0
    key = (mtime(LOGS / f"{lane}.log"), mtime(SCRATCH / lane / "results.json"), mtime(SCRATCH / lane / "names.json"),
           mtime(SCRATCH / lane / "summary.txt"), json.dumps(live.get(lane), sort_keys=True))
    hit = _rows.get(lane)
    if hit and hit[0] == key:
        return hit[1]
    row = lane_row(lane, live, experiments)
    _rows[lane] = (key, row)
    return row


def read_json(path, default=None):
    try:
        return json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, ValueError):
        return default


def tail(path, lines=1, limit=64 * 1024):
    try:
        with open(path, "rb") as handle:
            handle.seek(0, os.SEEK_END)
            size = handle.tell()
            handle.seek(max(0, size - limit))
            text = handle.read().decode("utf-8", errors="replace")
    except OSError:
        return []
    rows = [row.rstrip() for row in text.splitlines() if row.strip()]
    return rows[-lines:]


def kind_of(lane):
    if lane.startswith("r-b"):
        return "large functions"
    if lane.startswith("r-s"):
        return "small functions"
    if lane.startswith("r-n"):
        return "native handlers"
    if lane.startswith("nm-"):
        return "naming"
    if lane.startswith("a-"):
        return "re-run"
    return "special"


def result_rows(lane):
    rows = read_json(SCRATCH / lane / "results.json")
    if isinstance(rows, dict):
        rows = rows.get("functions") or rows.get("results") or rows.get("names")
    return rows if isinstance(rows, list) else None


def yield_of(lane):
    rows = result_rows(lane)
    if rows is None:
        names = read_json(SCRATCH / lane / "names.json")
        if isinstance(names, list):
            return {"done": sum(1 for r in names if isinstance(r, dict) and r.get("name")), "of": len(names), "unit": "named"}
        return None
    rows = [r for r in rows if isinstance(r, dict)]
    if not rows or "outcome" not in rows[0]:
        return None
    return {"done": sum(1 for r in rows if str(r.get("outcome")).startswith("verified")), "of": len(rows), "unit": "verified"}


def brief_tag(lane):
    path = BRIEFS / f"{lane}.txt"
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return None
    match = re.search(r"`brief` \(write `([\w.-]+)`\)", text)
    return match.group(1) if match else None


def lane_row(lane, live, experiments):
    log = LOGS / f"{lane}.log"
    try:
        stat = log.stat()
    except OSError:
        stat = None
    last = tail(log, 1)
    process = live.get(lane)
    exited = None
    if last:
        match = re.search(r"lane exited with code (-?\d+)", last[0])
        if match:
            exited = int(match.group(1))
    has_results = (SCRATCH / lane / "summary.txt").exists()
    if process:
        state = "running"
    elif exited == 0 or has_results:
        state = "finished"
    elif exited is not None:
        state = "died"
    else:
        state = "stopped"
    progress = last[0] if last else ""
    if exited is not None:
        earlier = tail(log, 3)
        progress = next((row for row in reversed(earlier) if "coordinator:" not in row), progress)
    batch = read_json(LISTS / f"{lane}.json")
    experiment = experiments.get(lane)
    return {"lane": lane, "kind": kind_of(lane), "state": state, "exit_code": exited,
            "effort": (process or {}).get("effort") or (experiment or {}).get("effort"),
            "started": (process or {}).get("started"),
            "last_activity": datetime.fromtimestamp(stat.st_mtime).isoformat(timespec="seconds") if stat else None,
            "progress": progress[:300], "yield": yield_of(lane), "brief": brief_tag(lane),
            "experiment": (experiment or {}).get("factor"), "batch": len(batch) if isinstance(batch, list) else None}


def state():
    # The first answer waits for the first process list, so lanes are not shown as stopped while it loads.
    deadline = time.time() + 45
    while not _processes["at"] and time.time() < deadline:
        time.sleep(0.2)
    with _lock:
        live = dict(_processes["lanes"])
        process_age = time.time() - _processes["at"] if _processes["at"] else None
        process_error = _processes["error"]
    experiments = {row["lane"]: row for row in (read_json(COORD / "experiments.json", []) or []) if not row.get("never_ran")}
    now = time.time()
    names = set(live)
    try:
        for entry in os.scandir(LOGS):
            if entry.name.endswith(".log") and now - entry.stat().st_mtime < RECENT_SECONDS:
                name = entry.name[:-4]
                if LANE_NAME.match(name) and name != "supervisor":
                    names.add(name)
    except OSError:
        pass
    lanes = [cached_row(name, live, experiments) for name in sorted(names)]
    order = {"running": 0, "died": 1, "stopped": 2, "finished": 3}
    lanes.sort(key=lambda row: (order[row["state"]], row["last_activity"] or ""), reverse=False)
    progress = read_json(ROOT / "docs" / "data" / "progress.json", {}) or {}
    history = read_json(COORD / "review_history.json", []) or []
    return {"now": datetime.now().isoformat(timespec="seconds"), "memory": memory(),
            "process_list_age_s": round(process_age, 1) if process_age is not None else None, "process_list_error": process_error,
            "functions": progress.get("functions"), "stages": progress.get("stages"), "phase": progress.get("phase"),
            "progress_updated": progress.get("updated_at") or progress.get("updated"),
            "review": history[-1] if history else None,
            "supervisor": {"settings": read_json(COORD / "supervisor.json"), "log": tail(LOGS / "supervisor.log", 6),
                           "stop_file": (COORD / "STOP").exists()},
            "lanes": lanes}


def lane_detail(lane):
    if not LANE_NAME.match(lane):
        return None
    brief = BRIEFS / f"{lane}.txt"
    try:
        prompt = brief.read_text(encoding="utf-8", errors="replace")
    except OSError:
        prompt = None
    summary = SCRATCH / lane / "summary.txt"
    try:
        summary_text = summary.read_text(encoding="utf-8", errors="replace")
    except OSError:
        summary_text = None
    rows = result_rows(lane)
    slim = None
    if rows:
        slim = [{"address": r.get("address"), "name": r.get("name"), "size": r.get("size"), "outcome": r.get("outcome"),
                 "reason": r.get("reason"), "attempts": r.get("attempts"), "minutes": r.get("minutes"),
                 "mutant_caught": r.get("mutant_caught"), "confidence": r.get("confidence"),
                 "detail": str(r.get("detail") or r.get("evidence") or "")[:400]} for r in rows if isinstance(r, dict)][:400]
    return {"lane": lane, "prompt": prompt, "prompt_chars": len(prompt) if prompt else 0, "log": tail(LOGS / f"{lane}.log", 80),
            "summary": summary_text, "results": slim, "batch": read_json(LISTS / f"{lane}.json")}


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def send(self, body, content_type="application/json; charset=utf-8", status=200):
        data = body if isinstance(body, bytes) else body.encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        try:
            if path in ("/", "/index.html"):
                self.send((HERE / "index.html").read_bytes(), "text/html; charset=utf-8")
            elif path == "/api/state":
                self.send(json.dumps(state()))
            elif path.startswith("/api/lane/"):
                detail = lane_detail(path[len("/api/lane/"):])
                self.send(json.dumps(detail) if detail else '{"error":"no such lane"}', status=200 if detail else 404)
            else:
                self.send('{"error":"not found"}', status=404)
        except Exception as error:
            self.send(json.dumps({"error": str(error)}), status=500)


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8772
    threading.Thread(target=process_loop, daemon=True).start()
    print(f"LibertyFlux dashboard on http://127.0.0.1:{port}/ (local only)", flush=True)
    ThreadingHTTPServer(("127.0.0.1", port), Handler).serve_forever()
