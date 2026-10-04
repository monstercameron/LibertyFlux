"""Production top-up: write briefs and lists for the next production lanes. Lanes use checker version 4.

Usage: python make_briefs_prod.py <native lanes> <small lanes> [<big lanes>]
  native  r-n##  25 script native handlers
  small   r-s##  20 neighbouring functions under 250 bytes
  big     r-b##  neighbouring functions of 250 bytes and more, about 3,000 bytes of code per lane
Numbering continues after the lanes that already have a list; nothing already assigned is assigned again.
Functions come from the q-order lane's classification, restricted to entries the f-boundaries lane confirmed.
Callers run this at the same time, so the whole run holds an exclusive lock.

Experiment hooks (environment): LF_EXP_TAG and LF_EXP_NOTE vary the brief for
one measured experiment; LF_SMALL_N, LF_BIG_BYTES and LF_BIG_MAX vary batch sizes.
"""

import json
import os
import random
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import common
import queue as queue_rules
from briefs import brief_version, render_preamble, render_production
from briefs import template as brief_template

import pefile

ROOT = common.find_root()
OUT = common.briefs_dir(ROOT)
LISTS = common.lists_dir(ROOT)
SCRATCH = common.scratch_dir(ROOT)
COORD = common.coord_dir(ROOT)

NATIVES_PER_LANE = 25
SMALL_PER_LANE = 20
BIG_BYTES = 3000
BIG_MAX = 8
# The C runtime block, held out whole until the runtime lane classified it
# function by function (it has; this stands as the fallback when its file is missing).
CRT_BLOCK = (0x00DF8A00, 0x00E17000)
ENC_OFFSET = 0xFB000


def main():
    n_native, n_small = int(sys.argv[1]), int(sys.argv[2])
    n_big = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    small_per_lane = int(os.environ.get("LF_SMALL_N") or SMALL_PER_LANE)
    big_bytes = int(os.environ.get("LF_BIG_BYTES") or BIG_BYTES)
    big_max = int(os.environ.get("LF_BIG_MAX") or BIG_MAX)
    exp_tag = os.environ.get("LF_EXP_TAG") or ""
    exp_note = os.environ.get("LF_EXP_NOTE") or ""
    va = common.va

    with common.batch_lock(LISTS):
        pe = pefile.PE(str(common.orig_exe(ROOT)), fast_load=True)
        text = next(s for s in pe.sections if s.Name.rstrip(b"\0") == b".text")
        enc_end = pe.OPTIONAL_HEADER.ImageBase + text.VirtualAddress + ENC_OFFSET

        assigned = {va(e["va"]) for e in json.loads((SCRATCH / "f-leaf-candidates" / "pilot_50.json").read_text(encoding="utf-8"))}
        last = queue_rules.next_lane_numbers((p.stem for p in LISTS.glob("*.json")))
        for path in LISTS.glob("*.json"):
            if re.fullmatch(r"(q|r-[nsb])-?\d+", path.stem):
                for entry in json.loads(path.read_text(encoding="utf-8")):
                    assigned.add(va(entry.get("start") or entry.get("handler")))

        natives = sorted(json.loads((SCRATCH / "p0-natives" / "natives.json").read_text(encoding="utf-8")),
                         key=lambda n: n["name"] or "")
        for index, native in enumerate(natives):
            native["analysis"] = f"n-{index // 140 + 1:02d}"
        handler_addresses = {va(n["handler"]) for n in natives}
        rng = random.Random(last["n"] * 1000003 + last["s"] * 1009 + last["b"])
        lanes = {}

        def emit(lane, crate, what, slug):
            lanes[lane] = render_production(lane, crate, what, slug, str(ROOT))
            if exp_tag:
                current = brief_version()
                lanes[lane] = lanes[lane].replace(f"`brief` (write `{current}`)", f"`brief` (write `{exp_tag}`)")
                lanes[lane] += ("\n\nEXPERIMENT " + exp_tag + ". This lane is part of a measured experiment on how lanes work. " + exp_note
                                + " Everything else in this brief applies unchanged. Read the clock for your start and end times and write both in summary.txt.\n")
            (LISTS / f"{lane}.v4").write_text("checker version 4\n", encoding="utf-8")

        if n_native:
            seen = set()
            free_natives = []
            for native in natives:
                handler = va(native["handler"])
                if handler < enc_end or handler in seen or handler in assigned:
                    continue
                seen.add(handler)
                free_natives.append(native)
            rng.shuffle(free_natives)
            batches = queue_rules.split_native_batches(free_natives, NATIVES_PER_LANE, n_native)
            for k, batch in enumerate(batches):
                if not batch:
                    break
                number = last["n"] + k + 1
                lane = f"r-n{number:02d}"
                path = LISTS / f"{lane}.json"
                path.write_text(json.dumps([{"name": n["name"], "hash": n["hash"], "handler": n["handler"], "analysis": n["analysis"]} for n in batch], indent=1), encoding="utf-8")
                emit(lane, f"lf_rn{number:02d}_rw",
                     brief_template("native_what.txt").format(count=len(batch), list_path=str(path)), f"n{number:02d}")
            print(f"native handlers still unassigned after this call: {max(0, len(free_natives) - n_native * NATIVES_PER_LANE)}")

        if n_small or n_big:
            v2 = json.loads((SCRATCH / "f-boundaries" / "functions_v2.json").read_text(encoding="utf-8"))
            v2 = common.load_rows(v2, "functions") or list(v2.values())
            confirmed = {va(f["start"]): f for f in v2 if str(f.get("class", "")).startswith("confirmed") and f.get("class") != "confirmed-weak"}
            features = json.loads((SCRATCH / "q-order" / "features.json").read_text(encoding="utf-8"))
            features = common.load_rows(features, "functions") or list(features.values())
            # Library code is not rewritten: crates replace zlib, the JPEG and TLS libraries, and Rust's standard
            # library replaces the C runtime. The library lane tagged the first four; the C runtime is only partly
            # tagged, so the block it occupies is held out whole until a lane has classified it (lane r-s130 was
            # handed string and printf routines from it).
            replaced = {va(f["start"]) for f in json.loads((SCRATCH / "f-library-ranges" / "library_functions.json").read_text(encoding="utf-8"))
                        if f.get("library") in queue_rules.REPLACED_LIBRARIES}
            crt_block = CRT_BLOCK
            runtime_file = SCRATCH / "f-runtime" / "runtime_functions.json"
            if runtime_file.exists():
                # The runtime lane classified the block function by function: hold out what it judged runtime or was
                # unsure about, and release the rest.
                rows = json.loads(runtime_file.read_text(encoding="utf-8"))
                rows = common.load_rows(rows, "functions") or list(rows.values())
                replaced |= {va(r["start"]) for r in rows}
                crt_block = (0, 0)
            fragments_file = SCRATCH / "f-ptronly" / "ptr_only_verdicts.json"
            if fragments_file.exists():
                # The pointer-only screening found a few entries that are pieces of other functions (exception
                # handler blocks): they are rewritten as part of the enclosing function, never on their own.
                replaced |= {va(r["start"]) for r in json.loads(fragments_file.read_text(encoding="utf-8")) if r.get("verdict") == "fragment"}
            weak_file = SCRATCH / "f-weak" / "weak_verdicts.json"
            if weak_file.exists():
                replaced |= {va(r["start"]) for r in json.loads(weak_file.read_text(encoding="utf-8")) if r.get("verdict") in ("fragment", "data")}
            small, big, held = queue_rules.filter_pool(
                features, confirmed, assigned=assigned, handlers=handler_addresses,
                replaced=replaced, crt_block=crt_block, enc_end=enc_end)
            print(f"queue: {len(small)} functions under 250 bytes, {len(big)} of 250 bytes and more; {held} library or C runtime functions held out")

            for k, (subsystem, chunk) in enumerate(queue_rules.pick(
                    queue_rules.chunks_of(small, lambda run: len(run) >= small_per_lane), n_small, rng)):
                number = last["s"] + k + 1
                lane = f"r-s{number:02d}"
                path = LISTS / f"{lane}.json"
                path.write_text(json.dumps(chunk, indent=1), encoding="utf-8")
                what = brief_template("small_what.txt").format(count=len(chunk), list_path=str(path), subsystem=subsystem)
                emit(lane, f"lf_rs{number:02d}_rw", what, f"s{number:02d}")
            for k, (subsystem, chunk) in enumerate(queue_rules.pick(
                    queue_rules.chunks_of(big, lambda run: len(run) >= big_max or sum(f["size"] for f in run) >= big_bytes), n_big, rng)):
                number = last["b"] + k + 1
                lane = f"r-b{number:02d}"
                path = LISTS / f"{lane}.json"
                path.write_text(json.dumps(chunk, indent=1), encoding="utf-8")
                sizes = [f["size"] for f in chunk]
                what = brief_template("big_what.txt").format(count=len(chunk), list_path=str(path), subsystem=subsystem,
                                                             smallest=min(sizes), largest=max(sizes), total=sum(sizes))
                emit(lane, f"lf_rb{number:02d}_rw", what, f"b{number:02d}")

        OUT.mkdir(parents=True, exist_ok=True)
        for lane, task in lanes.items():
            (OUT / f"{lane}.txt").write_text(render_preamble(lane, str(ROOT)) + task, encoding="utf-8", newline="\n")
        print("LANES " + " ".join(lanes))


if __name__ == "__main__":
    main()
