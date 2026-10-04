//! Foundation types: RAGE base classes, pools, maths and system services.
//!
//! Holds 10 draft layouts: `rage::datBase`, `rage::atReferenceCounter`, `rage::atAny::PlaceHolder`,
//! `rage::sysThreadPool::WorkItem`, `rage::sysTimeManager`, `std::exception`, `CAtdVirtualBase`,
//! the pool records `CPool` and `RAGE pool object`, and the `Matrix34` record seen by the
//! native-handler lanes. Every layout is Inferred; size confidence (the analysis lanes' own rating)
//! is high for 0, medium for 0 and low for 10. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CAtdVirtualBase`.
///
/// Size: 0x10 (low). Bases: none.
/// Lanes: c-misc-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CAtdVirtualBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core,c-misc-a).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: u32, lanes: c-core,c-misc-a).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: u32, lanes: c-core,c-misc-a).
    pub field_8: u32,
    /// field_c (confidence: high, kind: u32, lanes: c-core,c-misc-a).
    pub field_c: u32,
}
assert_size!(CAtdVirtualBase, 0x10); // merged size 0x10 rounded to 4
assert_offset!(CAtdVirtualBase, vfptr, 0x0);
assert_offset!(CAtdVirtualBase, field_4, 0x4);
assert_offset!(CAtdVirtualBase, field_8, 0x8);
assert_offset!(CAtdVirtualBase, field_c, 0xc);

/// Merged layout for `CPool`.
///
/// Size: 0x10 (low). Bases: none.
/// Lanes: n-07.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPool {
    /// entries_base (confidence: high, kind: u32, lanes: n-07).
    pub entries_base: u32,
    /// per_slot_flags_array (confidence: high, kind: flags, lanes: n-07).
    pub per_slot_flags_array: u32,
    /// slot_count (confidence: high, kind: i32, lanes: n-07).
    pub slot_count: u32,
    /// entry_size_bytes (confidence: high, kind: u32, lanes: n-07).
    pub entry_size_bytes: u32,
}
assert_size!(CPool, 0x10); // merged size 0x10 rounded to 4
assert_offset!(CPool, entries_base, 0x0);
assert_offset!(CPool, per_slot_flags_array, 0x4);
assert_offset!(CPool, slot_count, 0x8);
assert_offset!(CPool, entry_size_bytes, 0xc);

/// Merged layout for `Matrix34`.
///
/// Size: 0x3c (low). Bases: none.
/// Lanes: n-05, n-07.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Matrix34 {
    /// right_row (confidence: high, kind: float, lanes: n-05).
    pub right_row: [u8; 12],
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// forward_row (confidence: high, kind: float, lanes: n-05).
    pub forward_row: [u8; 12],
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// up_row (confidence: high, kind: float, lanes: n-05).
    pub up_row: [u8; 12],
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// position (confidence: high, kind: vec3, lanes: n-05,n-07).
    pub position: [f32; 3],
}
assert_size!(Matrix34, 0x3c); // merged size 0x3c rounded to 4
assert_offset!(Matrix34, right_row, 0x0);
assert_offset!(Matrix34, forward_row, 0x10);
assert_offset!(Matrix34, up_row, 0x20);
assert_offset!(Matrix34, position, 0x30);

/// Merged layout for `RAGE pool object`.
///
/// Size: 0x10 (low). Bases: none.
/// Lanes: n-03.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RAGEPoolObject {
    /// compound_part_pool_layout (confidence: medium, kind: u32, lanes: n-03).
    pub compound_part_pool_layout: u32,
    /// compound_part_bitmask (confidence: medium, kind: u32, lanes: n-03).
    pub compound_part_bitmask: u32,
    /// compound_part_count (confidence: medium, kind: i32, lanes: n-03).
    pub compound_part_count: u32,
    /// compound_part_element_size (confidence: medium, kind: u32, lanes: n-03).
    pub compound_part_element_size: u32,
}
assert_size!(RAGEPoolObject, 0x10); // merged size 0x10 rounded to 4
assert_offset!(RAGEPoolObject, compound_part_pool_layout, 0x0);
assert_offset!(RAGEPoolObject, compound_part_bitmask, 0x4);
assert_offset!(RAGEPoolObject, compound_part_count, 0x8);
assert_offset!(RAGEPoolObject, compound_part_element_size, 0xc);

/// Merged layout for `rage::atAny::PlaceHolder`.
///
/// Size: 0x1c (low). Bases: none.
/// Lanes: c-core, via:rage::atAny::Holder<const rage::Matrix34*>, via:rage::atAny::Holder<float>, via:rage::atAny::Holder<int>, via:rage::atAny::Holder<rage::Matrix34>, via:rage::atAny::Holder<rage::Vector3>, via:rage::atAny::Holder<rage::fragInst*>, via:rage::atAny::Holder<rage::phImpact::Iterator>, via:rage::atAny::Holder<rage::phInst*>.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAtAnyPlaceHolder {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// field_+0x4 (confidence: medium, kind: u32?, lanes: c-core,via:rage::atAny::Holder<const rage::Matrix34*>,via:rage::atAny::Holder<float>,via:rage::atAny::Holder<int>,via:rage::atAny::Holder<rage::fragInst*>,via:rage::atAny::Holder<rage::phImpact::Iterator>,via:rage::atAny::Holder<rage::phInst*> moved from siblings:rage::atAny::Holder<const rage::Matrix34*>,rage::atAny::Holder<float>,rage::atAny::Holder<int>,rage::atAny::Holder<rage::fragInst*>,rage::atAny::Holder<rage::phImpact::Iterator>,rage::atAny::Holder<rage::phInst*>).
    pub field_0x4: u32,
    /// Unknown bytes (0x8..0x10).
    pub _pad_0008: [u8; 0x8],
    /// field_+0x10 (confidence: high, kind: u32?, lanes: c-core,via:rage::atAny::Holder<rage::Matrix34>,via:rage::atAny::Holder<rage::Vector3>,via:rage::atAny::Holder<rage::phImpact::Iterator> moved from siblings:rage::atAny::Holder<rage::Matrix34>,rage::atAny::Holder<rage::Vector3>,rage::atAny::Holder<rage::phImpact::Iterator>).
    pub field_0x10: u32,
    /// field_+0x14 (confidence: medium, kind: u32?, lanes: c-core,via:rage::atAny::Holder<rage::Matrix34>,via:rage::atAny::Holder<rage::Vector3> moved from siblings:rage::atAny::Holder<rage::Matrix34>,rage::atAny::Holder<rage::Vector3>).
    pub field_0x14: u32,
    /// field_+0x18 (confidence: medium, kind: u32?, lanes: c-core,via:rage::atAny::Holder<rage::Matrix34>,via:rage::atAny::Holder<rage::Vector3> moved from siblings:rage::atAny::Holder<rage::Matrix34>,rage::atAny::Holder<rage::Vector3>).
    pub field_0x18: u32,
}
assert_size!(RageAtAnyPlaceHolder, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(RageAtAnyPlaceHolder, vfptr, 0x0);
assert_offset!(RageAtAnyPlaceHolder, field_0x4, 0x4);
assert_offset!(RageAtAnyPlaceHolder, field_0x10, 0x10);
assert_offset!(RageAtAnyPlaceHolder, field_0x14, 0x14);
assert_offset!(RageAtAnyPlaceHolder, field_0x18, 0x18);

/// Merged layout for `rage::atReferenceCounter`.
///
/// Size: 0x1a54 (low). Bases: rage::datBase@0x0.
/// Lanes: c-animation, c-core, c-render, via:crExpressionProcessorPooledObject, via:crFrameFilterBoneMask, via:crmtManagerPriority::crFrameFilterWeightCorrection, via:rage::crFrameFilterMover, via:rage::crFrameFilters<7$0A>, via:rage::crmtMotionTree, via:rage::crmtObserver, via:rage::ptxEffectRule, via:rage::ptxEffectRuleStd, via:rage::ptxEmitRuleStd, via:rage::ptxGpuRenderShader, via:rage::ptxGpuUpdateShader, via:rage::ptxModel, via:rage::ptxRule, via:rage::ptxSprite.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAtReferenceCounter {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_+0x4 (confidence: high, kind: u32?, lanes: c-animation,c-core,c-render).
    pub field_0x4: u32,
    /// Unknown bytes (0x8..0x28).
    pub _pad_0008: [u8; 0x20],
    /// bool_28 (confidence: high, kind: bool_or_u8, lanes: c-animation,c-render,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter).
    pub bool_28: u8,
    /// Unknown bytes (0x29..0x34).
    pub _pad_0029: [u8; 0xb],
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crFrameFilters<7$0A>,via:rage::crmtObserver,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::crFrameFilter,rage::crmtObserver,rage::ptxGpuShader).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:crFrameFilterBoneMask,via:rage::crmtObserver,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::crFrameFilter,rage::crmtObserver,rage::ptxGpuShader).
    pub field_38: u32,
    /// field_3c (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEmitRuleStd,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxEmitRule,rage::ptxGpuShader).
    pub field_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x48).
    pub _pad_0040: [u8; 0x8],
    /// field_48 (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxGpuShader).
    pub field_48: Ptr32<u8>,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxGpuShader).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxGpuShader).
    pub field_54: Ptr32<u8>,
    /// Unknown bytes (0x58..0x5c).
    pub _pad_0058: [u8; 0x4],
    /// field_5c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxGpuShader).
    pub field_5c: u32,
    /// Unknown bytes (0x60..0x64).
    pub _pad_0060: [u8; 0x4],
    /// field_64 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxGpuShader).
    pub field_64: u32,
    /// field_68 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxGpuShader).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxGpuUpdateShader moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxGpuShader).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: medium, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxEmitRuleStd moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxEmitRule).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule moved from siblings:rage::crmtObserver,rage::ptxEffectRule).
    pub field_78: u32,
    /// field_7c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule moved from siblings:rage::crmtObserver,rage::ptxEffectRule).
    pub field_7c: u32,
    /// field_80 (confidence: high, kind: pointer, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxRule moved from siblings:rage::crmtObserver,rage::ptxEffectRule,rage::ptxRule).
    pub field_80: Ptr32<u8>,
    /// Unknown bytes (0x84..0x88).
    pub _pad_0084: [u8; 0x4],
    /// field_88 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxRule moved from siblings:rage::crmtObserver,rage::ptxRule).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxRule moved from siblings:rage::crmtObserver,rage::ptxRule).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xf8).
    pub _pad_0094: [u8; 0x64],
    /// field_f8 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRule,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_f8: Ptr32<u8>,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRule,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_100: Ptr32<u8>,
    /// field_104 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxEmitRuleStd,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxEmitRule,rage::ptxRule).
    pub field_104: Ptr32<u8>,
    /// field_108 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEffectRule,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_108: Ptr32<u8>,
    /// Unknown bytes (0x10c..0x114).
    pub _pad_010c: [u8; 0x8],
    /// field_114 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRule,via:rage::ptxEmitRuleStd,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxEmitRule,rage::ptxRule).
    pub field_114: Ptr32<u8>,
    /// Unknown bytes (0x118..0x124).
    pub _pad_0118: [u8; 0xc],
    /// field_124 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxRule moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_124: Ptr32<u8>,
    /// Unknown bytes (0x128..0x12c).
    pub _pad_0128: [u8; 0x4],
    /// field_12c (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxEmitRuleStd moved from siblings:rage::ptxEffectRule,rage::ptxEmitRule).
    pub field_12c: Ptr32<u8>,
    /// Unknown bytes (0x130..0x160).
    pub _pad_0130: [u8; 0x30],
    /// field_160 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_160: Ptr32<u8>,
    /// Unknown bytes (0x164..0x184).
    pub _pad_0164: [u8; 0x20],
    /// field_184 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxSprite moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_184: Ptr32<u8>,
    /// field_188 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxEffectRuleStd,via:rage::ptxSprite moved from siblings:rage::ptxEffectRule,rage::ptxRule).
    pub field_188: Ptr32<u8>,
    /// Unknown bytes (0x18c..0x198).
    pub _pad_018c: [u8; 0xc],
    /// field_198 (confidence: high, kind: u32_or_ptr, lanes: c-animation,c-render,via:rage::crmtObserver,via:rage::ptxEffectRuleStd moved from siblings:rage::crmtObserver,rage::ptxEffectRule).
    pub field_198: u32,
    /// Unknown bytes (0x19c..0x1e4).
    pub _pad_019c: [u8; 0x48],
    /// field_1e4 (confidence: medium, kind: pointer, lanes: c-render,via:rage::ptxEmitRuleStd,via:rage::ptxSprite moved from siblings:rage::ptxEmitRule,rage::ptxRule).
    pub field_1e4: Ptr32<u8>,
    /// Unknown bytes (0x1e8..0x8b0).
    pub _pad_01e8: [u8; 0x6c8],
    /// field_8b0 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_8b0: u32,
    /// field_8b4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_8b4: u32,
    /// ptr_8B8 (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_8b8: Ptr32<u8>,
    /// Unknown bytes (0x8bc..0x116c).
    pub _pad_08bc: [u8; 0x8b0],
    /// field_116c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_116c: u32,
    /// Unknown bytes (0x1170..0x117c).
    pub _pad_1170: [u8; 0xc],
    /// ptr_117C (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_117c: Ptr32<u8>,
    /// Unknown bytes (0x1180..0x118c).
    pub _pad_1180: [u8; 0xc],
    /// ptr_118C (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_118c: Ptr32<u8>,
    /// Unknown bytes (0x1190..0x119c).
    pub _pad_1190: [u8; 0xc],
    /// ptr_119C (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_119c: Ptr32<u8>,
    /// Unknown bytes (0x11a0..0x11ac).
    pub _pad_11a0: [u8; 0xc],
    /// ptr_11AC (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_11ac: Ptr32<u8>,
    /// Unknown bytes (0x11b0..0x11c4).
    pub _pad_11b0: [u8; 0x14],
    /// ptr_11C4 (confidence: high, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_11c4: Ptr32<u8>,
    /// Unknown bytes (0x11c8..0x11dc).
    pub _pad_11c8: [u8; 0x14],
    /// ptr_11DC (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_11dc: u32,
    /// Unknown bytes (0x11e0..0x1a04).
    pub _pad_11e0: [u8; 0x824],
    /// field_1a04 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a04: u32,
    /// Unknown bytes (0x1a08..0x1a18).
    pub _pad_1a08: [u8; 0x10],
    /// field_1a18 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a18: u32,
    /// Unknown bytes (0x1a1c..0x1a28).
    pub _pad_1a1c: [u8; 0xc],
    /// field_1a28 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a28: u32,
    /// field_1a2c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a2c: u32,
    /// field_1a30 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a30: u32,
    /// field_1a34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a34: u32,
    /// field_1a38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a38: u32,
    /// ptr_1A3C (confidence: medium, kind: pointer, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub ptr_1a3c: Ptr32<u8>,
    /// field_1a40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a40: u32,
    /// Unknown bytes (0x1a44..0x1a4c).
    pub _pad_1a44: [u8; 0x8],
    /// field_1a4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a4c: u32,
    /// field_1a50 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection moved from siblings:rage::crExpressionProcessor,rage::crFrameFilter,rage::crFrameFilterN).
    pub field_1a50: u32,
}
assert_size!(RageAtReferenceCounter, 0x1a54); // merged size 0x1a54 rounded to 4
assert_offset!(RageAtReferenceCounter, field_0x4, 0x4);
assert_offset!(RageAtReferenceCounter, bool_28, 0x28);
assert_offset!(RageAtReferenceCounter, field_34, 0x34);
assert_offset!(RageAtReferenceCounter, field_38, 0x38);
assert_offset!(RageAtReferenceCounter, field_3c, 0x3c);
assert_offset!(RageAtReferenceCounter, field_48, 0x48);
assert_offset!(RageAtReferenceCounter, field_50, 0x50);
assert_offset!(RageAtReferenceCounter, field_54, 0x54);
assert_offset!(RageAtReferenceCounter, field_5c, 0x5c);
assert_offset!(RageAtReferenceCounter, field_64, 0x64);
assert_offset!(RageAtReferenceCounter, field_68, 0x68);
assert_offset!(RageAtReferenceCounter, field_70, 0x70);
assert_offset!(RageAtReferenceCounter, field_74, 0x74);
assert_offset!(RageAtReferenceCounter, field_78, 0x78);
assert_offset!(RageAtReferenceCounter, field_7c, 0x7c);
assert_offset!(RageAtReferenceCounter, field_80, 0x80);
assert_offset!(RageAtReferenceCounter, field_88, 0x88);
assert_offset!(RageAtReferenceCounter, field_90, 0x90);
assert_offset!(RageAtReferenceCounter, field_f8, 0xf8);
assert_offset!(RageAtReferenceCounter, field_100, 0x100);
assert_offset!(RageAtReferenceCounter, field_104, 0x104);
assert_offset!(RageAtReferenceCounter, field_108, 0x108);
assert_offset!(RageAtReferenceCounter, field_114, 0x114);
assert_offset!(RageAtReferenceCounter, field_124, 0x124);
assert_offset!(RageAtReferenceCounter, field_12c, 0x12c);
assert_offset!(RageAtReferenceCounter, field_160, 0x160);
assert_offset!(RageAtReferenceCounter, field_184, 0x184);
assert_offset!(RageAtReferenceCounter, field_188, 0x188);
assert_offset!(RageAtReferenceCounter, field_198, 0x198);
assert_offset!(RageAtReferenceCounter, field_1e4, 0x1e4);
assert_offset!(RageAtReferenceCounter, field_8b0, 0x8b0);
assert_offset!(RageAtReferenceCounter, field_8b4, 0x8b4);
assert_offset!(RageAtReferenceCounter, ptr_8b8, 0x8b8);
assert_offset!(RageAtReferenceCounter, field_116c, 0x116c);
assert_offset!(RageAtReferenceCounter, ptr_117c, 0x117c);
assert_offset!(RageAtReferenceCounter, ptr_118c, 0x118c);
assert_offset!(RageAtReferenceCounter, ptr_119c, 0x119c);
assert_offset!(RageAtReferenceCounter, ptr_11ac, 0x11ac);
assert_offset!(RageAtReferenceCounter, ptr_11c4, 0x11c4);
assert_offset!(RageAtReferenceCounter, ptr_11dc, 0x11dc);
assert_offset!(RageAtReferenceCounter, field_1a04, 0x1a04);
assert_offset!(RageAtReferenceCounter, field_1a18, 0x1a18);
assert_offset!(RageAtReferenceCounter, field_1a28, 0x1a28);
assert_offset!(RageAtReferenceCounter, field_1a2c, 0x1a2c);
assert_offset!(RageAtReferenceCounter, field_1a30, 0x1a30);
assert_offset!(RageAtReferenceCounter, field_1a34, 0x1a34);
assert_offset!(RageAtReferenceCounter, field_1a38, 0x1a38);
assert_offset!(RageAtReferenceCounter, ptr_1a3c, 0x1a3c);
assert_offset!(RageAtReferenceCounter, field_1a40, 0x1a40);
assert_offset!(RageAtReferenceCounter, field_1a4c, 0x1a4c);
assert_offset!(RageAtReferenceCounter, field_1a50, 0x1a50);

/// Merged layout for `rage::datBase`.
///
/// Size: 0x830 (low). Bases: none.
/// Lanes: c-animation, c-core, c-misc-b, c-physics, c-render, via:ART::Rockstar::phInstNM, via:atSingleton<rage::rmPtfxManager>, via:crExpressionProcessorPooledObject, via:crFrameFilterBoneMask, via:crmtManagerPriority::crFrameFilterWeightCorrection, via:fragInstNMGta, via:phMaterialMgrGta, via:rage::CPostFX, via:rage::ProceduralTexture, via:rage::ProceduralTextureSkyhat, via:rage::ProceduralTextureVerletWater, via:rage::SkyDome, via:rage::SkyhatMiniNoise, via:rage::atReferenceCounter, via:rage::crFrameFilterMover, via:rage::crmtObserver, via:rage::evtInstance, via:rage::evtSet, via:rage::fragCachePoolManager, via:rage::fragTuneStruct, via:rage::fragType, via:rage::grbTargetManager, via:rage::grcSetup, via:rage::pgDictionary<gtaDrawable>, via:rage::pgDictionary<rage::crAnimation>, via:rage::pgDictionary<rage::phBound>, via:rage::pgDictionary<rage::ptxEffectRule>, via:rage::pgDictionary<rage::ptxEmitRule>, via:rage::pgDictionary<rage::ptxRule>, via:rage::pgDictionary<rage::rmcDrawable>, via:rage::phLevelNew, via:rage::ptxDomain, via:rage::ptxDomainBox, via:rage::ptxDomainCylinder, via:rage::ptxDomainSphere, via:rage::ptxDomainVortex, via:rage::ptxEffectRule, via:rage::ptxEffectRuleStd, via:rage::ptxEmitRuleStd, via:rage::ptxGpuUpdateShader, via:rage::ptxModel, via:rage::ptxModelRulePropList, via:rage::ptxNetObject, via:rage::ptxRule, via:rage::ptxRulePropList, via:rage::ptxSprite, via:rage::ptxSpriteRulePropList, via:rage::rmPtfxManager, via:rage::rmPtfxShaderVar, via:rage::rmPtfxShaderVar_Keyframe.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageDatBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation,c-core,c-misc-b,c-physics).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x8).
    pub _pad_0004: [u8; 0x4],
    /// field_8 (confidence: high, kind: int16?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::fragCachePoolManager,via:rage::grbTargetManager,via:rage::pgDictionary<gtaDrawable>,via:rage::pgDictionary<rage::crAnimation>,via:rage::pgDictionary<rage::phBound>,via:rage::pgDictionary<rage::ptxEffectRule>,via:rage::pgDictionary<rage::ptxEmitRule>,via:rage::pgDictionary<rage::ptxRule>,via:rage::pgDictionary<rage::rmcDrawable>,via:rage::rmcDrawableBase moved from siblings:rage::fragCachePoolManager,rage::grbTargetManager).
    pub field_8: u16,
    /// field_a (confidence: medium, kind: int16?, lanes: c-misc-b,c-physics,via:rage::fragCachePoolManager,via:rage::grbTargetManager moved from siblings:rage::fragCachePoolManager,rage::grbTargetManager).
    pub field_a: u16,
    /// field_+0xc (confidence: high, kind: u32?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render).
    pub field_0xc: u32,
    /// Unknown bytes (0x10..0x13).
    pub _pad_0010: [u8; 0x3],
    /// field_13 (confidence: high, kind: int8/bool?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render,via:phMaterialMgrGta,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::crFrameFilterMover,via:rage::crmtMotionTree,via:rage::crmtObserver,via:rage::ptxEmitRuleStd,via:rage::rmPtfxShaderVar moved from siblings:rage::phMaterialMgr,rage::rmPtfxShaderVar).
    pub field_13: u8,
    /// u16_14 (confidence: high, kind: word?, lanes: c-animation,c-misc-b,c-physics,c-render,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::crFrameFilterMover,via:rage::crmtMotionTree,via:rage::crmtObserver,via:rage::evtInstance,via:rage::ptxEmitRuleStd moved from siblings:rage::atReferenceCounter,rage::evtInstance).
    pub u16_14: u16,
    /// bool_16 (confidence: high, kind: bool/byte?, lanes: c-animation,c-core,c-misc-b,via:rage::crFrameFilterMover,via:rage::evtInstance,via:rage::pgDictionary<gtaDrawable>,via:rage::pgDictionary<rage::crAnimation>,via:rage::pgDictionary<rage::phBound>,via:rage::pgDictionary<rage::ptxEffectRule>,via:rage::pgDictionary<rage::ptxEmitRule>,via:rage::pgDictionary<rage::ptxRule>,via:rage::pgDictionary<rage::rmcDrawable> moved from siblings:rage::atReferenceCounter,rage::evtInstance).
    pub bool_16: u8,
    /// field_17 (confidence: medium, kind: bool/byte?, lanes: c-misc-b,via:atSingleton<rage::rmPtfxManager>,via:rage::evtInstance moved from siblings:rage::evtInstance,rage::rmPtfxManager).
    pub field_17: u8,
    /// Unknown bytes (0x18..0x1a).
    pub _pad_0018: [u8; 0x2],
    /// u16_1A (confidence: high, kind: int16?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render,via:ART::Rockstar::phInstNM,via:phMaterialMgrGta,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::crmtMotionTree,via:rage::crmtObserver,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::phInst,rage::phMaterialMgr).
    pub u16_1a: u16,
    /// Unknown bytes (0x1c..0x1e).
    pub _pad_001c: [u8; 0x2],
    /// field_+0x1e (confidence: high, kind: u16?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::pgDictionary<gtaDrawable>,via:rage::pgDictionary<rage::crAnimation>,via:rage::pgDictionary<rage::phBound>,via:rage::pgDictionary<rage::ptxEffectRule>,via:rage::pgDictionary<rage::ptxEmitRule>,via:rage::pgDictionary<rage::ptxRule>,via:rage::pgDictionary<rage::rmcDrawable>,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader,via:rage::ptxNetObject moved from siblings:rage::pgBase,rage::ptxNetObject).
    pub field_0x1e: u16,
    /// Unknown bytes (0x20..0x24).
    pub _pad_0020: [u8; 0x4],
    /// bool_24 (confidence: high, kind: bool-or-byte, lanes: c-animation,c-misc-b,c-physics,c-render,via:rage::ProceduralTexture,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::crmtObserver,via:rage::phLevelNew,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::ProceduralTexture,rage::atReferenceCounter,rage::phLevelBase).
    pub bool_24: u8,
    /// Unknown bytes (0x25..0x2c).
    pub _pad_0025: [u8; 0x7],
    /// field_+0x2c (confidence: high, kind: u32?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render).
    pub field_0x2c: u32,
    /// field_30 (confidence: high, kind: bool-or-byte, lanes: c-animation,c-misc-b,c-physics,c-render,via:rage::ProceduralTexture,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::grcSetup,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader moved from siblings:rage::ProceduralTexture,rage::grcSetup).
    pub field_30: u8,
    /// Unknown bytes (0x31..0x40).
    pub _pad_0031: [u8; 0xf],
    /// field_40 (confidence: high, kind: double, lanes: c-animation,c-misc-b,c-physics,c-render,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::crmtObserver,via:rage::grcSetup,via:rage::ptxGpuRenderShader,via:rage::ptxGpuUpdateShader,via:rage::rmPtfxManager moved from siblings:rage::grcSetup,rage::rmPtfxManager).
    pub field_40: [u8; 8],
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// ptr_4c (confidence: high, kind: pointer, lanes: c-animation,c-misc-b,c-physics,c-render,via:phMaterialMgrGta,via:rage::ProceduralTexture,via:rage::ProceduralTextureSkyhat,via:rage::ProceduralTextureVerletWater,via:rage::atReferenceCounter,via:rage::crmtObserver,via:rage::ptxEffectRule,via:rage::ptxEmitRuleStd,via:rage::ptxGpuUpdateShader,via:rage::rmPtfxShaderVar_Keyframe moved from siblings:rage::ProceduralTexture,rage::atReferenceCounter,rage::phMaterialMgr,rage::rmPtfxShaderVar).
    pub ptr_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0x60).
    pub _pad_0050: [u8; 0x10],
    /// vfptr_embedded_+0x60 (confidence: high, kind: vtable_ptr, lanes: c-animation,c-core,c-misc-b,c-physics,c-render).
    pub vfptr_embedded_0x60: Ptr32<()>,
    /// Unknown bytes (0x64..0x6c).
    pub _pad_0064: [u8; 0x8],
    /// field_+0x6c (confidence: high, kind: u32?, lanes: c-animation,c-core,c-misc-b,c-physics,c-render).
    pub field_0x6c: u32,
    /// Unknown bytes (0x70..0xbc).
    pub _pad_0070: [u8; 0x4c],
    /// field_bc (confidence: medium, kind: flags, lanes: c-misc-b,c-physics,via:rage::evtSet,via:rage::fragType moved from siblings:rage::evtSet,rage::pgBase).
    pub field_bc: u32,
    /// Unknown bytes (0xc0..0x110).
    pub _pad_00c0: [u8; 0x50],
    /// field_110 (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::SkyhatMiniNoise,via:rage::ptxDomain moved from siblings:rage::ShaderFragment,rage::ptxDomain).
    pub field_110: Ptr32<u8>,
    /// Unknown bytes (0x114..0x120).
    pub _pad_0114: [u8; 0xc],
    /// field_120 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::atReferenceCounter,via:rage::ptxDomain,via:rage::ptxEffectRuleStd,via:rage::ptxRule moved from siblings:rage::ShaderFragment,rage::atReferenceCounter,rage::ptxDomain).
    pub field_120: Ptr32<u8>,
    /// Unknown bytes (0x124..0x130).
    pub _pad_0124: [u8; 0xc],
    /// field_130 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex,via:rage::ptxEffectRuleStd moved from siblings:rage::atReferenceCounter,rage::ptxDomain).
    pub field_130: Ptr32<u8>,
    /// Unknown bytes (0x134..0x138).
    pub _pad_0134: [u8; 0x4],
    /// field_138 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxDomainSphere,via:rage::ptxDomainVortex,via:rage::ptxEffectRuleStd moved from siblings:rage::atReferenceCounter,rage::ptxDomain).
    pub field_138: Ptr32<u8>,
    /// Unknown bytes (0x13c..0x140).
    pub _pad_013c: [u8; 0x4],
    /// field_140 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::ptxDomain).
    pub field_140: Ptr32<u8>,
    /// field_144 (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::grcSetup,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::grcSetup,rage::ptxDomain).
    pub field_144: Ptr32<u8>,
    /// field_148 (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::grcSetup,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxRulePropList,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::grcSetup,rage::ptxDomain,rage::ptxRulePropList).
    pub field_148: Ptr32<u8>,
    /// field_14c (confidence: high, kind: pointer, lanes: c-render,via:rage::grcSetup,via:rage::ptxEffectRuleStd moved from siblings:rage::atReferenceCounter,rage::grcSetup).
    pub field_14c: Ptr32<u8>,
    /// field_150 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::atReferenceCounter,via:rage::grcSetup,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ShaderFragment,rage::atReferenceCounter,rage::grcSetup,rage::ptxDomain).
    pub field_150: Ptr32<u8>,
    /// field_154 (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::grcSetup,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxEffectRuleStd,via:rage::ptxEmitRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::grcSetup,rage::ptxDomain).
    pub field_154: Ptr32<u8>,
    /// field_158 (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::ptxDomainBox,via:rage::ptxDomainCylinder,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::ptxDomain).
    pub field_158: Ptr32<u8>,
    /// Unknown bytes (0x15c..0x164).
    pub _pad_015c: [u8; 0x8],
    /// field_164 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::atReferenceCounter,via:rage::ptxEffectRuleStd,via:rage::ptxEmitRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ShaderFragment,rage::atReferenceCounter).
    pub field_164: Ptr32<u8>,
    /// field_168 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::atReferenceCounter,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ShaderFragment,rage::atReferenceCounter).
    pub field_168: Ptr32<u8>,
    /// Unknown bytes (0x16c..0x170).
    pub _pad_016c: [u8; 0x4],
    /// field_170 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::atReferenceCounter,via:rage::ptxEffectRuleStd,via:rage::ptxModel,via:rage::ptxSprite moved from siblings:rage::ShaderFragment,rage::atReferenceCounter).
    pub field_170: Ptr32<u8>,
    /// Unknown bytes (0x174..0x17c).
    pub _pad_0174: [u8; 0x8],
    /// field_17c (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::ptxEmitRuleStd,via:rage::ptxRulePropList,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_17c: Ptr32<u8>,
    /// field_180 (confidence: high, kind: pointer, lanes: c-render,via:rage::atReferenceCounter,via:rage::ptxEffectRuleStd,via:rage::ptxSprite,via:rage::rmPtfxManager moved from siblings:rage::atReferenceCounter,rage::rmPtfxManager).
    pub field_180: Ptr32<u8>,
    /// Unknown bytes (0x184..0x190).
    pub _pad_0184: [u8; 0xc],
    /// field_190 (confidence: high, kind: pointer, lanes: c-render,via:rage::grcSetup,via:rage::ptxEffectRuleStd,via:rage::rmPtfxManager moved from siblings:rage::atReferenceCounter,rage::grcSetup,rage::rmPtfxManager).
    pub field_190: Ptr32<u8>,
    /// field_194 (confidence: high, kind: pointer, lanes: c-animation,c-render,via:rage::atReferenceCounter,via:rage::crmtObserver,via:rage::grcSetup,via:rage::ptxEffectRuleStd moved from siblings:rage::atReferenceCounter,rage::grcSetup).
    pub field_194: Ptr32<u8>,
    /// Unknown bytes (0x198..0x1b0).
    pub _pad_0198: [u8; 0x18],
    /// field_1b0 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxRulePropList,via:rage::ptxSprite moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_1b0: Ptr32<u8>,
    /// Unknown bytes (0x1b4..0x1f2).
    pub _pad_01b4: [u8; 0x3e],
    /// field_1f2 (confidence: high, kind: bool/byte?, lanes: c-misc-b,c-physics,c-render,via:rage::evtSet,via:rage::fragType moved from siblings:rage::evtSet,rage::pgBase).
    pub field_1f2: u8,
    /// Unknown bytes (0x1f3..0x218).
    pub _pad_01f3: [u8; 0x25],
    /// field_218 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxRulePropList,via:rage::rmPtfxManager moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList,rage::rmPtfxManager).
    pub field_218: Ptr32<u8>,
    /// Unknown bytes (0x21c..0x230).
    pub _pad_021c: [u8; 0x14],
    /// field_230 (confidence: high, kind: bool-or-byte, lanes: c-render,via:rage::SkyhatMiniNoise,via:rage::rmPtfxManager moved from siblings:rage::ShaderFragment,rage::rmPtfxManager).
    pub field_230: u8,
    /// Unknown bytes (0x231..0x234).
    pub _pad_0231: [u8; 0x3],
    /// field_234 (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::ptxSprite,via:rage::rmPtfxManager moved from siblings:rage::atReferenceCounter,rage::rmPtfxManager).
    pub field_234: Ptr32<u8>,
    /// Unknown bytes (0x238..0x24c).
    pub _pad_0238: [u8; 0x14],
    /// field_24c (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::ptxRulePropList,via:rage::rmPtfxManager moved from siblings:rage::ptxRulePropList,rage::rmPtfxManager).
    pub field_24c: Ptr32<u8>,
    /// field_250 (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::SkyDome,via:rage::rmPtfxManager moved from siblings:rage::SkyDome,rage::rmPtfxManager).
    pub field_250: Ptr32<u8>,
    /// Unknown bytes (0x254..0x268).
    pub _pad_0254: [u8; 0x14],
    /// field_268 (confidence: high, kind: pointer, lanes: c-misc-b,c-render,via:rage::CPostFX,via:rage::rmPtfxManager moved from siblings:rage::CPostFX,rage::rmPtfxManager).
    pub field_268: Ptr32<u8>,
    /// Unknown bytes (0x26c..0x280).
    pub _pad_026c: [u8; 0x14],
    /// field_280 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_280: Ptr32<u8>,
    /// Unknown bytes (0x284..0x290).
    pub _pad_0284: [u8; 0xc],
    /// field_290 (confidence: medium, kind: flags, lanes: c-misc-b,c-physics,via:fragInstNMGta,via:rage::CPostFX moved from siblings:rage::CPostFX,rage::phInst).
    pub field_290: u32,
    /// Unknown bytes (0x294..0x2d0).
    pub _pad_0294: [u8; 0x3c],
    /// field_2d0 (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyDome,via:rage::ptxSprite moved from siblings:rage::SkyDome,rage::atReferenceCounter).
    pub field_2d0: Ptr32<u8>,
    /// Unknown bytes (0x2d4..0x2e8).
    pub _pad_02d4: [u8; 0x14],
    /// field_2e8 (confidence: high, kind: pointer, lanes: c-render,via:rage::SkyDome,via:rage::ptxModel,via:rage::ptxRulePropList moved from siblings:rage::SkyDome,rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_2e8: Ptr32<u8>,
    /// Unknown bytes (0x2ec..0x308).
    pub _pad_02ec: [u8; 0x1c],
    /// field_308 (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyDome,via:rage::ptxModel moved from siblings:rage::SkyDome,rage::atReferenceCounter).
    pub field_308: Ptr32<u8>,
    /// Unknown bytes (0x30c..0x31c).
    pub _pad_030c: [u8; 0x10],
    /// field_31c (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxModelRulePropList,via:rage::ptxSprite,via:rage::ptxSpriteRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_31c: Ptr32<u8>,
    /// Unknown bytes (0x320..0x32c).
    pub _pad_0320: [u8; 0xc],
    /// field_32c (confidence: medium, kind: pointer, lanes: c-render,via:rage::SkyDome,via:rage::ptxSprite moved from siblings:rage::SkyDome,rage::atReferenceCounter).
    pub field_32c: Ptr32<u8>,
    /// Unknown bytes (0x330..0x350).
    pub _pad_0330: [u8; 0x20],
    /// field_350 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_350: Ptr32<u8>,
    /// Unknown bytes (0x354..0x384).
    pub _pad_0354: [u8; 0x30],
    /// field_384 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_384: Ptr32<u8>,
    /// Unknown bytes (0x388..0x454).
    pub _pad_0388: [u8; 0xcc],
    /// field_454 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_454: Ptr32<u8>,
    /// Unknown bytes (0x458..0x488).
    pub _pad_0458: [u8; 0x30],
    /// field_488 (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModel,via:rage::ptxModelRulePropList,via:rage::ptxSpriteRulePropList moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList).
    pub field_488: Ptr32<u8>,
    /// Unknown bytes (0x48c..0x4bc).
    pub _pad_048c: [u8; 0x30],
    /// field_4bc (confidence: high, kind: pointer, lanes: c-render,via:rage::ptxModelRulePropList,via:rage::ptxSprite,via:rage::ptxSpriteRulePropList,via:rage::rmPtfxManager moved from siblings:rage::atReferenceCounter,rage::ptxRulePropList,rage::rmPtfxManager).
    pub field_4bc: Ptr32<u8>,
    /// Unknown bytes (0x4c0..0x828).
    pub _pad_04c0: [u8; 0x368],
    /// field_828 (confidence: high, kind: int?, lanes: c-animation,c-physics,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection,via:rage::atReferenceCounter,via:rage::fragTuneStruct moved from siblings:rage::atReferenceCounter,rage::fragTuneStruct).
    pub field_828: u32,
    /// ptr_82c (confidence: high, kind: pointer, lanes: c-animation,c-physics,via:crExpressionProcessorPooledObject,via:crFrameFilterBoneMask,via:crmtManagerPriority::crFrameFilterWeightCorrection,via:rage::atReferenceCounter,via:rage::fragTuneStruct moved from siblings:rage::atReferenceCounter,rage::fragTuneStruct).
    pub ptr_82c: Ptr32<u8>,
}
assert_size!(RageDatBase, 0x830); // merged size 0x830 rounded to 4
assert_offset!(RageDatBase, vfptr, 0x0);
assert_offset!(RageDatBase, field_8, 0x8);
assert_offset!(RageDatBase, field_a, 0xa);
assert_offset!(RageDatBase, field_0xc, 0xc);
assert_offset!(RageDatBase, field_13, 0x13);
assert_offset!(RageDatBase, u16_14, 0x14);
assert_offset!(RageDatBase, bool_16, 0x16);
assert_offset!(RageDatBase, field_17, 0x17);
assert_offset!(RageDatBase, u16_1a, 0x1a);
assert_offset!(RageDatBase, field_0x1e, 0x1e);
assert_offset!(RageDatBase, bool_24, 0x24);
assert_offset!(RageDatBase, field_0x2c, 0x2c);
assert_offset!(RageDatBase, field_30, 0x30);
assert_offset!(RageDatBase, field_40, 0x40);
assert_offset!(RageDatBase, ptr_4c, 0x4c);
assert_offset!(RageDatBase, vfptr_embedded_0x60, 0x60);
assert_offset!(RageDatBase, field_0x6c, 0x6c);
assert_offset!(RageDatBase, field_bc, 0xbc);
assert_offset!(RageDatBase, field_110, 0x110);
assert_offset!(RageDatBase, field_120, 0x120);
assert_offset!(RageDatBase, field_130, 0x130);
assert_offset!(RageDatBase, field_138, 0x138);
assert_offset!(RageDatBase, field_140, 0x140);
assert_offset!(RageDatBase, field_144, 0x144);
assert_offset!(RageDatBase, field_148, 0x148);
assert_offset!(RageDatBase, field_14c, 0x14c);
assert_offset!(RageDatBase, field_150, 0x150);
assert_offset!(RageDatBase, field_154, 0x154);
assert_offset!(RageDatBase, field_158, 0x158);
assert_offset!(RageDatBase, field_164, 0x164);
assert_offset!(RageDatBase, field_168, 0x168);
assert_offset!(RageDatBase, field_170, 0x170);
assert_offset!(RageDatBase, field_17c, 0x17c);
assert_offset!(RageDatBase, field_180, 0x180);
assert_offset!(RageDatBase, field_190, 0x190);
assert_offset!(RageDatBase, field_194, 0x194);
assert_offset!(RageDatBase, field_1b0, 0x1b0);
assert_offset!(RageDatBase, field_1f2, 0x1f2);
assert_offset!(RageDatBase, field_218, 0x218);
assert_offset!(RageDatBase, field_230, 0x230);
assert_offset!(RageDatBase, field_234, 0x234);
assert_offset!(RageDatBase, field_24c, 0x24c);
assert_offset!(RageDatBase, field_250, 0x250);
assert_offset!(RageDatBase, field_268, 0x268);
assert_offset!(RageDatBase, field_280, 0x280);
assert_offset!(RageDatBase, field_290, 0x290);
assert_offset!(RageDatBase, field_2d0, 0x2d0);
assert_offset!(RageDatBase, field_2e8, 0x2e8);
assert_offset!(RageDatBase, field_308, 0x308);
assert_offset!(RageDatBase, field_31c, 0x31c);
assert_offset!(RageDatBase, field_32c, 0x32c);
assert_offset!(RageDatBase, field_350, 0x350);
assert_offset!(RageDatBase, field_384, 0x384);
assert_offset!(RageDatBase, field_454, 0x454);
assert_offset!(RageDatBase, field_488, 0x488);
assert_offset!(RageDatBase, field_4bc, 0x4bc);
assert_offset!(RageDatBase, field_828, 0x828);
assert_offset!(RageDatBase, ptr_82c, 0x82c);

/// Merged layout for `rage::sysThreadPool::WorkItem`.
///
/// Size: 0x148 (low). Bases: none.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSysThreadPoolWorkItem {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x18).
    pub _pad_0004: [u8; 0x14],
    /// field_+0x18 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x18: u32,
    /// Unknown bytes (0x1c..0x38).
    pub _pad_001c: [u8; 0x1c],
    /// flag_+0x38 (confidence: low, kind: u8/bool?, lanes: c-core).
    pub flag_0x38: u8,
    /// Unknown bytes (0x39..0x40).
    pub _pad_0039: [u8; 0x7],
    /// vfptr_embedded_+0x40 (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr_embedded_0x40: Ptr32<()>,
    /// Unknown bytes (0x44..0x138).
    pub _pad_0044: [u8; 0xf4],
    /// field_+0x138 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x138: u32,
    /// field_+0x13c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x13c: u32,
    /// field_+0x140 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x140: u32,
    /// field_+0x144 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x144: u32,
}
assert_size!(RageSysThreadPoolWorkItem, 0x148); // merged size 0x148 rounded to 4
assert_offset!(RageSysThreadPoolWorkItem, vfptr, 0x0);
assert_offset!(RageSysThreadPoolWorkItem, field_0x18, 0x18);
assert_offset!(RageSysThreadPoolWorkItem, flag_0x38, 0x38);
assert_offset!(RageSysThreadPoolWorkItem, vfptr_embedded_0x40, 0x40);
assert_offset!(RageSysThreadPoolWorkItem, field_0x138, 0x138);
assert_offset!(RageSysThreadPoolWorkItem, field_0x13c, 0x13c);
assert_offset!(RageSysThreadPoolWorkItem, field_0x140, 0x140);
assert_offset!(RageSysThreadPoolWorkItem, field_0x144, 0x144);

/// Merged layout for `rage::sysTimeManager`.
///
/// Size: 0x5c (low). Bases: none.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSysTimeManager {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x8).
    pub _pad_0004: [u8; 0x4],
    /// field_+0x8 (confidence: medium, kind: float, lanes: c-core).
    pub field_0x8: f32,
    /// field_+0xc (confidence: medium, kind: float, lanes: c-core).
    pub field_0xc: f32,
    /// field_+0x10 (confidence: medium, kind: float, lanes: c-core).
    pub field_0x10: f32,
    /// field_+0x14 (confidence: medium, kind: float, lanes: c-core).
    pub field_0x14: f32,
    /// field_+0x18 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x18: u32,
    /// field_+0x1c (confidence: low, kind: float, lanes: c-core).
    pub field_0x1c: f32,
    /// field_+0x20 (confidence: low, kind: float, lanes: c-core).
    pub field_0x20: f32,
    /// field_+0x24 (confidence: low, kind: float, lanes: c-core).
    pub field_0x24: f32,
    /// field_+0x28 (confidence: medium, kind: u32?, lanes: c-core).
    pub field_0x28: u32,
    /// field_+0x2c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x2c: u32,
    /// flag_+0x30 (confidence: medium, kind: u8/bool?, lanes: c-core).
    pub flag_0x30: u8,
    /// Unknown bytes (0x31..0x32).
    pub _pad_0031: [u8; 0x1],
    /// flag_+0x32 (confidence: low, kind: u8/bool?, lanes: c-core).
    pub flag_0x32: u8,
    /// Unknown bytes (0x33..0x34).
    pub _pad_0033: [u8; 0x1],
    /// field_+0x34 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x34: u32,
    /// field_+0x38 (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x38: u32,
    /// field_+0x3c (confidence: low, kind: u32?, lanes: c-core).
    pub field_0x3c: u32,
    /// flag_+0x40 (confidence: low, kind: u8/bool?, lanes: c-core).
    pub flag_0x40: u8,
    /// Unknown bytes (0x41..0x4c).
    pub _pad_0041: [u8; 0xb],
    /// field_+0x4c (confidence: medium, kind: float, lanes: c-core).
    pub field_0x4c: f32,
    /// field_+0x50 (confidence: low, kind: float, lanes: c-core).
    pub field_0x50: f32,
    /// field_+0x54 (confidence: low, kind: float, lanes: c-core).
    pub field_0x54: f32,
    /// field_+0x58 (confidence: low, kind: float, lanes: c-core).
    pub field_0x58: f32,
}
assert_size!(RageSysTimeManager, 0x5c); // merged size 0x5c rounded to 4
assert_offset!(RageSysTimeManager, vfptr, 0x0);
assert_offset!(RageSysTimeManager, field_0x8, 0x8);
assert_offset!(RageSysTimeManager, field_0xc, 0xc);
assert_offset!(RageSysTimeManager, field_0x10, 0x10);
assert_offset!(RageSysTimeManager, field_0x14, 0x14);
assert_offset!(RageSysTimeManager, field_0x18, 0x18);
assert_offset!(RageSysTimeManager, field_0x1c, 0x1c);
assert_offset!(RageSysTimeManager, field_0x20, 0x20);
assert_offset!(RageSysTimeManager, field_0x24, 0x24);
assert_offset!(RageSysTimeManager, field_0x28, 0x28);
assert_offset!(RageSysTimeManager, field_0x2c, 0x2c);
assert_offset!(RageSysTimeManager, flag_0x30, 0x30);
assert_offset!(RageSysTimeManager, flag_0x32, 0x32);
assert_offset!(RageSysTimeManager, field_0x34, 0x34);
assert_offset!(RageSysTimeManager, field_0x38, 0x38);
assert_offset!(RageSysTimeManager, field_0x3c, 0x3c);
assert_offset!(RageSysTimeManager, flag_0x40, 0x40);
assert_offset!(RageSysTimeManager, field_0x4c, 0x4c);
assert_offset!(RageSysTimeManager, field_0x50, 0x50);
assert_offset!(RageSysTimeManager, field_0x54, 0x54);
assert_offset!(RageSysTimeManager, field_0x58, 0x58);

/// Merged layout for `std::exception`.
///
/// Size: 0x9 (low). Bases: none.
/// Lanes: c-core.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdException {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-core).
    pub vfptr: Ptr32<()>,
    /// field_+0x4 (confidence: high, kind: u32?, lanes: c-core).
    pub field_0x4: u32,
    /// flag_+0x8 (confidence: medium, kind: u8/bool?, lanes: c-core).
    pub flag_0x8: u8,
    /// Unknown trailing bytes (0x9..0xc).
    pub _pad_end: [u8; 0x3],
}
assert_size!(StdException, 0xc); // merged size 0x9 rounded to 4
assert_offset!(StdException, vfptr, 0x0);
assert_offset!(StdException, field_0x4, 0x4);
assert_offset!(StdException, flag_0x8, 0x8);
