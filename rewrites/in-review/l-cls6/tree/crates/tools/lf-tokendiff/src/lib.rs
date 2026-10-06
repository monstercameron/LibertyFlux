//! `lf-tokendiff`: differential tests of the lifted tokenizer.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`,
//! with the same macro and function names the checker builds them
//! against. Numbered callees dispatch through a script the test installs,
//! with snapshotting stubs where the rewrite passes buffers; virtual-slot
//! calls land on test-owned stubs; the streaming globals map to
//! test-owned tables.
//!
//! The differential cases run on the 32-bit target only: the rewrites
//! take real addresses. On other hosts the crate builds but runs no
//! rewrite cases (the lifted crate's own host tests run everywhere).

// The crate root re-exports the runtime surface under the checker's names
// so `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

// The runtime declares stand-ins with the 32-bit calling conventions the
// rewrites call through, which exist only on that target.
#[cfg(target_arch = "x86")]
pub mod rt;

#[cfg(target_arch = "x86")]
pub use rt::callee_addr;
#[cfg(target_arch = "x86")]
pub use rt::global;
#[cfg(target_arch = "x86")]
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;
