//! Front-end user interface, HTML rendering, replay editor and blip records.
//!
//! Holds 37 draft layouts: The `UI*` frame classes (montage editor, file viewers, menus, scroll
//! bars), `CHtmlNode` and `CHtmlParser`, the `CReplay*` widgets, `CPlayStatBase`,
//! `CFontStringProcess`, and the `BlipRecord` and `GPS race-track point` records seen by the
//! native-handler lanes. Every layout is Inferred; size confidence (the analysis lanes' own rating)
//! is high for 3, medium for 0 and low for 34. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `BlipRecord`.
///
/// Size: 0x60 (low). Bases: none.
/// Lanes: n-05.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlipRecord {
    /// generation_counter_matched_against (confidence: high, kind: i32, lanes: n-05).
    pub generation_counter_matched_against: u16,
    /// Unknown bytes (0x2..0x8).
    pub _pad_0002: [u8; 0x6],
    /// attached_full_data (confidence: low, kind: flags, lanes: n-05).
    pub attached_full_data: u8,
    /// Unknown bytes (0x9..0x20).
    pub _pad_0009: [u8; 0x17],
    /// static_blip_floats (confidence: low, kind: float, lanes: n-05).
    pub static_blip_floats: [u8; 8],
    /// Unknown bytes (0x28..0x30).
    pub _pad_0028: [u8; 0x8],
    /// attached_blip_floats (confidence: low, kind: float, lanes: n-05).
    pub attached_blip_floats: [u8; 12],
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// rotation_radians_converted_floored (confidence: low, kind: u32, lanes: n-05).
    pub rotation_radians_converted_floored: u32,
    /// Unknown bytes (0x44..0x48).
    pub _pad_0044: [u8; 0x4],
    /// blip_type_id (confidence: low, kind: u32, lanes: n-05).
    pub blip_type_id: u32,
    /// blip_display (confidence: low, kind: u32, lanes: n-05).
    pub blip_display: u32,
    /// Unknown bytes (0x50..0x54).
    pub _pad_0050: [u8; 0x4],
    /// colour_record_passed_colour (confidence: medium, kind: u32, lanes: n-05).
    pub colour_record_passed_colour: u32,
    /// alpha (confidence: low, kind: u8, lanes: n-05).
    pub alpha: u8,
    /// Unknown bytes (0x59..0x5c).
    pub _pad_0059: [u8; 0x3],
    /// sprite (confidence: medium, kind: pointer, lanes: n-05).
    pub sprite: Ptr32<u8>,
}
assert_size!(BlipRecord, 0x60); // merged size 0x60 rounded to 4
assert_offset!(BlipRecord, generation_counter_matched_against, 0x0);
assert_offset!(BlipRecord, attached_full_data, 0x8);
assert_offset!(BlipRecord, static_blip_floats, 0x20);
assert_offset!(BlipRecord, attached_blip_floats, 0x30);
assert_offset!(BlipRecord, rotation_radians_converted_floored, 0x40);
assert_offset!(BlipRecord, blip_type_id, 0x48);
assert_offset!(BlipRecord, blip_display, 0x4c);
assert_offset!(BlipRecord, colour_record_passed_colour, 0x54);
assert_offset!(BlipRecord, alpha, 0x58);
assert_offset!(BlipRecord, sprite, 0x5c);

/// Merged layout for `CFontStringProcess`.
///
/// Size: 0xc (low). Bases: none.
/// Lanes: c-misc-b, c-ui, via:Z::SAHMMPAGPAM::CFont::?5??GetNumberLines::CFontGetNumberLines, via:Z::SAXMMPAGHH::CFont::?1??ProcessStringToDisplay::CFontRenderString.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CFontStringProcess {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-misc-b,c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b,via:Z::SAHMMPAGPAM::CFont::?5??GetNumberLines::CFontGetNumberLines,via:Z::SAXMMPAGHH::CFont::?1??ProcessStringToDisplay::CFontRenderString moved from siblings:Z::SAHMMPAGPAM::CFont::?5??GetNumberLines::CFontGetNumberLines,Z::SAXMMPAGHH::CFont::?1??ProcessStringToDisplay::CFontRenderString).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: flags, lanes: c-misc-b,via:Z::SAHMMPAGPAM::CFont::?5??GetNumberLines::CFontGetNumberLines,via:Z::SAXMMPAGHH::CFont::?1??ProcessStringToDisplay::CFontRenderString moved from siblings:Z::SAHMMPAGPAM::CFont::?5??GetNumberLines::CFontGetNumberLines,Z::SAXMMPAGHH::CFont::?1??ProcessStringToDisplay::CFontRenderString).
    pub field_8: u32,
}
assert_size!(CFontStringProcess, 0xc); // merged size 0xc rounded to 4
assert_offset!(CFontStringProcess, vfptr, 0x0);
assert_offset!(CFontStringProcess, field_4, 0x4);
assert_offset!(CFontStringProcess, field_8, 0x8);

/// Merged layout for `CHtmlNode`.
///
/// Size: 0x104 (low). Bases: none.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHtmlNode {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_8: u32,
    /// field_c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_14: u32,
    /// Unknown bytes (0x18..0x38).
    pub _pad_0018: [u8; 0x20],
    /// field_38 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0xd8).
    pub _pad_003c: [u8; 0x9c],
    /// field_d8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_d8: u32,
    /// field_dc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_dc: u32,
    /// field_e0 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_e0: u32,
    /// field_e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_e4: u32,
    /// ptr_E8 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_e8: Ptr32<u8>,
    /// ptr_EC (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_ec: Ptr32<u8>,
    /// field_f0 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_f0: u32,
    /// field_f4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_f4: u32,
    /// field_f8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_f8: u32,
    /// field_fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_fc: u32,
    /// field_100 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_100: u32,
}
assert_size!(CHtmlNode, 0x104); // merged size 0x104 rounded to 4
assert_offset!(CHtmlNode, vfptr, 0x0);
assert_offset!(CHtmlNode, field_4, 0x4);
assert_offset!(CHtmlNode, field_8, 0x8);
assert_offset!(CHtmlNode, field_c, 0xc);
assert_offset!(CHtmlNode, field_10, 0x10);
assert_offset!(CHtmlNode, field_14, 0x14);
assert_offset!(CHtmlNode, field_38, 0x38);
assert_offset!(CHtmlNode, field_d8, 0xd8);
assert_offset!(CHtmlNode, field_dc, 0xdc);
assert_offset!(CHtmlNode, field_e0, 0xe0);
assert_offset!(CHtmlNode, field_e4, 0xe4);
assert_offset!(CHtmlNode, ptr_e8, 0xe8);
assert_offset!(CHtmlNode, ptr_ec, 0xec);
assert_offset!(CHtmlNode, field_f0, 0xf0);
assert_offset!(CHtmlNode, field_f4, 0xf4);
assert_offset!(CHtmlNode, field_f8, 0xf8);
assert_offset!(CHtmlNode, field_fc, 0xfc);
assert_offset!(CHtmlNode, field_100, 0x100);

/// Merged layout for `CHtmlParser`.
///
/// Size: 0x458 (low). Bases: rage::parStreamInXml@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHtmlParser {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x28).
    pub _pad_0020: [u8; 0x8],
    /// field_28 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_28: u32,
    /// field_2c (confidence: low, kind: int32?, lanes: c-ui).
    pub field_2c: u32,
    /// Unknown bytes (0x30..0x38).
    pub _pad_0030: [u8; 0x8],
    /// field_38 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_38: u32,
    /// ptr_3C (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x448).
    pub _pad_0040: [u8; 0x408],
    /// field_448 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_448: u32,
    /// Unknown bytes (0x44c..0x44d).
    pub _pad_044c: [u8; 0x1],
    /// flag_44D (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_44d: u8,
    /// flag_44E (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_44e: u8,
    /// flag_44F (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_44f: u8,
    /// Unknown bytes (0x450..0x454).
    pub _pad_0450: [u8; 0x4],
    /// ptr_454 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_454: Ptr32<u8>,
}
assert_size!(CHtmlParser, 0x458); // merged size 0x458 rounded to 4
assert_offset!(CHtmlParser, field_4, 0x4);
assert_offset!(CHtmlParser, field_8, 0x8);
assert_offset!(CHtmlParser, field_c, 0xc);
assert_offset!(CHtmlParser, field_10, 0x10);
assert_offset!(CHtmlParser, field_14, 0x14);
assert_offset!(CHtmlParser, field_18, 0x18);
assert_offset!(CHtmlParser, field_1c, 0x1c);
assert_offset!(CHtmlParser, field_28, 0x28);
assert_offset!(CHtmlParser, field_2c, 0x2c);
assert_offset!(CHtmlParser, field_38, 0x38);
assert_offset!(CHtmlParser, ptr_3c, 0x3c);
assert_offset!(CHtmlParser, field_448, 0x448);
assert_offset!(CHtmlParser, flag_44d, 0x44d);
assert_offset!(CHtmlParser, flag_44e, 0x44e);
assert_offset!(CHtmlParser, flag_44f, 0x44f);
assert_offset!(CHtmlParser, ptr_454, 0x454);

/// Merged layout for `CPlayStatBase`.
///
/// Size: 0x38 (low). Bases: none.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlayStatBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_c: u32,
    /// flag_10 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_10: u8,
    /// Unknown bytes (0x11..0x30).
    pub _pad_0011: [u8; 0x1f],
    /// field_30 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: int16?, lanes: c-ui).
    pub field_34: u16,
    /// field_36 (confidence: low, kind: int16?, lanes: c-ui).
    pub field_36: u16,
}
assert_size!(CPlayStatBase, 0x38); // merged size 0x38 rounded to 4
assert_offset!(CPlayStatBase, vfptr, 0x0);
assert_offset!(CPlayStatBase, field_4, 0x4);
assert_offset!(CPlayStatBase, field_8, 0x8);
assert_offset!(CPlayStatBase, field_c, 0xc);
assert_offset!(CPlayStatBase, flag_10, 0x10);
assert_offset!(CPlayStatBase, field_30, 0x30);
assert_offset!(CPlayStatBase, field_34, 0x34);
assert_offset!(CPlayStatBase, field_36, 0x36);

/// Merged layout for `CReplayButton`.
///
/// Size: 0x9c (low). Bases: CReplayWidget@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CReplayButton {
    /// Unknown bytes (0x0..0x22).
    pub _pad_0000: [u8; 0x22],
    /// flag_22 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_22: u8,
    /// Unknown bytes (0x23..0x24).
    pub _pad_0023: [u8; 0x1],
    /// field_24 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x2c).
    pub _pad_0028: [u8; 0x4],
    /// field_2c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_2c: u32,
    /// Unknown bytes (0x30..0x34).
    pub _pad_0030: [u8; 0x4],
    /// field_34 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_34: u32,
    /// Unknown bytes (0x38..0x3c).
    pub _pad_0038: [u8; 0x4],
    /// field_3c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_3c: u32,
    /// Unknown bytes (0x40..0x44).
    pub _pad_0040: [u8; 0x4],
    /// field_44 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: int16?, lanes: c-ui).
    pub field_48: u16,
    /// Unknown trailing bytes (0x4a..0x9c).
    pub _pad_end: [u8; 0x52],
}
assert_size!(CReplayButton, 0x9c); // merged size 0x9c rounded to 4
assert_offset!(CReplayButton, flag_22, 0x22);
assert_offset!(CReplayButton, field_24, 0x24);
assert_offset!(CReplayButton, field_2c, 0x2c);
assert_offset!(CReplayButton, field_34, 0x34);
assert_offset!(CReplayButton, field_3c, 0x3c);
assert_offset!(CReplayButton, field_44, 0x44);
assert_offset!(CReplayButton, field_48, 0x48);

/// Merged layout for `CReplayOverlay`.
///
/// Size: 0xec (low). Bases: CReplayWidget@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CReplayOverlay {
    /// Unknown bytes (0x0..0x1b).
    pub _pad_0000: [u8; 0x1b],
    /// flag_1B (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_1b: u8,
    /// Unknown bytes (0x1c..0x24).
    pub _pad_001c: [u8; 0x8],
    /// ptr_24 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_24: Ptr32<u8>,
    /// ptr_28 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_28: Ptr32<u8>,
    /// ptr_2C (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_2c: Ptr32<u8>,
    /// ptr_30 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_30: Ptr32<u8>,
    /// ptr_34 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_34: Ptr32<u8>,
    /// ptr_38 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_38: Ptr32<u8>,
    /// ptr_3C (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_3c: Ptr32<u8>,
    /// ptr_40 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_40: Ptr32<u8>,
    /// ptr_44 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_44: Ptr32<u8>,
    /// flag_48 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_48: u8,
    /// flag_49 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_49: u8,
    /// flag_4A (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_4a: u8,
    /// Unknown bytes (0x4b..0x50).
    pub _pad_004b: [u8; 0x5],
    /// field_50 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_50: u32,
    /// flag_54 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_54: u8,
    /// Unknown bytes (0x55..0x5c).
    pub _pad_0055: [u8; 0x7],
    /// field_5C_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_5c_f: f32,
    /// field_60_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_60_f: f32,
    /// field_64_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_64_f: f32,
    /// field_68_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_68_f: f32,
    /// Unknown trailing bytes (0x6c..0xec).
    pub _pad_end: [u8; 0x80],
}
assert_size!(CReplayOverlay, 0xec); // merged size 0xec rounded to 4
assert_offset!(CReplayOverlay, flag_1b, 0x1b);
assert_offset!(CReplayOverlay, ptr_24, 0x24);
assert_offset!(CReplayOverlay, ptr_28, 0x28);
assert_offset!(CReplayOverlay, ptr_2c, 0x2c);
assert_offset!(CReplayOverlay, ptr_30, 0x30);
assert_offset!(CReplayOverlay, ptr_34, 0x34);
assert_offset!(CReplayOverlay, ptr_38, 0x38);
assert_offset!(CReplayOverlay, ptr_3c, 0x3c);
assert_offset!(CReplayOverlay, ptr_40, 0x40);
assert_offset!(CReplayOverlay, ptr_44, 0x44);
assert_offset!(CReplayOverlay, flag_48, 0x48);
assert_offset!(CReplayOverlay, flag_49, 0x49);
assert_offset!(CReplayOverlay, flag_4a, 0x4a);
assert_offset!(CReplayOverlay, field_50, 0x50);
assert_offset!(CReplayOverlay, flag_54, 0x54);
assert_offset!(CReplayOverlay, field_5c_f, 0x5c);
assert_offset!(CReplayOverlay, field_60_f, 0x60);
assert_offset!(CReplayOverlay, field_64_f, 0x64);
assert_offset!(CReplayOverlay, field_68_f, 0x68);

/// Merged layout for `CReplayProgressBar`.
///
/// Size: 0x10b (low). Bases: CReplayWidget@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CReplayProgressBar {
    /// Unknown bytes (0x0..0x28).
    pub _pad_0000: [u8; 0x28],
    /// field_28_f (confidence: low, kind: float, lanes: c-ui).
    pub field_28_f: f32,
    /// field_2c (confidence: low, kind: int32?, lanes: c-ui).
    pub field_2c: u32,
    /// Unknown bytes (0x30..0x34).
    pub _pad_0030: [u8; 0x4],
    /// field_34 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x9c).
    pub _pad_003c: [u8; 0x60],
    /// field_9c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_9c: u32,
    /// ptr_A0 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_a0: Ptr32<u8>,
    /// ptr_A4 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_a4: Ptr32<u8>,
    /// field_A8_f (confidence: high, kind: float, lanes: c-ui).
    pub field_a8_f: f32,
    /// field_AC_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_ac_f: f32,
    /// ptr_B0 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_b0: Ptr32<u8>,
    /// field_B4_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_b4_f: f32,
    /// ptr_B8 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_b8: Ptr32<u8>,
    /// field_BC_f (confidence: high, kind: float, lanes: c-ui).
    pub field_bc_f: f32,
    /// field_C0_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_c0_f: f32,
    /// ptr_C4 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_c4: Ptr32<u8>,
    /// field_C8_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_c8_f: f32,
    /// ptr_CC (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_cc: Ptr32<u8>,
    /// ptr_D0 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_d0: Ptr32<u8>,
    /// ptr_D4 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_d4: Ptr32<u8>,
    /// ptr_D8 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_d8: Ptr32<u8>,
    /// ptr_DC (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_dc: Ptr32<u8>,
    /// field_e0 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_e0: u32,
    /// ptr_E4 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_e4: Ptr32<u8>,
    /// ptr_E8 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_e8: Ptr32<u8>,
    /// Unknown bytes (0xec..0xf4).
    pub _pad_00ec: [u8; 0x8],
    /// field_f4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_f4: u32,
    /// Unknown bytes (0xf8..0xfc).
    pub _pad_00f8: [u8; 0x4],
    /// ptr_FC (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_fc: Ptr32<u8>,
    /// Unknown bytes (0x100..0x104).
    pub _pad_0100: [u8; 0x4],
    /// field_104 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_104: u32,
    /// field_108 (confidence: low, kind: int16?, lanes: c-ui).
    pub field_108: u16,
    /// flag_10A (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_10a: u8,
    /// Unknown trailing bytes (0x10b..0x10c).
    pub _pad_end: [u8; 0x1],
}
assert_size!(CReplayProgressBar, 0x10c); // merged size 0x10b rounded to 4
assert_offset!(CReplayProgressBar, field_28_f, 0x28);
assert_offset!(CReplayProgressBar, field_2c, 0x2c);
assert_offset!(CReplayProgressBar, field_34, 0x34);
assert_offset!(CReplayProgressBar, field_38, 0x38);
assert_offset!(CReplayProgressBar, field_9c, 0x9c);
assert_offset!(CReplayProgressBar, ptr_a0, 0xa0);
assert_offset!(CReplayProgressBar, ptr_a4, 0xa4);
assert_offset!(CReplayProgressBar, field_a8_f, 0xa8);
assert_offset!(CReplayProgressBar, field_ac_f, 0xac);
assert_offset!(CReplayProgressBar, ptr_b0, 0xb0);
assert_offset!(CReplayProgressBar, field_b4_f, 0xb4);
assert_offset!(CReplayProgressBar, ptr_b8, 0xb8);
assert_offset!(CReplayProgressBar, field_bc_f, 0xbc);
assert_offset!(CReplayProgressBar, field_c0_f, 0xc0);
assert_offset!(CReplayProgressBar, ptr_c4, 0xc4);
assert_offset!(CReplayProgressBar, field_c8_f, 0xc8);
assert_offset!(CReplayProgressBar, ptr_cc, 0xcc);
assert_offset!(CReplayProgressBar, ptr_d0, 0xd0);
assert_offset!(CReplayProgressBar, ptr_d4, 0xd4);
assert_offset!(CReplayProgressBar, ptr_d8, 0xd8);
assert_offset!(CReplayProgressBar, ptr_dc, 0xdc);
assert_offset!(CReplayProgressBar, field_e0, 0xe0);
assert_offset!(CReplayProgressBar, ptr_e4, 0xe4);
assert_offset!(CReplayProgressBar, ptr_e8, 0xe8);
assert_offset!(CReplayProgressBar, field_f4, 0xf4);
assert_offset!(CReplayProgressBar, ptr_fc, 0xfc);
assert_offset!(CReplayProgressBar, field_104, 0x104);
assert_offset!(CReplayProgressBar, field_108, 0x108);
assert_offset!(CReplayProgressBar, flag_10a, 0x10a);

/// Merged layout for `CReplayWidget`.
///
/// Size: 0x104 (low). Bases: none.
/// Lanes: c-ui, via:CReplayOverlay, via:CReplayProgressBar, via:CReplayToolOverlay.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CReplayWidget {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_4: u32,
    /// field_8_f (confidence: low, kind: float, lanes: c-ui).
    pub field_8_f: f32,
    /// field_C_f (confidence: low, kind: float, lanes: c-ui).
    pub field_c_f: f32,
    /// field_10_f (confidence: low, kind: float, lanes: c-ui).
    pub field_10_f: f32,
    /// field_14_f (confidence: low, kind: float, lanes: c-ui).
    pub field_14_f: f32,
    /// flag_18 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_18: u8,
    /// Unknown bytes (0x19..0x1c).
    pub _pad_0019: [u8; 0x3],
    /// ptr_1C (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1c: Ptr32<u8>,
    /// field_20 (confidence: medium, kind: int16?, lanes: c-ui).
    pub field_20: u16,
    /// Unknown bytes (0x22..0x4c).
    pub _pad_0022: [u8; 0x2a],
    /// field_4c (confidence: high, kind: int32?, lanes: c-ui,via:CReplayOverlay,via:CReplayProgressBar moved from siblings:CReplayOverlay,CReplayProgressBar).
    pub field_4c: u32,
    /// Unknown bytes (0x50..0x58).
    pub _pad_0050: [u8; 0x8],
    /// field_58 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x88).
    pub _pad_005c: [u8; 0x2c],
    /// field_88 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_88: u32,
    /// field_8c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_8c: u32,
    /// field_90 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_90: u32,
    /// field_94 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_94: u32,
    /// field_98 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xec).
    pub _pad_009c: [u8; 0x50],
    /// ptr_EC (confidence: high, kind: pointer, lanes: c-ui,via:CReplayProgressBar,via:CReplayToolOverlay moved from siblings:CReplayOverlay,CReplayProgressBar).
    pub ptr_ec: Ptr32<u8>,
    /// ptr_F0 (confidence: medium, kind: pointer, lanes: c-ui,via:CReplayProgressBar,via:CReplayToolOverlay moved from siblings:CReplayOverlay,CReplayProgressBar).
    pub ptr_f0: Ptr32<u8>,
    /// Unknown bytes (0xf4..0xf8).
    pub _pad_00f4: [u8; 0x4],
    /// ptr_F8 (confidence: medium, kind: pointer, lanes: c-ui,via:CReplayProgressBar,via:CReplayToolOverlay moved from siblings:CReplayOverlay,CReplayProgressBar).
    pub ptr_f8: Ptr32<u8>,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: high, kind: int32?, lanes: c-ui,via:CReplayProgressBar,via:CReplayToolOverlay moved from siblings:CReplayOverlay,CReplayProgressBar).
    pub field_100: u32,
}
assert_size!(CReplayWidget, 0x104); // merged size 0x104 rounded to 4
assert_offset!(CReplayWidget, vfptr, 0x0);
assert_offset!(CReplayWidget, field_4, 0x4);
assert_offset!(CReplayWidget, field_8_f, 0x8);
assert_offset!(CReplayWidget, field_c_f, 0xc);
assert_offset!(CReplayWidget, field_10_f, 0x10);
assert_offset!(CReplayWidget, field_14_f, 0x14);
assert_offset!(CReplayWidget, flag_18, 0x18);
assert_offset!(CReplayWidget, ptr_1c, 0x1c);
assert_offset!(CReplayWidget, field_20, 0x20);
assert_offset!(CReplayWidget, field_4c, 0x4c);
assert_offset!(CReplayWidget, field_58, 0x58);
assert_offset!(CReplayWidget, field_88, 0x88);
assert_offset!(CReplayWidget, field_8c, 0x8c);
assert_offset!(CReplayWidget, field_90, 0x90);
assert_offset!(CReplayWidget, field_94, 0x94);
assert_offset!(CReplayWidget, field_98, 0x98);
assert_offset!(CReplayWidget, ptr_ec, 0xec);
assert_offset!(CReplayWidget, ptr_f0, 0xf0);
assert_offset!(CReplayWidget, ptr_f8, 0xf8);
assert_offset!(CReplayWidget, field_100, 0x100);

/// Merged layout for `GPS race-track point (array entry, 16 bytes)`.
///
/// Size: 0x10 (low). Bases: none.
/// Lanes: n-01.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GPSRaceTrackPointArrayEntry16Bytes {
    /// point_entry_appended_add (confidence: medium, kind: float, lanes: n-01).
    pub point_entry_appended_add: f32,
    /// point_entry_appended_add (confidence: medium, kind: float, lanes: n-01).
    pub point_entry_appended_add_2: f32,
    /// point_entry_appended_add (confidence: medium, kind: float, lanes: n-01).
    pub point_entry_appended_add_3: f32,
    /// point_entry_appended_add (confidence: medium, kind: float, lanes: n-01).
    pub point_entry_appended_add_4: f32,
}
assert_size!(GPSRaceTrackPointArrayEntry16Bytes, 0x10); // merged size 0x10 rounded to 4
assert_offset!(
    GPSRaceTrackPointArrayEntry16Bytes,
    point_entry_appended_add,
    0x0
);
assert_offset!(
    GPSRaceTrackPointArrayEntry16Bytes,
    point_entry_appended_add_2,
    0x4
);
assert_offset!(
    GPSRaceTrackPointArrayEntry16Bytes,
    point_entry_appended_add_3,
    0x8
);
assert_offset!(
    GPSRaceTrackPointArrayEntry16Bytes,
    point_entry_appended_add_4,
    0xc
);

/// Merged layout for `UIBasicClip`.
///
/// Size: 0x300 (high). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIBasicClip {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x2f8).
    pub _pad_01ec: [u8; 0x10c],
    /// flag_2F8 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_2f8: u8,
    /// Unknown bytes (0x2f9..0x2fc).
    pub _pad_02f9: [u8; 0x3],
    /// field_2FC_f (confidence: high, kind: float, lanes: c-ui).
    pub field_2fc_f: f32,
}
assert_size!(UIBasicClip, 0x300); // merged size 0x300 rounded to 4
assert_offset!(UIBasicClip, field_1e4, 0x1e4);
assert_offset!(UIBasicClip, field_1e8, 0x1e8);
assert_offset!(UIBasicClip, flag_2f8, 0x2f8);
assert_offset!(UIBasicClip, field_2fc_f, 0x2fc);

/// Merged layout for `UIClip`.
///
/// Size: 0x328 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIClip {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x2f8).
    pub _pad_01ec: [u8; 0x10c],
    /// ptr_2F8 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_2f8: Ptr32<u8>,
    /// ptr_2FC (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_2fc: Ptr32<u8>,
    /// flag_300 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_300: u8,
    /// Unknown bytes (0x301..0x320).
    pub _pad_0301: [u8; 0x1f],
    /// flag_320 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_320: u8,
    /// flag_321 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_321: u8,
    /// Unknown bytes (0x322..0x324).
    pub _pad_0322: [u8; 0x2],
    /// field_324 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_324: u32,
}
assert_size!(UIClip, 0x328); // merged size 0x328 rounded to 4
assert_offset!(UIClip, field_1e4, 0x1e4);
assert_offset!(UIClip, field_1e8, 0x1e8);
assert_offset!(UIClip, ptr_2f8, 0x2f8);
assert_offset!(UIClip, ptr_2fc, 0x2fc);
assert_offset!(UIClip, flag_300, 0x300);
assert_offset!(UIClip, flag_320, 0x320);
assert_offset!(UIClip, flag_321, 0x321);
assert_offset!(UIClip, field_324, 0x324);

/// Merged layout for `UIFileViewer`.
///
/// Size: 0x315 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIFileViewer {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// ptr_1E8 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1e8: Ptr32<u8>,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// Unknown bytes (0x204..0x308).
    pub _pad_0204: [u8; 0x104],
    /// ptr_308 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_308: Ptr32<u8>,
    /// Unknown bytes (0x30c..0x314).
    pub _pad_030c: [u8; 0x8],
    /// flag_314 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_314: u8,
    /// Unknown trailing bytes (0x315..0x318).
    pub _pad_end: [u8; 0x3],
}
assert_size!(UIFileViewer, 0x318); // merged size 0x315 rounded to 4
assert_offset!(UIFileViewer, field_1e4, 0x1e4);
assert_offset!(UIFileViewer, ptr_1e8, 0x1e8);
assert_offset!(UIFileViewer, field_1fc, 0x1fc);
assert_offset!(UIFileViewer, field_200, 0x200);
assert_offset!(UIFileViewer, ptr_308, 0x308);
assert_offset!(UIFileViewer, flag_314, 0x314);

/// Merged layout for `UIFontString`.
///
/// Size: 0x340 (low). Bases: UILayoutFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIFontString {
    /// Unknown bytes (0x0..0x1dc).
    pub _pad_0000: [u8; 0x1dc],
    /// ptr_1DC (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1dc: Ptr32<u8>,
    /// field_1E0_f (confidence: high, kind: float, lanes: c-ui).
    pub field_1e0_f: f32,
    /// field_1E4_f (confidence: high, kind: float, lanes: c-ui).
    pub field_1e4_f: f32,
    /// field_1E8_f (confidence: high, kind: float, lanes: c-ui).
    pub field_1e8_f: f32,
    /// field_1EC_f (confidence: high, kind: float, lanes: c-ui).
    pub field_1ec_f: f32,
    /// Unknown bytes (0x1f0..0x1f4).
    pub _pad_01f0: [u8; 0x4],
    /// ptr_1F4 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1f4: Ptr32<u8>,
    /// field_1f8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1f8: u32,
    /// field_1fc (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x20a).
    pub _pad_0208: [u8; 0x2],
    /// flag_20A (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_20a: u8,
    /// flag_20B (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_20b: u8,
    /// Unknown bytes (0x20c..0x20e).
    pub _pad_020c: [u8; 0x2],
    /// field_20e (confidence: high, kind: int32?, lanes: c-ui).
    pub field_20e: [u8; 4],
    /// Unknown bytes (0x212..0x30d).
    pub _pad_0212: [u8; 0xfb],
    /// flag_30D (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_30d: u8,
    /// field_30e (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_30e: [u8; 4],
    /// Unknown bytes (0x312..0x33c).
    pub _pad_0312: [u8; 0x2a],
    /// ptr_33C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_33c: Ptr32<u8>,
}
assert_size!(UIFontString, 0x340); // merged size 0x340 rounded to 4
assert_offset!(UIFontString, ptr_1dc, 0x1dc);
assert_offset!(UIFontString, field_1e0_f, 0x1e0);
assert_offset!(UIFontString, field_1e4_f, 0x1e4);
assert_offset!(UIFontString, field_1e8_f, 0x1e8);
assert_offset!(UIFontString, field_1ec_f, 0x1ec);
assert_offset!(UIFontString, ptr_1f4, 0x1f4);
assert_offset!(UIFontString, field_1f8, 0x1f8);
assert_offset!(UIFontString, field_1fc, 0x1fc);
assert_offset!(UIFontString, field_200, 0x200);
assert_offset!(UIFontString, field_204, 0x204);
assert_offset!(UIFontString, flag_20a, 0x20a);
assert_offset!(UIFontString, flag_20b, 0x20b);
assert_offset!(UIFontString, field_20e, 0x20e);
assert_offset!(UIFontString, flag_30d, 0x30d);
assert_offset!(UIFontString, field_30e, 0x30e);
assert_offset!(UIFontString, ptr_33c, 0x33c);

/// Merged layout for `UIFrame`.
///
/// Size: 0x314 (low). Bases: UILayoutFrame@0x0.
/// Lanes: c-ui, via:UIClip, via:UIFileViewer, via:UIGTAScrollingRow, via:UILayoutManager, via:UIMontageClip, via:UIMontageContainer, via:UIMontageEditor, via:UIMontageEditorLayout, via:UIMontageMoviePlayer, via:UIMontageOverview, via:UIMontageText, via:UIMouseCursor, via:UIMusicClip, via:UIMusicContainer, via:UIRawClipViewer, via:UIRowLayout, via:UIScrollBar, via:UIScrollingMenu, via:UISelectMenu, via:UIStackLayout, via:UITextContainer, via:UITextContainerItem, via:UITextField, via:UITimeOverview.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIFrame {
    /// Unknown bytes (0x0..0x1dc).
    pub _pad_0000: [u8; 0x1dc],
    /// field_1dc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1dc: u32,
    /// flag_1E0 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UILayoutManager,via:UITextContainer moved from siblings:UILayoutManager,UITextContainer).
    pub flag_1e0: u8,
    /// Unknown bytes (0x1e1..0x1ec).
    pub _pad_01e1: [u8; 0xb],
    /// flag_1EC (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIGTAScrollingRow,via:UIRowLayout,via:UIStackLayout moved from siblings:UIGTAScrollingRow,UILayoutManager).
    pub flag_1ec: u8,
    /// Unknown bytes (0x1ed..0x1f0).
    pub _pad_01ed: [u8; 0x3],
    /// flag_1F0 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMontageText,via:UIRowLayout,via:UIScrollingMenu,via:UIStackLayout moved from siblings:UILayoutManager,UIMontageText,UIScrollingMenu).
    pub flag_1f0: u8,
    /// Unknown bytes (0x1f1..0x1f4).
    pub _pad_01f1: [u8; 0x3],
    /// flag_1F4 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMontageOverview,via:UIMusicContainer,via:UIRowLayout,via:UIStackLayout moved from siblings:UIMontageOverview,UIMusicContainer).
    pub flag_1f4: u8,
    /// Unknown bytes (0x1f5..0x1f8).
    pub _pad_01f5: [u8; 0x3],
    /// flag_1F8 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIClip,via:UIScrollBar moved from siblings:UIClip,UIScrollBar).
    pub flag_1f8: u8,
    /// Unknown bytes (0x1f9..0x20b).
    pub _pad_01f9: [u8; 0x12],
    /// field_20b (confidence: medium, kind: int16?, lanes: c-ui,via:UITextContainerItem,via:UITextField moved from siblings:UISizedContainerItem,UITextField).
    pub field_20b: [u8; 2],
    /// Unknown bytes (0x20d..0x214).
    pub _pad_020d: [u8; 0x7],
    /// flag_214 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMouseCursor,via:UIMusicClip moved from siblings:UIMouseCursor,UIMusicClip).
    pub flag_214: u8,
    /// flag_215 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMouseCursor,via:UIScrollingMenu moved from siblings:UIMouseCursor,UIScrollingMenu).
    pub flag_215: u8,
    /// Unknown bytes (0x216..0x217).
    pub _pad_0216: [u8; 0x1],
    /// flag_217 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIGTAScrollingRow,via:UIScrollingMenu moved from siblings:UIGTAScrollingRow,UIScrollingMenu).
    pub flag_217: u8,
    /// flag_218 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIGTAScrollingRow,via:UIMontageContainer,via:UIScrollingMenu moved from siblings:UIGTAScrollingRow,UIMontageContainer,UIScrollingMenu).
    pub flag_218: u8,
    /// Unknown bytes (0x219..0x21c).
    pub _pad_0219: [u8; 0x3],
    /// flag_21C (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMontageContainer,via:UIRawClipViewer moved from siblings:UIMontageContainer,UIRawClipViewer).
    pub flag_21c: u8,
    /// Unknown bytes (0x21d..0x220).
    pub _pad_021d: [u8; 0x3],
    /// field_220 (confidence: high, kind: int32?, lanes: c-ui,via:UIMontageContainer,via:UIMontageEditor,via:UIMontageEditorLayout,via:UIMontageMoviePlayer,via:UIMontageText,via:UIRawClipViewer,via:UIScrollingMenu,via:UISelectMenu,via:UITimeOverview moved from siblings:UIMontageContainer,UIMontageEditor,UIMontageEditorLayout,UIMontageMoviePlayer,UIMontageText,UIRawClipViewer,UIScrollingMenu,UISelectMenu,UITimeOverview).
    pub field_220: u32,
    /// Unknown bytes (0x224..0x310).
    pub _pad_0224: [u8; 0xec],
    /// field_310 (confidence: high, kind: int32?, lanes: c-ui,via:UIFileViewer,via:UIMontageClip moved from siblings:UIBasicClip,UIFileViewer).
    pub field_310: u32,
}
assert_size!(UIFrame, 0x314); // merged size 0x314 rounded to 4
assert_offset!(UIFrame, field_1dc, 0x1dc);
assert_offset!(UIFrame, flag_1e0, 0x1e0);
assert_offset!(UIFrame, flag_1ec, 0x1ec);
assert_offset!(UIFrame, flag_1f0, 0x1f0);
assert_offset!(UIFrame, flag_1f4, 0x1f4);
assert_offset!(UIFrame, flag_1f8, 0x1f8);
assert_offset!(UIFrame, field_20b, 0x20b);
assert_offset!(UIFrame, flag_214, 0x214);
assert_offset!(UIFrame, flag_215, 0x215);
assert_offset!(UIFrame, flag_217, 0x217);
assert_offset!(UIFrame, flag_218, 0x218);
assert_offset!(UIFrame, flag_21c, 0x21c);
assert_offset!(UIFrame, field_220, 0x220);
assert_offset!(UIFrame, field_310, 0x310);

/// Merged layout for `UIGTAScrollingRow`.
///
/// Size: 0x221 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIGTAScrollingRow {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// ptr_1FC (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1fc: Ptr32<u8>,
    /// field_200 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// ptr_210 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// Unknown trailing bytes (0x214..0x224).
    pub _pad_end: [u8; 0x10],
}
assert_size!(UIGTAScrollingRow, 0x224); // merged size 0x221 rounded to 4
assert_offset!(UIGTAScrollingRow, field_1e4, 0x1e4);
assert_offset!(UIGTAScrollingRow, field_1e8, 0x1e8);
assert_offset!(UIGTAScrollingRow, ptr_1fc, 0x1fc);
assert_offset!(UIGTAScrollingRow, field_200, 0x200);
assert_offset!(UIGTAScrollingRow, field_204, 0x204);
assert_offset!(UIGTAScrollingRow, ptr_210, 0x210);

/// Merged layout for `UIGalleryFileViewer`.
///
/// Size: 0x37a (low). Bases: UIFileViewer@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIGalleryFileViewer {
    /// Unknown bytes (0x0..0x328).
    pub _pad_0000: [u8; 0x328],
    /// field_328 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_328: u32,
    /// field_32c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_32c: u32,
    /// field_330 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_330: u32,
    /// Unknown bytes (0x334..0x350).
    pub _pad_0334: [u8; 0x1c],
    /// field_350 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_350: u32,
    /// field_354 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_354: u32,
    /// field_358 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_358: u32,
    /// flag_35C (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_35c: u8,
    /// Unknown bytes (0x35d..0x360).
    pub _pad_035d: [u8; 0x3],
    /// ptr_360 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_360: Ptr32<u8>,
    /// field_364 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_364: u32,
    /// ptr_368 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_368: Ptr32<u8>,
    /// field_36c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_36c: u32,
    /// ptr_370 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_370: Ptr32<u8>,
    /// field_374 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_374: u32,
    /// flag_378 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_378: u8,
    /// flag_379 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_379: u8,
    /// Unknown trailing bytes (0x37a..0x37c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(UIGalleryFileViewer, 0x37c); // merged size 0x37a rounded to 4
assert_offset!(UIGalleryFileViewer, field_328, 0x328);
assert_offset!(UIGalleryFileViewer, field_32c, 0x32c);
assert_offset!(UIGalleryFileViewer, field_330, 0x330);
assert_offset!(UIGalleryFileViewer, field_350, 0x350);
assert_offset!(UIGalleryFileViewer, field_354, 0x354);
assert_offset!(UIGalleryFileViewer, field_358, 0x358);
assert_offset!(UIGalleryFileViewer, flag_35c, 0x35c);
assert_offset!(UIGalleryFileViewer, ptr_360, 0x360);
assert_offset!(UIGalleryFileViewer, field_364, 0x364);
assert_offset!(UIGalleryFileViewer, ptr_368, 0x368);
assert_offset!(UIGalleryFileViewer, field_36c, 0x36c);
assert_offset!(UIGalleryFileViewer, ptr_370, 0x370);
assert_offset!(UIGalleryFileViewer, field_374, 0x374);
assert_offset!(UIGalleryFileViewer, flag_378, 0x378);
assert_offset!(UIGalleryFileViewer, flag_379, 0x379);

/// Merged layout for `UILayoutFrame`.
///
/// Size: 0x233 (low). Bases: none.
/// Lanes: c-ui, via:UIFontString, via:UIFrame, via:UIMontageEditorLayout, via:UIMontageMoviePlayer, via:UITextContainerItem, via:UITextField, via:UITexture.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UILayoutFrame {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4_f (confidence: high, kind: float, lanes: c-ui).
    pub field_4_f: f32,
    /// field_8_f (confidence: high, kind: float, lanes: c-ui).
    pub field_8_f: f32,
    /// field_C_f (confidence: high, kind: float, lanes: c-ui).
    pub field_c_f: f32,
    /// field_10_f (confidence: high, kind: float, lanes: c-ui).
    pub field_10_f: f32,
    /// field_14 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_14: u32,
    /// field_18_f (confidence: high, kind: float, lanes: c-ui).
    pub field_18_f: f32,
    /// field_1c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x9c).
    pub _pad_0020: [u8; 0x7c],
    /// ptr_9C (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_9c: Ptr32<u8>,
    /// ptr_A0 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_a0: Ptr32<u8>,
    /// field_A4_f (confidence: high, kind: float, lanes: c-ui).
    pub field_a4_f: f32,
    /// field_a8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_a8: u32,
    /// field_ac (confidence: high, kind: int32?, lanes: c-ui).
    pub field_ac: u32,
    /// field_B0_f (confidence: high, kind: float, lanes: c-ui).
    pub field_b0_f: f32,
    /// field_b4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_b4: u32,
    /// field_b8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_b8: u32,
    /// field_bc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_bc: u32,
    /// field_c0 (confidence: high, kind: int16?, lanes: c-ui).
    pub field_c0: u16,
    /// Unknown bytes (0xc2..0xc4).
    pub _pad_00c2: [u8; 0x2],
    /// flag_C4 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_c4: u8,
    /// flag_C5 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_c5: u8,
    /// flag_C6 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_c6: u8,
    /// flag_C7 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_c7: u8,
    /// flag_C8 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_c8: u8,
    /// flag_C9 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_c9: u8,
    /// flag_CA (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_ca: u8,
    /// flag_CB (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_cb: u8,
    /// field_cc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_cc: u32,
    /// flag_D0 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_d0: u8,
    /// flag_D1 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_d1: u8,
    /// Unknown bytes (0xd2..0xd4).
    pub _pad_00d2: [u8; 0x2],
    /// field_d4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_d4: u32,
    /// field_d8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_d8: u32,
    /// field_dc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_dc: u32,
    /// field_e0 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_e0: u32,
    /// field_e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_e4: u32,
    /// flag_E8 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_e8: u8,
    /// flag_E9 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_e9: u8,
    /// flag_EA (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_ea: u8,
    /// flag_EB (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_eb: u8,
    /// field_ec (confidence: high, kind: int32?, lanes: c-ui).
    pub field_ec: u32,
    /// Unknown bytes (0xf0..0x110).
    pub _pad_00f0: [u8; 0x20],
    /// ptr_110 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_110: Ptr32<u8>,
    /// ptr_114 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_114: Ptr32<u8>,
    /// ptr_118 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_118: Ptr32<u8>,
    /// ptr_11C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_11c: Ptr32<u8>,
    /// ptr_120 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_120: Ptr32<u8>,
    /// Unknown bytes (0x124..0x130).
    pub _pad_0124: [u8; 0xc],
    /// flag_130 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_130: u8,
    /// Unknown bytes (0x131..0x154).
    pub _pad_0131: [u8; 0x23],
    /// ptr_154 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_154: Ptr32<u8>,
    /// ptr_158 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_158: Ptr32<u8>,
    /// ptr_15C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_15c: Ptr32<u8>,
    /// ptr_160 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_160: Ptr32<u8>,
    /// ptr_164 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_164: Ptr32<u8>,
    /// Unknown bytes (0x168..0x174).
    pub _pad_0168: [u8; 0xc],
    /// flag_174 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_174: u8,
    /// Unknown bytes (0x175..0x198).
    pub _pad_0175: [u8; 0x23],
    /// ptr_198 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_198: Ptr32<u8>,
    /// ptr_19C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_19c: Ptr32<u8>,
    /// ptr_1A0 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_1a0: Ptr32<u8>,
    /// ptr_1A4 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_1a4: Ptr32<u8>,
    /// ptr_1A8 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_1a8: Ptr32<u8>,
    /// Unknown bytes (0x1ac..0x1b8).
    pub _pad_01ac: [u8; 0xc],
    /// flag_1B8 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_1b8: u8,
    /// Unknown bytes (0x1b9..0x1bc).
    pub _pad_01b9: [u8; 0x3],
    /// field_1bc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1bc: u32,
    /// field_1c0 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1c0: u32,
    /// field_1c4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1c4: u32,
    /// field_1c8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1c8: u32,
    /// Unknown bytes (0x1cc..0x1d4).
    pub _pad_01cc: [u8; 0x8],
    /// field_1d4 (confidence: high, kind: int32?, lanes: c-ui,via:UIFontString,via:UIFrame,via:UITexture moved from siblings:UIFontString,UIFrame,UITexture).
    pub field_1d4: u32,
    /// field_1d8 (confidence: high, kind: int32?, lanes: c-ui,via:UIFontString,via:UITexture moved from siblings:UIFontString,UITexture).
    pub field_1d8: u32,
    /// Unknown bytes (0x1dc..0x208).
    pub _pad_01dc: [u8; 0x2c],
    /// flag_208 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIFontString,via:UIFrame,via:UITextContainerItem,via:UITextField moved from siblings:UIFontString,UIFrame).
    pub flag_208: u8,
    /// flag_209 (confidence: medium, kind: bool-or-byte, lanes: c-ui,via:UIFontString,via:UITextField moved from siblings:UIFontString,UIFrame).
    pub flag_209: u8,
    /// Unknown bytes (0x20a..0x20d).
    pub _pad_020a: [u8; 0x3],
    /// flag_20D (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIFontString,via:UIFrame,via:UITextContainerItem,via:UITextField moved from siblings:UIFontString,UIFrame).
    pub flag_20d: u8,
    /// Unknown bytes (0x20e..0x22c).
    pub _pad_020e: [u8; 0x1e],
    /// field_22c (confidence: high, kind: int32?, lanes: c-ui,via:UIFrame,via:UIMontageEditorLayout,via:UIMontageMoviePlayer,via:UITexture moved from siblings:UIFrame,UITexture).
    pub field_22c: u32,
    /// Unknown bytes (0x230..0x232).
    pub _pad_0230: [u8; 0x2],
    /// flag_232 (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIMontageEditorLayout,via:UITexture moved from siblings:UIFrame,UITexture).
    pub flag_232: u8,
    /// Unknown trailing bytes (0x233..0x234).
    pub _pad_end: [u8; 0x1],
}
assert_size!(UILayoutFrame, 0x234); // merged size 0x233 rounded to 4
assert_offset!(UILayoutFrame, vfptr, 0x0);
assert_offset!(UILayoutFrame, field_4_f, 0x4);
assert_offset!(UILayoutFrame, field_8_f, 0x8);
assert_offset!(UILayoutFrame, field_c_f, 0xc);
assert_offset!(UILayoutFrame, field_10_f, 0x10);
assert_offset!(UILayoutFrame, field_14, 0x14);
assert_offset!(UILayoutFrame, field_18_f, 0x18);
assert_offset!(UILayoutFrame, field_1c, 0x1c);
assert_offset!(UILayoutFrame, ptr_9c, 0x9c);
assert_offset!(UILayoutFrame, ptr_a0, 0xa0);
assert_offset!(UILayoutFrame, field_a4_f, 0xa4);
assert_offset!(UILayoutFrame, field_a8, 0xa8);
assert_offset!(UILayoutFrame, field_ac, 0xac);
assert_offset!(UILayoutFrame, field_b0_f, 0xb0);
assert_offset!(UILayoutFrame, field_b4, 0xb4);
assert_offset!(UILayoutFrame, field_b8, 0xb8);
assert_offset!(UILayoutFrame, field_bc, 0xbc);
assert_offset!(UILayoutFrame, field_c0, 0xc0);
assert_offset!(UILayoutFrame, flag_c4, 0xc4);
assert_offset!(UILayoutFrame, flag_c5, 0xc5);
assert_offset!(UILayoutFrame, flag_c6, 0xc6);
assert_offset!(UILayoutFrame, flag_c7, 0xc7);
assert_offset!(UILayoutFrame, flag_c8, 0xc8);
assert_offset!(UILayoutFrame, flag_c9, 0xc9);
assert_offset!(UILayoutFrame, flag_ca, 0xca);
assert_offset!(UILayoutFrame, flag_cb, 0xcb);
assert_offset!(UILayoutFrame, field_cc, 0xcc);
assert_offset!(UILayoutFrame, flag_d0, 0xd0);
assert_offset!(UILayoutFrame, flag_d1, 0xd1);
assert_offset!(UILayoutFrame, field_d4, 0xd4);
assert_offset!(UILayoutFrame, field_d8, 0xd8);
assert_offset!(UILayoutFrame, field_dc, 0xdc);
assert_offset!(UILayoutFrame, field_e0, 0xe0);
assert_offset!(UILayoutFrame, field_e4, 0xe4);
assert_offset!(UILayoutFrame, flag_e8, 0xe8);
assert_offset!(UILayoutFrame, flag_e9, 0xe9);
assert_offset!(UILayoutFrame, flag_ea, 0xea);
assert_offset!(UILayoutFrame, flag_eb, 0xeb);
assert_offset!(UILayoutFrame, field_ec, 0xec);
assert_offset!(UILayoutFrame, ptr_110, 0x110);
assert_offset!(UILayoutFrame, ptr_114, 0x114);
assert_offset!(UILayoutFrame, ptr_118, 0x118);
assert_offset!(UILayoutFrame, ptr_11c, 0x11c);
assert_offset!(UILayoutFrame, ptr_120, 0x120);
assert_offset!(UILayoutFrame, flag_130, 0x130);
assert_offset!(UILayoutFrame, ptr_154, 0x154);
assert_offset!(UILayoutFrame, ptr_158, 0x158);
assert_offset!(UILayoutFrame, ptr_15c, 0x15c);
assert_offset!(UILayoutFrame, ptr_160, 0x160);
assert_offset!(UILayoutFrame, ptr_164, 0x164);
assert_offset!(UILayoutFrame, flag_174, 0x174);
assert_offset!(UILayoutFrame, ptr_198, 0x198);
assert_offset!(UILayoutFrame, ptr_19c, 0x19c);
assert_offset!(UILayoutFrame, ptr_1a0, 0x1a0);
assert_offset!(UILayoutFrame, ptr_1a4, 0x1a4);
assert_offset!(UILayoutFrame, ptr_1a8, 0x1a8);
assert_offset!(UILayoutFrame, flag_1b8, 0x1b8);
assert_offset!(UILayoutFrame, field_1bc, 0x1bc);
assert_offset!(UILayoutFrame, field_1c0, 0x1c0);
assert_offset!(UILayoutFrame, field_1c4, 0x1c4);
assert_offset!(UILayoutFrame, field_1c8, 0x1c8);
assert_offset!(UILayoutFrame, field_1d4, 0x1d4);
assert_offset!(UILayoutFrame, field_1d8, 0x1d8);
assert_offset!(UILayoutFrame, flag_208, 0x208);
assert_offset!(UILayoutFrame, flag_209, 0x209);
assert_offset!(UILayoutFrame, flag_20d, 0x20d);
assert_offset!(UILayoutFrame, field_22c, 0x22c);
assert_offset!(UILayoutFrame, flag_232, 0x232);

/// Merged layout for `UILayoutManager`.
///
/// Size: 0x1f0 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui, via:UIRowLayout, via:UIStackLayout.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UILayoutManager {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1E4_f (confidence: high, kind: float, lanes: c-ui,via:UIRowLayout,via:UIStackLayout moved from siblings:UIRowLayout,UIStackLayout).
    pub field_1e4_f: f32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui,via:UIRowLayout,via:UIStackLayout moved from siblings:UIRowLayout,UIStackLayout).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1ed).
    pub _pad_01ec: [u8; 0x1],
    /// flag_1ED (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIRowLayout,via:UIStackLayout moved from siblings:UIRowLayout,UIStackLayout).
    pub flag_1ed: u8,
    /// flag_1EE (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIRowLayout,via:UIStackLayout moved from siblings:UIRowLayout,UIStackLayout).
    pub flag_1ee: u8,
    /// flag_1EF (confidence: high, kind: bool-or-byte, lanes: c-ui,via:UIRowLayout,via:UIStackLayout moved from siblings:UIRowLayout,UIStackLayout).
    pub flag_1ef: u8,
}
assert_size!(UILayoutManager, 0x1f0); // merged size 0x1f0 rounded to 4
assert_offset!(UILayoutManager, field_1e4_f, 0x1e4);
assert_offset!(UILayoutManager, field_1e8, 0x1e8);
assert_offset!(UILayoutManager, flag_1ed, 0x1ed);
assert_offset!(UILayoutManager, flag_1ee, 0x1ee);
assert_offset!(UILayoutManager, flag_1ef, 0x1ef);

/// Merged layout for `UIMontageClip`.
///
/// Size: 0x31c (low). Bases: UIBasicClip@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageClip {
    /// Unknown bytes (0x0..0x300).
    pub _pad_0000: [u8; 0x300],
    /// field_300 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_300: u32,
    /// field_304 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_304: u32,
    /// field_308 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_308: u32,
    /// field_30c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_30c: u32,
    /// Unknown bytes (0x310..0x314).
    pub _pad_0310: [u8; 0x4],
    /// field_314 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_314: u32,
    /// field_318 (confidence: high, kind: int16?, lanes: c-ui).
    pub field_318: u16,
    /// Unknown trailing bytes (0x31a..0x31c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(UIMontageClip, 0x31c); // merged size 0x31c rounded to 4
assert_offset!(UIMontageClip, field_300, 0x300);
assert_offset!(UIMontageClip, field_304, 0x304);
assert_offset!(UIMontageClip, field_308, 0x308);
assert_offset!(UIMontageClip, field_30c, 0x30c);
assert_offset!(UIMontageClip, field_314, 0x314);
assert_offset!(UIMontageClip, field_318, 0x318);

/// Merged layout for `UIMontageContainer`.
///
/// Size: 0x225 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageContainer {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// ptr_1FC (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_1fc: Ptr32<u8>,
    /// Unknown bytes (0x200..0x204).
    pub _pad_0200: [u8; 0x4],
    /// ptr_204 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_204: Ptr32<u8>,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// field_210 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_210: u32,
    /// Unknown bytes (0x214..0x219).
    pub _pad_0214: [u8; 0x5],
    /// flag_219 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_219: u8,
    /// Unknown bytes (0x21a..0x21b).
    pub _pad_021a: [u8; 0x1],
    /// flag_21B (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_21b: u8,
    /// Unknown bytes (0x21c..0x224).
    pub _pad_021c: [u8; 0x8],
    /// flag_224 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_224: u8,
    /// Unknown trailing bytes (0x225..0x228).
    pub _pad_end: [u8; 0x3],
}
assert_size!(UIMontageContainer, 0x228); // merged size 0x225 rounded to 4
assert_offset!(UIMontageContainer, field_1e4, 0x1e4);
assert_offset!(UIMontageContainer, field_1e8, 0x1e8);
assert_offset!(UIMontageContainer, ptr_1fc, 0x1fc);
assert_offset!(UIMontageContainer, ptr_204, 0x204);
assert_offset!(UIMontageContainer, field_210, 0x210);
assert_offset!(UIMontageContainer, flag_219, 0x219);
assert_offset!(UIMontageContainer, flag_21b, 0x21b);
assert_offset!(UIMontageContainer, flag_224, 0x224);

/// Merged layout for `UIMontageEditor`.
///
/// Size: 0x22c (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageEditor {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x210).
    pub _pad_01ec: [u8; 0x24],
    /// ptr_210 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// Unknown bytes (0x214..0x224).
    pub _pad_0214: [u8; 0x10],
    /// ptr_224 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_224: Ptr32<u8>,
    /// ptr_228 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_228: Ptr32<u8>,
}
assert_size!(UIMontageEditor, 0x22c); // merged size 0x22c rounded to 4
assert_offset!(UIMontageEditor, field_1e4, 0x1e4);
assert_offset!(UIMontageEditor, field_1e8, 0x1e8);
assert_offset!(UIMontageEditor, ptr_210, 0x210);
assert_offset!(UIMontageEditor, ptr_224, 0x224);
assert_offset!(UIMontageEditor, ptr_228, 0x228);

/// Merged layout for `UIMontageEditorControls`.
///
/// Size: 0x204 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageEditorControls {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_200: u32,
}
assert_size!(UIMontageEditorControls, 0x204); // merged size 0x204 rounded to 4
assert_offset!(UIMontageEditorControls, field_1e4, 0x1e4);
assert_offset!(UIMontageEditorControls, field_1e8, 0x1e8);
assert_offset!(UIMontageEditorControls, field_1fc, 0x1fc);
assert_offset!(UIMontageEditorControls, field_200, 0x200);

/// Merged layout for `UIMontageEditorLayout`.
///
/// Size: 0x233 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageEditorLayout {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x210).
    pub _pad_01ec: [u8; 0x24],
    /// field_210 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_210: u32,
    /// Unknown bytes (0x214..0x224).
    pub _pad_0214: [u8; 0x10],
    /// ptr_224 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_224: Ptr32<u8>,
    /// Unknown trailing bytes (0x228..0x234).
    pub _pad_end: [u8; 0xc],
}
assert_size!(UIMontageEditorLayout, 0x234); // merged size 0x233 rounded to 4
assert_offset!(UIMontageEditorLayout, field_1e4, 0x1e4);
assert_offset!(UIMontageEditorLayout, field_1e8, 0x1e8);
assert_offset!(UIMontageEditorLayout, field_210, 0x210);
assert_offset!(UIMontageEditorLayout, ptr_224, 0x224);

/// Merged layout for `UIMontageGallery`.
///
/// Size: 0x450 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageGallery {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// Unknown bytes (0x1e8..0x204).
    pub _pad_01e8: [u8; 0x1c],
    /// flag_204 (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_204: u8,
    /// Unknown bytes (0x205..0x229).
    pub _pad_0205: [u8; 0x24],
    /// flag_229 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_229: u8,
    /// Unknown bytes (0x22a..0x429).
    pub _pad_022a: [u8; 0x1ff],
    /// flag_429 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_429: u8,
    /// Unknown bytes (0x42a..0x42c).
    pub _pad_042a: [u8; 0x2],
    /// ptr_42C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_42c: Ptr32<u8>,
    /// field_430 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_430: u32,
    /// field_434 (confidence: medium, kind: int16?, lanes: c-ui).
    pub field_434: u16,
    /// Unknown bytes (0x436..0x438).
    pub _pad_0436: [u8; 0x2],
    /// ptr_438 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_438: Ptr32<u8>,
    /// field_43c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_43c: u32,
    /// ptr_440 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_440: Ptr32<u8>,
    /// field_444 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_444: u32,
    /// ptr_448 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_448: Ptr32<u8>,
    /// field_44c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_44c: u32,
}
assert_size!(UIMontageGallery, 0x450); // merged size 0x450 rounded to 4
assert_offset!(UIMontageGallery, field_1e4, 0x1e4);
assert_offset!(UIMontageGallery, flag_204, 0x204);
assert_offset!(UIMontageGallery, flag_229, 0x229);
assert_offset!(UIMontageGallery, flag_429, 0x429);
assert_offset!(UIMontageGallery, ptr_42c, 0x42c);
assert_offset!(UIMontageGallery, field_430, 0x430);
assert_offset!(UIMontageGallery, field_434, 0x434);
assert_offset!(UIMontageGallery, ptr_438, 0x438);
assert_offset!(UIMontageGallery, field_43c, 0x43c);
assert_offset!(UIMontageGallery, ptr_440, 0x440);
assert_offset!(UIMontageGallery, field_444, 0x444);
assert_offset!(UIMontageGallery, ptr_448, 0x448);
assert_offset!(UIMontageGallery, field_44c, 0x44c);

/// Merged layout for `UIMontageMoviePlayer`.
///
/// Size: 0x24a (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageMoviePlayer {
    /// Unknown bytes (0x0..0x8c).
    pub _pad_0000: [u8; 0x8c],
    /// field_8c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_8c: u32,
    /// Unknown bytes (0x90..0x1e4).
    pub _pad_0090: [u8; 0x154],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// field_210 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_210: u32,
    /// Unknown bytes (0x214..0x224).
    pub _pad_0214: [u8; 0x10],
    /// ptr_224 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_224: Ptr32<u8>,
    /// field_228 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_228: u32,
    /// Unknown bytes (0x22c..0x234).
    pub _pad_022c: [u8; 0x8],
    /// field_234 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_234: u32,
    /// flag_238 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_238: u8,
    /// Unknown bytes (0x239..0x23a).
    pub _pad_0239: [u8; 0x1],
    /// flag_23A (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_23a: u8,
    /// Unknown bytes (0x23b..0x23c).
    pub _pad_023b: [u8; 0x1],
    /// field_23c (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_23c: u32,
    /// field_240 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_240: u32,
    /// field_244 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_244: u32,
    /// flag_248 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_248: u8,
    /// flag_249 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_249: u8,
    /// Unknown trailing bytes (0x24a..0x24c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(UIMontageMoviePlayer, 0x24c); // merged size 0x24a rounded to 4
assert_offset!(UIMontageMoviePlayer, field_8c, 0x8c);
assert_offset!(UIMontageMoviePlayer, field_1e4, 0x1e4);
assert_offset!(UIMontageMoviePlayer, field_1e8, 0x1e8);
assert_offset!(UIMontageMoviePlayer, field_1fc, 0x1fc);
assert_offset!(UIMontageMoviePlayer, field_200, 0x200);
assert_offset!(UIMontageMoviePlayer, field_204, 0x204);
assert_offset!(UIMontageMoviePlayer, field_210, 0x210);
assert_offset!(UIMontageMoviePlayer, ptr_224, 0x224);
assert_offset!(UIMontageMoviePlayer, field_228, 0x228);
assert_offset!(UIMontageMoviePlayer, field_234, 0x234);
assert_offset!(UIMontageMoviePlayer, flag_238, 0x238);
assert_offset!(UIMontageMoviePlayer, flag_23a, 0x23a);
assert_offset!(UIMontageMoviePlayer, field_23c, 0x23c);
assert_offset!(UIMontageMoviePlayer, field_240, 0x240);
assert_offset!(UIMontageMoviePlayer, field_244, 0x244);
assert_offset!(UIMontageMoviePlayer, flag_248, 0x248);
assert_offset!(UIMontageMoviePlayer, flag_249, 0x249);

/// Merged layout for `UIMontageText`.
///
/// Size: 0x224 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMontageText {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// field_210 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_210: u32,
    /// Unknown trailing bytes (0x214..0x224).
    pub _pad_end: [u8; 0x10],
}
assert_size!(UIMontageText, 0x224); // merged size 0x224 rounded to 4
assert_offset!(UIMontageText, field_1e4, 0x1e4);
assert_offset!(UIMontageText, field_1e8, 0x1e8);
assert_offset!(UIMontageText, field_1fc, 0x1fc);
assert_offset!(UIMontageText, field_200, 0x200);
assert_offset!(UIMontageText, field_204, 0x204);
assert_offset!(UIMontageText, field_210, 0x210);

/// Merged layout for `UIMouseCursor`.
///
/// Size: 0x218 (high). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMouseCursor {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x200).
    pub _pad_01ec: [u8; 0x14],
    /// field_200 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// ptr_204 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_204: Ptr32<u8>,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// field_210_f (confidence: low, kind: float, lanes: c-ui).
    pub field_210_f: f32,
    /// Unknown trailing bytes (0x214..0x218).
    pub _pad_end: [u8; 0x4],
}
assert_size!(UIMouseCursor, 0x218); // merged size 0x218 rounded to 4
assert_offset!(UIMouseCursor, field_1e4, 0x1e4);
assert_offset!(UIMouseCursor, field_1e8, 0x1e8);
assert_offset!(UIMouseCursor, field_200, 0x200);
assert_offset!(UIMouseCursor, ptr_204, 0x204);
assert_offset!(UIMouseCursor, field_210_f, 0x210);

/// Merged layout for `UIMusicClip`.
///
/// Size: 0x220 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMusicClip {
    /// Unknown bytes (0x0..0x147).
    pub _pad_0000: [u8; 0x147],
    /// ptr_147 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_147: [u8; 4],
    /// Unknown bytes (0x14b..0x14c).
    pub _pad_014b: [u8; 0x1],
    /// ptr_14C (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_14c: Ptr32<u8>,
    /// Unknown bytes (0x150..0x1e4).
    pub _pad_0150: [u8; 0x94],
    /// field_1e4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// flag_210 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_210: u8,
    /// flag_211 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_211: u8,
    /// flag_212 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_212: u8,
    /// flag_213 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_213: u8,
    /// Unknown trailing bytes (0x214..0x220).
    pub _pad_end: [u8; 0xc],
}
assert_size!(UIMusicClip, 0x220); // merged size 0x220 rounded to 4
assert_offset!(UIMusicClip, ptr_147, 0x147);
assert_offset!(UIMusicClip, ptr_14c, 0x14c);
assert_offset!(UIMusicClip, field_1e4, 0x1e4);
assert_offset!(UIMusicClip, field_1e8, 0x1e8);
assert_offset!(UIMusicClip, field_1fc, 0x1fc);
assert_offset!(UIMusicClip, field_200, 0x200);
assert_offset!(UIMusicClip, field_204, 0x204);
assert_offset!(UIMusicClip, flag_210, 0x210);
assert_offset!(UIMusicClip, flag_211, 0x211);
assert_offset!(UIMusicClip, flag_212, 0x212);
assert_offset!(UIMusicClip, flag_213, 0x213);

/// Merged layout for `UIMusicContainer`.
///
/// Size: 0x3d0 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIMusicContainer {
    /// Unknown bytes (0x0..0x140).
    pub _pad_0000: [u8; 0x140],
    /// field_140 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_140: u32,
    /// Unknown bytes (0x144..0x1e4).
    pub _pad_0144: [u8; 0xa0],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// ptr_1E8 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1e8: Ptr32<u8>,
    /// Unknown bytes (0x1ec..0x1f5).
    pub _pad_01ec: [u8; 0x9],
    /// flag_1F5 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_1f5: u8,
    /// Unknown bytes (0x1f6..0x1fc).
    pub _pad_01f6: [u8; 0x6],
    /// field_1fc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x3cc).
    pub _pad_0208: [u8; 0x1c4],
    /// field_3cc (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_3cc: u32,
}
assert_size!(UIMusicContainer, 0x3d0); // merged size 0x3d0 rounded to 4
assert_offset!(UIMusicContainer, field_140, 0x140);
assert_offset!(UIMusicContainer, field_1e4, 0x1e4);
assert_offset!(UIMusicContainer, ptr_1e8, 0x1e8);
assert_offset!(UIMusicContainer, flag_1f5, 0x1f5);
assert_offset!(UIMusicContainer, field_1fc, 0x1fc);
assert_offset!(UIMusicContainer, field_200, 0x200);
assert_offset!(UIMusicContainer, field_204, 0x204);
assert_offset!(UIMusicContainer, field_3cc, 0x3cc);

/// Merged layout for `UIRawClipViewer`.
///
/// Size: 0x3f4 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIRawClipViewer {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// embedded_UIRolloverMessage (confidence: high, kind: embedded-object, lanes: c-ui).
    pub embedded_uirollovermessage: u32,
    /// Unknown bytes (0x214..0x334).
    pub _pad_0214: [u8; 0x120],
    /// flag_334 (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_334: u8,
    /// Unknown bytes (0x335..0x338).
    pub _pad_0335: [u8; 0x3],
    /// field_338 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_338: u32,
    /// field_33c (confidence: high, kind: int32?, lanes: c-ui).
    pub field_33c: u32,
    /// field_340 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_340: u32,
    /// field_344 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_344: u32,
    /// Unknown bytes (0x348..0x350).
    pub _pad_0348: [u8; 0x8],
    /// ptr_350 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_350: Ptr32<u8>,
    /// Unknown bytes (0x354..0x3f0).
    pub _pad_0354: [u8; 0x9c],
    /// field_3f0 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_3f0: u32,
}
assert_size!(UIRawClipViewer, 0x3f4); // merged size 0x3f4 rounded to 4
assert_offset!(UIRawClipViewer, field_1e4, 0x1e4);
assert_offset!(UIRawClipViewer, field_1e8, 0x1e8);
assert_offset!(UIRawClipViewer, field_1fc, 0x1fc);
assert_offset!(UIRawClipViewer, field_200, 0x200);
assert_offset!(UIRawClipViewer, field_204, 0x204);
assert_offset!(UIRawClipViewer, embedded_uirollovermessage, 0x210);
assert_offset!(UIRawClipViewer, flag_334, 0x334);
assert_offset!(UIRawClipViewer, field_338, 0x338);
assert_offset!(UIRawClipViewer, field_33c, 0x33c);
assert_offset!(UIRawClipViewer, field_340, 0x340);
assert_offset!(UIRawClipViewer, field_344, 0x344);
assert_offset!(UIRawClipViewer, ptr_350, 0x350);
assert_offset!(UIRawClipViewer, field_3f0, 0x3f0);

/// Merged layout for `UIRolloverMessage`.
///
/// Size: 0x44c (low). Bases: none.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIRolloverMessage {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-ui).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_4: u32,
    /// field_8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_8: u32,
    /// field_c (confidence: low, kind: int32?, lanes: c-ui).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x1f8).
    pub _pad_0010: [u8; 0x1e8],
    /// embedded_UIRolloverMessage (confidence: medium, kind: embedded-object, lanes: c-ui).
    pub embedded_uirollovermessage: u32,
    /// Unknown bytes (0x1fc..0x210).
    pub _pad_01fc: [u8; 0x14],
    /// embedded_UIRolloverMessage (confidence: medium, kind: embedded-object, lanes: c-ui).
    pub embedded_uirollovermessage_2: u32,
    /// Unknown bytes (0x214..0x438).
    pub _pad_0214: [u8; 0x224],
    /// ptr_438 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_438: Ptr32<u8>,
    /// Unknown bytes (0x43c..0x440).
    pub _pad_043c: [u8; 0x4],
    /// ptr_440 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_440: Ptr32<u8>,
    /// Unknown bytes (0x444..0x448).
    pub _pad_0444: [u8; 0x4],
    /// ptr_448 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_448: Ptr32<u8>,
}
assert_size!(UIRolloverMessage, 0x44c); // merged size 0x44c rounded to 4
assert_offset!(UIRolloverMessage, vfptr, 0x0);
assert_offset!(UIRolloverMessage, field_4, 0x4);
assert_offset!(UIRolloverMessage, field_8, 0x8);
assert_offset!(UIRolloverMessage, field_c, 0xc);
assert_offset!(UIRolloverMessage, embedded_uirollovermessage, 0x1f8);
assert_offset!(UIRolloverMessage, embedded_uirollovermessage_2, 0x210);
assert_offset!(UIRolloverMessage, ptr_438, 0x438);
assert_offset!(UIRolloverMessage, ptr_440, 0x440);
assert_offset!(UIRolloverMessage, ptr_448, 0x448);

/// Merged layout for `UIScrollBar`.
///
/// Size: 0x220 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIScrollBar {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1f9).
    pub _pad_01ec: [u8; 0xd],
    /// flag_1F9 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_1f9: u8,
    /// Unknown bytes (0x1fa..0x1fc).
    pub _pad_01fa: [u8; 0x2],
    /// field_1fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200_f (confidence: low, kind: float, lanes: c-ui).
    pub field_200_f: f32,
    /// Unknown bytes (0x204..0x210).
    pub _pad_0204: [u8; 0xc],
    /// field_210 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_210: u32,
    /// Unknown trailing bytes (0x214..0x220).
    pub _pad_end: [u8; 0xc],
}
assert_size!(UIScrollBar, 0x220); // merged size 0x220 rounded to 4
assert_offset!(UIScrollBar, field_1e4, 0x1e4);
assert_offset!(UIScrollBar, field_1e8, 0x1e8);
assert_offset!(UIScrollBar, flag_1f9, 0x1f9);
assert_offset!(UIScrollBar, field_1fc, 0x1fc);
assert_offset!(UIScrollBar, field_200_f, 0x200);
assert_offset!(UIScrollBar, field_210, 0x210);

/// Merged layout for `UIScrollingMenu`.
///
/// Size: 0x224 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UIScrollingMenu {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// ptr_1FC (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1fc: Ptr32<u8>,
    /// ptr_200 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_200: Ptr32<u8>,
    /// field_204 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// ptr_210 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// Unknown bytes (0x214..0x216).
    pub _pad_0214: [u8; 0x2],
    /// flag_216 (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_216: u8,
    /// Unknown trailing bytes (0x217..0x224).
    pub _pad_end: [u8; 0xd],
}
assert_size!(UIScrollingMenu, 0x224); // merged size 0x224 rounded to 4
assert_offset!(UIScrollingMenu, field_1e4, 0x1e4);
assert_offset!(UIScrollingMenu, field_1e8, 0x1e8);
assert_offset!(UIScrollingMenu, ptr_1fc, 0x1fc);
assert_offset!(UIScrollingMenu, ptr_200, 0x200);
assert_offset!(UIScrollingMenu, field_204, 0x204);
assert_offset!(UIScrollingMenu, ptr_210, 0x210);
assert_offset!(UIScrollingMenu, flag_216, 0x216);

/// Merged layout for `UISelectMenu`.
///
/// Size: 0x22c (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UISelectMenu {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// field_200 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// ptr_210 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// Unknown bytes (0x214..0x224).
    pub _pad_0214: [u8; 0x10],
    /// field_224 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_224: u32,
    /// field_228 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_228: u32,
}
assert_size!(UISelectMenu, 0x22c); // merged size 0x22c rounded to 4
assert_offset!(UISelectMenu, field_1e4, 0x1e4);
assert_offset!(UISelectMenu, field_1e8, 0x1e8);
assert_offset!(UISelectMenu, field_1fc, 0x1fc);
assert_offset!(UISelectMenu, field_200, 0x200);
assert_offset!(UISelectMenu, field_204, 0x204);
assert_offset!(UISelectMenu, ptr_210, 0x210);
assert_offset!(UISelectMenu, field_224, 0x224);
assert_offset!(UISelectMenu, field_228, 0x228);

/// Merged layout for `UITextField`.
///
/// Size: 0x220 (low). Bases: UIFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UITextField {
    /// Unknown bytes (0x0..0x1e4).
    pub _pad_0000: [u8; 0x1e4],
    /// field_1e4 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1e8 (confidence: high, kind: int32?, lanes: c-ui).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1fc).
    pub _pad_01ec: [u8; 0x10],
    /// field_1fc (confidence: low, kind: int32?, lanes: c-ui).
    pub field_1fc: u32,
    /// Unknown bytes (0x200..0x204).
    pub _pad_0200: [u8; 0x4],
    /// field_204 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x20e).
    pub _pad_0208: [u8; 0x6],
    /// flag_20E (confidence: high, kind: bool-or-byte, lanes: c-ui).
    pub flag_20e: u8,
    /// flag_20F (confidence: medium, kind: bool-or-byte, lanes: c-ui).
    pub flag_20f: u8,
    /// ptr_210 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// Unknown trailing bytes (0x214..0x220).
    pub _pad_end: [u8; 0xc],
}
assert_size!(UITextField, 0x220); // merged size 0x220 rounded to 4
assert_offset!(UITextField, field_1e4, 0x1e4);
assert_offset!(UITextField, field_1e8, 0x1e8);
assert_offset!(UITextField, field_1fc, 0x1fc);
assert_offset!(UITextField, field_204, 0x204);
assert_offset!(UITextField, flag_20e, 0x20e);
assert_offset!(UITextField, flag_20f, 0x20f);
assert_offset!(UITextField, ptr_210, 0x210);

/// Merged layout for `UITexture`.
///
/// Size: 0x25c (high). Bases: UILayoutFrame@0x0.
/// Lanes: c-ui.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UITexture {
    /// Unknown bytes (0x0..0x1dc).
    pub _pad_0000: [u8; 0x1dc],
    /// flag_1DC (confidence: low, kind: bool-or-byte, lanes: c-ui).
    pub flag_1dc: u8,
    /// Unknown bytes (0x1dd..0x1e0).
    pub _pad_01dd: [u8; 0x3],
    /// ptr_1E0 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_1e0: Ptr32<u8>,
    /// field_1e4 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1e4: u32,
    /// field_1E8_f (confidence: medium, kind: float, lanes: c-ui).
    pub field_1e8_f: f32,
    /// field_1ec (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1ec: u32,
    /// field_1f0 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_1f0: u32,
    /// ptr_1F4 (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1f4: [u8; 8],
    /// ptr_1FC (confidence: high, kind: pointer, lanes: c-ui).
    pub ptr_1fc: [u8; 8],
    /// ptr_204 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_204: Ptr32<u8>,
    /// Unknown bytes (0x208..0x210).
    pub _pad_0208: [u8; 0x8],
    /// ptr_210 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_210: Ptr32<u8>,
    /// ptr_214 (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_214: Ptr32<u8>,
    /// ptr_218 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_218: Ptr32<u8>,
    /// ptr_21C (confidence: medium, kind: pointer, lanes: c-ui).
    pub ptr_21c: Ptr32<u8>,
    /// ptr_220 (confidence: low, kind: pointer, lanes: c-ui).
    pub ptr_220: Ptr32<u8>,
    /// field_224 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_224: u32,
    /// field_228 (confidence: medium, kind: int32?, lanes: c-ui).
    pub field_228: u32,
    /// Unknown bytes (0x22c..0x230).
    pub _pad_022c: [u8; 0x4],
    /// field_230 (confidence: medium, kind: int16?, lanes: c-ui).
    pub field_230: u16,
    /// Unknown bytes (0x232..0x258).
    pub _pad_0232: [u8; 0x26],
    /// field_258 (confidence: low, kind: int32?, lanes: c-ui).
    pub field_258: u32,
}
assert_size!(UITexture, 0x25c); // merged size 0x25c rounded to 4
assert_offset!(UITexture, flag_1dc, 0x1dc);
assert_offset!(UITexture, ptr_1e0, 0x1e0);
assert_offset!(UITexture, field_1e4, 0x1e4);
assert_offset!(UITexture, field_1e8_f, 0x1e8);
assert_offset!(UITexture, field_1ec, 0x1ec);
assert_offset!(UITexture, field_1f0, 0x1f0);
assert_offset!(UITexture, ptr_1f4, 0x1f4);
assert_offset!(UITexture, ptr_1fc, 0x1fc);
assert_offset!(UITexture, ptr_204, 0x204);
assert_offset!(UITexture, ptr_210, 0x210);
assert_offset!(UITexture, ptr_214, 0x214);
assert_offset!(UITexture, ptr_218, 0x218);
assert_offset!(UITexture, ptr_21c, 0x21c);
assert_offset!(UITexture, ptr_220, 0x220);
assert_offset!(UITexture, field_224, 0x224);
assert_offset!(UITexture, field_228, 0x228);
assert_offset!(UITexture, field_230, 0x230);
assert_offset!(UITexture, field_258, 0x258);
