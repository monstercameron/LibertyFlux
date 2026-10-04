//! Controller input.
//!
//! Holds 3 draft layouts: `rage::ioValue` and the `CControlState` and `Pad` records seen by the
//! native-handler lanes. Every layout is Inferred; size confidence (the analysis lanes' own rating)
//! is high for 0, medium for 0 and low for 3. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CControlState`.
///
/// Size: 0x328e (low). Bases: none.
/// Lanes: n-10.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CControlState {
    /// Unknown bytes (0x0..0x269c).
    pub _pad_0000: [u8; 0x269c],
    /// per_control_base (confidence: medium, kind: u8, lanes: n-10).
    pub per_control_base: u8,
    /// Unknown bytes (0x269d..0x269e).
    pub _pad_269d: [u8; 0x1],
    /// per_control_state (confidence: medium, kind: u8, lanes: n-10).
    pub per_control_state: u8,
    /// Unknown bytes (0x269f..0x328d).
    pub _pad_269f: [u8; 0xbee],
    /// non_zero_joypad_input (confidence: medium, kind: u8, lanes: n-10).
    pub non_zero_joypad_input: u8,
    /// Unknown trailing bytes (0x328e..0x3290).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CControlState, 0x3290); // merged size 0x328e rounded to 4
assert_offset!(CControlState, per_control_base, 0x269c);
assert_offset!(CControlState, per_control_state, 0x269e);
assert_offset!(CControlState, non_zero_joypad_input, 0x328d);

/// Merged layout for `Pad`.
///
/// Size: 0x328e (low). Bases: none.
/// Lanes: n-11.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pad {
    /// Unknown bytes (0x0..0x29fc).
    pub _pad_0000: [u8; 0x29fc],
    /// horn_button_previous_state (confidence: medium, kind: u8, lanes: n-11).
    pub horn_button_previous_state: u8,
    /// Unknown bytes (0x29fd..0x29fe).
    pub _pad_29fd: [u8; 0x1],
    /// horn_button_current_state (confidence: medium, kind: u8, lanes: n-11).
    pub horn_button_current_state: u8,
    /// Unknown bytes (0x29ff..0x328c).
    pub _pad_29ff: [u8; 0x88d],
    /// using_controller (confidence: high, kind: flags, lanes: n-11).
    pub using_controller: u8,
    /// help_message_input_gate (confidence: low, kind: flags, lanes: n-11).
    pub help_message_input_gate: u8,
    /// Unknown trailing bytes (0x328e..0x3290).
    pub _pad_end: [u8; 0x2],
}
assert_size!(Pad, 0x3290); // merged size 0x328e rounded to 4
assert_offset!(Pad, horn_button_previous_state, 0x29fc);
assert_offset!(Pad, horn_button_current_state, 0x29fe);
assert_offset!(Pad, using_controller, 0x328c);
assert_offset!(Pad, help_message_input_gate, 0x328d);

/// Merged layout for `rage::ioValue`.
///
/// Size: 0x3a84 (low). Bases: none.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageIoValue {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// field_4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_1c: Ptr32<u8>,
    /// field_20 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_20: Ptr32<u8>,
    /// field_24 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_24: u8,
    /// Unknown bytes (0x25..0x28).
    pub _pad_0025: [u8; 0x3],
    /// field_28 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_28: Ptr32<u8>,
    /// field_2c (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_30: Ptr32<u8>,
    /// field_34 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_34: Ptr32<u8>,
    /// field_38 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_38: Ptr32<u8>,
    /// field_3c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3c: u32,
    /// field_40 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_58: Ptr32<u8>,
    /// field_5c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_5c: u32,
    /// field_60 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_60: u32,
    /// field_64 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_64: u32,
    /// field_68 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_68: u32,
    /// field_6c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_6c: u32,
    /// Unknown bytes (0x70..0x7b8).
    pub _pad_0070: [u8; 0x748],
    /// field_7b8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_7b8: u32,
    /// Unknown bytes (0x7bc..0x81c).
    pub _pad_07bc: [u8; 0x60],
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable: Ptr32<()>,
    /// Unknown bytes (0x820..0x828).
    pub _pad_0820: [u8; 0x8],
    /// field_828 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_828: u32,
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_2: Ptr32<()>,
    /// Unknown bytes (0x830..0x838).
    pub _pad_0830: [u8; 0x8],
    /// field_838 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_838: u32,
    /// Unknown bytes (0x83c..0x844).
    pub _pad_083c: [u8; 0x8],
    /// field_844 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_844: u8,
    /// Unknown bytes (0x845..0xf70).
    pub _pad_0845: [u8; 0x72b],
    /// field_f70 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_f70: u32,
    /// Unknown bytes (0xf74..0x1728).
    pub _pad_0f74: [u8; 0x7b4],
    /// field_1728 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1728: u32,
    /// Unknown bytes (0x172c..0x1ee0).
    pub _pad_172c: [u8; 0x7b4],
    /// field_1ee0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1ee0: u32,
    /// Unknown bytes (0x1ee4..0x269d).
    pub _pad_1ee4: [u8; 0x7b9],
    /// field_269d (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_269d: [u8; 4],
    /// Unknown bytes (0x26a1..0x3248).
    pub _pad_26a1: [u8; 0xba7],
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_3: Ptr32<()>,
    /// field_324c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_324c: u32,
    /// field_3250 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_3250: u8,
    /// Unknown bytes (0x3251..0x3254).
    pub _pad_3251: [u8; 0x3],
    /// field_3254 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_3254: u32,
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_4: Ptr32<()>,
    /// field_325c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_325c: u32,
    /// field_3260 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_3260: u8,
    /// Unknown bytes (0x3261..0x3264).
    pub _pad_3261: [u8; 0x3],
    /// field_3264 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3264: u32,
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_5: Ptr32<()>,
    /// field_326c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_326c: u32,
    /// field_3270 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_3270: u8,
    /// Unknown bytes (0x3271..0x3274).
    pub _pad_3271: [u8; 0x3],
    /// field_3274 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3274: u32,
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_6: Ptr32<()>,
    /// field_327c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_327c: u32,
    /// field_3280 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_3280: u8,
    /// Unknown bytes (0x3281..0x3284).
    pub _pad_3281: [u8; 0x3],
    /// field_3284 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3284: u32,
    /// Unknown bytes (0x3288..0x328c).
    pub _pad_3288: [u8; 0x4],
    /// field_328c (confidence: low, kind: word?, lanes: c-misc-b).
    pub field_328c: u16,
    /// field_328e (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_328e: u8,
    /// Unknown bytes (0x328f..0x3290).
    pub _pad_328f: [u8; 0x1],
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable_7: Ptr32<()>,
    /// field_3294 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_3294: u32,
    /// field_3298 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_3298: u8,
    /// Unknown bytes (0x3299..0x329c).
    pub _pad_3299: [u8; 0x3],
    /// field_329c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_329c: u32,
    /// field_32a0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_32a0: u32,
    /// Unknown bytes (0x32a4..0x32ac).
    pub _pad_32a4: [u8; 0x8],
    /// field_32ac (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_32ac: u32,
    /// Unknown bytes (0x32b0..0x3a80).
    pub _pad_32b0: [u8; 0x7d0],
    /// field_3a80 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3a80: u32,
}
assert_size!(RageIoValue, 0x3a84); // merged size 0x3a84 rounded to 4
assert_offset!(RageIoValue, vftable, 0x0);
assert_offset!(RageIoValue, field_4, 0x4);
assert_offset!(RageIoValue, field_8, 0x8);
assert_offset!(RageIoValue, field_c, 0xc);
assert_offset!(RageIoValue, field_10, 0x10);
assert_offset!(RageIoValue, field_14, 0x14);
assert_offset!(RageIoValue, field_18, 0x18);
assert_offset!(RageIoValue, field_1c, 0x1c);
assert_offset!(RageIoValue, field_20, 0x20);
assert_offset!(RageIoValue, field_24, 0x24);
assert_offset!(RageIoValue, field_28, 0x28);
assert_offset!(RageIoValue, field_2c, 0x2c);
assert_offset!(RageIoValue, field_30, 0x30);
assert_offset!(RageIoValue, field_34, 0x34);
assert_offset!(RageIoValue, field_38, 0x38);
assert_offset!(RageIoValue, field_3c, 0x3c);
assert_offset!(RageIoValue, field_40, 0x40);
assert_offset!(RageIoValue, field_44, 0x44);
assert_offset!(RageIoValue, field_48, 0x48);
assert_offset!(RageIoValue, field_4c, 0x4c);
assert_offset!(RageIoValue, field_50, 0x50);
assert_offset!(RageIoValue, field_54, 0x54);
assert_offset!(RageIoValue, field_58, 0x58);
assert_offset!(RageIoValue, field_5c, 0x5c);
assert_offset!(RageIoValue, field_60, 0x60);
assert_offset!(RageIoValue, field_64, 0x64);
assert_offset!(RageIoValue, field_68, 0x68);
assert_offset!(RageIoValue, field_6c, 0x6c);
assert_offset!(RageIoValue, field_7b8, 0x7b8);
assert_offset!(RageIoValue, embedded_vftable, 0x81c);
assert_offset!(RageIoValue, field_828, 0x828);
assert_offset!(RageIoValue, embedded_vftable_2, 0x82c);
assert_offset!(RageIoValue, field_838, 0x838);
assert_offset!(RageIoValue, field_844, 0x844);
assert_offset!(RageIoValue, field_f70, 0xf70);
assert_offset!(RageIoValue, field_1728, 0x1728);
assert_offset!(RageIoValue, field_1ee0, 0x1ee0);
assert_offset!(RageIoValue, field_269d, 0x269d);
assert_offset!(RageIoValue, embedded_vftable_3, 0x3248);
assert_offset!(RageIoValue, field_324c, 0x324c);
assert_offset!(RageIoValue, field_3250, 0x3250);
assert_offset!(RageIoValue, field_3254, 0x3254);
assert_offset!(RageIoValue, embedded_vftable_4, 0x3258);
assert_offset!(RageIoValue, field_325c, 0x325c);
assert_offset!(RageIoValue, field_3260, 0x3260);
assert_offset!(RageIoValue, field_3264, 0x3264);
assert_offset!(RageIoValue, embedded_vftable_5, 0x3268);
assert_offset!(RageIoValue, field_326c, 0x326c);
assert_offset!(RageIoValue, field_3270, 0x3270);
assert_offset!(RageIoValue, field_3274, 0x3274);
assert_offset!(RageIoValue, embedded_vftable_6, 0x3278);
assert_offset!(RageIoValue, field_327c, 0x327c);
assert_offset!(RageIoValue, field_3280, 0x3280);
assert_offset!(RageIoValue, field_3284, 0x3284);
assert_offset!(RageIoValue, field_328c, 0x328c);
assert_offset!(RageIoValue, field_328e, 0x328e);
assert_offset!(RageIoValue, embedded_vftable_7, 0x3290);
assert_offset!(RageIoValue, field_3294, 0x3294);
assert_offset!(RageIoValue, field_3298, 0x3298);
assert_offset!(RageIoValue, field_329c, 0x329c);
assert_offset!(RageIoValue, field_32a0, 0x32a0);
assert_offset!(RageIoValue, field_32ac, 0x32ac);
assert_offset!(RageIoValue, field_3a80, 0x3a80);
