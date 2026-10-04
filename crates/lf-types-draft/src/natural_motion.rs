//! NaturalMotion behaviour engine (`ART::` and `NMutils::` namespaces).
//!
//! Holds 29 draft layouts: The `ART::` engine, plug-in and transform classes, the
//! `ART::Rockstar::NmRs*` effectors and behaviour tasks (`NmRsCBU*`) with their shared base
//! `ART::Rockstar::CBUTaskBase`, and the `NMutils::` memory streams. Every layout is Inferred; size
//! confidence (the analysis lanes' own rating) is high for 0, medium for 22 and low for 7. The
//! conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `ART::ARTFeedbackInterface`.
///
/// Size: 0x194 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTARTFeedbackInterface {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation,c-misc-a).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-misc-a).
    pub field_4: u32,
    /// Unknown bytes (0x8..0xc).
    pub _pad_0008: [u8; 0x4],
    /// bool_C (confidence: medium, kind: bool_or_u8, lanes: c-animation,c-misc-a).
    pub bool_c: u8,
    /// Unknown bytes (0xd..0x18c).
    pub _pad_000d: [u8; 0x17f],
    /// field_18c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-misc-a).
    pub field_18c: u32,
    /// field_190 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,c-misc-a).
    pub field_190: u32,
}
assert_size!(ARTARTFeedbackInterface, 0x194); // merged size 0x194 rounded to 4
assert_offset!(ARTARTFeedbackInterface, vfptr, 0x0);
assert_offset!(ARTARTFeedbackInterface, field_4, 0x4);
assert_offset!(ARTARTFeedbackInterface, bool_c, 0xc);
assert_offset!(ARTARTFeedbackInterface, field_18c, 0x18c);
assert_offset!(ARTARTFeedbackInterface, field_190, 0x190);

/// Merged layout for `ART::Engine`.
///
/// Size: 0x99 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTEngine {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine: u32,
    /// ptr_C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_2: u32,
    /// field_14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_3: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_4: u32,
    /// field_24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x40).
    pub _pad_0028: [u8; 0x18],
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// field_5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x70).
    pub _pad_0064: [u8; 0xc],
    /// field_70 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_88: u32,
    /// field_8c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// ptr_94 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_94: Ptr32<u8>,
    /// bool_98 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_98: u8,
    /// Unknown trailing bytes (0x99..0x9c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(ARTEngine, 0x9c); // merged size 0x99 rounded to 4
assert_offset!(ARTEngine, vfptr, 0x0);
assert_offset!(ARTEngine, ptr_4, 0x4);
assert_offset!(ARTEngine, embedded_nmrsengine, 0x8);
assert_offset!(ARTEngine, ptr_c, 0xc);
assert_offset!(ARTEngine, embedded_nmrsengine_2, 0x10);
assert_offset!(ARTEngine, field_14, 0x14);
assert_offset!(ARTEngine, embedded_nmrsengine_3, 0x18);
assert_offset!(ARTEngine, field_1c, 0x1c);
assert_offset!(ARTEngine, embedded_nmrsengine_4, 0x20);
assert_offset!(ARTEngine, field_24, 0x24);
assert_offset!(ARTEngine, field_40, 0x40);
assert_offset!(ARTEngine, field_44, 0x44);
assert_offset!(ARTEngine, field_48, 0x48);
assert_offset!(ARTEngine, field_4c, 0x4c);
assert_offset!(ARTEngine, field_50, 0x50);
assert_offset!(ARTEngine, field_54, 0x54);
assert_offset!(ARTEngine, f32_58, 0x58);
assert_offset!(ARTEngine, field_5c, 0x5c);
assert_offset!(ARTEngine, field_60, 0x60);
assert_offset!(ARTEngine, field_70, 0x70);
assert_offset!(ARTEngine, field_74, 0x74);
assert_offset!(ARTEngine, field_78, 0x78);
assert_offset!(ARTEngine, field_80, 0x80);
assert_offset!(ARTEngine, field_84, 0x84);
assert_offset!(ARTEngine, field_88, 0x88);
assert_offset!(ARTEngine, field_8c, 0x8c);
assert_offset!(ARTEngine, field_90, 0x90);
assert_offset!(ARTEngine, ptr_94, 0x94);
assert_offset!(ARTEngine, bool_98, 0x98);

/// Merged layout for `ART::IncomingTransformProcessor`.
///
/// Size: 0x99 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTIncomingTransformProcessor {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine: u32,
    /// ptr_C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_2: u32,
    /// field_14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_3: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_4: u32,
    /// field_24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x40).
    pub _pad_0028: [u8; 0x18],
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// field_5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x70).
    pub _pad_0064: [u8; 0xc],
    /// field_70 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_88: u32,
    /// field_8c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// ptr_94 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_94: Ptr32<u8>,
    /// bool_98 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_98: u8,
    /// Unknown trailing bytes (0x99..0x9c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(ARTIncomingTransformProcessor, 0x9c); // merged size 0x99 rounded to 4
assert_offset!(ARTIncomingTransformProcessor, vfptr, 0x0);
assert_offset!(ARTIncomingTransformProcessor, ptr_4, 0x4);
assert_offset!(ARTIncomingTransformProcessor, embedded_nmrsengine, 0x8);
assert_offset!(ARTIncomingTransformProcessor, ptr_c, 0xc);
assert_offset!(ARTIncomingTransformProcessor, embedded_nmrsengine_2, 0x10);
assert_offset!(ARTIncomingTransformProcessor, field_14, 0x14);
assert_offset!(ARTIncomingTransformProcessor, embedded_nmrsengine_3, 0x18);
assert_offset!(ARTIncomingTransformProcessor, field_1c, 0x1c);
assert_offset!(ARTIncomingTransformProcessor, embedded_nmrsengine_4, 0x20);
assert_offset!(ARTIncomingTransformProcessor, field_24, 0x24);
assert_offset!(ARTIncomingTransformProcessor, field_40, 0x40);
assert_offset!(ARTIncomingTransformProcessor, field_44, 0x44);
assert_offset!(ARTIncomingTransformProcessor, field_48, 0x48);
assert_offset!(ARTIncomingTransformProcessor, field_4c, 0x4c);
assert_offset!(ARTIncomingTransformProcessor, field_50, 0x50);
assert_offset!(ARTIncomingTransformProcessor, field_54, 0x54);
assert_offset!(ARTIncomingTransformProcessor, f32_58, 0x58);
assert_offset!(ARTIncomingTransformProcessor, field_5c, 0x5c);
assert_offset!(ARTIncomingTransformProcessor, field_60, 0x60);
assert_offset!(ARTIncomingTransformProcessor, field_70, 0x70);
assert_offset!(ARTIncomingTransformProcessor, field_74, 0x74);
assert_offset!(ARTIncomingTransformProcessor, field_78, 0x78);
assert_offset!(ARTIncomingTransformProcessor, field_80, 0x80);
assert_offset!(ARTIncomingTransformProcessor, field_84, 0x84);
assert_offset!(ARTIncomingTransformProcessor, field_88, 0x88);
assert_offset!(ARTIncomingTransformProcessor, field_8c, 0x8c);
assert_offset!(ARTIncomingTransformProcessor, field_90, 0x90);
assert_offset!(ARTIncomingTransformProcessor, ptr_94, 0x94);
assert_offset!(ARTIncomingTransformProcessor, bool_98, 0x98);

/// Merged layout for `ART::PluginManager`.
///
/// Size: 0xc (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTPluginManager {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// ptr_8 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
}
assert_size!(ARTPluginManager, 0xc); // merged size 0xc rounded to 4
assert_offset!(ARTPluginManager, vfptr, 0x0);
assert_offset!(ARTPluginManager, ptr_4, 0x4);
assert_offset!(ARTPluginManager, ptr_8, 0x8);

/// Merged layout for `ART::PluginRegistry`.
///
/// Size: 0x14 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTPluginRegistry {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4: u32,
    /// ptr_8 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_8: Ptr32<u8>,
    /// field_c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_c: u32,
    /// field_10 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
}
assert_size!(ARTPluginRegistry, 0x14); // merged size 0x14 rounded to 4
assert_offset!(ARTPluginRegistry, vfptr, 0x0);
assert_offset!(ARTPluginRegistry, field_4, 0x4);
assert_offset!(ARTPluginRegistry, ptr_8, 0x8);
assert_offset!(ARTPluginRegistry, field_c, 0xc);
assert_offset!(ARTPluginRegistry, field_10, 0x10);

/// Merged layout for `ART::Rockstar::CBUTaskBase`.
///
/// Size: 0xbfd (low). Bases: none.
/// Lanes: c-animation, via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive, via:ART::Rockstar::NmRsCBUBodyBalance, via:ART::Rockstar::NmRsCBUBodyFoetal, via:ART::Rockstar::NmRsCBUBodyWrithe, via:ART::Rockstar::NmRsCBUBraceForImpact, via:ART::Rockstar::NmRsCBUCatchFall, via:ART::Rockstar::NmRsCBUDynamicBalancer, via:ART::Rockstar::NmRsCBUFallOverWall, via:ART::Rockstar::NmRsCBUFlinch, via:ART::Rockstar::NmRsCBUGrab, via:ART::Rockstar::NmRsCBUHeadLook, via:ART::Rockstar::NmRsCBUHighFall, via:ART::Rockstar::NmRsCBUPedal, via:ART::Rockstar::NmRsCBUPointArm, via:ART::Rockstar::NmRsCBURollDownStairs, via:ART::Rockstar::NmRsCBURollUp, via:ART::Rockstar::NmRsCBUShot, via:ART::Rockstar::NmRsCBUSpineTwist.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarCBUTaskBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_4: u32,
    /// field_8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// ptr_14 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_14: u32,
    /// f32_18 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot).
    pub f32_18: u32,
    /// ptr_1C (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_1c: u32,
    /// ptr_20 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_20: u32,
    /// f32_24 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_24: u32,
    /// field_28 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_28: u32,
    /// field_2c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp).
    pub field_2c: u32,
    /// ptr_30 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_30: u32,
    /// Unknown bytes (0x34..0x48).
    pub _pad_0034: [u8; 0x14],
    /// bool_48 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs moved from siblings:ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs).
    pub bool_48: u8,
    /// bool_49 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs moved from siblings:ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs).
    pub bool_49: u8,
    /// Unknown bytes (0x4a..0x54).
    pub _pad_004a: [u8; 0xa],
    /// f32_54 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_54: u32,
    /// Unknown bytes (0x58..0x5c).
    pub _pad_0058: [u8; 0x4],
    /// f32_5C (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_5c: u32,
    /// Unknown bytes (0x60..0x64).
    pub _pad_0060: [u8; 0x4],
    /// f32_64 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_64: u32,
    /// Unknown bytes (0x68..0x6c).
    pub _pad_0068: [u8; 0x4],
    /// bool_6C (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBURollUp moved from siblings:ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBURollUp).
    pub bool_6c: u8,
    /// Unknown bytes (0x6d..0x70).
    pub _pad_006d: [u8; 0x3],
    /// ptr_70 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_70: u32,
    /// ptr_74 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_74: u32,
    /// ptr_78 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_78: u32,
    /// field_7c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_7c: u32,
    /// ptr_80 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_80: u32,
    /// f32_84 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_84: u32,
    /// f32_88 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_88: u32,
    /// f32_8C (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_8c: u32,
    /// ptr_90 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_90: u32,
    /// ptr_94 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHeadLook,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHeadLook,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_94: u32,
    /// bool_98 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUHeadLook moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUHeadLook).
    pub bool_98: u8,
    /// Unknown bytes (0x99..0xa0).
    pub _pad_0099: [u8; 0x7],
    /// ptr_A0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_a0: u32,
    /// f32_A4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub f32_a4: u32,
    /// bool_A8 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUFlinch moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUFlinch).
    pub bool_a8: u8,
    /// Unknown bytes (0xa9..0xac).
    pub _pad_00a9: [u8; 0x3],
    /// field_ac (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_ac: u32,
    /// field_b0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_b0: u32,
    /// bool_B4 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub bool_b4: u8,
    /// Unknown bytes (0xb5..0xb8).
    pub _pad_00b5: [u8; 0x3],
    /// ptr_B8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal moved from siblings:ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal).
    pub ptr_b8: u32,
    /// ptr_BC (confidence: high, kind: pointer, lanes: c-animation,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal moved from siblings:ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal).
    pub ptr_bc: Ptr32<u8>,
    /// f32_C0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot).
    pub f32_c0: u32,
    /// ptr_C4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot).
    pub ptr_c4: u32,
    /// ptr_C8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot).
    pub ptr_c8: u32,
    /// field_cc (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot).
    pub field_cc: u32,
    /// field_d0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUShot).
    pub field_d0: u32,
    /// field_d4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub field_d4: u32,
    /// field_d8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub field_d8: u32,
    /// field_dc (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_dc: u32,
    /// field_e0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUGrab).
    pub field_e0: u32,
    /// field_e4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_e4: u32,
    /// bool_E8 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUPointArm moved from siblings:ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUPointArm).
    pub bool_e8: u8,
    /// Unknown bytes (0xe9..0xf0).
    pub _pad_00e9: [u8; 0x7],
    /// field_f0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_f0: u32,
    /// field_f4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub field_f4: u32,
    /// field_f8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// ptr_100 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_100: u32,
    /// ptr_104 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub ptr_104: u32,
    /// field_108 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub field_108: u32,
    /// field_10c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact).
    pub field_10c: u32,
    /// ptr_110 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUGrab).
    pub ptr_110: u32,
    /// field_114 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUGrab).
    pub field_114: u32,
    /// field_118 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_118: u32,
    /// field_11c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_11c: u32,
    /// f32_120 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUGrab).
    pub f32_120: u32,
    /// field_124 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_124: u32,
    /// field_128 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_128: u32,
    /// field_12c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUGrab).
    pub field_12c: u32,
    /// ptr_130 (confidence: high, kind: pointer, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUShot).
    pub ptr_130: Ptr32<u8>,
    /// field_134 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUShot).
    pub field_134: u32,
    /// ptr_138 (confidence: high, kind: pointer, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUShot).
    pub ptr_138: Ptr32<u8>,
    /// field_13c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUShot).
    pub field_13c: u32,
    /// ptr_140 (confidence: high, kind: pointer, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUShot).
    pub ptr_140: Ptr32<u8>,
    /// bool_144 (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBraceForImpact moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBraceForImpact).
    pub bool_144: u8,
    /// Unknown bytes (0x145..0x160).
    pub _pad_0145: [u8; 0x1b],
    /// field_160 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUSpineTwist).
    pub field_160: u32,
    /// field_164 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUPointArm moved from siblings:ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUPointArm).
    pub field_164: u32,
    /// field_168 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUShot).
    pub field_168: u32,
    /// Unknown bytes (0x16c..0x18c).
    pub _pad_016c: [u8; 0x20],
    /// field_18c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBURollUp moved from siblings:ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBURollUp).
    pub field_18c: u32,
    /// Unknown bytes (0x190..0x1c0).
    pub _pad_0190: [u8; 0x30],
    /// field_1c0 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUShot).
    pub field_1c0: u32,
    /// Unknown bytes (0x1c4..0x220).
    pub _pad_01c4: [u8; 0x5c],
    /// field_220 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollDownStairs,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollDownStairs,ART::Rockstar::NmRsCBUShot).
    pub field_220: u32,
    /// Unknown bytes (0x224..0x280).
    pub _pad_0224: [u8; 0x5c],
    /// ptr_280 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBURollDownStairs moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBURollDownStairs).
    pub ptr_280: u32,
    /// Unknown bytes (0x284..0x2d4).
    pub _pad_0284: [u8; 0x50],
    /// ptr_2D4 (confidence: medium, kind: pointer, lanes: c-animation,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyFoetal,via:ART::Rockstar::NmRsCBUBraceForImpact,via:ART::Rockstar::NmRsCBUCatchFall,via:ART::Rockstar::NmRsCBUFallOverWall,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUHighFall,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBUPointArm,via:ART::Rockstar::NmRsCBUShot,via:ART::Rockstar::NmRsCBUSpineTwist moved from siblings:ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyFoetal,ART::Rockstar::NmRsCBUBraceForImpact,ART::Rockstar::NmRsCBUCatchFall,ART::Rockstar::NmRsCBUFallOverWall,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUHighFall,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBUPointArm,ART::Rockstar::NmRsCBUShot,ART::Rockstar::NmRsCBUSpineTwist).
    pub ptr_2d4: Ptr32<u8>,
    /// Unknown bytes (0x2d8..0x338).
    pub _pad_02d8: [u8; 0x60],
    /// field_338 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot).
    pub field_338: u32,
    /// field_33c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,via:ART::Rockstar::NmRsCBUBodyBalance,via:ART::Rockstar::NmRsCBUBodyWrithe,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUPedal,via:ART::Rockstar::NmRsCBURollUp,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUArmsWindmillAdaptive,ART::Rockstar::NmRsCBUBodyBalance,ART::Rockstar::NmRsCBUBodyWrithe,ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUPedal,ART::Rockstar::NmRsCBURollUp,ART::Rockstar::NmRsCBUShot).
    pub field_33c: u32,
    /// Unknown bytes (0x340..0x350).
    pub _pad_0340: [u8; 0x10],
    /// ptr_350 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUFlinch,via:ART::Rockstar::NmRsCBUHighFall moved from siblings:ART::Rockstar::NmRsCBUFlinch,ART::Rockstar::NmRsCBUHighFall).
    pub ptr_350: u32,
    /// Unknown bytes (0x354..0x9c0).
    pub _pad_0354: [u8; 0x66c],
    /// field_9c0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUGrab).
    pub field_9c0: u32,
    /// field_9c4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUGrab).
    pub field_9c4: u32,
    /// field_9c8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUGrab moved from siblings:ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUGrab).
    pub field_9c8: u32,
    /// Unknown bytes (0x9cc..0xbf0).
    pub _pad_09cc: [u8; 0x224],
    /// field_bf0 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_bf0: u32,
    /// field_bf4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_bf4: u32,
    /// field_bf8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_bf8: u32,
    /// bool_BFC (confidence: high, kind: bool_or_u8, lanes: c-animation,via:ART::Rockstar::NmRsCBUDynamicBalancer,via:ART::Rockstar::NmRsCBUGrab,via:ART::Rockstar::NmRsCBUShot moved from siblings:ART::Rockstar::NmRsCBUDynamicBalancer,ART::Rockstar::NmRsCBUGrab,ART::Rockstar::NmRsCBUShot).
    pub bool_bfc: u8,
    /// Unknown trailing bytes (0xbfd..0xc00).
    pub _pad_end: [u8; 0x3],
}
assert_size!(ARTRockstarCBUTaskBase, 0xc00); // merged size 0xbfd rounded to 4
assert_offset!(ARTRockstarCBUTaskBase, vfptr, 0x0);
assert_offset!(ARTRockstarCBUTaskBase, ptr_4, 0x4);
assert_offset!(ARTRockstarCBUTaskBase, field_8, 0x8);
assert_offset!(ARTRockstarCBUTaskBase, field_c, 0xc);
assert_offset!(ARTRockstarCBUTaskBase, field_10, 0x10);
assert_offset!(ARTRockstarCBUTaskBase, ptr_14, 0x14);
assert_offset!(ARTRockstarCBUTaskBase, f32_18, 0x18);
assert_offset!(ARTRockstarCBUTaskBase, ptr_1c, 0x1c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_20, 0x20);
assert_offset!(ARTRockstarCBUTaskBase, f32_24, 0x24);
assert_offset!(ARTRockstarCBUTaskBase, field_28, 0x28);
assert_offset!(ARTRockstarCBUTaskBase, field_2c, 0x2c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_30, 0x30);
assert_offset!(ARTRockstarCBUTaskBase, bool_48, 0x48);
assert_offset!(ARTRockstarCBUTaskBase, bool_49, 0x49);
assert_offset!(ARTRockstarCBUTaskBase, f32_54, 0x54);
assert_offset!(ARTRockstarCBUTaskBase, f32_5c, 0x5c);
assert_offset!(ARTRockstarCBUTaskBase, f32_64, 0x64);
assert_offset!(ARTRockstarCBUTaskBase, bool_6c, 0x6c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_70, 0x70);
assert_offset!(ARTRockstarCBUTaskBase, ptr_74, 0x74);
assert_offset!(ARTRockstarCBUTaskBase, ptr_78, 0x78);
assert_offset!(ARTRockstarCBUTaskBase, field_7c, 0x7c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_80, 0x80);
assert_offset!(ARTRockstarCBUTaskBase, f32_84, 0x84);
assert_offset!(ARTRockstarCBUTaskBase, f32_88, 0x88);
assert_offset!(ARTRockstarCBUTaskBase, f32_8c, 0x8c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_90, 0x90);
assert_offset!(ARTRockstarCBUTaskBase, ptr_94, 0x94);
assert_offset!(ARTRockstarCBUTaskBase, bool_98, 0x98);
assert_offset!(ARTRockstarCBUTaskBase, ptr_a0, 0xa0);
assert_offset!(ARTRockstarCBUTaskBase, f32_a4, 0xa4);
assert_offset!(ARTRockstarCBUTaskBase, bool_a8, 0xa8);
assert_offset!(ARTRockstarCBUTaskBase, field_ac, 0xac);
assert_offset!(ARTRockstarCBUTaskBase, field_b0, 0xb0);
assert_offset!(ARTRockstarCBUTaskBase, bool_b4, 0xb4);
assert_offset!(ARTRockstarCBUTaskBase, ptr_b8, 0xb8);
assert_offset!(ARTRockstarCBUTaskBase, ptr_bc, 0xbc);
assert_offset!(ARTRockstarCBUTaskBase, f32_c0, 0xc0);
assert_offset!(ARTRockstarCBUTaskBase, ptr_c4, 0xc4);
assert_offset!(ARTRockstarCBUTaskBase, ptr_c8, 0xc8);
assert_offset!(ARTRockstarCBUTaskBase, field_cc, 0xcc);
assert_offset!(ARTRockstarCBUTaskBase, field_d0, 0xd0);
assert_offset!(ARTRockstarCBUTaskBase, field_d4, 0xd4);
assert_offset!(ARTRockstarCBUTaskBase, field_d8, 0xd8);
assert_offset!(ARTRockstarCBUTaskBase, field_dc, 0xdc);
assert_offset!(ARTRockstarCBUTaskBase, field_e0, 0xe0);
assert_offset!(ARTRockstarCBUTaskBase, field_e4, 0xe4);
assert_offset!(ARTRockstarCBUTaskBase, bool_e8, 0xe8);
assert_offset!(ARTRockstarCBUTaskBase, field_f0, 0xf0);
assert_offset!(ARTRockstarCBUTaskBase, field_f4, 0xf4);
assert_offset!(ARTRockstarCBUTaskBase, field_f8, 0xf8);
assert_offset!(ARTRockstarCBUTaskBase, ptr_100, 0x100);
assert_offset!(ARTRockstarCBUTaskBase, ptr_104, 0x104);
assert_offset!(ARTRockstarCBUTaskBase, field_108, 0x108);
assert_offset!(ARTRockstarCBUTaskBase, field_10c, 0x10c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_110, 0x110);
assert_offset!(ARTRockstarCBUTaskBase, field_114, 0x114);
assert_offset!(ARTRockstarCBUTaskBase, field_118, 0x118);
assert_offset!(ARTRockstarCBUTaskBase, field_11c, 0x11c);
assert_offset!(ARTRockstarCBUTaskBase, f32_120, 0x120);
assert_offset!(ARTRockstarCBUTaskBase, field_124, 0x124);
assert_offset!(ARTRockstarCBUTaskBase, field_128, 0x128);
assert_offset!(ARTRockstarCBUTaskBase, field_12c, 0x12c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_130, 0x130);
assert_offset!(ARTRockstarCBUTaskBase, field_134, 0x134);
assert_offset!(ARTRockstarCBUTaskBase, ptr_138, 0x138);
assert_offset!(ARTRockstarCBUTaskBase, field_13c, 0x13c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_140, 0x140);
assert_offset!(ARTRockstarCBUTaskBase, bool_144, 0x144);
assert_offset!(ARTRockstarCBUTaskBase, field_160, 0x160);
assert_offset!(ARTRockstarCBUTaskBase, field_164, 0x164);
assert_offset!(ARTRockstarCBUTaskBase, field_168, 0x168);
assert_offset!(ARTRockstarCBUTaskBase, field_18c, 0x18c);
assert_offset!(ARTRockstarCBUTaskBase, field_1c0, 0x1c0);
assert_offset!(ARTRockstarCBUTaskBase, field_220, 0x220);
assert_offset!(ARTRockstarCBUTaskBase, ptr_280, 0x280);
assert_offset!(ARTRockstarCBUTaskBase, ptr_2d4, 0x2d4);
assert_offset!(ARTRockstarCBUTaskBase, field_338, 0x338);
assert_offset!(ARTRockstarCBUTaskBase, field_33c, 0x33c);
assert_offset!(ARTRockstarCBUTaskBase, ptr_350, 0x350);
assert_offset!(ARTRockstarCBUTaskBase, field_9c0, 0x9c0);
assert_offset!(ARTRockstarCBUTaskBase, field_9c4, 0x9c4);
assert_offset!(ARTRockstarCBUTaskBase, field_9c8, 0x9c8);
assert_offset!(ARTRockstarCBUTaskBase, field_bf0, 0xbf0);
assert_offset!(ARTRockstarCBUTaskBase, field_bf4, 0xbf4);
assert_offset!(ARTRockstarCBUTaskBase, field_bf8, 0xbf8);
assert_offset!(ARTRockstarCBUTaskBase, bool_bfc, 0xbfc);

/// Merged layout for `ART::Rockstar::NmRs1DofEffector`.
///
/// Size: 0x2bc (low). Bases: ART::Rockstar::NmRsEffectorBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRs1DofEffector {
    /// Unknown bytes (0x0..0xf0).
    pub _pad_0000: [u8; 0xf0],
    /// field_f0 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_f0: [u8; 8],
    /// field_f8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_f8: [u8; 8],
    /// field_100 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_100: [u8; 8],
    /// Unknown bytes (0x108..0x10c).
    pub _pad_0108: [u8; 0x4],
    /// field_10c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_10c: u32,
    /// Unknown bytes (0x110..0x128).
    pub _pad_0110: [u8; 0x18],
    /// field_128 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_128: u32,
    /// field_12c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_12c: u32,
    /// field_130 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_130: u32,
    /// Unknown bytes (0x134..0x2b0).
    pub _pad_0134: [u8; 0x17c],
    /// field_2b0 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2b0: u32,
    /// field_2b4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2b4: u32,
    /// field_2b8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2b8: u32,
}
assert_size!(ARTRockstarNmRs1DofEffector, 0x2bc); // merged size 0x2bc rounded to 4
assert_offset!(ARTRockstarNmRs1DofEffector, field_f0, 0xf0);
assert_offset!(ARTRockstarNmRs1DofEffector, field_f8, 0xf8);
assert_offset!(ARTRockstarNmRs1DofEffector, field_100, 0x100);
assert_offset!(ARTRockstarNmRs1DofEffector, field_10c, 0x10c);
assert_offset!(ARTRockstarNmRs1DofEffector, field_128, 0x128);
assert_offset!(ARTRockstarNmRs1DofEffector, field_12c, 0x12c);
assert_offset!(ARTRockstarNmRs1DofEffector, field_130, 0x130);
assert_offset!(ARTRockstarNmRs1DofEffector, field_2b0, 0x2b0);
assert_offset!(ARTRockstarNmRs1DofEffector, field_2b4, 0x2b4);
assert_offset!(ARTRockstarNmRs1DofEffector, field_2b8, 0x2b8);

/// Merged layout for `ART::Rockstar::NmRs3DofEffector`.
///
/// Size: 0x190 (low). Bases: ART::Rockstar::NmRsEffectorBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRs3DofEffector {
    /// Unknown bytes (0x0..0xf0).
    pub _pad_0000: [u8; 0xf0],
    /// ptr_F0 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_f0: Ptr32<u8>,
    /// field_f4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_f4: u32,
    /// field_f8 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// ptr_100 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_100: Ptr32<u8>,
    /// field_104 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_104: u32,
    /// Unknown bytes (0x108..0x10c).
    pub _pad_0108: [u8; 0x4],
    /// bool_10C (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_10c: u8,
    /// bool_10D (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_10d: u8,
    /// bool_10E (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_10e: u8,
    /// Unknown bytes (0x10f..0x14c).
    pub _pad_010f: [u8; 0x3d],
    /// field_14c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_14c: u32,
    /// field_150 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_150: u32,
    /// field_154 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_154: u32,
    /// field_158 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_158: u32,
    /// ptr_15C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_15c: Ptr32<u8>,
    /// ptr_160 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_160: Ptr32<u8>,
    /// field_164 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_164: u32,
    /// field_168 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_168: u32,
    /// field_16c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_16c: u32,
    /// Unknown bytes (0x170..0x174).
    pub _pad_0170: [u8; 0x4],
    /// field_174 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_174: u32,
    /// field_178 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_178: u32,
    /// field_17c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_17c: u32,
    /// field_180 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_180: u32,
    /// field_184 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_184: u32,
    /// field_188 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_188: u32,
    /// field_18c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_18c: u32,
}
assert_size!(ARTRockstarNmRs3DofEffector, 0x190); // merged size 0x190 rounded to 4
assert_offset!(ARTRockstarNmRs3DofEffector, ptr_f0, 0xf0);
assert_offset!(ARTRockstarNmRs3DofEffector, field_f4, 0xf4);
assert_offset!(ARTRockstarNmRs3DofEffector, field_f8, 0xf8);
assert_offset!(ARTRockstarNmRs3DofEffector, ptr_100, 0x100);
assert_offset!(ARTRockstarNmRs3DofEffector, field_104, 0x104);
assert_offset!(ARTRockstarNmRs3DofEffector, bool_10c, 0x10c);
assert_offset!(ARTRockstarNmRs3DofEffector, bool_10d, 0x10d);
assert_offset!(ARTRockstarNmRs3DofEffector, bool_10e, 0x10e);
assert_offset!(ARTRockstarNmRs3DofEffector, field_14c, 0x14c);
assert_offset!(ARTRockstarNmRs3DofEffector, field_150, 0x150);
assert_offset!(ARTRockstarNmRs3DofEffector, field_154, 0x154);
assert_offset!(ARTRockstarNmRs3DofEffector, field_158, 0x158);
assert_offset!(ARTRockstarNmRs3DofEffector, ptr_15c, 0x15c);
assert_offset!(ARTRockstarNmRs3DofEffector, ptr_160, 0x160);
assert_offset!(ARTRockstarNmRs3DofEffector, field_164, 0x164);
assert_offset!(ARTRockstarNmRs3DofEffector, field_168, 0x168);
assert_offset!(ARTRockstarNmRs3DofEffector, field_16c, 0x16c);
assert_offset!(ARTRockstarNmRs3DofEffector, field_174, 0x174);
assert_offset!(ARTRockstarNmRs3DofEffector, field_178, 0x178);
assert_offset!(ARTRockstarNmRs3DofEffector, field_17c, 0x17c);
assert_offset!(ARTRockstarNmRs3DofEffector, field_180, 0x180);
assert_offset!(ARTRockstarNmRs3DofEffector, field_184, 0x184);
assert_offset!(ARTRockstarNmRs3DofEffector, field_188, 0x188);
assert_offset!(ARTRockstarNmRs3DofEffector, field_18c, 0x18c);

/// Merged layout for `ART::Rockstar::NmRsCBUArmsWindmillAdaptive`.
///
/// Size: 0x340 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUArmsWindmillAdaptive {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// ptr_40 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_40: Ptr32<u8>,
    /// ptr_44 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_44: Ptr32<u8>,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// field_4c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// ptr_50 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_50: Ptr32<u8>,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// field_68 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown trailing bytes (0x6c..0x340).
    pub _pad_end: [u8; 0x2d4],
}
assert_size!(ARTRockstarNmRsCBUArmsWindmillAdaptive, 0x340); // merged size 0x340 rounded to 4
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, ptr_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, ptr_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, ptr_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, f32_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUArmsWindmillAdaptive, field_68, 0x68);

/// Merged layout for `ART::Rockstar::NmRsCBUBodyBalance`.
///
/// Size: 0x340 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUBodyBalance {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// ptr_38 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_38: Ptr32<u8>,
    /// field_3c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// Unknown bytes (0x44..0x9c).
    pub _pad_0044: [u8; 0x58],
    /// field_9c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_9c: u32,
    /// Unknown bytes (0xa0..0xaa).
    pub _pad_00a0: [u8; 0xa],
    /// bool_AA (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_aa: u8,
    /// Unknown bytes (0xab..0xec).
    pub _pad_00ab: [u8; 0x41],
    /// field_ec (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_ec: u32,
    /// Unknown bytes (0xf0..0xfc).
    pub _pad_00f0: [u8; 0xc],
    /// field_fc (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_fc: u32,
    /// Unknown trailing bytes (0x100..0x340).
    pub _pad_end: [u8; 0x240],
}
assert_size!(ARTRockstarNmRsCBUBodyBalance, 0x340); // merged size 0x340 rounded to 4
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, ptr_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_9c, 0x9c);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, bool_aa, 0xaa);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_ec, 0xec);
assert_offset!(ARTRockstarNmRsCBUBodyBalance, field_fc, 0xfc);

/// Merged layout for `ART::Rockstar::NmRsCBUBodyFoetal`.
///
/// Size: 0x2dc (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUBodyFoetal {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// field_3c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// field_40 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// field_4c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x16c).
    pub _pad_005c: [u8; 0x110],
    /// field_16c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_16c: u32,
    /// Unknown bytes (0x170..0x2b8).
    pub _pad_0170: [u8; 0x148],
    /// field_2b8 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_2b8: u32,
    /// Unknown bytes (0x2bc..0x2d8).
    pub _pad_02bc: [u8; 0x1c],
    /// field_2d8 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_2d8: u32,
}
assert_size!(ARTRockstarNmRsCBUBodyFoetal, 0x2dc); // merged size 0x2dc rounded to 4
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_16c, 0x16c);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_2b8, 0x2b8);
assert_offset!(ARTRockstarNmRsCBUBodyFoetal, field_2d8, 0x2d8);

/// Merged layout for `ART::Rockstar::NmRsCBUBodyWrithe`.
///
/// Size: 0x340 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUBodyWrithe {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// f32_34 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_34: f32,
    /// f32_38 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_38: f32,
    /// f32_3C (confidence: medium, kind: float, lanes: c-animation).
    pub f32_3c: f32,
    /// f32_40 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_40: f32,
    /// f32_44 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_44: f32,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// f32_4C (confidence: medium, kind: float, lanes: c-animation).
    pub f32_4c: f32,
    /// f32_50 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_50: f32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// f32_60 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_60: f32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// f32_68 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_68: f32,
    /// Unknown trailing bytes (0x6c..0x340).
    pub _pad_end: [u8; 0x2d4],
}
assert_size!(ARTRockstarNmRsCBUBodyWrithe, 0x340); // merged size 0x340 rounded to 4
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUBodyWrithe, f32_68, 0x68);

/// Merged layout for `ART::Rockstar::NmRsCBUDynamicBalancer`.
///
/// Size: 0xbfd (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUDynamicBalancer {
    /// Unknown bytes (0x0..0x1c8).
    pub _pad_0000: [u8; 0x1c8],
    /// field_1c8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c8: u32,
    /// Unknown bytes (0x1cc..0x9d0).
    pub _pad_01cc: [u8; 0x804],
    /// field_9d0 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9d0: u32,
    /// field_9d4 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9d4: u32,
    /// field_9d8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_9d8: u32,
    /// f32_9DC (confidence: medium, kind: float, lanes: c-animation).
    pub f32_9dc: f32,
    /// f32_9E0 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_9e0: f32,
    /// field_9e4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_9e4: u32,
    /// field_9e8 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9e8: u32,
    /// field_9ec (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9ec: u32,
    /// f32_9F0 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_9f0: f32,
    /// ptr_9F4 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_9f4: Ptr32<u8>,
    /// ptr_9F8 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_9f8: Ptr32<u8>,
    /// field_9fc (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_9fc: u32,
    /// Unknown bytes (0xa00..0xa10).
    pub _pad_0a00: [u8; 0x10],
    /// field_a10 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_a10: u32,
    /// field_a14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a14: u32,
    /// field_a18 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a18: u32,
    /// field_a1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a1c: u32,
    /// field_a20 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a20: u32,
    /// field_a24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a24: u32,
    /// bool_A28 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_a28: u8,
    /// Unknown bytes (0xa29..0xa30).
    pub _pad_0a29: [u8; 0x7],
    /// field_a30 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_a30: u32,
    /// field_a34 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_a34: u32,
    /// Unknown bytes (0xa38..0xac8).
    pub _pad_0a38: [u8; 0x90],
    /// field_ac8 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_ac8: u32,
    /// Unknown bytes (0xacc..0xb5c).
    pub _pad_0acc: [u8; 0x90],
    /// field_b5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_b5c: u32,
    /// Unknown trailing bytes (0xb60..0xc00).
    pub _pad_end: [u8; 0xa0],
}
assert_size!(ARTRockstarNmRsCBUDynamicBalancer, 0xc00); // merged size 0xbfd rounded to 4
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_1c8, 0x1c8);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9d0, 0x9d0);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9d4, 0x9d4);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9d8, 0x9d8);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, f32_9dc, 0x9dc);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, f32_9e0, 0x9e0);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9e4, 0x9e4);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9e8, 0x9e8);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9ec, 0x9ec);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, f32_9f0, 0x9f0);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, ptr_9f4, 0x9f4);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, ptr_9f8, 0x9f8);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_9fc, 0x9fc);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a10, 0xa10);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a14, 0xa14);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a18, 0xa18);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a1c, 0xa1c);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a20, 0xa20);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a24, 0xa24);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, bool_a28, 0xa28);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a30, 0xa30);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_a34, 0xa34);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_ac8, 0xac8);
assert_offset!(ARTRockstarNmRsCBUDynamicBalancer, field_b5c, 0xb5c);

/// Merged layout for `ART::Rockstar::NmRsCBUFlinch`.
///
/// Size: 0x354 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUFlinch {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// field_3c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// f32_40 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_40: f32,
    /// bool_44 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_44: u8,
    /// Unknown bytes (0x45..0x4c).
    pub _pad_0045: [u8; 0x7],
    /// field_4c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// Unknown bytes (0x50..0x9c).
    pub _pad_0050: [u8; 0x4c],
    /// field_9c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_9c: u32,
    /// Unknown trailing bytes (0xa0..0x354).
    pub _pad_end: [u8; 0x2b4],
}
assert_size!(ARTRockstarNmRsCBUFlinch, 0x354); // merged size 0x354 rounded to 4
assert_offset!(ARTRockstarNmRsCBUFlinch, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUFlinch, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUFlinch, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUFlinch, f32_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUFlinch, bool_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUFlinch, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUFlinch, field_9c, 0x9c);

/// Merged layout for `ART::Rockstar::NmRsCBUGrab`.
///
/// Size: 0xbfd (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUGrab {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
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
    /// Unknown bytes (0x48..0x50).
    pub _pad_0048: [u8; 0x8],
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// field_68 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x9c).
    pub _pad_006c: [u8; 0x30],
    /// f32_9C (confidence: medium, kind: float, lanes: c-animation).
    pub f32_9c: f32,
    /// Unknown bytes (0xa0..0x2f4).
    pub _pad_00a0: [u8; 0x254],
    /// field_2f4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2f4: u32,
    /// Unknown bytes (0x2f8..0x9cc).
    pub _pad_02f8: [u8; 0x6d4],
    /// field_9cc (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9cc: u32,
    /// Unknown trailing bytes (0x9d0..0xc00).
    pub _pad_end: [u8; 0x230],
}
assert_size!(ARTRockstarNmRsCBUGrab, 0xc00); // merged size 0xbfd rounded to 4
assert_offset!(ARTRockstarNmRsCBUGrab, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUGrab, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUGrab, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUGrab, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUGrab, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUGrab, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUGrab, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUGrab, field_68, 0x68);
assert_offset!(ARTRockstarNmRsCBUGrab, f32_9c, 0x9c);
assert_offset!(ARTRockstarNmRsCBUGrab, field_2f4, 0x2f4);
assert_offset!(ARTRockstarNmRsCBUGrab, field_9cc, 0x9cc);

/// Merged layout for `ART::Rockstar::NmRsCBUHeadLook`.
///
/// Size: 0x9c (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUHeadLook {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// f32_34 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_34: f32,
    /// f32_38 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_38: f32,
    /// field_3c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// Unknown bytes (0x44..0x4c).
    pub _pad_0044: [u8; 0x8],
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown trailing bytes (0x64..0x9c).
    pub _pad_end: [u8; 0x38],
}
assert_size!(ARTRockstarNmRsCBUHeadLook, 0x9c); // merged size 0x9c rounded to 4
assert_offset!(ARTRockstarNmRsCBUHeadLook, f32_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUHeadLook, f32_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUHeadLook, field_60, 0x60);

/// Merged layout for `ART::Rockstar::NmRsCBUPedal`.
///
/// Size: 0x340 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUPedal {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
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
    /// Unknown bytes (0x48..0x50).
    pub _pad_0048: [u8; 0x8],
    /// field_50 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// ptr_60 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_60: Ptr32<u8>,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// field_68 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x9c).
    pub _pad_006c: [u8; 0x30],
    /// f32_9C (confidence: medium, kind: float, lanes: c-animation).
    pub f32_9c: f32,
    /// Unknown trailing bytes (0xa0..0x340).
    pub _pad_end: [u8; 0x2a0],
}
assert_size!(ARTRockstarNmRsCBUPedal, 0x340); // merged size 0x340 rounded to 4
assert_offset!(ARTRockstarNmRsCBUPedal, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUPedal, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUPedal, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUPedal, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUPedal, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUPedal, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUPedal, ptr_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUPedal, field_68, 0x68);
assert_offset!(ARTRockstarNmRsCBUPedal, f32_9c, 0x9c);

/// Merged layout for `ART::Rockstar::NmRsCBUPointArm`.
///
/// Size: 0x2d8 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUPointArm {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// f32_3C (confidence: medium, kind: float, lanes: c-animation).
    pub f32_3c: f32,
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// f32_44 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_44: f32,
    /// Unknown bytes (0x48..0x50).
    pub _pad_0048: [u8; 0x8],
    /// field_50 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// ptr_58 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_58: Ptr32<u8>,
    /// Unknown bytes (0x5c..0x68).
    pub _pad_005c: [u8; 0xc],
    /// bool_68 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_68: u8,
    /// bool_69 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_69: u8,
    /// Unknown trailing bytes (0x6a..0x2d8).
    pub _pad_end: [u8; 0x26e],
}
assert_size!(ARTRockstarNmRsCBUPointArm, 0x2d8); // merged size 0x2d8 rounded to 4
assert_offset!(ARTRockstarNmRsCBUPointArm, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUPointArm, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUPointArm, f32_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUPointArm, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUPointArm, f32_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUPointArm, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUPointArm, ptr_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUPointArm, bool_68, 0x68);
assert_offset!(ARTRockstarNmRsCBUPointArm, bool_69, 0x69);

/// Merged layout for `ART::Rockstar::NmRsCBURollDownStairs`.
///
/// Size: 0x2b8 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBURollDownStairs {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// f32_34 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_34: f32,
    /// bool_38 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_38: u8,
    /// Unknown bytes (0x39..0x3c).
    pub _pad_0039: [u8; 0x3],
    /// field_3c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// f32_40 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_40: f32,
    /// field_44 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x4a).
    pub _pad_0048: [u8; 0x2],
    /// bool_4A (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_4a: u8,
    /// Unknown bytes (0x4b..0x4c).
    pub _pad_004b: [u8; 0x1],
    /// field_4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// ptr_68 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_68: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x9c).
    pub _pad_006c: [u8; 0x30],
    /// field_9c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_9c: u32,
    /// Unknown bytes (0xa0..0x2b4).
    pub _pad_00a0: [u8; 0x214],
    /// field_2b4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2b4: u32,
}
assert_size!(ARTRockstarNmRsCBURollDownStairs, 0x2b8); // merged size 0x2b8 rounded to 4
assert_offset!(ARTRockstarNmRsCBURollDownStairs, f32_34, 0x34);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, bool_38, 0x38);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, f32_40, 0x40);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, bool_4a, 0x4a);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, ptr_68, 0x68);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_9c, 0x9c);
assert_offset!(ARTRockstarNmRsCBURollDownStairs, field_2b4, 0x2b4);

/// Merged layout for `ART::Rockstar::NmRsCBURollUp`.
///
/// Size: 0x340 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBURollUp {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// ptr_34 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_34: Ptr32<u8>,
    /// ptr_38 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_38: Ptr32<u8>,
    /// ptr_3C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_3c: Ptr32<u8>,
    /// ptr_40 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_40: Ptr32<u8>,
    /// ptr_44 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_44: Ptr32<u8>,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// u16_68 (confidence: medium, kind: u16_or_i16, lanes: c-animation).
    pub u16_68: u16,
    /// bool_6A (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_6a: u8,
    /// Unknown bytes (0x6b..0x6d).
    pub _pad_006b: [u8; 0x2],
    /// bool_6D (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_6d: u8,
    /// Unknown bytes (0x6e..0x334).
    pub _pad_006e: [u8; 0x2c6],
    /// field_334 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_334: u32,
    /// Unknown trailing bytes (0x338..0x340).
    pub _pad_end: [u8; 0x8],
}
assert_size!(ARTRockstarNmRsCBURollUp, 0x340); // merged size 0x340 rounded to 4
assert_offset!(ARTRockstarNmRsCBURollUp, ptr_34, 0x34);
assert_offset!(ARTRockstarNmRsCBURollUp, ptr_38, 0x38);
assert_offset!(ARTRockstarNmRsCBURollUp, ptr_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBURollUp, ptr_40, 0x40);
assert_offset!(ARTRockstarNmRsCBURollUp, ptr_44, 0x44);
assert_offset!(ARTRockstarNmRsCBURollUp, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBURollUp, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBURollUp, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBURollUp, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBURollUp, u16_68, 0x68);
assert_offset!(ARTRockstarNmRsCBURollUp, bool_6a, 0x6a);
assert_offset!(ARTRockstarNmRsCBURollUp, bool_6d, 0x6d);
assert_offset!(ARTRockstarNmRsCBURollUp, field_334, 0x334);

/// Merged layout for `ART::Rockstar::NmRsCBUShot`.
///
/// Size: 0xbfd (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUShot {
    /// Unknown bytes (0x0..0x44).
    pub _pad_0000: [u8; 0x44],
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0xb5).
    pub _pad_0064: [u8; 0x51],
    /// bool_B5 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_b5: u8,
    /// Unknown bytes (0xb6..0x148).
    pub _pad_00b6: [u8; 0x92],
    /// field_148 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_148: u32,
    /// field_14c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_14c: u32,
    /// Unknown bytes (0x150..0x170).
    pub _pad_0150: [u8; 0x20],
    /// field_170 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_170: u32,
    /// Unknown bytes (0x174..0x17c).
    pub _pad_0174: [u8; 0x8],
    /// field_17c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_17c: u32,
    /// field_180 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_180: u32,
    /// field_184 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_184: u32,
    /// f32_188 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_188: f32,
    /// Unknown bytes (0x18c..0x190).
    pub _pad_018c: [u8; 0x4],
    /// field_190 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_190: u32,
    /// field_194 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_194: u32,
    /// field_198 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_198: u32,
    /// field_19c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_19c: u32,
    /// field_1a0 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1a0: u32,
    /// Unknown bytes (0x1a4..0x1ac).
    pub _pad_01a4: [u8; 0x8],
    /// field_1ac (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1ac: u32,
    /// Unknown bytes (0x1b0..0x27c).
    pub _pad_01b0: [u8; 0xcc],
    /// bool_27C (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_27c: u8,
    /// bool_27D (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_27d: u8,
    /// Unknown bytes (0x27e..0x285).
    pub _pad_027e: [u8; 0x7],
    /// bool_285 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_285: u8,
    /// bool_286 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_286: u8,
    /// bool_287 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_287: u8,
    /// bool_288 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_288: u8,
    /// bool_289 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_289: u8,
    /// bool_28A (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_28a: u8,
    /// Unknown bytes (0x28b..0x28c).
    pub _pad_028b: [u8; 0x1],
    /// bool_28C (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_28c: u8,
    /// bool_28D (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_28d: u8,
    /// Unknown bytes (0x28e..0x2cc).
    pub _pad_028e: [u8; 0x3e],
    /// field_2cc (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2cc: u32,
    /// Unknown trailing bytes (0x2d0..0xc00).
    pub _pad_end: [u8; 0x930],
}
assert_size!(ARTRockstarNmRsCBUShot, 0xc00); // merged size 0xbfd rounded to 4
assert_offset!(ARTRockstarNmRsCBUShot, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUShot, field_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUShot, field_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUShot, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUShot, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUShot, bool_b5, 0xb5);
assert_offset!(ARTRockstarNmRsCBUShot, field_148, 0x148);
assert_offset!(ARTRockstarNmRsCBUShot, field_14c, 0x14c);
assert_offset!(ARTRockstarNmRsCBUShot, field_170, 0x170);
assert_offset!(ARTRockstarNmRsCBUShot, field_17c, 0x17c);
assert_offset!(ARTRockstarNmRsCBUShot, field_180, 0x180);
assert_offset!(ARTRockstarNmRsCBUShot, field_184, 0x184);
assert_offset!(ARTRockstarNmRsCBUShot, f32_188, 0x188);
assert_offset!(ARTRockstarNmRsCBUShot, field_190, 0x190);
assert_offset!(ARTRockstarNmRsCBUShot, field_194, 0x194);
assert_offset!(ARTRockstarNmRsCBUShot, field_198, 0x198);
assert_offset!(ARTRockstarNmRsCBUShot, field_19c, 0x19c);
assert_offset!(ARTRockstarNmRsCBUShot, field_1a0, 0x1a0);
assert_offset!(ARTRockstarNmRsCBUShot, field_1ac, 0x1ac);
assert_offset!(ARTRockstarNmRsCBUShot, bool_27c, 0x27c);
assert_offset!(ARTRockstarNmRsCBUShot, bool_27d, 0x27d);
assert_offset!(ARTRockstarNmRsCBUShot, bool_285, 0x285);
assert_offset!(ARTRockstarNmRsCBUShot, bool_286, 0x286);
assert_offset!(ARTRockstarNmRsCBUShot, bool_287, 0x287);
assert_offset!(ARTRockstarNmRsCBUShot, bool_288, 0x288);
assert_offset!(ARTRockstarNmRsCBUShot, bool_289, 0x289);
assert_offset!(ARTRockstarNmRsCBUShot, bool_28a, 0x28a);
assert_offset!(ARTRockstarNmRsCBUShot, bool_28c, 0x28c);
assert_offset!(ARTRockstarNmRsCBUShot, bool_28d, 0x28d);
assert_offset!(ARTRockstarNmRsCBUShot, field_2cc, 0x2cc);

/// Merged layout for `ART::Rockstar::NmRsCBUSpineTwist`.
///
/// Size: 0x2d8 (medium). Bases: ART::Rockstar::CBUTaskBase@0x0.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsCBUSpineTwist {
    /// Unknown bytes (0x0..0x34).
    pub _pad_0000: [u8; 0x34],
    /// field_34 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// field_3c (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_3c: u32,
    /// field_40 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// ptr_4C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_4c: Ptr32<u8>,
    /// ptr_50 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_50: Ptr32<u8>,
    /// Unknown bytes (0x54..0x58).
    pub _pad_0054: [u8; 0x4],
    /// field_58 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x68).
    pub _pad_0064: [u8; 0x4],
    /// field_68 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x9c).
    pub _pad_006c: [u8; 0x30],
    /// field_9c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_9c: u32,
    /// Unknown trailing bytes (0xa0..0x2d8).
    pub _pad_end: [u8; 0x238],
}
assert_size!(ARTRockstarNmRsCBUSpineTwist, 0x2d8); // merged size 0x2d8 rounded to 4
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_34, 0x34);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_38, 0x38);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_3c, 0x3c);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_40, 0x40);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_44, 0x44);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, ptr_4c, 0x4c);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, ptr_50, 0x50);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_58, 0x58);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_60, 0x60);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_68, 0x68);
assert_offset!(ARTRockstarNmRsCBUSpineTwist, field_9c, 0x9c);

/// Merged layout for `ART::Rockstar::NmRsEffectorBase`.
///
/// Size: 0x14c (low). Bases: none.
/// Lanes: c-animation, via:ART::Rockstar::NmRs1DofEffector, via:ART::Rockstar::NmRs3DofEffector.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsEffectorBase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x8).
    pub _pad_0004: [u8; 0x4],
    /// field_8 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_8: [u8; 8],
    /// field_10 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
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
    /// Unknown bytes (0x4c..0xd0).
    pub _pad_004c: [u8; 0x84],
    /// field_d0 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_d0: u32,
    /// field_d4 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_d4: u32,
    /// field_d8 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_d8: u32,
    /// field_dc (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_dc: u32,
    /// field_e0 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_e4: u32,
    /// bool_E8 (confidence: medium, kind: bool_or_u8, lanes: c-animation).
    pub bool_e8: u8,
    /// Unknown bytes (0xe9..0x108).
    pub _pad_00e9: [u8; 0x1f],
    /// field_108 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_108: u32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// ptr_110 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub ptr_110: u32,
    /// field_114 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_114: u32,
    /// field_118 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_118: u32,
    /// field_11c (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_11c: u32,
    /// field_120 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_120: u32,
    /// field_124 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_124: u32,
    /// Unknown bytes (0x128..0x134).
    pub _pad_0128: [u8; 0xc],
    /// field_134 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_134: u32,
    /// field_138 (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_138: u32,
    /// field_13c (confidence: medium, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_13c: u32,
    /// field_140 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_140: u32,
    /// field_144 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_144: u32,
    /// field_148 (confidence: high, kind: u32_or_ptr, lanes: c-animation,via:ART::Rockstar::NmRs1DofEffector,via:ART::Rockstar::NmRs3DofEffector moved from siblings:ART::Rockstar::NmRs1DofEffector,ART::Rockstar::NmRs3DofEffector).
    pub field_148: u32,
}
assert_size!(ARTRockstarNmRsEffectorBase, 0x14c); // merged size 0x14c rounded to 4
assert_offset!(ARTRockstarNmRsEffectorBase, vfptr, 0x0);
assert_offset!(ARTRockstarNmRsEffectorBase, field_8, 0x8);
assert_offset!(ARTRockstarNmRsEffectorBase, field_10, 0x10);
assert_offset!(ARTRockstarNmRsEffectorBase, field_14, 0x14);
assert_offset!(ARTRockstarNmRsEffectorBase, field_18, 0x18);
assert_offset!(ARTRockstarNmRsEffectorBase, field_20, 0x20);
assert_offset!(ARTRockstarNmRsEffectorBase, field_24, 0x24);
assert_offset!(ARTRockstarNmRsEffectorBase, field_28, 0x28);
assert_offset!(ARTRockstarNmRsEffectorBase, field_30, 0x30);
assert_offset!(ARTRockstarNmRsEffectorBase, field_34, 0x34);
assert_offset!(ARTRockstarNmRsEffectorBase, field_38, 0x38);
assert_offset!(ARTRockstarNmRsEffectorBase, field_40, 0x40);
assert_offset!(ARTRockstarNmRsEffectorBase, field_44, 0x44);
assert_offset!(ARTRockstarNmRsEffectorBase, field_48, 0x48);
assert_offset!(ARTRockstarNmRsEffectorBase, field_d0, 0xd0);
assert_offset!(ARTRockstarNmRsEffectorBase, field_d4, 0xd4);
assert_offset!(ARTRockstarNmRsEffectorBase, field_d8, 0xd8);
assert_offset!(ARTRockstarNmRsEffectorBase, field_dc, 0xdc);
assert_offset!(ARTRockstarNmRsEffectorBase, field_e0, 0xe0);
assert_offset!(ARTRockstarNmRsEffectorBase, field_e4, 0xe4);
assert_offset!(ARTRockstarNmRsEffectorBase, bool_e8, 0xe8);
assert_offset!(ARTRockstarNmRsEffectorBase, field_108, 0x108);
assert_offset!(ARTRockstarNmRsEffectorBase, ptr_110, 0x110);
assert_offset!(ARTRockstarNmRsEffectorBase, field_114, 0x114);
assert_offset!(ARTRockstarNmRsEffectorBase, field_118, 0x118);
assert_offset!(ARTRockstarNmRsEffectorBase, field_11c, 0x11c);
assert_offset!(ARTRockstarNmRsEffectorBase, field_120, 0x120);
assert_offset!(ARTRockstarNmRsEffectorBase, field_124, 0x124);
assert_offset!(ARTRockstarNmRsEffectorBase, field_134, 0x134);
assert_offset!(ARTRockstarNmRsEffectorBase, field_138, 0x138);
assert_offset!(ARTRockstarNmRsEffectorBase, field_13c, 0x13c);
assert_offset!(ARTRockstarNmRsEffectorBase, field_140, 0x140);
assert_offset!(ARTRockstarNmRsEffectorBase, field_144, 0x144);
assert_offset!(ARTRockstarNmRsEffectorBase, field_148, 0x148);

/// Merged layout for `ART::Rockstar::NmRsEngine`.
///
/// Size: 0x338 (medium). Bases: ART::SubAgentBuilder@0x0, ART::SegmentLoader@0x8, ART::Engine@0x10, ART::TransformSource@0x18, ART::IncomingTransformProcessor@0x20.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTRockstarNmRsEngine {
    /// Unknown bytes (0x0..0x28).
    pub _pad_0000: [u8; 0x28],
    /// field_28 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_28: u32,
    /// field_2c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_2c: u32,
    /// ptr_30 (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_30: Ptr32<u8>,
    /// field_34 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_38: u32,
    /// ptr_3C (confidence: high, kind: pointer, lanes: c-animation).
    pub ptr_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x2f8).
    pub _pad_0040: [u8; 0x2b8],
    /// field_2f8 (confidence: high, kind: u32_or_ptr, lanes: c-animation).
    pub field_2f8: u32,
    /// Unknown bytes (0x2fc..0x334).
    pub _pad_02fc: [u8; 0x38],
    /// field_334 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_334: u32,
}
assert_size!(ARTRockstarNmRsEngine, 0x338); // merged size 0x338 rounded to 4
assert_offset!(ARTRockstarNmRsEngine, field_28, 0x28);
assert_offset!(ARTRockstarNmRsEngine, field_2c, 0x2c);
assert_offset!(ARTRockstarNmRsEngine, ptr_30, 0x30);
assert_offset!(ARTRockstarNmRsEngine, field_34, 0x34);
assert_offset!(ARTRockstarNmRsEngine, field_38, 0x38);
assert_offset!(ARTRockstarNmRsEngine, ptr_3c, 0x3c);
assert_offset!(ARTRockstarNmRsEngine, field_2f8, 0x2f8);
assert_offset!(ARTRockstarNmRsEngine, field_334, 0x334);

/// Merged layout for `ART::SegmentLoader`.
///
/// Size: 0x99 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTSegmentLoader {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine: u32,
    /// ptr_C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_2: u32,
    /// field_14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_3: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_4: u32,
    /// field_24 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x40).
    pub _pad_0028: [u8; 0x18],
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// field_5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x70).
    pub _pad_0064: [u8; 0xc],
    /// field_70 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_88: u32,
    /// field_8c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// ptr_94 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_94: Ptr32<u8>,
    /// bool_98 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_98: u8,
    /// Unknown trailing bytes (0x99..0x9c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(ARTSegmentLoader, 0x9c); // merged size 0x99 rounded to 4
assert_offset!(ARTSegmentLoader, vfptr, 0x0);
assert_offset!(ARTSegmentLoader, ptr_4, 0x4);
assert_offset!(ARTSegmentLoader, embedded_nmrsengine, 0x8);
assert_offset!(ARTSegmentLoader, ptr_c, 0xc);
assert_offset!(ARTSegmentLoader, embedded_nmrsengine_2, 0x10);
assert_offset!(ARTSegmentLoader, field_14, 0x14);
assert_offset!(ARTSegmentLoader, embedded_nmrsengine_3, 0x18);
assert_offset!(ARTSegmentLoader, field_1c, 0x1c);
assert_offset!(ARTSegmentLoader, embedded_nmrsengine_4, 0x20);
assert_offset!(ARTSegmentLoader, field_24, 0x24);
assert_offset!(ARTSegmentLoader, field_40, 0x40);
assert_offset!(ARTSegmentLoader, field_44, 0x44);
assert_offset!(ARTSegmentLoader, field_48, 0x48);
assert_offset!(ARTSegmentLoader, field_4c, 0x4c);
assert_offset!(ARTSegmentLoader, field_50, 0x50);
assert_offset!(ARTSegmentLoader, field_54, 0x54);
assert_offset!(ARTSegmentLoader, f32_58, 0x58);
assert_offset!(ARTSegmentLoader, field_5c, 0x5c);
assert_offset!(ARTSegmentLoader, field_60, 0x60);
assert_offset!(ARTSegmentLoader, field_70, 0x70);
assert_offset!(ARTSegmentLoader, field_74, 0x74);
assert_offset!(ARTSegmentLoader, field_78, 0x78);
assert_offset!(ARTSegmentLoader, field_80, 0x80);
assert_offset!(ARTSegmentLoader, field_84, 0x84);
assert_offset!(ARTSegmentLoader, field_88, 0x88);
assert_offset!(ARTSegmentLoader, field_8c, 0x8c);
assert_offset!(ARTSegmentLoader, field_90, 0x90);
assert_offset!(ARTSegmentLoader, ptr_94, 0x94);
assert_offset!(ARTSegmentLoader, bool_98, 0x98);

/// Merged layout for `ART::SubAgentBuilder`.
///
/// Size: 0x98 (low). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTSubAgentBuilder {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x8).
    pub _pad_0004: [u8; 0x4],
    /// embedded_NmRsEngine (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine: u32,
    /// ptr_C (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_2: u32,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// embedded_NmRsEngine (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_3: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// embedded_NmRsEngine (confidence: medium, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_4: u32,
    /// Unknown bytes (0x24..0x40).
    pub _pad_0024: [u8; 0x1c],
    /// field_40 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// Unknown bytes (0x44..0x4c).
    pub _pad_0044: [u8; 0x8],
    /// field_4c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// Unknown bytes (0x50..0x8c).
    pub _pad_0050: [u8; 0x3c],
    /// field_8c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// Unknown bytes (0x90..0x94).
    pub _pad_0090: [u8; 0x4],
    /// ptr_94 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_94: Ptr32<u8>,
}
assert_size!(ARTSubAgentBuilder, 0x98); // merged size 0x98 rounded to 4
assert_offset!(ARTSubAgentBuilder, vfptr, 0x0);
assert_offset!(ARTSubAgentBuilder, embedded_nmrsengine, 0x8);
assert_offset!(ARTSubAgentBuilder, ptr_c, 0xc);
assert_offset!(ARTSubAgentBuilder, embedded_nmrsengine_2, 0x10);
assert_offset!(ARTSubAgentBuilder, embedded_nmrsengine_3, 0x18);
assert_offset!(ARTSubAgentBuilder, embedded_nmrsengine_4, 0x20);
assert_offset!(ARTSubAgentBuilder, field_40, 0x40);
assert_offset!(ARTSubAgentBuilder, field_4c, 0x4c);
assert_offset!(ARTSubAgentBuilder, field_8c, 0x8c);
assert_offset!(ARTSubAgentBuilder, ptr_94, 0x94);

/// Merged layout for `ART::TransformSource`.
///
/// Size: 0x99 (medium). Bases: none.
/// Lanes: c-animation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ARTTransformSource {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation).
    pub vfptr: Ptr32<()>,
    /// ptr_4 (confidence: low, kind: pointer, lanes: c-animation).
    pub ptr_4: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine: u32,
    /// ptr_C (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_c: Ptr32<u8>,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_2: u32,
    /// field_14 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_14: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_3: u32,
    /// field_1c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_1c: u32,
    /// embedded_NmRsEngine (confidence: high, kind: embedded_object, lanes: c-animation).
    pub embedded_nmrsengine_4: u32,
    /// field_24 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x40).
    pub _pad_0028: [u8; 0x18],
    /// field_40 (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_48: u32,
    /// field_4c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_4c: u32,
    /// field_50 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_54: u32,
    /// f32_58 (confidence: medium, kind: float, lanes: c-animation).
    pub f32_58: f32,
    /// field_5c (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_5c: u32,
    /// field_60 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x70).
    pub _pad_0064: [u8; 0xc],
    /// field_70 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_88: u32,
    /// field_8c (confidence: medium, kind: u32_or_ptr, lanes: c-animation).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-animation).
    pub field_90: u32,
    /// ptr_94 (confidence: medium, kind: pointer, lanes: c-animation).
    pub ptr_94: Ptr32<u8>,
    /// bool_98 (confidence: low, kind: bool_or_u8, lanes: c-animation).
    pub bool_98: u8,
    /// Unknown trailing bytes (0x99..0x9c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(ARTTransformSource, 0x9c); // merged size 0x99 rounded to 4
assert_offset!(ARTTransformSource, vfptr, 0x0);
assert_offset!(ARTTransformSource, ptr_4, 0x4);
assert_offset!(ARTTransformSource, embedded_nmrsengine, 0x8);
assert_offset!(ARTTransformSource, ptr_c, 0xc);
assert_offset!(ARTTransformSource, embedded_nmrsengine_2, 0x10);
assert_offset!(ARTTransformSource, field_14, 0x14);
assert_offset!(ARTTransformSource, embedded_nmrsengine_3, 0x18);
assert_offset!(ARTTransformSource, field_1c, 0x1c);
assert_offset!(ARTTransformSource, embedded_nmrsengine_4, 0x20);
assert_offset!(ARTTransformSource, field_24, 0x24);
assert_offset!(ARTTransformSource, field_40, 0x40);
assert_offset!(ARTTransformSource, field_44, 0x44);
assert_offset!(ARTTransformSource, field_48, 0x48);
assert_offset!(ARTTransformSource, field_4c, 0x4c);
assert_offset!(ARTTransformSource, field_50, 0x50);
assert_offset!(ARTTransformSource, field_54, 0x54);
assert_offset!(ARTTransformSource, f32_58, 0x58);
assert_offset!(ARTTransformSource, field_5c, 0x5c);
assert_offset!(ARTTransformSource, field_60, 0x60);
assert_offset!(ARTTransformSource, field_70, 0x70);
assert_offset!(ARTTransformSource, field_74, 0x74);
assert_offset!(ARTTransformSource, field_78, 0x78);
assert_offset!(ARTTransformSource, field_80, 0x80);
assert_offset!(ARTTransformSource, field_84, 0x84);
assert_offset!(ARTTransformSource, field_88, 0x88);
assert_offset!(ARTTransformSource, field_8c, 0x8c);
assert_offset!(ARTTransformSource, field_90, 0x90);
assert_offset!(ARTTransformSource, ptr_94, 0x94);
assert_offset!(ARTTransformSource, bool_98, 0x98);

/// Merged layout for `NMutils::MemoryStream`.
///
/// Size: 0x39 (low). Bases: none.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NMutilsMemoryStream {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// field_4 (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_4: Ptr32<u8>,
    /// field_8 (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_8: Ptr32<u8>,
    /// field_c (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_18: Ptr32<u8>,
    /// field_1c (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: pointer, lanes: c-misc-b).
    pub field_24: Ptr32<u8>,
    /// field_28 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x2d).
    pub _pad_002c: [u8; 0x1],
    /// field_2d (confidence: high, kind: bool/byte?, lanes: c-misc-b).
    pub field_2d: u8,
    /// Unknown bytes (0x2e..0x30).
    pub _pad_002e: [u8; 0x2],
    /// field_30 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: high, kind: bool/byte?, lanes: c-misc-b).
    pub field_38: u8,
    /// Unknown trailing bytes (0x39..0x3c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(NMutilsMemoryStream, 0x3c); // merged size 0x39 rounded to 4
assert_offset!(NMutilsMemoryStream, vftable, 0x0);
assert_offset!(NMutilsMemoryStream, field_4, 0x4);
assert_offset!(NMutilsMemoryStream, field_8, 0x8);
assert_offset!(NMutilsMemoryStream, field_c, 0xc);
assert_offset!(NMutilsMemoryStream, field_10, 0x10);
assert_offset!(NMutilsMemoryStream, field_14, 0x14);
assert_offset!(NMutilsMemoryStream, field_18, 0x18);
assert_offset!(NMutilsMemoryStream, field_1c, 0x1c);
assert_offset!(NMutilsMemoryStream, field_20, 0x20);
assert_offset!(NMutilsMemoryStream, field_24, 0x24);
assert_offset!(NMutilsMemoryStream, field_28, 0x28);
assert_offset!(NMutilsMemoryStream, field_2d, 0x2d);
assert_offset!(NMutilsMemoryStream, field_30, 0x30);
assert_offset!(NMutilsMemoryStream, field_34, 0x34);
assert_offset!(NMutilsMemoryStream, field_38, 0x38);

/// Merged layout for `NMutils::MemoryStreamReader`.
///
/// Size: 0x38 (low). Bases: none.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NMutilsMemoryStreamReader {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// field_c (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x24).
    pub _pad_001c: [u8; 0x8],
    /// field_24 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// Unknown bytes (0x28..0x34).
    pub _pad_0028: [u8; 0xc],
    /// field_34 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
}
assert_size!(NMutilsMemoryStreamReader, 0x38); // merged size 0x38 rounded to 4
assert_offset!(NMutilsMemoryStreamReader, vftable, 0x0);
assert_offset!(NMutilsMemoryStreamReader, field_4, 0x4);
assert_offset!(NMutilsMemoryStreamReader, field_8, 0x8);
assert_offset!(NMutilsMemoryStreamReader, field_c, 0xc);
assert_offset!(NMutilsMemoryStreamReader, field_10, 0x10);
assert_offset!(NMutilsMemoryStreamReader, field_14, 0x14);
assert_offset!(NMutilsMemoryStreamReader, field_18, 0x18);
assert_offset!(NMutilsMemoryStreamReader, field_24, 0x24);
assert_offset!(NMutilsMemoryStreamReader, field_34, 0x34);
