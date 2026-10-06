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

/// Writes a little-endian word into a test image.
pub fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// The handle table's base (relocated).
pub const TABLE_VA: u32 = 0x0118_F6F8;
/// The default-slot index word (global).
pub const DEFAULT_VA: u32 = 0x0103_4494;
/// The three notify-sink words (globals holding the sink objects).
pub const SINK_VAS: [u32; 3] = [0x012E_22A4, 0x018B_6F1C, 0x0163_2C60];
/// The announce sink object (relocated: the word's value is the object).
pub const EMIT_SINK_VA: u32 = 0x0116_BFF0;
/// The word naming the destroy routine's TLS slot (global).
pub const TLS_WORD_VA: u32 = 0x017A_BA14;
/// The allocation registry's counter word (global).
pub const COUNT_VA: u32 = 0x0118_F4E0;
/// The registry's live-entry table (relocated).
pub const TAB_A_VA: u32 = 0x0118_F4EC;
/// The registry's failed-allocation table (relocated).
pub const TAB_B_VA: u32 = 0x0118_F4F0;
/// The key table's base (relocated).
pub const KEY_TAB_VA: u32 = 0x011A_0BF0;
/// The key table's end (relocated).
pub const KEY_END_VA: u32 = 0x011A_13F0;
