from __future__ import annotations

"""Run one unchanged checker2 contract under local_slot with pinned evidence."""
import argparse
import ctypes
import hashlib
import importlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import sysconfig
import threading
import time
import tomllib
import uuid
from datetime import datetime, timezone


def repo_root(start: Path) -> Path:
    for parent in (start.resolve(), *start.resolve().parents):
        if (parent / "scripts/checker/checker2.py").is_file() and (parent / "AGENTS.md").is_file():
            return parent
    raise RuntimeError("cannot locate LibertyFlux repository root")


ROOT = repo_root(Path(__file__).resolve().parent)
DRIVER = ROOT / "scripts/checker/checker2.py"
SLOT = ROOT / "scripts/coordinator/local_slot.py"
BUILD = ROOT / ".artifacts/build/cargo-tools/i686-pc-windows-msvc/release"
STOCK_PATHS = {
    "driver": DRIVER,
    "local_slot": SLOT,
    "worker_source": ROOT / "crates/tools/lf-checker-worker/src/main.rs",
    "worker_manifest": ROOT / "crates/tools/lf-checker-worker/Cargo.toml",
    "worker_binary": BUILD / "lf-checker-worker.exe",
    "proof_source": ROOT / "crates/tools/lf-checker-proofs/src/lib.rs",
    "proof_manifest": ROOT / "crates/tools/lf-checker-proofs/Cargo.toml",
    "proof_binary": BUILD / "lf_checker_proofs.dll",
    "original_exe": ROOT / "orig/GTAIV.exe",
}
ENV_KEYS = (
    "LF_CHECKER_BUILD_DIR", "LF_CHECKER_WORKER", "LF_CHECKER_DLL",
    "K2_DLL", "LIBERTYFLUX_ORIG_EXE", "LF_CHECKER_OUT",
)
EXIT_EVIDENCE = 4
EXIT_NEGATIVE = 5
EXPECTED_STOCK_DRIVER_SHA256 = "024a01985ff5f85751087d8a0e8709fa22de18d04d1e398a02a8293f225968fa"
BUILD_TARGET = "i686-pc-windows-msvc"


def now() -> str:
    return datetime.now(timezone.utc).isoformat()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def sha256_json(value: object) -> str:
    data = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return hashlib.sha256(data).hexdigest()


def relative(path: Path) -> str:
    return Path(path).resolve().relative_to(ROOT.resolve()).as_posix()


def evidence_path(path: Path) -> str:
    path = Path(path).resolve()
    try:
        return relative(path)
    except ValueError:
        return str(path)


def write_json(path: Path, value: object) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    tmp.replace(path)


def source_tree(source_root: Path) -> dict:
    source_root = source_root.resolve()
    rows = []
    for path in sorted(p for p in source_root.rglob("*") if p.is_file() and p.suffix in {".rs", ".toml", ".lock"}):
        rows.append({"path": evidence_path(path), "sha256": sha256_file(path)})
    if not any(Path(row["path"]).suffix == ".rs" for row in rows):
        raise ValueError("candidate source tree must contain at least one Rust source file")
    digest = hashlib.sha256("\n".join(f"{row['path']} {row['sha256']}" for row in rows).encode()).hexdigest()
    return {"root": evidence_path(source_root), "sha256": digest, "files": rows}


def environment_fingerprint(env: dict[str, str]) -> str:
    rows = sorted(((key, value) for key, value in env.items()), key=lambda pair: pair[0].casefold())
    return sha256_json(rows)


def effective_environment(base: dict[str, str], output: Path) -> dict[str, str]:
    env = dict(base)
    env["LF_CHECKER_BUILD_DIR"] = str(BUILD.resolve())
    env["LF_CHECKER_WORKER"] = str(STOCK_PATHS["worker_binary"].resolve())
    env["LF_CHECKER_DLL"] = str(STOCK_PATHS["proof_binary"].resolve())
    env.pop("K2_DLL", None)
    env["LIBERTYFLUX_ORIG_EXE"] = str(STOCK_PATHS["original_exe"].resolve())
    env["LF_CHECKER_OUT"] = str(output.resolve())
    return env


def require_owned_path(path: Path, owner: Path, label: str) -> Path:
    resolved = Path(path).resolve()
    try:
        resolved.relative_to(owner.resolve())
    except ValueError as exc:
        raise ValueError(f"{label} must stay inside the lane scratch folder") from exc
    return resolved


def contract_dll_path(contract: dict) -> Path:
    value = contract.get("dll")
    if not isinstance(value, str) or not value:
        raise ValueError("contract must pin its candidate DLL in the dll field")
    path = Path(value)
    return (path if path.is_absolute() else ROOT / path).resolve()



IMAGE_SCN_MEM_EXECUTE = 0x20000000


def _pe_slice(data: bytes, offset: int, size: int, label: str) -> bytes:
    if offset < 0 or size < 0 or offset + size > len(data):
        raise ValueError(f"original PE is truncated while reading {label}")
    return data[offset:offset + size]


def _pe_u16(data: bytes, offset: int, label: str) -> int:
    return int.from_bytes(_pe_slice(data, offset, 2, label), "little")


def _pe_u32(data: bytes, offset: int, label: str) -> int:
    return int.from_bytes(_pe_slice(data, offset, 4, label), "little")


def _read_pe_image(path: Path) -> tuple[bytes, dict]:
    try:
        data = Path(path).read_bytes()
    except OSError as exc:
        raise ValueError(f"cannot read original PE {path}: {exc}") from exc
    if _pe_slice(data, 0, 2, "DOS signature") != b"MZ":
        raise ValueError("original executable has no MZ signature")
    pe_offset = _pe_u32(data, 0x3C, "PE header offset")
    if _pe_slice(data, pe_offset, 4, "PE signature") != b"PE\0\0":
        raise ValueError("original executable has no PE signature")
    section_count = _pe_u16(data, pe_offset + 6, "section count")
    optional_size = _pe_u16(data, pe_offset + 20, "optional-header size")
    optional = pe_offset + 24
    magic = _pe_u16(data, optional, "optional-header magic")
    if magic == 0x10B:
        image_base = _pe_u32(data, optional + 28, "PE32 ImageBase")
    elif magic == 0x20B:
        image_base = int.from_bytes(_pe_slice(data, optional + 24, 8, "PE32+ ImageBase"), "little")
    else:
        raise ValueError(f"unsupported PE optional-header magic 0x{magic:x}")
    if optional_size < 64:
        raise ValueError("original PE optional header is too short")
    image_size = _pe_u32(data, optional + 56, "SizeOfImage")
    headers_size = _pe_u32(data, optional + 60, "SizeOfHeaders")
    section_table = optional + optional_size
    sections = []
    for index in range(section_count):
        header = section_table + index * 40
        raw = _pe_slice(data, header, 40, f"section header {index}")
        name = raw[:8].split(b"\0", 1)[0].decode("ascii", errors="replace")
        virtual_size = int.from_bytes(raw[8:12], "little")
        virtual_address = int.from_bytes(raw[12:16], "little")
        raw_size = int.from_bytes(raw[16:20], "little")
        raw_offset = int.from_bytes(raw[20:24], "little")
        characteristics = int.from_bytes(raw[36:40], "little")
        mapped_size = max(virtual_size, raw_size)
        if mapped_size:
            sections.append({
                "name": name, "rva_start": virtual_address,
                "rva_end": virtual_address + mapped_size,
                "raw_size": raw_size, "raw_offset": raw_offset,
                "characteristics": characteristics,
                "executable": bool(characteristics & IMAGE_SCN_MEM_EXECUTE),
            })
    return data, {
        "path": evidence_path(Path(path)), "image_base": image_base,
        "image_size": image_size, "headers_size": headers_size,
        "sections": sections,
    }


def _contract_hex(value: object, field: str, *, allow_integer: bool = False) -> int:
    if isinstance(value, bool):
        raise ValueError(f"{field} must be a hexadecimal address")
    if allow_integer and isinstance(value, int):
        number = value
    elif isinstance(value, str):
        try:
            number = int(value, 16)
        except ValueError as exc:
            raise ValueError(f"{field} must be a hexadecimal address") from exc
    else:
        raise ValueError(f"{field} must be a hexadecimal address")
    if number < 0:
        raise ValueError(f"{field} must be nonnegative")
    return number


def _mapped_executable_bytes(data: bytes, pe: dict, rva: int, size: int,
                             label: str) -> tuple[bytes, dict]:
    if rva + size > pe["image_size"]:
        raise ValueError(f"{label} RVA 0x{rva:x} is outside SizeOfImage")
    matches = [section for section in pe["sections"]
               if section["rva_start"] <= rva and rva + size <= section["rva_end"]]
    if len(matches) != 1:
        raise ValueError(f"{label} RVA 0x{rva:x} does not map to exactly one PE section")
    section = matches[0]
    if not section["executable"]:
        raise ValueError(f"{label} RVA 0x{rva:x} is in non-executable section {section['name']}")
    delta = rva - section["rva_start"]
    if delta + size > section["raw_size"]:
        raise ValueError(f"{label} RVA 0x{rva:x} is not backed by file bytes in {section['name']}")
    file_offset = section["raw_offset"] + delta
    code = _pe_slice(data, file_offset, size, f"{label} bytes")
    return code, {"name": section["name"], "rva_start": f"0x{section['rva_start']:x}",
                  "file_offset": f"0x{file_offset:x}",
                  "characteristics": f"0x{section['characteristics']:08x}"}


def _patch_rows(contract: dict, data: bytes, pe: dict) -> list[dict]:
    rows = []
    for key, opcode, width in (("patches", b"\xE8", 5), ("tailpatches", b"\xE9", 5)):
        entries = contract.get(key, [])
        if not isinstance(entries, list):
            raise ValueError(f"{key} must be a list")
        for index, entry in enumerate(entries):
            if not isinstance(entry, dict):
                raise ValueError(f"{key}[{index}] must be an object")
            if "site_selftest" in entry:
                rows.append({"kind": key, "index": index,
                             "site_selftest": entry["site_selftest"],
                             "validation": "stock self-test site; no game RVA"})
                continue
            if "site" not in entry:
                raise ValueError(f"{key}[{index}] needs site or site_selftest")
            rva = _contract_hex(entry["site"], f"{key}[{index}].site", allow_integer=True)
            code, section = _mapped_executable_bytes(data, pe, rva, width, f"{key}[{index}]")
            if code[:1] != opcode:
                raise ValueError(
                    f"{key}[{index}] RVA 0x{rva:x} has opcode 0x{code[0]:02X}; "
                    f"expected 0x{opcode[0]:02X}")
            rows.append({"kind": key, "index": index, "site_rva": f"0x{rva:x}",
                         "opcode": f"0x{code[0]:02x}", "patch_width": width,
                         "section": section["name"],
                         "bytes_sha256": hashlib.sha256(code).hexdigest()})
    entries = contract.get("ctailpatches", [])
    if not isinstance(entries, list):
        raise ValueError("ctailpatches must be a list")
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValueError(f"ctailpatches[{index}] must be an object")
        if "site_selftest" in entry:
            rows.append({"kind": "ctailpatches", "index": index,
                         "site_selftest": entry["site_selftest"],
                         "validation": "stock self-test site; no game RVA"})
            continue
        if "site" not in entry:
            raise ValueError(f"ctailpatches[{index}] needs site or site_selftest")
        rva = _contract_hex(entry["site"], f"ctailpatches[{index}].site", allow_integer=True)
        code, section = _mapped_executable_bytes(data, pe, rva, 6, f"ctailpatches[{index}]")
        if code[0] != 0x0F or not 0x80 <= code[1] <= 0x8F:
            raise ValueError(f"ctailpatches[{index}] RVA 0x{rva:x} is not a near Jcc (0F 80..8F)")
        rows.append({"kind": "ctailpatches", "index": index, "site_rva": f"0x{rva:x}",
                     "opcode": f"0x{code[0]:02x} 0x{code[1]:02x}", "patch_width": 6,
                     "section": section["name"],
                     "bytes_sha256": hashlib.sha256(code).hexdigest()})
    return rows


def validate_contract_code(contract: dict, original_exe: Path) -> dict:
    callees = contract.get("callees", [])
    if not isinstance(callees, list):
        raise ValueError("callees must be a list")
    for index, callee in enumerate(callees):
        if not isinstance(callee, dict):
            raise ValueError(f"callees[{index}] must be an object")
        script = callee.get("script")
        if script == "edges":
            pass
        elif not isinstance(script, list) or not script:
            raise ValueError(f"callee {callee.get('id', index)} needs a nonempty script list or 'edges'")
        else:
            for script_index, entry in enumerate(script):
                if not isinstance(entry, (int, dict)):
                    raise ValueError(
                        f"callee {callee.get('id', index)} script[{script_index}] must be an int or object")

        # resolve_scripts only skips a missing/None wscript. Any present value
        # is cycled by trial and each selected row is iterated as word specs.
        wscript = callee.get("wscript")
        if wscript is not None:
            if not isinstance(wscript, list) or not wscript:
                raise ValueError(
                    f"callee {callee.get('id', index)} wscript must be a nonempty list when supplied")
            for row_index, row in enumerate(wscript):
                if not isinstance(row, list):
                    raise ValueError(
                        f"callee {callee.get('id', index)} wscript[{row_index}] must be a list")
                for word_index, word in enumerate(row):
                    if not isinstance(word, (int, dict)):
                        raise ValueError(
                            f"callee {callee.get('id', index)} wscript[{row_index}][{word_index}] "
                            "must be an int or object")

    data, pe = _read_pe_image(original_exe)
    function = contract.get("function")
    result = {"original_exe": pe["path"], "image_base": f"0x{pe['image_base']:x}",
              "image_size": f"0x{pe['image_size']:x}"}
    if isinstance(function, str) and function.startswith("selftest:"):
        if "expected_function_va" in contract:
            raise ValueError("expected_function_va cannot be checked for a selftest function")
        result.update({"function_kind": "checker_selftest", "function": function})
    else:
        if not isinstance(function, str):
            raise ValueError("contract.function must be a hexadecimal RVA string")
        function_rva = _contract_hex(function, "contract.function")
        code, section = _mapped_executable_bytes(data, pe, function_rva, 1, "contract.function")
        function_va = pe["image_base"] + function_rva
        if function_va >= 1 << (64 if pe["image_base"] > 0xFFFFFFFF else 32):
            raise ValueError("contract.function VA overflows the PE address width")
        expected_va = contract.get("expected_function_va")
        expected_va_hex = None
        if expected_va is not None:
            expected_va_value = _contract_hex(expected_va, "expected_function_va", allow_integer=True)
            expected_va_hex = f"0x{expected_va_value:x}"
            if expected_va_value - pe["image_base"] != function_rva:
                raise ValueError(
                    f"expected_function_va {expected_va_hex} minus ImageBase {result['image_base']} "
                    f"does not equal contract.function RVA 0x{function_rva:x}")
        result.update({"function_kind": "mapped_pe_rva", "function_rva": f"0x{function_rva:x}",
                       "function_va": f"0x{function_va:x}", "expected_function_va": expected_va_hex,
                       "section": section["name"], "file_offset": section["file_offset"],
                       "entry_byte_sha256": hashlib.sha256(code).hexdigest()})
    result["patch_sites"] = _patch_rows(contract, data, pe)
    return result


def make_manifest(mode: str, lane: str, python_exe: Path, run_dir: Path,
                  contract_dir: Path | None = None, contract_name: str | None = None,
                  candidate_source_root: Path | None = None, candidate_dll: Path | None = None,
                  trials: int | None = None) -> tuple[dict, dict[str, str]]:
    if not python_exe.is_file():
        raise ValueError("an explicit existing Python executable is required")
    if trials is not None and trials <= 0:
        raise ValueError("--trials must be positive")
    if not re.fullmatch(r"[A-Za-z0-9_-]+", lane):
        raise ValueError("lane must be a simple scratch-folder name")
    lane_root = (ROOT / ".artifacts" / "scratch" / lane).resolve()
    run_dir = require_owned_path(run_dir, lane_root, "run directory")
    if run_dir.exists() and any(run_dir.iterdir()):
        raise ValueError("run directory must be empty before preflight")
    run_dir.mkdir(parents=True, exist_ok=True)
    out_dir = run_dir / "out"
    paths = {}
    for role, path in STOCK_PATHS.items():
        if not path.is_file():
            raise FileNotFoundError(f"missing stock input: {role}")
        paths[role] = {"path": relative(path), "sha256": sha256_file(path)}
    if paths["driver"]["sha256"] != EXPECTED_STOCK_DRIVER_SHA256:
        raise ValueError("canonical checker2.py does not match the reviewed stock SHA-256")
    candidate = None
    contract_record = None
    source = None
    if mode == "run":
        if contract_dir is None or contract_dir.name != "contracts":
            raise ValueError("contract directory must be named contracts")
        contract_dir = require_owned_path(contract_dir, lane_root, "contract directory")
        if not contract_name or not re.fullmatch(r"[A-Za-z0-9_.-]+", contract_name.removesuffix(".json")):
            raise ValueError("contract must be supplied as a basename")
        contract_name = contract_name.removesuffix(".json")
        contract_path = contract_dir / f"{contract_name}.json"
        raw = contract_path.read_bytes()
        contract_obj = json.loads(raw.decode("utf-8-sig"))
        stock_name = contract_obj.get("name")
        if (not isinstance(stock_name, str) or stock_name != contract_name
                or not re.fullmatch(r"[A-Za-z0-9_.-]+", stock_name)):
            raise ValueError("contract name must equal its safe selected basename")
        if candidate_source_root is None or candidate_dll is None:
            raise ValueError("candidate source root and DLL are required")
        candidate_source_root = require_owned_path(candidate_source_root, lane_root, "candidate source")
        candidate_dll = require_owned_path(candidate_dll, lane_root, "candidate DLL")
        if not candidate_dll.is_file():
            raise FileNotFoundError("candidate DLL is missing")
        if contract_dll_path(contract_obj) != candidate_dll:
            raise ValueError("contract dll field does not resolve to the pinned candidate DLL")
        function_validation = validate_contract_code(contract_obj, STOCK_PATHS["original_exe"])
        source = source_tree(candidate_source_root)
        candidate = {"path": relative(candidate_dll), "sha256": sha256_file(candidate_dll)}
        contract_record = {
            "name": contract_name,
            "path": relative(contract_path),
            "sha256": hashlib.sha256(raw).hexdigest(),
            "contract_name": contract_obj.get("name"),
            "mut_export": contract_obj.get("mut_export"),
            "dll": relative(candidate_dll),
            "function_validation": function_validation,
        }
        if not contract_obj.get("mut_export"):
            raise ValueError("contract must include a same-contract mut_export")
    elif mode != "selftest":
        raise ValueError(f"unsupported mode: {mode}")
    env = effective_environment(os.environ.copy(), out_dir)
    manifest = {
        "schema": "common-stock-runner-v1",
        "mode": mode,
        "lane": lane,
        "recorded_before_local_slot_utc": now(),
        "python": {"path": relative(python_exe), "sha256": sha256_file(python_exe)},
        "stock_inputs": paths,
        "candidate_source": source,
        "candidate_dll": candidate,
        "contract": contract_record,
        "trials_override": trials,
        "run_dir": relative(run_dir),
        "output_dir": relative(out_dir),
        "effective_environment": {key: env.get(key) for key in ENV_KEYS},
        "effective_environment_sha256": environment_fingerprint(env),
        "environment_keys": sorted(env, key=str.casefold),
    }
    return manifest, env


def verify_manifest(manifest: dict, env: dict[str, str]) -> list[str]:
    errors = []
    if environment_fingerprint(env) != manifest["effective_environment_sha256"]:
        errors.append("effective environment changed after preflight")
    python_row = manifest["python"]
    try:
        python_hash = sha256_file(ROOT / python_row["path"])
    except OSError:
        python_hash = None
    if python_hash != python_row["sha256"]:
        errors.append("explicit Python executable changed")
    for key, row in manifest["stock_inputs"].items():
        path = ROOT / row["path"]
        try:
            actual = sha256_file(path)
        except OSError:
            actual = None
        if actual != row["sha256"]:
            errors.append(f"stock input changed: {key}")
        if key == "driver" and actual != EXPECTED_STOCK_DRIVER_SHA256:
            errors.append("canonical checker2.py is not the reviewed stock driver")
    if manifest.get("candidate_dll"):
        row = manifest["candidate_dll"]
        if sha256_file(ROOT / row["path"]) != row["sha256"]:
            errors.append("candidate DLL changed")
    if manifest.get("candidate_source"):
        src = manifest["candidate_source"]
        try:
            current = source_tree(ROOT / src["root"])
        except Exception:
            current = {"sha256": None}
        if current["sha256"] != src["sha256"] or current["files"] != src["files"]:
            errors.append("candidate source tree changed")
    if manifest.get("contract"):
        row = manifest["contract"]
        contract_path = ROOT / row["path"]
        if sha256_file(contract_path) != row["sha256"]:
            errors.append("contract changed")
        else:
            try:
                contract_obj = json.loads(contract_path.read_text(encoding="utf-8-sig"))
                current_validation = validate_contract_code(contract_obj, STOCK_PATHS["original_exe"])
                if current_validation != row.get("function_validation"):
                    errors.append("contract mapped function or patch sites changed")
            except (OSError, ValueError, json.JSONDecodeError) as exc:
                errors.append(f"contract PE preflight failed: {exc}")
    return errors


def pid_chain(pid: int, parents: dict[int, int], max_depth: int = 32) -> list[int]:
    chain, seen = [], set()
    current = int(pid)
    for _ in range(max_depth):
        if current in seen:
            break
        chain.append(current)
        seen.add(current)
        parent = parents.get(current)
        if not parent or parent == current:
            break
        current = int(parent)
    return chain


def slot_ancestry_evidence(launch_pid: int, owner_pid: int | None, child_pid: int | None,
                           observed_chains: dict[int, list[int]]) -> dict:
    """Verify the local_slot owner and child through any Windows intermediates."""
    owner_chain = observed_chains.get(owner_pid) if owner_pid is not None else None
    child_chain = observed_chains.get(child_pid) if child_pid is not None else None
    owner_is_child_ancestor = bool(child_chain and owner_pid in child_chain[1:])
    launch_is_owner_ancestor = bool(owner_chain and launch_pid in owner_chain[1:])
    return {
        "slot_owner_verified_ancestor_of_child": owner_is_child_ancestor,
        "outer_launch_verified_ancestor_of_slot_owner": launch_is_owner_ancestor,
        "slot_owner_ancestor_chain_sample": owner_chain,
        "child_ancestor_chain_sample": child_chain,
        "ancestry_ok": owner_is_child_ancestor and launch_is_owner_ancestor,
    }


def slot_transcript_evidence(text: str, launched_wrapper_pid: int | None, wrapper_exit: int) -> dict:
    acquired = re.findall(r"acquired slot (\d+); wrapper PID (\d+)", text)
    children = re.findall(r"child PID (\d+)", text)
    released = re.findall(r"released slot; exit (-?\d+)", text)
    owner = int(acquired[-1][1]) if acquired else None
    child = int(children[-1]) if children else None
    release_exit = int(released[-1]) if released else None
    release_ok = release_exit == int(wrapper_exit)
    if not released and "launch failed before child creation; slot released" in text:
        release_ok = True
    return {
        "slot": acquired[-1][0] if acquired else None,
        "owner_pid_from_transcript": owner,
        "outer_launch_pid": int(launched_wrapper_pid) if launched_wrapper_pid is not None else None,
        "child_pid_from_transcript": child,
        "release_exit_from_transcript": release_exit,
        "release_matches_wrapper_exit": release_ok,
        "release_verified_from_this_transcript": bool(owner is not None and release_ok),
    }


def cargo_library_name(manifest_path: Path) -> tuple[str, str]:
    try:
        document = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        raise ValueError(f"cannot read Cargo manifest: {exc}") from exc
    package = document.get("package") or {}
    library = document.get("lib") or {}
    package_name = package.get("name")
    crate_types = library.get("crate-type") or []
    library_name = library.get("name") or (package_name.replace("-", "_") if isinstance(package_name, str) else "")
    if not isinstance(package_name, str) or not re.fullmatch(r"[A-Za-z0-9_-]+", package_name):
        raise ValueError("Cargo package name must be present and safe")
    if not isinstance(library_name, str) or not re.fullmatch(r"[A-Za-z0-9_]+", library_name):
        raise ValueError("Cargo library name must be present and safe")
    if not isinstance(crate_types, list) or "cdylib" not in crate_types:
        raise ValueError("Cargo manifest must declare a cdylib library target")
    return package_name, library_name


_CARGO_DEPENDENCY_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def _read_cargo_manifest(path: Path) -> tuple[dict | None, str | None]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8")), None
    except (OSError, tomllib.TOMLDecodeError) as exc:
        return None, f"cannot read {evidence_path(path)}: {exc}"


def _workspace_manifest(manifest_path: Path, document: dict) -> tuple[Path, dict]:
    if isinstance(document.get("workspace"), dict):
        return manifest_path, document
    for parent in manifest_path.parent.parents:
        candidate = parent / "Cargo.toml"
        if not candidate.is_file():
            continue
        ancestor, error = _read_cargo_manifest(candidate)
        if error is None and isinstance(ancestor.get("workspace"), dict):
            return candidate, ancestor
    return manifest_path, document


def _dependency_sections(document: dict) -> tuple[list[dict], list[str]]:
    sections = []
    errors = []
    for key in _CARGO_DEPENDENCY_TABLES:
        value = document.get(key)
        if value is None:
            continue
        if not isinstance(value, dict):
            errors.append(f"{key} is not a TOML table")
        else:
            sections.append(value)
    targets = document.get("target")
    if targets is not None:
        if not isinstance(targets, dict):
            errors.append("target dependencies are not a TOML table")
        else:
            for condition, target in targets.items():
                if not isinstance(target, dict):
                    errors.append(f"target {condition!r} dependencies are not a TOML table")
                    continue
                for key in _CARGO_DEPENDENCY_TABLES:
                    value = target.get(key)
                    if value is None:
                        continue
                    if not isinstance(value, dict):
                        errors.append(f"target {condition!r} {key} is not a TOML table")
                    else:
                        sections.append(value)
    return sections, errors


def _unsupported_cargo_overrides(document: dict, manifest_path: Path) -> list[str]:
    errors = []
    patches = document.get("patch")
    if patches is not None:
        if not isinstance(patches, dict):
            errors.append(f"{evidence_path(manifest_path)}: Cargo [patch.*] is not a TOML table")
        elif any(bool(entries) for entries in patches.values()):
            errors.append(f"{evidence_path(manifest_path)}: Cargo [patch.*] overrides are not modeled")
    replacements = document.get("replace")
    if replacements is not None and replacements:
        errors.append(f"{evidence_path(manifest_path)}: Cargo [replace] overrides are not modeled")
    return errors


def build_candidate_source(manifest_path: Path) -> dict:
    """Hash the candidate and every discoverable transitive local path dependency."""
    manifest_path = manifest_path.resolve()
    root_source = source_tree(manifest_path.parent)
    pending = [manifest_path]
    visited: set[Path] = set()
    dependency_manifests: set[Path] = set()
    workspace_manifests: set[Path] = set()
    workspace_locks: list[dict] = []
    errors: set[str] = set()
    while pending:
        current = pending.pop().resolve()
        if current in visited:
            continue
        visited.add(current)
        document, error = _read_cargo_manifest(current)
        if error is not None:
            errors.add(error)
            continue
        workspace_manifest, workspace_document = _workspace_manifest(current, document)
        if workspace_manifest != current:
            workspace_manifests.add(workspace_manifest)
        errors.update(_unsupported_cargo_overrides(document, current))
        errors.update(_unsupported_cargo_overrides(workspace_document, workspace_manifest))
        sections, section_errors = _dependency_sections(document)
        errors.update(f"{evidence_path(current)}: {message}" for message in section_errors)
        for section in sections:
            for alias, raw in section.items():
                specs = raw if isinstance(raw, list) else [raw]
                for spec in specs:
                    if not isinstance(spec, dict):
                        continue
                    if spec.get("workspace") is True:
                        workspace_table = workspace_document.get("workspace") or {}
                        workspace_dependencies = (workspace_table.get("dependencies") or {}
                                                  if isinstance(workspace_table, dict) else {})
                        shared = workspace_dependencies.get(alias) if isinstance(workspace_dependencies, dict) else None
                        if shared is None:
                            errors.add(f"{evidence_path(current)}: unresolved workspace dependency {alias!r}")
                            continue
                        specs_to_check = shared if isinstance(shared, list) else [shared]
                    else:
                        specs_to_check = [spec]
                    for path_spec in specs_to_check:
                        if not isinstance(path_spec, dict):
                            continue
                        dep_path = path_spec.get("path")
                        if dep_path is None:
                            continue
                        if not isinstance(dep_path, str) or not dep_path:
                            errors.add(f"{evidence_path(current)}: dependency {alias!r} has an invalid path")
                            continue
                        path_base = workspace_manifest.parent if spec.get("workspace") is True else current.parent
                        dep_manifest = (path_base / dep_path / "Cargo.toml").resolve()
                        if not dep_manifest.is_file():
                            errors.add(f"{evidence_path(current)}: local path dependency {alias!r} has no Cargo.toml at {evidence_path(dep_manifest)}")
                            continue
                        if dep_manifest != manifest_path:
                            dependency_manifests.add(dep_manifest)
                        if dep_manifest not in visited:
                            pending.append(dep_manifest)

    dependency_sources = []
    for path in sorted(dependency_manifests, key=lambda item: str(item).casefold()):
        try:
            dependency_sources.append(source_tree(path.parent))
        except Exception as exc:
            errors.add(f"cannot hash local path dependency {evidence_path(path)}: {exc}")
    workspace_inputs = []
    for path in sorted(workspace_manifests, key=lambda item: str(item).casefold()):
        try:
            workspace_inputs.append({"path": evidence_path(path), "sha256": sha256_file(path)})
            lock_path = path.parent / "Cargo.lock"
            workspace_locks.append({
                "path": evidence_path(lock_path),
                "present": lock_path.is_file(),
                "sha256": sha256_file(lock_path) if lock_path.is_file() else None,
            })
        except OSError as exc:
            errors.add(f"cannot hash Cargo workspace manifest {evidence_path(path)}: {exc}")
    complete = not errors
    dependency_record = {
        "status": "complete" if complete else "unknown",
        "manifests": [evidence_path(path) for path in sorted(dependency_manifests, key=lambda item: str(item).casefold())],
        "sources": dependency_sources,
        "errors": sorted(errors),
    }
    snapshot = {"root": root_source, "local_path_dependencies": dependency_record,
                "workspace_manifests": workspace_inputs,
                "workspace_locks": workspace_locks,
                "status": "complete" if complete else "unknown"}
    snapshot["sha256"] = sha256_json(snapshot) if complete else None
    return snapshot


def make_build_plan(lane: str, python_exe: Path, cargo_manifest: Path) -> dict:
    if not re.fullmatch(r"[A-Za-z0-9_-]+", lane):
        raise ValueError("lane must be a simple scratch-folder name")
    python_exe = Path(python_exe)
    cargo_manifest = Path(cargo_manifest)
    if not python_exe.is_absolute():
        python_exe = ROOT / python_exe
    if not cargo_manifest.is_absolute():
        cargo_manifest = ROOT / cargo_manifest
    python_exe = python_exe.resolve()
    if not python_exe.is_file():
        raise FileNotFoundError("an explicit existing Python executable is required")
    lane_root = (ROOT / ".artifacts" / "scratch" / lane).resolve()
    manifest_path = require_owned_path(cargo_manifest, lane_root, "Cargo manifest")
    if manifest_path.name != "Cargo.toml":
        raise ValueError("explicit build manifest must be named Cargo.toml")
    if not manifest_path.is_file():
        raise FileNotFoundError(f"Cargo manifest must exist before local_slot: {manifest_path}")
    package_name, library_name = cargo_library_name(manifest_path)
    sources = build_candidate_source(manifest_path)
    cargo = shutil.which("cargo")
    if not cargo or not Path(cargo).is_file():
        raise FileNotFoundError("cargo executable was not found before local_slot")
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    build_id = f"{stamp}-{(sources['sha256'] or sources['root']['sha256'])[:10]}-{uuid.uuid4().hex[:8]}"
    build_dir = lane_root / "build-runs" / build_id
    target_dir = ROOT / ".artifacts" / "build" / "lanes" / lane / build_id
    generation_dir = lane_root / "generations" / build_id
    expected_dll = target_dir / BUILD_TARGET / "release" / f"{library_name}.dll"
    generation_dll = generation_dir / f"{library_name}.dll"
    return {
        "schema": "common-stock-build-plan-v1",
        "lane": lane,
        "build_id": build_id,
        "created_utc": now(),
        "python": {"path": str(python_exe), "sha256": sha256_file(python_exe)},
        "cargo_executable": str(Path(cargo).resolve()),
        "cargo_manifest": relative(manifest_path),
        "cargo_manifest_sha256": sha256_file(manifest_path),
        "package_name": package_name,
        "library_name": library_name,
        "candidate_source": sources,
        "target": BUILD_TARGET,
        "target_dir": relative(target_dir),
        "expected_dll": relative(expected_dll),
        "generation_dir": relative(generation_dir),
        "generation_dll": relative(generation_dll),
        "build_dir": relative(build_dir),
    }


def build_cargo_argv(plan: dict) -> list[str]:
    return [
        plan["cargo_executable"], "build",
        "--manifest-path", str((ROOT / plan["cargo_manifest"]).resolve()),
        "--target-dir", str((ROOT / plan["target_dir"]).resolve()),
        "--jobs", "1", "--target", BUILD_TARGET, "--release",
    ]


def build_local_slot_argv(plan: dict) -> list[str]:
    return [
        str(Path(plan["python"]["path"]).resolve()), str(SLOT.resolve()),
        "--lane", plan["lane"], "--cwd", str(ROOT.resolve()), "--",
        *build_cargo_argv(plan),
    ]


def run_build(lane: str, python_exe: Path, cargo_manifest: Path) -> int:
    # Resolve and validate the explicit owned Cargo.toml before starting local_slot.
    plan = make_build_plan(lane, python_exe, cargo_manifest)
    build_dir = require_owned_path(ROOT / plan["build_dir"], ROOT / ".artifacts/scratch" / lane,
                                  "build evidence directory")
    build_dir.mkdir(parents=True, exist_ok=False)
    plan["cargo_argv"] = build_cargo_argv(plan)
    plan["local_slot_argv"] = build_local_slot_argv(plan)
    plan_path = build_dir / "build_plan.json"
    write_json(plan_path, plan)
    transcript_path = build_dir / "local_slot.log"
    result_path = build_dir / "build_result.json"
    result = {
        "schema": "common-stock-build-result-v1", "lane": lane,
        "build_id": plan["build_id"],
        "completed_utc": now(), "local_slot_exit_code": None,
        "local_slot": None, "build_plan": relative(plan_path),
        "local_slot_transcript": None, "local_slot_transcript_sha256": None,
        "cargo_started": False, "cargo_exit_code": None,
        "generation_dll": None, "dll_sha256": None,
        "copy_hash_verified": False,
    }
    if plan["candidate_source"].get("status") != "complete":
        result["status"] = "path_dependency_hash_unknown"
        result["path_dependency_hash_errors"] = plan["candidate_source"]["local_path_dependencies"].get("errors", [])
        write_json(result_path, result)
        return 3
    proc = subprocess.run(plan["local_slot_argv"], cwd=ROOT, stdout=subprocess.PIPE,
                          stderr=subprocess.STDOUT, text=True, encoding="utf-8",
                          errors="replace", check=False)
    transcript = proc.stdout or ""
    transcript_path.write_text(transcript, encoding="utf-8")
    slot = slot_transcript_evidence(transcript, None, proc.returncode)
    cargo_started = slot["child_pid_from_transcript"] is not None
    cargo_exit_known = cargo_started and slot["release_verified_from_this_transcript"]
    result.update({
        "local_slot_exit_code": proc.returncode,
        "local_slot": slot,
        "local_slot_transcript": relative(transcript_path),
        "local_slot_transcript_sha256": sha256_file(transcript_path),
        "cargo_started": cargo_started,
        "cargo_exit_code": proc.returncode if cargo_exit_known else None,
    })
    if not cargo_started:
        result["status"] = "local_slot_failed_before_cargo"
        result["failure_detail"] = "no child PID was recorded by this local_slot transcript"
        write_json(result_path, result)
        return proc.returncode if proc.returncode else 3
    if not slot["release_verified_from_this_transcript"]:
        result["status"] = "local_slot_incomplete_after_cargo_start"
        write_json(result_path, result)
        return proc.returncode if proc.returncode else 3
    if proc.returncode != 0:
        result["status"] = "cargo_failed"
        write_json(result_path, result)
        return proc.returncode
    manifest_path = ROOT / plan["cargo_manifest"]
    if not manifest_path.is_file() or sha256_file(manifest_path) != plan["cargo_manifest_sha256"]:
        result["status"] = "cargo_manifest_changed"
        write_json(result_path, result)
        return 3
    try:
        source_after = build_candidate_source(manifest_path)
    except Exception:
        source_after = None
    if source_after != plan["candidate_source"]:
        result["status"] = "candidate_source_changed"
        write_json(result_path, result)
        return 3
    built_dll = ROOT / plan["expected_dll"]
    if not built_dll.is_file():
        result["status"] = "expected_dll_missing"
        write_json(result_path, result)
        return 3
    source_hash = sha256_file(built_dll)
    generation_dir = require_owned_path(ROOT / plan["generation_dir"],
                                        ROOT / ".artifacts/scratch" / lane,
                                        "DLL generation directory")
    generation_dir.mkdir(parents=True, exist_ok=False)
    generation_dll = require_owned_path(ROOT / plan["generation_dll"], generation_dir,
                                        "preserved candidate DLL")
    shutil.copy2(built_dll, generation_dll)
    copied_hash = sha256_file(generation_dll)
    result.update({
        "status": "built_and_preserved" if copied_hash == source_hash else "copy_hash_mismatch",
        "built_dll": relative(built_dll), "built_dll_sha256": source_hash,
        "generation_dll": relative(generation_dll), "dll_sha256": copied_hash,
        "copy_hash_verified": copied_hash == source_hash,
    })
    write_json(result_path, result)
    if copied_hash != source_hash:
        return 3
    write_json(generation_dir / "build_receipt.json", result)
    return 0


def assess_verdicts(contract: dict, positive: dict | None, negative: dict | None) -> dict:
    if not positive:
        return {"positive_state": "no_verdict", "positive_nonvacuous_pass": False,
                "callee_coverage": "unavailable", "same_contract_negative_caught": False,
                "function_pair_eligible": False, "no_trial_setup_credited": False}
    trials = int(positive.get("trials", 0) or 0)
    orig_ok = int(positive.get("orig_ok_trials", 0) or 0)
    positive_coverage = positive.get("coverage")
    positive_checks_complete = (isinstance(positive_coverage, dict)
                                and positive_coverage.get("checks_missing") == [])
    passed = (positive.get("passed") is True and trials > 0 and orig_ok == trials
              and int(positive.get("fails", 0) or 0) == 0
              and positive.get("vacuous") is False and positive_checks_complete)
    if trials == 0 or orig_ok == 0:
        positive_state = "no_trial_setup"
    elif passed:
        positive_state = "pass_nonvacuous"
    else:
        positive_state = "failed_or_vacuous"
    cov = positive.get("coverage") or {}
    declared = cov.get("callees_declared")
    if declared is None:
        raw = contract.get("callees", [])
        declared = [str(x.get("id")) for x in raw] if isinstance(raw, list) else list(raw)
    fired = cov.get("callees_fired_on_ok") or {}
    if not declared:
        callee_coverage = "n/a"
    elif all(int(fired.get(str(cid), 0) or 0) > 0 for cid in declared):
        callee_coverage = "covered"
    else:
        callee_coverage = "uncovered"
    same_hash = bool(negative and positive.get("contract_hash")
                     and positive.get("contract_hash") == negative.get("contract_hash"))
    semantic_checks = {"ret", "esp", "heap", "stack", "globals", "calls", "undeclared", "x87"}
    mismatch_checks = (negative.get("first_mismatch") or {}).get("checks", []) if negative else []
    semantic_mismatch = any(
        isinstance(check, dict) and check.get("name") in semantic_checks and check.get("passed") is False
        for check in mismatch_checks
    )
    negative_caught = bool(negative and negative.get("passed") is False
                           and int(negative.get("fails", 0) or 0) > 0
                           and int(negative.get("trials", 0) or 0) > 0
                           and int(negative.get("orig_ok_trials", 0) or 0) == int(negative.get("trials", 0) or 0)
                           and negative.get("error") is None
                           and same_hash and semantic_mismatch)
    pair_eligible = bool(passed and callee_coverage in {"covered", "n/a"}
                         and bool(contract.get("mut_export")) and negative_caught)
    return {
        "positive_state": positive_state,
        "positive_nonvacuous_pass": passed,
        "positive_trials": trials,
        "original_success_trials": orig_ok,
        "callee_coverage": callee_coverage,
        "declared_callees": list(declared),
        "same_contract_negative_caught": negative_caught,
        "negative_semantic_mismatch": semantic_mismatch,
        "positive_negative_contract_hash_equal": same_hash,
        "function_pair_eligible": pair_eligible,
        "no_trial_setup_credited": False,
    }


def process_parent_map() -> dict[int, int]:
    if os.name != "nt":
        return {}
    DWORD, ULONG_PTR = ctypes.c_ulong, ctypes.c_size_t

    class PROCESSENTRY32W(ctypes.Structure):
        _fields_ = [("dwSize", DWORD), ("cntUsage", DWORD), ("th32ProcessID", DWORD),
                    ("th32DefaultHeapID", ULONG_PTR), ("th32ModuleID", DWORD),
                    ("cntThreads", DWORD), ("th32ParentProcessID", DWORD),
                    ("pcPriClassBase", ctypes.c_long), ("dwFlags", DWORD),
                    ("szExeFile", ctypes.c_wchar * 260)]

    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.CreateToolhelp32Snapshot.argtypes = [DWORD, DWORD]
    k32.CreateToolhelp32Snapshot.restype = ctypes.c_void_p
    k32.Process32FirstW.argtypes = [ctypes.c_void_p, ctypes.POINTER(PROCESSENTRY32W)]
    k32.Process32FirstW.restype = ctypes.c_int
    k32.Process32NextW.argtypes = [ctypes.c_void_p, ctypes.POINTER(PROCESSENTRY32W)]
    k32.Process32NextW.restype = ctypes.c_int
    k32.CloseHandle.argtypes = [ctypes.c_void_p]
    k32.CloseHandle.restype = ctypes.c_int
    handle = k32.CreateToolhelp32Snapshot(2, 0)
    if handle in (None, ctypes.c_void_p(-1).value):
        raise OSError("CreateToolhelp32Snapshot failed")
    result = {}
    try:
        row = PROCESSENTRY32W()
        row.dwSize = ctypes.sizeof(row)
        ok = k32.Process32FirstW(handle, ctypes.byref(row))
        while ok:
            result[int(row.th32ProcessID)] = int(row.th32ParentProcessID)
            ok = k32.Process32NextW(handle, ctypes.byref(row))
    finally:
        k32.CloseHandle(handle)
    return result


def import_stock_driver(env: dict[str, str], manifest: dict):
    path = ROOT / manifest["stock_inputs"]["driver"]["path"]
    # This file lives beside coordinator/queue.py. When Python executes a
    # script from that directory, the directory leads sys.path and shadows
    # the stdlib queue imported by checker2.Worker.start(). Import the driver
    # with that directory excluded, and replace a cached local queue module.
    coordinator_dir = SLOT.parent.resolve()
    saved_path = list(sys.path)
    try:
        sys.path[:] = [entry for entry in sys.path
                       if Path(entry or os.curdir).resolve() != coordinator_dir]
        stdlib_queue = (Path(sysconfig.get_path("stdlib")) / "queue.py").resolve()
        queued = sys.modules.get("queue")
        queued_path = Path(getattr(queued, "__file__", "")).resolve() if queued else None
        if queued is not None and queued_path != stdlib_queue:
            del sys.modules["queue"]
        queue_module = importlib.import_module("queue")
        actual_queue = Path(getattr(queue_module, "__file__", "")).resolve()
        if actual_queue != stdlib_queue:
            raise RuntimeError(f"checker queue import is not the Python stdlib module: {actual_queue}")
        spec = importlib.util.spec_from_file_location("libertyflux_common_stock_checker2", path)
        if spec is None or spec.loader is None:
            raise RuntimeError("cannot load canonical checker2.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        spec.loader.exec_module(module)
    finally:
        sys.path[:] = saved_path
    if Path(module.__file__).resolve() != path.resolve():
        raise RuntimeError("checker module did not come from the pinned canonical path")
    expected = {
        "BUILD": env["LF_CHECKER_BUILD_DIR"],
        "WORKER": env["LF_CHECKER_WORKER"],
        "DLL": env["LF_CHECKER_DLL"],
        "EXE": env["LIBERTYFLUX_ORIG_EXE"],
    }
    for attr, value in expected.items():
        if os.path.normcase(str(Path(getattr(module, attr)).resolve())) != os.path.normcase(str(Path(value).resolve())):
            raise RuntimeError(f"checker {attr} does not match the pinned stock path")
    return module


def preflight_driver_routing(manifest: dict, env: dict[str, str]) -> None:
    """Fail before local_slot if the imported stock API cannot route this invocation."""
    saved = dict(os.environ)
    try:
        os.environ.clear()
        os.environ.update(env)
        module = import_stock_driver(env, manifest)
        out_dir = (ROOT / manifest["output_dir"]).resolve()
        if os.path.normcase(str(Path(module.OUT).resolve())) != os.path.normcase(str(out_dir)):
            raise RuntimeError("checker output environment does not resolve to the owned output folder")
        if manifest["mode"] == "run":
            contract = manifest["contract"]
            contract_path = (ROOT / contract["path"]).resolve()
            module.HERE = str(contract_path.parent.parent.resolve())
            routed = (Path(module.HERE) / "contracts" / f"{contract['name']}.json").resolve()
            if routed != contract_path:
                raise RuntimeError("checker basename lookup does not resolve to the pinned contract")
            if Path(contract_path).stem != contract["name"]:
                raise RuntimeError("checker contract basename changed during preflight")
            contract_obj = json.loads(contract_path.read_text(encoding="utf-8-sig"))
            if contract_obj.get("name") != contract["name"]:
                raise RuntimeError("checker contract name changed after manifest creation")
            module.validate_contract(contract_obj)
            current_validation = validate_contract_code(contract_obj, STOCK_PATHS["original_exe"])
            if current_validation != contract.get("function_validation"):
                raise RuntimeError("contract PE preflight evidence changed after manifest creation")
    finally:
        os.environ.clear()
        os.environ.update(saved)


def execute_manifest(manifest_path: Path, expected_manifest_hash: str) -> int:
    manifest_path = manifest_path.resolve()
    if sha256_file(manifest_path) != expected_manifest_hash:
        raise RuntimeError("preflight manifest changed before checker launch")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    errors = verify_manifest(manifest, dict(os.environ))
    if errors:
        raise RuntimeError("preflight recheck failed: " + "; ".join(errors))
    module = import_stock_driver(dict(os.environ), manifest)
    receipt_path = ROOT / manifest["run_dir"] / "driver_receipt.json"
    started = now()
    if manifest["mode"] == "selftest":
        argv = [str(DRIVER), "--selftest"]
    else:
        contract = manifest["contract"]
        contract_path = ROOT / contract["path"]
        module.HERE = str(contract_path.parent.parent.resolve())
        argv = [str(DRIVER), contract_path.stem, "--out", str((ROOT / manifest["output_dir"]).resolve())]
        if manifest.get("trials_override") is not None:
            argv.extend(["--trials", str(manifest["trials_override"])])
    try:
        code = int(module.main(argv))
        write_json(receipt_path, {"started_utc": started, "completed_utc": now(),
                                  "driver_exit_code": code, "exception": None,
                                  "driver_path": manifest["stock_inputs"]["driver"]["path"],
                                  "driver_sha256": manifest["stock_inputs"]["driver"]["sha256"]})
        return code
    except BaseException as exc:
        write_json(receipt_path, {"started_utc": started, "completed_utc": now(),
                                  "driver_exit_code": 1, "exception": f"{type(exc).__name__}: {exc}",
                                  "driver_path": manifest["stock_inputs"]["driver"]["path"],
                                  "driver_sha256": manifest["stock_inputs"]["driver"]["sha256"]})
        raise


def launch(manifest: dict, env: dict[str, str], python_exe: Path, run_dir: Path) -> int:
    errors = verify_manifest(manifest, env)
    if errors:
        raise RuntimeError("preflight recheck before local_slot failed: " + "; ".join(errors))
    manifest_path = run_dir / "preflight.json"
    write_json(manifest_path, manifest)
    manifest_hash = sha256_file(manifest_path)
    transcript_path = run_dir / "local_slot.log"
    command = [str(python_exe.resolve()), str(SLOT.resolve()), "--lane", manifest["lane"],
               "--cwd", str(ROOT.resolve()), "--", str(python_exe.resolve()),
               str(Path(__file__).resolve()), "_execute", "--manifest", str(manifest_path.resolve()),
               "--manifest-sha256", manifest_hash]
    lines: list[str] = []
    proc = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, text=True, encoding="utf-8",
                            errors="replace", bufsize=1)

    def drain() -> None:
        assert proc.stdout is not None
        for line in proc.stdout:
            lines.append(line.rstrip("\r\n"))

    reader = threading.Thread(target=drain, name="common-stock-runner-output", daemon=True)
    reader.start()
    observed: dict[int, list[int]] = {}
    while proc.poll() is None:
        parents = process_parent_map()
        for pid in list(parents):
            chain = pid_chain(pid, parents)
            if proc.pid in chain[1:]:
                observed[pid] = chain
        time.sleep(0.025)
    proc.wait()
    reader.join(timeout=5)
    transcript = "\n".join(lines) + ("\n" if lines else "")
    transcript_path.write_text(transcript, encoding="utf-8")
    slot = slot_transcript_evidence(transcript, proc.pid, proc.returncode)
    child = slot["child_pid_from_transcript"]
    owner = slot["owner_pid_from_transcript"]
    ancestry = slot_ancestry_evidence(proc.pid, owner, child, observed)
    final_parents = process_parent_map()
    remaining = sorted(pid for pid in observed if pid in final_parents)
    receipt_path = run_dir / "driver_receipt.json"
    receipt = json.loads(receipt_path.read_text(encoding="utf-8")) if receipt_path.is_file() else None
    output = ROOT / manifest["output_dir"]
    contract_obj = json.loads((ROOT / manifest["contract"]["path"]).read_text(encoding="utf-8")) if manifest.get("contract") else None
    positive = negative = None
    assessment = None
    if contract_obj:
        name = contract_obj.get("name") or manifest["contract"]["name"]
        pos_path = output / "verdicts" / f"{name}.json"
        neg_path = output / "mutants" / f"{name}.json"
        positive = json.loads(pos_path.read_text(encoding="utf-8")) if pos_path.is_file() else None
        negative = json.loads(neg_path.read_text(encoding="utf-8")) if neg_path.is_file() else None
        assessment = assess_verdicts(contract_obj, positive, negative)
    hash_errors = verify_manifest(manifest, env)
    wrapper_ok = slot["release_verified_from_this_transcript"]
    execution_ok = proc.returncode == 0 and receipt is not None and receipt.get("driver_exit_code") == 0
    ancestry_ok = ancestry["ancestry_ok"] if os.name == "nt" else False
    integrity_ok = execution_ok and wrapper_ok and ancestry_ok and not remaining and not hash_errors
    if assessment is not None:
        assessment["function_pair_eligible"] = bool(assessment["function_pair_eligible"] and integrity_ok)
    report = {
        "schema": "common-stock-runner-result-v1", "lane": manifest["lane"],
        "completed_utc": now(), "local_slot_exit_code": proc.returncode,
        "driver_receipt": receipt, "local_slot_transcript": relative(transcript_path),
        "local_slot_transcript_sha256": sha256_file(transcript_path),
        "local_slot": slot,
        "process_audit": {**ancestry,
                          "observed_owned_pids": sorted(observed),
                          "remaining_observed_pids_after_wrapper_exit": remaining},
        "post_run_hash_errors": hash_errors, "integrity_ok": integrity_ok,
        "verdict_assessment": assessment,
        "limitations": ["process ancestry is verified from Windows PID parent snapshots while the invocation is live; non-Windows runs cannot establish it"],
    }
    write_json(run_dir / "run.json", report)
    if proc.returncode != 0:
        return proc.returncode
    if not integrity_ok:
        return 3
    if assessment and assessment["positive_state"] == "no_trial_setup":
        return EXIT_EVIDENCE
    if assessment and not assessment["function_pair_eligible"]:
        return EXIT_NEGATIVE
    return 0


def cli(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="mode", required=True)
    build_parser = sub.add_parser("build", help="build one owned cdylib under local_slot and preserve it")
    build_parser.add_argument("--lane", required=True)
    build_parser.add_argument("--python", required=True, help="explicit interpreter for local_slot")
    build_parser.add_argument("--manifest-path", required=True, help="owned Cargo.toml to build")
    for mode in ("run", "selftest"):
        p = sub.add_parser(mode)
        p.add_argument("--lane", required=True)
        p.add_argument("--python", required=True, help="explicit interpreter used by both local_slot and its child")
        p.add_argument("--run-dir", required=True)
        if mode == "run":
            p.add_argument("--contract-dir", required=True)
            p.add_argument("--contract", required=True, help="basename in --contract-dir")
            p.add_argument("--candidate-source-root", required=True)
            p.add_argument("--candidate-dll", required=True)
            p.add_argument("--trials", type=int)
    internal = sub.add_parser("_execute")
    internal.add_argument("--manifest", required=True)
    internal.add_argument("--manifest-sha256", required=True)
    args = parser.parse_args(argv)
    if args.mode == "_execute":
        return execute_manifest(Path(args.manifest), args.manifest_sha256)
    if args.mode == "build":
        python_exe = Path(args.python)
        cargo_manifest = Path(args.manifest_path)
        if not python_exe.is_absolute():
            python_exe = ROOT / python_exe
        if not cargo_manifest.is_absolute():
            cargo_manifest = ROOT / cargo_manifest
        return run_build(args.lane, python_exe.resolve(), cargo_manifest.resolve())
    python_exe = Path(args.python).resolve()
    run_dir = Path(args.run_dir)
    if not run_dir.is_absolute():
        run_dir = ROOT / run_dir
    lane_root = ROOT / ".artifacts" / "scratch" / args.lane
    run_dir = require_owned_path(run_dir, lane_root, "run directory")
    run_dir.mkdir(parents=True, exist_ok=True)
    contract_dir = Path(args.contract_dir) if args.mode == "run" else None
    if contract_dir is not None and not contract_dir.is_absolute():
        contract_dir = ROOT / contract_dir
    candidate_source_root = Path(args.candidate_source_root) if args.mode == "run" else None
    if candidate_source_root is not None and not candidate_source_root.is_absolute():
        candidate_source_root = ROOT / candidate_source_root
    candidate_dll = Path(args.candidate_dll) if args.mode == "run" else None
    if candidate_dll is not None and not candidate_dll.is_absolute():
        candidate_dll = ROOT / candidate_dll
    manifest, env = make_manifest(args.mode, args.lane, python_exe, run_dir,
                                  contract_dir, getattr(args, "contract", None),
                                  candidate_source_root, candidate_dll,
                                  getattr(args, "trials", None))
    preflight_driver_routing(manifest, env)
    return launch(manifest, env, python_exe, run_dir)


if __name__ == "__main__":
    raise SystemExit(cli(sys.argv[1:]))
