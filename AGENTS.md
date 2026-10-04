# Agent rules for LibertyFlux

Read this file before doing anything in this repository. `plan.md` has the plan and current status.

## Rule 1: no decompiled code in the repository

You MUST NOT copy decompiled code into any tracked file, and you MUST NOT commit it.

Decompiled code means anything derived directly from the original game binaries:

- Decompiler output from Ghidra, IDA, Hex-Rays or any other tool, edited or not.
- Disassembly listings.
- C or C++ reconstructions of original functions.
- Any of the above pasted into comments, documentation, test fixtures or commit messages.
- Code copied from other reverse-engineering projects. IV-SDK, IV:MP, LibertyRecomp, scrDbg, FiveM
  and similar sources may be read for names and layouts. Their code is not copied in.

The only things derived from the game that may be tracked:

| Allowed | Meaning |
|---|---|
| Structures | Type and layout definitions: fields, sizes, offsets, enums, virtual table slot order |
| Symbols | Names, addresses, signatures, hashes, class hierarchy, string references |
| Rust rewrites | New Rust implementations of the behaviour, written as Rust, not pseudocode transliterated line by line |

Tooling scripts, tests that do not embed decompiled code, and documentation are also tracked.

Decompiler output, disassembly, intermediate C or C++ and lane attempts live under `.artifacts/` and
stay there. Original game files live under `orig/`. Both are ignored by git. Never move anything out
of them into a tracked path, and never use `git add -f`.

No game files or assets are committed, in any form.

If you are unsure whether something counts as decompiled code, it does. Leave it in `.artifacts/` and
say so in your report.

## Rule 2: publishing is Cam's decision

Agents never add a git remote, push, create a hosted repository, upload repository contents or
game-derived material to any service, or post about the project. Cam does those, or asks for one
specific action explicitly.

## Rule 3: a machine check decides, not you

- A function is done only when the checker accepts it. Your own judgement that it matches is not a result.
- Never edit the checker, the queue, the symbol tables, shared structure definitions or reference
  data to make a check pass. Report the problem instead.
- No fake passes: no stubs presented as implementations, no hard-coded original addresses in place of
  real references, no inline assembly, no weakening of a comparison.
- Deferring a function with a stated blocker is a normal outcome. Guessing is not.

## Working rules for lanes

- Write only inside your own folder under `.artifacts/scratch/`. Lanes do not edit tracked files and
  do not commit. The coordinator integrates accepted work and makes all commits.
- A script hands you your next function. Do not pick your own.
- Change one thing per attempt and state what difference it targets. Stop at the attempt cap.
- Append per-function progress to your log under `.artifacts/logs/` as you go.
- Label findings as Verified (you read or ran it), Inferred, or Unknown. Do not fill gaps with guesses.

## Rule 4: the coordinator keeps the progress file true

`docs/data/progress.json` is the single source for the badges in `README.md` and the numbers and
code map on the project site. The coordinator owns it. Lanes never edit it.

- Every number comes from a machine source, never from memory or estimate: function totals from the
  Ghidra inventory, stage counts from the checker's accept records and the queue ledger, structure
  and symbol counts from the tracked files. If a number cannot be derived yet, it stays `null` or
  `0` and the site says "not measured yet".
- Update it at the end of every accepted wave, whenever a phase gate is passed, and whenever the
  inventory changes (for example after library code is re-tagged). Do not let it lag a commit
  behind the work.
- Fields:
  - `updated`: the date of this update, `YYYY-MM-DD`.
  - `phase`: `index` and `name` of the roadmap phase in progress. Change it only when the previous
    phase's exit gate in `plan.md` has been met.
  - `functions`: `total` (all functions found), `library` (excluded runtime and third-party code),
    `game` (`total` minus `library`; the denominator for every percentage).
  - `stages`: cumulative counts of game functions that are `identified`, `named`, `rewritten`,
    `verified`. Each stage includes the ones after it, so `verified` ≤ `rewritten` ≤ `named` ≤
    `identified` ≤ `functions.game`. A function counts as `verified` only if the checker accepted
    it; a rewrite that has not passed is `rewritten` at most.
  - `structures`, `symbols`: counts of documented type layouts and named symbols in tracked files.
  - `map`: the code map, in address order. One object per slice with `stage` (the least-advanced
    stage among the slice's functions, or `unmeasured`), `from` and `to` (the slice's address
    range as hex strings) and `count` (functions in the slice). Keep the slice count a multiple of
    120 so the grid fills whole rows at every screen width. `mapRange` is the full range as text.
- After editing, run `python scripts/update_progress.py`, open the site and confirm the numbers,
  meters and map agree with each other, then commit `progress.json` together with the regenerated
  `progress.js` and badges, with a changelog entry.
- If a count ever goes down (a false accept is found, functions are re-split), say why in the
  changelog entry. Never quietly lower or raise a number.
- Record the commands or queries that produced each number in the devlog the first time, so the
  next coordinator derives them the same way.

## Rule 5: private notes go in `notes.md`, and nowhere else

`notes.md` at the repository root is ignored by git. It is the place for anything that should not
be public. Read it at the start of a session; it has the location of the game install and the
local tool paths.

Put these in `notes.md`, never in a tracked file, the site, the changelog, the devlog or a commit
message:

- Local file paths outside the repository, user names and machine names.
- Where the owner's copy of the game is installed, and anything about that copy or the accounts it
  is tied to.
- Account, quota, rate-limit and billing details for any service or model.
- Working notes, half-formed ideas and decisions that are not ready or not meant for publication.
- Anything a lane found that you are unsure is safe to publish. Put it here and ask.

Tracked files refer to locations generically (`orig/`, `tools/`, "the game folder"). If a tracked
file needs a path, use one relative to the repository.

Never commit `notes.md`, never `git add -f` it, and never copy its contents into another file. If
you move something from it into a tracked file, rewrite it without the private details first.
Secrets such as API keys and passwords do not go in `notes.md` either; they do not go in any file.

## Rule 6: no piracy, and no help with it

This project requires a copy of the game that its user bought. It does not condone piracy.

- Never add, link to or describe how to obtain game files, product keys, cracks, cracked launchers
  or ways around the game's copy protection. This applies to code, documentation, the site, issues
  and commit messages.
- Do not design anything that depends on a pirated, cracked or downgraded-by-crack copy. Work
  against the build the owner bought.
- When documentation tells a reader they need the game, tell them to buy it: on Steam, from an
  authorised key seller, or from a retailer.
- Requests for help with pirated or cracked copies get no answer beyond a pointer to this rule.

## Commit history: one feature at a time

Keep the history readable as a sequence of single changes. Someone reading the log should be able to
say what each commit did in one sentence, and revert any one of them without losing unrelated work.

- One logical change per commit: one feature, one fix, one tooling change, one plan revision, or one
  accepted wave of functions in one subsystem. If the subject line needs "and", it is probably two
  commits.
- Do not clobber unrelated changes together. A site redesign, a new script and a plan update that
  happen in the same session are three commits, not one.
- Stage by file, or by hunk when one file holds two changes. Never `git add -A` or `git add .`
  without reading `git status` first.
- A commit should leave the repository working: the site renders, scripts run, the Rust code builds.
  Generated files (`docs/data/*.js`, badges) go in the same commit as the data that produced them.
- Aim for commits a person could review in a few minutes. A wave of rewritten functions is split by
  subsystem if it is large.
- Keep refactors and formatting separate from behaviour changes.
- Pushes carry the same history. When Cam asks for a push, push the commits as they are, in order.
  Do not squash a series into one commit before pushing, and never force-push or rewrite history
  that has been pushed.
- If work has already piled up uncommitted, split it into its logical commits before committing,
  and say in the report how it was split.

## Changelog and devlog

Both are pages on the project site and both are part of the work, not an afterthought.

Changelog (`docs/changelog.html`, data in `docs/data/changelog.json`):

- Every commit is one change (see above) and has one changelog entry. Do not batch a day's work
  into one commit.
- Every commit adds an entry at the top of `docs/data/changelog.json`, in the same commit: `date`,
  `title`, `commit` set to `null`, an optional `summary`, and `changes` as a list of plain sentences
  saying what changed and why.
- The commit's subject line must equal the entry's `title` exactly. That is how the hash is matched.
- Run `python scripts/update_progress.py` before committing. It regenerates the site data and fills
  in the hashes of earlier entries.
- Lanes do not commit. A lane reports what it changed; the coordinator writes the entry and commits.

Devlog (`docs/devlog.html`):

- When you learn something another agent would otherwise have to rediscover, add an entry: a tool
  trap, a method that worked or failed, a research finding, an assumption that turned out wrong.
- Every finding about the game itself gets an entry the day it is made, written like the entry
  "What the executable shows": what was asked, a table of fact, value and what it means, what was
  decided, and how to reproduce it. That covers header facts, function and class counts, library
  identification, the start-up stub, engine subsystems, file formats and anything measured while
  running the game. If a finding corrects an earlier entry, say so in both.
- Add it as a new `<article class="entry">` at the top of the entries, with a date, and link it
  from the contents list. Say what was asked, what was found and what was decided, and link the source.
- Label findings Verified, Inferred or Unknown. Record failures as plainly as successes.
- Read the devlog before starting work in an area it covers.

Neither page may contain decompiled code, disassembly, game function addresses or game data.

## Repository conventions

- All build output, caches, logs and scratch work go under `.artifacts/`. Do not create build
  folders elsewhere. One shared build cache; no per-lane copies of the tree.
- No git worktrees and no `git stash`.
- LF line endings, except `.bat`, `.cmd` and `.ps1`.
- Do not create new `.md` files without Cam's approval.
- Do not download or install anything without Cam's approval.

## Models

- Opus 5.5 coordinates and reviews only.
- Muse Spark 1.3 at max reasoning effort does the work.
- Sonnet takes functions Muse deferred and task classes where Muse's measured results fall short.

## Environment

- The host is Windows 11 on ARM64. The original game is 32-bit x86.
- Ghidra is in `tools/ghidra` and needs `JAVA_HOME` set to `tools/jdk`. Both are x64 and run under emulation.
- Python is `.venv` (x64).
- Rust targets installed: `aarch64-pc-windows-msvc`, `i686-pc-windows-msvc`.
