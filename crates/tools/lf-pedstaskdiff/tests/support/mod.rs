//! Shared differential-test support: seeded RNG, image helpers, serial lock.
//!
//! One lock guards each whole test: the runtime's global cells are plain
//! words behind stable addresses, so tests sharing a binary must not run
//! rewrite calls concurrently.

// Shared support: each test binary uses only its own subset.
#![allow(dead_code)]

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

/// Address of a stub or rewrite function as the rewrites see it (32-bit
/// only). A macro because each stub has its own function type.
macro_rules! fn_addr {
    ($f:expr) => {
        $f as *const () as usize as u32
    };
}

pub(crate) use fn_addr;

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

/// Writes the low word of a global cell.
pub fn set_cell_u32(file_va: u32, v: u32) {
    unsafe {
        lf_pedstaskdiff::global::<u32>(file_va).write_unaligned(v);
    }
}

/// Reads the low word of a global cell through a raw pointer.
#[must_use]
pub fn cell_u32(file_va: u32) -> u32 {
    unsafe { lf_pedstaskdiff::global::<u32>(file_va).read_unaligned() }
}

/// Writes the low byte of a global cell.
pub fn set_cell_u8(file_va: u32, v: u8) {
    unsafe {
        lf_pedstaskdiff::global::<u8>(file_va).write(v);
    }
}

/// Reads the low byte of a global cell through a raw pointer.
#[must_use]
pub fn cell_u8(file_va: u32) -> u8 {
    unsafe { lf_pedstaskdiff::global::<u8>(file_va).read() }
}

/// Writes a whole 8-byte global cell (the double classifier bound).
pub fn set_cell_u64(file_va: u32, v: u64) {
    unsafe {
        lf_pedstaskdiff::global::<u64>(file_va).write_unaligned(v);
    }
}

/// The shared-state flag byte.
pub const ST_FLAG: u32 = 0x0171_BBE0;
/// The out bias word.
pub const ST_OUT_BIAS: u32 = 0x0171_BBE4;
/// The gate bias word.
pub const ST_GATE_BIAS: u32 = 0x0171_BBE8;
/// The range upper-bound word.
pub const ST_RANGE_HI: u32 = 0x0171_BBEC;
/// The gate vector words.
pub const ST_A: [u32; 3] = [0x0171_BC10, 0x0171_BC14, 0x0171_BC18];
/// The gate vector's pad word (rewrite pins it to zero).
pub const ST_A_PAD: u32 = 0x0171_BC1C;
/// The work vector words.
pub const ST_B: [u32; 3] = [0x0171_BF20, 0x0171_BF24, 0x0171_BF28];
/// The raw float cells in object order: (a, c, f, raw4, b, e, d, raw7).
pub const ST_RAW: [u32; 8] = [
    0x0171_BF10,
    0x0171_BF14,
    0x0171_BF18,
    0x0171_BF1C,
    0x0171_BF00,
    0x0171_BF04,
    0x0171_BF08,
    0x0171_BF0C,
];
/// The extra scratch word (rewrite pins it to zero).
pub const ST_EXTRA: u32 = 0x0171_BF2C;
/// The build's copy source (also the reaction counter threshold).
pub const ST_SRC_DWORD: u32 = 0x0117_35B4;
/// The reaction near-window upper bound (-0.2 in the image).
pub const C_LOWER: u32 = 0x00FE_8D68;
/// The reaction near-window lower bound (-1.2 in the image).
pub const C_FAR: u32 = 0x00EB_9504;
/// The reaction mid-window upper bound (+1.2 in the image).
pub const C_NEAR: u32 = 0x00FE_891C;
/// The reaction mid-window lower bound (0.2 in the image, double).
pub const C_MID: u32 = 0x00EA_87B8;
/// The state consumer callback passed to the enumeration call.
pub const CONSUMER_VA: u32 = 0x00CB_7CF0;
