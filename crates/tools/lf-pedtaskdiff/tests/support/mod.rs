//! Shared differential-test support: seeded RNG, image helpers, serial lock.
//!
//! One lock guards each whole test: the runtime's global slots are plain
//! words behind stable addresses, so tests sharing a binary must not run
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

    /// A word below `n`.
    pub fn below(&mut self, n: u32) -> u32 {
        if n == 0 { 0 } else { self.u32() % n }
    }

    /// A word with each byte picked from edge-heavy corners sometimes.
    pub fn edge_word(&mut self) -> u32 {
        match self.below(8) {
            0 => [0, 1, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF][self.below(5) as usize],
            1 => self.below(256),
            _ => self.u32(),
        }
    }

    /// A float bit pattern: edge values, boundary floats and random bits.
    pub fn float_bits(&mut self) -> u32 {
        // Edge patterns: zeros, ones, extrema, infinities, NaNs, subnormals.
        const EDGES: [u32; 12] = [
            0x0000_0000,
            0x8000_0000,
            0x3F80_0000,
            0xBF80_0000,
            0x7F7F_FFFF,
            0xFF7F_FFFF,
            0x7F80_0000,
            0xFF80_0000,
            0x7FC0_0000,
            0xFFC0_0000,
            0x0000_0001,
            0x8000_0001,
        ];
        match self.below(4) {
            0 => EDGES[self.below(12) as usize],
            1 => self.u32() & 0x7FFF_FFFF,
            _ => self.u32(),
        }
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

/// Address of a boxed test allocation's contents (32-bit only).
#[must_use]
pub fn heap_addr<T>(r: &Box<T>) -> u32 {
    (&**r as *const T as usize) as u32
}

/// Writes a little-endian word into a test image.
pub fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// Reads a little-endian word from a test image through a raw pointer,
/// so the compiler cannot forward a pre-call value.
#[must_use]
pub fn get_u32(buf: &[u8], off: usize) -> u32 {
    unsafe { (buf.as_ptr().add(off) as *const u32).read_unaligned() }
}

/// The pose tuning words, in test order.
pub const HALF_VA: u32 = 0x00FE_8830;
/// The one word of the normalisation guards.
pub const ONE_VA: u32 = 0x00FE_88E8;
/// The half-pi shift word.
pub const HALF_PI_VA: u32 = 0x00FE_8978;
/// The full-turn modulus word.
pub const TAU_VA: u32 = 0x00FE_8AEC;
/// The sign-flip mask word.
pub const NEG_VA: u32 = 0x00FE_8FA0;
/// The absolute-value mask word.
pub const ABS_VA: u32 = 0x00FE_8F80;
/// The ped manager word.
pub const PEDMGR_VA: u32 = 0x018B_6F1C;
/// The argument lookup word.
pub const LOOKUP_VA: u32 = 0x012E_22A4;
