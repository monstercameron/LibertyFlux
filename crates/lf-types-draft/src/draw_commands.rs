//! Draw commands: every class whose stated base is `CBaseDC`.
//!
//! Holds 44 draft layouts: The `CDraw*DC`, `CHud_Render*DC` and render-target commands, the generic
//! callback commands `T_CB_Generic_*`, and `T_SetShaderGroupVar_1Arg<rage::Vector4>`. Grouped by
//! the base class named on each type's `Bases:` line, not by name. Every layout is Inferred; size
//! confidence (the analysis lanes' own rating) is high for 8, medium for 17 and low for 19. The
//! conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CClearRenderTargetDC`.
///
/// Size: 0x18 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CClearRenderTargetDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_14: u8,
    /// field_15 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_15: u8,
    /// field_16 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_16: u8,
    /// Unknown trailing bytes (0x17..0x18).
    pub _pad_end: [u8; 0x1],
}
assert_size!(CClearRenderTargetDC, 0x18); // merged size 0x18 rounded to 4
assert_offset!(CClearRenderTargetDC, field_8, 0x8);
assert_offset!(CClearRenderTargetDC, field_c, 0xc);
assert_offset!(CClearRenderTargetDC, field_10, 0x10);
assert_offset!(CClearRenderTargetDC, field_14, 0x14);
assert_offset!(CClearRenderTargetDC, field_15, 0x15);
assert_offset!(CClearRenderTargetDC, field_16, 0x16);

/// Merged layout for `CCreateRenderListGroupDC`.
///
/// Size: 0x18 (low). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCreateRenderListGroupDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
}
assert_size!(CCreateRenderListGroupDC, 0x18); // merged size 0x18 rounded to 4
assert_offset!(CCreateRenderListGroupDC, field_8, 0x8);
assert_offset!(CCreateRenderListGroupDC, field_c, 0xc);
assert_offset!(CCreateRenderListGroupDC, field_10, 0x10);
assert_offset!(CCreateRenderListGroupDC, field_14, 0x14);

/// Merged layout for `CDrawCurvedWindowDC`.
///
/// Size: 0x1c (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawCurvedWindowDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
}
assert_size!(CDrawCurvedWindowDC, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(CDrawCurvedWindowDC, field_8, 0x8);
assert_offset!(CDrawCurvedWindowDC, field_c, 0xc);
assert_offset!(CDrawCurvedWindowDC, field_10, 0x10);
assert_offset!(CDrawCurvedWindowDC, field_14, 0x14);
assert_offset!(CDrawCurvedWindowDC, field_18, 0x18);

/// Merged layout for `CDrawDefLight`.
///
/// Size: 0x20 (low). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawDefLight {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
}
assert_size!(CDrawDefLight, 0x20); // merged size 0x20 rounded to 4
assert_offset!(CDrawDefLight, field_8, 0x8);
assert_offset!(CDrawDefLight, field_c, 0xc);
assert_offset!(CDrawDefLight, field_10, 0x10);
assert_offset!(CDrawDefLight, field_14, 0x14);
assert_offset!(CDrawDefLight, field_18, 0x18);
assert_offset!(CDrawDefLight, field_1c, 0x1c);

/// Merged layout for `CDrawEntityDC`.
///
/// Size: 0x40 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawEntityDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: int16, lanes: c-render).
    pub field_30: u16,
    /// field_32 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_32: u8,
    /// field_33 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_33: u8,
    /// field_34 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_34: u8,
    /// field_35 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_35: u8,
    /// field_36 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_36: u8,
    /// Unknown trailing bytes (0x37..0x40).
    pub _pad_end: [u8; 0x9],
}
assert_size!(CDrawEntityDC, 0x40); // merged size 0x40 rounded to 4
assert_offset!(CDrawEntityDC, field_10, 0x10);
assert_offset!(CDrawEntityDC, field_14, 0x14);
assert_offset!(CDrawEntityDC, field_18, 0x18);
assert_offset!(CDrawEntityDC, field_1c, 0x1c);
assert_offset!(CDrawEntityDC, field_20, 0x20);
assert_offset!(CDrawEntityDC, field_24, 0x24);
assert_offset!(CDrawEntityDC, field_28, 0x28);
assert_offset!(CDrawEntityDC, field_2c, 0x2c);
assert_offset!(CDrawEntityDC, field_30, 0x30);
assert_offset!(CDrawEntityDC, field_32, 0x32);
assert_offset!(CDrawEntityDC, field_33, 0x33);
assert_offset!(CDrawEntityDC, field_34, 0x34);
assert_offset!(CDrawEntityDC, field_35, 0x35);
assert_offset!(CDrawEntityDC, field_36, 0x36);

/// Merged layout for `CDrawFragDC`.
///
/// Size: 0x60 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawFragDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: medium, kind: int16, lanes: c-render).
    pub field_44: u16,
    /// field_46 (confidence: medium, kind: int16, lanes: c-render).
    pub field_46: u16,
    /// field_48 (confidence: low, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// Unknown bytes (0x4c..0x54).
    pub _pad_004c: [u8; 0x8],
    /// field_54 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_54: u8,
    /// field_55 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_55: u8,
    /// field_56 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_56: u8,
    /// field_57 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_57: u8,
    /// field_58 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_58: u8,
    /// field_59 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_59: u8,
    /// Unknown trailing bytes (0x5a..0x60).
    pub _pad_end: [u8; 0x6],
}
assert_size!(CDrawFragDC, 0x60); // merged size 0x60 rounded to 4
assert_offset!(CDrawFragDC, field_10, 0x10);
assert_offset!(CDrawFragDC, field_14, 0x14);
assert_offset!(CDrawFragDC, field_18, 0x18);
assert_offset!(CDrawFragDC, field_1c, 0x1c);
assert_offset!(CDrawFragDC, field_20, 0x20);
assert_offset!(CDrawFragDC, field_24, 0x24);
assert_offset!(CDrawFragDC, field_28, 0x28);
assert_offset!(CDrawFragDC, field_2c, 0x2c);
assert_offset!(CDrawFragDC, field_30, 0x30);
assert_offset!(CDrawFragDC, field_34, 0x34);
assert_offset!(CDrawFragDC, field_38, 0x38);
assert_offset!(CDrawFragDC, field_3c, 0x3c);
assert_offset!(CDrawFragDC, field_40, 0x40);
assert_offset!(CDrawFragDC, field_44, 0x44);
assert_offset!(CDrawFragDC, field_46, 0x46);
assert_offset!(CDrawFragDC, field_48, 0x48);
assert_offset!(CDrawFragDC, field_54, 0x54);
assert_offset!(CDrawFragDC, field_55, 0x55);
assert_offset!(CDrawFragDC, field_56, 0x56);
assert_offset!(CDrawFragDC, field_57, 0x57);
assert_offset!(CDrawFragDC, field_58, 0x58);
assert_offset!(CDrawFragDC, field_59, 0x59);

/// Merged layout for `CDrawFragTypeDC`.
///
/// Size: 0x60 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawFragTypeDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: low, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// field_58 (confidence: medium, kind: int16, lanes: c-render).
    pub field_58: u16,
    /// field_5a (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_5a: u8,
    /// field_5b (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_5b: u8,
    /// field_5c (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_5c: u8,
    /// Unknown trailing bytes (0x5d..0x60).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CDrawFragTypeDC, 0x60); // merged size 0x60 rounded to 4
assert_offset!(CDrawFragTypeDC, field_10, 0x10);
assert_offset!(CDrawFragTypeDC, field_14, 0x14);
assert_offset!(CDrawFragTypeDC, field_18, 0x18);
assert_offset!(CDrawFragTypeDC, field_20, 0x20);
assert_offset!(CDrawFragTypeDC, field_24, 0x24);
assert_offset!(CDrawFragTypeDC, field_28, 0x28);
assert_offset!(CDrawFragTypeDC, field_30, 0x30);
assert_offset!(CDrawFragTypeDC, field_34, 0x34);
assert_offset!(CDrawFragTypeDC, field_38, 0x38);
assert_offset!(CDrawFragTypeDC, field_40, 0x40);
assert_offset!(CDrawFragTypeDC, field_44, 0x44);
assert_offset!(CDrawFragTypeDC, field_48, 0x48);
assert_offset!(CDrawFragTypeDC, field_50, 0x50);
assert_offset!(CDrawFragTypeDC, field_54, 0x54);
assert_offset!(CDrawFragTypeDC, field_58, 0x58);
assert_offset!(CDrawFragTypeDC, field_5a, 0x5a);
assert_offset!(CDrawFragTypeDC, field_5b, 0x5b);
assert_offset!(CDrawFragTypeDC, field_5c, 0x5c);

/// Merged layout for `CDrawMobilePhoneCameraDC`.
///
/// Size: 0x420 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawMobilePhoneCameraDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// field_10 (confidence: high, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0x400).
    pub _pad_0014: [u8; 0x3ec],
    /// field_400 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_400: Ptr32<u8>,
    /// field_404 (confidence: low, kind: pointer, lanes: c-render).
    pub field_404: Ptr32<u8>,
    /// field_408 (confidence: low, kind: pointer, lanes: c-render).
    pub field_408: Ptr32<u8>,
    /// field_40c (confidence: low, kind: pointer, lanes: c-render).
    pub field_40c: Ptr32<u8>,
    /// field_410 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_410: Ptr32<u8>,
    /// field_414 (confidence: low, kind: pointer, lanes: c-render).
    pub field_414: Ptr32<u8>,
    /// field_418 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_418: Ptr32<u8>,
    /// field_41c (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_41c: u8,
    /// field_41d (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_41d: u8,
    /// Unknown trailing bytes (0x41e..0x420).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CDrawMobilePhoneCameraDC, 0x420); // merged size 0x420 rounded to 4
assert_offset!(CDrawMobilePhoneCameraDC, field_8, 0x8);
assert_offset!(CDrawMobilePhoneCameraDC, field_10, 0x10);
assert_offset!(CDrawMobilePhoneCameraDC, field_400, 0x400);
assert_offset!(CDrawMobilePhoneCameraDC, field_404, 0x404);
assert_offset!(CDrawMobilePhoneCameraDC, field_408, 0x408);
assert_offset!(CDrawMobilePhoneCameraDC, field_40c, 0x40c);
assert_offset!(CDrawMobilePhoneCameraDC, field_410, 0x410);
assert_offset!(CDrawMobilePhoneCameraDC, field_414, 0x414);
assert_offset!(CDrawMobilePhoneCameraDC, field_418, 0x418);
assert_offset!(CDrawMobilePhoneCameraDC, field_41c, 0x41c);
assert_offset!(CDrawMobilePhoneCameraDC, field_41d, 0x41d);

/// Merged layout for `CDrawPedDC`.
///
/// Size: 0xa0 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPedDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: high, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// Unknown bytes (0xc..0x7c).
    pub _pad_000c: [u8; 0x70],
    /// field_7c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_7c: Ptr32<u8>,
    /// field_80 (confidence: low, kind: pointer, lanes: c-render).
    pub field_80: Ptr32<u8>,
    /// field_84 (confidence: low, kind: pointer, lanes: c-render).
    pub field_84: Ptr32<u8>,
    /// field_88 (confidence: low, kind: pointer, lanes: c-render).
    pub field_88: Ptr32<u8>,
    /// field_8c (confidence: low, kind: pointer, lanes: c-render).
    pub field_8c: Ptr32<u8>,
    /// field_90 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: medium, kind: int16, lanes: c-render).
    pub field_94: u16,
    /// field_96 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_96: u8,
    /// field_97 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_97: u8,
    /// field_98 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_98: u8,
    /// field_99 (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_99: u8,
    /// field_9a (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_9a: u8,
    /// field_9b (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_9b: u8,
    /// field_9c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_9c: Ptr32<u8>,
}
assert_size!(CDrawPedDC, 0xa0); // merged size 0xa0 rounded to 4
assert_offset!(CDrawPedDC, field_8, 0x8);
assert_offset!(CDrawPedDC, field_7c, 0x7c);
assert_offset!(CDrawPedDC, field_80, 0x80);
assert_offset!(CDrawPedDC, field_84, 0x84);
assert_offset!(CDrawPedDC, field_88, 0x88);
assert_offset!(CDrawPedDC, field_8c, 0x8c);
assert_offset!(CDrawPedDC, field_90, 0x90);
assert_offset!(CDrawPedDC, field_94, 0x94);
assert_offset!(CDrawPedDC, field_96, 0x96);
assert_offset!(CDrawPedDC, field_97, 0x97);
assert_offset!(CDrawPedDC, field_98, 0x98);
assert_offset!(CDrawPedDC, field_99, 0x99);
assert_offset!(CDrawPedDC, field_9a, 0x9a);
assert_offset!(CDrawPedDC, field_9b, 0x9b);
assert_offset!(CDrawPedDC, field_9c, 0x9c);

/// Merged layout for `CDrawPedPropDC`.
///
/// Size: 0x100 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPedPropDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0xa0).
    pub _pad_0014: [u8; 0x8c],
    /// field_a0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_a0: Ptr32<u8>,
    /// field_a4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a4: Ptr32<u8>,
    /// field_a8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_a8: Ptr32<u8>,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// field_b0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_b4: Ptr32<u8>,
    /// field_b8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_b8: Ptr32<u8>,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c0: Ptr32<u8>,
    /// field_c4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c4: Ptr32<u8>,
    /// field_c8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_c8: Ptr32<u8>,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d0: Ptr32<u8>,
    /// field_d4 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d4: Ptr32<u8>,
    /// field_d8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_d8: Ptr32<u8>,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_e0: Ptr32<u8>,
    /// field_e4 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_e4: Ptr32<u8>,
    /// field_e8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_e8: Ptr32<u8>,
    /// field_ec (confidence: medium, kind: int16, lanes: c-render).
    pub field_ec: u16,
    /// field_ee (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_ee: u8,
    /// field_ef (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_ef: u8,
    /// field_f0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_f0: Ptr32<u8>,
    /// field_f4 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_f4: u8,
    /// Unknown trailing bytes (0xf5..0x100).
    pub _pad_end: [u8; 0xb],
}
assert_size!(CDrawPedPropDC, 0x100); // merged size 0x100 rounded to 4
assert_offset!(CDrawPedPropDC, field_10, 0x10);
assert_offset!(CDrawPedPropDC, field_a0, 0xa0);
assert_offset!(CDrawPedPropDC, field_a4, 0xa4);
assert_offset!(CDrawPedPropDC, field_a8, 0xa8);
assert_offset!(CDrawPedPropDC, field_b0, 0xb0);
assert_offset!(CDrawPedPropDC, field_b4, 0xb4);
assert_offset!(CDrawPedPropDC, field_b8, 0xb8);
assert_offset!(CDrawPedPropDC, field_c0, 0xc0);
assert_offset!(CDrawPedPropDC, field_c4, 0xc4);
assert_offset!(CDrawPedPropDC, field_c8, 0xc8);
assert_offset!(CDrawPedPropDC, field_d0, 0xd0);
assert_offset!(CDrawPedPropDC, field_d4, 0xd4);
assert_offset!(CDrawPedPropDC, field_d8, 0xd8);
assert_offset!(CDrawPedPropDC, field_e0, 0xe0);
assert_offset!(CDrawPedPropDC, field_e4, 0xe4);
assert_offset!(CDrawPedPropDC, field_e8, 0xe8);
assert_offset!(CDrawPedPropDC, field_ec, 0xec);
assert_offset!(CDrawPedPropDC, field_ee, 0xee);
assert_offset!(CDrawPedPropDC, field_ef, 0xef);
assert_offset!(CDrawPedPropDC, field_f0, 0xf0);
assert_offset!(CDrawPedPropDC, field_f4, 0xf4);

/// Merged layout for `CDrawPedPropsDC`.
///
/// Size: 0xc0 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPedPropsDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0xa0).
    pub _pad_0014: [u8; 0x8c],
    /// field_a0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_a0: Ptr32<u8>,
    /// Unknown bytes (0xa4..0xa8).
    pub _pad_00a4: [u8; 0x4],
    /// field_a8 (confidence: medium, kind: int16, lanes: c-render).
    pub field_a8: u16,
    /// field_aa (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_aa: u8,
    /// field_ab (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_ab: u8,
    /// field_ac (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_ac: u8,
    /// field_ad (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_ad: u8,
    /// field_ae (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_ae: u8,
    /// Unknown bytes (0xaf..0xb0).
    pub _pad_00af: [u8; 0x1],
    /// field_b0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_b0: Ptr32<u8>,
    /// field_b4 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_b4: u8,
    /// Unknown trailing bytes (0xb5..0xc0).
    pub _pad_end: [u8; 0xb],
}
assert_size!(CDrawPedPropsDC, 0xc0); // merged size 0xc0 rounded to 4
assert_offset!(CDrawPedPropsDC, field_10, 0x10);
assert_offset!(CDrawPedPropsDC, field_a0, 0xa0);
assert_offset!(CDrawPedPropsDC, field_a8, 0xa8);
assert_offset!(CDrawPedPropsDC, field_aa, 0xaa);
assert_offset!(CDrawPedPropsDC, field_ab, 0xab);
assert_offset!(CDrawPedPropsDC, field_ac, 0xac);
assert_offset!(CDrawPedPropsDC, field_ad, 0xad);
assert_offset!(CDrawPedPropsDC, field_ae, 0xae);
assert_offset!(CDrawPedPropsDC, field_b0, 0xb0);
assert_offset!(CDrawPedPropsDC, field_b4, 0xb4);

/// Merged layout for `CDrawPhoneDC_NY`.
///
/// Size: 0x40 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPhoneDCNY {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: int16, lanes: c-render).
    pub field_8: u16,
    /// Unknown bytes (0xa..0x10).
    pub _pad_000a: [u8; 0x6],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0x30).
    pub _pad_0024: [u8; 0xc],
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
}
assert_size!(CDrawPhoneDCNY, 0x40); // merged size 0x40 rounded to 4
assert_offset!(CDrawPhoneDCNY, field_8, 0x8);
assert_offset!(CDrawPhoneDCNY, field_10, 0x10);
assert_offset!(CDrawPhoneDCNY, field_14, 0x14);
assert_offset!(CDrawPhoneDCNY, field_18, 0x18);
assert_offset!(CDrawPhoneDCNY, field_1c, 0x1c);
assert_offset!(CDrawPhoneDCNY, field_20, 0x20);
assert_offset!(CDrawPhoneDCNY, field_30, 0x30);
assert_offset!(CDrawPhoneDCNY, field_34, 0x34);
assert_offset!(CDrawPhoneDCNY, field_38, 0x38);
assert_offset!(CDrawPhoneDCNY, field_3c, 0x3c);

/// Merged layout for `CDrawPlayerDC`.
///
/// Size: 0x3b0 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPlayerDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: int16, lanes: c-render).
    pub field_8: u16,
    /// field_a (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_a: u8,
    /// field_b (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_b: u8,
    /// field_c (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_c: u8,
    /// Unknown bytes (0xd..0x10).
    pub _pad_000d: [u8; 0x3],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// Unknown bytes (0x18..0x20).
    pub _pad_0018: [u8; 0x8],
    /// field_20 (confidence: high, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// Unknown bytes (0x24..0x390).
    pub _pad_0024: [u8; 0x36c],
    /// field_390 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_390: Ptr32<u8>,
    /// Unknown bytes (0x394..0x398).
    pub _pad_0394: [u8; 0x4],
    /// field_398 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_398: Ptr32<u8>,
    /// field_39c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_39c: Ptr32<u8>,
    /// field_3a0 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_3a0: Ptr32<u8>,
    /// Unknown trailing bytes (0x3a4..0x3b0).
    pub _pad_end: [u8; 0xc],
}
assert_size!(CDrawPlayerDC, 0x3b0); // merged size 0x3b0 rounded to 4
assert_offset!(CDrawPlayerDC, field_8, 0x8);
assert_offset!(CDrawPlayerDC, field_a, 0xa);
assert_offset!(CDrawPlayerDC, field_b, 0xb);
assert_offset!(CDrawPlayerDC, field_c, 0xc);
assert_offset!(CDrawPlayerDC, field_10, 0x10);
assert_offset!(CDrawPlayerDC, field_14, 0x14);
assert_offset!(CDrawPlayerDC, field_20, 0x20);
assert_offset!(CDrawPlayerDC, field_390, 0x390);
assert_offset!(CDrawPlayerDC, field_398, 0x398);
assert_offset!(CDrawPlayerDC, field_39c, 0x39c);
assert_offset!(CDrawPlayerDC, field_3a0, 0x3a0);

/// Merged layout for `CDrawPolyLoadingClockDC`.
///
/// Size: 0x3c (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawPolyLoadingClockDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
}
assert_size!(CDrawPolyLoadingClockDC, 0x3c); // merged size 0x3c rounded to 4
assert_offset!(CDrawPolyLoadingClockDC, field_8, 0x8);
assert_offset!(CDrawPolyLoadingClockDC, field_c, 0xc);
assert_offset!(CDrawPolyLoadingClockDC, field_10, 0x10);
assert_offset!(CDrawPolyLoadingClockDC, field_14, 0x14);
assert_offset!(CDrawPolyLoadingClockDC, field_18, 0x18);
assert_offset!(CDrawPolyLoadingClockDC, field_1c, 0x1c);
assert_offset!(CDrawPolyLoadingClockDC, field_20, 0x20);
assert_offset!(CDrawPolyLoadingClockDC, field_24, 0x24);
assert_offset!(CDrawPolyLoadingClockDC, field_28, 0x28);
assert_offset!(CDrawPolyLoadingClockDC, field_2c, 0x2c);
assert_offset!(CDrawPolyLoadingClockDC, field_30, 0x30);
assert_offset!(CDrawPolyLoadingClockDC, field_34, 0x34);
assert_offset!(CDrawPolyLoadingClockDC, field_38, 0x38);

/// Merged layout for `CDrawRadarCircleDC`.
///
/// Size: 0x1c (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawRadarCircleDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
}
assert_size!(CDrawRadarCircleDC, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(CDrawRadarCircleDC, field_8, 0x8);
assert_offset!(CDrawRadarCircleDC, field_c, 0xc);
assert_offset!(CDrawRadarCircleDC, field_10, 0x10);
assert_offset!(CDrawRadarCircleDC, field_14, 0x14);
assert_offset!(CDrawRadarCircleDC, field_18, 0x18);

/// Merged layout for `CDrawRadarMapSectionDC`.
///
/// Size: 0x80 (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawRadarMapSectionDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: low, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: low, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: low, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: low, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: low, kind: pointer, lanes: c-render).
    pub field_4c: Ptr32<u8>,
    /// field_50 (confidence: low, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: low, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// field_58 (confidence: low, kind: pointer, lanes: c-render).
    pub field_58: Ptr32<u8>,
    /// field_5c (confidence: low, kind: pointer, lanes: c-render).
    pub field_5c: Ptr32<u8>,
    /// field_60 (confidence: low, kind: pointer, lanes: c-render).
    pub field_60: Ptr32<u8>,
    /// field_64 (confidence: low, kind: pointer, lanes: c-render).
    pub field_64: Ptr32<u8>,
    /// field_68 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_68: Ptr32<u8>,
    /// field_6c (confidence: low, kind: pointer, lanes: c-render).
    pub field_6c: Ptr32<u8>,
    /// field_70 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_74: Ptr32<u8>,
    /// Unknown trailing bytes (0x78..0x80).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CDrawRadarMapSectionDC, 0x80); // merged size 0x80 rounded to 4
assert_offset!(CDrawRadarMapSectionDC, field_10, 0x10);
assert_offset!(CDrawRadarMapSectionDC, field_14, 0x14);
assert_offset!(CDrawRadarMapSectionDC, field_18, 0x18);
assert_offset!(CDrawRadarMapSectionDC, field_1c, 0x1c);
assert_offset!(CDrawRadarMapSectionDC, field_20, 0x20);
assert_offset!(CDrawRadarMapSectionDC, field_24, 0x24);
assert_offset!(CDrawRadarMapSectionDC, field_28, 0x28);
assert_offset!(CDrawRadarMapSectionDC, field_2c, 0x2c);
assert_offset!(CDrawRadarMapSectionDC, field_30, 0x30);
assert_offset!(CDrawRadarMapSectionDC, field_34, 0x34);
assert_offset!(CDrawRadarMapSectionDC, field_38, 0x38);
assert_offset!(CDrawRadarMapSectionDC, field_3c, 0x3c);
assert_offset!(CDrawRadarMapSectionDC, field_40, 0x40);
assert_offset!(CDrawRadarMapSectionDC, field_44, 0x44);
assert_offset!(CDrawRadarMapSectionDC, field_48, 0x48);
assert_offset!(CDrawRadarMapSectionDC, field_4c, 0x4c);
assert_offset!(CDrawRadarMapSectionDC, field_50, 0x50);
assert_offset!(CDrawRadarMapSectionDC, field_54, 0x54);
assert_offset!(CDrawRadarMapSectionDC, field_58, 0x58);
assert_offset!(CDrawRadarMapSectionDC, field_5c, 0x5c);
assert_offset!(CDrawRadarMapSectionDC, field_60, 0x60);
assert_offset!(CDrawRadarMapSectionDC, field_64, 0x64);
assert_offset!(CDrawRadarMapSectionDC, field_68, 0x68);
assert_offset!(CDrawRadarMapSectionDC, field_6c, 0x6c);
assert_offset!(CDrawRadarMapSectionDC, field_70, 0x70);
assert_offset!(CDrawRadarMapSectionDC, field_74, 0x74);

/// Merged layout for `CDrawRadioHudTextDC`.
///
/// Size: 0x30 (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawRadioHudTextDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: low, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
}
assert_size!(CDrawRadioHudTextDC, 0x30); // merged size 0x30 rounded to 4
assert_offset!(CDrawRadioHudTextDC, field_8, 0x8);
assert_offset!(CDrawRadioHudTextDC, field_c, 0xc);
assert_offset!(CDrawRadioHudTextDC, field_10, 0x10);
assert_offset!(CDrawRadioHudTextDC, field_14, 0x14);
assert_offset!(CDrawRadioHudTextDC, field_18, 0x18);
assert_offset!(CDrawRadioHudTextDC, field_1c, 0x1c);
assert_offset!(CDrawRadioHudTextDC, field_20, 0x20);
assert_offset!(CDrawRadioHudTextDC, field_24, 0x24);
assert_offset!(CDrawRadioHudTextDC, field_28, 0x28);
assert_offset!(CDrawRadioHudTextDC, field_2c, 0x2c);

/// Merged layout for `CDrawSkinnedEntityDC`.
///
/// Size: 0x20 (low). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawSkinnedEntityDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: int16, lanes: c-render).
    pub field_8: u16,
    /// field_a (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_a: u8,
    /// field_b (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_b: u8,
    /// field_c (confidence: low, kind: bool-or-byte, lanes: c-render).
    pub field_c: u8,
    /// Unknown bytes (0xd..0x10).
    pub _pad_000d: [u8; 0x3],
    /// field_10 (confidence: low, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
}
assert_size!(CDrawSkinnedEntityDC, 0x20); // merged size 0x20 rounded to 4
assert_offset!(CDrawSkinnedEntityDC, field_8, 0x8);
assert_offset!(CDrawSkinnedEntityDC, field_a, 0xa);
assert_offset!(CDrawSkinnedEntityDC, field_b, 0xb);
assert_offset!(CDrawSkinnedEntityDC, field_c, 0xc);
assert_offset!(CDrawSkinnedEntityDC, field_10, 0x10);
assert_offset!(CDrawSkinnedEntityDC, field_14, 0x14);
assert_offset!(CDrawSkinnedEntityDC, field_18, 0x18);
assert_offset!(CDrawSkinnedEntityDC, field_1c, 0x1c);

/// Merged layout for `CDrawSpriteDC`.
///
/// Size: 0x30 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawSpriteDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
}
assert_size!(CDrawSpriteDC, 0x30); // merged size 0x30 rounded to 4
assert_offset!(CDrawSpriteDC, field_8, 0x8);
assert_offset!(CDrawSpriteDC, field_c, 0xc);
assert_offset!(CDrawSpriteDC, field_10, 0x10);
assert_offset!(CDrawSpriteDC, field_14, 0x14);
assert_offset!(CDrawSpriteDC, field_18, 0x18);
assert_offset!(CDrawSpriteDC, field_1c, 0x1c);
assert_offset!(CDrawSpriteDC, field_20, 0x20);
assert_offset!(CDrawSpriteDC, field_24, 0x24);
assert_offset!(CDrawSpriteDC, field_28, 0x28);
assert_offset!(CDrawSpriteDC, field_2c, 0x2c);

/// Merged layout for `CDrawSpriteInPerspectiveDC`.
///
/// Size: 0x60 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawSpriteInPerspectiveDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: low, kind: pointer, lanes: c-render).
    pub field_4c: Ptr32<u8>,
    /// field_50 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// Unknown trailing bytes (0x58..0x60).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CDrawSpriteInPerspectiveDC, 0x60); // merged size 0x60 rounded to 4
assert_offset!(CDrawSpriteInPerspectiveDC, field_10, 0x10);
assert_offset!(CDrawSpriteInPerspectiveDC, field_14, 0x14);
assert_offset!(CDrawSpriteInPerspectiveDC, field_18, 0x18);
assert_offset!(CDrawSpriteInPerspectiveDC, field_1c, 0x1c);
assert_offset!(CDrawSpriteInPerspectiveDC, field_20, 0x20);
assert_offset!(CDrawSpriteInPerspectiveDC, field_24, 0x24);
assert_offset!(CDrawSpriteInPerspectiveDC, field_28, 0x28);
assert_offset!(CDrawSpriteInPerspectiveDC, field_2c, 0x2c);
assert_offset!(CDrawSpriteInPerspectiveDC, field_30, 0x30);
assert_offset!(CDrawSpriteInPerspectiveDC, field_34, 0x34);
assert_offset!(CDrawSpriteInPerspectiveDC, field_38, 0x38);
assert_offset!(CDrawSpriteInPerspectiveDC, field_3c, 0x3c);
assert_offset!(CDrawSpriteInPerspectiveDC, field_40, 0x40);
assert_offset!(CDrawSpriteInPerspectiveDC, field_44, 0x44);
assert_offset!(CDrawSpriteInPerspectiveDC, field_48, 0x48);
assert_offset!(CDrawSpriteInPerspectiveDC, field_4c, 0x4c);
assert_offset!(CDrawSpriteInPerspectiveDC, field_50, 0x50);
assert_offset!(CDrawSpriteInPerspectiveDC, field_54, 0x54);

/// Merged layout for `CDrawSpritePerspDC`.
///
/// Size: 0x60 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawSpritePerspDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: low, kind: pointer, lanes: c-render).
    pub field_4c: Ptr32<u8>,
    /// field_50 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_54: Ptr32<u8>,
    /// Unknown trailing bytes (0x58..0x60).
    pub _pad_end: [u8; 0x8],
}
assert_size!(CDrawSpritePerspDC, 0x60); // merged size 0x60 rounded to 4
assert_offset!(CDrawSpritePerspDC, field_10, 0x10);
assert_offset!(CDrawSpritePerspDC, field_14, 0x14);
assert_offset!(CDrawSpritePerspDC, field_18, 0x18);
assert_offset!(CDrawSpritePerspDC, field_1c, 0x1c);
assert_offset!(CDrawSpritePerspDC, field_20, 0x20);
assert_offset!(CDrawSpritePerspDC, field_24, 0x24);
assert_offset!(CDrawSpritePerspDC, field_28, 0x28);
assert_offset!(CDrawSpritePerspDC, field_2c, 0x2c);
assert_offset!(CDrawSpritePerspDC, field_30, 0x30);
assert_offset!(CDrawSpritePerspDC, field_34, 0x34);
assert_offset!(CDrawSpritePerspDC, field_38, 0x38);
assert_offset!(CDrawSpritePerspDC, field_3c, 0x3c);
assert_offset!(CDrawSpritePerspDC, field_40, 0x40);
assert_offset!(CDrawSpritePerspDC, field_44, 0x44);
assert_offset!(CDrawSpritePerspDC, field_48, 0x48);
assert_offset!(CDrawSpritePerspDC, field_4c, 0x4c);
assert_offset!(CDrawSpritePerspDC, field_50, 0x50);
assert_offset!(CDrawSpritePerspDC, field_54, 0x54);

/// Merged layout for `CDrawSpriteUVDC`.
///
/// Size: 0x50 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawSpriteUVDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-render).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: pointer, lanes: c-render).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-render).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_4c: Ptr32<u8>,
}
assert_size!(CDrawSpriteUVDC, 0x50); // merged size 0x50 rounded to 4
assert_offset!(CDrawSpriteUVDC, field_8, 0x8);
assert_offset!(CDrawSpriteUVDC, field_c, 0xc);
assert_offset!(CDrawSpriteUVDC, field_10, 0x10);
assert_offset!(CDrawSpriteUVDC, field_14, 0x14);
assert_offset!(CDrawSpriteUVDC, field_18, 0x18);
assert_offset!(CDrawSpriteUVDC, field_1c, 0x1c);
assert_offset!(CDrawSpriteUVDC, field_20, 0x20);
assert_offset!(CDrawSpriteUVDC, field_24, 0x24);
assert_offset!(CDrawSpriteUVDC, field_28, 0x28);
assert_offset!(CDrawSpriteUVDC, field_2c, 0x2c);
assert_offset!(CDrawSpriteUVDC, field_30, 0x30);
assert_offset!(CDrawSpriteUVDC, field_34, 0x34);
assert_offset!(CDrawSpriteUVDC, field_38, 0x38);
assert_offset!(CDrawSpriteUVDC, field_3c, 0x3c);
assert_offset!(CDrawSpriteUVDC, field_40, 0x40);
assert_offset!(CDrawSpriteUVDC, field_44, 0x44);
assert_offset!(CDrawSpriteUVDC, field_48, 0x48);
assert_offset!(CDrawSpriteUVDC, field_4c, 0x4c);

/// Merged layout for `CDrawTriShapeDC`.
///
/// Size: 0x78 (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDrawTriShapeDC {
    /// Unknown bytes (0x0..0x38).
    pub _pad_0000: [u8; 0x38],
    /// field_38 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_38: Ptr32<u8>,
    /// Unknown bytes (0x3c..0x68).
    pub _pad_003c: [u8; 0x2c],
    /// field_68 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_68: Ptr32<u8>,
    /// field_6c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_6c: Ptr32<u8>,
    /// field_70 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-render).
    pub field_74: Ptr32<u8>,
}
assert_size!(CDrawTriShapeDC, 0x78); // merged size 0x78 rounded to 4
assert_offset!(CDrawTriShapeDC, field_38, 0x38);
assert_offset!(CDrawTriShapeDC, field_68, 0x68);
assert_offset!(CDrawTriShapeDC, field_6c, 0x6c);
assert_offset!(CDrawTriShapeDC, field_70, 0x70);
assert_offset!(CDrawTriShapeDC, field_74, 0x74);

/// Merged layout for `CHud_RenderBarDC`.
///
/// Size: 0x24 (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHudRenderBarDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_1c: u8,
    /// Unknown bytes (0x1d..0x20).
    pub _pad_001d: [u8; 0x3],
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
}
assert_size!(CHudRenderBarDC, 0x24); // merged size 0x24 rounded to 4
assert_offset!(CHudRenderBarDC, field_8, 0x8);
assert_offset!(CHudRenderBarDC, field_c, 0xc);
assert_offset!(CHudRenderBarDC, field_10, 0x10);
assert_offset!(CHudRenderBarDC, field_14, 0x14);
assert_offset!(CHudRenderBarDC, field_18, 0x18);
assert_offset!(CHudRenderBarDC, field_1c, 0x1c);
assert_offset!(CHudRenderBarDC, field_20, 0x20);

/// Merged layout for `CHud_RenderSpriteDC`.
///
/// Size: 0x2c (medium). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHudRenderSpriteDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
}
assert_size!(CHudRenderSpriteDC, 0x2c); // merged size 0x2c rounded to 4
assert_offset!(CHudRenderSpriteDC, field_8, 0x8);
assert_offset!(CHudRenderSpriteDC, field_c, 0xc);
assert_offset!(CHudRenderSpriteDC, field_10, 0x10);
assert_offset!(CHudRenderSpriteDC, field_14, 0x14);
assert_offset!(CHudRenderSpriteDC, field_18, 0x18);
assert_offset!(CHudRenderSpriteDC, field_1c, 0x1c);
assert_offset!(CHudRenderSpriteDC, field_20, 0x20);
assert_offset!(CHudRenderSpriteDC, field_24, 0x24);
assert_offset!(CHudRenderSpriteDC, field_28, 0x28);

/// Merged layout for `CHud_RenderWindowDC`.
///
/// Size: 0x1c (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHudRenderWindowDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: low, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
}
assert_size!(CHudRenderWindowDC, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(CHudRenderWindowDC, field_8, 0x8);
assert_offset!(CHudRenderWindowDC, field_c, 0xc);
assert_offset!(CHudRenderWindowDC, field_10, 0x10);
assert_offset!(CHudRenderWindowDC, field_14, 0x14);
assert_offset!(CHudRenderWindowDC, field_18, 0x18);

/// Merged layout for `CUnLockRenderTargetDC`.
///
/// Size: 0x2c (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUnLockRenderTargetDC {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-render).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: double, lanes: c-render).
    pub field_14: [u8; 8],
    /// field_1c (confidence: low, kind: double, lanes: c-render).
    pub field_1c: [u8; 8],
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_28: u8,
    /// Unknown trailing bytes (0x29..0x2c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CUnLockRenderTargetDC, 0x2c); // merged size 0x2c rounded to 4
assert_offset!(CUnLockRenderTargetDC, field_8, 0x8);
assert_offset!(CUnLockRenderTargetDC, field_c, 0xc);
assert_offset!(CUnLockRenderTargetDC, field_10, 0x10);
assert_offset!(CUnLockRenderTargetDC, field_14, 0x14);
assert_offset!(CUnLockRenderTargetDC, field_1c, 0x1c);
assert_offset!(CUnLockRenderTargetDC, field_24, 0x24);
assert_offset!(CUnLockRenderTargetDC, field_28, 0x28);

/// Merged layout for `CUseLightsInAreaDC`.
///
/// Size: 0x40 (high). Bases: CBaseDC@0x0.
/// Lanes: c-render.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUseLightsInAreaDC {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-render).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-render).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: low, kind: pointer, lanes: c-render).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-render).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: pointer, lanes: c-render).
    pub field_28: Ptr32<u8>,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: medium, kind: pointer, lanes: c-render).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: medium, kind: bool-or-byte, lanes: c-render).
    pub field_38: u8,
    /// Unknown trailing bytes (0x39..0x40).
    pub _pad_end: [u8; 0x7],
}
assert_size!(CUseLightsInAreaDC, 0x40); // merged size 0x40 rounded to 4
assert_offset!(CUseLightsInAreaDC, field_10, 0x10);
assert_offset!(CUseLightsInAreaDC, field_14, 0x14);
assert_offset!(CUseLightsInAreaDC, field_18, 0x18);
assert_offset!(CUseLightsInAreaDC, field_20, 0x20);
assert_offset!(CUseLightsInAreaDC, field_24, 0x24);
assert_offset!(CUseLightsInAreaDC, field_28, 0x28);
assert_offset!(CUseLightsInAreaDC, field_30, 0x30);
assert_offset!(CUseLightsInAreaDC, field_34, 0x34);
assert_offset!(CUseLightsInAreaDC, field_38, 0x38);

/// Merged layout for `T_CB_Generic_1Arg<void(*)(bool), bool>`.
///
/// Size: 0x93c (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric1ArgVoidBoolBool {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_c: u8,
    /// Unknown bytes (0xd..0x20).
    pub _pad_000d: [u8; 0x13],
    /// field_20 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_20: u8,
    /// field_21 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_21: u8,
    /// Unknown bytes (0x22..0x44).
    pub _pad_0022: [u8; 0x22],
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// Unknown bytes (0x48..0xb0).
    pub _pad_0048: [u8; 0x68],
    /// field_b0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_b0: u32,
    /// Unknown bytes (0xb4..0x8e8).
    pub _pad_00b4: [u8; 0x834],
    /// field_8e8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_8e8: u32,
    /// Unknown bytes (0x8ec..0x938).
    pub _pad_08ec: [u8; 0x4c],
    /// field_938 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_938: u32,
}
assert_size!(TCBGeneric1ArgVoidBoolBool, 0x93c); // merged size 0x93c rounded to 4
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_8, 0x8);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_c, 0xc);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_20, 0x20);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_21, 0x21);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_44, 0x44);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_b0, 0xb0);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_8e8, 0x8e8);
assert_offset!(TCBGeneric1ArgVoidBoolBool, field_938, 0x938);

/// Merged layout for `T_CB_Generic_2Args<void(*)(float, float), float, float>`.
///
/// Size: 0x14 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric2ArgsVoidFloatFloatFloatFloat {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
}
assert_size!(TCBGeneric2ArgsVoidFloatFloatFloatFloat, 0x14); // merged size 0x14 rounded to 4
assert_offset!(TCBGeneric2ArgsVoidFloatFloatFloatFloat, field_8, 0x8);
assert_offset!(TCBGeneric2ArgsVoidFloatFloatFloatFloat, field_c, 0xc);
assert_offset!(TCBGeneric2ArgsVoidFloatFloatFloatFloat, field_10, 0x10);

/// Merged layout for `T_CB_Generic_2Args<void(*)(int, bool), int, bool>`.
///
/// Size: 0x11 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric2ArgsVoidIntBoolIntBool {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_10: u8,
    /// Unknown trailing bytes (0x11..0x14).
    pub _pad_end: [u8; 0x3],
}
assert_size!(TCBGeneric2ArgsVoidIntBoolIntBool, 0x14); // merged size 0x11 rounded to 4
assert_offset!(TCBGeneric2ArgsVoidIntBoolIntBool, field_8, 0x8);
assert_offset!(TCBGeneric2ArgsVoidIntBoolIntBool, field_c, 0xc);
assert_offset!(TCBGeneric2ArgsVoidIntBoolIntBool, field_10, 0x10);

/// Merged layout for `T_CB_Generic_2Args<void(*)(rage::grcColorWrite, unsigned int), rage::grcColorWrite, unsigned int>`.
///
/// Size: 0x14 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric2ArgsVoidRageGrcColorWriteUnsignedIntRageGrcColorWriteUnsignedInt {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
}
assert_size!(
    TCBGeneric2ArgsVoidRageGrcColorWriteUnsignedIntRageGrcColorWriteUnsignedInt,
    0x14
); // merged size 0x14 rounded to 4
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcColorWriteUnsignedIntRageGrcColorWriteUnsignedInt,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcColorWriteUnsignedIntRageGrcColorWriteUnsignedInt,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcColorWriteUnsignedIntRageGrcColorWriteUnsignedInt,
    field_10,
    0x10
);

/// Merged layout for `T_CB_Generic_2Args<void(*)(rage::grcRenderTarget*, int), rage::grcRenderTarget*, int>`.
///
/// Size: 0x14 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric2ArgsVoidRageGrcRenderTargetIntRageGrcRenderTargetInt {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
}
assert_size!(
    TCBGeneric2ArgsVoidRageGrcRenderTargetIntRageGrcRenderTargetInt,
    0x14
); // merged size 0x14 rounded to 4
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcRenderTargetIntRageGrcRenderTargetInt,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcRenderTargetIntRageGrcRenderTargetInt,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric2ArgsVoidRageGrcRenderTargetIntRageGrcRenderTargetInt,
    field_10,
    0x10
);

/// Merged layout for `T_CB_Generic_3Args<void(*)(float, float, float), float, float, float>`.
///
/// Size: 0x18 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
}
assert_size!(TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat, 0x18); // merged size 0x18 rounded to 4
assert_offset!(
    TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric3ArgsVoidFloatFloatFloatFloatFloatFloat,
    field_14,
    0x14
);

/// Merged layout for `T_CB_Generic_3Args<void(*)(int, const rage::Matrix34&, unsigned char), int, rage::Matrix34, unsigned char>`.
///
/// Size: 0x51 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_50: u8,
    /// Unknown trailing bytes (0x51..0x54).
    pub _pad_end: [u8; 0x3],
}
assert_size!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    0x54
); // merged size 0x51 rounded to 4
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_18,
    0x18
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_20,
    0x20
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_24,
    0x24
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_28,
    0x28
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_30,
    0x30
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_34,
    0x34
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_38,
    0x38
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_40,
    0x40
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_44,
    0x44
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_48,
    0x48
);
assert_offset!(
    TCBGeneric3ArgsVoidIntConstRageMatrix34UnsignedCharIntRageMatrix34UnsignedChar,
    field_50,
    0x50
);

/// Merged layout for `T_CB_Generic_3Args<void(*)(int, rage::Vector4&, rage::Matrix34&), int, rage::Vector4, rage::Matrix34>`.
///
/// Size: 0x5c (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34 {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_58: u32,
}
assert_size!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    0x5c
); // merged size 0x5c rounded to 4
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_18,
    0x18
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_1c,
    0x1c
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_20,
    0x20
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_24,
    0x24
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_28,
    0x28
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_30,
    0x30
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_34,
    0x34
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_38,
    0x38
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_40,
    0x40
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_44,
    0x44
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_48,
    0x48
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_50,
    0x50
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_54,
    0x54
);
assert_offset!(
    TCBGeneric3ArgsVoidIntRageVector4RageMatrix34IntRageVector4RageMatrix34,
    field_58,
    0x58
);

/// Merged layout for `T_CB_Generic_4Args<void(*)(int, int, bool, bool), int, int, bool, bool>`.
///
/// Size: 0x16 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_14: u8,
    /// field_15 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_15: u8,
    /// Unknown trailing bytes (0x16..0x18).
    pub _pad_end: [u8; 0x2],
}
assert_size!(TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool, 0x18); // merged size 0x16 rounded to 4
assert_offset!(
    TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric4ArgsVoidIntIntBoolBoolIntIntBoolBool,
    field_15,
    0x15
);

/// Merged layout for `T_CB_Generic_4Args<void(*)(rage::Vector4&, float, float, float), rage::Vector4, float, float, float>`.
///
/// Size: 0x2c (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
}
assert_size!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    0x2c
); // merged size 0x2c rounded to 4
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_18,
    0x18
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_1c,
    0x1c
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_20,
    0x20
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_24,
    0x24
);
assert_offset!(
    TCBGeneric4ArgsVoidRageVector4FloatFloatFloatRageVector4FloatFloatFloat,
    field_28,
    0x28
);

/// Merged layout for `T_CB_Generic_4Args<void(*)(rage::grcTexture*, CRect&, CRect&, rage::Color32&), rage::grcTexture*, CRect, CRect, rage::Color32>`.
///
/// Size: 0x34 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32
{
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: unknown, lanes: c-misc-b).
    pub field_18: [u8; 8],
    /// field_20 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: unknown, lanes: c-misc-b).
    pub field_28: [u8; 8],
    /// field_30 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_30: Ptr32<u8>,
}
assert_size!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    0x34
); // merged size 0x34 rounded to 4
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_18,
    0x18
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_20,
    0x20
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_24,
    0x24
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_28,
    0x28
);
assert_offset!(
    TCBGeneric4ArgsVoidRageGrcTextureCRectCRectRageColor32RageGrcTextureCRectCRectRageColor32,
    field_30,
    0x30
);

/// Merged layout for `T_CB_Generic_5Args<void(*)(float, float, CRect&, HtmlRenderState&, bool), float, float, CRect, HtmlRenderState, bool>`.
///
/// Size: 0xe9 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: unknown, lanes: c-misc-b).
    pub field_1c: [u8; 8],
    /// field_24 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// Unknown bytes (0x28..0xe8).
    pub _pad_0028: [u8; 0xc0],
    /// field_e8 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_e8: u8,
    /// Unknown trailing bytes (0xe9..0xec).
    pub _pad_end: [u8; 0x3],
}
assert_size!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    0xec
); // merged size 0xe9 rounded to 4
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_8,
    0x8
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_c,
    0xc
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_10,
    0x10
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_14,
    0x14
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_18,
    0x18
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_1c,
    0x1c
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_24,
    0x24
);
assert_offset!(
    TCBGeneric5ArgsVoidFloatFloatCRectHtmlRenderStateBoolFloatFloatCRectHtmlRenderStateBool,
    field_e8,
    0xe8
);

/// Merged layout for `T_CB_Generic_6Args<void(*)(HtmlRenderState&, float, float, float, float, unsigned int), HtmlRenderState, float, float, float, float, int>`.
///
/// Size: 0xe4 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt
{
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// Unknown bytes (0x10..0xd0).
    pub _pad_0010: [u8; 0xc0],
    /// field_d0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_d0: u32,
    /// field_d4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_d4: u32,
    /// field_d8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_d8: u32,
    /// field_dc (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_dc: u32,
    /// field_e0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_e0: u32,
}
assert_size!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, 0xe4); // merged size 0xe4 rounded to 4
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_8, 0x8);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_c, 0xc);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_d0, 0xd0);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_d4, 0xd4);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_d8, 0xd8);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_dc, 0xdc);
assert_offset!(TCBGeneric6ArgsVoidHtmlRenderStateFloatFloatFloatFloatUnsignedIntHtmlRenderStateFloatFloatFloatFloatInt, field_e0, 0xe0);

/// Merged layout for `T_CB_Generic_6Args<void(*)(const rage::rmcDrawable*, const rage::Matrix34&, int, int, int, int), rage::rmcDrawable*, rage::Matrix34, int, int, signed char, int>`.
///
/// Size: 0x60 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt
{
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: bool/byte?, lanes: c-misc-b).
    pub field_58: u8,
    /// Unknown bytes (0x59..0x5c).
    pub _pad_0059: [u8; 0x3],
    /// field_5c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_5c: u32,
}
assert_size!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, 0x60); // merged size 0x60 rounded to 4
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_8, 0x8);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_c, 0xc);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_10, 0x10);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_14, 0x14);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_18, 0x18);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_20, 0x20);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_24, 0x24);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_28, 0x28);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_30, 0x30);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_34, 0x34);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_38, 0x38);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_40, 0x40);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_44, 0x44);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_48, 0x48);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_50, 0x50);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_54, 0x54);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_58, 0x58);
assert_offset!(TCBGeneric6ArgsVoidConstRageRmcDrawableConstRageMatrix34IntIntIntIntRageRmcDrawableRageMatrix34IntIntSignedCharInt, field_5c, 0x5c);

/// Merged layout for `T_CB_Generic_NoArgs<void(*)(void), <Z>>`.
///
/// Size: 0x9a9 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TCBGenericNoArgsVoidVoidZ {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// Unknown bytes (0x34..0x38).
    pub _pad_0034: [u8; 0x4],
    /// field_38 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: word?, lanes: c-misc-b).
    pub field_3c: u16,
    /// Unknown bytes (0x3e..0x44).
    pub _pad_003e: [u8; 0x6],
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x6c).
    pub _pad_0048: [u8; 0x24],
    /// field_6c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_6c: u32,
    /// field_70 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_70: u32,
    /// Unknown bytes (0x74..0xb0).
    pub _pad_0074: [u8; 0x3c],
    /// field_b0 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_b0: u32,
    /// Unknown bytes (0xb4..0x160).
    pub _pad_00b4: [u8; 0xac],
    /// field_160 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_160: u32,
    /// Unknown bytes (0x164..0x8e8).
    pub _pad_0164: [u8; 0x784],
    /// field_8e8 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_8e8: u32,
    /// Unknown bytes (0x8ec..0x938).
    pub _pad_08ec: [u8; 0x4c],
    /// field_938 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_938: u32,
    /// Unknown bytes (0x93c..0x940).
    pub _pad_093c: [u8; 0x4],
    /// field_940 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_940: u8,
    /// Unknown bytes (0x941..0x9a8).
    pub _pad_0941: [u8; 0x67],
    /// field_9a8 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_9a8: u8,
    /// Unknown trailing bytes (0x9a9..0x9ac).
    pub _pad_end: [u8; 0x3],
}
assert_size!(TCBGenericNoArgsVoidVoidZ, 0x9ac); // merged size 0x9a9 rounded to 4
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_8, 0x8);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_c, 0xc);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_14, 0x14);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_18, 0x18);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_1c, 0x1c);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_20, 0x20);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_24, 0x24);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_28, 0x28);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_30, 0x30);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_38, 0x38);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_3c, 0x3c);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_44, 0x44);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_6c, 0x6c);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_70, 0x70);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_b0, 0xb0);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_160, 0x160);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_8e8, 0x8e8);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_938, 0x938);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_940, 0x940);
assert_offset!(TCBGenericNoArgsVoidVoidZ, field_9a8, 0x9a8);

/// Merged layout for `T_SetShaderGroupVar_1Arg<rage::Vector4>`.
///
/// Size: 0x20 (low). Bases: CBaseDC@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TSetShaderGroupVar1ArgRageVector4 {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
}
assert_size!(TSetShaderGroupVar1ArgRageVector4, 0x20); // merged size 0x20 rounded to 4
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_8, 0x8);
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_c, 0xc);
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_10, 0x10);
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_14, 0x14);
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_18, 0x18);
assert_offset!(TSetShaderGroupVar1ArgRageVector4, field_1c, 0x1c);
