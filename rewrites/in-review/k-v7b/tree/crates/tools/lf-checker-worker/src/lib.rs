//! `lf_checker_worker`: the pure, target-independent logic of the checker
//! worker (the binary in `main.rs` is the 32-bit process that executes
//! original code).
//!
//! Everything here is plain computation over values the worker has already
//! captured: decoding the x87 state from an FXSAVE image and comparing it,
//! planning where pointed-to snapshot words and extra vector registers land
//! in the call log, encoding the few instruction bytes those features emit,
//! and planning the preferred-base window that catches unrelocated absolute
//! accesses. None of it touches memory it does not own or calls the OS, so
//! it builds and its unit tests run on any host
//! (`cargo test -p lf-checker-worker --lib`), not only on the Windows
//! runner that can execute the worker itself.

pub mod abswin;
pub mod snap;
pub mod vecregs;
pub mod x87;
