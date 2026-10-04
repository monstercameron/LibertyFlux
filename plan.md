# LibertyFlux plan

Status: phase 0 can start; the game is installed. Last updated 2026-10-03.

## Goal

Grand Theft Auto IV's engine rewritten in Rust, function by function, so the game runs as a native
64-bit program at hundreds of frames per second, with room for new gameplay and graphics.

The Rust port exists to make the game 64-bit and portable (Cam, 2026-10-03).

| Target | Graphics | Upscaling and frame generation |
|---|---|---|
| Windows x64 | Vulkan, D3D12 optional | DLSS and FSR for upscaling and frame generation; XeSS for upscaling |
| Windows ARM64 | Vulkan | FSR only, and it needs a source rebuild (inferred). DLSS is reported unsupported; XeSS documents x64 only |
| macOS (Apple silicon) | MoltenVK first, native Metal for MetalFX and ray tracing | MetalFX. FSR on macOS is unconfirmed |

Also wanted: no redistributable installs (.NET, Visual C++ runtime, DirectX end-user runtime), and
path tracing as a stretch goal.

The upscaler research is in `documentation/upscaling-and-frame-generation.md`, with a source link
and a verified or inferred label on every claim. What it changes here:

- DLSS 5 exists (NVIDIA developer blog, 22 September 2026): neural rendering that needs an RTX 50
  series GPU. The blog does not say an SDK is available to developers yet, so plan on DLSS 4.5
  through the Streamline SDK and treat DLSS 5 as unavailable until an SDK ships.
- XeSS frame generation is DirectX 12 only. With a Vulkan renderer we get XeSS upscaling but not
  its frame generation, unless the optional D3D12 backend is built.
- Every frame generation framework takes over presentation with its own swap chain, so the
  renderer must not assume it owns the swap chain.
- MoltenVK has no ray tracing in any release (an open pull request only), so path tracing on macOS
  needs the native Metal backend. MetalFX is Metal only and reaching it through MoltenVK is fragile.
- Rust bindings exist for MetalFX (maintained) and are new or placeholder for DLSS and FSR; XeSS
  has none. Expect to write or generate bindings against the C headers.
- Not yet established: the exact motion vector, depth and jitter conventions for each framework.
  The research lane could not read the long guides in full. A follow-up lane with a working shell
  should read them from cloned repositories before the renderer is designed.

## Decisions

| Decision | Choice | Reason |
|---|---|---|
| What is committed | Structures, symbols, Rust rewrites, tooling, docs. Never decompiled code | Cam's rule, in AGENTS.md |
| Intermediate C++ stage | None. Lanes read decompiler output from `.artifacts/` and write Rust directly | Decompiled C++ cannot be committed, and an uncommitted months-long intermediate is a loss risk. Removes a phase |
| How a function is checked | Emulator comparison: the original function and the 32-bit Rust build of the rewrite run in a CPU emulator from the same state; results, memory writes and outgoing calls must agree | Byte-matching is impossible for Rust. Both sides run in the emulator, so the check does not depend on the game running on this ARM64 machine |
| Second check | Accepted functions are swapped into the running game through an injected 32-bit DLL with a per-function switch | Catches what unit comparison misses; a regression is found by switching replacements off in halves |
| Target binary | The Steam Complete Edition executable (1.2.0.x) | Cam owns it and it runs without a cracked launcher. FusionFix and scrDbg already support it |
| Older builds | 1.0.8.0 is used only for static analysis, if an official patch executable is available, to carry names across with Ghidra Version Tracking | IV-SDK and most public class knowledge target 1.0.7.0 and 1.0.8.0. Downgrading the installed game needs a cracked launcher, which we are not doing |
| Xbox 360 recompilation | Not our base. Used as a reference for class names, virtual table order and engine documentation | Its output keeps the PowerPC machine model permanently and cannot become clean Rust |
| Third-party libraries | Identified by signature and excluded from the queue until phase 5, then replaced | See "Dependencies" |
| Parallelism | Lanes write only to their own scratch folder; the coordinator integrates | No git worktrees, no stash |
| Publishing | Cam's decision. README and a GitHub Pages site exist locally; nothing is pushed until Cam asks | The Modern Warfare 2 project lost its hosting and its agents' work after takedowns in September 2026 |

## Model routing

Set by Cam on 2026-10-03.

| Model | How it runs | Work |
|---|---|---|
| Opus 5.5 | This session | Coordination only: plan, harness and checker design, review of anything that changes the checker or shared structures, audit samples, wave sign-off |
| Muse Spark 1.3, max effort | `muse exec --model muse-spark-1.3-contributor --reasoning-effort max`, one background task per lane | Everything else by default |
| Sonnet | Agent tool | Second pass on functions Muse deferred, task classes where Muse's numbers fall short, and design critique |

"Struggling" is measured per task class: accept rate, attempts per accept, false accepts found by audit.

Muse track record so far: seven research lanes. Spot-checks of lane A against two cited source
files matched exactly. Lane H (dependencies) was mostly inferred from forum posts and said so.
Muse lanes could not run shell commands in their sandbox; that must be fixed before phase 1
(`muse sandbox`, workspace trust).

## This machine

- ARM64: Snapdragon X2 Elite Extreme, Adreno X2-90, fanless. The original game is 32-bit x86 and
  runs under emulation here.
- Visual Studio Build Tools 2026, MSVC 14.51 with x86, x64 and ARM64 targets. Windows SDK 10.0.26100
  with x86 libraries. CMake, Ninja, Python 3.11 (x64).
- GTA IV is installed through Steam as of 2026-10-03: Complete Edition, version 1.2.0.59. Its
  location and other machine-specific details are in the untracked `notes.md`.

Installed for this project on 2026-10-03:

| Tool | Where | Checked by |
|---|---|---|
| Rust target `i686-pc-windows-msvc` | rustup | Built a 32-bit DLL exporting a `thiscall` function |
| Temurin JDK 21.0.12 x64 | `tools\jdk` | SHA-256 matched Adoptium; `java -version` runs |
| Ghidra 12.1.4 | `tools\ghidra` | SHA-256 listed in the official release notes; headless import and full analysis of a 32-bit program succeeded |
| `pefile`, `capstone`, `pyghidra` | `.venv` | pip install succeeded |

Ghidra needs `JAVA_HOME` set to `tools\jdk`. The JDK is x64 on purpose: Ghidra ships Windows native
components for x64 only.

Not installed yet: a CPU emulator library for the checker (Unicorn), a hooking library, DXVK (only
if the game needs it to run here), `objdiff` (only if a byte-level comparison is ever wanted).

## Repository layout

Local git repository on `main`. Nothing committed yet.

| Path | Tracked | Holds |
|---|---|---|
| `README.md`, `AGENTS.md`, `plan.md` | Yes | Project front page, rules for agents, this plan |
| `docs/` | Yes | GitHub Pages site, badges, `data/progress.json` |
| `scripts/` | Yes | Tooling. `update_progress.py` regenerates badges and site data |
| `.artifacts/build` | No | Compiler and linker output. Cargo is pointed here by `.cargo/config.toml` |
| `.artifacts/cache` | No | Ghidra project databases, decompiler output, similarity caches |
| `.artifacts/logs` | No | Lane progress logs |
| `.artifacts/scratch` | No | Lane attempts |
| `tools/`, `.venv/` | No | JDK, Ghidra, Python environment |
| `orig/` | No | The original executable |
| `notes.md` | No | Private notes: local paths, machine and account details, anything not for publication |
| `documentation/` | Yes | Reference material with source links, starting with the upscaler reference |

## Phases

| Phase | Work | Exit gate |
|---|---|---|
| 0. Measure | See task list below | Function count, library and game split, compiler version, class list, and a yes or no on whether the game runs here |
| 1. Harness | Work queue, context packets, emulator comparison, injected DLL with per-function switch, lane permissions, progress log | 50 functions accepted end to end by one lane |
| 2. Pilot | 3 Muse lanes plus a reviewer for a few days; Sonnet on the deferred set | Measured functions per day and false-accept rate; routing confirmed or changed |
| 3. Rewrite | 10 to 20 lanes; full regression and a checkpoint commit between waves | Every game function has an accepted Rust rewrite; the game plays with every switch on |
| 4. Standalone | Build the Rust code as its own 32-bit program. This is a test configuration | It runs without the original executable |
| 5. 64-bit | Converting resource loader, script VM pointer handles, platform layer replacing Win32, replacements for every third-party library | Windows x64 and Windows ARM64 builds play |
| 6. Renderer | Vulkan renderer with motion vectors, depth and jitter outputs; fixed-timestep simulation; macOS | Frame-rate target measured on all three platforms |
| 7. Upscalers and new work | DLSS, FSR, XeSS, MetalFX behind one interface; frame generation; new gameplay and graphics; path tracing | Each upscaler verified on hardware that supports it |

Phases 0 to 4 are faithful reconstruction. No 64-bit or renderer work before the phase 4 gate; the
Modern Warfare 2 project found that mixing those goals stalled the lanes.

Rust written in phase 3 must be pointer-width independent from the first line, even though it is
checked in a 32-bit build. The 32-bit build is the only configuration where the original and the
rewrite share memory layouts and can be compared.

Path tracing needs ray-tracing acceleration structures over the world geometry, materials converted
from the game's Direct3D 9 shaders into a form a path tracer can shade, and a denoiser (DLSS Ray
Reconstruction, FSR Ray Regeneration, or MetalFX's denoising scaler; XeSS has none). MoltenVK has
no ray tracing in any release, so path tracing on macOS needs the native Metal backend.

## Phase 0 findings so far

First look at the executable on 2026-10-03, from its headers only (a copy sits in `orig/`). Verified
by reading the file with `pefile`, except where marked.

| Fact | Value | What it means |
|---|---|---|
| Version and size | 1.2.0.59, 17,425,752 bytes | The current Steam Complete Edition build |
| SHA-256 | `08759a5516f9837920ea504436236bbab89d0826a8e4d04ff106345177b5345d` | Identifies the exact build all addresses refer to |
| Built | Link timestamp 1674831669 (27 January 2023), linker version 11.0 | Rebuilt with Visual Studio 2012, not the 2005-era compiler the original 2008 release used. Ghidra's stock library signatures cover that version |
| Link-time code generation | The compiler record lists 796 objects under the product ID normally used for C++ compiled with link-time code generation, and 653 without (inferred from the standard product ID table) | The risk in the plan is real: a large part of the code was optimised across function boundaries |
| Code size | `.text` is 10.95 MB with entropy 6.71 | The main code is not packed or encrypted on disk, so static analysis can read it |
| Entry point | In a 657-byte writable, executable section named `.rkstr`, next to a 1 MB section `.tbm` with entropy 8.00 | A Rockstar start-up stub runs before the game, with an encrypted or compressed blob beside it. What it does is unknown; it matters for running the game with replacements injected |
| Type information | 3,618 type descriptor strings | Class names are present, as hoped. The Xbox 360 build has 3,332 |
| Relocations | Present, 1.27 MB | Every absolute address reference is listed, which helps reference resolution |
| Debug record | Names a PDB file, `GTA4_Win32_Final.pdb` | Confirms no symbols ship; only the file name survives |
| C runtime | No runtime DLL imported | Statically linked. No Visual C++ redistributable is needed by the engine |

Imports settle most of the dependency questions:

| Imported | Meaning |
|---|---|
| `d3d9.dll` (1 function), no `d3dx9` | Direct3D 9 only. The helper library is statically linked or unused, so no DirectX end-user runtime |
| `DSOUND.dll` (4) | Audio output is DirectSound, not XAudio2 or OpenAL |
| `DINPUT8.dll` (1), no XInput | Input goes through DirectInput. Gamepad support through XInput, if any, is loaded at run time |
| `binkw32.dll` (16) | Bink video, version 1.9r |
| `WMVCore.DLL` (2) | Windows Media, for user music or replay export |
| `WS2_32.dll` (27) | The engine uses sockets directly |
| `WINTRUST.dll`, `CRYPT32.dll` | Signature and certificate checks, probably for the start-up stub |
| No `xlive.dll`, no .NET, no Social Club import | Confirms those are gone from this build or loaded only by the launcher |

Shipped beside the executable: a launcher (`PlayGTAIV.exe`), `gtaEncoder.exe`, `steam_api.dll` and
`MTLX.dll`. The last two are not in the import table, so they are loaded at run time.

Still to do in phase 0: everything from task 3 onward.

## Phase 0 tasks

1. Install GTA IV from Steam. (Cam)
2. Record the executable: version, size, hash. Check for a DRM or launcher wrapper around it.
3. Run the game on this machine with its own Direct3D 9. Note whether it reaches gameplay and at
   what frame rate. DXVK on Snapdragon under Windows has an open failure report, so do not assume it.
4. Read the compiler and linker version from the executable's Rich header. Look for link-time
   code generation markers.
5. Dump the import tables, delay-load imports and runtime-loaded libraries of the executable and
   every DLL and launcher in the game folder. Classify each as engine, launcher only, DRM or online
   service.
6. Run Ghidra headless analysis. Export the function inventory, cross-references and call graph
   to JSON under `.artifacts/cache`.
7. Tag library code with Ghidra Function ID. This build was linked with Visual Studio 2012, which
   Ghidra's stock databases cover, so try those first. Older library code is also linked in (the
   compiler record shows objects from Visual C++ 2003, 2005, 2008 and 2010 toolchains); if the stock
   databases miss it, build one from the matching static libraries. Ghidra cannot import `.lib`
   files directly; extract the object files first.
8. Recover class names and virtual tables from runtime type information with Ghidra's built-in
   class recovery scripts. Export the hierarchy.
9. Collect names:
   - the script engine layout and ten byte patterns from scrDbg, which targets this build;
   - the three public databases of script-callable functions, cross-checked against each other,
     since some entries are debug functions stripped from release builds;
   - strings, assertion text and `Class::Method` debug strings;
   - IV-SDK and IV:MP class headers, as names and layouts only;
   - Xbox 360 class names carried across by matching virtual table slot order.
10. Find functions reachable only through pointer tables, virtual tables and static constructors.
11. Write the findings into this file and into `docs/data/progress.json`, and revise the estimate.

Tasks 5 to 10 go to Muse lanes once their sandbox can run commands. Tasks 2 to 4 and 11 stay with
the coordinator.

## Harness design

Taken from the projects reviewed. Numbers are Thief 3's unless stated.

- Queue: a script orders functions easiest first by instruction count, branches, calls, switches
  and exception frames. Lanes do not choose.
- Claims: one file per claimed function, created atomically, expiring after 2 hours unless renewed
  by an attempt.
- Context packet per function: the target's disassembly and decompiler output, resolved references
  with names, the structures it touches, the most similar accepted functions with their Rust
  source, names that accepted callers already use for it, and the best earlier attempt on a second
  pass.
- Similarity: Ghidra BSim. The tool the Snowboard Kids project used has no x86 support.
- Families: functions with identical bytes after masking references are grouped. One accepted
  member is stamped onto the rest and passed through the same gate with no model call.
- Attempt budget: cap of 12 per claim, stop after 4 attempts with no improvement. Thief 3's logs
  show 77% of the functions tried matched on the first attempt, then 65%, 37%, 39% and 23% of
  those reaching a second to fifth attempt.
- Deferring is a normal outcome, recorded with a blocker tag. Deferred functions go to Sonnet.
- Lane permissions enforced by tooling: lanes write only under `.artifacts/scratch`, may run only
  the queue, context, try and accept commands, and cannot touch the checker, the symbol tables,
  shared structures or git.
- Lint on every candidate: no inline assembly, no hard-coded original addresses, no stubs.
- Structure definitions carry compile-time size and offset assertions, as gta-reversed does.
- Every batch ends with a sweep that releases stale claims and appends cost, attempts and deferral
  reasons to a ledger.
- Evidence labels on every written finding: Verified, Inferred or Unknown.
- All iteration is local. decomp.me blocks scripted clients and forbids agent-driven requests.

Known weakness of the emulator comparison: floating-point results can differ in the last bits
between the original's code and Rust's, so those comparisons need a tolerance, and a tolerance can
hide a real difference. The in-game switch is the backstop.

## Dependencies

From lane H, mostly inferred; the import dump in phase 0 settles it.

| Dependency | Used by | Plan |
|---|---|---|
| .NET 3.5, Games for Windows LIVE, Social Club, Rockstar launcher, SecuROM, Flash | Launcher, DRM and dead online services | Drop |
| Direct3D 9 and its helper library | Engine | New Vulkan renderer; own math |
| Gamepad and window input | Engine | SDL3 |
| Audio output and 3D audio (API unknown) | Engine | Decide after the import dump |
| Bink video (32-bit DLL) | Engine | FFmpeg's Bink decoder |
| Windows Media (startup link, user music, replay export to WMV) | Engine | FFmpeg |
| Euphoria (statically linked, no source) | Engine | Rewritten like game code; it cannot be carried over as a 32-bit binary |
| Physics | Engine | Rewritten like game code. LibertyRecomp's notes show it is built on Bullet |

## What the research changed

- Size: three pipelines count 31,782, 35,888 and 36,409 functions in the Xbox 360 build. Expect
  the PC build to be the same order, about twice Modern Warfare 2's 16,324.
- No symbols exist anywhere for the PC executable. The Xbox 360 build has real class names in its
  type information but no real function names.
- Names and addresses from older builds do not transfer by byte pattern: one audit found nine of
  nine older patches matched nothing on the current Steam build.
- Game logic is tied to frame time in many places: vehicle handling, Euphoria, pathfinding,
  helicopters, cameras, water, and at least one script function. FusionFix's fix list is the
  starting list of regression tests for the fixed-timestep work.
- Resource files are memory images whose pointers are 28-bit offsets plus a 4-bit block type. An
  unlicensed Rust crate already reads and writes the archive formats.
- Nothing public documents the render thread split or how Euphoria is integrated. Both need
  first-hand analysis.
- Licences: IV-SDK, IV:MP and LibertyRecomp are GPL; FiveM's resource loader is reference-only.
  Read for names and layouts; copy no code.

## Pitfalls from the recompilation projects' history

From lane F, which read all 240 commits of LibertyRecomp's GTA IV history and the second project's
notes. Their bugs are mostly specific to translating PowerPC code; the patterns are not.

| What happened to them | What we do about it |
|---|---|
| Seven subsystem stubs and a loading-gate bypass, added to reach boot, were never verifiably replaced | A stub is never an accepted function. Anything stubbed to make the standalone build boot is a queue row with a blocker tag, counted as not rewritten |
| Rendering was the long pole: first-draw work ran January to April and ended in a revert; the game only became visible after the renderer was rewritten | Treat phase 6 as the largest single phase. Start its design notes during phase 3, without writing renderer code before the phase 4 gate |
| A crash was investigated for weeks as a use-after-free by 14 agents; the "poison" bytes were a legitimate colour and the real bug was an unhooked function | No fix without proof: a before and after log comparison or a caller trace that shows the cause. Agent conclusions need a runnable check |
| Their own hooks broke what they instrumented: a zeroed frame counter, a skipped increment, hooks on the wrong function. Audits later removed about 150 dead or harmful hooks | Every replacement swapped into the running game must preserve the original's side effects, which is what the emulator comparison checks. Diagnostic hooks stay out of tracked code |
| Wrong function addresses and boundaries failed silently and cascaded (two wrong library addresses corrupted about 300 call sites) | Function boundaries and library tags are verified in phase 0 and treated as shared truth that lanes cannot edit |
| The player fell through roads that rendered correctly, in both projects; one is still stuck after two months | Rendering proves nothing about collision. Phase 4's gate includes standing and driving on streamed-in world geometry |
| Async file reads were served synchronously, hiding completion-path bugs the streaming system depends on | The platform layer in phase 5 implements real asynchronous reads |
| About 400 stray debug prints caused a major slowdown; gigabyte logs | Diagnostics live under `.artifacts/` and are stripped before commit |
| 54 research documents in one cycle with no behavioural change shipped | Research ends in a decision or a check recorded in the devlog, not a pile of documents |
| History was rewritten and branches deleted, which stranded forks and destroyed forensics; a 1,056-file accidental deletion needed a restore commit | Never rewrite history. Small, described commits, each with a changelog entry |
| Research documents were deleted in cleanup commits | The devlog is the durable home for findings |

Engine behaviour they documented on the Xbox 360 build, to confirm on PC rather than assume: a boot
sequence that initialises 63 subsystems before a loading gate and the main loop; worker threads
split between rendering, streaming and resource loading; a streaming system with a registry of
24-byte records, 8 load slots and per-type fix-up callbacks; a present gate that stalls if frames
are submitted faster than they are presented; and timing constants where one wrong value freezes
every time-gated task. They did not document which script threads must run for the world to appear.

## Risks

- The emulator comparison is our own design, not a method another project has proven at this scale.
  Phase 1's 50-function gate is where it is tested.
- The game may not run under emulation on this machine, which would remove the in-game check here.
- Functions over about 1,000 instructions and vector-math functions stalled every project reviewed.
- If the executable was built with link-time code generation, function boundaries and calling
  conventions are less regular, which makes both analysis and comparison harder.
- Estimate: Modern Warfare 2 took about ten weeks to playable for 16,324 functions with 4 to 17
  lanes. This is about twice the size, in a different language, with no symbols. Phase 2 measures it.

## Open questions

- Is an x86 Windows PC available if the game does not run here?
- Does Cam want the repository published, and if so where and when?
- Which script threads must run for the world to appear? Neither recompilation project documented it.
