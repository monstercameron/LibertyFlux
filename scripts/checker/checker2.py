"""checker2.py: driver for the checker (differential equivalence of rewrites).

Usage (from the repository root). Build the 32-bit worker and proof DLL first:
  cargo build --release --target i686-pc-windows-msvc -p lf-checker-worker
  RUSTFLAGS="-C panic=abort" cargo build --release --target i686-pc-windows-msvc -p lf-checker-proofs
Then run contracts:
  checker2.py CONTRACT [CONTRACT ...]   run contracts (correct + mutant)
  checker2.py --all                     run scripts/checker/contracts/*.json
  checker2.py --smoke                   2 contracts x 10 trials, fast check
  checker2.py --trials N ...            override trial counts
  checker2.py --exe PATH ...            original executable (default below)
  checker2.py --out DIR ...             output dir for verdicts (default below)
Run with the repo Python, e.g.:
  .venv/Scripts/python.exe scripts/checker/checker2.py --smoke

Writes verdicts/<name>.json (accept.py-compatible), mutants/<name>.json and
results.json, by default next to the driver (override with --out or
LF_CHECKER_OUT). Several drivers may run at once on different contracts (one
worker process each); each run overwrites results.json, so merge verdicts
afterwards (merge_results.py).

Paths (all relative to the repository root unless overridden):
  worker   .artifacts/build/cargo-tools/i686-pc-windows-msvc/release/lf-checker-worker.exe
  proof DLL .../lf_checker_proofs.dll (LF_CHECKER_DLL, or legacy K2_DLL)
  original orig/GTAIV.exe (LIBERTYFLUX_ORIG_EXE, or --exe)
  LF_CHECKER_WORKER overrides the worker path; LF_CHECKER_BUILD_DIR overrides
  the build directory both binaries are found under. A contract-level "dll"
  runs a different rewrite DLL; relative paths resolve against the repo root.

This driver folds every pilot-lane driver extension into one contract format:
small/edge-biased inputs, scripted heap words and pins, seeded/linked globals
incl. heap pointers, null args, per-callee scripts, stub out-param words, TLS
fabrication, XMM entry values, defined stack fills, status histograms and
orig-ok counts.

What it is
----------
The checker decides whether a Rust rewrite of one original function behaves
the same as the original. It runs the ORIGINAL (mapped from the executable,
relocated, executed natively in a 32-bit process) and the REWRITE (a 32-bit
DLL export) from bit-identical starting state over many generated inputs and
compares everything observable: return value, written memory, stack-pointer
adjustment, outgoing calls, and faults.

Parts: the worker (crates/tools/lf-checker-worker, i686, zero deps), the
rewrite runtime (crates/tools/lf-checker-rt), the proof rewrites
(crates/tools/lf-checker-proofs, the regression test), this driver (runs on
the 64-bit venv), the contract generator (gen_contracts.py, emits the k2
proof contracts), contracts/*.json (one per function) and the verdict
outputs. The 20 first-version contracts are carried over unchanged as the
regression; three k2 contracts run the pilot lanes' rewrite DLLs (see
"Regression" below) and the rest run the proof DLL.

How a trial works
-----------------
1. The driver generates inputs from the contract (seeded RNG + edge values)
   and sends one `trial` request to the worker: register values, stack words,
   heap segment contents, global fills, per-callee scripted answers and
   out-param words, TLS slots, XMM values.
2. The worker fills the heap with a trial-specific pattern, applies the
   segments, resets data sections to pristine + fill, plants the TLS slots,
   loads the XMM entry values, and calls the ORIGINAL through a trampoline
   that captures registers, EFLAGS, x87/SSE state (fxsave), heap/stack/
   global diffs and the call log. Faults are caught in-process by a vectored
   handler that resumes at a landing pad; the worker never dies on a bad trial.
3. The worker repeats step 2 for the REWRITE export with identical inputs
   (fresh below-ESP scratch, same fills). While the rewrite runs, the
   original's code pages are revoked (PAGE_NOACCESS), so a rewrite that
   calls the original or reads its code faults and is reported as `cheat`.
4. The worker compares the two observations per the declared checks and
   returns pass/fail plus details. The driver aggregates trials into a
   verdict file.

A hang is cut off by a watchdog: each trial runs on its own thread with a
timeout; on expiry the worker reports `hang` and exits so the driver starts
a fresh worker (a hung trial taints the process).

Contract format
---------------
One JSON file per function:
- `name`, `function` (original RVA hex), `conv` (`cdecl`, `stdcall`,
  `thiscall`, `fastcall`), `export` / `mut_export` (mutant optional),
  `trials`, `seed`, `timeout_ms`, `dll` (optional override of the rewrite
  DLL; also LF_CHECKER_DLL in the environment).
- `ret`: compared return channel: `eax`, `ax`, `al`, `edx_eax`, `st0`,
  `xmm0`, `none`.
- `regs`: 7 input specs [eax,ecx,edx,ebx,esi,edi,ebp]: {"any":true},
  {"heap":seg,"plus":bytes} (plus may be a per-trial list),
  {"int":N}, {"small":max} (ranged int), {"cycle":[...]}, {"null":true}.
- `stack`: list of {kind}: `int`, `byte`, `float`, `ptr` (+seg),
  `zero`/`null`, `ptr_or_null` (+seg, NULL on even trials),
  `small` (+max), `smallint` (switch-index-like), `cycle` (+values).
- `heapsegs`: [{off,size,fill,links,floats_at,words,pin}]. `fill` rotates
  per trial between zeros, pattern and random when `random`. `links`
  pre-link nested pointers. `floats_at` marks float-edge words. `words`
  gives explicit per-word specs instead of a fill. `pin` pins words to
  per-trial cycled values: [{at, vals, size?}] (size 1/2/4, default 4).
  Legacy aliases accepted: contract-level `pokes`, seg-level `pinned`,
  contract-level `heap_scripts`.
- Word specs (used in `words`, `pin` vals, scripts, global values):
  an integer literal, {"int":N}, {"heap":seg,"plus":n} (live pointer),
  {"heap_off":bytes} (heap-relative pointer), {"any":true} (random),
  {"small":max}, {"cycle":[...]}, {"rot":[...]} (alias),
  {"float":true} (float-edge bits), {"stub":id} (recorder-stub address
  for vtable/data-slot planting). A bare {"heap":N} with no `plus` means
  segment N when N names one, else a byte offset.
- `globals`: [{rva,size}] declared ranges (dword-aligned). `globals_fill`:
  `random`, `pristine`, an explicit [{rva,words}] list, `globals_values`
  ([{rva,words}] with word specs, supports heap/stub pointers), or
  `globals_fill_spec` (`heap_ptr`/`cycle`/`words` per range).
- `callees`: [{id,conv,nargs,ret,script,writes,wscript,snap,logxmm,
  xmm0_from_stack}]. `conv` sets cleanup (`cdecl` callers clean up, all
  others pop nargs*4); `ret` is the scripted answer channel (`u32`,
  `u64`, `al`, `f32xmm0`, `f64xmm0`, `f32st0`, `f64st0`); `script` is a
  value list cycled per trial (each callee gets its own slot),
  `"edges"`, or entries resolving to heap pointers. `writes` declares
  out-param stores: [{arg:N|reg:"ecx"/"edx", at, n}]; `wscript` is the
  per-trial word matrix cycled alongside. `snap` declares pointed-to
  snapshots: [{kind:"arg"/"ecx"/"edx", idx, n}], at most 8 words total
  per callee. `logxmm` logs the 16 bytes of XMM0 at the call.
  `xmm0_from_stack` names the stack arg the stub loads XMM0 from on
  the rewrite side (transport for xmm0-arg callees; call args are then
  skipped on both sides and only XMM0 is compared).
- `patches`: [{site (RVA hex of an E8), id}]. `tailpatches`:
  [{site (RVA hex of an E9), id}]; the tail stub logs like a normal stub
  then returns straight to the trampoline with `outer_pop + 4`
  (`outer_pop` defaults from `conv` + stack length, overridable).
  `iat`: [{dll,name,id}] import slots rewritten to recorder stubs.
- `tls`: [{slot|slot_rva, seg, plus} | {slot, int}]: fabricated
  TLS slots, planted on the trial thread before each side. Slots 0-63.
- `xmm`: {reg: [4 word specs]}: scripted XMM entry values; use the
  string `"float"` for a float-edge low word. Unlisted words are zero.
- `stack_fill`: a defined fill for uninitialized stack (integer, or
  `"pattern"` for the default trial pattern). Use `0` when either side
  reads stack it did not write.
- `checks`: `ret`, `esp`, `heap`, `stack`, `globals`, `calls`,
  `undeclared` (all bool), `fulldata` (bool), `fp_tol`, `fp64`,
  `call_regs` (optional {id:[regs]}), `call_skip`
  ({id:[stack arg indexes]} dropped from the call comparison).

Indirect calls: the planting design
-----------------------------------
Register-indirect calls, calls through object vtables, calls through data
tables, and computed tail jumps cannot be patched with a 5-byte direct call
(the instruction is too short). The checker intercepts them by fabrication
instead of patching:
1. The contract declares a callee for each indirect target as usual.
2. The setup response returns `stub_addrs`, one recorder-stub address per id.
3. The driver plants those addresses into fabricated heap objects
   ({"stub":id} word specs in `pin`/`words`) and global slots
   (`globals_values`) before each trial.
4. The original's indirect call lands on the recorder stub, which logs and
   answers exactly like a patched direct site. The rewrite performs the
   same load-and-call through the same fabricated object, so both sides
   call the same stub. No breakpoint or guard-page scheme is needed, and
   the rewrite stays faithful (no ctable use for these calls).

E9 tail jumps are patched (`tailpatches`): the tail stub logs, answers,
and returns straight to the trampoline. Computed tail jumps (`jmp [reg]`)
through planted vtables work with normal stubs (pop 0): the jump lands on
the stub and its plain `ret` returns to the trampoline.

Why this design: it reuses the existing stub machinery (one code path for
logging, answers, writes and snapshots), it needs no single-stepping or
page games in the fault-heavy worker, and both sides execute the same
indirection, so the comparison stays differential. Its limit is honest:
only pointers the contract can fabricate (heap objects, global slots) can
be intercepted; code pointers computed at runtime stay out of reach.

Pointer-aware call comparison
-----------------------------
Call arguments and compared registers are normalized by region before
comparison: heap addresses as `H+offset`, scratch-stack addresses as
`S+/-offset-from-incoming-ESP`, image addresses as `I+rva`. Raw values
outside the worker windows compare as before.

Normalization classifies; it rarely equates on its own (heap and image
bases are identical on both sides already, while frame layouts genuinely
differ). The working pattern for a pointer argument is therefore:
- `call_skip` the address (or `call_regs` for ECX/EDX), and
- `snap` the pointed-to words, which the stub copies into the call log at
  call time and which compare by value.

TLS, XMM, and rewrite-side transports
-------------------------------------
Rust cannot read FS-based TLS or incoming vector registers, and it cannot
place a value in XMM0 for a call. Three transports bridge the gap; in each
case the same scripted value reaches both sides and only the transport
differs, which the contract documents:
- `tls_slot(n)` (runtime) reads the fabricated TLS value the original
  reads through FS:[0x2c].
- `xmm_word(reg, i)` (runtime) reads the scripted XMM entry value the
  original reads from its register.
- `xmm0_from_stack` (callee option) has the stub load XMM0 from a stack
  argument on the rewrite side only; the original side passes real XMM0.
  The logged XMM0 bytes are then compared.

Verdict format (accept.py-compatible)
-------------------------------------
`verdicts/<name>.json` keeps the five required keys (`function`, `passed`,
`inputs_tested`, `comparisons`, `checker_version`) and adds: `name`,
`export`, `contract_hash`, `trials`, `fails`, `comparisons_detail`,
`fp_exact`, `worker_us_total`, `wall_s`, `first_mismatch`, plus
`status_hist` ({"ok/ok": N, ...}) and `orig_ok_trials` (trials where the
original side returned OK; the accept gate should require this to be
non-zero before any fault-driven pass counts).

Writing a rewrite
-----------------
Rewrites live in a 32-bit cdylib and use `lf-checker-rt`:
- Declare each export with `lf_checker_rt::export!(conv, name(args) -> ret
  { ... })` using the original's convention, so stack cleanup matches.
- Never hard-code a mapped address. Derive addresses with
  `relocated(file_va)` / `global::<T>(file_va)`; the worker patches
  `CHECKER_XBASE` at load.
- Call intercepted direct callees with `callee_cdecl!` / `callee_stdcall!`
  / `callee_thiscall!` / `callee_fastcall!`; the worker patches
  `CHECKER_CTABLE`. Calling an undeclared id faults and fails honestly.
- Call indirect/data-table targets by loading and calling through the
  fabricated object exactly like the original (transmuted pointer); both
  sides land on the same planted stub.
- Read fabricated TLS through `tls_slot(n)` and scripted XMM entry values
  through `xmm_word(reg, i)`; pass xmm0-arg values on the stack when the
  callee declares `xmm0_from_stack`.
- Return FP through the real channel (`f64` return = ST0 on this target;
  an `f32` return type reads an ST0 float result without assembly).
- Use wrapping arithmetic explicitly; the DLL builds with `panic=abort`
  so a panic becomes a fault, never an unwind into the worker.

Regression
----------
`--all` runs the 20 first-version contracts plus the k2 proof contracts.
Three k2 contracts (k2_percal1, k2_percal2, k2_ptr1) run the pilot lanes'
rewrite DLLs, found relative to the repo root under .artifacts/build/;
they are lane build outputs, not tracked files, so those three are skipped
when the DLLs are absent. k2_ptr1 is a documented negative (its premise
is false; it fails by design), k2_stw1b is a mutant-as-export control
(fails by design), and k2_ptr1m_skip is a skip-only vacuity control
(passes by design); everything else must pass, and every declared mutant
must fail.

What it can and cannot verify
-----------------------------
Can: multi-callee branch coverage via per-callee answers; callees that
fill structs and buffers; register-indirect, vtable, data-table and tail
calls; frame-pointer call arguments (address skipped, contents compared);
TLS-slot-driven logic (slots 0-63); xmm0-arg callees and xmm entry
values; functions reading uninitialized stack (with a defined fill).

Still cannot: functions in the encrypted first megabyte of the code
section; behaviour needing a running game (initialized heap graphs, OS
handles); import implementations (calls observed, answers scripted);
timing/`rdtsc`/CPUID paths; self-modifying or preferred-base-dependent
code; concurrency; TLS expansion slots (>= 64); x87 entry values; anything
after the first fault in a trial.

Current limits: one worker is single-trial-at-a-time (run one driver per
core); snapshots cap at 8 words per callee and out-param words at 16 per
callee; computed `jmp reg` with non-vtable targets still needs per-case
analysis.
"""
import json, os, sys, time, random, struct, hashlib, subprocess, threading, queue

HERE = os.path.dirname(os.path.abspath(__file__))


def repo_root():
    """Repository root, derived from this file's location (scripts/checker/)."""
    here = os.path.abspath(HERE)
    while True:
        if os.path.exists(os.path.join(here, "AGENTS.md")):
            return here
        parent = os.path.dirname(here)
        if parent == here:
            break
        here = parent
    # Fallback: scripts/checker/ is two levels below the root.
    return os.path.dirname(os.path.dirname(HERE))


ROOT = repo_root()
BUILD = os.environ.get("LF_CHECKER_BUILD_DIR", os.path.join(
    ROOT, ".artifacts", "build", "cargo-tools", "i686-pc-windows-msvc", "release"))
WORKER = os.environ.get("LF_CHECKER_WORKER", os.path.join(BUILD, "lf-checker-worker.exe"))
# Proof DLL default; LF_CHECKER_DLL selects another rewrite DLL, and the
# legacy K2_DLL name is still honored (lets one worker run the pilot lanes'
# existing rewrite DLLs unchanged for fold-back proofs).
DLL = os.environ.get("LF_CHECKER_DLL", os.environ.get(
    "K2_DLL", os.path.join(BUILD, "lf_checker_proofs.dll")))
EXE = os.environ.get("LIBERTYFLUX_ORIG_EXE", os.path.join(ROOT, "orig", "GTAIV.exe"))
OUT = os.environ.get("LF_CHECKER_OUT", HERE)


def dll_for(contract):
    if contract.get("dll"):
        d = contract["dll"]
        return d if os.path.isabs(d) else os.path.join(ROOT, d)
    return DLL

INT_EDGES = [0, 1, 2, 3, 0xFF, 0x100, 0xFFFF, 0x10000, 0x7FFFFFFF, 0x80000000,
             0x80000001, 0xFFFFFFFE, 0xFFFFFFFF, 0x7F, 0x80, 0x8000, 0x12345678]
BYTE_EDGES = [0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF]
FLOAT_EDGES = [0x00000000, 0x80000000, 0x3F800000, 0xBF800000, 0x3F000000,
               0x40490FDB, 0x7F800000, 0xFF800000, 0x7FC00000, 0x00800000,
               0x007FFFFF, 0x00000001, 0x501502F9, 0x2EDBE6FF, 0xC0000000]


def fbits(rng):
    r = rng.random()
    if r < 0.15:
        return rng.choice(FLOAT_EDGES)
    if r < 0.20:  # subnormal
        return rng.getrandbits(23)
    if r < 0.25:  # small normal
        return 0x00800000 | rng.getrandbits(23)
    v = rng.getrandbits(32)
    if (v & 0x7F800000) == 0x7F800000 and (v & 0x007FFFFF) != 0:
        v |= 0x00400000  # quiet any signaling NaN (x87/SSE quieting differs)
    return v


class Worker:
    def __init__(self):
        self.heap = 0
        self.stubs = {}
        self.start()

    def start(self):
        self.p = subprocess.Popen([WORKER], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                  stderr=open(os.devnull, "w"), text=True, bufsize=1)
        self.q = queue.Queue()
        self.t = threading.Thread(target=self._reader, daemon=True)
        self.t.start()
        banner = self.recv(60)
        assert banner.get("ready"), banner

    def _reader(self):
        try:
            for line in self.p.stdout:
                self.q.put(line)
        except Exception:
            pass

    def send(self, obj):
        self.p.stdin.write(json.dumps(obj) + "\n")
        self.p.stdin.flush()

    def recv(self, timeout):
        try:
            line = self.q.get(timeout=timeout)
        except queue.Empty:
            raise TimeoutError("worker silent")
        return json.loads(line)

    def call(self, obj, timeout=60):
        self.send(obj)
        return self.recv(timeout)

    def stop(self):
        try:
            self.p.kill()
        except Exception:
            pass


def gen_int(rng, trial, reg_i=0):
    if trial < len(INT_EDGES):
        return INT_EDGES[(trial + reg_i) % len(INT_EDGES)]
    return rng.getrandbits(32)


def gen_small(rng, trial, mx):
    cands = [v for v in (0, 1, 2, 3, 4, 7, 8, 15, 16, mx, mx - 1) if 0 <= v <= mx]
    if trial < 24 and cands:
        return cands[trial % len(cands)]
    return rng.randint(0, mx)


def gen_byteword(rng, trial):
    lo = BYTE_EDGES[trial % len(BYTE_EDGES)] if trial < 24 else rng.getrandbits(8)
    return lo | (rng.getrandbits(24) << 8)


def resolve_word(spec, rng, trial, heap, contract, stubs):
    """Resolve one unified word spec to a u32.

    int -> literal; {"int":N}; {"heap":seg,"plus":n} live pointer;
    {"heap_off":bytes} heap-base-relative pointer; {"any":true} random;
    {"small":max} ranged int; {"cycle":[...]} per-trial rotation;
    {"rot":[...]} alias; {"stub":id} recorder-stub address (vtable planting);
    {"float":true} float-edge-biased bits.
    """
    if isinstance(spec, int):
        return spec & 0xFFFFFFFF
    if "stub" in spec:
        sid = str(spec["stub"])
        if sid not in stubs:
            raise KeyError("unknown stub id %s (setup stub_addrs: %s)" % (sid, sorted(stubs)))
        return stubs[sid] & 0xFFFFFFFF
    if "heap" in spec:
        # bare {"heap":N}: segment index when N names one (q-05 form, or q-16
        # values below the segment count); byte offset otherwise (q-16 form).
        # {"heap_off":N} is always a byte offset; prefer it in new contracts.
        n = spec["heap"]
        if "plus" in spec or n < len(contract["heapsegs"]):
            seg = contract["heapsegs"][n]
            return (heap + seg["off"] + spec.get("plus", 0)) & 0xFFFFFFFF
        return (heap + n) & 0xFFFFFFFF
    if "heap_off" in spec:
        return (heap + spec["heap_off"]) & 0xFFFFFFFF
    if "int" in spec:
        return spec["int"] & 0xFFFFFFFF
    if "small" in spec:
        return gen_small(rng, trial, spec["small"])
    if "cycle" in spec:
        seq = spec["cycle"]
        return resolve_word(seq[trial % len(seq)], rng, trial, heap, contract, stubs)
    if "rot" in spec:
        seq = spec["rot"]
        return resolve_word(seq[trial % len(seq)], rng, trial, heap, contract, stubs)
    if "float" in spec:
        return fbits(rng)
    return rng.getrandbits(32)  # {"any": true}


def resolve_regs(contract, rng, trial, heap):
    out = []
    for i, spec in enumerate(contract["regs"]):
        if "heap" in spec:
            seg = contract["heapsegs"][spec["heap"]]
            p = spec.get("plus", 0)
            if isinstance(p, list):  # q-05: cycled offsets
                p = p[trial % len(p)]
            out.append((heap + seg["off"] + p) & 0xFFFFFFFF)
        elif "int" in spec:
            out.append(spec["int"] & 0xFFFFFFFF)
        elif "small" in spec:
            out.append(gen_small(rng, trial + i * 3, spec["small"]))
        elif "cycle" in spec:
            out.append(spec["cycle"][trial % len(spec["cycle"])] & 0xFFFFFFFF)
        elif "null" in spec:
            out.append(0)
        else:
            out.append(gen_int(rng, trial, i))
    return out


def resolve_stack(contract, rng, trial, heap):
    out = []
    for s in contract["stack"]:
        k = s["kind"]
        if k == "int":
            out.append(gen_int(rng, trial, len(out)))
        elif k == "byte":
            out.append(gen_byteword(rng, trial + len(out) * 7))
        elif k == "float":
            out.append(fbits(rng) if trial >= len(FLOAT_EDGES) else
                       FLOAT_EDGES[(trial + len(out)) % len(FLOAT_EDGES)])
        elif k == "zero" or k == "null":
            out.append(0)
        elif k == "ptr":
            seg = contract["heapsegs"][s["seg"]]
            p = s.get("plus", 0)
            if isinstance(p, list):
                p = p[trial % len(p)]
            out.append((heap + seg["off"] + p) & 0xFFFFFFFF)
        elif k == "ptr_or_null":  # q-06: NULL on even trials, pointer on odd
            if trial % 2 == 0:
                out.append(0)
            else:
                seg = contract["heapsegs"][s["seg"]]
                out.append((heap + seg["off"] + s.get("plus", 0)) & 0xFFFFFFFF)
        elif k == "small":  # q-01/q-16 unified ranged int
            out.append(gen_small(rng, trial + len(out) * 3, s.get("max", 3)))
        elif k == "smallint":  # q-13 switch-index-like values
            if trial < 40:
                out.append(((trial - 5) & 0xFFFFFFFF))
            elif rng.random() < 0.7:
                out.append(rng.getrandbits(32) % 40)
            else:
                out.append(gen_int(rng, trial, len(out)))
        elif k == "cycle":  # q-02/q-04 deterministic per-trial values
            vals = s["values"]
            out.append(vals[(trial + len(out)) % len(vals)] & 0xFFFFFFFF)
        else:
            raise ValueError("stack kind " + k)
    return out


def _apply_pin(words, at, size, val):
    wi = at // 4
    sh = (at % 4) * 8
    mask = {1: 0xFF, 2: 0xFFFF, 4: 0xFFFFFFFF}[size] << sh
    words[wi] = (words[wi] & ~mask & 0xFFFFFFFF) | ((val << sh) & mask & 0xFFFFFFFF)


def resolve_segs(contract, rng, trial, heap, stubs):
    res = []
    for si, sg in enumerate(contract["heapsegs"]):
        nw = sg["size"] // 4
        if "words" in sg:  # q-06 explicit words
            specs = sg["words"]
            assert len(specs) == nw, (si, len(specs), nw)
            res.append({"off": sg["off"],
                        "words": [resolve_word(x, rng, trial, heap, contract, stubs)
                                  for x in specs]})
            continue
        fill = sg.get("fill", "random")
        pol = trial % 3 if fill == "random" else -1
        words = []
        for w in range(nw):
            if fill == "zeros" or pol == 0:
                words.append(0)
            elif fill == "pattern" or pol == 1:
                words.append(((si * 0x1000 + w) * 0x9E3779B1) & 0xFFFFFFFF)
            elif fill == "floats":
                words.append(rng.getrandbits(32))
            else:
                words.append(rng.getrandbits(32))
        if fill == "floats":
            for wi in sg.get("floats_at", []):
                if trial < len(FLOAT_EDGES):
                    words[wi] = FLOAT_EDGES[(trial + wi) % len(FLOAT_EDGES)]
                else:
                    words[wi] = fbits(rng)
        for link in sg.get("links", []):
            tgt = contract["heapsegs"][link["to_seg"]]
            words[link["at"] // 4] = (heap + tgt["off"] + link.get("plus", 0)) & 0xFFFFFFFF
        # unified pins: [{at, vals, size?}] cycled per trial (q-04/q-05/q-07/q-08)
        pins = list(sg.get("pin", []))
        for p in contract.get("pokes", []):  # q-07 alias (contract-level, has seg)
            if p["seg"] == si:
                pins.append(p)
        for idx, vals in sg.get("pinned", []):  # q-08 alias [[word,[vals]]]
            pins.append({"at": idx * 4, "vals": vals})
        for sc in contract.get("heap_scripts", []):  # q-02 alias [{seg,word,cycle}]
            if sc["seg"] == si:
                pins.append({"at": sc["word"] * 4, "vals": sc["cycle"]})
        for pin in pins:
            v = pin["vals"][trial % len(pin["vals"])]
            v = resolve_word(v, rng, trial, heap, contract, stubs)
            _apply_pin(words, pin["at"], pin.get("size", 4), v)
        res.append({"off": sg["off"], "words": words})
    return res


def resolve_globals_fill(contract, rng, trial, heap, stubs):
    # q-06 explicit values (most general; supports heap/stub specs)
    if "globals_values" in contract:
        out = []
        for g in contract["globals_values"]:
            rva = g["rva"]
            rva = int(rva, 16) if isinstance(rva, str) else rva
            out.append({"rva": rva,
                        "words": [resolve_word(x, rng, trial, heap, contract, stubs)
                                  for x in g["words"]]})
        return out
    # q-02 seeded/linked globals
    spec = contract.get("globals_fill_spec")
    if spec is not None:
        out = []
        for g in spec:
            rva = g["rva"]
            rva = int(rva, 16) if isinstance(rva, str) else rva
            if "heap_ptr" in g:
                hp = g["heap_ptr"]
                seg = contract["heapsegs"][hp["seg"]]
                out.append({"rva": rva,
                            "words": [(heap + seg["off"] + hp.get("plus", 0)) & 0xFFFFFFFF]})
            elif "cycle" in g:
                out.append({"rva": rva, "words": list(g["cycle"][trial % len(g["cycle"])])})
            elif "words" in g:
                out.append({"rva": rva, "words": list(g["words"])})
            else:
                raise ValueError("globals_fill_spec entry needs heap_ptr/cycle/words")
        return out
    return contract.get("globals_fill", "pristine")


def resolve_script_val(v, rng, trial, heap, contract, stubs):
    if isinstance(v, dict):
        return resolve_word(v, rng, trial, heap, contract, stubs)
    return v & 0xFFFFFFFF


def resolve_scripts(contract, rng, trial, heap, stubs):
    out = []
    for c in contract.get("callees", []):
        sc = c["script"]
        if sc == "edges":
            v, hi = INT_EDGES[trial % len(INT_EDGES)], 0
        else:
            raw = sc[trial % len(sc)]
            if isinstance(raw, dict) and "lo" in raw:  # 64-bit script word
                v = resolve_script_val(raw["lo"], rng, trial, heap, contract, stubs)
                hi = resolve_script_val(raw.get("hi", 0), rng, trial, heap, contract, stubs)
            else:
                v = resolve_script_val(raw, rng, trial, heap, contract, stubs)
                hi = 0
        e = {"id": c["id"], "lo": v & 0xFFFFFFFF, "hi": hi & 0xFFFFFFFF}
        ws = c.get("wscript")
        if ws is not None:  # v2 out-param write words, cycled per trial
            e["w"] = [resolve_script_val(x, rng, trial, heap, contract, stubs)
                      for x in ws[trial % len(ws)]]
        out.append(e)
    return out


def resolve_tls(contract, trial, heap):
    out = []
    for t in contract.get("tls", []):
        e = {}
        if "slot_rva" in t:
            e["slot_rva"] = t["slot_rva"]
        else:
            e["slot"] = t.get("slot", 0)
        if "int" in t:
            e["value"] = t["int"] & 0xFFFFFFFF
        else:
            seg = contract["heapsegs"][t["seg"]]
            e["value"] = (heap + seg["off"] + t.get("plus", 0)) & 0xFFFFFFFF
        out.append(e)
    return out


def resolve_xmm(contract, rng, trial):
    xm = contract.get("xmm")
    if xm is None:
        return None
    out = {}
    for reg, specs in xm.items():
        words = []
        for sp in specs:
            if isinstance(sp, str) and sp == "float":
                if trial < len(FLOAT_EDGES):
                    words.append(FLOAT_EDGES[trial % len(FLOAT_EDGES)])
                else:
                    words.append(fbits(rng))
            else:
                words.append(sp & 0xFFFFFFFF)
        out[str(reg)] = words
    return out


def run_contract(w, contract, export, trials, seed, stop_after_fails=None):
    rng = random.Random(seed)
    results = []
    fails = 0
    first_fail = None
    check_names = None
    t0 = time.perf_counter()
    for t in range(trials):
        req = {
            "cmd": "trial",
            "fn_rva": int(contract["function"], 16),
            "export": export,
            "trial": t,
            "seed": seed & 0xFFFFFFFF,
            "regs": resolve_regs(contract, rng, t, w.heap),
            "stack": resolve_stack(contract, rng, t, w.heap),
            "heapsegs": resolve_segs(contract, rng, t, w.heap, w.stubs),
            "globals_fill": resolve_globals_fill(contract, rng, t, w.heap, w.stubs),
            "script_vals": resolve_scripts(contract, rng, t, w.heap, w.stubs),
            "checks": contract["checks"],
            "timeout_ms": contract.get("timeout_ms", 10000),
        }
        tls = resolve_tls(contract, t, w.heap)
        if tls:
            req["tls"] = tls
        xmm = resolve_xmm(contract, rng, t)
        if xmm:
            req["xmm"] = xmm
        if "stack_fill" in contract:
            req["stack_fill"] = contract["stack_fill"]
        try:
            r = w.call(req, timeout=contract.get("timeout_ms", 10000) / 1000.0 + 30)
        except (TimeoutError, BrokenPipeError, ConnectionError, OSError):
            return {"error": "worker-lost", "results": results}
        if check_names is None:
            check_names = [c["name"] for c in r.get("checks", [])]
        results.append(r)
        if not r.get("pass"):
            fails += 1
            if first_fail is None:
                first_fail = {"trial": t, "req_regs": req["regs"], "req_stack": req["stack"],
                              "resp": r}
            if stop_after_fails and fails >= stop_after_fails:
                break
    wall = time.perf_counter() - t0
    hist = {}
    cov = {}
    for r in results:
        try:
            k = (r.get("orig", {}).get("status", "?") + "/" +
                 r.get("rw", {}).get("status", "?"))
        except Exception:
            k = "?"
        hist[k] = hist.get(k, 0) + 1
        try:
            for side in ("orig", "rw"):
                for c in (r.get(side, {}) or {}).get("calls", []) or []:
                    cov[str(c.get("id"))] = cov.get(str(c.get("id")), 0) + 1
        except Exception:
            pass
    return {"results": results, "fails": fails, "first_fail": first_fail,
            "wall_s": wall, "check_names": check_names or [],
            "status_hist": hist, "call_coverage": cov}


def verdict(contract, run, export):
    chash = hashlib.sha1(json.dumps(contract, sort_keys=True).encode()).hexdigest()
    results = run["results"]
    n = len(results)
    agg = {}
    for r in results:
        for c in r.get("checks", []):
            a = agg.setdefault(c["name"], {"pass": 0, "total": 0})
            a["total"] += 1
            if c.get("passed"):
                a["pass"] += 1
    comps = [{"name": k, "passed": v["pass"] == v["total"] and n > 0,
              "pass": v["pass"], "total": v["total"]} for k, v in sorted(agg.items())]
    passed = run.get("fails", 1) == 0 and n > 0 and "error" not in run
    fp_exact = sum(1 for r in results if r.get("fp_exact"))
    us = sum(r.get("trial_us", 0) for r in results)
    orig_ok = sum(1 for r in results if r.get("orig", {}).get("status") == "ok")
    v = {
        "function": contract["function"],
        "passed": passed,
        "inputs_tested": n,
        "comparisons": [{"name": c["name"], "passed": c["passed"]} for c in comps],
        "checker_version": "checker2",
        "name": contract["name"],
        "export": export,
        "contract_hash": chash,
        "trials": n,
        "fails": run.get("fails", 0),
        "comparisons_detail": comps,
        "fp_exact": fp_exact,
        "worker_us_total": us,
        "wall_s": round(run.get("wall_s", 0), 2),
        "status_hist": run.get("status_hist", {}),
        "orig_ok_trials": orig_ok,
        "call_coverage": run.get("call_coverage", {}),
    }
    if run.get("first_fail"):
        ff = run["first_fail"]
        v["first_mismatch"] = {"trial": ff["trial"],
                               "detail": ff["resp"].get("first_mismatch"),
                               "checks": ff["resp"].get("checks")}
    if run.get("error"):
        v["error"] = run["error"]
    return v


def outer_pop_for(contract):
    if "outer_pop" in contract:
        return contract["outer_pop"]
    if contract.get("conv", "cdecl") == "cdecl":
        return 0
    return len(contract.get("stack", [])) * 4


def setup_worker(w, contract, all_exports):
    import copy
    # NOTE: deep-copy: normalizing the worker record must not destroy the
    # contract's own dict script entries (a shared-list bug here once
    # zeroed every heap-pointer script before the first trial, passing
    # vacuously; the content-mutant control caught it).
    scallees = copy.deepcopy(contract.get("callees", []))
    q = {"cmd": "setup", "exe": EXE, "dll": dll_for(contract), "exports": all_exports,
         "callees": scallees,
         "patches": contract.get("patches", []),
         "tailpatches": contract.get("tailpatches", []),
         "outer_pop": outer_pop_for(contract),
         "iat": contract.get("iat", []),
         "globals": contract.get("globals", [])}
    # resolve "edges" scripts to concrete lists for the worker record
    for c in q["callees"]:
        if c.get("script") == "edges":
            c["script"] = list(INT_EDGES)
        elif isinstance(c.get("script"), list):
            c["script"] = [0 if isinstance(v, dict) else v for v in c["script"]]
    r = w.call(q, timeout=120)
    assert r.get("ok"), r
    w.heap = int(r["heap"], 16)
    w.stubs = {k: int(v, 16) for k, v in r.get("stub_addrs", {}).items()}
    return r


def main(argv):
    global EXE, OUT
    args = argv[1:]
    trials_override = None
    if "--trials" in args:
        i = args.index("--trials")
        trials_override = int(args[i + 1])
        args = args[:i] + args[i + 2:]
    if "--exe" in args:
        i = args.index("--exe")
        EXE = args[i + 1]
        args = args[:i] + args[i + 2:]
    if "--out" in args:
        i = args.index("--out")
        OUT = args[i + 1]
        args = args[:i] + args[i + 2:]
    if args == ["--all"]:
        names = sorted(f[:-5] for f in os.listdir(os.path.join(HERE, "contracts")) if f.endswith(".json"))
    elif args == ["--smoke"]:
        names = ["const58", "subf"]
    elif args and all(not a.startswith("-") for a in args):
        names = [a[:-5] if a.endswith(".json") else a for a in args]
    else:
        print(__doc__)
        return 1
    os.makedirs(os.path.join(OUT, "verdicts"), exist_ok=True)
    os.makedirs(os.path.join(OUT, "mutants"), exist_ok=True)
    contracts = [json.load(open(os.path.join(HERE, "contracts", n + ".json"))) for n in names]
    w = Worker()
    summary = []
    t_all = time.perf_counter()
    for c in contracts:
        dll = dll_for(c)
        if not os.path.exists(dll):
            print("=== %s: SKIP (rewrite DLL not found: %s) ===" % (c["name"], dll), flush=True)
            summary.append({"name": c["name"], "skipped": True, "dll": dll})
            continue
        trials = trials_override or (c["trials"] if args != ["--smoke"] else 10)
        print("=== %s (%s) ===" % (c["name"], c["function"]), flush=True)
        c_exports = [c["export"]] + ([c["mut_export"]] if c.get("mut_export") else [])
        c_exports = [e for e in c_exports if not e.startswith("CHECKER_")]
        try:
            sr = setup_worker(w, c, c_exports)
        except Exception as e:
            print("SETUP FAILED, restarting worker:", e, flush=True)
            w.stop()
            w = Worker()
            sr = setup_worker(w, c, c_exports)
        if sr.get("errors"):
            print("setup errors:", sr["errors"], flush=True)
        print("img=%s heap=%s dll=%s stubs=%s" % (sr.get("img_base"), sr.get("heap"),
              sr.get("dll_base"), sr.get("stub_addrs")), flush=True)
        run = run_contract(w, c, c["export"], trials, c["seed"])
        v = verdict(c, run, c["export"])
        json.dump(v, open(os.path.join(OUT, "verdicts", c["name"] + ".json"), "w"), indent=1)
        tus = (v["worker_us_total"] / max(1, v["trials"]))
        print("correct: pass=%s fails=%d/%d fp_exact=%d orig_ok=%d worker=%.0fus/trial wall=%.1fs hist=%s" % (
            v["passed"], v["fails"], v["trials"], v["fp_exact"], v["orig_ok_trials"], tus,
            v["wall_s"], v.get("status_hist")), flush=True)
        if not v["passed"]:
            print("FIRST MISMATCH:", json.dumps(v.get("first_mismatch"), indent=1)[:2000], flush=True)
            try:
                ff = run.get("first_fail") or {}
                resp = ff.get("resp") or {}
                print("ORIG status=%s fault=%s | RW status=%s fault=%s" % (
                    (resp.get("orig") or {}).get("status"), (resp.get("orig") or {}).get("fault"),
                    (resp.get("rw") or {}).get("status"), (resp.get("rw") or {}).get("fault")), flush=True)
            except Exception:
                pass
        if c.get("mut_export"):
            mrun = run_contract(w, c, c["mut_export"], min(trials, 100), c["seed"], stop_after_fails=3)
            mv = verdict(c, mrun, c["mut_export"])
            json.dump(mv, open(os.path.join(OUT, "mutants", c["name"] + ".json"), "w"), indent=1)
            print("mutant: pass=%s fails=%d/%d first=%s" % (
                mv["passed"], mv["fails"], mv["trials"],
                (mv.get("first_mismatch") or {}).get("detail", "")[:160]), flush=True)
            mfail, mtri, mfirst, mwall = mv["fails"], mv["trials"], \
                (mv.get("first_mismatch") or {}).get("detail", "")[:120], mv["wall_s"]
        else:
            print("mutant: (none declared, skipped)", flush=True)
            mfail, mtri, mfirst, mwall = 0, 0, "", 0
        try:
            w.call({"cmd": "teardown"}, timeout=30)
        except Exception:
            w.stop()  # worker already dead (e.g. rewrite aborted it)
            w = Worker()
        summary.append({"name": c["name"], "correct_pass": v["passed"],
                        "correct_fails": v["fails"], "correct_trials": v["trials"],
                        "orig_ok_trials": v["orig_ok_trials"],
                        "status_hist": v.get("status_hist"),
                        "mut_fails": mfail, "mut_trials": mtri,
                        "mut_first": mfirst,
                        "wall_s": round(v["wall_s"] + mwall, 2),
                        "worker_us_per_trial": int(tus)})
    wall_all = time.perf_counter() - t_all
    total_trials = sum(s["correct_trials"] + s["mut_trials"] for s in summary)
    out = {"contracts": summary,
           "wall_s_total": round(wall_all, 1),
           "trials_total": total_trials,
           "trials_per_s": round(total_trials / max(0.1, wall_all), 1)}
    json.dump(out, open(os.path.join(OUT, "results.json"), "w"), indent=1)
    print("TOTAL: %d trials in %.1fs = %.1f trials/s" % (total_trials, wall_all, out["trials_per_s"]), flush=True)
    w.stop()
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
