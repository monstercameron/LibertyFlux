//! RAGE animation and motion-tree classes.
//!
//! Holds 22 draft layouts: `rage::cr*` (animations, frames, creature components), `rage::crmt*` and
//! `crmt*` (motion-tree composers, iterators, nodes and requests), and the game's `CAnimTaskInfo`,
//! `CAnimTaskInfoBase` and `CMovementEventHandler`. Every layout is Inferred; size confidence (the
//! analysis lanes' own rating) is high for 0, medium for 15 and low for 7. The conventions are
//! those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CAnimTaskInfo`.
///
/// Size: 0x2a (medium). Bases: CAnimTaskInfoBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CAnimTaskInfo {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// field_24 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// bool_28 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_28: u8,
    /// bool_29 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_29: u8,
    /// Unknown trailing bytes (0x2a..0x2c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CAnimTaskInfo, 0x2c); // merged size 0x2a rounded to 4
assert_offset!(CAnimTaskInfo, field_24, 0x24);
assert_offset!(CAnimTaskInfo, bool_28, 0x28);
assert_offset!(CAnimTaskInfo, bool_29, 0x29);

/// Merged layout for `CAnimTaskInfoBase`.
///
/// Size: 0x24 (low). Bases: none.
/// Lanes: c-animation, c-core, via:CAnimTaskInfo, via:CScriptAnimTaskInfo.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CAnimTaskInfoBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation,c-core,via:CAnimTaskInfo,via:CScriptAnimTaskInfo moved from siblings:CAnimTaskInfo,CScriptAnimTaskInfo).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x14).
    pub _pad_0004: [u8; 0x10],
    /// field_+0x14 (confidence: high, kind: u32?, lanes: c-animation,c-core,via:CAnimTaskInfo,via:CScriptAnimTaskInfo moved from siblings:CAnimTaskInfo,CScriptAnimTaskInfo).
    pub field_0x14: u32,
    /// field_+0x18 (confidence: high, kind: u32?, lanes: c-animation,c-core,via:CAnimTaskInfo,via:CScriptAnimTaskInfo moved from siblings:CAnimTaskInfo,CScriptAnimTaskInfo).
    pub field_0x18: u32,
    /// field_+0x1c (confidence: high, kind: float, lanes: c-animation,c-core,via:CAnimTaskInfo,via:CScriptAnimTaskInfo moved from siblings:CAnimTaskInfo,CScriptAnimTaskInfo).
    pub field_0x1c: f32,
    /// field_+0x20 (confidence: high, kind: u32?, lanes: c-animation,c-core,via:CAnimTaskInfo,via:CScriptAnimTaskInfo moved from siblings:CAnimTaskInfo,CScriptAnimTaskInfo).
    pub field_0x20: u32,
}
assert_size!(CAnimTaskInfoBase, 0x24); // merged size 0x24 rounded to 4
assert_offset!(CAnimTaskInfoBase, vfptr, 0x0);
assert_offset!(CAnimTaskInfoBase, field_0x14, 0x14);
assert_offset!(CAnimTaskInfoBase, field_0x18, 0x18);
assert_offset!(CAnimTaskInfoBase, field_0x1c, 0x1c);
assert_offset!(CAnimTaskInfoBase, field_0x20, 0x20);

/// Merged layout for `CMovementEventHandler`.
///
/// Size: 0x48 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMovementEventHandler {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x18).
    pub _pad_000c: [u8; 0xc],
    /// ptr_18 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x44).
    pub _pad_0020: [u8; 0x24],
    /// field_44 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
}
assert_size!(CMovementEventHandler, 0x48); // merged size 0x48 rounded to 4
assert_offset!(CMovementEventHandler, vfptr, 0x0);
assert_offset!(CMovementEventHandler, ptr_4, 0x4);
assert_offset!(CMovementEventHandler, field_8, 0x8);
assert_offset!(CMovementEventHandler, ptr_18, 0x18);
assert_offset!(CMovementEventHandler, field_1c, 0x1c);
assert_offset!(CMovementEventHandler, field_44, 0x44);

/// Merged layout for `crmtManagerChannel`.
///
/// Size: 0x1a08 (medium). Bases: rage::crmtManagerMixer@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CrmtManagerChannel {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// embedded_crFrameBuffer (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_crframebuffer: u32,
    /// Unknown bytes (0x28..0x38).
    pub _pad_0028: [u8; 0x10],
    /// field_38 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x44).
    pub _pad_003c: [u8; 0x8],
    /// bool_44 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_44: u8,
    /// Unknown bytes (0x45..0x58).
    pub _pad_0045: [u8; 0x13],
    /// ptr_58 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_58: Ptr32<u8>,
    /// Unknown bytes (0x5c..0x30c).
    pub _pad_005c: [u8; 0x2b0],
    /// field_30c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_30c: u32,
    /// Unknown bytes (0x310..0x388).
    pub _pad_0310: [u8; 0x78],
    /// ptr_388 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_388: Ptr32<u8>,
    /// field_38c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_38c: u32,
    /// Unknown bytes (0x390..0x78c).
    pub _pad_0390: [u8; 0x3fc],
    /// field_78c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78c: u32,
    /// Unknown bytes (0x790..0x80c).
    pub _pad_0790: [u8; 0x7c],
    /// ptr_80C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_80c: Ptr32<u8>,
    /// Unknown bytes (0x810..0x824).
    pub _pad_0810: [u8; 0x14],
    /// bool_824 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_824: u8,
    /// Unknown bytes (0x825..0x8b8).
    pub _pad_0825: [u8; 0x93],
    /// ptr_8B8 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_8b8: Ptr32<u8>,
    /// Unknown bytes (0x8bc..0x116c).
    pub _pad_08bc: [u8; 0x8b0],
    /// field_116c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_116c: u32,
    /// Unknown bytes (0x1170..0x117c).
    pub _pad_1170: [u8; 0xc],
    /// field_117c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_117c: u32,
    /// Unknown bytes (0x1180..0x118c).
    pub _pad_1180: [u8; 0xc],
    /// field_118c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_118c: u32,
    /// Unknown bytes (0x1190..0x119c).
    pub _pad_1190: [u8; 0xc],
    /// field_119c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_119c: u32,
    /// Unknown bytes (0x11a0..0x11ac).
    pub _pad_11a0: [u8; 0xc],
    /// field_11ac (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_11ac: u32,
    /// Unknown bytes (0x11b0..0x11c4).
    pub _pad_11b0: [u8; 0x14],
    /// field_11c4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_11c4: u32,
    /// Unknown bytes (0x11c8..0x11dc).
    pub _pad_11c8: [u8; 0x14],
    /// ptr_11DC (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_11dc: Ptr32<u8>,
    /// Unknown bytes (0x11e0..0x1a04).
    pub _pad_11e0: [u8; 0x824],
    /// field_1a04 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1a04: u32,
}
assert_size!(CrmtManagerChannel, 0x1a08); // merged size 0x1a08 rounded to 4
assert_offset!(CrmtManagerChannel, embedded_crframebuffer, 0x24);
assert_offset!(CrmtManagerChannel, field_38, 0x38);
assert_offset!(CrmtManagerChannel, bool_44, 0x44);
assert_offset!(CrmtManagerChannel, ptr_58, 0x58);
assert_offset!(CrmtManagerChannel, field_30c, 0x30c);
assert_offset!(CrmtManagerChannel, ptr_388, 0x388);
assert_offset!(CrmtManagerChannel, field_38c, 0x38c);
assert_offset!(CrmtManagerChannel, field_78c, 0x78c);
assert_offset!(CrmtManagerChannel, ptr_80c, 0x80c);
assert_offset!(CrmtManagerChannel, bool_824, 0x824);
assert_offset!(CrmtManagerChannel, ptr_8b8, 0x8b8);
assert_offset!(CrmtManagerChannel, field_116c, 0x116c);
assert_offset!(CrmtManagerChannel, field_117c, 0x117c);
assert_offset!(CrmtManagerChannel, field_118c, 0x118c);
assert_offset!(CrmtManagerChannel, field_119c, 0x119c);
assert_offset!(CrmtManagerChannel, field_11ac, 0x11ac);
assert_offset!(CrmtManagerChannel, field_11c4, 0x11c4);
assert_offset!(CrmtManagerChannel, ptr_11dc, 0x11dc);
assert_offset!(CrmtManagerChannel, field_1a04, 0x1a04);

/// Merged layout for `crmtRequestSource<0, <r>, <a>, <g>, <e>>`.
///
/// Size: 0x24 (low). Bases: none.
/// Lanes: c-animation, via:rage::crmtRequestExpression, via:rage::crmtRequestExtrapolate, via:rage::crmtRequestFilter, via:rage::crmtRequestInsert.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CrmtRequestSource0RAGE {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// ptr_4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestExpression,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestFilter,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExpression,rage::crmtRequestExtrapolate,rage::crmtRequestFilter,rage::crmtRequestInsert).
    pub ptr_4: u32,
    /// ptr_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestExpression,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestFilter,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExpression,rage::crmtRequestExtrapolate,rage::crmtRequestFilter,rage::crmtRequestInsert).
    pub ptr_8: u32,
    /// embedded_crmtObserver (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestExpression,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestFilter,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExpression,rage::crmtRequestExtrapolate,rage::crmtRequestFilter,rage::crmtRequestInsert).
    pub embedded_crmtobserver: u32,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestExpression,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestFilter,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExpression,rage::crmtRequestExtrapolate,rage::crmtRequestFilter,rage::crmtRequestInsert).
    pub field_14: u32,
    /// ptr_18 (confidence: high, kind: pointer, lanes: c-animation,via:rage::crmtRequestExpression,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestFilter moved from siblings:rage::crmtRequestExpression,rage::crmtRequestExtrapolate,rage::crmtRequestFilter).
    pub ptr_18: Ptr32<u8>,
    /// embedded_crmtRequest (confidence: high, kind: embedded_object, lanes: c-animation,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExtrapolate,rage::crmtRequestInsert).
    pub embedded_crmtrequest: u32,
    /// embedded_crmtObserver (confidence: high, kind: embedded_object, lanes: c-animation,via:rage::crmtRequestExtrapolate,via:rage::crmtRequestInsert moved from siblings:rage::crmtRequestExtrapolate,rage::crmtRequestInsert).
    pub embedded_crmtobserver_2: u32,
}
assert_size!(CrmtRequestSource0RAGE, 0x24); // merged size 0x24 rounded to 4
assert_offset!(CrmtRequestSource0RAGE, ptr_4, 0x4);
assert_offset!(CrmtRequestSource0RAGE, ptr_8, 0x8);
assert_offset!(CrmtRequestSource0RAGE, embedded_crmtobserver, 0xc);
assert_offset!(CrmtRequestSource0RAGE, field_14, 0x14);
assert_offset!(CrmtRequestSource0RAGE, ptr_18, 0x18);
assert_offset!(CrmtRequestSource0RAGE, embedded_crmtrequest, 0x1c);
assert_offset!(CrmtRequestSource0RAGE, embedded_crmtobserver_2, 0x20);

/// Merged layout for `crmtRequestSource<1, <r>, <a>, <g>, <e>>`.
///
/// Size: 0x24 (low). Bases: none.
/// Lanes: c-animation, via:rage::crmtRequestAddSubtract, via:rage::crmtRequestBlend.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CrmtRequestSource1RAGE {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_8: u32,
    /// field_c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_18: u32,
    /// field_1c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub field_1c: u32,
    /// ptr_20 (confidence: high, kind: pointer, lanes: c-animation,via:rage::crmtRequestAddSubtract,via:rage::crmtRequestBlend moved from siblings:rage::crmtRequestAddSubtract,rage::crmtRequestBlend).
    pub ptr_20: Ptr32<u8>,
}
assert_size!(CrmtRequestSource1RAGE, 0x24); // merged size 0x24 rounded to 4
assert_offset!(CrmtRequestSource1RAGE, vfptr, 0x0);
assert_offset!(CrmtRequestSource1RAGE, field_4, 0x4);
assert_offset!(CrmtRequestSource1RAGE, field_8, 0x8);
assert_offset!(CrmtRequestSource1RAGE, field_c, 0xc);
assert_offset!(CrmtRequestSource1RAGE, field_14, 0x14);
assert_offset!(CrmtRequestSource1RAGE, field_18, 0x18);
assert_offset!(CrmtRequestSource1RAGE, field_1c, 0x1c);
assert_offset!(CrmtRequestSource1RAGE, ptr_20, 0x20);

/// Merged layout for `rage::crAnimChannel`.
///
/// Size: 0x38 (medium). Bases: none.
/// Lanes: c-animation, via:rage::crAnimChannelCurveFloat, via:rage::crAnimChannelDeltaFloat, via:rage::crAnimChannelRawBool, via:rage::crAnimChannelRawFloat, via:rage::crAnimChannelRawInt, via:rage::crAnimChannelRawQuaternion, via:rage::crAnimChannelRleInt.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrAnimChannel {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// bool_4 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_4: u8,
    /// bool_5 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_5: u8,
    /// u16_6 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_6: u16,
    /// ptr_8 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
    /// u16_C (confidence: high, kind: u16_or_i16, lanes: c-animation,via:rage::crAnimChannelCurveFloat,via:rage::crAnimChannelRawBool,via:rage::crAnimChannelRawFloat,via:rage::crAnimChannelRawInt,via:rage::crAnimChannelRleInt moved from siblings:rage::crAnimChannelCurveFloat,rage::crAnimChannelRawBool,rage::crAnimChannelRawFloat,rage::crAnimChannelRawInt,rage::crAnimChannelRleInt).
    pub u16_c: u16,
    /// u16_E (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_e: u16,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crAnimChannelDeltaFloat,via:rage::crAnimChannelRawQuaternion moved from siblings:rage::crAnimChannelDeltaFloat,rage::crAnimChannelRawQuaternion).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x2c).
    pub _pad_0028: [u8; 0x4],
    /// field_2c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
}
assert_size!(RageCrAnimChannel, 0x38); // merged size 0x38 rounded to 4
assert_offset!(RageCrAnimChannel, vfptr, 0x0);
assert_offset!(RageCrAnimChannel, bool_4, 0x4);
assert_offset!(RageCrAnimChannel, bool_5, 0x5);
assert_offset!(RageCrAnimChannel, u16_6, 0x6);
assert_offset!(RageCrAnimChannel, ptr_8, 0x8);
assert_offset!(RageCrAnimChannel, u16_c, 0xc);
assert_offset!(RageCrAnimChannel, u16_e, 0xe);
assert_offset!(RageCrAnimChannel, field_10, 0x10);
assert_offset!(RageCrAnimChannel, field_14, 0x14);
assert_offset!(RageCrAnimChannel, field_18, 0x18);
assert_offset!(RageCrAnimChannel, field_20, 0x20);
assert_offset!(RageCrAnimChannel, field_24, 0x24);
assert_offset!(RageCrAnimChannel, field_2c, 0x2c);
assert_offset!(RageCrAnimChannel, field_30, 0x30);
assert_offset!(RageCrAnimChannel, field_34, 0x34);

/// Merged layout for `rage::crAnimation`.
///
/// Size: 0xee (low). Bases: rage::datBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrAnimation {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4: u32,
    /// Unknown bytes (0x8..0x20).
    pub _pad_0008: [u8; 0x18],
    /// field_20 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x28).
    pub _pad_0024: [u8; 0x4],
    /// u16_28 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_28: u16,
    /// Unknown bytes (0x2a..0xe8).
    pub _pad_002a: [u8; 0xbe],
    /// field_e8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_e8: u32,
    /// u16_EC (confidence: low, kind: u16_or_i16, lanes: c-animation).
    pub u16_ec: u16,
    /// Unknown trailing bytes (0xee..0xf0).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RageCrAnimation, 0xf0); // merged size 0xee rounded to 4
assert_offset!(RageCrAnimation, field_4, 0x4);
assert_offset!(RageCrAnimation, field_20, 0x20);
assert_offset!(RageCrAnimation, u16_28, 0x28);
assert_offset!(RageCrAnimation, field_e8, 0xe8);
assert_offset!(RageCrAnimation, u16_ec, 0xec);

/// Merged layout for `rage::crCreatureComponent`.
///
/// Size: 0x52 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrCreatureComponent {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4: u32,
    /// ptr_8 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
    /// ptr_C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// ptr_14 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_14: Ptr32<u8>,
    /// ptr_18 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_18: Ptr32<u8>,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// field_2c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// u16_50 (confidence: low, kind: u16_or_i16, lanes: c-animation).
    pub u16_50: u16,
    /// Unknown trailing bytes (0x52..0x54).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RageCrCreatureComponent, 0x54); // merged size 0x52 rounded to 4
assert_offset!(RageCrCreatureComponent, vfptr, 0x0);
assert_offset!(RageCrCreatureComponent, field_4, 0x4);
assert_offset!(RageCrCreatureComponent, ptr_8, 0x8);
assert_offset!(RageCrCreatureComponent, ptr_c, 0xc);
assert_offset!(RageCrCreatureComponent, field_10, 0x10);
assert_offset!(RageCrCreatureComponent, ptr_14, 0x14);
assert_offset!(RageCrCreatureComponent, ptr_18, 0x18);
assert_offset!(RageCrCreatureComponent, field_1c, 0x1c);
assert_offset!(RageCrCreatureComponent, field_20, 0x20);
assert_offset!(RageCrCreatureComponent, field_24, 0x24);
assert_offset!(RageCrCreatureComponent, field_28, 0x28);
assert_offset!(RageCrCreatureComponent, field_2c, 0x2c);
assert_offset!(RageCrCreatureComponent, field_30, 0x30);
assert_offset!(RageCrCreatureComponent, field_34, 0x34);
assert_offset!(RageCrCreatureComponent, field_38, 0x38);
assert_offset!(RageCrCreatureComponent, field_40, 0x40);
assert_offset!(RageCrCreatureComponent, field_44, 0x44);
assert_offset!(RageCrCreatureComponent, field_48, 0x48);
assert_offset!(RageCrCreatureComponent, u16_50, 0x50);

/// Merged layout for `rage::crFrame`.
///
/// Size: 0x2e (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrFrame {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// ptr_8 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
    /// ptr_C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// ptr_10 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_10: Ptr32<u8>,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// bool_2C (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_2c: u8,
    /// bool_2D (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_2d: u8,
    /// Unknown trailing bytes (0x2e..0x30).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RageCrFrame, 0x30); // merged size 0x2e rounded to 4
assert_offset!(RageCrFrame, vfptr, 0x0);
assert_offset!(RageCrFrame, ptr_4, 0x4);
assert_offset!(RageCrFrame, ptr_8, 0x8);
assert_offset!(RageCrFrame, ptr_c, 0xc);
assert_offset!(RageCrFrame, ptr_10, 0x10);
assert_offset!(RageCrFrame, field_14, 0x14);
assert_offset!(RageCrFrame, field_18, 0x18);
assert_offset!(RageCrFrame, field_1c, 0x1c);
assert_offset!(RageCrFrame, field_20, 0x20);
assert_offset!(RageCrFrame, field_24, 0x24);
assert_offset!(RageCrFrame, field_28, 0x28);
assert_offset!(RageCrFrame, bool_2c, 0x2c);
assert_offset!(RageCrFrame, bool_2d, 0x2d);

/// Merged layout for `rage::crFrameBuffer`.
///
/// Size: 0x1a08 (medium). Bases: rage::crFrame@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrFrameBuffer {
    /// Unknown bytes (0x0..0x38).
    pub _pad_0000: [u8; 0x38],
    /// field_38 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x44).
    pub _pad_003c: [u8; 0x8],
    /// bool_44 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_44: u8,
    /// Unknown bytes (0x45..0x48).
    pub _pad_0045: [u8; 0x3],
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// Unknown bytes (0x50..0x58).
    pub _pad_0050: [u8; 0x8],
    /// ptr_58 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_58: Ptr32<u8>,
    /// Unknown bytes (0x5c..0x8b8).
    pub _pad_005c: [u8; 0x85c],
    /// ptr_8B8 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_8b8: Ptr32<u8>,
    /// Unknown bytes (0x8bc..0x116c).
    pub _pad_08bc: [u8; 0x8b0],
    /// field_116c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_116c: u32,
    /// Unknown bytes (0x1170..0x117c).
    pub _pad_1170: [u8; 0xc],
    /// field_117c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_117c: u32,
    /// Unknown bytes (0x1180..0x118c).
    pub _pad_1180: [u8; 0xc],
    /// field_118c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_118c: u32,
    /// Unknown bytes (0x1190..0x119c).
    pub _pad_1190: [u8; 0xc],
    /// field_119c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_119c: u32,
    /// Unknown bytes (0x11a0..0x11ac).
    pub _pad_11a0: [u8; 0xc],
    /// field_11ac (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_11ac: u32,
    /// Unknown bytes (0x11b0..0x11c4).
    pub _pad_11b0: [u8; 0x14],
    /// field_11c4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_11c4: u32,
    /// Unknown bytes (0x11c8..0x11dc).
    pub _pad_11c8: [u8; 0x14],
    /// ptr_11DC (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_11dc: Ptr32<u8>,
    /// Unknown bytes (0x11e0..0x1a04).
    pub _pad_11e0: [u8; 0x824],
    /// field_1a04 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1a04: u32,
}
assert_size!(RageCrFrameBuffer, 0x1a08); // merged size 0x1a08 rounded to 4
assert_offset!(RageCrFrameBuffer, field_38, 0x38);
assert_offset!(RageCrFrameBuffer, bool_44, 0x44);
assert_offset!(RageCrFrameBuffer, field_48, 0x48);
assert_offset!(RageCrFrameBuffer, field_4c, 0x4c);
assert_offset!(RageCrFrameBuffer, ptr_58, 0x58);
assert_offset!(RageCrFrameBuffer, ptr_8b8, 0x8b8);
assert_offset!(RageCrFrameBuffer, field_116c, 0x116c);
assert_offset!(RageCrFrameBuffer, field_117c, 0x117c);
assert_offset!(RageCrFrameBuffer, field_118c, 0x118c);
assert_offset!(RageCrFrameBuffer, field_119c, 0x119c);
assert_offset!(RageCrFrameBuffer, field_11ac, 0x11ac);
assert_offset!(RageCrFrameBuffer, field_11c4, 0x11c4);
assert_offset!(RageCrFrameBuffer, ptr_11dc, 0x11dc);
assert_offset!(RageCrFrameBuffer, field_1a04, 0x1a04);

/// Merged layout for `rage::crFrameDof`.
///
/// Size: 0x74 (low). Bases: none.
/// Lanes: c-animation, via:rage::crFrameDofQuaternion, via:rage::crFrameDofVector3.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrFrameDof {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// bool_4 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_4: u8,
    /// bool_5 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_5: u8,
    /// u16_6 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_6: u16,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// field_c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: [u8; 8],
    /// field_1c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// embedded_crFrameDofQuaternion (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub embedded_crframedofquaternion: u32,
    /// Unknown bytes (0x24..0x38).
    pub _pad_0024: [u8; 0x14],
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x50).
    pub _pad_003c: [u8; 0x14],
    /// embedded_crFrameDofQuaternion (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub embedded_crframedofquaternion_2: u32,
    /// field_54 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// embedded_crFrameDofVector3 (confidence: high, kind: embedded_object, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub embedded_crframedofvector3: u32,
    /// Unknown bytes (0x64..0x70).
    pub _pad_0064: [u8; 0xc],
    /// embedded_crFrameDofVector3 (confidence: high, kind: embedded_object, lanes: c-animation,via:rage::crFrameDofQuaternion,via:rage::crFrameDofVector3 moved from siblings:rage::crFrameDofQuaternion,rage::crFrameDofVector3).
    pub embedded_crframedofvector3_2: u32,
}
assert_size!(RageCrFrameDof, 0x74); // merged size 0x74 rounded to 4
assert_offset!(RageCrFrameDof, vfptr, 0x0);
assert_offset!(RageCrFrameDof, bool_4, 0x4);
assert_offset!(RageCrFrameDof, bool_5, 0x5);
assert_offset!(RageCrFrameDof, u16_6, 0x6);
assert_offset!(RageCrFrameDof, field_8, 0x8);
assert_offset!(RageCrFrameDof, field_c, 0xc);
assert_offset!(RageCrFrameDof, field_10, 0x10);
assert_offset!(RageCrFrameDof, field_14, 0x14);
assert_offset!(RageCrFrameDof, field_1c, 0x1c);
assert_offset!(RageCrFrameDof, embedded_crframedofquaternion, 0x20);
assert_offset!(RageCrFrameDof, field_38, 0x38);
assert_offset!(RageCrFrameDof, embedded_crframedofquaternion_2, 0x50);
assert_offset!(RageCrFrameDof, field_54, 0x54);
assert_offset!(RageCrFrameDof, field_58, 0x58);
assert_offset!(RageCrFrameDof, embedded_crframedofvector3, 0x60);
assert_offset!(RageCrFrameDof, embedded_crframedofvector3_2, 0x70);

/// Merged layout for `rage::crmtComposer`.
///
/// Size: 0x74 (medium). Bases: rage::crmtIterator@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtComposer {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x20).
    pub _pad_000c: [u8; 0x14],
    /// field_20 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x28).
    pub _pad_0024: [u8; 0x4],
    /// field_28 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// field_64 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_64: u32,
    /// field_68 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// u16_70 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_70: u16,
    /// u16_72 (confidence: low, kind: u16_or_i16, lanes: c-animation).
    pub u16_72: u16,
}
assert_size!(RageCrmtComposer, 0x74); // merged size 0x74 rounded to 4
assert_offset!(RageCrmtComposer, field_4, 0x4);
assert_offset!(RageCrmtComposer, field_8, 0x8);
assert_offset!(RageCrmtComposer, field_20, 0x20);
assert_offset!(RageCrmtComposer, field_28, 0x28);
assert_offset!(RageCrmtComposer, field_30, 0x30);
assert_offset!(RageCrmtComposer, field_34, 0x34);
assert_offset!(RageCrmtComposer, field_38, 0x38);
assert_offset!(RageCrmtComposer, field_40, 0x40);
assert_offset!(RageCrmtComposer, field_44, 0x44);
assert_offset!(RageCrmtComposer, field_48, 0x48);
assert_offset!(RageCrmtComposer, field_50, 0x50);
assert_offset!(RageCrmtComposer, field_54, 0x54);
assert_offset!(RageCrmtComposer, field_58, 0x58);
assert_offset!(RageCrmtComposer, field_60, 0x60);
assert_offset!(RageCrmtComposer, field_64, 0x64);
assert_offset!(RageCrmtComposer, field_68, 0x68);
assert_offset!(RageCrmtComposer, u16_70, 0x70);
assert_offset!(RageCrmtComposer, u16_72, 0x72);

/// Merged layout for `rage::crmtComposerData`.
///
/// Size: 0x1c (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtComposerData {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// field_c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
}
assert_size!(RageCrmtComposerData, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(RageCrmtComposerData, vfptr, 0x0);
assert_offset!(RageCrmtComposerData, field_4, 0x4);
assert_offset!(RageCrmtComposerData, field_8, 0x8);
assert_offset!(RageCrmtComposerData, field_c, 0xc);
assert_offset!(RageCrmtComposerData, field_10, 0x10);
assert_offset!(RageCrmtComposerData, field_14, 0x14);
assert_offset!(RageCrmtComposerData, field_18, 0x18);

/// Merged layout for `rage::crmtComposerOptimized`.
///
/// Size: 0x99 (medium). Bases: rage::crmtComposer@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtComposerOptimized {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// ptr_80 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_80: Ptr32<u8>,
    /// field_84 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// ptr_88 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_88: Ptr32<u8>,
    /// ptr_8C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_8c: Ptr32<u8>,
    /// field_90 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// field_94 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_94: u32,
    /// bool_98 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_98: u8,
    /// Unknown trailing bytes (0x99..0x9c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageCrmtComposerOptimized, 0x9c); // merged size 0x99 rounded to 4
assert_offset!(RageCrmtComposerOptimized, ptr_80, 0x80);
assert_offset!(RageCrmtComposerOptimized, field_84, 0x84);
assert_offset!(RageCrmtComposerOptimized, ptr_88, 0x88);
assert_offset!(RageCrmtComposerOptimized, ptr_8c, 0x8c);
assert_offset!(RageCrmtComposerOptimized, field_90, 0x90);
assert_offset!(RageCrmtComposerOptimized, field_94, 0x94);
assert_offset!(RageCrmtComposerOptimized, bool_98, 0x98);

/// Merged layout for `rage::crmtComposerOptimizedData`.
///
/// Size: 0x1b20 (medium). Bases: rage::crmtComposerData@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtComposerOptimizedData {
    /// Unknown bytes (0x0..0x20).
    pub _pad_0000: [u8; 0x20],
    /// field_20 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x2c).
    pub _pad_0024: [u8; 0x8],
    /// field_2c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// Unknown bytes (0x34..0x3c).
    pub _pad_0034: [u8; 0x8],
    /// ptr_3C (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0xbf0).
    pub _pad_0040: [u8; 0xbb0],
    /// field_bf0 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_bf0: u32,
    /// Unknown bytes (0xbf4..0xe1c).
    pub _pad_0bf4: [u8; 0x228],
    /// ptr_E1C (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_e1c: Ptr32<u8>,
    /// Unknown bytes (0xe20..0xe4c).
    pub _pad_0e20: [u8; 0x2c],
    /// ptr_E4C (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_e4c: Ptr32<u8>,
    /// Unknown bytes (0xe50..0x1b1c).
    pub _pad_0e50: [u8; 0xccc],
    /// ptr_1B1C (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_1b1c: Ptr32<u8>,
}
assert_size!(RageCrmtComposerOptimizedData, 0x1b20); // merged size 0x1b20 rounded to 4
assert_offset!(RageCrmtComposerOptimizedData, field_20, 0x20);
assert_offset!(RageCrmtComposerOptimizedData, field_2c, 0x2c);
assert_offset!(RageCrmtComposerOptimizedData, field_30, 0x30);
assert_offset!(RageCrmtComposerOptimizedData, ptr_3c, 0x3c);
assert_offset!(RageCrmtComposerOptimizedData, field_bf0, 0xbf0);
assert_offset!(RageCrmtComposerOptimizedData, ptr_e1c, 0xe1c);
assert_offset!(RageCrmtComposerOptimizedData, ptr_e4c, 0xe4c);
assert_offset!(RageCrmtComposerOptimizedData, ptr_1b1c, 0x1b1c);

/// Merged layout for `rage::crmtFrameBufferDynamic`.
///
/// Size: 0x74 (medium). Bases: rage::crmtFrameBuffer@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtFrameBufferDynamic {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// field_10 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_20: u32,
    /// field_24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// u16_70 (confidence: low, kind: u16_or_i16, lanes: c-animation).
    pub u16_70: u16,
    /// u16_72 (confidence: low, kind: u16_or_i16, lanes: c-animation).
    pub u16_72: u16,
}
assert_size!(RageCrmtFrameBufferDynamic, 0x74); // merged size 0x74 rounded to 4
assert_offset!(RageCrmtFrameBufferDynamic, field_8, 0x8);
assert_offset!(RageCrmtFrameBufferDynamic, field_10, 0x10);
assert_offset!(RageCrmtFrameBufferDynamic, field_14, 0x14);
assert_offset!(RageCrmtFrameBufferDynamic, field_18, 0x18);
assert_offset!(RageCrmtFrameBufferDynamic, field_1c, 0x1c);
assert_offset!(RageCrmtFrameBufferDynamic, field_20, 0x20);
assert_offset!(RageCrmtFrameBufferDynamic, field_24, 0x24);
assert_offset!(RageCrmtFrameBufferDynamic, field_28, 0x28);
assert_offset!(RageCrmtFrameBufferDynamic, field_30, 0x30);
assert_offset!(RageCrmtFrameBufferDynamic, field_34, 0x34);
assert_offset!(RageCrmtFrameBufferDynamic, field_38, 0x38);
assert_offset!(RageCrmtFrameBufferDynamic, field_40, 0x40);
assert_offset!(RageCrmtFrameBufferDynamic, field_44, 0x44);
assert_offset!(RageCrmtFrameBufferDynamic, field_48, 0x48);
assert_offset!(RageCrmtFrameBufferDynamic, field_50, 0x50);
assert_offset!(RageCrmtFrameBufferDynamic, field_54, 0x54);
assert_offset!(RageCrmtFrameBufferDynamic, field_58, 0x58);
assert_offset!(RageCrmtFrameBufferDynamic, field_60, 0x60);
assert_offset!(RageCrmtFrameBufferDynamic, field_64, 0x64);
assert_offset!(RageCrmtFrameBufferDynamic, field_68, 0x68);
assert_offset!(RageCrmtFrameBufferDynamic, u16_70, 0x70);
assert_offset!(RageCrmtFrameBufferDynamic, u16_72, 0x72);

/// Merged layout for `rage::crmtIterator`.
///
/// Size: 0x28 (low). Bases: none.
/// Lanes: c-animation, via:rage::crmtComposer, via:rage::crmtUpdater.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtIterator {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0xc).
    pub _pad_0004: [u8; 0x8],
    /// ptr_C (confidence: high, kind: pointer, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub ptr_c: Ptr32<u8>,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub field_18: u32,
    /// field_1c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0x24).
    pub _pad_0020: [u8; 0x4],
    /// field_24 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtComposer,via:rage::crmtUpdater moved from siblings:rage::crmtComposer,rage::crmtUpdater).
    pub field_24: u32,
}
assert_size!(RageCrmtIterator, 0x28); // merged size 0x28 rounded to 4
assert_offset!(RageCrmtIterator, vfptr, 0x0);
assert_offset!(RageCrmtIterator, ptr_c, 0xc);
assert_offset!(RageCrmtIterator, field_10, 0x10);
assert_offset!(RageCrmtIterator, field_14, 0x14);
assert_offset!(RageCrmtIterator, field_18, 0x18);
assert_offset!(RageCrmtIterator, field_1c, 0x1c);
assert_offset!(RageCrmtIterator, field_24, 0x24);

/// Merged layout for `rage::crmtNode`.
///
/// Size: 0x39 (low). Bases: none.
/// Lanes: c-animation, via:rage::crmtNodeAddN, via:rage::crmtNodeAnimation, via:rage::crmtNodeBlendN, via:rage::crmtNodeCapture, via:rage::crmtNodeExpression, via:rage::crmtNodeExtrapolate, via:rage::crmtNodeFilter, via:rage::crmtNodeFrame, via:rage::crmtNodeMirror, via:rage::crmtNodeN, via:rage::crmtNodePair, via:rage::crmtNodeProxy.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtNode {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// u16_4 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_4: u16,
    /// u16_6 (confidence: medium, kind: u16_or_i16, lanes: c-animation,via:rage::crmtNodeFrame,via:rage::crmtNodeN,via:rage::crmtNodePair moved from siblings:rage::crmtNodeFrame,rage::crmtNodeParent).
    pub u16_6: u16,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_8: u32,
    /// field_c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAnimation,via:rage::crmtNodeCapture,via:rage::crmtNodeExpression,via:rage::crmtNodeExtrapolate,via:rage::crmtNodeFilter,via:rage::crmtNodeMirror,via:rage::crmtNodeN,via:rage::crmtNodePair,via:rage::crmtNodeProxy moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent,rage::crmtNodeProxy).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAnimation,via:rage::crmtNodeCapture,via:rage::crmtNodeExpression,via:rage::crmtNodeExtrapolate,via:rage::crmtNodeFilter,via:rage::crmtNodeMirror,via:rage::crmtNodeN,via:rage::crmtNodePair,via:rage::crmtNodeProxy moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent,rage::crmtNodeProxy).
    pub field_18: u32,
    /// ptr_1C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_1c: Ptr32<u8>,
    /// bool_20 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_20: u8,
    /// Unknown bytes (0x21..0x24).
    pub _pad_0021: [u8; 0x3],
    /// field_24 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAnimation,via:rage::crmtNodeExtrapolate,via:rage::crmtNodePair,via:rage::crmtNodeProxy moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent,rage::crmtNodeProxy).
    pub field_24: u32,
    /// ptr_28 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAddN,via:rage::crmtNodeAnimation,via:rage::crmtNodeBlendN,via:rage::crmtNodeExtrapolate,via:rage::crmtNodePair moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent).
    pub ptr_28: u32,
    /// field_2c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAnimation,via:rage::crmtNodePair moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent).
    pub field_2c: u32,
    /// f32_30 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:rage::crmtNodeAnimation,via:rage::crmtNodeExtrapolate moved from siblings:rage::crmtNodeAnimation,rage::crmtNodeParent).
    pub f32_30: u32,
    /// field_34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// bool_38 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_38: u8,
    /// Unknown trailing bytes (0x39..0x3c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageCrmtNode, 0x3c); // merged size 0x39 rounded to 4
assert_offset!(RageCrmtNode, vfptr, 0x0);
assert_offset!(RageCrmtNode, u16_4, 0x4);
assert_offset!(RageCrmtNode, u16_6, 0x6);
assert_offset!(RageCrmtNode, field_8, 0x8);
assert_offset!(RageCrmtNode, field_c, 0xc);
assert_offset!(RageCrmtNode, field_10, 0x10);
assert_offset!(RageCrmtNode, field_14, 0x14);
assert_offset!(RageCrmtNode, field_18, 0x18);
assert_offset!(RageCrmtNode, ptr_1c, 0x1c);
assert_offset!(RageCrmtNode, bool_20, 0x20);
assert_offset!(RageCrmtNode, field_24, 0x24);
assert_offset!(RageCrmtNode, ptr_28, 0x28);
assert_offset!(RageCrmtNode, field_2c, 0x2c);
assert_offset!(RageCrmtNode, f32_30, 0x30);
assert_offset!(RageCrmtNode, field_34, 0x34);
assert_offset!(RageCrmtNode, bool_38, 0x38);

/// Merged layout for `rage::crmtRequest`.
///
/// Size: 0x19c (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtRequest {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// ptr_8 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
    /// embedded_crmtObserver (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_crmtobserver: u32,
    /// embedded_crmtObserver (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_crmtobserver_2: u32,
    /// field_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
    /// embedded_crmtRequestInsert (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_crmtrequestinsert: u32,
    /// embedded_crmtObserver (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_crmtobserver_3: u32,
    /// bool_24 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_24: u8,
    /// bool_25 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_25: u8,
    /// Unknown bytes (0x26..0x28).
    pub _pad_0026: [u8; 0x2],
    /// field_28 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// field_2c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// field_5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// field_6c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_6c: u32,
    /// field_70 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78: u32,
    /// field_7c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_7c: u32,
    /// field_80 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_88: u32,
    /// field_8c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// Unknown bytes (0x94..0x194).
    pub _pad_0094: [u8; 0x100],
    /// field_194 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_194: u32,
    /// field_198 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_198: u32,
}
assert_size!(RageCrmtRequest, 0x19c); // merged size 0x19c rounded to 4
assert_offset!(RageCrmtRequest, vfptr, 0x0);
assert_offset!(RageCrmtRequest, ptr_4, 0x4);
assert_offset!(RageCrmtRequest, ptr_8, 0x8);
assert_offset!(RageCrmtRequest, embedded_crmtobserver, 0xc);
assert_offset!(RageCrmtRequest, embedded_crmtobserver_2, 0x10);
assert_offset!(RageCrmtRequest, field_14, 0x14);
assert_offset!(RageCrmtRequest, field_18, 0x18);
assert_offset!(RageCrmtRequest, embedded_crmtrequestinsert, 0x1c);
assert_offset!(RageCrmtRequest, embedded_crmtobserver_3, 0x20);
assert_offset!(RageCrmtRequest, bool_24, 0x24);
assert_offset!(RageCrmtRequest, bool_25, 0x25);
assert_offset!(RageCrmtRequest, field_28, 0x28);
assert_offset!(RageCrmtRequest, field_2c, 0x2c);
assert_offset!(RageCrmtRequest, field_30, 0x30);
assert_offset!(RageCrmtRequest, field_34, 0x34);
assert_offset!(RageCrmtRequest, field_38, 0x38);
assert_offset!(RageCrmtRequest, field_3c, 0x3c);
assert_offset!(RageCrmtRequest, field_40, 0x40);
assert_offset!(RageCrmtRequest, field_44, 0x44);
assert_offset!(RageCrmtRequest, field_48, 0x48);
assert_offset!(RageCrmtRequest, field_4c, 0x4c);
assert_offset!(RageCrmtRequest, field_50, 0x50);
assert_offset!(RageCrmtRequest, field_54, 0x54);
assert_offset!(RageCrmtRequest, field_58, 0x58);
assert_offset!(RageCrmtRequest, field_5c, 0x5c);
assert_offset!(RageCrmtRequest, field_60, 0x60);
assert_offset!(RageCrmtRequest, field_64, 0x64);
assert_offset!(RageCrmtRequest, field_68, 0x68);
assert_offset!(RageCrmtRequest, field_6c, 0x6c);
assert_offset!(RageCrmtRequest, field_70, 0x70);
assert_offset!(RageCrmtRequest, field_74, 0x74);
assert_offset!(RageCrmtRequest, field_78, 0x78);
assert_offset!(RageCrmtRequest, field_7c, 0x7c);
assert_offset!(RageCrmtRequest, field_80, 0x80);
assert_offset!(RageCrmtRequest, field_84, 0x84);
assert_offset!(RageCrmtRequest, field_88, 0x88);
assert_offset!(RageCrmtRequest, field_8c, 0x8c);
assert_offset!(RageCrmtRequest, field_90, 0x90);
assert_offset!(RageCrmtRequest, field_194, 0x194);
assert_offset!(RageCrmtRequest, field_198, 0x198);

/// Merged layout for `rage::crmtRequestBlend`.
///
/// Size: 0x30 (medium). Bases: crmtRequestSource<1, <r>, <a>, <g>, <e>>@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtRequestBlend {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// bool_24 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_24: u8,
    /// Unknown bytes (0x25..0x28).
    pub _pad_0025: [u8; 0x3],
    /// field_28 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// field_2c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
}
assert_size!(RageCrmtRequestBlend, 0x30); // merged size 0x30 rounded to 4
assert_offset!(RageCrmtRequestBlend, bool_24, 0x24);
assert_offset!(RageCrmtRequestBlend, field_28, 0x28);
assert_offset!(RageCrmtRequestBlend, field_2c, 0x2c);

/// Merged layout for `rage::crmtRequestExtrapolate`.
///
/// Size: 0x26 (medium). Bases: crmtRequestSource<0, <r>, <a>, <g>, <e>>@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCrmtRequestExtrapolate {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x24).
    pub _pad_0004: [u8; 0x20],
    /// bool_24 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_24: u8,
    /// bool_25 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_25: u8,
    /// Unknown trailing bytes (0x26..0x28).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RageCrmtRequestExtrapolate, 0x28); // merged size 0x26 rounded to 4
assert_offset!(RageCrmtRequestExtrapolate, vfptr, 0x0);
assert_offset!(RageCrmtRequestExtrapolate, bool_24, 0x24);
assert_offset!(RageCrmtRequestExtrapolate, bool_25, 0x25);
