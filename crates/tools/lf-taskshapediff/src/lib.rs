//! `lf-taskshapediff`: differential tests of two task shapes of the ped
//! task family against their verified 32-bit rewrites.
//!
//! The verified rewrite files are included unchanged; on the 32-bit target
//! they see this crate as `lf_checker_rt` via `extern crate self as`, with
//! the same macro name the checker builds them against. The routines in
//! these shapes read and write only memory the test plants, so no callee
//! stubs or global windows are needed.
//!
//! The differential cases run on the 32-bit target only: the rewrites take
//! raw addresses. On other hosts the crate builds and runs no rewrite
//! cases; the lifted crate's own host tests run everywhere.

// The crate root re-exports the runtime surface under the checker's name so
// `lf_checker_rt::...` paths in the included verified files resolve here.
extern crate self as lf_checker_rt;

pub mod rt;

pub use rt::callee_addr;
pub use rt::global;
pub use rt::relocated;

#[cfg(target_arch = "x86")]
pub mod rewrites;

/// A small deterministic generator for the test cases. Each test program
/// seeds it from a constant, so a failing case can be replayed.
pub struct Xorshift(u32);

impl Xorshift {
    /// A generator from a nonzero seed.
    #[must_use]
    pub fn new(seed: u32) -> Self {
        Self(if seed == 0 { 0x9E37_79B9 } else { seed })
    }

    /// The next 32-bit value.
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// A value from a small set of boundary values or a random one, so the
    /// edge cases (zero, one, the sign boundary) come up often.
    pub fn word(&mut self) -> u32 {
        const EDGES: [u32; 8] = [0, 1, 2, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF, 0x80, 0x3C0];
        let pick = self.next_u32();
        if pick % 3 == 0 {
            EDGES[(pick >> 8) as usize % EDGES.len()]
        } else {
            self.next_u32()
        }
    }
}
