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
  checker2.py --selftest                driver-only host checks (no worker)
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
orig-ok counts. Version 5 adds, each opt-in per contract and recorded in
the verdict's `features`: x87 entry values and the x87 state check,
XMM2-XMM7 call arguments, snapshots of up to 64 words at any offset, a
read-only shadow for unrelocated absolute reads (`abs_shadow`), and
built-in self-test originals for the checker's own regression. One v5
change is not opt-in: the worker holds the preferred-base window, so an
unrelocated absolute access by the original faults loudly instead of
silently reading worker memory (see "Unrelocated absolute accesses").
The doubles extension (lane k-f64, unadopted) adds, likewise opt-in per
contract and recorded in `features`: an 8-byte rewrite-side transport
for doubles in vector registers (`xmm_from_stack64`), a double answer
the rewrite can read (`ret: "f64xmm0edx"`), scripted doubles
(`{"double": ...}`), and per-trial double XMM entry values (`"double").
Version 7 (candidate) adds, each opt-in per contract and recorded in
the verdict's `features`: near conditional tail-jump patches
(`ctailpatches`), x87 ST0 call arguments (`logst0` with the
`st0_from_stack` transport), a running digest over past-cap calls
(`log_digest`), and 4-byte vector-register narrowing
(`logxmm32_regs`). It also records two contract shapes that were
silently wrong before: an unknown return kind (which fell back to a
32-bit integer) and an unknown word-spec key (which resolved to random
bits). Contracts written before the strict option keep running exactly
as on stock -- the word resolves to the same random bits, a boolean
`checks.ret` compares the return register -- and the verdict records
them under `features.unresolved_word_specs` / `features.ret_boolean`.
A contract with the top-level option `strict: true` is refused instead
if it holds either shape. Every NEW contract should set `strict: true`.

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
   out-param words, TLS slots, XMM values, v5 x87 entry values.
2. The worker fills the heap with a trial-specific pattern, applies the
   segments, resets data sections to pristine + fill, plants the TLS slots,
   loads the XMM entry values (and, on the original side only, pushes the
   x87 entry values), and calls the ORIGINAL through a trampoline
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
a fresh worker (a hung trial taints the process). v4: a trial lost to a
silent/crashed worker is retried on a fresh worker (same inputs, at most 2
retries) and recorded in the verdict; only exhausted retries stay
`worker-lost`.

Contract format
---------------
One JSON file per function:
- `name`, `function` (original RVA hex), `conv` (`cdecl`, `stdcall`,
  `thiscall`, `fastcall`), `export` / `mut_export` (mutant optional),
  `trials`, `seed`, `timeout_ms`, `dll` (optional override of the rewrite
  DLL; also LF_CHECKER_DLL in the environment). v5: `function` may be
  `selftest:<name>`, a built-in self-test original the worker emits
  (`x87_store`, `xmm_call`, `abs_read`; see "Regression"). Such a verdict
  carries `"selftest": true` and never verifies a game function.
- `ret`: compared return channel: `eax`, `ax`, `al`, `edx_eax`, `st0`,
  `xmm0`, `none`.
- `regs`: 7 input specs [eax,ecx,edx,ebx,esi,edi,ebp]: {"any":true},
  {"heap":seg,"plus":bytes} (plus may be a per-trial list),
  {"int":N}, {"small":max} (ranged int), {"cycle":[...]}, {"null":true}.
- `stack`: list of {kind}: `int`, `byte`, `float`, `ptr` (+seg),
  `zero`/`null`, `ptr_or_null` (+seg, NULL on even trials),
  `small` (+max), `smallint` (switch-index-like), `cycle` (+values),
  v4 `srange` (+lo, +hi: signed-range sweep, lo+trial while trial <
  span else uniform rng in [lo,hi]; lo may be negative).
  v3 adds: `heapidx` (+seg, +count default 16: 2/3 of trials index heap
  words as (heap+off)>>2 + trial%count for functions that scale the arg
  by 4, 1/3 wild ints redrawn while arg*4 lands in code pages);
  `ptr_or_null` accepts `phase`/`period` (NULL iff
  (trial+phase)%period == 0) and list/dict `plus` cycled on live
  trials; `ptr`/`regs` `plus` accepts {"rot"|"cycle": [...]} dicts;
  stack `rot` cycles values trial-indexed (unlike `cycle`, which adds
  the arg position).
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
  for vtable/data-slot planting), {"null":...} (literal 0; v3 -- v2 fell
  through to random bits). A bare {"heap":N} with no `plus` means
  segment N when N names one, else a byte offset. A dict with none of
  these keys resolves to random bits exactly as on stock (one draw, same
  stream position) and is recorded in the verdict's
  `features.unresolved_word_specs` as {where, keys}; with `strict: true`
  it is refused at load. New contracts should set `strict: true` so a
  mistyped key fails loudly instead of silently becoming random input.
- `globals`: [{rva,size}] declared ranges (dword-aligned). `globals_fill`:
  `random`, `pristine`, an explicit [{rva,words}] list, `globals_values`
  ([{rva,words}] with word specs, supports heap/stub pointers), or
  `globals_fill_spec` (`heap_ptr`/`cycle`/`words` per range).
- `callees`: [{id,conv,nargs,ret,script,writes,wscript,snap,logxmm,
  xmm0_from_stack,logxmm1,xmm1_from_stack,logxmm_regs,xmm_from_stack,
  seq,preserve}]. `conv` sets cleanup
  (`cdecl` callers clean up, all others pop nargs*4); `ret` is the
  scripted answer channel (`u32`, `u64`, `al`, `f32xmm0`, `f64xmm0`,
  `f32st0`, `f64st0`, v4 `preserve`, and the doubles extension's
  `f64xmm0edx`: the scripted double answers in XMM0 for the original and
  in edx:eax for the rewrite, which reads it as a u64 return); `script`
  is a value list cycled per trial (each
  callee gets its own slot), `"edges"`, or entries resolving to heap
  pointers. A script or `seq` entry may be `{"double": bits}` (an f64 as
  an integer), `{"double": "edges"}` (the f64 edge pool cycled per
  trial), or `{"double": "random"}` (a drawn double): the two words
  reach the stub as lo and hi. `writes` declares out-param stores:
  [{arg:N|reg:"ecx"/"edx", at, n}]; `wscript` is the per-trial word
  matrix cycled alongside. v4 adds an optional `dst` byte offset per
  write (default 0): the words land at [pointer+dst] instead of
  [pointer+0], for out-slots past the pointer. `snap` declares pointed-to snapshots:
  [{kind:"arg"/"ecx"/"edx", idx, n, at}]: n words copied at call time
  from `at` bytes past the pointer (v5; default 0, may be negative,
  |at| <= 0x10000), at most 64 words total per callee (v5; v4 allowed 8
  at offset 0). Setup refuses more, an unknown kind or a bad `at`; a
  snapshot is never truncated. v5 also fixes a second ecx/edx entry in
  one callee, which v4 read through a clobbered register.
  `logxmm` logs the 16 bytes of XMM0 at the call. v5 `logxmm_regs`
  [n, ...] logs any of XMM0-XMM7 and `xmm_from_stack` {"n": idx} is the
  rewrite-side transport for any of them (`movss` from stack arg idx,
  upper lanes zeroed, as the v2/v3 keys do for XMM0/XMM1, which keep
  their meaning). The v5 keys fail closed: a transported register must
  be logged and idx must be below `nargs`. The doubles extension adds
  `xmm_from_stack64` {"n": idx} for doubles: on the rewrite side the
  stub loads the low 8 bytes of XMMn (`movsd`, upper half zeroed) from
  stack args idx (low word) and idx+1 (high word). It fails closed like
  the 4-byte keys (both words declared, register logged) and refuses to
  share a register with them. `logxmm64_regs` [n, ...] narrows the
  comparison of logged registers to their low 8 bytes (the double the
  callee reads; use it when the upper half holds whatever an earlier
  conversion left there rather than a callee input): the stub still logs
  all 16 bytes, the call key keeps the low two words, and the verdict's
  `features` records it. The register must still be logged. v7
  `logxmm32_regs` [n, ...] narrows to the low 4 bytes instead (the
  float the callee reads; use it when the callee never looks at the
  upper 12, which the compiler may fill differently on the rewrite's
  side), never beside the 8-byte narrowing on the same register.
  v7 `logst0` ("f32" or "f64") logs the x87 ST0 call argument for a
  callee that takes its float argument on the float stack: the stub
  stores ST0 without popping it, so the callee still finds it on top,
  and the call key compares one word (f32) or two (f64). v7
  `st0_from_stack` (an argument index) is the rewrite-side transport
  (the stub loads ST0 from that stack word, or word pair for f64, on
  the rewrite side only); like the vector transports it skips the
  stack words on both sides and fails closed (logged, declared). A
  transported call leaves one value on the rewrite's FPU stack that
  safe Rust cannot pop, so the x87 state check stays off on such
  contracts; the argument itself is compared at the call.
  `xmm0_from_stack` names the stack arg the stub loads XMM0 from on
  the rewrite side (transport for xmm0-arg callees; call args are then
  skipped on both sides and only XMM0 is compared). v3 adds `logxmm1`
  and `xmm1_from_stack`, the same pair for XMM1 (either transport
  skips the stack args on both sides and compares the vector regs),
  and `seq`: a per-call answer sequence (at most 16 [lo,hi] steps,
  same entry shapes as `script`) consumed in call order per callee per
  side -- for callees polled in a loop until a sentinel. Calls past
  the last step repeat it. Without `seq` every call in the trial gets
  the script value, as before. v4 adds two register-preservation
  options (both default off, existing stubs byte-identical):
  `ret: "preserve"` sets no scripted answer and exits the stub with
  the entry eax/ecx/edx intact, for callees the original calls after
  its return value is set (e.g. the CRT security-cookie check, which
  genuinely preserves them); any script/seq value is ignored.
  `"preserve": true` keeps the scripted eax answer but restores entry
  ecx (and edx, except for `u64` returns where edx carries the high
  word), for callees whose callers keep registers live across the
  call (e.g. a base constructor that leaves the object pointer in
  ecx). Use it only after confirming the real callee preserves them.
  v4 also adds `eax_from_stack` (the stack arg the stub loads eax from
  on the rewrite side only, mirroring `xmm0_from_stack`; the logged
  entry eax is compared via `call_regs` `"eax"`, and the stack args are
  then skipped on both sides like the other transports) and `noclean`
  (the real callee takes register args but pops nothing: the stub
  returns plain `ret` on the original side and `ret N` on the rewrite
  side, so the rewrite's stack stays balanced).
- `patches`: [{site (RVA hex of an E8), id}]. `tailpatches`:
  [{site (RVA hex of an E9), id}]; the tail stub logs like a normal stub
  then returns straight to the trampoline with `outer_pop + 4`
  (`outer_pop` defaults from `conv` + stack length, overridable).
  `iat`: [{dll,name,id}] import slots rewritten to recorder stubs.
  v7 `ctailpatches`: [{site (RVA hex of a near Jcc, 0F 80..8F), id}];
  the site becomes a call to a per-site stub that re-evaluates the
  same condition on the still-live flags and either logs and
  tail-returns with `outer_pop` (taken) or resumes at site+6 with
  registers, flags and esp untouched (not taken). Short (2-byte)
  conditional jumps have no room for a call and are refused. As with
  tailpatches, the contract author must confirm the site runs at entry
  ESP. Any patch entry may use `site_selftest` + `at` instead of `site`
  to patch the checker's own self-test originals (regression only).
- `tls`: [{slot|slot_rva, seg, plus} | {slot, int}]: fabricated
  TLS slots, planted on the trial thread before each side. Slots 0-63.
- `xmm`: {reg: [4 word specs]}: scripted XMM entry values for any of
  xmm0-xmm7 (both sides since v2); use the string `"float"` for a
  float-edge low word. The doubles extension adds `"double"` at an even
  word index: one per-trial double across that word and the next (f64
  edges first, then drawn doubles, mirroring `"float"`). Unlisted words
  are zero.
- v5 `x87`: [{kind, vals?}, ...]: x87 entry values, ST0 first, at most 8.
  `kind` is `f32`, `f64` (the value an `fld dword`/`fld qword` of those bits
  loads: exact, a signalling NaN quieted) or `f80` (an 80-bit value).
  Without `vals` the driver generates them (f32: FLOAT_EDGES then fbits;
  f64: F64_EDGES then f64bits; f80: F80_EDGES then f80rand, which never
  yield a signalling NaN or an invalid encoding); `vals` cycles literal
  bits per trial (f32/f64: an int; f80: [significand, sign/exponent]).
  The original receives them on its FPU stack (pushed deepest first by
  the trampoline). The rewrite starts with an EMPTY FPU stack and reads
  them through `lf_checker_rt::x87_raw`/`x87_f64`/`x87_f32` (the
  `CHECKER_X87` mirror): a Rust rewrite cannot pop x87 registers, so it
  is treated as having consumed them. Declaring `x87` turns the x87 state
  check on (`checks.x87_state: false` beside it is refused), which then
  also verifies the original consumed exactly its entry values. A rewrite
  DLL without `CHECKER_X87` fails setup for such a contract.
- v5 `abs_shadow` (bool, default false): serve the original's unrelocated
  absolute reads of headers and read-only sections from a read-only copy
  of the file at the preferred base (see "Unrelocated absolute
  accesses"). Random draws are then also kept out of that window.
- `stack_fill`: a defined fill for uninitialized stack (integer, or
  `"pattern"` for the default trial pattern). Use `0` when either side
  reads stack it did not write.
- v3 code-pointer rule: unconstrained random draws (`any`, `int` tails,
  `float`/`byte` words, random heap fills, wide `small`) never produce
  addresses inside the original's code pages; a draw that lands there is
  remapped deterministically (+0x80000000, unmapped) without consuming
  more RNG, so every other draw keeps its v2 value. Contract-authored
  values are never remapped. `"allow_code_pointers": true` restores the
  exact v2 stream for audit.
- `checks`: `ret` (a channel name, not a bool: a boolean compares the
  return register exactly as on stock and is recorded as
  `features.ret_boolean`, refused only with `strict: true`),
  `esp`, `heap`, `stack`, `globals`, `calls`,
  `undeclared` (all bool), `fulldata` (bool), `fp_tol`, `fp64`,
  v5 `x87_state` (bool, default false; implied by `x87`: the final x87
  stack top, abridged tag byte and the 80-bit contents of every valid
  register must be identical; check name `x87`),
  `call_regs` (optional {id:[regs]}, v4 adds `"eax"` for
  `eax_from_stack` callees -- rejected without the transport),
  `call_skip` ({id:[stack arg indexes]} dropped from the call
  comparison), v4 `call_mask` ({id:{idx:mask}}: compare only
  (value & mask) of a pushed argument as raw hex, for one-byte values
  pushed as full words; declared per callee and argument, recorded in
  the verdict, zero rejected) and its alias `call_low8`
  ({id:[idx,...]}, each compared low-byte-only, i.e. mask 0xFF).
- Top-level `log_max` (v4, default 256, max 1024): per-side call-log
  cap. A trial where either side attempts more calls than the cap fails
  its `calls` check (the tail would go uncompared); raise the cap and
  rerun. Previously such trials passed silently. v7 `log_digest`
  (default false): past the cap the stub folds every further call into
  a running digest (two 32-bit FNV accumulators over the callee id,
  the convention-default compared registers (thiscall: ecx; fastcall:
  ecx+edx), nargs and the stack words; words inside the scratch-stack
  window fold as value-minus-esp0, the rest raw) instead of dropping
  it, and the calls check then requires the logged prefix, the
  attempted counts and both digests to match. A digest match shows
  the two sides made the same number of past-cap calls with the same
  (id, default registers, stack words) sequence, up to a 64-bit
  collision; it does NOT show snapshot words, vector registers
  (including narrowed ones), entry eax, the ST0 argument, out-param
  writes or scripted answers past the cap (they are not folded in),
  so use it only when those do not vary past the cap, e.g.
  scalar-argument loops. The verdict's `features` records the option
  and on how many trials it was used.
- Top-level `strict` (default false): refuse at load the two legacy
  shapes the driver otherwise grandfathers (an unknown word-spec key,
  a boolean `checks.ret`). Existing contracts omit it and run exactly
  as on stock with the shapes recorded in the verdict's `features`;
  every NEW contract should set `strict: true` so both fail loudly.

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
  The logged XMM0 bytes are then compared. v5 `xmm_from_stack` does the
  same for any of XMM0-XMM7. The doubles extension's `xmm_from_stack64`
  loads a double (two stack words); its `f64xmm0edx` answer channel puts
  the scripted double in XMM0 for the original and in edx:eax for the
  rewrite, which reads it as a u64 return. v7 `st0_from_stack` has the
  stub load ST0 from a stack word (or word pair for f64) on the rewrite
  side only; the original side passes real ST0, logged by `logst0`.
- v5 `x87_raw(i)` (runtime) reads the x87 entry value the original
  receives in ST(i); `x87_f64(i)`/`x87_f32(i)` round it exactly as an x87
  `fst qword`/`fst dword` does (round to nearest even, the trampoline's
  control word).

Unrelocated absolute accesses (v5)
----------------------------------
The image runs away from its preferred base with relocations applied, so
a forgotten relocation is caught. The executable has absolute operands
WITHOUT relocation entries (it never needs relocating in the game); in the
worker they point at the preferred base, which v4 left to whatever the
worker allocated there, so the original silently read unrelated memory
(devlog "The checker disagreed with the file on one constant": a float
constant read that way made a correct rewrite fail). v5:
- the worker reserves that window at start-up (it is linked at a high
  base so its own image is not in it). An access there faults, and the
  fault detail carries an "abs-window" note with the file address.
  Contracts whose original makes such accesses now fail loudly on the
  original side instead of comparing against worker memory.
- `abs_shadow: true` commits a read-only copy of the file's headers and
  read-only sections there (unrelocated, as the game sees them), readable
  only while the original runs. The rewrite must still read through
  `relocated()`/`global()`; reading the file address directly faults.
  Writable sections stay unmapped in the window (an unrelocated access to
  writable data still faults), and so do writes. Pointer-valued read-only
  data (vtables) holds unrelocated values there, so it cannot match a
  relocated read; the shadow suits constants. A worker whose window is
  not fully free fails setup; the driver retries on fresh workers.

Verdict format (accept.py-compatible)
-------------------------------------
`verdicts/<name>.json` keeps the five required keys (`function`, `passed`,
`inputs_tested`, `comparisons`, `checker_version`) and adds: `name`,
`export`, `contract_hash`, `trials`, `fails`, `comparisons_detail`,
`fp_exact`, `worker_us_total`, `wall_s`, `first_mismatch`, plus
`status_hist` ({"ok/ok": N, ...}) and `orig_ok_trials` (trials where the
original side returned OK; the accept gate should require this to be
non-zero before any fault-driven pass counts). v3 adds `coverage` (trials
completed, checks seen, callees fired on completed trials, distinct
call-sequence shapes, trials where the original wrote), `vacuous` (true
when the verdict has no failures but the coverage rule below refused it)
and `vacuous_reasons`. v4 adds `call_masks_used` (the effective masks,
alias resolved, so a narrowed comparison is always visible), `log_max`
(the cap in force), and `worker_retries` (one record per retried trial:
trial, attempt, loss class and worker exit code). v5 adds `features`
(x87 entry kinds and whether the x87 state check ran, xmm entry
registers, logged and transported call registers per callee, snapshot
words and non-zero offsets per callee, `abs_shadow` and the worker's
reported window coverage, the self-test name) and `selftest: true` for a
self-test verdict. `checker_version` stays "checker4" until the
coordinator adopts v5. The doubles extension adds three `features` keys:
`xmm_call_transport64` (8-byte transports per callee), `xmm_call_cmp64`
(registers compared on their low 8 bytes only, per callee) and
`f64_answers` (callees answering `f64xmm0edx`). v7 adds, only when used:
`xmm_call_cmp32` (registers compared on their low 4 bytes, per callee),
`st0_call_logged`/`st0_call_transport` (ST0 widths and transports, per
callee), `ctail_sites` (conditional tail patches) and `log_digest`
(whether asked, and on how many trials it was used).

v3 coverage rule: a verdict with zero failures still fails as `vacuous`
unless (a) at least `min_orig_ok_share` of trials (default 0.10,
overridable per contract) completed on the original without a fault,
(b) every enabled behavioural check ran on at least one trial, and
(c) every declared callee fired on at least one completed trial (a
callee that cannot fire must be listed in the contract's
`coverage_exempt_callees`, never silently uncovered). The thresholds are
part of the contract so a fault-heavy-but-honest contract stays runnable
and the exemption is reviewable.

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
  callee declares `xmm0_from_stack` (v5: any `xmm_from_stack` register).
  Pass a double as two stack words (low, high) when the callee declares
  `xmm_from_stack64`, and read its `f64xmm0edx` answer as a u64 return.
  Pass an st0-arg value on the stack when the callee declares v7
  `st0_from_stack` (a double as two stack words for f64).
- v5: read x87 entry values through `x87_raw(i)` (exact 80 bits) or
  `x87_f64(i)`/`x87_f32(i)`; prefer the bit converters
  `f80_to_f64_bits`/`f80_to_f32_bits` when the result is stored as bits.
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

v5 adds seven k5 contracts (generated by gen_contracts.py), each with a
mutant the v4 checker could not see:
- k5_x87 (selftest:x87_store): three x87 entry values popped into an f32,
  an f64 and an 80-bit slot; mutant swaps ST0 and ST1.
- k5_x87bal: the same original with no return channel compared; the
  mutant leaves a value in ST0 (only the x87 state check sees it).
- k5_xmm (selftest:xmm_call): XMM2/XMM5 call arguments with rewrite-side
  transports plus an XMM6 entry value; mutant swaps the two registers.
- k5_abs (selftest:abs_read, abs_shadow): unrelocated reads of the
  header page; mutant reads the file address directly (v4: both sides
  read the same worker memory and matched).
- k5_snap / k5_snapneg (original 0x9e09f0, rewrite rw_k2_f4): a 16-word
  snapshot 0x1D8 bytes into the object, and a negative offset as a second
  ECX entry; each mutant disturbs one covered word only while the callee
  runs (final memory identical, so only the call-time snapshot sees it).
- k5_absguard: documented negative (fails by design, no mutant): without
  abs_shadow the original's unrelocated read faults in the reserved
  window ("abs-window" note) while the correct rewrite completes.
The doubles extension adds four k6 contracts: k6_f64arg, k6_f64ans and
k6_f64neg over the self-test original f64_call (floats widened to
doubles across a helper call that answers a double): k6_f64arg (mutant
negates one double argument), k6_f64ans (mutant ignores the double
answer) and k6_f64neg (documented negative: the same mutant without the
new options, passing by design); k6_f64entry over xmm_wide (the high
word of an XMM entry value from two per-trial doubles, proving the
entry path carries all 16 bytes per register).
The self-test originals are a few hand-written instructions in the worker
(build_selftests), used because no tracked original is known to take x87
arguments, pass XMM2-XMM7 to a callee or read through an unrelocated
address. v7 adds ten k7 contracts (generated by hand, in the style of
gen_contracts.py): k7_ctail (selftest:ctail_guard, conditional tail
patched; mutant inverts the guard), k7_st0 (selftest:st0_call, ST0
argument with rewrite-side transport; mutant passes a disturbed word)
with k7_st0_64 (the double-width form) and k7_st0neg (documented
negative: the same mutant without the ST0 option, passing by design),
k7_digest (selftest:digest_loop past a cap of 8 with the digest; mutant
disturbs one past-cap argument) with k7_digestfull (the same pair fully
logged, proving the mutant is a real difference) and k7_digestneg
(documented negative, truncated without the digest), k7_cmp32
(selftest:xmm32_call with the 4-byte narrowing; mutant disturbs the low
word) with k7_cmp32neg (documented negative, unnarrowed) and k7_ctailneg
(documented negative, unpatched). v8 adds three k8 contracts pinning the
globals position fix (each changed word recorded with its own address):
k8_shift (the right value one slot too low passes without the fix),
k8_swap (two values swapped) and k8_single (a one-word range behaves as
before). `checker2.py --selftest` runs the
driver's own host-side checks (no worker needed).

What it can and cannot verify
-----------------------------
Can: multi-callee branch coverage via per-callee answers; callees that
fill structs and buffers; register-indirect, vtable, data-table and tail
calls; frame-pointer call arguments (address skipped, contents compared);
TLS-slot-driven logic (slots 0-63); xmm0-arg callees and xmm entry
values; functions reading uninitialized stack (with a defined fill).
v5: x87 entry values the original consumes; the final x87 stack (top,
tags, valid registers); XMM0-XMM7 call arguments; call-time snapshots of
up to 64 words at any offset; unrelocated absolute reads of read-only
data (with abs_shadow). Doubles extension: doubles in vector registers
across calls, in both directions, with scripted double answers.
v7: near conditional tail jumps; x87 ST0 call arguments; past-cap call
tails via the running digest (id, registers and stack words); vector
registers compared on their low float only.

Still cannot: functions in the encrypted first megabyte of the code
section; behaviour needing a running game (initialized heap graphs, OS
handles); import implementations (calls observed, answers scripted);
timing/`rdtsc`/CPUID paths; self-modifying or preferred-base-dependent
code; concurrency; TLS expansion slots (>= 64); anything after the first
fault in a trial. v5 limits: an original that leaves x87 entry values on
the stack cannot be matched by a Rust rewrite (the x87 check fails);
80-bit signalling NaNs and invalid encodings are not generated; the
4-byte transports load 4 bytes (`movss`) and the doubles extension's
8-byte transports load a double (`movsd`); unrelocated writes and
unrelocated accesses to writable data always fault; the x87 control
word, MXCSR and FPU condition codes after return are not compared.
v7 limits: short (2-byte) conditional tail jumps cannot be patched;
the digest covers only callee id, entry registers and stack words
past the cap (no snapshots, vector registers or out-params there);
st0-transported calls leave the argument on the rewrite's FPU stack,
so the x87 state check stays off on those contracts.

Current limits: one worker is single-trial-at-a-time (run one driver per
core); snapshots cap at 64 words per callee (v5; 8 before) and out-param
words at 16 per callee; call arguments log and compare up to 40 words per callee and setup
rejects more (v2 silently compared only the first 8); the call log holds
256 calls per side by default and trials past it fail loudly (raise
`log_max` to 1024, or ask for the v7 `log_digest`); computed `jmp reg`
with non-vtable targets still needs per-case analysis. `log_digest`
refuses to combine with checks.call_regs/call_skip/call_mask.
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


# v5 x87 entry values. FLOAT_EDGES serve "f32" entries; these serve "f64"
# and "f80" entries. The f80 pool exercises rounding (ties to even, carries
# into the exponent, overflow, denormal results) as stored by fst; it holds
# no signalling NaN and no encoding the FPU rejects (unnormal, pseudo-NaN),
# whose store behaviour is not modelled.
F64_EDGES = [0x0000000000000000, 0x8000000000000000, 0x3FF0000000000000,
             0xBFF0000000000000, 0x3FE0000000000000, 0x400921FB54442D18,
             0x7FF0000000000000, 0xFFF0000000000000, 0x7FF8000000000000,
             0x0010000000000000, 0x000FFFFFFFFFFFFF, 0x0000000000000001,
             0x47EFFFFFE0000000, 0x3810000000000000, 0x36A0000000000000,
             0x3FF0000010000000, 0xC00FFFFFFFFFFFFF]
F80_EDGES = [(0, 0x0000), (0, 0x8000), (1 << 63, 0x3FFF), (1 << 63, 0xBFFF),
             (1 << 63, 0x7FFF), (1 << 63, 0xFFFF), (0xC000000000000000, 0x7FFF),
             (0xC000000000000000, 0xFFFF), (0xC000000000000801, 0x7FFF),
             (0xC90FDAA22168C235, 0x4000), (0x8000000000000400, 0x3FFF),
             (0x8000000000000C00, 0x3FFF), (0x8000008000000000, 0x3FFF),
             (0x8000018000000000, 0x3FFF), (0xFFFFFFFFFFFFFFFF, 0x3FFF + 127),
             (0xFFFFFFFFFFFFFFFF, 0x3FFF + 1023), (1 << 63, 0x0001), (1, 0x0000),
             (1 << 63, 0x3FFF - 1022), (1 << 63, 0x3FFF - 1074),
             (0xC000000000000000, 0x3FFF - 149), (0xAAAAAAAAAAAAAAAB, 0x3FFD)]


def f32_to_f80(bits):
    """(significand, sign/exponent) an x87 `fld dword` loads for these f32
    bits: exact, denormals normalised, a signalling NaN quieted."""
    bits &= 0xFFFFFFFF
    sign = (bits >> 31) << 15
    e = (bits >> 23) & 0xFF
    f = bits & 0x7FFFFF
    if e == 0xFF:
        q = (1 << 62) if f else 0
        return ((1 << 63) | (f << 40) | q, sign | 0x7FFF)
    if e == 0:
        if f == 0:
            return (0, sign)
        msb = f.bit_length() - 1
        return (f << (63 - msb), sign | (msb - 149 + 16383))
    return ((1 << 63) | (f << 40), sign | (e - 127 + 16383))


def f64_to_f80(bits):
    """As f32_to_f80, for an x87 `fld qword`."""
    bits &= 0xFFFFFFFFFFFFFFFF
    sign = (bits >> 63) << 15
    e = (bits >> 52) & 0x7FF
    f = bits & ((1 << 52) - 1)
    if e == 0x7FF:
        q = (1 << 62) if f else 0
        return ((1 << 63) | (f << 11) | q, sign | 0x7FFF)
    if e == 0:
        if f == 0:
            return (0, sign)
        msb = f.bit_length() - 1
        return (f << (63 - msb), sign | (msb - 1074 + 16383))
    return ((1 << 63) | (f << 11), sign | (e - 1023 + 16383))


def f64bits(rng):
    """f64 bits biased like fbits: edges, denormals, small normals, random."""
    r = rng.random()
    if r < 0.15:
        return rng.choice(F64_EDGES)
    if r < 0.20:
        return rng.getrandbits(52)
    if r < 0.25:
        return (0x3FF - 30 + rng.randint(0, 60)) << 52 | rng.getrandbits(52)
    return rng.getrandbits(64)


def f80rand(rng):
    """A random 80-bit value: mostly normal around 1.0, some across the whole
    exponent range, some denormal. Never a NaN or an invalid encoding."""
    sign = rng.getrandbits(1) << 15
    r = rng.random()
    if r < 0.1:
        return (rng.getrandbits(63) >> rng.randint(0, 62), sign)
    if r < 0.3:
        e = rng.randint(1, 0x7FFE)
    else:
        e = 0x3FFF + rng.randint(-200, 200)
    return ((1 << 63) | rng.getrandbits(63), sign | e)


def fbits(rng):
    r = rng.random()
    if r < 0.15:
        return rng.choice(FLOAT_EDGES)
    if r < 0.20:  # subnormal
        return rng.getrandbits(23)
    if r < 0.25:  # small normal
        return 0x00800000 | rng.getrandbits(23)
    v = rand32(rng)
    if (v & 0x7F800000) == 0x7F800000 and (v & 0x007FFFFF) != 0:
        v |= 0x00400000  # quiet any signaling NaN (x87/SSE quieting differs)
    return v


# v3 code-pointer rule (lane a-08): while the rewrite runs, the worker
# revokes the original's code pages, so a trial input that points into them
# is unjudgeable by design -- the original reads the byte, the faithful
# rewrite faults and is misreported as cheating. Unconstrained random draws
# therefore never produce code-range values: a draw that lands there is
# remapped deterministically (+0x80000000, into unmapped space, so the
# trial stays an honest wild-pointer parity trial) without consuming more
# RNG, so every other draw keeps its v2 value. Contract-authored values
# (literals, cycles, heap/stub pointers) are never remapped: the contract
# asked for them. "allow_code_pointers": true restores the v2 stream
# exactly (for audit; such trials cannot pass when dereferenced).
CODE_LO = 0
CODE_HI = 0
CODE_ALLOW = False
# v5: with abs_shadow the preferred-base window is readable by the original
# and not by the rewrite, so a random draw landing there is unjudgeable in
# the same way as a code pointer and is remapped the same way. Only set for
# abs_shadow contracts, so every other contract keeps its v4 stream.
ABS_LO = 0
ABS_HI = 0


def demap(v):
    v &= 0xFFFFFFFF
    if not CODE_ALLOW and (CODE_LO <= v < CODE_HI or ABS_LO <= v < ABS_HI):
        v = (v + 0x80000000) & 0xFFFFFFFF
    return v


def rand32(rng):
    return demap(rng.getrandbits(32))


class Worker:
    def __init__(self):
        self.heap = 0
        self.stubs = {}
        self.setup = {}
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
    return rand32(rng)


def gen_small(rng, trial, mx):
    cands = [v for v in (0, 1, 2, 3, 4, 7, 8, 15, 16, mx, mx - 1) if 0 <= v <= mx]
    if trial < 24 and cands:
        return cands[trial % len(cands)]
    return demap(rng.randint(0, mx))


def gen_byteword(rng, trial):
    lo = BYTE_EDGES[trial % len(BYTE_EDGES)] if trial < 24 else rng.getrandbits(8)
    return demap(lo | (rng.getrandbits(24) << 8))


# v7: the primary keys of a unified word spec. "plus" is a secondary
# key (it only rides beside "heap"); {"double":...} and
# {"lo":...,"hi":...} are script/seq wrappers resolved before resolve_word.
# v7b: a dict with none of these keys resolves to random bits exactly as
# on stock (grandfathered; the verdict records it under
# features.unresolved_word_specs), unless the contract sets strict:true,
# which refuses it. Every NEW contract should set strict: true.
WORD_SPEC_KEYS = frozenset(("stub", "heap", "heap_off", "int", "small",
                             "cycle", "rot", "float", "null", "any"))


def resolve_word(spec, rng, trial, heap, contract, stubs):
    """Resolve one unified word spec to a u32.

    int -> literal; {"int":N}; {"heap":seg,"plus":n} live pointer;
    {"heap_off":bytes} heap-base-relative pointer; {"any":true} random;
    {"small":max} ranged int; {"cycle":[...]} per-trial rotation;
    {"rot":[...]} alias; {"stub":id} recorder-stub address (vtable planting);
    {"float":true} float-edge-biased bits. v7b: an unknown key resolves to
    random bits exactly as on stock (same stream position), unless the
    contract sets strict:true, which refuses it.
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
    if "null" in spec:  # v3: explicit null (v2 fell through to random bits)
        return 0
    if "any" in spec:
        return rand32(rng)  # {"any": true}
    # v7b: without strict, an unknown word-spec key resolves exactly as on
    # stock (one rand32 draw, same stream position, so every later draw
    # keeps its stock value); validation has already recorded it under
    # features.unresolved_word_specs. With strict:true it is refused.
    if contract.get("strict", False):
        raise ValueError("contract %s: unknown word-spec keys %s (known: %s)"
                         % (contract.get("name"), sorted(spec), ", ".join(sorted(WORD_SPEC_KEYS))))
    return rand32(rng)


def resolve_regs(contract, rng, trial, heap):
    out = []
    for i, spec in enumerate(contract["regs"]):
        if "heap" in spec:
            seg = contract["heapsegs"][spec["heap"]]
            p = resolve_plus(spec.get("plus", 0), trial)
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


def resolve_plus(plus, trial):
    # v3: "plus" may be a scalar, a list cycled per trial (v2 form), or a
    # {"rot"|"cycle": [...]} dict cycled per trial (v1 r-s03/r-s09 form).
    if isinstance(plus, dict):
        seq = plus.get("rot", plus.get("cycle"))
        if seq is not None:
            return seq[trial % len(seq)]
        return 0
    if isinstance(plus, list):
        return plus[trial % len(plus)]
    return plus


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
            p = resolve_plus(s.get("plus", 0), trial)
            out.append((heap + seg["off"] + p) & 0xFFFFFFFF)
        elif k == "ptr_or_null":  # q-06: NULL on even trials, pointer on odd
            # v3: optional "phase"/"period" (v1 r-s03 rule: null when
            # (trial+phase) % period == 0; defaults 0 and 2 reproduce v2)
            # and list/dict "plus" cycled on live trials (v1 tokenize form).
            if (trial + s.get("phase", 0)) % s.get("period", 2) == 0:
                out.append(0)
            else:
                seg = contract["heapsegs"][s["seg"]]
                p = resolve_plus(s.get("plus", 0), trial)
                out.append((heap + seg["off"] + p) & 0xFFFFFFFF)
        elif k == "heapidx":  # v3: v1 r-s07 kind, exact port
            # The function scales this arg by 4 and dereferences it. 2/3 of
            # trials index heap words ((heap+off)>>2 + trial%count) so
            # post-load logic runs on real data; 1/3 stay wild ints (fault
            # parity), redrawn while arg*4 would land in the original's
            # code pages (unjudgeable there: item 4).
            if trial % 3 != 0:
                seg = contract["heapsegs"][s["seg"]]
                base = (heap + seg["off"]) & 0xFFFFFFFF
                span = s.get("count", 16)
                assert base % 4 == 0
                out.append(((base >> 2) + (trial % span)) & 0xFFFFFFFF)
            else:
                v = gen_int(rng, trial, len(out))
                for _ in range(100):
                    if not (CODE_LO <= ((v * 4) & 0xFFFFFFFF) < CODE_HI):
                        break
                    v = rng.getrandbits(32)
                out.append(v)
        elif k == "srange":  # v4: v1 r-s18 signed-range kind (a-F03/a-09
            # translated these by hand; now native). {"kind":"srange",
            # "lo":L,"hi":H}: trial-indexed sweep lo+trial while trial <
            # span, uniform rng in [lo,hi] after. lo may be negative
            # (masked to u32); the sweep is contract-authored (undemapped),
            # the rng tail is demapped like gen_small.
            lo, hi = s["lo"], s["hi"]
            span = hi - lo + 1
            assert span >= 1, s
            if trial < span:
                out.append((lo + trial) & 0xFFFFFFFF)
            else:
                out.append(demap(rng.randint(lo, hi)))
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
        elif k == "rot":  # v3: v1 r-s03 kind; trial-indexed, unlike "cycle"
            vals = s["values"]
            out.append(vals[trial % len(vals)] & 0xFFFFFFFF)
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
                words.append(rand32(rng))
            else:
                words.append(rand32(rng))
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


def resolve_double_bits(spec, rng, trial):
    """Doubles extension: the f64 bits a {"double": ...} script entry names:
    an int is literal bits, "edges" cycles F64_EDGES per trial (no RNG, so
    the stream is untouched), "random" draws f64bits."""
    if spec == "edges":
        return F64_EDGES[trial % len(F64_EDGES)]
    if spec == "random":
        return f64bits(rng)
    return spec & 0xFFFFFFFFFFFFFFFF


def resolve_scripts(contract, rng, trial, heap, stubs):
    out = []
    for c in contract.get("callees", []):
        sc = c["script"]
        if sc == "edges":
            v, hi = INT_EDGES[trial % len(INT_EDGES)], 0
        else:
            raw = sc[trial % len(sc)]
            if isinstance(raw, dict) and "double" in raw:  # scripted double
                bits = resolve_double_bits(raw["double"], rng, trial)
                v, hi = bits & 0xFFFFFFFF, (bits >> 32) & 0xFFFFFFFF
            elif isinstance(raw, dict) and "lo" in raw:  # 64-bit script word
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
        sq = c.get("seq")  # v3 per-call answer sequence (lane r-b04)
        if sq is not None:
            if len(sq) > 16:
                raise ValueError("callee %s seq exceeds 16 steps" % c["id"])
            steps = []
            for raw in sq:
                if isinstance(raw, dict) and "double" in raw:
                    bits = resolve_double_bits(raw["double"], rng, trial)
                    steps.append([bits & 0xFFFFFFFF, (bits >> 32) & 0xFFFFFFFF])
                    continue
                if isinstance(raw, dict) and "lo" in raw:  # 64-bit step word
                    slo = resolve_script_val(raw["lo"], rng, trial, heap,
                                             contract, stubs)
                    shi = resolve_script_val(raw.get("hi", 0), rng, trial,
                                             heap, contract, stubs)
                else:
                    slo = resolve_script_val(raw, rng, trial, heap,
                                             contract, stubs)
                    shi = 0
                steps.append([slo & 0xFFFFFFFF, shi & 0xFFFFFFFF])
            e["seq"] = steps
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


X87_KINDS = ("f32", "f64", "f80")


def resolve_x87(contract, rng, trial):
    """v5: the trial's x87 entry values, ST0 first, each as the worker's
    [significand lo, significand hi, sign/exponent] words; None when the
    contract declares none (so no RNG is consumed and v4 streams stay
    bit-identical). Entry kinds: "f32" (float-edge words, then fbits),
    "f64" (F64_EDGES, then f64bits), "f80" (F80_EDGES, then f80rand); "vals"
    instead cycles literal bits per trial (f32/f64: an int; f80: [man, sexp])."""
    xs = contract.get("x87")
    if not xs:
        return None
    out = []
    for slot, e in enumerate(xs):
        k = e["kind"]
        vals = e.get("vals")
        if k == "f32":
            if vals is not None:
                b = vals[trial % len(vals)]
            elif trial < len(FLOAT_EDGES):
                b = FLOAT_EDGES[(trial + slot) % len(FLOAT_EDGES)]
            else:
                b = fbits(rng)
            man, sexp = f32_to_f80(b)
        elif k == "f64":
            if vals is not None:
                b = vals[trial % len(vals)]
            elif trial < len(F64_EDGES):
                b = F64_EDGES[(trial + slot) % len(F64_EDGES)]
            else:
                b = f64bits(rng)
            man, sexp = f64_to_f80(b)
        else:
            if vals is not None:
                man, sexp = vals[trial % len(vals)]
            elif trial < len(F80_EDGES):
                man, sexp = F80_EDGES[(trial + slot) % len(F80_EDGES)]
            else:
                man, sexp = f80rand(rng)
        out.append([man & 0xFFFFFFFF, (man >> 32) & 0xFFFFFFFF, sexp & 0xFFFF])
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
            elif isinstance(sp, str) and sp == "double":
                # Doubles extension: one per-trial double across this word
                # and the next (edges first, then f64bits, mirroring
                # "float"). Must land on an even word with room for two.
                if len(words) % 2 or len(words) + 1 >= 4:
                    raise ValueError(
                        "contract %s: xmm %s \"double\" needs an even word "
                        "index with room for two words"
                        % (contract.get("name"), reg))
                if trial < len(F64_EDGES):
                    bits = F64_EDGES[trial % len(F64_EDGES)]
                else:
                    bits = f64bits(rng)
                words.append(bits & 0xFFFFFFFF)
                words.append((bits >> 32) & 0xFFFFFFFF)
            else:
                words.append(sp & 0xFFFFFFFF)
        if len(words) > 4:
            raise ValueError("contract %s: xmm %s expands past 4 words"
                             % (contract.get("name"), reg))
        out[str(reg)] = words
    return out


def build_trial_req(contract, rng, t, heap, stubs, export, seed):
    """One trial request. Shared by the main loop and the retry replay so
    regenerated inputs after a worker restart are bit-identical."""
    fn = contract["function"]
    selftest = fn[len("selftest:"):] if fn.startswith("selftest:") else None
    req = {
        "cmd": "trial",
        "fn_rva": 0 if selftest else int(fn, 16),
        "export": export,
        "trial": t,
        "seed": seed & 0xFFFFFFFF,
        "regs": resolve_regs(contract, rng, t, heap),
        "stack": resolve_stack(contract, rng, t, heap),
        "heapsegs": resolve_segs(contract, rng, t, heap, stubs),
        "globals_fill": resolve_globals_fill(contract, rng, t, heap, stubs),
        "script_vals": resolve_scripts(contract, rng, t, heap, stubs),
        "checks": contract["checks"],
        "timeout_ms": contract.get("timeout_ms", 10000),
    }
    tls = resolve_tls(contract, t, heap)
    if tls:
        req["tls"] = tls
    xmm = resolve_xmm(contract, rng, t)
    if xmm:
        req["xmm"] = xmm
    x87 = resolve_x87(contract, rng, t)  # v5; None (no RNG used) when absent
    if x87:
        req["x87"] = x87
    if selftest:  # v5 built-in self-test original (checker regression only)
        req["fn_selftest"] = selftest
    if "stack_fill" in contract:
        req["stack_fill"] = contract["stack_fill"]
    return req


# v4: bounded retries of a lost trial on a fresh worker. Many lanes report
# a few worker-lost trials per run that pass on rerun; the driver now
# retries the same trial inputs (regenerated identically) instead of
# abandoning the contract, and records every retry in the verdict.
WORKER_RETRIES = 2


def _retry_lost(w, contract, export, seed, t, req, retries, first_err):
    """Retry trial t on a fresh worker (bounded). Returns the trial response,
    or None when the retries are exhausted (the contract is then worker-lost
    as before). Appends one record per attempt to `retries` with the loss
    class (timeout vs pipe vs crash exit code) for the verdict."""
    kind = type(first_err).__name__
    for attempt in range(WORKER_RETRIES):
        try:
            code = w.p.poll()
        except Exception:
            code = "unknown"
        # None = process alive but silent (timeout/hang); an int = the
        # worker exited (crash or watchdog exit) or the pipe broke.
        retries.append({"trial": t, "attempt": attempt + 1, "loss": kind,
                        "worker_code": code})
        try:
            w.stop()
            w.start()
            # Full export list, not just the running export: the worker is
            # shared by the correct and mutant runs, and narrowing here
            # would break the later run with "unknown export".
            exps = [export] + ([contract["mut_export"]]
                               if contract.get("mut_export") else [])
            exps = [e for e in exps if not e.startswith("CHECKER_")]
            setup_worker(w, contract, exps)
        except Exception as e:
            kind = "resetup:%s" % type(e).__name__
            continue
        # A fresh worker may map the image/heap elsewhere (preferred
        # addresses can be taken under load), which would stale heap
        # pointers baked into the saved request. Regenerate trial t's
        # inputs by replaying the seeded stream, then send. Identical to
        # the lost attempt when the mapping is stable (the common case);
        # valid either way, since every trial is self-consistent.
        try:
            rrng = random.Random(seed)
            for i in range(t):
                build_trial_req(contract, rrng, i, w.heap, w.stubs,
                                export, seed)
            req = build_trial_req(contract, rrng, t, w.heap, w.stubs,
                                  export, seed)
            r = w.call(req, timeout=contract.get("timeout_ms", 10000)
                       / 1000.0 + 30)
            return (r, req)
        except (TimeoutError, BrokenPipeError, ConnectionError,
                OSError) as e:
            kind = type(e).__name__
            continue
        except Exception as e:
            kind = "call:%s" % type(e).__name__
            continue
    return None


def run_contract(w, contract, export, trials, seed, stop_after_fails=None):
    global CODE_ALLOW
    CODE_ALLOW = bool(contract.get("allow_code_pointers", False))
    rng = random.Random(seed)
    results = []
    fails = 0
    first_fail = None
    check_names = None
    retries = []
    t0 = time.perf_counter()
    for t in range(trials):
        req = build_trial_req(contract, rng, t, w.heap, w.stubs, export, seed)
        try:
            r = w.call(req, timeout=contract.get("timeout_ms", 10000) / 1000.0 + 30)
        except (TimeoutError, BrokenPipeError, ConnectionError, OSError) as e:
            rr = _retry_lost(w, contract, export, seed, t, req, retries, e)
            if rr is None:
                return {"error": "worker-lost", "results": results,
                        "worker_retries": retries}
            r, req = rr
        if check_names is None:
            check_names = [c["name"] for c in r.get("checks", [])]
        # Keep only what the verdict and the coverage report read. A full
        # trial response carries every compared word; holding a thousand of
        # them per contract drove this process past 8 GB on long runs.
        results.append(_slim(r))
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
            "setup": getattr(w, "setup", {}),
            "wall_s": wall, "check_names": check_names or [],
            "status_hist": hist, "call_coverage": cov,
            "worker_retries": retries}


def _slim(r):
    """The fields of one trial response that verdict() and coverage_of() use."""
    def side(s):
        s = s or {}
        return {"status": s.get("status"),
                "calls": [{"id": c.get("id")} for c in (s.get("calls") or [])],
                "heap_n": s.get("heap_n"), "stack_n": s.get("stack_n"),
                "globals_writes": bool(s.get("globals_writes")),
                "log_attempted": s.get("log_attempted", 0),
                "log_logged": s.get("log_logged", 0)}
    return {"pass": r.get("pass"),
            "checks": [{"name": c.get("name"), "passed": c.get("passed")}
                       for c in r.get("checks", [])],
            "fp_exact": r.get("fp_exact"), "trial_us": r.get("trial_us", 0),
            "orig": side(r.get("orig")), "rw": side(r.get("rw"))}


def coverage_of(contract, results):
    """v3 coverage: what the trials actually exercised.

    Returns a dict with the trials that completed on the original, the
    checks that ran at least once, the callees that fired on a completed
    trial, the distinct call-sequence shapes observed (a branch proxy:
    different paths make different call sequences), and the trials where
    the original wrote heap/stack/globals.
    """
    n = len(results)
    checks_seen = set()
    for r in results:
        for c in r.get("checks", []):
            checks_seen.add(c["name"])
    declared = [str(c["id"]) for c in contract.get("callees", [])]
    fired_on_ok = {}
    branch_sigs = set()
    heap_wrote = stack_wrote = globals_wrote = 0
    orig_ok = 0
    for r in results:
        o = r.get("orig", {}) or {}
        if o.get("status") != "ok":
            continue
        orig_ok += 1
        seq = []
        for c in o.get("calls", []) or []:
            sid = str(c.get("id"))
            fired_on_ok[sid] = fired_on_ok.get(sid, 0) + 1
            seq.append(sid)
        branch_sigs.add(tuple(seq))
        if (o.get("heap_n") or 0) > 0:
            heap_wrote += 1
        if (o.get("stack_n") or 0) > 0:
            stack_wrote += 1
        if o.get("globals_writes"):
            globals_wrote += 1
    checks_cfg = contract.get("checks", {})
    enabled = []
    if checks_cfg.get("ret", "eax") != "none":
        enabled.append("ret")
    for k in ("esp", "heap", "stack", "globals", "calls", "undeclared"):
        if checks_cfg.get(k, True):
            enabled.append(k)
    if x87_state_on(contract):  # v5
        enabled.append("x87")
    missing = [k for k in enabled if k not in checks_seen]
    exempt = set(str(x) for x in contract.get("coverage_exempt_callees", []))
    unfired = sorted(i for i in declared if i not in fired_on_ok and i not in exempt)
    examples = sorted(">".join(s) if s else "(no calls)" for s in branch_sigs)[:5]
    return {
        "trials": n,
        "orig_ok_trials": orig_ok,
        "orig_ok_share": round(orig_ok / max(1, n), 4),
        "checks_seen": sorted(checks_seen),
        "checks_missing": sorted(missing),
        "callees_declared": sorted(declared),
        "callees_fired_on_ok": {k: fired_on_ok[k] for k in sorted(fired_on_ok)},
        "callees_unfired": unfired,
        "branch_shapes": len(branch_sigs),
        "branch_examples": examples,
        "heap_wrote_trials": heap_wrote,
        "stack_wrote_trials": stack_wrote,
        "globals_wrote_trials": globals_wrote,
    }


def resolved_masks(contract):
    """v4: the effective per-argument call masks, alias resolved.

    Merges checks.call_mask {id:{idx:mask}} over checks.call_low8
    {id:[idx,...]} (each alias entry = 0xFF), explicit entries winning.
    Values are hex strings. The verdict records this so a narrowed
    comparison is always visible."""
    out = {}
    checks = contract.get("checks", {})
    low8 = checks.get("call_low8") or {}
    for cid, idxs in low8.items():
        for i in idxs:
            out.setdefault(str(cid), {})[str(i)] = "0xff"
    masks = checks.get("call_mask") or {}
    for cid, per in masks.items():
        for i, m in per.items():
            v = int(m, 16) if isinstance(m, str) else m
            out.setdefault(str(cid), {})[str(i)] = "0x%x" % (v & 0xFFFFFFFF)
    return out


def x87_state_on(contract):
    """v5: the x87 state check runs when the contract declares x87 entry
    values (it verifies the original consumed them) or asks for it."""
    return bool(contract.get("x87")) or bool(
        (contract.get("checks", {}) or {}).get("x87_state", False))


SNAP_CAP_WORDS = 64     # v5 per-callee snapshot cap (the worker enforces it too)
SNAP_AT_LIMIT = 0x10000  # v5 bound on |at|


def validate_v5(contract):
    """v5: fail-fast checks of the v5 contract keys (x87, snapshots with
    `at` and more than 8 words, XMM0-XMM7 call options, abs_shadow,
    self-test originals). Contracts without these keys are untouched."""
    name = contract.get("name")
    xs = contract.get("x87")
    if xs is not None:
        if not isinstance(xs, list) or not 1 <= len(xs) <= 8:
            raise ValueError("contract %s: x87 needs 1-8 entries (st0 first)" % name)
        for i, e in enumerate(xs):
            if e.get("kind") not in X87_KINDS:
                raise ValueError("contract %s: x87 st%d kind %r (f32, f64, f80)"
                                 % (name, i, e.get("kind")))
            vals = e.get("vals")
            if vals is not None:
                if not vals:
                    raise ValueError("contract %s: x87 st%d vals is empty" % (name, i))
                for v in vals:
                    ok = (isinstance(v, list) and len(v) == 2 and 0 <= v[0] < 1 << 64
                          and 0 <= v[1] < 1 << 16) if e["kind"] == "f80" else \
                        (isinstance(v, int) and 0 <= v < 1 << (32 if e["kind"] == "f32" else 64))
                    if not ok:
                        raise ValueError("contract %s: x87 st%d value %r does not fit %s"
                                         % (name, i, v, e["kind"]))
        if (contract.get("checks", {}) or {}).get("x87_state", True) is False:
            raise ValueError("contract %s: x87 entry values need the x87 state check "
                             "(x87_state false refused)" % name)
    for c in contract.get("callees", []):
        cid = c.get("id")
        total = 0
        for sn in c.get("snap", []) or []:
            if sn.get("kind", "arg") not in ("arg", "ecx", "edx"):
                raise ValueError("contract %s: callee %s snap kind %r" % (name, cid, sn.get("kind")))
            at = sn.get("at", 0)
            if not isinstance(at, int) or abs(at) > SNAP_AT_LIMIT:
                raise ValueError("contract %s: callee %s snap at %r (integer, |at| <= %d)"
                                 % (name, cid, at, SNAP_AT_LIMIT))
            total += sn.get("n", 0)
        if total > SNAP_CAP_WORDS:
            raise ValueError("contract %s: callee %s snap declares %d words (cap %d)"
                             % (name, cid, total, SNAP_CAP_WORDS))
        regs = c.get("logxmm_regs")
        if regs is not None and (not isinstance(regs, list) or
                                 any(not isinstance(r, int) or not 0 <= r < 8 for r in regs)):
            raise ValueError("contract %s: callee %s logxmm_regs %r (registers 0-7)"
                             % (name, cid, regs))
        logged = set(regs or [])
        if c.get("logxmm"):
            logged.add(0)
        if c.get("logxmm1"):
            logged.add(1)
        regs64 = c.get("logxmm64_regs")
        if regs64 is not None and (not isinstance(regs64, list) or
                                   any(not isinstance(r, int) or not 0 <= r < 8
                                       for r in regs64)):
            raise ValueError("contract %s: callee %s logxmm64_regs %r (registers 0-7)"
                             % (name, cid, regs64))
        for r in regs64 or []:
            if r not in logged:
                raise ValueError("contract %s: callee %s narrows xmm%d to 8 bytes without "
                                 "logging it" % (name, cid, r))
        for k, idx in (c.get("xmm_from_stack") or {}).items():
            r = int(k)
            if not 0 <= r < 8 or not isinstance(idx, int) or not 0 <= idx < c.get("nargs", 0):
                raise ValueError("contract %s: callee %s xmm_from_stack %s:%r (register 0-7, "
                                 "a declared stack argument)" % (name, cid, k, idx))
            if r not in logged:
                raise ValueError("contract %s: callee %s transports xmm%d without logging it "
                                 "(the argument would be compared nowhere)" % (name, cid, r))
        narrow = {int(k) for k in (c.get("xmm_from_stack") or {})}
        if c.get("xmm0_from_stack") is not None:
            narrow.add(0)
        if c.get("xmm1_from_stack") is not None:
            narrow.add(1)
        for k, idx in (c.get("xmm_from_stack64") or {}).items():
            # Doubles extension: the high word comes from the next stack
            # argument, so both must be declared; never beside the 4-byte
            # transport on the same register.
            r = int(k)
            if not 0 <= r < 8 or not isinstance(idx, int) \
                    or not 0 <= idx + 1 < c.get("nargs", 0):
                raise ValueError("contract %s: callee %s xmm_from_stack64 %s:%r (register 0-7, "
                                 "a declared stack-argument pair)" % (name, cid, k, idx))
            if r in narrow:
                raise ValueError("contract %s: callee %s transports xmm%d as 4 bytes and "
                                 "8 bytes (pick one)" % (name, cid, r))
            if r not in logged:
                raise ValueError("contract %s: callee %s transports xmm%d without logging it "
                                 "(the argument would be compared nowhere)" % (name, cid, r))
        for key in ("script", "seq"):
            entries = c.get(key)
            if entries is None or entries == "edges":
                continue
            for e in entries:
                if isinstance(e, dict) and "double" in e:
                    d = e["double"]
                    if not (isinstance(d, int) and 0 <= d < 1 << 64
                            or d in ("edges", "random")):
                        raise ValueError("contract %s: callee %s %s double %r "
                                         "(bits, edges or random)" % (name, cid, key, d))
    for reg, specs in (contract.get("xmm") or {}).items():
        # Doubles extension: "double" fills two words from an even word.
        words = 0
        for sp in specs:
            if sp == "double":
                if words % 2 or words + 1 >= 4:
                    raise ValueError("contract %s: xmm %s \"double\" needs an even word "
                                     "index with room for two words" % (name, reg))
                words += 2
            else:
                words += 1
        if words > 4:
            raise ValueError("contract %s: xmm %s expands past 4 words" % (name, reg))
    fn = contract.get("function", "")
    if fn.startswith("selftest:") and fn[len("selftest:"):] not in SELFTESTS:
        raise ValueError("contract %s: unknown self-test original %s (known: %s)"
                         % (name, fn, ", ".join(SELFTESTS)))


# v5 built-in self-test originals the worker emits (checker regression only).
# The doubles extension adds f64_call (floats widened to doubles across a
# helper call that answers a double) and xmm_wide (the high word of an XMM
# entry value, proving the entry path carries all 16 bytes per register).
# v7 adds ctail_guard (a guard ending in a near conditional tail jump),
# st0_call/st0_call64 (a callee taking its float/double argument in ST0),
# digest_loop (a call loop for past-cap digests) and xmm32_call (a callee
# reading only the low float of a vector register).
SELFTESTS = ("x87_store", "xmm_call", "abs_read", "f64_call", "xmm_wide",
             "ctail_guard", "st0_call", "st0_call64", "digest_loop",
             "xmm32_call", "glob_store_hi", "glob_store_2", "glob_store_1")


def features_of(contract, setup):
    """v5: what the contract used of the v5 abilities, for the verdict, so a
    proof's reach is visible without reading the contract."""
    callees = contract.get("callees", [])
    snap_words, snap_at, xmm_logged, xmm_transport, xmm_transport64 = {}, {}, {}, {}, {}
    xmm_cmp64 = {}
    xmm_cmp32 = {}
    st0_logged = {}
    st0_transport = {}
    f64_answers = []
    for c in callees:
        cid = str(c["id"])
        sn = c.get("snap") or []
        if sn:
            snap_words[cid] = sum(x.get("n", 0) for x in sn)
            ats = sorted({x.get("at", 0) for x in sn} - {0})
            if ats:
                snap_at[cid] = ats
        regs = set(c.get("logxmm_regs") or [])
        if c.get("logxmm"):
            regs.add(0)
        if c.get("logxmm1"):
            regs.add(1)
        if regs:
            xmm_logged[cid] = sorted(regs)
        cmp64 = sorted(set(c.get("logxmm64_regs") or []))
        if cmp64:
            xmm_cmp64[cid] = cmp64
        cmp32 = sorted(set(c.get("logxmm32_regs") or []))
        if cmp32:
            xmm_cmp32[cid] = cmp32
        if c.get("logst0"):
            st0_logged[cid] = c["logst0"]
        if c.get("st0_from_stack") is not None:
            st0_transport[cid] = c["st0_from_stack"]
        tr = {str(k): v for k, v in (c.get("xmm_from_stack") or {}).items()}
        if c.get("xmm0_from_stack") is not None:
            tr["0"] = c["xmm0_from_stack"]
        if c.get("xmm1_from_stack") is not None:
            tr["1"] = c["xmm1_from_stack"]
        if tr:
            xmm_transport[cid] = dict(sorted(tr.items()))
        tr64 = {str(k): v for k, v in (c.get("xmm_from_stack64") or {}).items()}
        if tr64:
            xmm_transport64[cid] = dict(sorted(tr64.items()))
        if c.get("ret") == "f64xmm0edx":
            f64_answers.append(c["id"])
    fn = contract.get("function", "")
    f = {
        "x87_entry": [e["kind"] for e in contract.get("x87", []) or []],
        "x87_state": x87_state_on(contract),
        "xmm_entry_regs": sorted(int(k) for k in (contract.get("xmm") or {})),
        "xmm_call_logged": xmm_logged,
        "xmm_call_transport": xmm_transport,
        "xmm_call_transport64": xmm_transport64,
        "xmm_call_cmp64": xmm_cmp64,
        "f64_answers": sorted(f64_answers),
        "snap_words": snap_words,
        "snap_offsets": snap_at,
        "abs_shadow": bool(contract.get("abs_shadow")),
        "abs_window": (setup or {}).get("abs_window"),
        "selftest": fn[len("selftest:"):] if fn.startswith("selftest:") else None,
    }
    # v7: new feature keys appear only when the contract uses them, so
    # verdicts of older contracts keep their exact shape.
    if xmm_cmp32:
        f["xmm_call_cmp32"] = xmm_cmp32
    if st0_logged:
        f["st0_call_logged"] = st0_logged
    if st0_transport:
        f["st0_call_transport"] = st0_transport
    if contract.get("ctailpatches"):
        f["ctail_sites"] = len(contract["ctailpatches"])
    if contract.get("log_digest"):
        f["log_digest"] = True
    # v7b: grandfathered legacy specs, so a verdict says what the author
    # wrote that the checker resolves by stock fallback instead of by
    # meaning. Both keys are absent when unused, so older verdicts keep
    # their exact shape.
    compat = []
    for site, spec, in_seq in iter_word_spec_sites(contract):
        collect_word_spec(spec, site, compat, in_seq)
    if compat:
        f["unresolved_word_specs"] = [{"where": w, "keys": k} for w, k in compat]
    if isinstance((contract.get("checks", {}) or {}).get("ret", "eax"), bool):
        f["ret_boolean"] = True
    return f


RET_KINDS = ("eax", "ax", "al", "edx_eax", "st0", "xmm0", "none")
CALLEE_RET_KINDS = ("u32", "u64", "al", "f32xmm0", "f64xmm0", "f32st0",
                    "f64st0", "preserve", "f64xmm0edx")


def iter_word_spec_sites(contract):
    """v7b: every word-spec site in the contract as (site, spec, in_seq),
    in validation order. `site` is the path without the contract-name
    prefix; validation prefixes it for messages, the verdict's
    features.unresolved_word_specs records it bare."""
    for si, sg in enumerate(contract.get("heapsegs", [])):
        for wi, spec in enumerate(sg.get("words", [])):
            yield ("heapsegs[%d].words[%d]" % (si, wi), spec, False)
        for pi, pin in enumerate(sg.get("pin", [])):
            for vi, v in enumerate(pin["vals"]):
                yield ("heapsegs[%d].pin[%d].vals[%d]" % (si, pi, vi), v, False)
        for vals in sg.get("pinned", []):
            for vi, v in enumerate(vals[1]):
                yield ("heapsegs[%d].pinned.vals[%d]" % (si, vi), v, False)
    for pi, p in enumerate(contract.get("pokes", [])):
        for vi, v in enumerate(p["vals"]):
            yield ("pokes[%d].vals[%d]" % (pi, vi), v, False)
    for hi, sc in enumerate(contract.get("heap_scripts", [])):
        for vi, v in enumerate(sc["cycle"]):
            yield ("heap_scripts[%d].cycle[%d]" % (hi, vi), v, False)
    for gi, g in enumerate(contract.get("globals_values", [])):
        for wi, spec in enumerate(g["words"]):
            yield ("globals_values[%d].words[%d]" % (gi, wi), spec, False)
    for c in contract.get("callees", []):
        cid = c.get("id")
        sc = c.get("script")
        if isinstance(sc, list):
            for vi, v in enumerate(sc):
                yield ("callee %s script[%d]" % (cid, vi), v, True)
        ws = c.get("wscript")
        if ws is not None:
            for ti, row in enumerate(ws):
                for vi, v in enumerate(row):
                    yield ("callee %s wscript[%d][%d]" % (cid, ti, vi), v, False)
        sq = c.get("seq")
        if isinstance(sq, list):
            for vi, v in enumerate(sq):
                yield ("callee %s seq[%d]" % (cid, vi), v, True)


def collect_word_spec(spec, where, out, in_seq=False):
    """v7b: the non-strict twin of check_word_spec. A word spec the stock
    driver resolves to random bits is appended to `out` as (where, keys)
    instead of refused; resolve_word then resolves it to those same bits,
    so the verdict is unchanged. Mirrors stock resolution exactly: in a
    script/seq entry {"double":...} is consumed by the caller and
    {"lo":...} recurses into plain word specs, while cycle/rot entries
    always resolve through resolve_word (a double/lo nested there is
    random bits on stock, hence recorded)."""
    if isinstance(spec, int):
        return
    if not isinstance(spec, dict):
        raise ValueError("%s: word spec %r is not an int or a dict" % (where, spec))
    if in_seq and ("double" in spec or "lo" in spec):
        if "double" in spec:
            return  # shape checked by validate_v5, resolved by the caller
        collect_word_spec(spec["lo"], where + ".lo", out, False)
        collect_word_spec(spec.get("hi", 0), where + ".hi", out, False)
        return
    if not any(k in spec for k in WORD_SPEC_KEYS):
        out.append((where, sorted(spec)))
        return
    for k in ("cycle", "rot"):
        if k in spec:
            for i, e in enumerate(spec[k]):
                collect_word_spec(e, "%s.%s[%d]" % (where, k, i), out, False)


def check_word_spec(spec, where, in_seq=False):
    """v7: refuse a word spec with no known key (it used to resolve to
    random bits). Mirrors resolve_word's dispatch, including the
    script/seq-only {"double":...} and {"lo":...,"hi":...} wrappers and
    the {"cycle":...}/{"rot":...} recursion. v7b: used only when the
    contract sets strict:true; otherwise collect_word_spec records."""
    if isinstance(spec, int):
        return
    if not isinstance(spec, dict):
        raise ValueError("%s: word spec %r is not an int or a dict" % (where, spec))
    if "double" in spec or "lo" in spec:
        if not in_seq:
            raise ValueError("%s: %r is a script/seq entry, not a word spec"
                             % (where, spec))
        if "double" in spec:
            return  # shape checked by validate_v5
        check_word_spec(spec["lo"], where + ".lo", in_seq)
        check_word_spec(spec.get("hi", 0), where + ".hi", in_seq)
        return
    if not any(k in spec for k in WORD_SPEC_KEYS):
        raise ValueError("%s: unknown word-spec keys %s (known: %s)"
                         % (where, sorted(spec), ", ".join(sorted(WORD_SPEC_KEYS))))
    for k in ("cycle", "rot"):
        if k in spec:
            for i, e in enumerate(spec[k]):
                check_word_spec(e, "%s.%s[%d]" % (where, k, i), in_seq)


def validate_v7(contract):
    """v7: fail-fast checks of the v7 contract keys, plus two refusals of
    contracts that were silently wrong before (an unknown return kind fell
    back to a 32-bit integer; an unknown word-spec key resolved to random
    bits). v7b: existing contracts that hold those legacy shapes keep
    running exactly as on stock (an unknown word-spec key resolves to the
    same random bits, a boolean checks.ret compares the return register)
    and the verdict records them under features.unresolved_word_specs /
    features.ret_boolean; only a contract with strict:true is refused.
    Every NEW contract should set strict: true. Contracts without the v7
    keys are otherwise untouched."""
    name = contract.get("name")
    where = "contract %s" % name
    strict = bool(contract.get("strict", False))
    ret = (contract.get("checks", {}) or {}).get("ret", "eax")
    if isinstance(ret, bool):
        # On stock a boolean ret reaches the worker, whose non-string
        # fallback compares eax; grandfathered unless strict.
        if strict:
            raise ValueError("%s: checks.ret %r (known: %s)"
                             % (where, ret, ", ".join(RET_KINDS)))
    elif ret not in RET_KINDS:
        raise ValueError("%s: checks.ret %r (known: %s)"
                         % (where, ret, ", ".join(RET_KINDS)))
    ids = set()
    for c in contract.get("callees", []):
        cid = c.get("id")
        ids.add(cid)
        if c.get("ret", "u32") not in CALLEE_RET_KINDS:
            raise ValueError("%s: callee %s ret %r (known: %s)"
                             % (where, cid, c.get("ret"), ", ".join(CALLEE_RET_KINDS)))
        # The v7 ST0 call argument: explicit width, transported only when
        # logged, over declared stack words (the worker enforces this too).
        st0 = c.get("logst0")
        if st0 is not None and st0 not in ("f32", "f64"):
            raise ValueError("%s: callee %s logst0 %r (f32 or f64)"
                             % (where, cid, st0))
        tport = c.get("st0_from_stack")
        if tport is not None:
            if st0 is None:
                raise ValueError("%s: callee %s transports ST0 without logging it "
                                 "(the argument would be compared nowhere)" % (where, cid))
            nargs = c.get("nargs", 0)
            need = tport + 1 if (st0 == "f64" and isinstance(tport, int)) \
                else tport
            if not isinstance(need, int) or not 0 <= need < nargs:
                raise ValueError("%s: callee %s st0_from_stack %r (a declared stack "
                                 "argument%s)" % (where, cid, tport,
                                                  ", pair" if st0 == "f64" else ""))
        # The v7 4-byte narrowing: logged registers only, never beside the
        # 8-byte narrowing on the same register.
        regs32 = c.get("logxmm32_regs")
        if regs32 is not None:
            if not isinstance(regs32, list) or \
                    any(not isinstance(r, int) or not 0 <= r < 8 for r in regs32):
                raise ValueError("%s: callee %s logxmm32_regs %r (registers 0-7)"
                                 % (where, cid, regs32))
            logged = set(c.get("logxmm_regs") or [])
            if c.get("logxmm"):
                logged.add(0)
            if c.get("logxmm1"):
                logged.add(1)
            for r in regs32:
                if r not in logged:
                    raise ValueError("%s: callee %s narrows xmm%d to 4 bytes without "
                                     "logging it" % (where, cid, r))
                if r in (c.get("logxmm64_regs") or []):
                    raise ValueError("%s: callee %s narrows xmm%d to 8 bytes and 4 "
                                     "bytes (pick one)" % (where, cid, r))
    for p in contract.get("ctailpatches", []):
        if p.get("id") not in ids:
            raise ValueError("%s: ctailpatch names undeclared callee %r"
                             % (where, p.get("id")))
        if "site" not in p and "site_selftest" not in p:
            raise ValueError("%s: ctailpatch needs a site or a site_selftest"
                             % where)
    if "log_digest" in contract and not isinstance(contract["log_digest"], bool):
        raise ValueError("%s: log_digest %r (true or false)"
                         % (where, contract["log_digest"]))
    # The digest folds the conv-default registers and all stack words, so
    # a contract that reselects the compared stream cannot use it: one
    # stream in the prefix and another past the cap would be two rules.
    if contract.get("log_digest"):
        ch = contract.get("checks", {}) or {}
        for k in ("call_regs", "call_skip", "call_mask"):
            if k in ch:
                raise ValueError("%s: log_digest cannot combine with checks.%s"
                                 % (where, k))
    # Every word spec in the contract, so a mistyped key fails at load,
    # not on trial 0 -- unless the contract predates strict, in which case
    # it is recorded for the verdict's features instead (v7b).
    compat = []  # recorded here only to fail malformed specs at load;
    # features_of recomputes the same list for the verdict (validation
    # must not mutate the contract: the verdict hashes it).
    for site, spec, in_seq in iter_word_spec_sites(contract):
        if strict:
            check_word_spec(spec, "%s: %s" % (where, site), in_seq)
        else:
            collect_word_spec(spec, "%s: %s" % (where, site), compat, in_seq)


def validate_contract(contract):
    """v4: fail-fast contract checks the worker also enforces per trial."""
    validate_v5(contract)
    validate_v7(contract)
    masks = (contract.get("checks", {}) or {}).get("call_mask") or {}
    for cid, per in masks.items():
        for i, m in per.items():
            v = int(m, 16) if isinstance(m, str) else m
            if (v & 0xFFFFFFFF) == 0:
                raise ValueError(
                    "contract %s: call_mask %s.%s is zero (rejected: "
                    "a zero mask compares nothing)"
                    % (contract.get("name"), cid, i))


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
    fp_exact = sum(1 for r in results if r.get("fp_exact"))
    us = sum(r.get("trial_us", 0) for r in results)
    orig_ok = sum(1 for r in results if r.get("orig", {}).get("status") == "ok")
    cov = coverage_of(contract, results)
    # v3 vacuous rule: a verdict with no failures is still a failure unless
    # a stated minimum share of trials completed on the original, every
    # enabled behavioural check ran somewhere, and every declared callee
    # fired on a completed trial. Contracts with legitimately fault-heavy
    # inputs override the share with "min_orig_ok_share" (documented in the
    # contract); callees that cannot fire are exempted explicitly with
    # "coverage_exempt_callees", never silently.
    min_share = float(contract.get("min_orig_ok_share", 0.10))
    reasons = []
    if cov["orig_ok_share"] < min_share:
        reasons.append("orig_ok_share %.4f < %.2f"
                       % (cov["orig_ok_share"], min_share))
    if cov["callees_unfired"]:
        reasons.append("callees never fired on a completed trial: %s"
                       % ",".join(cov["callees_unfired"]))
    if cov["checks_missing"] and orig_ok > 0:
        reasons.append("checks never ran: %s" % ",".join(cov["checks_missing"]))
    clean = run.get("fails", 1) == 0 and n > 0 and "error" not in run
    vacuous = bool(clean and reasons)
    passed = bool(clean and not vacuous)
    v = {
        "function": contract["function"],
        "passed": passed,
        "inputs_tested": n,
        "comparisons": [{"name": c["name"], "passed": c["passed"]} for c in comps],
        "checker_version": "checker4",
        "call_masks_used": resolved_masks(contract),
        "log_max": int(contract.get("log_max", 256)),
        "worker_retries": run.get("worker_retries", []),
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
        "coverage": cov,
        "min_orig_ok_share": min_share,
        "vacuous": vacuous,
        "vacuous_reasons": reasons,
        "features": features_of(contract, run.get("setup")),
    }
    # v7: when the contract asked for the past-cap digest, the verdict says
    # on how many trials it was actually used (either side past the cap).
    if contract.get("log_digest"):
        used = sum(1 for r in results
                   if (r.get("orig", {}) or {}).get("log_attempted", 0)
                   > (r.get("orig", {}) or {}).get("log_logged", 0)
                   or (r.get("rw", {}) or {}).get("log_attempted", 0)
                   > (r.get("rw", {}) or {}).get("log_logged", 0))
        v["features"]["log_digest"] = {"asked": True, "trials_used": used}
    if contract.get("function", "").startswith("selftest:"):
        # A self-test exercises the checker; it never verifies a function.
        v["selftest"] = True
    if run.get("first_fail"):
        ff = run["first_fail"]
        v["first_mismatch"] = {"trial": ff["trial"],
                               "detail": ff["resp"].get("first_mismatch") or "",
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
         "globals": contract.get("globals", []),
         "log_max": contract.get("log_max", 256),
         # v5: the read-only shadow at the preferred base, and whether the
         # rewrite DLL must export the x87 mirror.
         "abs_shadow": bool(contract.get("abs_shadow")),
         "x87": bool(contract.get("x87")),
         # v7: conditional tail-jump patches and the past-cap call digest.
         "ctailpatches": contract.get("ctailpatches", []),
         "log_digest": bool(contract.get("log_digest", False))}
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
    global CODE_LO, CODE_HI, ABS_LO, ABS_HI
    if "text_lo" in r and "text_hi" in r:  # v3 worker reports the code range
        CODE_LO = int(r["text_lo"], 16)
        CODE_HI = int(r["text_hi"], 16)
    aw = r.get("abs_window") or {}
    if contract.get("abs_shadow"):
        if not aw.get("shadow"):
            raise RuntimeError("abs_shadow requested but the worker reports no shadow: %r" % aw)
        ABS_LO, ABS_HI = int(aw["lo"], 16), int(aw["hi"], 16)
    else:
        ABS_LO = ABS_HI = 0
    w.setup = r
    return r


def selftest():
    """Host-side checks of the driver's pure parts (no worker, no game):
    the f32/f64 to 80-bit conversions, the v5 contract validation over
    every tracked contract plus refused samples, the x87 input generation,
    the unchanged v4 input streams, and the features record. Prints one
    line per check; returns 0 when all pass."""
    fails = []

    def check(name, ok):
        print("%s %s" % ("ok  " if ok else "FAIL", name))
        if not ok:
            fails.append(name)

    check("f32 1.0", f32_to_f80(0x3F800000) == (1 << 63, 0x3FFF))
    check("f32 -0", f32_to_f80(0x80000000) == (0, 0x8000))
    check("f32 min denormal", f32_to_f80(1) == (1 << 63, 16383 - 149))
    check("f32 snan quieted", f32_to_f80(0x7F800001) == ((1 << 63) | (1 << 62) | (1 << 40), 0x7FFF))
    check("f64 pi", f64_to_f80(0x400921FB54442D18) == (0xC90FDAA22168C000, 0x4000))
    check("f64 -inf", f64_to_f80(0xFFF0000000000000) == (1 << 63, 0xFFFF))
    check("f64 max denormal", f64_to_f80(0x000FFFFFFFFFFFFF)
          == (0xFFFFFFFFFFFFF000, 16383 - 1023))
    names = sorted(f[:-5] for f in os.listdir(os.path.join(HERE, "contracts")) if f.endswith(".json"))
    bad = []
    for n in names:
        try:
            validate_contract(json.load(open(os.path.join(HERE, "contracts", n + ".json"))))
        except Exception as e:  # noqa: BLE001 - reported below
            bad.append("%s: %s" % (n, e))
    check("every tracked contract validates (%d)%s" % (len(names), "" if not bad else ": " + "; ".join(bad)),
          not bad)
    base = json.load(open(os.path.join(HERE, "contracts", "k5_x87.json")))

    def refused(mut):
        c = json.loads(json.dumps(base))
        mut(c)
        try:
            validate_contract(c)
        except ValueError:
            return True
        return False
    check("refuse 9 x87 entries", refused(lambda c: c.__setitem__("x87", [{"kind": "f64"}] * 9)))
    check("refuse x87 kind", refused(lambda c: c["x87"][0].__setitem__("kind", "f16")))
    check("refuse x87_state false", refused(lambda c: c["checks"].__setitem__("x87_state", False)))
    check("refuse f80 value", refused(lambda c: c["x87"][1].__setitem__("vals", [[1 << 64, 0]])))
    check("refuse unknown selftest", refused(lambda c: c.__setitem__("function", "selftest:nope")))
    cal = {"id": 1, "conv": "cdecl", "nargs": 2, "ret": "u32", "script": [0]}
    check("refuse 65 snap words", refused(lambda c: c.__setitem__("callees", [dict(cal, snap=[
        {"kind": "arg", "idx": 0, "n": 40}, {"kind": "ecx", "n": 25}])])))
    check("refuse snap at", refused(lambda c: c.__setitem__("callees", [dict(cal, snap=[
        {"kind": "ecx", "n": 1, "at": 0x10004}])])))
    check("refuse snap kind", refused(lambda c: c.__setitem__("callees", [dict(cal, snap=[
        {"kind": "esi", "n": 1}])])))
    check("refuse xmm8", refused(lambda c: c.__setitem__("callees", [dict(cal, logxmm_regs=[8])])))
    check("refuse unlogged transport", refused(lambda c: c.__setitem__("callees", [dict(
        cal, xmm_from_stack={"3": 0})])))
    check("refuse transport past nargs", refused(lambda c: c.__setitem__("callees", [dict(
        cal, logxmm_regs=[3], xmm_from_stack={"3": 2})])))
    # x87 generation: shape, determinism, kinds.
    xs = [resolve_x87(base, random.Random(7), t) for t in range(40)]
    check("x87 words shape", all(len(x) == 3 and all(len(w) == 3 for w in x) for x in xs))
    check("x87 deterministic", xs == [resolve_x87(base, random.Random(7), t) for t in range(40)])
    check("x87 st0 is an f64 edge at trial 0", xs[0][0] == [0, 0, 0])
    # v4 contracts: no x87 means no RNG consumed, so streams are unchanged.
    old = json.load(open(os.path.join(HERE, "contracts", "k2_xmm1.json")))
    r1, r2 = random.Random(5), random.Random(5)
    check("no x87, no RNG", resolve_x87(old, r1, 3) is None and r1.random() == r2.random())
    check("demap unchanged without shadow", ABS_LO == ABS_HI == 0 and demap(0x400000) == 0x400000)
    f = features_of(json.load(open(os.path.join(HERE, "contracts", "k5_xmm.json"))), {})
    check("features xmm", f["xmm_call_logged"] == {"1": [2, 5]}
          and f["xmm_call_transport"] == {"1": {"2": 0, "5": 1}} and f["selftest"] == "xmm_call"
          and f["xmm_entry_regs"] == [2, 5, 6])
    f = features_of(json.load(open(os.path.join(HERE, "contracts", "k5_snapneg.json"))), {})
    check("features snap", f["snap_words"] == {"1": 6} and f["snap_offsets"] == {"1": [-16]}
          and f["selftest"] is None)
    f = features_of(old, {})
    check("features v4 legacy keys", f["xmm_call_logged"] == {"1": [0], "2": [0]}
          and f["x87_entry"] == [] and not f["x87_state"])
    # Doubles extension (host-side only): scripted doubles resolve to the
    # two words, the transport validates like the 4-byte keys, and every
    # new key lands in the features record.
    check("double script bits split lo/hi",
          resolve_double_bits(0x3FF0000000000001, None, 0) == 0x3FF0000000000001)
    r1, r2 = random.Random(9), random.Random(9)
    check("double edges cycle per trial, no RNG",
          resolve_double_bits("edges", r1, 3) == F64_EDGES[3] and r1.random() == r2.random())
    r1, r2 = random.Random(9), random.Random(9)
    check("double xmm entry is an f64 edge at trial 0",
          resolve_xmm({"name": "t", "xmm": {"0": ["double", 5]}}, r1, 0)["0"]
          == [F64_EDGES[0] & 0xFFFFFFFF, (F64_EDGES[0] >> 32) & 0xFFFFFFFF, 5]
          and r1.random() == r2.random())
    bad64 = json.loads(json.dumps(base))
    bad64["name"] = "t64"
    bad64["function"] = "selftest:f64_call"
    bad64["callees"] = [{"id": 1, "conv": "cdecl", "nargs": 4, "ret": "f64xmm0edx",
                         "script": [{"double": "edges"}],
                         "logxmm_regs": [0, 1],
                         "logxmm64_regs": [0],
                         "xmm_from_stack64": {"0": 0, "1": 2}}]
    try:
        validate_contract(bad64)
        ok64 = True
    except Exception:
        ok64 = False
    check("doubles contract validates", ok64)
    f = features_of(bad64, {})
    check("features doubles", f["xmm_call_transport64"] == {"1": {"0": 0, "1": 2}}
          and f["f64_answers"] == [1] and f["selftest"] == "f64_call"
          and f["xmm_call_cmp64"] == {"1": [0]})

    def refused64(mut):
        c = json.loads(json.dumps(bad64))
        mut(c)
        try:
            validate_contract(c)
            return False
        except ValueError:
            return True
    check("transport64 past nargs refused",
          refused64(lambda c: c["callees"][0]["xmm_from_stack64"].update({"1": 3})))
    check("transport64 unlogged refused",
          refused64(lambda c: c["callees"][0].update({"logxmm_regs": [0]})))
    check("narrowed comparison unlogged refused",
          refused64(lambda c: c["callees"][0].update({"logxmm64_regs": [2]})))
    check("narrowed comparison past xmm7 refused",
          refused64(lambda c: c["callees"][0].update({"logxmm64_regs": [8]})))
    check("transport64 clashing with 4-byte refused",
          refused64(lambda c: c["callees"][0].update({"xmm_from_stack": {"0": 1}})))
    check("bad double script refused",
          refused64(lambda c: c["callees"][0].update({"script": [{"double": "nope"}]})))
    check("double entry on odd word refused",
          refused64(lambda c: c.update({"xmm": {"0": ["float", "double"]}})))
    # v7: the two silent-fallback refusals, the v7 key validation, and the
    # v7 features record.
    v7 = json.loads(json.dumps(base))
    v7["name"] = "t7"
    v7["function"] = "selftest:st0_call"
    v7["callees"] = [{"id": 1, "conv": "cdecl", "nargs": 1, "ret": "u32",
                      "script": [0], "logst0": "f32", "st0_from_stack": 0,
                      "logxmm": True, "logxmm32_regs": [0]}]
    v7["ctailpatches"] = [{"site_selftest": "ctail_guard", "at": 6, "id": 1}]
    v7["log_digest"] = True
    try:
        validate_contract(v7)
        ok7 = True
    except Exception:
        ok7 = False
    check("v7 contract validates", ok7)
    f = features_of(v7, {})
    check("features v7", f["st0_call_logged"] == {"1": "f32"}
          and f["st0_call_transport"] == {"1": 0}
          and f["xmm_call_cmp32"] == {"1": [0]}
          and f["ctail_sites"] == 1 and f["log_digest"] is True
          and f["selftest"] == "st0_call")

    def refused7(mut):
        c = json.loads(json.dumps(v7))
        mut(c)
        try:
            validate_contract(c)
            return False
        except ValueError:
            return True
    check("unknown checks.ret refused",
          refused7(lambda c: c["checks"].__setitem__("ret", "eaxx")))
    check("unknown callee ret refused",
          refused7(lambda c: c["callees"][0].__setitem__("ret", "u33")))
    # v7b: the two legacy shapes run by default (recorded in features)
    # and are refused only with strict:true.
    def strict7(mut):
        c = json.loads(json.dumps(v7))
        c["strict"] = True
        mut(c)
        try:
            validate_contract(c)
            return False
        except ValueError:
            return True

    def accepted7(mut):
        c = json.loads(json.dumps(v7))
        mut(c)
        try:
            validate_contract(c)
        except ValueError:
            return None
        return c
    legacy_word = accepted7(lambda c: c["callees"][0].__setitem__(
        "script", [{"bogus": 1}]))
    check("unknown word-spec key accepted by default",
          legacy_word is not None)
    check("unknown word-spec key recorded in features",
          legacy_word is not None and features_of(legacy_word, {}).get(
              "unresolved_word_specs") == [{"where": "callee 1 script[0]",
                                            "keys": ["bogus"]}])
    check("unknown word-spec key refused under strict",
          strict7(lambda c: c["callees"][0].__setitem__(
              "script", [{"bogus": 1}])))
    legacy_ws = accepted7(lambda c: c["callees"][0].__setitem__(
        "wscript", [[{"double": 7}]]))
    check("wscript double accepted by default and recorded",
          legacy_ws is not None and features_of(legacy_ws, {}).get(
              "unresolved_word_specs") == [{"where": "callee 1 wscript[0][0]",
                                            "keys": ["double"]}])
    check("wscript double refused under strict",
          strict7(lambda c: c["callees"][0].__setitem__(
              "wscript", [[{"double": 7}]])))
    legacy_ret = accepted7(lambda c: c["checks"].__setitem__("ret", True))
    check("boolean checks.ret accepted by default and recorded",
          legacy_ret is not None and features_of(legacy_ret, {}).get(
              "ret_boolean") is True)
    check("boolean checks.ret refused under strict",
          strict7(lambda c: c["checks"].__setitem__("ret", True)))
    r_stock, r_mine = random.Random(11), random.Random(11)
    check("unknown word-spec key resolves as stock (same stream)",
          resolve_word({"bogus": 1}, r_mine, 0, 0x10000,
                       {"name": "t", "heapsegs": []}, {}) == rand32(r_stock))
    try:
        resolve_word({"bogus": 1}, random.Random(11), 0, 0x10000,
                     {"name": "t", "strict": True, "heapsegs": []}, {})
        backstop = False
    except ValueError:
        backstop = True
    check("resolve_word backstop refuses under strict", backstop)
    check("bad logst0 refused",
          refused7(lambda c: c["callees"][0].__setitem__("logst0", "f16")))
    check("st0 transport unlogged refused",
          refused7(lambda c: c["callees"][0].update(
              {"logst0": None, "st0_from_stack": 0})))
    check("st0 transport past nargs refused",
          refused7(lambda c: c["callees"][0].update(
              {"st0_from_stack": 1})))
    check("narrow32 unlogged refused",
          refused7(lambda c: c["callees"][0].update(
              {"logxmm": False, "logxmm32_regs": [0]})))
    check("narrow32 clashing with narrow64 refused",
          refused7(lambda c: c["callees"][0].update({"logxmm64_regs": [0]})))
    check("ctailpatch unknown callee refused",
          refused7(lambda c: c.__setitem__(
              "ctailpatches", [{"site": "0x1000", "id": 9}])))
    check("log_digest non-bool refused",
          refused7(lambda c: c.__setitem__("log_digest", 1)))
    check("log_digest with call_skip refused",
          refused7(lambda c: c["checks"].__setitem__(
              "call_skip", {"1": [0]})))
    # v7 features stay out of older verdicts.
    f = features_of(old, {})
    check("features v7 absent when unused",
          "xmm_call_cmp32" not in f and "st0_call_logged" not in f
          and "st0_call_transport" not in f and "ctail_sites" not in f
          and "log_digest" not in f and "unresolved_word_specs" not in f
          and "ret_boolean" not in f)
    print("selftest: %d failed" % len(fails))
    return 1 if fails else 0


SHADOW_RETRIES = 3


def main(argv):
    global EXE, OUT
    args = argv[1:]
    if args == ["--selftest"]:
        return selftest()
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
    for c in contracts:
        validate_contract(c)
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
            # v5: an abs_shadow setup fails when something in the fresh
            # worker already sits in the preferred-base window (address
            # layout varies per process); a few more fresh workers are
            # tried before giving up loudly.
            tries = SHADOW_RETRIES if c.get("abs_shadow") else 0
            while True:
                try:
                    sr = setup_worker(w, c, c_exports)
                    break
                except Exception as e2:
                    if tries <= 0:
                        raise
                    tries -= 1
                    print("SETUP FAILED again (%s), fresh worker" % e2, flush=True)
                    w.stop()
                    w = Worker()
        if sr.get("errors"):
            print("setup errors:", sr["errors"], flush=True)
        print("img=%s heap=%s dll=%s stubs=%s" % (sr.get("img_base"), sr.get("heap"),
              sr.get("dll_base"), sr.get("stub_addrs")), flush=True)
        run = run_contract(w, c, c["export"], trials, c["seed"])
        v = verdict(c, run, c["export"])
        json.dump(v, open(os.path.join(OUT, "verdicts", c["name"] + ".json"), "w"), indent=1)
        tus = (v["worker_us_total"] / max(1, v["trials"]))
        print("correct: pass=%s fails=%d/%d fp_exact=%d orig_ok=%d (share %.3f) vacuous=%s branches=%s worker=%.0fus/trial wall=%.1fs hist=%s" % (
            v["passed"], v["fails"], v["trials"], v["fp_exact"], v["orig_ok_trials"],
            v["coverage"]["orig_ok_share"], v["vacuous"], v["coverage"]["branch_shapes"],
            tus, v["wall_s"], v.get("status_hist")), flush=True)
        if v.get("vacuous"):
            print("VACUOUS: %s" % ("; ".join(v["vacuous_reasons"])), flush=True)
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
                        "vacuous": v.get("vacuous"),
                        "vacuous_reasons": v.get("vacuous_reasons"),
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
