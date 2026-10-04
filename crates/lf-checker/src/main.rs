//! `lf-checker`: the comparison harness and queue tooling (host-only).
//!
//! README for future lanes:
//! - This binary runs the native side-by-side comparison from plan.md: it
//!   drives a 32-bit helper that calls the original function and the
//!   32-bit Rust build of the rewrite with the same inputs and compares
//!   results, memory writes and outgoing calls.
//! - It also owns the work queue, context packets, claims and accept
//!   records. Lanes may run its commands but never edit it (AGENTS.md).
//! - Host-only tool: it runs on CI and dev machines, never inside the
//!   game, and is never a dependency of engine crates.
//! - Not implemented yet: this stub only proves the workspace builds a
//!   binary. The h-checker lane owns the real design.

fn main() {
    println!("lf-checker: not implemented yet (scaffold only)");
}
