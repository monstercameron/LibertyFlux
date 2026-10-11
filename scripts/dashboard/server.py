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
import math
import os
import re
import subprocess
import sys
import threading
import time
from datetime import datetime
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path, PurePosixPath, PureWindowsPath
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
    # A Muse lane is found by the brief its process was given; a Luna lane started with the Codex command line by
    # the file its last message goes to, which is in the lane's scratch folder (since 10 October 2026).
    command = ("Get-CimInstance Win32_Process | Where-Object { $_.Name -like 'muse*' -or "
               "($_.Name -eq 'codex.exe' -and $_.CommandLine -like '*codex-last-message.txt*') } | ForEach-Object { "
               "[pscustomobject]@{ pid = $_.ProcessId; started = $_.CreationDate.ToString('s'); cmd = $_.CommandLine; "
               "mem = [int]($_.WorkingSetSize / 1MB) } } | ConvertTo-Json -Compress")
    try:
        out = subprocess.run(["powershell", "-NoProfile", "-Command", command], capture_output=True, text=True, timeout=40).stdout.strip()
        rows = json.loads(out) if out else []
        rows = rows if isinstance(rows, list) else [rows]
        lanes = {}
        for row in rows:
            cmd = row.get("cmd") or ""
            codex = re.search(r"scratch[\\/]([\w-]+)[\\/]codex-last-message\.txt", cmd)
            brief = re.search(r"briefs[\\/]([\w-]+)\.txt", cmd) or codex
            if not brief:
                continue
            effort = re.search(r"(?:--reasoning-effort\s+|model_reasoning_effort=)\"?(\w+)", cmd)
            model = re.search(r"--model\s+(\S+)", cmd)
            lanes[brief.group(1)] = {"pid": row.get("pid"), "started": row.get("started"), "mem_mb": row.get("mem"),
                                     "effort": effort.group(1) if effort else None, "model": model.group(1) if model else None,
                                     "source": "codex-exec" if codex else "muse"}
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
        if path is None:
            return 0
        try:
            return path.stat().st_mtime
        except (OSError, TypeError):
            return 0
    def file_key(path):
        return (str(path) if path is not None else None, mtime(path))
    process = live.get(lane) or {}
    key = (file_key(lane_file_path(lane, "log", registry_entry)),
           file_key(lane_file_path(lane, "results", registry_entry)), file_key(lane_default_path(lane, "names.json")),
           file_key(lane_file_path(lane, "summary", registry_entry)),
           file_key(lane_file_path(lane, "prompt", registry_entry)), process.get("pid"), process.get("mem_mb"),
           tuple(sorted((registry_entry or {}).items())))
    hit = _rows.get(lane)
    if hit and hit[0] == key:
        return hit[1]
    row = lane_row(lane, live, experiments, registry_entry)
    _rows[lane] = (key, row)
    return row


def read_json(path, default=None):
    if path is None:
        return default
    try:
        return json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, ValueError, TypeError):
        return default


def registry_path_value(item, key):
    value = item.get(key)
    return value if isinstance(value, str) and len(value) <= 4096 else None


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
            "prompt_path": registry_path_value(item, "prompt_path"),
            "results_path": registry_path_value(item, "results_path"),
            "log_path": registry_path_value(item, "log_path"),
            "summary_path": registry_path_value(item, "summary_path"),
            # A registry lane is a Codex subagent unless its entry says it is a Claude subagent (10 October 2026).
            "source": "claude" if item.get("source") == "claude" else "codex",
        }
    registry = {"state": "fresh" if fresh else ("stale" if age > CODEX_REGISTRY_MAX_AGE_SECONDS else "unknown"),
                "fresh": fresh, "updated_at": updated_at if isinstance(updated_at, str) else None,
                "age_seconds": round(age, 1), "error": None if fresh else "registry is stale or from the future"}
    return entries, registry


def _inside(path, root):
    try:
        return bool(path.relative_to(root).parts)
    except (ValueError, OSError):
        return False


def _lane_roots(lane):
    """Resolve the configured scratch/log roots and reject redirected lane directories."""
    if not isinstance(lane, str) or not LANE_NAME.fullmatch(lane):
        return None, None, None
    try:
        artifact_root = ART.resolve(strict=False)
        scratch_root = SCRATCH.resolve(strict=False)
        logs_root = LOGS.resolve(strict=False)
        lane_root = (SCRATCH / lane).resolve(strict=False)
    except (OSError, RuntimeError):
        return None, None, None
    if scratch_root.parent != artifact_root or scratch_root.name.casefold() != "scratch":
        return None, None, None
    if logs_root.parent != artifact_root or logs_root.name.casefold() != "logs":
        return None, None, None
    if lane_root.parent != scratch_root or lane_root.name.casefold() != lane.casefold():
        return None, None, None
    return lane_root, scratch_root, logs_root


def lane_default_path(lane, filename):
    """Return a legacy lane file only if its resolved target remains inside that lane's scratch folder."""
    lane_root, _, _ = _lane_roots(lane)
    if lane_root is None or not isinstance(filename, str) or Path(filename).name != filename:
        return None
    try:
        path = (SCRATCH / lane / filename).resolve(strict=False)
    except (OSError, RuntimeError):
        return None
    return path if _inside(path, lane_root) else None


def lane_file_path(lane, kind, registry_entry=None):
    """Use a registry path only when it is an existing lane-owned file; otherwise keep the legacy path."""
    fields = {"prompt": ("prompt_path", "prompt.txt", ".txt"),
              "results": ("results_path", "results.json", ".json"),
              "summary": ("summary_path", "summary.txt", ".txt"),
              "log": ("log_path", f"{lane}.log", ".log")}
    spec = fields.get(kind)
    if spec is None:
        return None
    field, default_name, suffix = spec
    lane_root, _, logs_root = _lane_roots(lane)
    if lane_root is None:
        return None
    is_log = kind == "log"
    allowed_root = logs_root if is_log else lane_root
    raw = (registry_entry or {}).get(field)
    if isinstance(raw, str) and raw and len(raw) <= 4096 and raw.strip() == raw:
        # Registry paths are repo-relative POSIX paths. Reject traversal and Windows/UNC paths
        # before resolving symlinks, then validate the resolved target against its owner root.
        parts = raw.split("/")
        windows_path = PureWindowsPath(raw)
        if (not raw.startswith("/") and "\\" not in raw and "\x00" not in raw
                and not windows_path.is_absolute() and not windows_path.drive
                and all(part not in {"", ".", ".."} for part in parts)):
            try:
                candidate = ROOT.joinpath(*PurePosixPath(raw).parts).resolve(strict=True)
            except (OSError, RuntimeError):
                candidate = None
            if candidate is not None and candidate.is_file() and candidate.suffix.casefold() == suffix:
                if is_log:
                    owned_name = candidate.name == f"{lane}.log" or candidate.name.startswith((f"{lane}-", f"{lane}_"))
                    owned = candidate.parent == allowed_root and owned_name
                else:
                    owned = _inside(candidate, allowed_root)
                if owned:
                    return candidate

    if is_log:
        try:
            default_path = (LOGS / default_name).resolve(strict=False)
        except (OSError, RuntimeError):
            return None
        owned = default_path.parent == logs_root and default_path.name == f"{lane}.log"
    else:
        default_path = lane_default_path(lane, default_name)
        if default_path is None:
            return None
        owned = _inside(default_path, lane_root)
    return default_path if owned else None


def registered_lane_path(lane, kind, registry_entry=None):
    """Return a registry-routed file only after the same lane ownership checks as lane_file_path."""
    fields = {"prompt": ("prompt_path", ".txt"), "results": ("results_path", ".json"),
              "summary": ("summary_path", ".txt")}
    spec = fields.get(kind)
    if spec is None or not isinstance(registry_entry, dict):
        return None
    field, suffix = spec
    raw = registry_path_value(registry_entry, field)
    if not raw:
        return None
    parts = raw.split("/")
    windows_path = PureWindowsPath(raw)
    if (raw.startswith("/") or "\\" in raw or "\x00" in raw
            or windows_path.is_absolute() or windows_path.drive
            or any(part in {"", ".", ".."} for part in parts)):
        return None
    # Keep even metadata resolution inside the named lane's lexical root. lane_file_path then
    # resolves symlinks and rejects any target that escaped that root.
    expected_prefix = f".artifacts/scratch/{lane}/"
    if not raw.startswith(expected_prefix):
        return None
    try:
        candidate = ROOT.joinpath(*PurePosixPath(raw).parts).resolve(strict=True)
    except (OSError, RuntimeError):
        return None
    if not candidate.is_file() or candidate.suffix.casefold() != suffix:
        return None
    routed = lane_file_path(lane, kind, registry_entry)
    return candidate if routed == candidate else None


def has_current_task_context(lane, registry_entry=None):
    """Current prompt/results routing means a lane-root fallback summary is historical."""
    return any(registered_lane_path(lane, kind, registry_entry) is not None
               for kind in ("prompt", "results"))


def tail(path, lines=1, limit=64 * 1024):
    if path is None:
        return []
    try:
        with open(path, "rb") as handle:
            handle.seek(0, os.SEEK_END)
            size = handle.tell()
            handle.seek(max(0, size - limit))
            text = handle.read().decode("utf-8", errors="replace")
    except (OSError, TypeError):
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


def result_document(lane, registry_entry=None):
    data = read_json(lane_file_path(lane, "results", registry_entry))
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


def result_rows(lane, registry_entry=None):
    rows = read_json(lane_file_path(lane, "results", registry_entry))
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


def _count(value):
    return value if isinstance(value, int) and not isinstance(value, bool) and value >= 0 else None


def _first_count(record, *keys):
    if not isinstance(record, dict):
        return None
    for key in keys:
        value = _count(record.get(key))
        if value is not None:
            return value
    return None


def _first_bool(record, *keys):
    if not isinstance(record, dict):
        return None
    for key in keys:
        value = record.get(key)
        if isinstance(value, bool):
            return value
    return None


def _proof_metrics(positive, negative=None):
    """Extract structured trial evidence. A mutant only counts as caught after a clean positive run."""
    positive = positive if isinstance(positive, dict) else {}
    negative = negative if isinstance(negative, dict) else positive
    positive_trials = _first_count(positive, "positive_trials", "correct_trials", "trials")
    positive_fails = _first_count(positive, "positive_fails", "correct_fails", "fails")
    original_trials = _first_count(positive, "orig_ok_trials")
    positive_passed = _first_bool(positive, "positive_passed", "correct_pass", "passed")
    positive_ok = (positive_passed is True and positive_trials is not None and positive_trials > 0
                   and positive_fails == 0 and original_trials == positive_trials
                   and positive.get("vacuous") is False)

    mutant_trials = _first_count(negative, "mutant_trials", "mut_trials", "trials")
    mutant_fails = _first_count(negative, "mutant_fails", "mut_fails", "failed_trials")
    mutant_flag = _first_bool(negative, "mutant_caught")
    has_mutant_evidence = (mutant_flag is not None or mutant_trials is not None or mutant_fails is not None)
    if not has_mutant_evidence:
        mutant_caught = None
    elif mutant_flag is False:
        mutant_caught = False
    else:
        mutant_caught = bool(positive_ok and mutant_trials is not None and mutant_trials > 0
                             and mutant_fails is not None and mutant_fails > 0)
    return {"positive_trials": positive_trials, "positive_fails": positive_fails,
            "original_trials": original_trials, "positive_passed": positive_passed,
            "positive_ok": positive_ok, "mutant_trials": mutant_trials,
            "mutant_fails": mutant_fails, "mutant_caught": mutant_caught}


def _evidence_row(label, outcome, metrics=None, detail=""):
    metrics = metrics or {}
    return {"name": display_value(label, 120), "outcome": display_value(outcome, 120),
            "positive_trials": metrics.get("positive_trials"), "positive_fails": metrics.get("positive_fails"),
            "original_trials": metrics.get("original_trials"), "mutant_trials": metrics.get("mutant_trials"),
            "mutant_fails": metrics.get("mutant_fails"), "mutant_caught": metrics.get("mutant_caught"),
            "full_function_verified": False, "detail": display_value(detail, 260)}


def _accepted_decision(value):
    if not isinstance(value, str):
        return False
    first = value.casefold().strip().split(";", 1)[0].strip()
    return first == "accept" or first.startswith("accept_") or first.startswith("accept ")



def _task_activity_rows(data, max_items=12):
    """Expose bounded task notes without turning them into proof or progress credit."""
    rows = []
    for field, category, base_name in (
        ("preliminary_findings", "Finding", "Finding"),
        ("recommendation", "Recommendation", "Recommendation"),
    ):
        value = data.get(field)
        if value is None or value == "" or value == [] or value == {}:
            continue
        items = value if isinstance(value, list) else [value]
        for index, item in enumerate(items[:max_items], 1):
            if isinstance(item, dict):
                name = next((item.get(key) for key in ("id", "name", "title", "finding")
                             if isinstance(item.get(key), str) and item.get(key).strip()), None)
                outcome = next((item.get(key) for key in ("severity", "confidence", "status", "type")
                                if isinstance(item.get(key), str) and item.get(key).strip()), "recorded")
                detail_parts = []
                for key in ("summary", "evidence", "effect", "detail", "scope", "limitations",
                            "text", "description", "decision", "action", "recommendation"):
                    part = item.get(key)
                    if part is not None and part != "" and part != [] and part != {}:
                        detail_parts.append(f"{key}: {display_note(part, 220)}")
                detail = "; ".join(detail_parts) if detail_parts else display_note(item, 260)
                name = display_value(name or f"{base_name} {index}", 120)
                outcome = display_value(outcome, 120)
            else:
                name = f"{base_name} {index}" if isinstance(value, list) else base_name
                outcome = "recorded"
                detail = display_note(item, 260)
            rows.append({"category": category, "name": name, "outcome": outcome,
                         "detail": display_value(detail, 260)})
        if len(items) > max_items:
            rows.append({"category": category, "name": "Additional items", "outcome": "omitted",
                         "detail": "Further items are omitted from this dashboard view; full content remains in results.json."})
    return rows


def ended_recently(lane_state, activity_epoch, now=None):
    """Use only finite, known past activity times for the recent-finished view."""
    if lane_state != "finished" or isinstance(activity_epoch, bool) or not isinstance(activity_epoch, (int, float)):
        return False
    now = time.time() if now is None else now
    try:
        activity_epoch = float(activity_epoch)
        now = float(now)
    except (TypeError, ValueError, OverflowError):
        return False
    if not math.isfinite(activity_epoch) or not math.isfinite(now):
        return False
    age = now - activity_epoch
    return 0 <= age <= RECENT_SECONDS


def annotate_ended_recent(lanes, now=None):
    """Mark recent finished lanes without filtering the running or unknown lane rows."""
    now = time.time() if now is None else now
    for lane in lanes:
        if isinstance(lane, dict):
            lane["ended_recent"] = ended_recently(
                lane.get("state"), lane.get("last_activity_epoch"), now)
    return lanes


def _has_row_outcomes(data):
    """Identify the legacy row-proof schema without assigning any new credit."""
    rows = data.get("functions") or data.get("results") or data.get("names")
    return isinstance(rows, list) and any(isinstance(row, dict) and "outcome" in row for row in rows)


def structured_result_evidence(data):
    """Normalize recent lane schemas without treating stage or claim evidence as full-function proof."""
    if not isinstance(data, dict):
        return None

    # Q135 stores stage runs and control replays as named objects alongside its attempts.
    stage_number = _count(data.get("new_stage"))
    if stage_number is not None and isinstance(data.get("attempts"), list) and isinstance(data.get("same_candidate_controls"), dict):
        rows = []
        stage_prefix = f"s{stage_number}-"
        stage_runs = {}
        for key, record in data.items():
            normalized = str(key).casefold().replace("_", "-")
            if normalized.startswith(stage_prefix) and isinstance(record, dict) and "status" in record:
                stage_runs[normalized] = record
        full_stage_passed = 0
        passed_stage_runs = 0
        for key, record in stage_runs.items():
            metrics = _proof_metrics(record)
            raw = display_value(record.get("status"), 80) or "status not recorded"
            status_ok = isinstance(record.get("status"), str) and record.get("status").casefold() in {
                "passed", "accepted", "verified"}
            if metrics["positive_passed"] is False or (metrics["positive_fails"] is not None and metrics["positive_fails"] > 0):
                outcome = "positive proof failed or incomplete"
            elif metrics["positive_ok"] is True and metrics["mutant_caught"] is False:
                outcome = "positive passed; mutant not caught"
            elif not status_ok:
                outcome = "stage run failed or incomplete"
            elif metrics["positive_ok"] is not True:
                outcome = "positive proof failed or incomplete"
            elif metrics["mutant_caught"] is True:
                outcome = "stage proof passed" if "full" in key and "probe" not in key else "probe passed"
                passed_stage_runs += 1
                if "full" in key and "probe" not in key:
                    full_stage_passed += 1
            else:
                outcome = "positive passed; mutant not caught"
            rows.append(_evidence_row(f"Stage {stage_number} · {key[len(stage_prefix):]}", outcome, metrics,
                                      "Stage-scoped evidence; complete-function verification is not established."))

        controls = data.get("same_candidate_controls") or {}
        control_records = controls.get("records") if isinstance(controls.get("records"), list) else []
        replay_passed = 0
        for index, record in enumerate(control_records, 1):
            if not isinstance(record, dict):
                continue
            metrics = _proof_metrics(record)
            passed = (record.get("status") == "passed" and metrics["positive_ok"] is True
                      and metrics["mutant_caught"] is True)
            replay_passed += int(passed)
            outcome = "control replay passed" if passed else ("positive passed; mutant not caught"
                       if metrics["positive_ok"] is True and metrics["mutant_caught"] is not True
                       else "control replay failed or incomplete")
            rows.append(_evidence_row(f"Prior-stage replay {index}", outcome, metrics,
                                      "Replay evidence for the current candidate; not a new stage or function credit."))
        replay_total = len(control_records) if control_records else _count(controls.get("total")) or 0

        # These are live current replays; they have no trial result yet and receive no proof credit.
        live_controls = []
        for key, record in data.items():
            normalized = str(key).casefold().replace("_", "-")
            if normalized.startswith("control-s") and isinstance(record, dict) and record.get("status") == "running":
                live_controls.append(normalized)
        for key in sorted(live_controls):
            rows.append(_evidence_row(f"Current replay {key.removeprefix('control-')}", "running", {},
                                      "No replay trial result is recorded yet."))

        display = (f"Stage {stage_number}: {full_stage_passed} partial stage proof(s) passed; "
                   f"{replay_passed}/{replay_total} prior-stage control replays passed; "
                   "whole-function verification: 0.")
        return {"rows": rows, "label": "Stage runs and control replays", "item_label": "Run / replay",
                "kind": "evidence", "display": display, "done": full_stage_passed,
                "of": 1, "unit": "partial stages", "partial_stages": full_stage_passed,
                "verified_functions": 0, "status": display_value(data.get("status"), 100),
                "scope": "Stage results and control replays do not establish whole-function verification."}

    # Q134 records proof progress per attempt, including a run identifier and completed-contract count.
    attempts = data.get("attempts")
    if isinstance(attempts, list) and any(isinstance(item, dict) and "proof_completed_contracts" in item for item in attempts):
        rows = []
        latest_completed = 0
        for index, attempt in enumerate(attempts, 1):
            if not isinstance(attempt, dict):
                continue
            completed = _count(attempt.get("proof_completed_contracts")) or 0
            latest_completed = completed
            run = attempt.get("proof_run_id")
            if not isinstance(run, str) or not re.fullmatch(r"[A-Za-z0-9_.-]{1,48}", run):
                run = f"attempt {index}"
            status = attempt.get("status") if isinstance(attempt.get("status"), str) else "status not recorded"
            rows.append(_evidence_row(f"Proof run {run}", status, {},
                                      f"{completed} completed contract record(s); this summary has no structured trial or mutant totals."))
        return {"rows": rows, "label": "Proof run progress", "item_label": "Proof run",
                "kind": "evidence", "display": f"{latest_completed} completed proof contract(s) in the latest run; "
                          "partial run in progress; whole-function verification is not established.",
                "done": latest_completed, "of": latest_completed, "unit": "completed proof contracts",
                "partial_stages": 0, "verified_functions": 0,
                "status": display_value(data.get("status"), 100),
                "scope": "Completed-contract counts are shown separately from full-function verification."}

    # Q133 stores one narrow contract result per function with explicit positive and negative records.
    functions = data.get("functions")
    if isinstance(functions, list) and any(isinstance(item, dict) and isinstance(item.get("stock_positive"), dict)
                                           for item in functions):
        rows = []
        accepted = 0
        for index, function in enumerate(functions, 1):
            if not isinstance(function, dict):
                continue
            positive = function.get("stock_positive")
            negative = function.get("same_contract_negative")
            metrics = _proof_metrics(positive, negative)
            accepted_here = (_accepted_decision(function.get("decision")) and metrics["positive_ok"] is True
                             and metrics["mutant_caught"] is True)
            accepted += int(accepted_here)
            outcome = "narrow contract accepted" if accepted_here else (
                "positive passed; mutant not caught" if metrics["positive_ok"] is True else "contract proof failed or incomplete")
            rows.append(_evidence_row(f"Function contract {index}", outcome, metrics,
                                      "Narrow contract only; full-function verification is not established."))
        return {"rows": rows, "label": "Narrow contract evidence", "item_label": "Contract proof",
                "kind": "evidence", "display": f"{accepted} narrow contract proof(s) accepted; whole-function verification: 0.",
                "done": accepted, "of": len(rows), "unit": "accepted narrow contracts",
                "partial_stages": 0, "accepted_contracts": accepted, "verified_functions": 0,
                "status": display_value(data.get("status"), 100),
                "scope": "Accepted narrow contracts remain separate from full-function verification."}

    # Q138 records claim decisions and prose notes, but no structured trial result on each claim.
    claims = data.get("claims")
    if isinstance(claims, list) and any(isinstance(item, dict) and "decision" in item for item in claims):
        rows = []
        accepted = 0
        for index, claim in enumerate(claims, 1):
            if not isinstance(claim, dict):
                continue
            decision = claim.get("decision") if isinstance(claim.get("decision"), str) else "decision not recorded"
            accepted += int(_accepted_decision(decision))
            rows.append(_evidence_row(f"Claim record {index}", decision, {},
                                      "Claim decision only; this record has no structured trial or mutant totals."))
        return {"rows": rows, "label": "Claim decisions", "item_label": "Claim",
                "kind": "evidence", "display": f"{accepted} accepted claim decision(s) recorded; "
                          "whole-function verification is not established by these summary records.",
                "done": accepted, "of": len(rows), "unit": "accepted claim decisions",
                "partial_stages": 0, "accepted_claims": accepted, "verified_functions": 0,
                "status": display_value(data.get("status"), 100),
                "scope": "Claim decisions and their prose notes are not converted into trial or mutant counts."}

    # Generic result status is work activity after the specific structured proof adapters above.
    # It never creates proof, mutant, partial-stage, or full-function credit.
    task_rows = _task_activity_rows(data)
    status = data.get("status")
    mixed_row_proof = (bool(task_rows) and not (isinstance(status, str) and status.strip())
                       and _has_row_outcomes(data))
    if not mixed_row_proof and ((isinstance(status, str) and status.strip()) or task_rows):
        tests = data.get("tests") if isinstance(data.get("tests"), list) else []
        changes = data.get("changes") if isinstance(data.get("changes"), list) else []
        rows = []
        passed = failed = 0
        for index, item in enumerate(tests, 1):
            if isinstance(item, dict):
                name = item.get("name") or item.get("test") or item.get("item") or f"Test {index}"
                outcome = item.get("status") or item.get("outcome") or item.get("result") or "result not recorded"
                detail = item.get("details") or item.get("detail") or item.get("summary") or item.get("description") or ""
            else:
                name, outcome, detail = f"Test {index}", "result not recorded", item
            normalized = outcome.casefold().strip() if isinstance(outcome, str) else ""
            passed += int(normalized in {"pass", "passed", "success", "succeeded"})
            failed += int(normalized in {"fail", "failed", "error", "errored"})
            rows.append({"category": "Test", "name": display_value(name, 120),
                         "outcome": display_value(outcome, 120), "detail": display_value(detail, 260)})
        for index, item in enumerate(changes, 1):
            if isinstance(item, dict):
                name = item.get("name") or item.get("title") or item.get("item") or f"Change {index}"
                detail = item.get("description") or item.get("details") or item.get("detail") or item.get("summary") or ""
            else:
                name, detail = f"Change {index}", item
            rows.append({"category": "Change", "name": display_value(name, 120),
                         "outcome": "recorded", "detail": display_value(detail, 260)})
        rows.extend(task_rows)
        test_summary = f"{len(tests)} test result(s) recorded"
        if passed or failed:
            counts = []
            if passed:
                counts.append(f"{passed} passed")
            if failed:
                counts.append(f"{failed} failed")
            test_summary += " (" + ", ".join(counts) + ")"
        change_summary = f"{len(changes)} change(s) recorded"
        nested_target = data.get("target") if isinstance(data.get("target"), dict) else {}
        target_value = data.get("target_va")
        if not isinstance(target_value, str) or not target_value.strip():
            target_value = nested_target.get("address")
        target_display = display_value(target_value, 160) if isinstance(target_value, str) else ""
        task_type = data.get("task_type") if isinstance(data.get("task_type"), str) else ""
        display_parts = ([f"Work status: {display_value(status, 100)}"]
                         if isinstance(status, str) and status.strip() else ["Task activity recorded"])
        if task_type.strip():
            display_parts.append(f"task type: {display_value(task_type, 120)}")
        if isinstance(data.get("scope"), str) and data["scope"].strip():
            display_parts.append(f"scope: {display_value(data['scope'], 240)}")
        if target_display.strip():
            display_parts.append(f"target: {target_display}")
        if tests or changes:
            display_parts.extend((change_summary, test_summary))
        elif not task_rows:
            display_parts.append("no changes or test results recorded")
        display = "; ".join(display_parts) + "."
        scope_value = (display_value(data.get("scope"), 400) if isinstance(data.get("scope"), str)
                       else "Recorded task activity only; no proof or function credit is inferred.")
        return {"rows": rows, "label": "Tool-result activity", "item_label": "Activity",
                "kind": "activity", "display": display, "done": len(changes), "of": len(changes),
                "unit": "changes recorded", "status": display_value(status, 100),
                "task": display_value(data.get("task"), 2000) if isinstance(data.get("task"), str) else "",
                "task_type": display_value(task_type, 120), "target_va": target_display,
                "tests_count": len(tests), "tests_passed": passed, "tests_failed": failed,
                "changes_count": len(changes), "scope": scope_value}
    return None


def yield_of(lane, registry_entry=None):
    data = result_document(lane, registry_entry)
    evidence = structured_result_evidence(data)
    if evidence is not None:
        return {key: evidence[key] for key in ("done", "of", "unit", "display", "status", "scope",
                                                  "verified_functions", "partial_stages", "accepted_contracts", "accepted_claims",
                                                  "kind", "task", "task_type", "target_va", "tests_count", "tests_passed", "tests_failed", "changes_count")
                if key in evidence}
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

    rows = result_rows(lane, registry_entry)
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


def result_detail_rows(lane, registry_entry=None):
    data = result_document(lane, registry_entry)
    evidence = structured_result_evidence(data)
    if evidence is not None:
        return evidence["rows"], evidence["label"], evidence["item_label"], evidence["kind"]
    rows = result_rows(lane, registry_entry)
    if rows is not None:
        task_rows = _task_activity_rows(data)
        if task_rows and not (isinstance(data.get("status"), str) and data.get("status").strip()) \
                and _has_row_outcomes(data):
            # Show notes beside proof rows; yield_of still counts only the original result rows.
            rows = list(rows) + [{"name": f"{row['category']}: {row['name']}",
                                  "outcome": row["outcome"], "detail": row["detail"]}
                                 for row in task_rows]
            return rows, "Function results and task activity", "Result / note", "legacy"
        if any(isinstance(row, dict) and row.get("status") and not row.get("outcome") for row in rows):
            return rows, "Function evidence", "Function", "legacy"
        return rows, "Results", "Function", "legacy"

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
        return rows, "Stage checks", "Stage", "legacy"

    contracts = accepted_contracts(data)
    if contracts:
        rows = [{"name": name, "outcome": "accepted (branch-scoped)",
                 "detail": "Accepted contract on this candidate; full-function verification is not established."}
                for name in contracts]
        return rows, "Accepted branch-scoped contracts", "Contract", "legacy"
    return None, None, None, "legacy"


def initial_prompt(lane, registry_entry=None):
    """Prefer saved prompts; fall back to explicit prompt/task text in the lane result document."""
    prompt_path = lane_file_path(lane, "prompt", registry_entry)
    try:
        default_prompt_path = (SCRATCH / lane / "prompt.txt").resolve(strict=False)
    except (OSError, RuntimeError):
        default_prompt_path = None
    prompt_source = "registry prompt_path" if prompt_path is not None and prompt_path != default_prompt_path else "lane prompt.txt"
    candidates = ((prompt_path, prompt_source), (COORD / "prompts" / f"{lane}.txt", "coordinator prompt file"))
    for path, source in candidates:
        if path is None:
            continue
        try:
            prompt = path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue
        if prompt.strip() and prompt.strip().casefold() not in {"no prompt recorded", "no initial prompt recorded", "not recorded", "unknown", "none", "n/a"}:
            return prompt, source
    prompt = (registry_entry or {}).get("prompt")
    if isinstance(prompt, str) and prompt.strip() and prompt.strip().casefold() not in {"no prompt recorded", "no initial prompt recorded", "not recorded", "unknown", "none", "n/a"}:
        return prompt, "registry assignment"
    data = result_document(lane, registry_entry)
    for key, source in (("full_prompt", "results.json full_prompt"), ("initial_prompt", "results.json initial_prompt"),
                        ("prompt", "results.json prompt"), ("prompt_text", "results.json prompt_text"),
                        ("task", "results.json task text")):
        prompt = data.get(key)
        if isinstance(prompt, str) and prompt.strip() and prompt.strip().casefold() not in {
                "no prompt recorded", "no initial prompt recorded", "not recorded", "unknown", "none", "n/a"}:
            limit = 2000
            suffix = "\n\n[truncated at 2,000 characters]"
            if len(prompt) > limit:
                prompt = prompt[:limit - len(suffix)] + suffix
            return prompt, source
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
    log = lane_file_path(lane, "log", registry_entry)
    try:
        stat = log.stat() if log is not None else None
    except (OSError, AttributeError):
        stat = None
    last = tail(log, 1)
    process = live.get(lane)
    exited = None
    if last:
        match = re.search(r"lane exited with code (-?\d+)", last[0])
        if match:
            exited = int(match.group(1))
    summary_path = lane_file_path(lane, "summary", registry_entry)
    has_results = summary_path.exists() if summary_path is not None else False
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
            "source": (registry_entry or {}).get("source") or (process or {}).get("source") or "muse",
            "started": (registry_entry or {}).get("started") or (process or {}).get("started"),
            "agent_memory_mb": None if registry_entry is not None else (process or {}).get("mem_mb"),
            "last_activity": datetime.fromtimestamp(stat.st_mtime).isoformat(timespec="seconds") if stat else None,
            "last_activity_epoch": stat.st_mtime if stat else None,
            "progress": progress[:300], "yield": yield_of(lane, registry_entry), "brief": brief_tag(lane),
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
    annotate_ended_recent(lanes, now)
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
    summary_path = lane_file_path(lane, "summary", registry_entry)
    try:
        summary_text = summary_path.read_text(encoding="utf-8", errors="replace") if summary_path is not None else None
    except OSError:
        summary_text = None
    summary_is_current = registered_lane_path(lane, "summary", registry_entry) is not None
    summary_label = ("Current-task summary" if summary_is_current else
                     "Historical summary" if has_current_task_context(lane, registry_entry) and summary_text is not None
                     else "Lane summary")
    rows, results_label, result_item_label, result_kind = result_detail_rows(lane, registry_entry)
    slim = None
    if rows:
        slim = []
        for row in rows[:400]:
            if not isinstance(row, dict):
                continue
            if result_kind == "evidence":
                slim.append({"address": None, "name": display_value(row.get("name"), 120) or None,
                             "size": None, "outcome": display_value(row.get("outcome"), 120) or None,
                             "reason": None, "attempts": None, "mutant_caught": row.get("mutant_caught")
                             if isinstance(row.get("mutant_caught"), bool) else None,
                             "confidence": None, "detail": display_value(row.get("detail"), 260),
                             "positive_trials": _count(row.get("positive_trials")),
                             "positive_fails": _count(row.get("positive_fails")),
                             "original_trials": _count(row.get("original_trials")),
                             "mutant_trials": _count(row.get("mutant_trials")),
                             "mutant_fails": _count(row.get("mutant_fails")),
                             "full_function_verified": row.get("full_function_verified") is True})
                continue
            if result_kind == "activity":
                slim.append({"category": display_value(row.get("category"), 40),
                             "name": display_value(row.get("name"), 120),
                             "outcome": display_value(row.get("outcome"), 120),
                             "detail": display_value(row.get("detail"), 260)})
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
    result = yield_of(lane, registry_entry)
    return {"lane": lane, "model": registry_entry.get("model"), "effort": registry_entry.get("effort"),
            "prompt": prompt, "prompt_source": prompt_source, "prompt_chars": len(prompt) if prompt else 0,
            "summary_label": summary_label, "log": tail(lane_file_path(lane, "log", registry_entry), 80), "summary": summary_text, "results": slim,
            "results_label": results_label, "result_item_label": result_item_label, "result_kind": result_kind,
            "results_total": len(rows) if isinstance(rows, list) else 0, "yield": result,
            "batch": read_json(LISTS / f"{lane}.json")}


# ---------------------------------------------------------------------------------------------------------
# History: one sample a minute (lanes running, verified, memory), kept in memory and in a rolling file.
# ---------------------------------------------------------------------------------------------------------

_history = []  # rows as written to HISTORY_FILE, oldest first


def history_sample(now, lanes, progress, mem):
    """One history row. `lanes` is the live-lane map, `progress` the site's progress data, `mem` memory() or None."""
    stages = (progress or {}).get("stages") or {}
    memory_unknown = any(p.get("source") in ("codex", "claude") for p in lanes.values())
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
