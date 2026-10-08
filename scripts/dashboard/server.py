"""Local dashboard for watching the lane swarm without a model in the loop.

Serves one page and a small read-only JSON API on 127.0.0.1. It reads what is already on disk under
.artifacts/ (lane logs, briefs, results, the supervisor's log and settings), the site's progress data, and
the process list. It starts nothing. The one file it writes is its own rolling history,
.artifacts/cache/dashboard/history.jsonl (one sample a minute while it runs, a week kept; never tracked),
which the page's history charts draw from together with the hourly review snapshots.

    python scripts/dashboard/server.py [port]        (default 8772)

API: /api/state, /api/lane/<lane>, /api/history?hours=N (lanes running, verified, memory over time),
/api/experiments (pass rate and time per function for each brief version and experiment tag).

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
from urllib.parse import parse_qs, urlsplit

HERE = Path(__file__).resolve().parent
ROOT = next((path for path in (HERE, *HERE.parents) if (path / "docs" / "data" / "progress.json").is_file()), HERE.parents[1])
ART = ROOT / ".artifacts"
LOGS = ART / "logs"
SCRATCH = ART / "scratch"
COORD = SCRATCH / "coordinator"
BRIEFS = COORD / "briefs"
LISTS = COORD / "lists"
LANE_NAME = re.compile(r"^[A-Za-z0-9][\w-]{0,40}$")
RECENT_SECONDS = 3 * 3600
CODEX_REGISTRY = COORD / "codex_lanes.json"
CODEX_REGISTRY_MAX_AGE_SECONDS = 180
REGISTRY_STATES = {"running", "finished", "died", "stopped", "unknown"}
HISTORY_FILE = ART / "cache" / "dashboard" / "history.jsonl"
HISTORY_EVERY_SECONDS = 60
HISTORY_KEEP = 7 * 24 * 60          # a week of one-minute samples
HISTORY_MAX_POINTS = 360            # per chart; longer ranges keep the last sample of each time bucket
BRIEF_NOTES = HERE.parent / "coordinator" / "briefs" / "brief_notes.json"
NOT_ATTEMPTED = ("not_reached", "not yet run")

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


class FileTime(ctypes.Structure):
    _fields_ = [("low", ctypes.c_ulong), ("high", ctypes.c_ulong)]

    def value(self):
        return (self.high << 32) | self.low


_usage = {"cpu": None, "history": []}  # history rows: [seconds since the epoch, processor %, memory %, commit %]


def system_times():
    idle, kernel, user = FileTime(), FileTime(), FileTime()
    ctypes.windll.kernel32.GetSystemTimes(ctypes.byref(idle), ctypes.byref(kernel), ctypes.byref(user))
    return idle.value(), kernel.value() + user.value()  # kernel time includes idle time


def usage_loop():
    """Processor and memory use, sampled every two seconds; three minutes of history for the page's sparklines."""
    if os.name != "nt":
        return
    idle0, busy0 = system_times()
    while True:
        time.sleep(2)
        idle1, busy1 = system_times()
        total = busy1 - busy0
        cpu = round(100 * (1 - (idle1 - idle0) / total), 1) if total > 0 else None
        idle0, busy0 = idle1, busy1
        m = memory()
        row = [int(time.time()), cpu,
               round(100 * (1 - m["free_gb"] / m["total_gb"]), 1), round(100 * (1 - m["commit_headroom_gb"] / m["commit_limit_gb"]), 1)]
        with _lock:
            _usage["cpu"] = cpu
            _usage["history"] = (_usage["history"] + [row])[-90:]


def refresh_processes():
    """Which lanes have a live agent process, with its start time and reasoning effort (from its command line)."""
    command = ("Get-CimInstance Win32_Process -Filter \"Name like 'muse%'\" | ForEach-Object { "
               "[pscustomobject]@{ pid = $_.ProcessId; started = $_.CreationDate.ToString('s'); cmd = $_.CommandLine; "
               "mem = [int]($_.WorkingSetSize / 1MB) } } | ConvertTo-Json -Compress")
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
            lanes[brief.group(1)] = {"pid": row.get("pid"), "started": row.get("started"), "mem_mb": row.get("mem"),
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


def cached_row(lane, live, experiments, registry_entry=None):
    """A lane's row changes only when its log, results or process state change; reading 170 lanes' files on
    every poll took almost two seconds."""
    def mtime(path):
        try:
            return path.stat().st_mtime
        except OSError:
            return 0
    process = live.get(lane) or {}
    key = (mtime(LOGS / f"{lane}.log"), mtime(SCRATCH / lane / "results.json"), mtime(SCRATCH / lane / "names.json"),
           mtime(SCRATCH / lane / "summary.txt"), process.get("pid"), process.get("mem_mb"),
           tuple(sorted((registry_entry or {}).items())))
    hit = _rows.get(lane)
    if hit and hit[0] == key:
        return hit[1]
    row = lane_row(lane, live, experiments, registry_entry)
    _rows[lane] = (key, row)
    return row


def read_json(path, default=None):
    try:
        return json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, ValueError):
        return default


def read_codex_registry(path=CODEX_REGISTRY, now=None):
    """Read coordinator-owned Codex lane state; a stale or invalid registry never marks a lane running."""
    now = time.time() if now is None else now
    try:
        data = json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, ValueError):
        return {}, {"state": "unknown", "fresh": False, "updated_at": None, "age_seconds": None,
                    "error": "missing or invalid registry"}
    if not isinstance(data, dict) or not isinstance(data.get("lanes"), list):
        return {}, {"state": "unknown", "fresh": False, "updated_at": None, "age_seconds": None,
                    "error": "invalid registry structure"}
    updated_at = data.get("updated_at")
    try:
        updated = datetime.fromisoformat(updated_at.replace("Z", "+00:00"))
        age = now - updated.timestamp()
    except (AttributeError, TypeError, ValueError, OverflowError):
        return {}, {"state": "unknown", "fresh": False, "updated_at": None, "age_seconds": None,
                    "error": "invalid registry timestamp"}
    fresh = 0 <= age <= CODEX_REGISTRY_MAX_AGE_SECONDS
    entries = {}
    for item in data["lanes"]:
        if not isinstance(item, dict):
            continue
        lane = item.get("lane")
        if not isinstance(lane, str) or not LANE_NAME.fullmatch(lane):
            continue
        state = item.get("state")
        if not isinstance(state, str) or state not in REGISTRY_STATES or not fresh:
            state = "unknown"
        entries[lane] = {
            "state": state,
            "model": item.get("model") if isinstance(item.get("model"), str) else None,
            "effort": item.get("effort") if isinstance(item.get("effort"), str) else None,
            "started": item.get("started") if isinstance(item.get("started"), str) else None,
            "prompt": item.get("prompt") if isinstance(item.get("prompt"), str) else None,
            "source": "codex",
        }
    registry = {"state": "fresh" if fresh else ("stale" if age > CODEX_REGISTRY_MAX_AGE_SECONDS else "unknown"),
                "fresh": fresh, "updated_at": updated_at if isinstance(updated_at, str) else None,
                "age_seconds": round(age, 1), "error": None if fresh else "registry is stale or from the future"}
    return entries, registry


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


def result_document(lane):
    data = read_json(SCRATCH / lane / "results.json")
    return data if isinstance(data, dict) else {}


def display_value(value, limit=400):
    """Convert nested result values to readable JSON before they reach JavaScript."""
    if value is None:
        return ""
    if isinstance(value, str):
        text = value
    elif isinstance(value, (dict, list, tuple)):
        text = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"), default=str)
    else:
        text = str(value)
    return text[:limit]


def display_note(value, limit=360):
    """Keep list-valued limitations readable and bounded in the detail table."""
    if isinstance(value, list):
        parts = [display_value(item, 150) for item in value[:2] if item]
        text = "; ".join(parts)
        if len(value) > 2:
            text += f"; +{len(value) - 2} more"
        return text[:limit]
    return display_value(value, limit)


def result_rows(lane):
    rows = read_json(SCRATCH / lane / "results.json")
    if isinstance(rows, dict):
        rows = rows.get("functions") or rows.get("results") or rows.get("names")
    return rows if isinstance(rows, list) else None


def _contract_names(value):
    if not isinstance(value, list):
        return []
    names = []
    for item in value:
        name = item if isinstance(item, str) else (item.get("contract_name") or item.get("name") or item.get("contract_file")) if isinstance(item, dict) else None
        if name:
            name = display_value(name)
            if name not in names:
                names.append(name)
    return names


def accepted_contracts(data):
    """Return one explicit candidate acceptance list; never add inherited and current lists together."""
    for owner in (data.get("candidate"), data.get("final_scope_audit"), data):
        if isinstance(owner, dict):
            names = _contract_names(owner.get("accepted_contracts") or owner.get("branch_scoped_accepted_contracts"))
            if names:
                return names
    return []


def accepted_contract_label(count):
    noun = "contract" if count == 1 else "contracts"
    return f"{count} branch-scoped {noun} accepted"


def full_function_verified(data):
    """Read only explicit whole-function fields; stage and branch contracts stay partial."""
    for owner in (data, data.get("candidate"), data.get("function")):
        if not isinstance(owner, dict):
            continue
        value = owner.get("full_function_verification")
        if isinstance(value, bool):
            return value
        status = owner.get("full_function_status")
        if isinstance(status, str):
            normalized = status.casefold()
            if normalized in {"verified", "verified_full", "full_function_verified"}:
                return True
            if any(word in normalized for word in ("partial", "bounded", "unverified", "not_verified")):
                return False
    proof_scope = data.get("proof_scope")
    if isinstance(proof_scope, dict) and proof_scope.get("partial_only") is True:
        return False
    return None


def evidence_scope(data):
    function = data.get("function") if isinstance(data.get("function"), dict) else {}
    candidate = data.get("candidate") if isinstance(data.get("candidate"), dict) else {}
    text = data.get("scope") or function.get("scope") or candidate.get("verification_status") or data.get("verification_status")
    if not text:
        proof_scope = data.get("proof_scope")
        if isinstance(proof_scope, dict):
            text = proof_scope.get("candidate_contract_scope")
    text = display_value(text, 360)
    if full_function_verified(data) is False and "full-function verification" not in text.casefold():
        text = (text + "; " if text else "") + "full-function verification is not established"
    return text


def _is_full_verified_outcome(outcome):
    value = str(outcome or "").casefold()
    return value.startswith("verified") and not any(word in value for word in ("partial", "bounded", "branch", "stage"))


def yield_of(lane):
    data = result_document(lane)
    stages = data.get("stages")
    if isinstance(stages, dict) and stages:
        records = [stage for stage in stages.values() if isinstance(stage, dict)]
        full = sum(stage.get("status") == "passed_full" for stage in records)
        probes = sum(stage.get("status") == "passed_probe" for stage in records)
        failed = sum("fail" in str(stage.get("status") or "").casefold() for stage in records)
        other = len(records) - full - probes - failed
        pieces = [f"{full} full-stage check{'s' if full != 1 else ''} passed",
                  f"{probes} probe check{'s' if probes != 1 else ''} passed"]
        if failed:
            pieces.append(f"{failed} stage check{'s' if failed != 1 else ''} failed")
        if other:
            pieces.append(f"{other} with unclassified status")
        if full_function_verified(data) is False:
            pieces.append("function remains partial")
        return {"done": full + probes, "of": len(records), "unit": "stage checks passed",
                "display": "; ".join(pieces), "status": display_value(data.get("status")), "scope": evidence_scope(data),
                "verified_functions": 0}

    rows = result_rows(lane)
    if rows is not None:
        rows = [row for row in rows if isinstance(row, dict)]
        if rows and any("outcome" in row for row in rows):
            if lane.startswith("a-G"):
                checked = sum(row.get("outcome") != "not_reached" for row in rows)
                return {"done": checked, "of": len(rows), "unit": "checked", "display": f"{checked} of {len(rows)} checked"}
            full_scope = full_function_verified(data)
            verified = sum(_is_full_verified_outcome(row.get("outcome")) for row in rows) if full_scope is not False else 0
            bounded = sum(any(word in str(row.get("outcome") or "").casefold() for word in ("partial", "bounded", "branch", "stage")) for row in rows)
            display = f"{verified} of {len(rows)} verified"
            if full_scope is False:
                display += "; full-function verification not established"
            if bounded:
                display += f"; {bounded} bounded partial"
            return {"done": verified, "of": len(rows), "unit": "verified", "display": display,
                    "verified_functions": verified, "partial_functions": bounded}

        partial = sum(any(word in str(row.get("status") or "").casefold() for word in ("partial", "partly_proven")) for row in rows)
        contracts = accepted_contracts(data)
        if partial or full_function_verified(data) is False:
            display = f"{partial} of {len(rows)} functions have bounded partial evidence"
            if contracts:
                display += "; " + accepted_contract_label(len(contracts))
            display += "; full-function verification not established"
            return {"done": partial, "of": len(rows), "unit": "partly proven functions", "display": display,
                    "status": display_value(data.get("status")), "scope": evidence_scope(data),
                    "verified_functions": 0, "partial_functions": partial,
                    "accepted_contracts": len(contracts)}

    contracts = accepted_contracts(data)
    if contracts:
        display = accepted_contract_label(len(contracts)) + "; full-function verification not established"
        return {"done": len(contracts), "of": len(contracts), "unit": "branch-scoped contracts accepted",
                "display": display, "status": display_value(data.get("status")), "scope": evidence_scope(data),
                "verified_functions": 0, "accepted_contracts": len(contracts)}

    if rows is None:
        names = read_json(SCRATCH / lane / "names.json")
        if isinstance(names, list):
            named = sum(1 for row in names if isinstance(row, dict) and row.get("name"))
            return {"done": named, "of": len(names), "unit": "named", "display": f"{named} of {len(names)} named"}
    return None


def result_detail_rows(lane):
    data = result_document(lane)
    rows = result_rows(lane)
    if rows is not None:
        if any(isinstance(row, dict) and row.get("status") and not row.get("outcome") for row in rows):
            return rows, "Function evidence", "Function"
        return rows, "Results", "Function"

    stages = data.get("stages")
    if isinstance(stages, dict):
        rows = []
        for name, stage in stages.items():
            if not isinstance(stage, dict):
                continue
            mutant = stage.get("mutant")
            mutant_fails = stage.get("mutant_fails")
            explicit_mutant_failure = ((isinstance(mutant_fails, int) and not isinstance(mutant_fails, bool) and mutant_fails > 0)
                                       or (isinstance(mutant, str) and mutant.casefold().strip().startswith("fail"))
                                       or (isinstance(mutant, dict) and mutant.get("passed") is False
                                           and isinstance(mutant.get("fails"), int) and mutant.get("fails") > 0))
            caught = stage.get("status") in {"passed_probe", "passed_full"} and explicit_mutant_failure
            detail = "; ".join(value for value in ("correct: " + display_value(stage.get("correct")) if stage.get("correct") else "",
                                                       "mutant: " + display_value(mutant) if mutant else "") if value)
            rows.append({"name": name, "status": stage.get("status"), "attempts": stage.get("trials"),
                         "mutant_caught": caught, "detail": detail})
        return rows, "Stage checks", "Stage"

    contracts = accepted_contracts(data)
    if contracts:
        rows = [{"name": name, "outcome": "accepted (branch-scoped)",
                 "detail": "Accepted contract on this candidate; full-function verification is not established."}
                for name in contracts]
        return rows, "Accepted branch-scoped contracts", "Contract"
    return None, None, None


def initial_prompt(lane, registry_entry=None):
    """Prefer saved task text and registry assignment; never label a generic brief as the initial prompt."""
    candidates = ((SCRATCH / lane / "prompt.txt", "lane prompt.txt"),
                  (COORD / "prompts" / f"{lane}.txt", "coordinator prompt file"))
    for path, source in candidates:
        try:
            prompt = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if prompt.strip() and prompt.strip().casefold() not in {"no prompt recorded", "no initial prompt recorded", "not recorded", "unknown", "none", "n/a"}:
            return prompt, source
    prompt = (registry_entry or {}).get("prompt")
    if isinstance(prompt, str) and prompt.strip() and prompt.strip().casefold() not in {"no prompt recorded", "no initial prompt recorded", "not recorded", "unknown", "none", "n/a"}:
        return prompt, "registry assignment"
    return None, None

def brief_tag(lane):
    path = BRIEFS / f"{lane}.txt"
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return None
    match = re.search(r"`brief` \(write `([\w.-]+)`\)", text)
    return match.group(1) if match else None


def lane_row(lane, live, experiments, registry_entry=None):
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
    if registry_entry is not None:
        state = registry_entry["state"]
    elif process:
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
            "model": (registry_entry or {}).get("model") or (process or {}).get("model"),
            "effort": (registry_entry or {}).get("effort") or (process or {}).get("effort") or (experiment or {}).get("effort"),
            "source": (registry_entry or {}).get("source") or "muse",
            "started": (registry_entry or {}).get("started") or (process or {}).get("started"),
            "agent_memory_mb": None if registry_entry is not None else (process or {}).get("mem_mb"),
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
    registry_lanes, registry_status = read_codex_registry(now=time.time())
    experiments = {row["lane"]: row for row in (read_json(COORD / "experiments.json", []) or []) if not row.get("never_ran")}
    now = time.time()
    names = set(live) | set(registry_lanes)
    try:
        for entry in os.scandir(LOGS):
            if entry.name.endswith(".log") and now - entry.stat().st_mtime < RECENT_SECONDS:
                name = entry.name[:-4]
                if LANE_NAME.match(name) and name != "supervisor":
                    names.add(name)
    except OSError:
        pass
    lanes = [cached_row(name, live, experiments, registry_lanes.get(name)) for name in sorted(names)]
    order = {"running": 0, "died": 1, "stopped": 2, "unknown": 3, "finished": 4}
    lanes.sort(key=lambda row: (order[row["state"]], row["last_activity"] or ""), reverse=False)
    progress = read_json(ROOT / "docs" / "data" / "progress.json", {}) or {}
    history = read_json(COORD / "review_history.json", []) or []
    with _lock:
        cpu, usage_history = _usage["cpu"], list(_usage["history"])
    return {"now": datetime.now().isoformat(timespec="seconds"), "memory": memory(),
            "cpu_percent": cpu, "cores": os.cpu_count(), "usage_history": usage_history,
            "process_list_age_s": round(process_age, 1) if process_age is not None else None, "process_list_error": process_error,
            "functions": progress.get("functions"), "stages": progress.get("stages"), "phase": progress.get("phase"),
            "progress_updated": progress.get("updated_at") or progress.get("updated"),
            "review": history[-1] if history else None, "codex_registry": registry_status,
            "supervisor": {"settings": read_json(COORD / "supervisor.json"), "log": tail(LOGS / "supervisor.log", 6),
                           "stop_file": (COORD / "STOP").exists()},
            "lanes": lanes}


def lane_detail(lane):
    if not LANE_NAME.match(lane):
        return None
    registry_lanes, _ = read_codex_registry()
    registry_entry = registry_lanes.get(lane) or {}
    prompt, prompt_source = initial_prompt(lane, registry_entry)
    summary = SCRATCH / lane / "summary.txt"
    try:
        summary_text = summary.read_text(encoding="utf-8", errors="replace")
    except OSError:
        summary_text = None
    rows, results_label, result_item_label = result_detail_rows(lane)
    slim = None
    if rows:
        slim = []
        for row in rows[:400]:
            if not isinstance(row, dict):
                continue
            attempts = row.get("attempts")
            attempt_count = len(attempts) if isinstance(attempts, list) else attempts
            note = row.get("detail") or row.get("evidence") or row.get("scope") or row.get("limitations") or ""
            if isinstance(attempts, list):
                accepted = set()
                for attempt in attempts:
                    if isinstance(attempt, dict) and attempt.get("accepted") is True:
                        name = attempt.get("contract_name") or attempt.get("contract_file")
                        if name:
                            accepted.add(str(name))
                if accepted:
                    note = (display_note(note, 300) + "; " if note else "") + f"{len(accepted)} accepted branch-scoped contracts"
            slim.append({"address": display_value(row.get("address")) or None,
                         "name": display_value(row.get("name") or row.get("item")) or None,
                         "size": display_value(row.get("size")) or None,
                         "outcome": display_value(row.get("outcome") or row.get("status") or row.get("confidence")) or None,
                         "reason": display_value(row.get("reason")) or None,
                         "attempts": attempt_count if isinstance(attempt_count, (int, float)) else display_value(attempt_count) or None,
                         "mutant_caught": row.get("mutant_caught") if isinstance(row.get("mutant_caught"), bool) else None,
                         "confidence": display_value(row.get("confidence")) or None,
                         "detail": display_note(note)[:400]})
    result = yield_of(lane)
    return {"lane": lane, "model": registry_entry.get("model"), "effort": registry_entry.get("effort"),
            "prompt": prompt, "prompt_source": prompt_source, "prompt_chars": len(prompt) if prompt else 0,
            "log": tail(LOGS / f"{lane}.log", 80), "summary": summary_text, "results": slim,
            "results_label": results_label, "result_item_label": result_item_label,
            "results_total": len(rows) if isinstance(rows, list) else 0, "yield": result,
            "batch": read_json(LISTS / f"{lane}.json")}


# ---------------------------------------------------------------------------------------------------------
# History: one sample a minute (lanes running, verified, memory), kept in memory and in a rolling file.
# ---------------------------------------------------------------------------------------------------------

_history = []  # rows as written to HISTORY_FILE, oldest first


def history_sample(now, lanes, progress, mem):
    """One history row. `lanes` is the live-lane map, `progress` the site's progress data, `mem` memory() or None."""
    stages = (progress or {}).get("stages") or {}
    memory_unknown = any(p.get("source") == "codex" for p in lanes.values())
    row = {"t": int(now), "lanes": len(lanes), "verified": stages.get("verified"), "rewritten": stages.get("rewritten"),
           "agent_gb": None if memory_unknown else round(sum((p.get("mem_mb") or 0) for p in lanes.values()) / 1024, 2),
           "mem_pct": None, "commit_pct": None}
    if mem and mem.get("total_gb") and mem.get("commit_limit_gb"):
        row["mem_pct"] = round(100 * (1 - mem["free_gb"] / mem["total_gb"]), 1)
        row["commit_pct"] = round(100 * (1 - mem["commit_headroom_gb"] / mem["commit_limit_gb"]), 1)
    return row


def load_history(path, keep=HISTORY_KEEP):
    """The last `keep` rows of the history file; unreadable lines are skipped, a missing file is empty."""
    rows = []
    try:
        with open(path, encoding="utf-8") as handle:
            for line in handle:
                try:
                    row = json.loads(line)
                except ValueError:
                    continue
                if isinstance(row, dict) and isinstance(row.get("t"), (int, float)):
                    rows.append(row)
    except OSError:
        return []
    return rows[-keep:]


def append_history(path, row, rows, keep=HISTORY_KEEP):
    """Add a row in memory and on disk. The file is rewritten with the last `keep` rows once it holds a quarter
    more than that, so it stays about a week long without being rewritten every minute."""
    rows.append(row)
    path.parent.mkdir(parents=True, exist_ok=True)
    if len(rows) > keep * 5 // 4:
        del rows[:-keep]
        tmp = path.with_suffix(".tmp")
        tmp.write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")
        os.replace(tmp, path)
    else:
        with open(path, "a", encoding="utf-8") as handle:
            handle.write(json.dumps(row) + "\n")


def review_points(history):
    """(epoch seconds, verified) from the hourly review snapshots (local times written as YYYY-MM-DD HH:MM)."""
    points = []
    for snapshot in history or []:
        try:
            when = datetime.strptime(snapshot["at"], "%Y-%m-%d %H:%M").timestamp()
        except (KeyError, TypeError, ValueError):
            continue
        if isinstance(snapshot.get("verified"), int):
            points.append([int(when), snapshot["verified"]])
    return sorted(points)


def thin(points, since, until, max_points=HISTORY_MAX_POINTS):
    """Points inside [since, until], at most `max_points`: the range is cut into equal buckets and the last point
    of each bucket kept, so a count (verified) keeps its true latest value and nothing is averaged into a
    value that never occurred."""
    inside = [p for p in points if since <= p[0] <= until]
    if len(inside) <= max_points:
        return inside
    width = (until - since) / max_points
    kept = {}
    for point in inside:
        kept[min(int((point[0] - since) / width), max_points - 1)] = point
    return [kept[k] for k in sorted(kept)]


def compose_history(rows, reviews, since, until, max_points=HISTORY_MAX_POINTS):
    """The page's series over [since, until]: each a list of [epoch seconds, value]. Verified adds the review
    snapshots from before the first sample, so its line reaches back past the dashboard's own start."""
    def series(key):
        return thin([[r["t"], r[key]] for r in rows if r.get(key) is not None], since, until, max_points)
    first = rows[0]["t"] if rows else until + 1
    verified = [p for p in reviews if p[0] < first] + [[r["t"], r["verified"]] for r in rows if r.get("verified") is not None]
    return {"since": since, "until": until, "first_sample": rows[0]["t"] if rows else None,
            "lanes": series("lanes"), "verified": thin(sorted(verified), since, until, max_points),
            "memory": series("mem_pct"), "commit": series("commit_pct"), "agent_gb": series("agent_gb")}


def history_lanes(muse_lanes, codex_lanes, registry_fresh):
    """Merge explicit fresh Codex activity with Muse processes without assigning Codex memory."""
    lanes = dict(muse_lanes)
    if registry_fresh:
        for lane, entry in codex_lanes.items():
            if entry.get("state") == "running":
                lanes.setdefault(lane, entry)
    return lanes


def history_loop():
    """Sample once a minute for as long as the server runs."""
    with _lock:
        _history[:] = load_history(HISTORY_FILE)
    while True:
        deadline = time.time() + 45
        while not _processes["at"] and time.time() < deadline:
            time.sleep(1)  # the first sample waits for the first process list
        with _lock:
            lanes = dict(_processes["lanes"])
        registry_lanes, registry_status = read_codex_registry(now=time.time())
        lanes = history_lanes(lanes, registry_lanes, registry_status["fresh"])
        row = history_sample(time.time(), lanes, read_json(ROOT / "docs" / "data" / "progress.json", {}), memory())
        try:
            with _lock:
                append_history(HISTORY_FILE, row, _history)
        except OSError:
            pass  # a full or locked disk must not stop the page
        time.sleep(HISTORY_EVERY_SECONDS)


def history(hours):
    hours = max(1, min(int(hours), HISTORY_KEEP // 60))
    until = int(time.time())
    with _lock:
        rows = list(_history)
    reviews = review_points(read_json(COORD / "review_history.json", []))
    return dict(compose_history(rows, reviews, until - hours * 3600, until), hours=hours)


# ---------------------------------------------------------------------------------------------------------
# Experiments: pass rate and time per function for each brief version, from lanes' results.json.
# ---------------------------------------------------------------------------------------------------------

def natural_key(text):
    return [int(part) if part.isdigit() else part for part in re.split(r"(\d+)", str(text))]


def experiment_table(lanes, experiments, brief_of_lane=lambda lane: None, current=None):
    """Rows grouped by (brief version, lane kind) from (lane, rows) pairs.

    A row's brief version is its `brief` field (or `brief_version`), else the experiment tag recorded for its
    lane in experiments.json, else the tag written into the lane's brief file (`brief_of_lane`), else "not
    recorded". Attempted excludes not_reached and not yet run. Pass rate is verified over attempted. Minutes:
    the median of verified rows' `minutes`, and all recorded minutes divided by the verified count (what one
    verified function cost, failures included).
    """
    groups = {}
    for lane, rows in lanes:
        experiment = experiments.get(lane) or {}
        fallback = experiment.get("tag") or brief_of_lane(lane)
        for row in rows:
            if not isinstance(row, dict) or "outcome" not in row:
                continue
            brief = str(row.get("brief") or row.get("brief_version") or fallback or "not recorded")
            group = groups.setdefault((brief, kind_of(lane)), {"lanes": set(), "functions": 0, "attempted": 0, "verified": 0,
                                                                "deferred": 0, "verified_minutes": [], "minutes": 0.0})
            outcome = str(row.get("outcome"))
            minutes = row.get("minutes") if isinstance(row.get("minutes"), (int, float)) and not isinstance(row.get("minutes"), bool) else None
            group["lanes"].add(lane)
            group["functions"] += 1
            group["attempted"] += outcome not in NOT_ATTEMPTED
            group["deferred"] += outcome == "deferred"
            if minutes is not None:
                group["minutes"] += minutes
            if outcome.startswith("verified"):
                group["verified"] += 1
                if minutes is not None:
                    group["verified_minutes"].append(minutes)
    factors = {}
    for row in experiments.values():
        if row.get("tag"):
            factors.setdefault(row["tag"], row.get("factor"))
    table = []
    for (brief, kind), g in sorted(groups.items(), key=lambda item: (natural_key(item[0][0]), item[0][1])):
        mins = sorted(g["verified_minutes"])
        median = (mins[len(mins) // 2] if len(mins) % 2 else (mins[len(mins) // 2 - 1] + mins[len(mins) // 2]) / 2) if mins else None
        table.append({"brief": brief, "kind": kind, "factor": factors.get(brief), "current": brief == current,
                      "lanes": len(g["lanes"]), "functions": g["functions"], "attempted": g["attempted"],
                      "verified": g["verified"], "deferred": g["deferred"],
                      "pass_rate": round(g["verified"] / g["attempted"], 3) if g["attempted"] else None,
                      "median_minutes": round(median, 1) if median is not None else None,
                      "minutes_per_verified": round(g["minutes"] / g["verified"], 1) if g["verified"] and g["minutes"] else None})
    return table


_parsed = {}  # results path -> (mtime, rows)
_experiments = {"key": None, "value": None}


def experiments_state():
    """The experiment table, recomputed only when a results file or experiments.json changed."""
    paths = sorted(SCRATCH.glob("r-*/results.json")) + sorted(SCRATCH.glob("a-*/results.json"))
    stamps = []
    for path in paths:
        try:
            stamps.append((str(path), path.stat().st_mtime))
        except OSError:
            continue
    exp_path = COORD / "experiments.json"
    key = (tuple(stamps), exp_path.stat().st_mtime if exp_path.exists() else 0)
    if _experiments["key"] == key:
        return _experiments["value"]
    lanes = []
    for name, mtime in stamps:
        hit = _parsed.get(name)
        if not hit or hit[0] != mtime:
            rows = read_json(Path(name))
            if isinstance(rows, dict):
                rows = rows.get("functions") or rows.get("results")
            hit = (mtime, rows if isinstance(rows, list) else [])
            _parsed[name] = hit
        lanes.append((Path(name).parent.name, hit[1]))
    experiments = {row["lane"]: row for row in (read_json(exp_path, []) or []) if isinstance(row, dict) and row.get("lane")}
    current = (read_json(BRIEF_NOTES, {}) or {}).get("version")
    value = {"current_brief": current, "lanes": len(lanes),
             "rows": experiment_table(lanes, experiments, brief_tag, current)}
    _experiments.update(key=key, value=value)
    return value


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
        parts = urlsplit(self.path)
        path = parts.path
        try:
            if path in ("/", "/index.html"):
                self.send((HERE / "index.html").read_bytes(), "text/html; charset=utf-8")
            elif path == "/api/state":
                self.send(json.dumps(state()))
            elif path == "/api/history":
                hours = (parse_qs(parts.query).get("hours") or ["24"])[0]
                self.send(json.dumps(history(int(hours) if hours.isdigit() else 24)))
            elif path == "/api/experiments":
                self.send(json.dumps(experiments_state()))
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
    threading.Thread(target=usage_loop, daemon=True).start()
    threading.Thread(target=history_loop, daemon=True).start()
    print(f"LibertyFlux dashboard on http://127.0.0.1:{port}/ (local only)", flush=True)
    ThreadingHTTPServer(("127.0.0.1", port), Handler).serve_forever()
