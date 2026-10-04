//! `lf-lift`: verified rewrites lifted to portable, safe Rust.
//!
//! This crate documentation is the specification of the lift. Findings are
//! labelled **Verified** (run or read here), **Inferred** (reasoned from
//! what was run or read) or **Unknown**.
//!
//! # What the lift is
//!
//! A verified rewrite (in `rewrites/verified/`) is Rust in the checker's
//! test form: 32-bit addresses held in `u32`, globals reached through
//! `relocated()`/`global()`, other code reached through numbered callee
//! slots, x86 calling conventions. It is proven equal to the original by
//! the checker. The lift restates the same behaviour as code for the
//! portable 64-bit game:
//!
//! 1. No addresses. Objects are [`lf_core::Handle`]s into
//!    [`lf_core::Arena`]s; tables are slices indexed by element; opaque
//!    values owned by code not yet lifted are [`lf_core::Handle32`]s.
//!    Addresses appear only in `layout` modules, which the lifted
//!    functions never call.
//! 2. No numbered slots: callees are trait methods (see *Callees* below).
//! 3. No global state: globals are fields of state structs passed
//!    explicitly (see *Global state* below).
//! 4. No dependence on pointer width: every integer the original computes
//!    with keeps its width (`u32`, `i32`, `u16`, `u8`); `usize` appears
//!    only as an element index or count.
//! 5. `#![forbid(unsafe_code)]`.
//! 6. Arithmetic wraps exactly where the original's wraps, and only there
//!    (`wrapping_*` is written out).
//! 7. Floating point: operations happen in the original's order and
//!    precision (`f32` stays `f32`; a widening or narrowing conversion is
//!    written where the original makes it). Results match bit for bit,
//!    except that the payload of a NaN result is not part of the lifted
//!    contract: the target hardware decides it (ARM and x86 propagate
//!    payloads differently), so a lifted NaN only has to be a NaN.
//! 8. An argument the original narrows (low byte, low half, one flag bit)
//!    is narrowed at the boundary and arrives as the narrow type (`u8`,
//!    `u16`, `bool`). A result that carries residue beyond its meaning
//!    (register bits above a byte answer, an address instead of an index)
//!    is narrowed to its meaning, and the differential test pins the
//!    original's shape on the 32-bit side.
//! 9. Where the original reads memory it never validated, the lift either
//!    has a narrower domain (stated on the item: a valid range, a cursor
//!    inside its table) or panics with a message naming the violation. It
//!    never guesses. (Rule from the first lift pilot, kept.)
//!
//! Every narrowing is recorded per function in [`registry::LIFTED`].
//!
//! # Global state
//!
//! Decision: **per-subsystem state structs, passed explicitly.** Each
//! cluster's globals become the fields of one state struct (for example
//! [`slot_table::SlotTableState`]); a lifted function takes `&State` when
//! it reads and `&mut State` when it writes, plus one argument per further
//! subsystem it touches. A top-level game object owns every state struct
//! and lends them out; nothing below it reaches state any other way.
//!
//! Alternatives weighed (Inferred from the pilot and from writing the
//! clusters here):
//!
//! | Option | For | Against |
//! |---|---|---|
//! | One state struct per function (the pilot) | Exact borrows; trivial tests | Thousands of tiny structs; callers must assemble and split them; the same global appears in many structs and drifts |
//! | One `World` passed everywhere | One parameter; easy to thread | Every function borrows everything, so a callee that also needs the world cannot be called while any part is borrowed; tests must build the whole world; signatures say nothing about what a function touches |
//! | Thread-locals or statics | Closest to the original; no signature changes | Hidden state is the problem being removed: tests cannot run in parallel or with fakes, `RefCell` turns aliasing bugs into run-time panics, and ordering of initialisation returns |
//! | **Per-subsystem structs** | Signatures name the subsystems touched; one struct per subsystem matches how the original groups its globals; tests build only the subsystems involved; borrows are checked at compile time | A function spanning subsystems takes several arguments; a global's subsystem must be decided when it is first lifted (recorded in its `layout` module) |
//!
//! Verified on [`slot_table`]: thirteen functions over twelve globals share
//! one struct; the read-only functions take `&`, and the borrow checker
//! needed no workarounds. A callee that may touch the caller's state
//! receives it as an argument (`ops.refresh_entry(st, index)`) instead of
//! the caller holding a borrow across the call.
//!
//! Values in read-only data (a scale constant, say) are fields too, filled
//! when state is loaded, so no game data is copied into source.
//!
//! # Callees
//!
//! Decision: **one trait per collaborator group, statically dispatched.**
//! Every callee slot a lifted function uses becomes a method of a trait
//! named for what the callees do ([`forward::Lifecycle`],
//! [`forward::InsertionSort`], [`slot_table::SlotTableOps`]). Functions take
//! `&mut impl Trait` (written as a generic with `?Sized`, so `dyn` also
//! works); production code passes the subsystem that implements it, tests
//! pass a fake that records calls and scripts answers. Method arguments
//! are the lifted forms (handles, indices, element ranges, `bool`s), and
//! the slot's convention, placeholder arguments and return-register
//! residue stay at the boundary. Family members that differ only in their
//! callees share one generic function (seven deleting destructors are one
//! [`forward::deleting_destructor`]).
//!
//! Rejected: closures per callee (the pilot's form; fine for one or two,
//! unreadable at five, and they cannot share state with the caller without
//! `RefCell`), and function-pointer tables (the original's form; untyped).
//!
//! # The boundary
//!
//! Specified in [`lf_core::boundary`]: `#[repr(C)]` layouts with explicit
//! little-endian codecs, [`Image32`](lf_core::boundary::Image32) address
//! spaces, conversion traits that write into existing layouts so
//! unmodelled bytes survive, and an
//! [`AddressMap`](lf_core::boundary::AddressMap) from 32-bit addresses to
//! handles. Each cluster's `layout` module names its globals' addresses
//! and implements `load`/`store` of its state struct (Verified on
//! [`slot_table::layout`]: a load and store round trip leaves every byte
//! unchanged except used bytes, which normalise to 0 or 1).
//!
//! # Proof
//!
//! A lifted function is proven against its verified rewrite, never
//! against the original directly (the rewrite is already proven equal to
//! the original by the checker). `crates/tools/lf-lift-diff` compiles the
//! verified rewrite files unchanged and runs both forms on the same
//! generated inputs:
//!
//! - **Inputs**: edge values (zero, one, sign boundaries, all ones, NaN,
//!   infinities, subnormals, table bounds) mixed with seeded random values.
//! - **Results**: equal after the documented narrowing; floats bit for
//!   bit, NaN payload excepted.
//! - **Calls**: every callee call the rewrite makes (slot, arguments in
//!   order) equals the translation of every trait call the lift makes;
//!   both sides get the same scripted answers.
//! - **State**: both forms run against the same image buffer, reset to
//!   the same starting bytes before each run (one buffer, so an address
//!   either side computes is in the same address space). The lifted state
//!   is stored over the buffer after the lift's run and compared with the
//!   image the rewrite left, byte for byte, over every modelled region; a
//!   final whole-image compare catches writes outside them.
//! - **Wrong versions**: every case also runs a deliberately wrong lift,
//!   which must be caught. A case whose wrong version passes does not
//!   count.
//!
//! On the 32-bit target the rewrites run with the real checker runtime
//! (`lf-checker-rt`), its statics pointed at a test buffer and at
//! recording stubs. On other hosts a small stand-in runtime runs the same
//! files for the cases that do not need 32-bit pointers (Verified: 57 of
//! the 58 cases run on a 64-bit Linux host). The stand-in's `relocated()`
//! depends on the buffer as the real runtime's does, so a case that mixes
//! two address spaces fails on the host too.
//!
//! The first 32-bit run (Windows runner, 4 October 2026) passed 44 of 47
//! cases. The other three compared addresses computed against two
//! different test buffers, a defect of the harness, not of the lifts
//! (Verified: the host stand-in, made buffer-dependent, reproduced it on
//! the same input). The harness now runs both forms against one buffer;
//! whether all 47 pass on the 32-bit target is Unknown until the next run.
//!
//! # Order of work
//!
//! Inferred from the pilot and from this crate: lift pure functions first
//! (mechanical), then forwarders grouped by family (one generic per
//! family, then a registry line and a differential case per member), then
//! one subsystem's globals together with its `layout` module, then
//! callback-dense code. Functions that embed addresses in values (pointer
//! arithmetic on buffers, member addresses returned) wait for the layout of
//! what they point into; they are recorded in [`registry::DEFERRED`] with
//! the reason.
//!
//! Verified, approximately (a text scan of `rewrites/verified/functions/`
//! on 4 October 2026 for `global`/`relocated`, `callee_*` and raw-pointer
//! patterns; 6,038 files): 1.8% touch no memory, globals or callees; 3.8% only call
//! callees; 7.2% only touch globals; 69.6% dereference object memory
//! (with or without globals and callees). That last group needs the native
//! struct and the layout codec of each object it touches before it can
//! lift, so the order above is also the order in which the boundary work
//! gets paid for: one struct per class, shared by every function of the
//! class.

#![forbid(unsafe_code)]

pub mod forward;
pub mod pure;
pub mod registry;
pub mod slot_table;
