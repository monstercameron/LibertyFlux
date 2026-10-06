//! Shared differential-test support: generator, edge values, addresses.
//!
//! Included by each `diff*.rs` test target (`#[path]`), so every target
//! gets its own copy. 32-bit only: addresses are real.

use lf_animation::channel::frame::{SNAP_HI, SNAP_LO};

/// Small deterministic generator (splitmix64).
pub struct Rng(pub u64);

impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn u32(&mut self) -> u32 {
        (self.next() >> 32) as u32
    }

    pub fn u64(&mut self) -> u64 {
        self.next()
    }

    /// Any f32 bit pattern, NaN payloads and subnormals included.
    pub fn f32_bits(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }

    /// A small exact float in `lo..hi`.
    pub fn small(&mut self, lo: i32, hi: i32) -> f32 {
        let span = (hi - lo) as u32;
        (lo as f32) + ((self.u32() % span.max(1)) as f32) * 0.5
    }

    /// A frame in `[-2, end + 2)`: in range, at the edges, and past them.
    pub fn frame(&mut self, end: f32) -> f32 {
        let pick = self.u32() % 8;
        match pick {
            0 => self.small(-4, (end as i32) + 5),
            1 => self.small(-4, (end as i32) + 5) + 0.9995,
            2 => self.small(-4, (end as i32) + 5) + 0.0005,
            3 => self.f32_bits(),
            4 => -self.small(0, 200),
            5 => end + self.small(0, 200),
            6 => F32_EDGE[(self.u32() as usize) % F32_EDGE.len()],
            _ => self.small(0, (end as i32).max(1) + 1),
        }
    }
}

/// Edge float values: zeros, ones, snap thresholds, rounding pivots,
/// extremes, infinities, quiet and signalling NaNs.
pub const F32_EDGE: [f32; 24] = [
    0.0,
    -0.0,
    1.0,
    -1.0,
    0.5,
    -0.5,
    2.5,
    SNAP_LO,
    SNAP_HI,
    0.9995,
    1.0005,
    0.1,
    100.0,
    8_388_608.0,
    8_388_607.0,
    2_147_483_648.0,
    f32::MIN_POSITIVE,
    f32::MIN,
    f32::MAX,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::NAN,
    f32::from_bits(0x7FC0_1234),
    f32::from_bits(0x7F80_0001),
];

/// Edge words: zero, one, sign boundary, all ones, small counts.
pub const U32_EDGE: [u32; 10] = [
    0,
    1,
    2,
    3,
    0x7FFF_FFFF,
    0x8000_0000,
    0xFFFF_FFFE,
    0xFFFF_FFFF,
    0x5E,
    0xDEAD_BEEF,
];

/// Address of a referent as the rewrites take it (32-bit target only).
pub fn addr<T>(r: &T) -> u32 {
    (r as *const T).addr() as u32
}

/// `#[repr(C)]` 12-byte static object: vtable, header, inline value word.
#[repr(C)]
pub struct StaticObj {
    pub vtable: u32,
    pub hdr: [u8; 4],
    pub value: u32,
}

/// `#[repr(C)]` 16-byte raw object: vtable, header, keys, count, gate.
#[repr(C)]
pub struct RawObj {
    pub vtable: u32,
    pub hdr: [u8; 4],
    pub keys: u32,
    pub count: u16,
    pub gate: u16,
}

/// `#[repr(C)]` 12-byte slotted object: vtable, header, value-block pointer.
#[repr(C)]
pub struct SlotObj {
    pub vtable: u32,
    pub hdr: [u8; 4],
    pub slot: u32,
}

/// `#[repr(C)]` 24-byte curve object: vtable, header, key base, count,
/// capacity, scale and bias words.
#[repr(C)]
pub struct CurveObj {
    pub vtable: u32,
    pub hdr: [u8; 4],
    pub base: u32,
    pub count: u16,
    pub cap: u16,
    pub scale: u32,
    pub bias: u32,
}

/// `#[repr(C)]` 8-byte curve key record: key, order byte, pad, coefficient
/// pointer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CurveKeyRec {
    pub key: u16,
    pub order: u8,
    pub pad: u8,
    pub coeff: u32,
}

/// Zeroed 64-byte object blob for methods reading words at fixed offsets.
pub fn blob64() -> Box<[u8; 64]> {
    Box::new([0u8; 64])
}

/// Writes a little-endian word into a blob.
pub fn put_u32(blob: &mut [u8; 64], off: usize, v: u32) {
    blob[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

/// Writes a little-endian half word into a blob.
pub fn put_u16(blob: &mut [u8; 64], off: usize, v: u16) {
    blob[off..off + 2].copy_from_slice(&v.to_le_bytes());
}
