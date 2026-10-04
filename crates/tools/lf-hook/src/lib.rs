//! `lf-hook`: hook engine for the `LibertyFlux` replacement loader.
//!
//! * [`decode`]: x86-32 instruction-length decoder.
//! * [`detour`]: 5-byte jump hooks with trampolines.
//! * [`slot`]: vtable / import-slot pointer hooks.
//! * [`adapters`]: generated `thiscall` stubs for stable Rust.
//! * [`pe`]: minimal PE reader (import slots, sections, export tables).
//! * [`forward`]: the winmm proxy's export-forwarding table generator
//!   (pure; `lf-proxy`'s build script `#[path]`-includes it).
//! * [`mem`]: raw Win32 memory/thread helpers.
//! * [`log`]: timestamped file logger.
//!
//! Windows-only. The decoder and PE file reader are pure logic and are unit
//! tested on the host (`cargo test -p lf-hook`); everything that patches or
//! executes machine code is exercised by the 32-bit `lf-test-target` binary.
//!
//! Usage: `Detour` for inline hooks, `SlotHook` for vtable/import slots,
//! `adapters` for `thiscall` stubs around plain `extern "C"` Rust functions.
//! Only modules that patch memory or call Win32 re-allow `unsafe_code`,
//! each with its own justification comment.

// Integrated lane code, proven by host unit tests and the 32-bit live
// suite: address casts, long decoders and shared match arms are inherent
// to the domain, so pedantic style lints stay off here. Correctness lints
// (clippy::all) still apply; narrow this if the code is reworked.
#![allow(clippy::pedantic)]

pub mod adapters;
pub mod decode;
pub mod detour;
pub mod forward;
pub mod log;
pub mod mem;
pub mod pe;
pub mod slot;
