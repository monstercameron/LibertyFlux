//! Shared differential-test support: seeded RNG, image helpers, serial lock.
//!
//! One lock guards each whole test: the runtime's global slot is a plain
//! word behind a stable address, so tests sharing a binary must not run
//! rewrite calls concurrently.

use std::sync::Mutex;
use std::sync::MutexGuard;

static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Holds the binary's test lock for the whole test.
pub fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

/// Minimal seeded RNG (xorshift32): deterministic across targets.
#[derive(Clone)]
pub struct Rng(pub u32);

impl Rng {
    /// Next word.
    pub fn u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }

    /// Fills `out` with random bytes.
    pub fn bytes(&mut self, out: &mut [u8]) {
        for b in out.iter_mut() {
            *b = self.u32() as u8;
        }
    }
}

/// Address of a test allocation as the rewrites see it (32-bit only).
#[must_use]
pub fn addr<T>(r: &T) -> u32 {
    (r as *const T as usize) as u32
}

/// Edge words: zero, small, sign boundary, all ones.
pub const EDGE_WORDS: [u32; 8] = [
    0x0000_0000,
    0x0000_0001,
    0x0000_0002,
    0x0000_0100,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_fffe,
    0xffff_ffff,
];

/// Edge bytes: zero, one, sign boundary neighbours, all ones.
pub const EDGE_BYTES: [u8; 8] = [0x00, 0x01, 0x02, 0x7f, 0x80, 0xfe, 0xff, 0x0e];
