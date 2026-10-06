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

/// Reads a little-endian word from a test image.
#[must_use]
pub fn get_u32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

/// The pool context global both sides publish through.
pub const CTX_VA: u32 = 0x0117_64C0;
/// The indexed store's table scale global.
pub const SCALE_VA: u32 = 0x0103_2F58;
/// The release's pool manager global.
pub const MGR_VA: u32 = 0x016D_D5D0;
/// The release's auxiliary global.
pub const AUX_VA: u32 = 0x0104_96E8;
/// The seven cursor-step record globals, in test order.
pub const REC_VAS: [u32; 7] = [
    0x016F_7D60,
    0x012B_D0E8,
    0x0166_D9EC,
    0x018B_6F10,
    0x0163_2C60,
    0x018B_6F1C,
    0x012E_22A4,
];
/// The entry table's base and end-bias pointers (relocated).
pub const E_TAB_VA: u32 = 0x0120_F2B8;
/// The entry table's end-bias pointer: 8 past the base (relocated).
pub const E_END_VA: u32 = 0x0120_F2C0;
/// The revocation table's header words.
pub const R_A_VA: u32 = 0x0167_CA10;
/// The revocation table's second header word.
pub const R_B_VA: u32 = 0x0167_CA14;
/// The revocation table's four header bytes.
pub const R_C_VA: u32 = 0x0167_CA18;
/// The revocation table's fourth header word.
pub const R_D_VA: u32 = 0x0167_CA1C;
/// The revocation table's slot-reset object (relocated).
pub const R_SLOTS_VA: u32 = 0x0167_CA20;
/// The revocation table's first flag byte (relocated).
pub const R_FIRST_VA: u32 = 0x0167_0D29;
/// The revocation table's end cursor (relocated).
pub const R_END_VA: u32 = 0x0167_CA39;
/// The handle table's base (relocated).
pub const H_TAB_VA: u32 = 0x0129_5CD8;
/// The bare bump pool's count and base globals.
pub const BC_COUNT_VA: u32 = 0x0103_ADBC;
/// The bare bump pool's base global.
pub const BC_BASE_VA: u32 = 0x0103_ADC0;
/// The two published-object global slots.
pub const OBJ_VAS: [u32; 2] = [0x012B_4160, 0x012B_4164];
