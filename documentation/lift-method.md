# The lift: from verified 32-bit rewrites to portable Rust

This document is the readable overview of how verified rewrites become code for the portable 64-bit game. The specification is the crate documentation of [`lf-lift`](../crates/lf-lift/src/lib.rs); where the two disagree, the crate documentation wins. The method builds on the first pilot ([`lf-lift-pilot`](../crates/lf-lift-pilot/src/lib.rs) and the devlog entry "First pilot of the 64-bit lift"). It was written on 2026-10-04. Each finding is labelled: `(verified)` means it was run or read, `(inferred)` means it was reasoned from what was run or read.

## What the lift is

A verified rewrite is Rust in the checker's test form. It holds 32-bit addresses in `u32` words, reaches globals through the checker runtime's `relocated()` and `global()`, reaches other code through numbered callee slots, and uses x86 calling conventions. The checker has proven it equal to the original function (verified: that is what "verified" means in `rewrites/verified/`).

The lift restates the same behaviour as ordinary Rust for the portable game: no addresses, no numbered slots, no global state, no dependence on pointer width, and no `unsafe`. A lifted function is proven against its verified rewrite, not against the original, because the rewrite already carries the checker's proof.

The first lift crate holds 58 functions in three clusters (verified):

| Cluster | Functions | What it shows |
|---|---|---|
| Pure functions | 26 | Code maps, classifiers and float arithmetic; argument narrowing moved to the boundary |
| Forwarders | 19 | Callee slots become trait methods; seven deleting destructors and seven sort tails each become one generic function |
| Slot table | 13 | Twelve globals carried in one state struct; callees that may touch the same state receive it |

Three further functions were examined and deferred, each with its reason recorded in the crate's registry (verified).

## Global state: per-subsystem state structs

Every global a cluster touches becomes a field of one state struct for its subsystem. A lifted function takes that struct by shared reference when it only reads and by mutable reference when it writes, plus one argument for each further subsystem it touches. A top-level game object owns all the state structs and lends them out. Nothing reaches state any other way.

Four options were weighed (inferred from the pilot and from writing the clusters):

| Option | For | Against |
|---|---|---|
| One state struct per function (the pilot's form) | Exact borrows; trivial tests | Thousands of tiny structs; callers must assemble and split them; one global appears in many structs |
| One world object passed everywhere | One parameter | Every function borrows everything, so a callee that also needs the world cannot be called while any part is borrowed; tests must build the whole world; signatures say nothing about what a function touches |
| Thread-locals or statics | Closest to the original | Hidden state is what the lift removes: no parallel tests, no fakes, run-time borrow panics instead of compile-time errors |
| Per-subsystem structs (chosen) | Signatures name the subsystems touched; matches how the original groups its globals; tests build only what is involved; borrows checked at compile time | A function spanning subsystems takes several arguments; each global's subsystem is decided when it is first lifted |

On the slot table, thirteen functions over twelve globals share one struct, the read-only ones take a shared reference, and the borrow checker needed no workarounds (verified). Values the original keeps in read-only data are fields too, filled when state is loaded, so no game data is copied into source.

## Callees: one trait per collaborator group

Every callee slot becomes a method of a trait named for what the callees do: a lifecycle trait with "destruct" and "release", an insertion-sort trait with its two passes, a slot-table trait with "next serial", "current stamp" and "refresh entry". Lifted functions take the trait as a generic parameter, so dispatch is static. Production code passes the subsystem that implements the trait; tests pass a fake that records calls and scripts answers.

Method arguments are lifted forms: handles instead of object addresses, element ranges instead of byte ranges, booleans instead of flag words. The calling convention, placeholder arguments and leftover bits in a return register stay at the boundary. Closures per callee (the pilot's form) were dropped: readable for one or two callees, unreadable at five, and unable to share state with the caller without run-time borrow checks (inferred).

## The boundary between 32-bit layouts and native data

The boundary types live in [`lf-core`](../crates/lf-core/src/boundary.rs) and work the same on 32-bit and 64-bit targets (verified: they build and test on a 64-bit host and type-check for the 32-bit Windows target):

| Type | Role |
|---|---|
| `Handle` and `Arena` | How lifted code refers to objects: a slot index and a generation, two 32-bit words on every target; a stale handle never aliases a new object, and indexing with one panics |
| `Handle32` | A typed, opaque 32-bit value the lifted code carries but never interprets, such as the identity of an object owned by code not yet lifted |
| `FixedLayout` | A 32-bit `#[repr(C)]` layout with an explicit little-endian codec, tied to its Rust size at compile time |
| `Image32` | A 32-bit address space seen as bytes: a test buffer, a snapshot, or later the running original |
| `FromLayout` and `IntoLayout` | Conversions between layouts and native structs; writing goes into an existing layout, so bytes the native struct does not model survive |
| `AddressMap` | Binds 32-bit addresses to handles and back; an unbound address is an error, never a guess |

Each cluster has a `layout` module that names where its globals live and loads and stores its state struct. On the slot table, a load and store round trip leaves every byte as it was except the used bytes, which normalise to 0 or 1 (verified). Addresses appear only in these modules; the lifted functions never use them.

## Rules for every lifted function

- Integers keep the width the original computes with, and arithmetic wraps exactly where the original's does.
- Floating-point operations happen in the original's order and precision. Results match bit for bit, except that the payload of a NaN result is left to the hardware, because ARM and x86 propagate payloads differently (inferred from the hardware documentation; the comparison treats any NaN as equal to any NaN).
- An argument the original narrows (a low byte, a low half, one flag bit) arrives as the narrow type. A result that carries more than its meaning (register residue above a byte answer, an address instead of an index) is narrowed to its meaning, and the differential test pins the original's shape.
- Where the original reads memory it never validated, the lift has a stated narrower domain or panics with a message. It never guesses (rule from the pilot, kept).
- Every narrowing is recorded per function in the crate's registry, which is also where lifted and deferred counts come from.

## How a lift is proven

The differential harness ([`lf-lift-diff`](../crates/tools/lf-lift-diff/src/lib.rs)) compiles the verified rewrite files unchanged and runs each rewrite and its lift on the same generated inputs:

1. Inputs mix edge values (zero, sign boundaries, all ones, NaN, infinities, subnormals, table bounds) with seeded random values; small domains such as bytes are enumerated in full.
2. Results must agree after the documented narrowing.
3. Every callee call the rewrite makes must equal the translation of every trait call the lift makes, slot and arguments in order, with both sides given the same scripted answers.
4. For state, both sides run against the same test buffer, reset to the same starting bytes before each run, so addresses either side computes are comparable. The lifted state is stored over it and compared with the image the rewrite left, over every modelled region and then in whole, which catches writes outside the model.
5. Every case also runs a deliberately wrong lift (a plausible mistake: an off-by-one threshold, the wrong flag bit, a different operation order), which must be caught. A case whose wrong version passes does not count.

On the 32-bit target the rewrites call the real checker runtime, with its relocated base pointed at a test buffer and its callee table at recording stubs. On other hosts a small stand-in runtime with the same names runs the same files: 57 of the 58 cases run and pass on a 64-bit Linux host (verified). The one remaining case turns a relocated address into a pointer itself and needs the 32-bit target. The first 32-bit run on the Windows runner passed 44 of 47 cases; the other three compared addresses computed against two different test buffers, a harness defect now fixed (the host stand-in now catches that kind of mistake too). A rerun is pending.

## What it costs

The rates come from the first pilot: about five minutes per plain function and about eleven per function with callees, for a lane after its one-time tooling (as recorded in the pilot's devlog entry; not re-measured here, because this crate's functions were the easiest and most of its time went into the boundary types and the harness).

The mix comes from a text scan of all 6,038 verified rewrites on 2026-10-04 for global access, callee calls and raw-pointer patterns (verified, approximate because it is a pattern scan):

| Share | What the functions touch |
|---|---|
| 1.8% | Nothing: arguments in, value out |
| 3.8% | Callees only |
| 7.2% | Globals only |
| 17.7% | Globals and callees, no object memory |
| 69.6% | Object memory, with or without globals and callees |

Applying the pilot's rates to that mix (4,290 functions with callees at eleven minutes, 1,748 without at five) gives about 930 lane-hours (inferred). That figure leaves out the largest unknown: the 69.6% that dereference object memory need each object's native struct and layout codec before they can lift. That work is paid once per class and shared by every function of the class, and neither the pilot nor this crate has measured it.

## Order of work

1. Pure functions, which are mechanical.
2. Forwarders, grouped by family: one generic function per family, then a registry line and a differential case per member.
3. One subsystem's globals at a time, together with its `layout` module.
4. Classes: an object's native struct and layout codec, then the functions that touch it, starting with the classes that have the most verified functions.
5. Callback-dense code last.

Functions that embed addresses in values (pointer arithmetic on buffers, addresses of members returned) wait for the layout of what they point into, and are recorded as deferred with the reason (inferred from the three deferred so far).
