//! Resource paging and loading.
//!
//! Holds 4 draft layouts: RAGE's paged-resource classes `rage::pgBase`, `rage::pgBasicScheduler`
//! and `rage::pgStreamableRef<rage::fragType>`, and the game's `CLoaderDirect`. Every layout is
//! Inferred; size confidence (the analysis lanes' own rating) is high for 0, medium for 0 and low
//! for 4. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CLoaderDirect`.
///
/// Size: 0x3148 (low). Bases: CLoader@0x0.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLoaderDirect {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_+0x4 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x4: u32,
    /// Unknown bytes (0x8..0x10).
    pub _pad_0008: [u8; 0x8],
    /// field_+0x10 (confidence: medium, kind: float, lanes: c-core).
    pub field_0x10: f32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// field_+0x20 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x20: u32,
    /// Unknown bytes (0x24..0x28).
    pub _pad_0024: [u8; 0x4],
    /// field_+0x28 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x28: u32,
    /// Unknown bytes (0x2c..0x620).
    pub _pad_002c: [u8; 0x5f4],
    /// field_+0x620 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x620: u32,
    /// Unknown bytes (0x624..0x3138).
    pub _pad_0624: [u8; 0x2b14],
    /// field_+0x3138 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x3138: u32,
    /// field_+0x313c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x313c: u32,
    /// field_+0x3140 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x3140: u32,
    /// field_+0x3144 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x3144: u32,
}
assert_size!(CLoaderDirect, 0x3148); // merged size 0x3148 rounded to 4
assert_offset!(CLoaderDirect, field_0x4, 0x4);
assert_offset!(CLoaderDirect, field_0x10, 0x10);
assert_offset!(CLoaderDirect, field_0x20, 0x20);
assert_offset!(CLoaderDirect, field_0x28, 0x28);
assert_offset!(CLoaderDirect, field_0x620, 0x620);
assert_offset!(CLoaderDirect, field_0x3138, 0x3138);
assert_offset!(CLoaderDirect, field_0x313c, 0x313c);
assert_offset!(CLoaderDirect, field_0x3140, 0x3140);
assert_offset!(CLoaderDirect, field_0x3144, 0x3144);

/// Merged layout for `rage::pgBase`.
///
/// Size: 0xf0 (low). Bases: rage::datBase@0x0.
/// Lanes: c-core, c-misc-b, c-physics, c-render, p0-360, via:gtaDrawable, via:rage::fragDrawable, via:rage::fragType, via:rage::pgDictionary<gtaDrawable>, via:rage::pgDictionary<rage::crAnimation>, via:rage::pgDictionary<rage::phBound>, via:rage::pgDictionary<rage::ptxEffectRule>, via:rage::pgDictionary<rage::ptxEmitRule>, via:rage::pgDictionary<rage::ptxRule>, via:rage::pgDictionary<rage::rmcDrawable>, via:rage::rmcDrawableBase.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePgBase {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_+0x4 (confidence: high, kind: u32?, lanes: c-core,c-render,p0-360).
    pub field_0x4: u32,
    /// Unknown bytes (0x8..0x1c).
    pub _pad_0008: [u8; 0x14],
    /// field_+0x1c (confidence: high, kind: u16?, lanes: c-core,via:rage::pgDictionary<gtaDrawable>,via:rage::pgDictionary<rage::crAnimation>,via:rage::pgDictionary<rage::phBound>,via:rage::pgDictionary<rage::ptxEffectRule>,via:rage::pgDictionary<rage::ptxEmitRule>,via:rage::pgDictionary<rage::ptxRule>,via:rage::pgDictionary<rage::rmcDrawable> moved from siblings:rage::pgDictionary<gtaDrawable>,rage::pgDictionary<rage::crAnimation>,rage::pgDictionary<rage::phBound>,rage::pgDictionary<rage::ptxEffectRule>,rage::pgDictionary<rage::ptxEmitRule>,rage::pgDictionary<rage::ptxRule>,rage::pgDictionary<rage::rmcDrawable>).
    pub field_0x1c: u16,
    /// Unknown bytes (0x1e..0x80).
    pub _pad_001e: [u8; 0x62],
    /// field_80 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:gtaDrawable,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_80: u32,
    /// Unknown bytes (0x84..0x90).
    pub _pad_0084: [u8; 0xc],
    /// field_90 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xa0).
    pub _pad_0094: [u8; 0xc],
    /// field_a0 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_a0: u32,
    /// Unknown bytes (0xa4..0xb0).
    pub _pad_00a4: [u8; 0xc],
    /// field_b0 (confidence: medium, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_b0: u32,
    /// field_b4 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_b4: u32,
    /// field_b8 (confidence: medium, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_b8: u32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: high, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_c0: u32,
    /// Unknown bytes (0xc4..0xcc).
    pub _pad_00c4: [u8; 0x8],
    /// field_cc (confidence: high, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_cc: u32,
    /// field_d0 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_d0: u32,
    /// field_d4 (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_d4: u32,
    /// field_d8 (confidence: medium, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_d8: u32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: high, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_e0: u32,
    /// field_e4 (confidence: high, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_e4: u32,
    /// field_e8 (confidence: high, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_e8: u32,
    /// field_ec (confidence: medium, kind: int?, lanes: c-physics,via:rage::fragDrawable,via:rage::fragType moved from siblings:rage::fragType,rage::rmcDrawableBase).
    pub field_ec: u32,
}
assert_size!(RagePgBase, 0xf0); // merged size 0xf0 rounded to 4
assert_offset!(RagePgBase, field_0x4, 0x4);
assert_offset!(RagePgBase, field_0x1c, 0x1c);
assert_offset!(RagePgBase, field_80, 0x80);
assert_offset!(RagePgBase, field_90, 0x90);
assert_offset!(RagePgBase, field_a0, 0xa0);
assert_offset!(RagePgBase, field_b0, 0xb0);
assert_offset!(RagePgBase, field_b4, 0xb4);
assert_offset!(RagePgBase, field_b8, 0xb8);
assert_offset!(RagePgBase, field_c0, 0xc0);
assert_offset!(RagePgBase, field_cc, 0xcc);
assert_offset!(RagePgBase, field_d0, 0xd0);
assert_offset!(RagePgBase, field_d4, 0xd4);
assert_offset!(RagePgBase, field_d8, 0xd8);
assert_offset!(RagePgBase, field_e0, 0xe0);
assert_offset!(RagePgBase, field_e4, 0xe4);
assert_offset!(RagePgBase, field_e8, 0xe8);
assert_offset!(RagePgBase, field_ec, 0xec);

/// Merged layout for `rage::pgBasicScheduler`.
///
/// Size: 0xc38 (low). Bases: none.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePgBasicScheduler {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// ptr_+0x4 (confidence: high, kind: pointer, lanes: c-core).
    pub ptr_0x4: Ptr32<u8>,
    /// field_+0x8 (confidence: high, kind: float, lanes: c-core).
    pub field_0x8: f32,
    /// field_+0xc (confidence: high, kind: u32?, lanes: c-core).
    pub field_0xc: u32,
    /// ptr_+0x10 (confidence: high, kind: pointer, lanes: c-core).
    pub ptr_0x10: Ptr32<u8>,
    /// field_+0x14 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x14: u32,
    /// field_+0x18 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x18: u32,
    /// field_+0x1c (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x1c: u32,
    /// field_+0x20 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x20: u32,
    /// field_+0x24 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x24: u32,
    /// field_+0x28 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x28: u32,
    /// Unknown bytes (0x2c..0xc30).
    pub _pad_002c: [u8; 0xc04],
    /// field_+0xc30 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0xc30: u32,
    /// ptr_+0xc34 (confidence: high, kind: pointer, lanes: c-core).
    pub ptr_0xc34: Ptr32<u8>,
}
assert_size!(RagePgBasicScheduler, 0xc38); // merged size 0xc38 rounded to 4
assert_offset!(RagePgBasicScheduler, vfptr, 0x0);
assert_offset!(RagePgBasicScheduler, ptr_0x4, 0x4);
assert_offset!(RagePgBasicScheduler, field_0x8, 0x8);
assert_offset!(RagePgBasicScheduler, field_0xc, 0xc);
assert_offset!(RagePgBasicScheduler, ptr_0x10, 0x10);
assert_offset!(RagePgBasicScheduler, field_0x14, 0x14);
assert_offset!(RagePgBasicScheduler, field_0x18, 0x18);
assert_offset!(RagePgBasicScheduler, field_0x1c, 0x1c);
assert_offset!(RagePgBasicScheduler, field_0x20, 0x20);
assert_offset!(RagePgBasicScheduler, field_0x24, 0x24);
assert_offset!(RagePgBasicScheduler, field_0x28, 0x28);
assert_offset!(RagePgBasicScheduler, field_0xc30, 0xc30);
assert_offset!(RagePgBasicScheduler, ptr_0xc34, 0xc34);

/// Merged layout for `rage::pgStreamableRef<rage::fragType>`.
///
/// Size: 0x10 (low). Bases: rage::pgStreamableRefBase@0x0.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePgStreamableRefRageFragType {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// field_+0x4 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x4: u32,
    /// field_+0x8 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x8: u32,
    /// ptr_+0xc (confidence: high, kind: pointer, lanes: c-core).
    pub ptr_0xc: Ptr32<u8>,
}
assert_size!(RagePgStreamableRefRageFragType, 0x10); // merged size 0x10 rounded to 4
assert_offset!(RagePgStreamableRefRageFragType, vfptr, 0x0);
assert_offset!(RagePgStreamableRefRageFragType, field_0x4, 0x4);
assert_offset!(RagePgStreamableRefRageFragType, field_0x8, 0x8);
assert_offset!(RagePgStreamableRefRageFragType, ptr_0xc, 0xc);
