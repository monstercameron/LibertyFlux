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

## Rule 2: the coordinator pushes every commit; nothing else is published without Cam

The repository is public and the project site is served from it. Cam's standing instruction
(2026-10-03) is that every commit is pushed.

- The coordinator pushes to `origin main` straight after committing. Do not let commits pile up
  locally. When several commits are made in a row, push once after the last one, not once per
  commit: GitHub Pages fails a build that is triggered a few seconds after another, and the site
  is then stale until the next successful build.
- Because every commit is published at once, check before committing, not after: no decompiled
  code, no game files, nothing from `notes.md`, no addresses or code in the site pages.
- Never force-push and never rewrite pushed history.
- Lanes never commit and never push.
- Everything else that publishes is still Cam's decision: adding remotes, creating repositories,
  changing repository settings, uploading anything to another service, or posting about the project.

## Rule 3: a machine check decides, not you

- A function is done only when the checker accepts it. Your own judgement that it matches is not a result.
- Never edit the checker, the queue, the symbol tables, shared structure definitions or reference
  data to make a check pass. Report the problem instead.
- No fake passes: no stubs presented as implementations, no hard-coded original addresses in place of
  real references, no inline assembly, no weakening of a comparison.
- Deferring a function with a stated blocker is a normal outcome. Guessing is not.
- A pass counts only on the stock checker, the one built from the tracked source. A lane may build a
  private copy to demonstrate a defect, but a result from it is recorded as deferred with reason
  `checker_gap`, never as verified. Twenty-two lanes ran private copies before this was written
  down, and one of them relaxed a comparison.
- Every function gets its own deliberately wrong version, run through the same contract. It must
  fail. If it passes, the contract cannot see what was changed: widen the contract, and if the
  checker cannot observe it at all, defer the function. Until this was required, 6,515 of 6,776
  accepted rewrites had never had one.
- Say what a proof does not cover. Anything that narrows a function's comparison (a masked or
  skipped argument, a check switched off, a callee left running natively, a branch the inputs never
  take) is listed in that function's result. A narrow proof that says so is acceptable; one that
  looks full is not.
- Floating-point results match bit for bit, including the sign and payload of a not-a-number. A
  difference there means the rewrite's operand order differs from the original's; fix the rewrite
  (pin the order with `core::hint::black_box`), never the comparison.

## Working rules for lanes

- Write only inside your own folder under `.artifacts/scratch/`. Lanes do not edit tracked files and
  do not commit. The coordinator integrates accepted work and makes all commits.
- A script hands you your next function. Do not pick your own.
- Change one thing per attempt and state what difference it targets. Stop at the attempt cap.
- Append per-function progress to your log under `.artifacts/logs/` as you go.
- Label findings as Verified (you read or ran it), Inferred, or Unknown. Do not fill gaps with guesses.

Learned from the first 800 lanes (4 October 2026). Each line is here because its absence cost work.

- Read the clock. Every time you write a time or a duration, take it from the machine. Lanes wrote
  times an hour off and durations three times too long from memory.
- Write `results.json` in your first minutes and keep it current after every function. A lane is
  stopped at its time limit with no warning, and one that had written nothing lost two hours of work.
  Finish well inside your budget.
- Triage the whole batch before writing anything. If every function in it depends on something you
  cannot test (the encrypted first megabyte of code, an ability the checker lacks), write the
  rewrites, defer them with the reason and finish early: one lane spent an hour to verify none.
- Read the executable through its mapped image, never the raw file at a mapped address (the two
  differ by a constant in the code section), and decode one known function end to end before
  trusting your mapping. Three lanes skipped this; two concluded that real functions were not
  functions. A claim that an inventory entry is not a function is unverified until it is checked
  independently.
- A family of near-identical neighbouring functions is one routine instantiated many times. Work it
  out from three members, then generate every rewrite, contract and wrong version with a script that
  asserts each member's shape. Two such lanes verified 145 functions in under an hour each.
- Follow the best lane's output as the standard: a doc comment that is a specification, offsets and
  magic values as named constants, parameters and locals named for what they hold, float order
  pinned from the start. The production brief names the example to read.
- Leave nothing running. Before you finish, stop every process you started (drivers, workers,
  helper scripts), by process id and never by name, because every lane's workers share one name. A
  helper script left running held 8.7 GB for four hours after its lane had ended, and nine others
  each spun a processor core for up to thirteen hours.
- Do not build anything large in one piece. One build of a single crate holding 7,381 rewrites took
  commit headroom from 24 GB to under 1 GB and killed three other lanes. Split it. Never run more
  than three checker workers at once.
- A list meant for a program is a data file. Never rebuild a work list from a report written for
  people: a survey's printout stopped at 40 names per lane and hid 34 contracts from a re-run.

What happens to your Rust. On every five-minute tick the coordinator's scripts commit all Rust that
lanes have written, so nothing waits in a scratch folder:

| Folder | What goes there |
|---|---|
| `rewrites/verified/` | A rewrite as soon as its lane records it as verified on the stock checker, even while the lane is still running |
| `rewrites/unverified/` | A rewrite that exists but has not passed (deferred, failed, not yet run), with the reason; never counted as rewritten or verified |
| `rewrites/in-review/<lane>/` | Other lanes' Rust (checker changes, assembly, lifted code, tools) that is new or differs from the tracked file, waiting for the coordinator's review; trust-critical code is never replaced automatically |

Every file passes the publication scan first. Keep disassembly, byte dumps and machine paths out of
your Rust, comments included, or it is held back.

## Rule 4: the coordinator keeps the progress file true

`docs/data/progress.json` is the single source for the badges in `README.md` and the numbers and
code map on the project site. The coordinator owns it. Lanes never edit it.

- Every number comes from a machine source, never from memory or estimate: function totals from the
  Ghidra inventory, stage counts from the checker's accept records and the queue ledger, structure
  and symbol counts from the tracked files. If a number cannot be derived yet, it stays `null` or
  `0` and the site says "not measured yet".
- Update it with every status update tick (rule 7), so the public site is never more than five
  minutes behind. `python scripts/status.py --write --commit` records the timestamp and lane
  activity, regenerates the site data and badges, and commits them. Also update it at the end of
  every accepted wave, whenever a phase gate is passed, and whenever the inventory changes.
- `measured` is a list of facts established so far, each with `label`, `value`, `unit` and `source`.
  Add a fact when a lane or a tool has measured it; correct it, and say so in the changelog or
  devlog, if a later measurement disagrees. `activity` and `updated_at` are written by the script.
- Commits that change only the progress files have a subject starting `Progress:` and need no
  changelog entry. Every other commit still needs one.
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
  - `map`: the code map, in address order, rebuilt on every tick from the same sets as the stage
    counts. One object per slice with `from` and `to` (the slice's address range as hex strings),
    `count` (game functions in the slice), `named`, `rewritten` and `verified` (how many of them
    have reached each stage) and `stage` (the stage at least half of them have reached, or
    `unmeasured` when the slice holds no game functions). Keep the slice count a multiple of
    120 so the grid fills whole rows at every screen width. `mapRange` is the full range as text.
- After editing, run `python scripts/update_progress.py`, open the site and confirm the numbers,
  meters and map agree with each other, then commit `progress.json` together with the regenerated
  badges. The site pages fetch `progress.json` and `changelog.json` directly.
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

## Rule 6: no piracy

This project requires a copy of the game that its user bought. It does not condone piracy. This rule
is about piracy only. It does not limit how the owner's own purchased copy is analysed.

- Never distribute game files, product keys, cracks or cracked launchers, and never help anyone run
  the game without having bought it. This applies to code, documentation, the site, issues and
  commit messages.
- When documentation tells a reader they need the game, tell them to buy it: on Steam, from an
  authorised key seller, or from a retailer.
- Requests for help with pirated or cracked copies get no answer beyond a pointer to this rule.

Analysing the owner's own copy is in scope, in full. That includes code that is encrypted on disk:
it may be read from the owner's own running, purchased copy and analysed like the rest. Whatever is
obtained that way is game code, so rule 1 applies to it: it stays under `orig/` or `.artifacts/`,
is never committed, and is never published.

## Rule 7: the coordinator posts a status update every 5 minutes

While the project is being worked on, the coordinator posts a project status update in the chat
every 5 minutes, for the rest of the project. Cam should never have to ask what is happening.

Each update is short and covers, in this order:

- Phase and what changed since the last update. If nothing changed, say so in one line.
- Lanes: how many are running, how many have finished, how many failed, and what each running lane
  is working on, taken from its log under `.artifacts/logs/`, not from memory.
- Numbers from machine sources: functions counted, named, rewritten, verified; long-running jobs
  and how far along they are; memory in use.
- Problems: anything stuck, rate-limited, failing or surprising, stated plainly.
- What happens next, and anything that needs Cam.

Run `python scripts/status.py --write --commit` on every tick. It collects the facts, updates
`docs/data/progress.json` and the badges, and commits and pushes them, so the project site shows the
same picture as the chat. Add newly measured facts to the `measured` list in the progress file
before running it. Do not pad an update, do not repeat unchanged
detail, and do not report a lane's result before it has finished. Interesting, confusing or
surprising findings go in the devlog the same day, not only in the chat.

## Rule 8: the coordinator reviews the project critically every hour

Once an hour, for the rest of the project, the coordinator stops reporting and reviews. The point is
to catch a wrong direction or a wasteful process within an hour, using what the last hour taught.

Each review answers, with numbers from machine sources:

- Direction: is the work still pointed at the goal (a 64-bit, portable Rust version of the game), or
  at a proxy such as the function count? What is the largest risk nothing is running against?
- Throughput: verified functions and verified bytes per hour against the previous review, by lane
  kind; where time and memory went; what was deferred and why.
- Trust: anything in the last hour that weakens confidence in "verified", and what was done.
- Process: what was learned, and which brief, script, guard or allocation changes because of it.
- The lanes themselves: read a sample of what the Muse lanes actually produced in the hour (rewrites,
  contracts and results, not only their summaries) and refine the briefs for correctness first and
  speed second. Every change to the production brief gets a new brief version, written into each
  lane's results, so pass rate and time can be compared before and after the change.
- Stop doing: one thing that is no longer worth its cost.
- Workstation health: run the health check and act on it. It looks for processes left behind by
  finished lanes, anything of the project's that is very large or old and still burning processor
  time, processes that will not exit, memory and commit headroom, the page file, disk space, whether
  the supervisor is alive, and the state of the repository. A lane's leftover script once held
  8.7 GB for four hours and nine others each spun a processor core for up to thirteen hours before
  anyone looked. The five-minute tick's watchdog stops the clear cases by itself; the hourly check
  is where the rest gets noticed. Only this project's processes are ever stopped; the owner's own
  programs are reported, never touched, unless he asks.

The changes are made in the same hour, not listed for later. The review is posted in chat, and one row
(time, main finding, decision) is added to that day's "Hourly reviews" entry in the devlog, in a commit
with a changelog entry. `review_metrics.py` in the coordinator's scratch folder produces the numbers
and keeps a history so each review is compared with the one before.

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
  Generated files (the badges) go in the same commit as the data that produced them.
- Aim for commits a person could review in a few minutes. A wave of rewritten functions is split by
  subsystem if it is large.
- Keep refactors and formatting separate from behaviour changes.
- Push after every commit (rule 2). Pushes carry the same history: do not squash a series into one
  commit before pushing, and never force-push or rewrite history that has been pushed.
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
- The entries are rows in a SQLite database, `docs/data/devlog.sqlite`, which the devlog page
  downloads and searches in the browser. Write the entry as one `<article class="entry" id="...">`
  fragment with a date and a title and add it with `python scripts/devlog_db.py put <file>` (lane
  fragments go through `scripts/integrate_devlog.py`, which runs the publication checks first). To
  change an entry, `get` it, edit the fragment and `put` it back. Never edit the database by hand.
  Say what was asked, what was found and what was decided, and link the source.
- Label findings Verified, Inferred or Unknown. Record failures as plainly as successes.
- Read the devlog before starting work in an area it covers.

Neither page may contain decompiled code, disassembly, game function addresses or game data.

## Repository conventions

- All build output, caches, logs and scratch work go under `.artifacts/`. Do not create build
  folders elsewhere. One shared build cache; no per-lane copies of the tree.
- Exception for production lanes: each builds its own small rewrite crate with
  `--target-dir .artifacts/build/lanes/<lane>`. Fifty lanes building into one folder queue on
  cargo's lock and overwrite each other's output when crate names collide. These folders hold only
  a zero-dependency crate's output; delete a lane's folder once its results are integrated.
- Production lanes write `results.json` and `summary.txt`; a devlog entry is optional for them and
  is written only for a surprising finding. Lanes of every other kind still write one.
- The operating scripts are in `scripts/coordinator/` (the tick, the lane launcher and supervisor,
  the brief makers, the importers), with tests the pipeline runs. The production brief is a
  template plus numbered notes with a version (`scripts/coordinator/briefs/`); a lane writes that
  version into each result so a change to the brief can be measured.
- `scripts/dashboard/server.py` serves a local page showing every lane, its effort setting, its
  initial prompt, its log and results, and the laptop's load. Use it to see what lanes are doing;
  it costs no model usage.
- Visually inspect the dashboard after starting it or changing it. Confirm that the active lanes,
  their model and effort, and an opened lane's prompt, log and results are visible. A successful
  server start or API response alone is not proof that the dashboard works.
- The pipeline on GitHub builds and tests the portable crates on Windows, Linux and macOS and the
  32-bit tools on Windows, runs the publication check, and publishes a rolling pre-release when
  they pass. It cannot run the checker: that needs the game's executable, which is never uploaded.
- No git worktrees and no `git stash`.
- LF line endings, except `.bat`, `.cmd` and `.ps1`.
- Do not create new `.md` files without Cam's approval.
- Do not download or install anything without Cam's approval.

## Models

- The coordinator assigns work, reviews evidence and integrates results; lanes never commit.
- Luna (`gpt-6-luna`) at maximum reasoning effort does the work (Cam, 7 October 2026).
  Give every lane explicit instructions, its script-assigned task, its output paths, the checker
  and wrong-version requirements, process limits and a time budget. Do not launch Muse lanes.
  Historical throughput measurements for Muse do not establish Luna's throughput or accuracy;
  measure those from Luna's own accepted results and logs.
- Sonnet is on hold (Cam, 4 October 2026). Two Sonnet lanes were run once as a comparison on large
  functions: about one and a half times the verified code per lane-hour, and more rigour (a wrong
  version per function, limits of each proof stated, a function deferred when wrong versions showed
  the checker could not see half its output). Its habits were written into the Muse brief instead.
  If it is used again, the functions Muse defers and the largest functions are where it helps most.
- Lane settings that were tested and kept or dropped: small-function batches of 40 (kept, about
  half as much again per lane-minute as 20); family generator lanes (kept); six checker workers
  instead of three (no measurable gain, dropped); one large function per lane (dropped).

## Environment

- The host is Windows 11 on ARM64. The original game is 32-bit x86.
- Ghidra is in `tools/ghidra` and needs `JAVA_HOME` set to `tools/jdk`. Both are x64 and run under emulation.
- Python is `.venv` (x64).
- Rust targets installed: `aarch64-pc-windows-msvc`, `i686-pc-windows-msvc`.
