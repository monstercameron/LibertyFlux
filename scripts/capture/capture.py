#!/usr/bin/env python3
"""Capture the game's code from the owner's own running copy, for analysis.

Reads (never writes) the running game process: finds it by exe name,
snapshots its module list, reads the code, read-only data and initialised
data sections in 4 KB pages, undoes relocation so the bytes are expressed at
the preferred base, verifies them against the file on disk, and writes the
decrypted range + a reconstructed image + a JSON report. Game-capture outputs
go ONLY under orig/ (ignored by git, never published).

Read-only guarantees: the process is opened with query-information + read-
memory rights only. The tool never writes to the process, never suspends it,
never injects anything, never attaches a debugger.

Usage (from the repository root, with the repo Python):
  capture.py capture [--exe PATH] [--exe-name NAME] [--pid N] [--out-dir D]
      Capture the running game. The reference executable defaults to
      orig/GTAIV.exe (the game itself is found by process name, never by
      folder). Outputs go only under orig/ (the --out-dir back door still
      refuses anything else).
  capture.py selftest
      Dry run without the game: build/launch a 32-bit sleeper of our own
      (sleeper.c, compiled with the repo's zig), capture it through the
      same code path, verify byte for byte, terminate it. Writes under
      .artifacts/capture-selftest/ only. Ends with SELFTEST PASS.
Requires: 64-bit Windows Python + pefile. No admin rights needed.

Run-book: reading the game's code from the owner's running copy
---------------------------------------------------------------
What the owner does: start the game normally through Steam, as usual
(no options, no windowed mode, no admin rights); wait until the main menu
is on screen (the wrapper has decrypted the code by then); leave the game
running and tell the coordinator it is at the menu. That is all -- the
owner does not run anything.

The single command the coordinator runs, from the repository root with the
game at the main menu:
    .venv\\Scripts\\python.exe scripts\\capture\\capture.py capture
It takes about 5 seconds (roughly 2 s parsing the executable, 1 s
verifying; memory reads of 26 MB are effectively instant).

How to tell it worked: the command prints, and the run is good when all of
these hold:
- `encrypted-boundary check: ... OK`
- `capturing pid N ...` with no error after it
- `plaintext .text outside encrypted range: 0 diff bytes ... (EXACT MATCH)`
- `encrypted range: file prologues={...zeros...} captured prologues={...large...}`
  (zeros on the file side, hundreds or thousands on the captured side)
- `wrote ... text_decrypted.bin, ... GTAIV_decrypted.exe, ... capture_report.json`
Then confirm the three outputs exist under `orig/` and the reconstructed
image has exactly the same size as the original.

What the outputs are (all under `orig/`, never published):
- `text_decrypted.bin`: the decrypted first megabyte of the code section,
  expressed at the preferred base.
- `GTAIV_decrypted.exe`: a full copy of the game executable with only that
  range replaced. For Ghidra and the checker, loaded in place of the
  original. Static analysis only: never launch it (the start-up wrapper
  would try to decrypt already-decrypted code and corrupt it).
- `capture_report.json`: what was read and verified: process id, actual and
  preferred load addresses, pages read and unreadable, relocation counts,
  every byte difference outside the encrypted range, per-page entropy before
  and after inside it, prologue counts, the full loaded-module list with
  base addresses, and timings.

If something goes wrong:
- `no running process named GTAIV.exe`: the game is not running yet, or is
  running under a different exe name. Check the process list.
- `several GTAIV.exe processes: [...]`: pass one explicitly, e.g. append
  `--pid 1234`.
- `OpenProcess ... failed: Access is denied`: the game runs at a higher
  privilege than the shell. Start the shell the same way the game was
  started (normally: neither elevated); do not elevate to work around this
  without asking.
- `plaintext .text ... DIFFERS`: do not re-run to make it match. The
  differences are the finding (run-time patches by the game or its
  wrapper). Save the JSON report and hand it to the coordinator.
- `pages ... unreadable` nonzero: the range could not be fully read. Save
  the report and say so; the unreadable page list is in it.

Safety properties (verified in the self-test):
- The tool opens the game with query-information and read-memory rights
  only. It never writes to the process, never suspends it, never injects
  code, never attaches a debugger, and cannot affect gameplay.
- It reads only the code, read-only data and initialised data ranges.
- Game-capture outputs go only under `orig/`. The tool refuses any other
  output folder except its own self-test folder.
"""

import argparse
import ctypes
import datetime
import hashlib
import json
import math
import os
import struct
import subprocess
import sys
import time
from ctypes import wintypes

TOOL_VERSION = "1.0.0"

# --------------------------------------------------------------------------
# Constants (measured by earlier lanes, re-verified here against the file)

GAME_EXE_NAME = "GTAIV.exe"
CAPTURE_SECTIONS = (".text", ".rdata", ".data")
# Encrypted range inside .text, section-relative (p0-startup-stub: boundary at
# 0xFA000 verified by per-4KB entropy; .tbm duplicates .text[0:0xFB000]).
ENCRYPTED_REL_END = 0xFA000
PAGE = 0x1000

# Plausible x86 function-prologue byte patterns (non-overlapping counts).
PROLOGUES = {
    "push_ebp_mov_ebp_esp": b"\x55\x8b\xec",
    "hotpatch": b"\x8b\xff\x55\x8b\xec",
    "push_esi_mov_esi_ecx": b"\x56\x8b\xf1",
    "push_ebx_esi_edi": b"\x53\x56\x57",
}

MAX_DIFF_RUNS = 64       # diff runs stored per section (count is unbounded)
MAX_RUN_SAMPLE = 32      # hex-sampled bytes per side per diff run


def repo_root():
    # capture.py lives in scripts/capture/; find the root by marker.
    here = os.path.abspath(os.path.dirname(__file__))
    while True:
        if os.path.exists(os.path.join(here, "AGENTS.md")):
            return here
        parent = os.path.dirname(here)
        if parent == here:
            break
        here = parent
    # Fallback: scripts/capture/ is two levels below the root.
    here = os.path.abspath(os.path.dirname(__file__))
    return os.path.dirname(os.path.dirname(here))


def orig_dir():
    return os.path.join(repo_root(), "orig")


def selftest_dir():
    # Self-test outputs stay out of the tracked tree, under .artifacts/.
    return os.path.join(repo_root(), ".artifacts", "capture-selftest")


def scratch_dir():
    return os.path.abspath(os.path.dirname(__file__))


# --------------------------------------------------------------------------
# Win32 API (ctypes)

kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)

TH32CS_SNAPPROCESS = 0x00000002
TH32CS_SNAPMODULE = 0x00000008
TH32CS_SNAPMODULE32 = 0x00000020
PROCESS_QUERY_INFORMATION = 0x0400
PROCESS_VM_READ = 0x0010
READ_ONLY_ACCESS = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ
INVALID_HANDLE_VALUE = ctypes.c_void_p(-1).value


class PROCESSENTRY32W(ctypes.Structure):
    _fields_ = [
        ("dwSize", wintypes.DWORD),
        ("cntUsage", wintypes.DWORD),
        ("th32ProcessID", wintypes.DWORD),
        ("th32DefaultHeapID", ctypes.c_size_t),
        ("th32ModuleID", wintypes.DWORD),
        ("cntThreads", wintypes.DWORD),
        ("th32ParentProcessID", wintypes.DWORD),
        ("pcPriClassBase", wintypes.LONG),
        ("dwFlags", wintypes.DWORD),
        ("szExeFile", wintypes.WCHAR * 260),
    ]


class MODULEENTRY32W(ctypes.Structure):
    _fields_ = [
        ("dwSize", wintypes.DWORD),
        ("th32ModuleID", wintypes.DWORD),
        ("th32ProcessID", wintypes.DWORD),
        ("GlblcntUsage", wintypes.DWORD),
        ("ProccntUsage", wintypes.DWORD),
        ("modBaseAddr", ctypes.c_void_p),
        ("modBaseSize", wintypes.DWORD),
        ("hModule", wintypes.HMODULE),
        ("szModule", wintypes.WCHAR * 256),
        ("szExePath", wintypes.WCHAR * 260),
    ]


kernel32.CreateToolhelp32Snapshot.argtypes = [wintypes.DWORD, wintypes.DWORD]
kernel32.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
kernel32.Process32FirstW.argtypes = [wintypes.HANDLE,
                                    ctypes.POINTER(PROCESSENTRY32W)]
kernel32.Process32FirstW.restype = wintypes.BOOL
kernel32.Process32NextW.argtypes = [wintypes.HANDLE,
                                   ctypes.POINTER(PROCESSENTRY32W)]
kernel32.Process32NextW.restype = wintypes.BOOL
kernel32.Module32FirstW.argtypes = [wintypes.HANDLE,
                                   ctypes.POINTER(MODULEENTRY32W)]
kernel32.Module32FirstW.restype = wintypes.BOOL
kernel32.Module32NextW.argtypes = [wintypes.HANDLE,
                                  ctypes.POINTER(MODULEENTRY32W)]
kernel32.Module32NextW.restype = wintypes.BOOL
kernel32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL,
                                wintypes.DWORD]
kernel32.OpenProcess.restype = wintypes.HANDLE
kernel32.ReadProcessMemory.argtypes = [wintypes.HANDLE, wintypes.LPCVOID,
                                      wintypes.LPVOID, ctypes.c_size_t,
                                      ctypes.POINTER(ctypes.c_size_t)]
kernel32.ReadProcessMemory.restype = wintypes.BOOL
kernel32.CloseHandle.argtypes = [wintypes.HANDLE]
kernel32.CloseHandle.restype = wintypes.BOOL
kernel32.IsWow64Process.argtypes = [wintypes.HANDLE,
                                   ctypes.POINTER(wintypes.BOOL)]
kernel32.IsWow64Process.restype = wintypes.BOOL

psapi = ctypes.WinDLL("psapi", use_last_error=True)

LIST_MODULES_32BIT = 0x01
LIST_MODULES_64BIT = 0x02


class MODULEINFO(ctypes.Structure):
    _fields_ = [
        ("lpBaseOfDll", ctypes.c_void_p),
        ("SizeOfImage", wintypes.DWORD),
        ("EntryPoint", ctypes.c_void_p),
    ]


psapi.EnumProcessModulesEx.argtypes = [wintypes.HANDLE,
                                      ctypes.POINTER(wintypes.HMODULE),
                                      wintypes.DWORD,
                                      ctypes.POINTER(wintypes.DWORD),
                                      wintypes.DWORD]
psapi.EnumProcessModulesEx.restype = wintypes.BOOL
psapi.GetModuleFileNameExW.argtypes = [wintypes.HANDLE, wintypes.HMODULE,
                                      wintypes.LPWSTR, wintypes.DWORD]
psapi.GetModuleFileNameExW.restype = wintypes.DWORD
psapi.GetModuleInformation.argtypes = [wintypes.HANDLE, wintypes.HMODULE,
                                      ctypes.POINTER(MODULEINFO),
                                      wintypes.DWORD]
psapi.GetModuleInformation.restype = wintypes.BOOL


def enum_modules_ex(handle, pid, which):
    """Enumerate modules via EnumProcessModulesEx (sees the 32-bit list of
    a WoW64 target from a 64-bit caller, which toolhelp misses)."""
    needed = wintypes.DWORD(0)
    buf = (wintypes.HMODULE * 256)()
    ok = psapi.EnumProcessModulesEx(handle, buf, ctypes.sizeof(buf),
                                    ctypes.byref(needed), which)
    if not ok:
        raise OSError("EnumProcessModulesEx(pid=%d, filter=%d) failed: %s"
                      % (pid, which,
                         ctypes.WinError(ctypes.get_last_error())))
    count = needed.value // ctypes.sizeof(wintypes.HMODULE)
    mods = []
    for hmod in buf[:count]:
        mi = MODULEINFO()
        if not psapi.GetModuleInformation(handle, hmod, ctypes.byref(mi),
                                         ctypes.sizeof(mi)):
            continue
        name_buf = ctypes.create_unicode_buffer(260)
        psapi.GetModuleFileNameExW(handle, hmod, name_buf, 260)
        path = name_buf.value
        mods.append({
            "name": os.path.basename(path) if path else "0x%X" % hmod,
            "path": path,
            "base": mi.lpBaseOfDll,
            "size": mi.SizeOfImage,
        })
    return mods


def find_processes_by_name(exe_name):
    """Return [pid, ...] for processes whose exe name matches (case-blind)."""
    snap = kernel32.CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    if snap == INVALID_HANDLE_VALUE:
        raise OSError("CreateToolhelp32Snapshot(PROCESS) failed: %s"
                      % ctypes.WinError(ctypes.get_last_error()))
    pids = []
    try:
        entry = PROCESSENTRY32W()
        entry.dwSize = ctypes.sizeof(PROCESSENTRY32W)
        ok = kernel32.Process32FirstW(snap, ctypes.byref(entry))
        while ok:
            if entry.szExeFile.lower() == exe_name.lower():
                pids.append(entry.th32ProcessID)
            ok = kernel32.Process32NextW(snap, ctypes.byref(entry))
    finally:
        kernel32.CloseHandle(snap)
    return pids


def list_modules(pid):
    """Return [{name, path, base, size}] for all modules in pid.

    TH32CS_SNAPMODULE32 makes this work for a 32-bit target from a 64-bit
    caller. The first entry is the main module.
    """
    snap = kernel32.CreateToolhelp32Snapshot(
        TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
    if snap == INVALID_HANDLE_VALUE:
        raise OSError("CreateToolhelp32Snapshot(MODULE, pid=%d) failed: %s"
                      % (pid, ctypes.WinError(ctypes.get_last_error())))
    mods = []
    try:
        entry = MODULEENTRY32W()
        entry.dwSize = ctypes.sizeof(MODULEENTRY32W)
        ok = kernel32.Module32FirstW(snap, ctypes.byref(entry))
        while ok:
            mods.append({
                "name": entry.szModule,
                "path": entry.szExePath,
                "base": entry.modBaseAddr,
                "size": entry.modBaseSize,
            })
            ok = kernel32.Module32NextW(snap, ctypes.byref(entry))
    finally:
        kernel32.CloseHandle(snap)
    return mods


def read_pages(handle, base_addr, size):
    """Read [base_addr, base_addr+size) in 4 KB pages.

    Returns (data: bytearray, unreadable: [page_rva_offset, ...]).
    Unreadable pages are zero-filled so offsets stay stable.
    """
    data = bytearray(size)
    unreadable = []
    buf = (ctypes.c_char * PAGE)()
    nread = ctypes.c_size_t(0)
    for off in range(0, size, PAGE):
        chunk = min(PAGE, size - off)
        nread.value = 0
        ok = kernel32.ReadProcessMemory(handle, base_addr + off,
                                        buf, chunk, ctypes.byref(nread))
        if ok and nread.value == chunk:
            data[off:off + chunk] = bytes(buf)[:chunk]
        else:
            unreadable.append(off)
    return data, unreadable


# --------------------------------------------------------------------------
# PE helpers (pefile, file on disk, read-only)

def load_pe(exe_path):
    import pefile  # local import so --help works without it
    return pefile.PE(exe_path, fast_load=False)


def section_info(pe, name):
    for s in pe.sections:
        if s.Name.rstrip(b"\x00") == name.encode():
            return {
                "name": name,
                "rva": s.VirtualAddress,
                "vsize": s.Misc_VirtualSize,
                "rawptr": s.PointerToRawData,
                "rawsize": s.SizeOfRawData,
                "file_data": bytes(s.get_data()),
            }
    return None


def highlow_reloc_rvas(pe):
    """All RVA locations of IMAGE_REL_BASED_HIGHLOW entries."""
    rvas = []
    if not hasattr(pe, "DIRECTORY_ENTRY_BASERELOC"):
        return rvas
    for block in pe.DIRECTORY_ENTRY_BASERELOC:
        for entry in block.entries:
            if entry.type == 3:  # HIGHLOW
                rvas.append(entry.rva)
    return rvas


def unrelocate(buf, base_rva, reloc_rvas, delta):
    """Subtract the base delta at each relocated dword inside buf.

    buf covers [base_rva, base_rva+len(buf)). Returns count applied.
    """
    if delta == 0:
        # Still count how many sites fall inside, for the report.
        end = base_rva + len(buf)
        return sum(1 for r in reloc_rvas if base_rva <= r <= end - 4)
    end = base_rva + len(buf)
    n = 0
    for r in reloc_rvas:
        off = r - base_rva
        if 0 <= off <= len(buf) - 4:
            val = struct.unpack_from("<I", buf, off)[0]
            struct.pack_into("<I", buf, off, (val - delta) & 0xFFFFFFFF)
            n += 1
    return n


# --------------------------------------------------------------------------
# Verification helpers

def shannon_entropy(data):
    if not data:
        return 0.0
    counts = [0] * 256
    for b in data:
        counts[b] += 1
    ent = 0.0
    n = len(data)
    for c in counts:
        if c:
            p = c / n
            ent -= p * math.log2(p)
    return ent


def entropy_per_page(data):
    return [round(shannon_entropy(data[i:i + PAGE]), 4)
            for i in range(0, len(data), PAGE)]


def prologue_counts(data):
    return {name: data.count(pat) for name, pat in PROLOGUES.items()}


def cc_fraction(data):
    if not data:
        return 0.0
    return round(data.count(b"\xcc") / len(data), 6)


def diff_runs(a, b):
    """Mismatch runs between equal-length bytes. Returns (total, runs).

    runs: [{offset, len, a_hex, b_hex}] capped at MAX_DIFF_RUNS (total is
    exact).
    """
    assert len(a) == len(b)
    total = 0
    runs = []
    i, n = 0, len(a)
    while i < n:
        if a[i] != b[i]:
            j = i
            while j < n and a[j] != b[j]:
                j += 1
            total += j - i
            if len(runs) < MAX_DIFF_RUNS:
                runs.append({
                    "offset": i,
                    "len": j - i,
                    "file_hex": a[i:i + MAX_RUN_SAMPLE].hex(),
                    "mem_hex": b[i:i + MAX_RUN_SAMPLE].hex(),
                })
            i = j
        else:
            i += 1
    return total, runs


def sha256_hex(data):
    return hashlib.sha256(data).hexdigest()


# --------------------------------------------------------------------------
# Capture core (shared by game capture and self-test)

def capture_process(pid, exe_path, sections, out_dir, encrypted_rel_end,
                    exe_label):
    """Capture + unrelocate + verify pid. Returns (report, artifacts).

    artifacts: {section_name: unrelocated bytearray} for writing by caller.
    """
    t0 = time.time()
    timing = {}
    notes = []

    pe = load_pe(exe_path)
    preferred = pe.OPTIONAL_HEADER.ImageBase
    size_of_image = pe.OPTIONAL_HEADER.SizeOfImage
    secinfo = {}
    for name in sections:
        info = section_info(pe, name)
        if info is None:
            raise ValueError("section %s not found in %s" % (name, exe_path))
        secinfo[name] = info
    relocs = highlow_reloc_rvas(pe)
    timing["pe_parse_s"] = round(time.time() - t0, 2)

    # Open read-only: query information + read memory only.
    t1 = time.time()
    handle = kernel32.OpenProcess(READ_ONLY_ACCESS, False, pid)
    if not handle:
        raise OSError("OpenProcess(pid=%d, access=0x%x) failed: %s"
                      % (pid, READ_ONLY_ACCESS,
                         ctypes.WinError(ctypes.get_last_error())))
    try:
        wow64 = wintypes.BOOL(False)
        is_wow64 = bool(kernel32.IsWow64Process(handle, ctypes.byref(wow64))
                        and wow64.value)
        modules = list_modules(pid)
        if not modules:
            raise OSError("module snapshot for pid=%d is empty" % pid)
        for m in modules:
            m["via"] = "toolhelp"
        # Toolhelp from a 64-bit caller misses the 32-bit DLLs of a WoW64
        # target (kernel32 etc.); EnumProcessModulesEx sees them. Union
        # both lists, keyed by base address.
        try:
            enum32 = enum_modules_ex(handle, pid, LIST_MODULES_32BIT)
            seen = {m["base"] for m in modules}
            for m in enum32:
                if m["base"] not in seen:
                    m["via"] = "enum32"
                    modules.append(m)
                    seen.add(m["base"])
            if enum32 and enum32[0]["base"] != modules[0]["base"]:
                notes.append("main-module base differs between toolhelp "
                             "(0x%X) and EnumProcessModulesEx (0x%X); "
                             "using toolhelp" % (modules[0]["base"],
                                                 enum32[0]["base"]))
            else:
                notes.append("main-module base confirmed by two APIs")
        except OSError as e:
            notes.append("EnumProcessModulesEx unavailable: %s" % e)
        actual_base = modules[0]["base"]
        image_path = modules[0]["path"]

        captured = {}
        unreadable_all = {}
        t2 = time.time()
        for name in sections:
            info = secinfo[name]
            # In-memory footprint: VSize rounded up to pages.
            mem_size = (info["vsize"] + PAGE - 1) // PAGE * PAGE
            data, unread = read_pages(handle, actual_base + info["rva"],
                                      mem_size)
            captured[name] = data
            unreadable_all[name] = unread
        timing["read_s"] = round(time.time() - t2, 2)
    finally:
        kernel32.CloseHandle(handle)
    timing["open_s"] = round(time.time() - t1 - timing["read_s"], 2)

    delta = (actual_base - preferred) & 0xFFFFFFFF
    if actual_base >= 0x100000000 or preferred >= 0x100000000:
        raise ValueError("64-bit address seen for a 32-bit target")

    # Undo relocation at every HIGHLOW site inside the captured sections.
    t3 = time.time()
    applied = {}
    for name in sections:
        applied[name] = unrelocate(captured[name], secinfo[name]["rva"],
                                   relocs, delta)
    timing["unrelocate_s"] = round(time.time() - t3, 2)

    # --- Verify ------------------------------------------------------
    t4 = time.time()
    verify = {}
    for name in sections:
        info = secinfo[name]
        mem = bytes(captured[name][:info["vsize"]])
        file_bytes = info["file_data"][:info["vsize"]]
        if len(file_bytes) < info["vsize"]:
            # .data-style: no file bytes past RawSize; compare file-backed.
            mem = mem[:len(file_bytes)]
        total, runs = diff_runs(file_bytes, mem)
        # Re-base run offsets to RVAs for the report.
        for r in runs:
            r["rva"] = "0x%X" % (info["rva"] + r.pop("offset"))
        verify[name] = {
            "compared_bytes": len(file_bytes),
            "diff_bytes": total,
            "diff_runs_stored": len(runs),
            "diff_runs": runs,
            "match": total == 0,
        }

    encrypted = None
    if encrypted_rel_end and ".text" in secinfo:
        text = secinfo[".text"]
        erange = (text["rva"], text["rva"] + encrypted_rel_end)
        fenc = text["file_data"][:encrypted_rel_end]
        menc = bytes(captured[".text"][:encrypted_rel_end])
        # Plaintext part of .text must equal the file exactly.
        pv = verify.get(".text")
        plain_diff = None
        if pv is not None:
            # Recompute diff restricted to the plaintext sub-range.
            fplain = text["file_data"][encrypted_rel_end:text["vsize"]]
            mplain = bytes(captured[".text"][encrypted_rel_end:text["vsize"]])
            total, runs = diff_runs(fplain, mplain)
            for r in runs:
                r["rva"] = "0x%X" % (text["rva"] + encrypted_rel_end
                                     + r.pop("offset"))
            plain_diff = {"compared_bytes": len(fplain), "diff_bytes": total,
                          "diff_runs_stored": len(runs), "diff_runs": runs,
                          "match": total == 0}
        encrypted = {
            "range_rva": ["0x%X" % erange[0], "0x%X" % erange[1]],
            "pages": encrypted_rel_end // PAGE,
            "entropy_file": entropy_per_page(fenc),
            "entropy_captured": entropy_per_page(menc),
            "prologues_file": prologue_counts(fenc),
            "prologues_captured": prologue_counts(menc),
            "cc_fraction_file": cc_fraction(fenc),
            "cc_fraction_captured": cc_fraction(menc),
            "plaintext_check": plain_diff,
        }
    timing["verify_s"] = round(time.time() - t4, 2)

    # --- Report ------------------------------------------------------
    report = {
        "tool": "h-capture capture.py",
        "version": TOOL_VERSION,
        "mode": exe_label,
        "timestamp": datetime.datetime.now(
            datetime.timezone.utc).isoformat(timespec="seconds"),
        "host": {
            "python": sys.version.split()[0],
            "pointer_bits": struct.calcsize("P") * 8,
        },
        "target": {"exe_name": os.path.basename(exe_path), "pid": pid,
                   "image_path": image_path, "is_wow64": is_wow64},
        "image": {
            "preferred_base": "0x%X" % preferred,
            "actual_base": "0x%X" % actual_base,
            "delta": "0x%X" % delta,
            "size_of_image": "0x%X" % size_of_image,
        },
        "sections": [
            {"name": n, "rva": "0x%X" % secinfo[n]["rva"],
             "vsize": "0x%X" % secinfo[n]["vsize"],
             "rawsize": "0x%X" % secinfo[n]["rawsize"],
             "pages_total": len(captured[n]) // PAGE,
             "pages_unreadable": len(unreadable_all[n]),
             "unreadable_offsets": ["0x%X" % o for o in unreadable_all[n][:64]],
             "sha256_file": sha256_hex(secinfo[n]["file_data"]),
             "sha256_captured_unrelocated":
                 sha256_hex(bytes(captured[n]))}
            for n in sections
        ],
        "relocations": {
            "total_highlow": len(relocs),
            "applied": applied,
        },
        "verify": verify,
        "encrypted_range": encrypted,
        "modules": [
            {"name": m["name"], "path": m["path"],
             "base": "0x%X" % m["base"], "size": "0x%X" % m["size"],
             "via": m.get("via", "?")}
            for m in modules
        ],
        "timing": timing,
        "notes": notes,
    }
    return report, captured, secinfo


# --------------------------------------------------------------------------
# Game capture: writes ONLY under orig/

def check_out_dir(out_dir):
    out = os.path.abspath(out_dir)
    allowed = (os.path.abspath(orig_dir()),
               os.path.abspath(selftest_dir()))
    if out not in allowed:
        raise ValueError("refusing to write outside orig/ or the self-test "
                         "folder: %s" % out)
    os.makedirs(out, exist_ok=True)
    return out


def check_encrypted_boundary(exe_path):
    """Sanity-check the 0xFA000 boundary against the file's own entropy."""
    pe = load_pe(exe_path)
    text = section_info(pe, ".text")
    data = text["file_data"]
    before = round(shannon_entropy(data[ENCRYPTED_REL_END - PAGE:
                                         ENCRYPTED_REL_END]), 3)
    after = round(shannon_entropy(data[ENCRYPTED_REL_END:
                                       ENCRYPTED_REL_END + PAGE]), 3)
    ok = before > 7.5 > after
    return ok, before, after


def cmd_capture(args):
    out_dir = check_out_dir(args.out_dir or orig_dir())
    t0 = time.time()
    exe_path = args.exe or os.path.join(orig_dir(), "GTAIV.exe")

    ok, before, after = check_encrypted_boundary(exe_path)
    print("encrypted-boundary check: page before=%.3f after=%.3f %s"
          % (before, after, "OK" if ok else "MISMATCH - see report"))

    if args.pid is not None:
        pids = [args.pid]
    else:
        pids = find_processes_by_name(args.exe_name)
    if not pids:
        print("no running process named %s" % args.exe_name)
        return 1
    if len(pids) > 1 and args.pid is None:
        print("several %s processes: %s - rerun with --pid" % (args.exe_name,
                                                              pids))
        return 1
    pid = pids[0]
    print("capturing pid %d ..." % pid)

    report, captured, secinfo = capture_process(
        pid, exe_path, CAPTURE_SECTIONS, out_dir, ENCRYPTED_REL_END, "game")

    report["notes"].append(
        "encrypted-boundary file check: page before=%.3f after=%.3f %s"
        % (before, after, "OK" if ok else "MISMATCH"))

    # --- Write outputs (orig/ only) ----------------------------------
    text = secinfo[".text"]
    decrypted = bytes(captured[".text"][:ENCRYPTED_REL_END])
    p_dec = os.path.join(out_dir, "text_decrypted.bin")
    with open(p_dec, "wb") as f:
        f.write(decrypted)

    with open(exe_path, "rb") as f:
        image = bytearray(f.read())
    start = text["rawptr"]
    image[start:start + ENCRYPTED_REL_END] = decrypted
    p_img = os.path.join(out_dir, "GTAIV_decrypted.exe")
    with open(p_img, "wb") as f:
        f.write(image)
    report["notes"].append(
        "reconstructed image keeps the original PE checksum field unchanged; "
        "only .text[0:0xFA000] differs from the file.")

    report["outputs"] = {
        "decrypted_range": {"path": os.path.abspath(p_dec),
                            "bytes": len(decrypted),
                            "sha256": sha256_hex(decrypted)},
        "reconstructed_image": {"path": os.path.abspath(p_img),
                                "bytes": len(image),
                                "sha256": sha256_hex(bytes(image))},
    }
    report["timing"]["total_s"] = round(time.time() - t0, 2)
    p_rep = os.path.join(out_dir, "capture_report.json")
    with open(p_rep, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)

    # --- Console summary ----------------------------------------------
    print("base: preferred=%s actual=%s delta=%s"
          % (report["image"]["preferred_base"],
             report["image"]["actual_base"], report["image"]["delta"]))
    for s in report["sections"]:
        print("%s: %s pages, %s unreadable"
              % (s["name"], s["pages_total"], s["pages_unreadable"]))
    enc = report["encrypted_range"]
    if enc:
        pc = enc["plaintext_check"]
        print("plaintext .text outside encrypted range: %d diff bytes in %d "
              "compared (%s)" % (pc["diff_bytes"], pc["compared_bytes"],
                                 "EXACT MATCH" if pc["match"] else "DIFFERS"))
        print("encrypted range: file prologues=%s captured prologues=%s"
              % (enc["prologues_file"], enc["prologues_captured"]))
    print("modules loaded: %d" % len(report["modules"]))
    print("wrote %s, %s, %s in %.1fs"
          % (p_dec, p_img, p_rep, report["timing"]["total_s"]))
    return 0


# --------------------------------------------------------------------------
# Self-test: synthetic + live 32-bit sleeper, outputs under scratch only

SLEEPER_C = r"""/* h-capture self-test target: a tiny 32-bit program that idles so the
 * capture tool can read it. Our own code, written for this test. */
#include <windows.h>

static volatile unsigned long long counter = 0;

int main(void) {
    /* Idle ~90 s. Touch a global each second so .data is genuinely live. */
    for (int i = 0; i < 90; i++) {
        counter += (unsigned long long)(i + 1);
        Sleep(1000);
    }
    return (int)(counter & 1);
}
"""


def zig_exe():
    if os.environ.get("ZIG_EXE"):
        return os.environ["ZIG_EXE"]
    return os.path.join(repo_root(), "tools", "zig",
                        "zig-aarch64-windows-0.17.0", "zig.exe")


def build_sleeper(path):
    src = os.path.join(scratch_dir(), "sleeper.c")
    if not os.path.exists(src):
        with open(src, "w", encoding="utf-8", newline="\n") as f:
            f.write(SLEEPER_C)
    if os.path.exists(path):
        return "reused"
    cmd = [zig_exe(), "cc", "-target", "x86-windows-gnu", "-Os", "-s",
           src, "-o", path]
    r = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    if r.returncode != 0:
        raise RuntimeError("zig cc failed: %s %s" % (r.stdout, r.stderr))
    return "built"


def test_unrelocate_synthetic():
    """Round-trip: relocate a fake image, unrelocate it, compare."""
    random_sites = [0x100, 0x104, 0x2000, 0x3FFC]
    base = bytearray(0x4000)
    for i in range(0, 0x4000, 4):
        struct.pack_into("<I", base, i, 0x400000 + i)
    for delta in (0x0, 0x1234000, 0x7F000000, 0xFFFFFFFF):
        img = bytearray(base)
        for r in random_sites:
            v = struct.unpack_from("<I", img, r)[0]
            struct.pack_into("<I", img, r, (v + delta) & 0xFFFFFFFF)
        n = unrelocate(img, 0, random_sites, delta)
        assert n == len(random_sites), (delta, n)
        assert bytes(img) == bytes(base), \
            "round-trip failed, delta=0x%X" % delta
    return "synthetic unrelocate round-trip OK (4 deltas incl. 0/overflow)"


def cmd_selftest(_args):
    t0 = time.time()
    print("synthetic: %s" % test_unrelocate_synthetic())

    out_dir = check_out_dir(selftest_dir())
    sleeper = os.path.join(out_dir, "sleeper32.exe")
    print("sleeper: %s (%s)" % (build_sleeper(sleeper), sleeper))

    pe = load_pe(sleeper)
    mach = pe.FILE_HEADER.Machine
    chars = pe.OPTIONAL_HEADER.DllCharacteristics
    relocs = highlow_reloc_rvas(pe)
    print("sleeper PE: machine=0x%X dllchars=0x%X highlow=%d base=0x%X"
          % (mach, chars, len(relocs), pe.OPTIONAL_HEADER.ImageBase))
    if mach != 0x14C:
        print("SELFTEST FAIL: sleeper is not 32-bit x86")
        return 1

    sections = [n for n in CAPTURE_SECTIONS if section_info(pe, n)]
    print("sleeper sections to capture: %s" % sections)

    # Launch our own test program (NOT the game) and capture it, up to 3
    # tries so we will probably see a nonzero rebase delta at least once.
    attempts = []
    final = None
    for attempt in range(3):
        proc = subprocess.Popen([sleeper])
        try:
            time.sleep(1.5)
            if proc.poll() is not None:
                raise RuntimeError("sleeper exited early, rc=%s"
                                   % proc.poll())
            found = find_processes_by_name("sleeper32.exe")
            if proc.pid not in found:
                raise RuntimeError("find-by-name missed pid %d (found %s)"
                                   % (proc.pid, found))
            report, _cap, _sec = capture_process(
                proc.pid, sleeper, sections, out_dir, 0, "selftest")
            delta = int(report["image"]["delta"], 16)
            textv = report["verify"].get(".text", {})
            unread = sum(s["pages_unreadable"] for s in report["sections"])
            attempts.append({"pid": proc.pid,
                             "delta": report["image"]["delta"],
                             "text_diff": textv.get("diff_bytes"),
                             "unreadable": unread})
            print("attempt %d: pid=%d delta=%s .text diff=%s unreadable=%d"
                  % (attempt + 1, proc.pid, report["image"]["delta"],
                     textv.get("diff_bytes"), unread))
            if final is None or (delta != 0 and
                                 int(final["image"]["delta"], 16) == 0):
                final = report
            if delta != 0:
                break
        finally:
            proc.terminate()
            try:
                proc.wait(timeout=15)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait(timeout=15)

    final["selftest_attempts"] = attempts
    nonzero = any(int(a["delta"], 16) != 0 for a in attempts)
    final["notes"].append(
        "nonzero rebase delta observed: %s (unrelocate logic also covered "
        "by the synthetic round-trip)" % nonzero)
    final["timing"]["total_s"] = round(time.time() - t0, 2)
    p_rep = os.path.join(out_dir, "selftest_report.json")
    with open(p_rep, "w", encoding="utf-8") as f:
        json.dump(final, f, indent=2)

    textv = final["verify"].get(".text", {})
    unread = sum(s["pages_unreadable"] for s in final["sections"])
    ok = textv.get("match") and unread == 0
    print("modules seen in sleeper: %d (main=%s)"
          % (len(final["modules"]), final["modules"][0]["name"]))
    print("selftest report: %s" % p_rep)
    print("SELFTEST %s (%.1fs)"
          % ("PASS" if ok else "FAIL", final["timing"]["total_s"]))
    return 0 if ok else 1


# --------------------------------------------------------------------------


def main(argv):
    ap = argparse.ArgumentParser(description="capture a running 32-bit "
                                 "process's code for analysis (read-only)")
    sub = ap.add_subparsers(dest="cmd", required=True)
    c = sub.add_parser("capture", help="capture the running game -> orig/")
    c.add_argument("--exe", default=None,
                   help="reference executable (default: orig/GTAIV.exe)")
    c.add_argument("--exe-name", default=GAME_EXE_NAME)
    c.add_argument("--pid", type=int, default=None)
    c.add_argument("--out-dir", default=None)
    s = sub.add_parser("selftest", help="dry run against our own 32-bit "
                                        "sleeper; writes under .artifacts")
    args = ap.parse_args(argv)
    if args.cmd == "capture":
        return cmd_capture(args)
    return cmd_selftest(args)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))