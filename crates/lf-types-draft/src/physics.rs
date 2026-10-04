//! RAGE physics, fragments and collision detection.
//!
//! Holds 40 draft layouts: `rage::ph*` (bounds, archetypes, colliders, joints, broad phase,
//! simulator), `rage::frag*` and the game's `fragInstNMGta` and `phInstBehaviorExplosionGta`, the
//! collision-detection classes (`rage::btAxisSweep3`, `btConvexTriangleCallback`,
//! `rage::GjkPairDetector`, `CollisionShape` and relatives), and the curves
//! `rage::cvCurve<rage::Vector3>` and `rage::cvCurveNurbs<rage::Vector3>` that the physics lane
//! recovered. Every layout is Inferred; size confidence (the analysis lanes' own rating) is high
//! for 5, medium for 1 and low for 34. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CollisionShape`.
///
/// Size: 0xd4 (low). Bases: none.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CollisionShape {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// Unknown bytes (0x4..0x7c).
    pub _pad_0004: [u8; 0x78],
    /// field_7c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_7c: u32,
    /// field_80 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_80: u32,
    /// field_84 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_84: u32,
    /// field_88 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_88: u32,
    /// field_8c (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_8c: u32,
    /// field_90 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// field_94 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_94: u32,
    /// field_98 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_98: u32,
    /// field_9c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_9c: u32,
    /// field_a0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_a0: u32,
    /// field_a4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a4: u32,
    /// field_a8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a8: u32,
    /// field_ac (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_ac: u32,
    /// field_b0 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_b0: u32,
    /// field_b4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b4: u32,
    /// field_b8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b8: u32,
    /// Unknown bytes (0xbc..0xd0).
    pub _pad_00bc: [u8; 0x14],
    /// field_d0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d0: u32,
}
assert_size!(CollisionShape, 0xd4); // merged size 0xd4 rounded to 4
assert_offset!(CollisionShape, vftable, 0x0);
assert_offset!(CollisionShape, field_7c, 0x7c);
assert_offset!(CollisionShape, field_80, 0x80);
assert_offset!(CollisionShape, field_84, 0x84);
assert_offset!(CollisionShape, field_88, 0x88);
assert_offset!(CollisionShape, field_8c, 0x8c);
assert_offset!(CollisionShape, field_90, 0x90);
assert_offset!(CollisionShape, field_94, 0x94);
assert_offset!(CollisionShape, field_98, 0x98);
assert_offset!(CollisionShape, field_9c, 0x9c);
assert_offset!(CollisionShape, field_a0, 0xa0);
assert_offset!(CollisionShape, field_a4, 0xa4);
assert_offset!(CollisionShape, field_a8, 0xa8);
assert_offset!(CollisionShape, field_ac, 0xac);
assert_offset!(CollisionShape, field_b0, 0xb0);
assert_offset!(CollisionShape, field_b4, 0xb4);
assert_offset!(CollisionShape, field_b8, 0xb8);
assert_offset!(CollisionShape, field_d0, 0xd0);

/// Merged layout for `ContinuousConvexCollision`.
///
/// Size: 0x310 (low). Bases: ConvexCast@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContinuousConvexCollision {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// field_10 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: flags, lanes: c-misc-b).
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
    /// field_40 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xb0).
    pub _pad_0094: [u8; 0x1c],
    /// field_b0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b0: u32,
    /// field_b4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b4: u32,
    /// field_b8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b8: u32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_c0: u32,
    /// field_c4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c4: u32,
    /// field_c8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c8: u32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d0: u32,
    /// field_d4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d4: u32,
    /// field_d8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d8: u32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f0: u32,
    /// field_f4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f4: u32,
    /// field_f8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_100: u32,
    /// field_104 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_104: u32,
    /// field_108 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_108: u32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// field_110 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_110: u32,
    /// Unknown bytes (0x114..0x118).
    pub _pad_0114: [u8; 0x4],
    /// field_118 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_118: u8,
    /// field_119 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_119: u8,
    /// field_11a (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_11a: u8,
    /// Unknown bytes (0x11b..0x2e0).
    pub _pad_011b: [u8; 0x1c5],
    /// field_2e0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2e0: u32,
    /// Unknown bytes (0x2e4..0x2f0).
    pub _pad_02e4: [u8; 0xc],
    /// field_2f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f4: u32,
    /// field_2f8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f8: u32,
    /// field_2fc (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2fc: u32,
    /// field_300 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_300: Ptr32<u8>,
    /// Unknown bytes (0x304..0x308).
    pub _pad_0304: [u8; 0x4],
    /// field_308 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_308: u32,
    /// field_30c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30c: u32,
}
assert_size!(ContinuousConvexCollision, 0x310); // merged size 0x310 rounded to 4
assert_offset!(ContinuousConvexCollision, field_4, 0x4);
assert_offset!(ContinuousConvexCollision, field_8, 0x8);
assert_offset!(ContinuousConvexCollision, field_10, 0x10);
assert_offset!(ContinuousConvexCollision, field_14, 0x14);
assert_offset!(ContinuousConvexCollision, field_18, 0x18);
assert_offset!(ContinuousConvexCollision, field_20, 0x20);
assert_offset!(ContinuousConvexCollision, field_24, 0x24);
assert_offset!(ContinuousConvexCollision, field_28, 0x28);
assert_offset!(ContinuousConvexCollision, field_30, 0x30);
assert_offset!(ContinuousConvexCollision, field_34, 0x34);
assert_offset!(ContinuousConvexCollision, field_38, 0x38);
assert_offset!(ContinuousConvexCollision, field_40, 0x40);
assert_offset!(ContinuousConvexCollision, field_44, 0x44);
assert_offset!(ContinuousConvexCollision, field_48, 0x48);
assert_offset!(ContinuousConvexCollision, field_50, 0x50);
assert_offset!(ContinuousConvexCollision, field_54, 0x54);
assert_offset!(ContinuousConvexCollision, field_58, 0x58);
assert_offset!(ContinuousConvexCollision, field_60, 0x60);
assert_offset!(ContinuousConvexCollision, field_64, 0x64);
assert_offset!(ContinuousConvexCollision, field_68, 0x68);
assert_offset!(ContinuousConvexCollision, field_70, 0x70);
assert_offset!(ContinuousConvexCollision, field_74, 0x74);
assert_offset!(ContinuousConvexCollision, field_78, 0x78);
assert_offset!(ContinuousConvexCollision, field_80, 0x80);
assert_offset!(ContinuousConvexCollision, field_84, 0x84);
assert_offset!(ContinuousConvexCollision, field_88, 0x88);
assert_offset!(ContinuousConvexCollision, field_90, 0x90);
assert_offset!(ContinuousConvexCollision, field_b0, 0xb0);
assert_offset!(ContinuousConvexCollision, field_b4, 0xb4);
assert_offset!(ContinuousConvexCollision, field_b8, 0xb8);
assert_offset!(ContinuousConvexCollision, field_c0, 0xc0);
assert_offset!(ContinuousConvexCollision, field_c4, 0xc4);
assert_offset!(ContinuousConvexCollision, field_c8, 0xc8);
assert_offset!(ContinuousConvexCollision, field_d0, 0xd0);
assert_offset!(ContinuousConvexCollision, field_d4, 0xd4);
assert_offset!(ContinuousConvexCollision, field_d8, 0xd8);
assert_offset!(ContinuousConvexCollision, field_e0, 0xe0);
assert_offset!(ContinuousConvexCollision, field_e4, 0xe4);
assert_offset!(ContinuousConvexCollision, field_e8, 0xe8);
assert_offset!(ContinuousConvexCollision, field_f0, 0xf0);
assert_offset!(ContinuousConvexCollision, field_f4, 0xf4);
assert_offset!(ContinuousConvexCollision, field_f8, 0xf8);
assert_offset!(ContinuousConvexCollision, field_100, 0x100);
assert_offset!(ContinuousConvexCollision, field_104, 0x104);
assert_offset!(ContinuousConvexCollision, field_108, 0x108);
assert_offset!(ContinuousConvexCollision, field_110, 0x110);
assert_offset!(ContinuousConvexCollision, field_118, 0x118);
assert_offset!(ContinuousConvexCollision, field_119, 0x119);
assert_offset!(ContinuousConvexCollision, field_11a, 0x11a);
assert_offset!(ContinuousConvexCollision, field_2e0, 0x2e0);
assert_offset!(ContinuousConvexCollision, field_2f0, 0x2f0);
assert_offset!(ContinuousConvexCollision, field_2f4, 0x2f4);
assert_offset!(ContinuousConvexCollision, field_2f8, 0x2f8);
assert_offset!(ContinuousConvexCollision, field_2fc, 0x2fc);
assert_offset!(ContinuousConvexCollision, field_300, 0x300);
assert_offset!(ContinuousConvexCollision, field_308, 0x308);
assert_offset!(ContinuousConvexCollision, field_30c, 0x30c);

/// Merged layout for `ConvexPenetrationDepthSolver`.
///
/// Size: 0xc (low). Bases: none.
/// Lanes: c-misc-b, c-physics, via:MinkowskiPenetrationDepthSolver, via:TrianglePenetrationDepthSolver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConvexPenetrationDepthSolver {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-misc-b,c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: flags, lanes: c-misc-b,via:MinkowskiPenetrationDepthSolver,via:TrianglePenetrationDepthSolver moved from siblings:MinkowskiPenetrationDepthSolver,TrianglePenetrationDepthSolver).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-b,via:MinkowskiPenetrationDepthSolver,via:TrianglePenetrationDepthSolver moved from siblings:MinkowskiPenetrationDepthSolver,TrianglePenetrationDepthSolver).
    pub field_8: u32,
}
assert_size!(ConvexPenetrationDepthSolver, 0xc); // merged size 0xc rounded to 4
assert_offset!(ConvexPenetrationDepthSolver, vfptr, 0x0);
assert_offset!(ConvexPenetrationDepthSolver, field_4, 0x4);
assert_offset!(ConvexPenetrationDepthSolver, field_8, 0x8);

/// Merged layout for `DiscreteCollisionDetectorInterface::Result`.
///
/// Size: 0x10 (low). Bases: none.
/// Lanes: c-misc-b, c-physics, via:PointCollector, via:rage::phManifoldResult.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiscreteCollisionDetectorInterfaceResult {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b,c-physics,via:PointCollector,via:rage::phManifoldResult moved from siblings:PointCollector,rage::phManifoldResult).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: flags, lanes: c-misc-b,c-physics,via:PointCollector,via:rage::phManifoldResult moved from siblings:PointCollector,rage::phManifoldResult).
    pub field_8: u32,
    /// field_c (confidence: medium, kind: flags, lanes: c-misc-b,c-physics,via:PointCollector,via:rage::phManifoldResult moved from siblings:PointCollector,rage::phManifoldResult).
    pub field_c: u32,
}
assert_size!(DiscreteCollisionDetectorInterfaceResult, 0x10); // merged size 0x10 rounded to 4
assert_offset!(DiscreteCollisionDetectorInterfaceResult, vftable, 0x0);
assert_offset!(DiscreteCollisionDetectorInterfaceResult, field_4, 0x4);
assert_offset!(DiscreteCollisionDetectorInterfaceResult, field_8, 0x8);
assert_offset!(DiscreteCollisionDetectorInterfaceResult, field_c, 0xc);

/// Merged layout for `PointCollector`.
///
/// Size: 0x310 (low). Bases: DiscreteCollisionDetectorInterface::Result@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PointCollector {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// field_2c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4c: u32,
    /// field_50 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_58: u8,
    /// Unknown bytes (0x59..0x80).
    pub _pad_0059: [u8; 0x27],
    /// field_80 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xc0).
    pub _pad_0094: [u8; 0x2c],
    /// field_c0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c0: u32,
    /// field_c4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c4: u32,
    /// field_c8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c8: u32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d0: u32,
    /// field_d4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d4: u32,
    /// field_d8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_d8: u32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_f0: u32,
    /// field_f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_f4: u32,
    /// field_f8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_100: u32,
    /// field_104 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_104: u32,
    /// field_108 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_108: u32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// field_110 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_110: u32,
    /// Unknown bytes (0x114..0x119).
    pub _pad_0114: [u8; 0x5],
    /// field_119 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_119: u8,
    /// field_11a (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_11a: u8,
    /// Unknown bytes (0x11b..0x2e0).
    pub _pad_011b: [u8; 0x1c5],
    /// field_2e0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2e0: u32,
    /// Unknown bytes (0x2e4..0x2f0).
    pub _pad_02e4: [u8; 0xc],
    /// field_2f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f4: u32,
    /// field_2f8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f8: u32,
    /// field_2fc (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2fc: u32,
    /// field_300 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_300: Ptr32<u8>,
    /// Unknown bytes (0x304..0x308).
    pub _pad_0304: [u8; 0x4],
    /// field_308 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_308: u32,
    /// field_30c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30c: u32,
}
assert_size!(PointCollector, 0x310); // merged size 0x310 rounded to 4
assert_offset!(PointCollector, field_10, 0x10);
assert_offset!(PointCollector, field_14, 0x14);
assert_offset!(PointCollector, field_18, 0x18);
assert_offset!(PointCollector, field_1c, 0x1c);
assert_offset!(PointCollector, field_20, 0x20);
assert_offset!(PointCollector, field_24, 0x24);
assert_offset!(PointCollector, field_28, 0x28);
assert_offset!(PointCollector, field_2c, 0x2c);
assert_offset!(PointCollector, field_30, 0x30);
assert_offset!(PointCollector, field_34, 0x34);
assert_offset!(PointCollector, field_38, 0x38);
assert_offset!(PointCollector, field_3c, 0x3c);
assert_offset!(PointCollector, field_40, 0x40);
assert_offset!(PointCollector, field_44, 0x44);
assert_offset!(PointCollector, field_48, 0x48);
assert_offset!(PointCollector, field_4c, 0x4c);
assert_offset!(PointCollector, field_50, 0x50);
assert_offset!(PointCollector, field_54, 0x54);
assert_offset!(PointCollector, field_58, 0x58);
assert_offset!(PointCollector, field_80, 0x80);
assert_offset!(PointCollector, field_84, 0x84);
assert_offset!(PointCollector, field_88, 0x88);
assert_offset!(PointCollector, field_90, 0x90);
assert_offset!(PointCollector, field_c0, 0xc0);
assert_offset!(PointCollector, field_c4, 0xc4);
assert_offset!(PointCollector, field_c8, 0xc8);
assert_offset!(PointCollector, field_d0, 0xd0);
assert_offset!(PointCollector, field_d4, 0xd4);
assert_offset!(PointCollector, field_d8, 0xd8);
assert_offset!(PointCollector, field_e0, 0xe0);
assert_offset!(PointCollector, field_e4, 0xe4);
assert_offset!(PointCollector, field_e8, 0xe8);
assert_offset!(PointCollector, field_f0, 0xf0);
assert_offset!(PointCollector, field_f4, 0xf4);
assert_offset!(PointCollector, field_f8, 0xf8);
assert_offset!(PointCollector, field_100, 0x100);
assert_offset!(PointCollector, field_104, 0x104);
assert_offset!(PointCollector, field_108, 0x108);
assert_offset!(PointCollector, field_110, 0x110);
assert_offset!(PointCollector, field_119, 0x119);
assert_offset!(PointCollector, field_11a, 0x11a);
assert_offset!(PointCollector, field_2e0, 0x2e0);
assert_offset!(PointCollector, field_2f0, 0x2f0);
assert_offset!(PointCollector, field_2f4, 0x2f4);
assert_offset!(PointCollector, field_2f8, 0x2f8);
assert_offset!(PointCollector, field_2fc, 0x2fc);
assert_offset!(PointCollector, field_300, 0x300);
assert_offset!(PointCollector, field_308, 0x308);
assert_offset!(PointCollector, field_30c, 0x30c);

/// Merged layout for `btConvexTriangleCallback`.
///
/// Size: 0x310 (low). Bases: TriangleCallback@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BtConvexTriangleCallback {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// float_10 (confidence: low, kind: float, lanes: c-physics).
    pub float_10: f32,
    /// float_14 (confidence: low, kind: float, lanes: c-physics).
    pub float_14: f32,
    /// float_18 (confidence: low, kind: float, lanes: c-physics).
    pub float_18: f32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// float_20 (confidence: low, kind: float, lanes: c-physics).
    pub float_20: f32,
    /// float_24 (confidence: low, kind: float, lanes: c-physics).
    pub float_24: f32,
    /// float_28 (confidence: low, kind: float, lanes: c-physics).
    pub float_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// float_30 (confidence: low, kind: float, lanes: c-physics).
    pub float_30: f32,
    /// float_34 (confidence: low, kind: float, lanes: c-physics).
    pub float_34: f32,
    /// float_38 (confidence: low, kind: float, lanes: c-physics).
    pub float_38: f32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// float_40 (confidence: low, kind: float, lanes: c-physics).
    pub float_40: f32,
    /// float_44 (confidence: low, kind: float, lanes: c-physics).
    pub float_44: f32,
    /// float_48 (confidence: low, kind: float, lanes: c-physics).
    pub float_48: f32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: low, kind: int?, lanes: c-physics).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x80).
    pub _pad_0054: [u8; 0x2c],
    /// float_80 (confidence: low, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: low, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// float_88 (confidence: low, kind: float, lanes: c-physics).
    pub float_88: f32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// float_90 (confidence: low, kind: float, lanes: c-physics).
    pub float_90: f32,
    /// Unknown bytes (0x94..0xc0).
    pub _pad_0094: [u8; 0x2c],
    /// float_c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// float_c4 (confidence: low, kind: float, lanes: c-physics).
    pub float_c4: f32,
    /// float_c8 (confidence: low, kind: float, lanes: c-physics).
    pub float_c8: f32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// float_d0 (confidence: low, kind: float, lanes: c-physics).
    pub float_d0: f32,
    /// float_d4 (confidence: low, kind: float, lanes: c-physics).
    pub float_d4: f32,
    /// float_d8 (confidence: low, kind: float, lanes: c-physics).
    pub float_d8: f32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// float_e0 (confidence: low, kind: float, lanes: c-physics).
    pub float_e0: f32,
    /// float_e4 (confidence: low, kind: float, lanes: c-physics).
    pub float_e4: f32,
    /// float_e8 (confidence: low, kind: float, lanes: c-physics).
    pub float_e8: f32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// float_f0 (confidence: low, kind: float, lanes: c-physics).
    pub float_f0: f32,
    /// float_f4 (confidence: low, kind: float, lanes: c-physics).
    pub float_f4: f32,
    /// float_f8 (confidence: low, kind: float, lanes: c-physics).
    pub float_f8: f32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// float_100 (confidence: low, kind: float, lanes: c-physics).
    pub float_100: f32,
    /// float_104 (confidence: low, kind: float, lanes: c-physics).
    pub float_104: f32,
    /// float_108 (confidence: low, kind: float, lanes: c-physics).
    pub float_108: f32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// float_110 (confidence: low, kind: float, lanes: c-physics).
    pub float_110: f32,
    /// field_114 (confidence: low, kind: int?, lanes: c-physics).
    pub field_114: u32,
    /// field_118 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_118: u16,
    /// bool_11a (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_11a: u8,
    /// Unknown bytes (0x11b..0x120).
    pub _pad_011b: [u8; 0x5],
    /// field_120 (confidence: low, kind: int?, lanes: c-physics).
    pub field_120: u32,
    /// Unknown bytes (0x124..0x2f0).
    pub _pad_0124: [u8; 0x1cc],
    /// float_2f0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2f0: f32,
    /// float_2f4 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2f4: f32,
    /// float_2f8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2f8: f32,
    /// float_2fc (confidence: low, kind: float, lanes: c-physics).
    pub float_2fc: f32,
    /// ptr_300 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_300: Ptr32<u8>,
    /// field_304 (confidence: low, kind: int?, lanes: c-physics).
    pub field_304: u32,
    /// field_308 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_308: u32,
    /// field_30c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_30c: u32,
}
assert_size!(BtConvexTriangleCallback, 0x310); // merged size 0x310 rounded to 4
assert_offset!(BtConvexTriangleCallback, float_10, 0x10);
assert_offset!(BtConvexTriangleCallback, float_14, 0x14);
assert_offset!(BtConvexTriangleCallback, float_18, 0x18);
assert_offset!(BtConvexTriangleCallback, float_20, 0x20);
assert_offset!(BtConvexTriangleCallback, float_24, 0x24);
assert_offset!(BtConvexTriangleCallback, float_28, 0x28);
assert_offset!(BtConvexTriangleCallback, float_30, 0x30);
assert_offset!(BtConvexTriangleCallback, float_34, 0x34);
assert_offset!(BtConvexTriangleCallback, float_38, 0x38);
assert_offset!(BtConvexTriangleCallback, float_40, 0x40);
assert_offset!(BtConvexTriangleCallback, float_44, 0x44);
assert_offset!(BtConvexTriangleCallback, float_48, 0x48);
assert_offset!(BtConvexTriangleCallback, field_50, 0x50);
assert_offset!(BtConvexTriangleCallback, float_80, 0x80);
assert_offset!(BtConvexTriangleCallback, float_84, 0x84);
assert_offset!(BtConvexTriangleCallback, float_88, 0x88);
assert_offset!(BtConvexTriangleCallback, float_90, 0x90);
assert_offset!(BtConvexTriangleCallback, float_c0, 0xc0);
assert_offset!(BtConvexTriangleCallback, float_c4, 0xc4);
assert_offset!(BtConvexTriangleCallback, float_c8, 0xc8);
assert_offset!(BtConvexTriangleCallback, float_d0, 0xd0);
assert_offset!(BtConvexTriangleCallback, float_d4, 0xd4);
assert_offset!(BtConvexTriangleCallback, float_d8, 0xd8);
assert_offset!(BtConvexTriangleCallback, float_e0, 0xe0);
assert_offset!(BtConvexTriangleCallback, float_e4, 0xe4);
assert_offset!(BtConvexTriangleCallback, float_e8, 0xe8);
assert_offset!(BtConvexTriangleCallback, float_f0, 0xf0);
assert_offset!(BtConvexTriangleCallback, float_f4, 0xf4);
assert_offset!(BtConvexTriangleCallback, float_f8, 0xf8);
assert_offset!(BtConvexTriangleCallback, float_100, 0x100);
assert_offset!(BtConvexTriangleCallback, float_104, 0x104);
assert_offset!(BtConvexTriangleCallback, float_108, 0x108);
assert_offset!(BtConvexTriangleCallback, float_110, 0x110);
assert_offset!(BtConvexTriangleCallback, field_114, 0x114);
assert_offset!(BtConvexTriangleCallback, field_118, 0x118);
assert_offset!(BtConvexTriangleCallback, bool_11a, 0x11a);
assert_offset!(BtConvexTriangleCallback, field_120, 0x120);
assert_offset!(BtConvexTriangleCallback, float_2f0, 0x2f0);
assert_offset!(BtConvexTriangleCallback, float_2f4, 0x2f4);
assert_offset!(BtConvexTriangleCallback, float_2f8, 0x2f8);
assert_offset!(BtConvexTriangleCallback, float_2fc, 0x2fc);
assert_offset!(BtConvexTriangleCallback, ptr_300, 0x300);
assert_offset!(BtConvexTriangleCallback, field_304, 0x304);
assert_offset!(BtConvexTriangleCallback, field_308, 0x308);
assert_offset!(BtConvexTriangleCallback, field_30c, 0x30c);

/// Merged layout for `fragInstNMGta`.
///
/// Size: 0x2e2 (low). Bases: rage::fragInstNM@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FragInstNMGta {
    /// Unknown bytes (0x0..0xb0).
    pub _pad_0000: [u8; 0xb0],
    /// ptr_b0 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_b0: Ptr32<u8>,
    /// field_b4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_b4: u32,
    /// field_b8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_b8: u32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_c0: u32,
    /// field_c4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_c4: u32,
    /// field_c8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_c8: u32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_d0: u32,
    /// field_d4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_d4: u32,
    /// field_d8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_d8: u32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_f0: u32,
    /// field_f4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_f4: u32,
    /// field_f8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_100: u32,
    /// Unknown bytes (0x104..0x2a0).
    pub _pad_0104: [u8; 0x19c],
    /// field_2a0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2a0: u32,
    /// Unknown bytes (0x2a4..0x2e0).
    pub _pad_02a4: [u8; 0x3c],
    /// field_2e0 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2e0: u16,
    /// Unknown trailing bytes (0x2e2..0x2e4).
    pub _pad_end: [u8; 0x2],
}
assert_size!(FragInstNMGta, 0x2e4); // merged size 0x2e2 rounded to 4
assert_offset!(FragInstNMGta, ptr_b0, 0xb0);
assert_offset!(FragInstNMGta, field_b4, 0xb4);
assert_offset!(FragInstNMGta, field_b8, 0xb8);
assert_offset!(FragInstNMGta, field_c0, 0xc0);
assert_offset!(FragInstNMGta, field_c4, 0xc4);
assert_offset!(FragInstNMGta, field_c8, 0xc8);
assert_offset!(FragInstNMGta, field_d0, 0xd0);
assert_offset!(FragInstNMGta, field_d4, 0xd4);
assert_offset!(FragInstNMGta, field_d8, 0xd8);
assert_offset!(FragInstNMGta, field_e0, 0xe0);
assert_offset!(FragInstNMGta, field_e4, 0xe4);
assert_offset!(FragInstNMGta, field_e8, 0xe8);
assert_offset!(FragInstNMGta, field_f0, 0xf0);
assert_offset!(FragInstNMGta, field_f4, 0xf4);
assert_offset!(FragInstNMGta, field_f8, 0xf8);
assert_offset!(FragInstNMGta, field_100, 0x100);
assert_offset!(FragInstNMGta, field_2a0, 0x2a0);
assert_offset!(FragInstNMGta, field_2e0, 0x2e0);

/// Merged layout for `phInstBehaviorExplosionGta`.
///
/// Size: 0x1c (low). Bases: rage::phInstBehavior@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhInstBehaviorExplosionGta {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// float_08 (confidence: high, kind: float, lanes: c-physics).
    pub float_08: f32,
    /// float_0c (confidence: high, kind: float, lanes: c-physics).
    pub float_0c: f32,
    /// field_10 (confidence: low, kind: int?, lanes: c-physics).
    pub field_10: u32,
    /// float_14 (confidence: medium, kind: float, lanes: c-physics).
    pub float_14: f32,
    /// field_18 (confidence: high, kind: int?, lanes: c-physics).
    pub field_18: u32,
}
assert_size!(PhInstBehaviorExplosionGta, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(PhInstBehaviorExplosionGta, float_08, 0x8);
assert_offset!(PhInstBehaviorExplosionGta, float_0c, 0xc);
assert_offset!(PhInstBehaviorExplosionGta, field_10, 0x10);
assert_offset!(PhInstBehaviorExplosionGta, float_14, 0x14);
assert_offset!(PhInstBehaviorExplosionGta, field_18, 0x18);

/// Merged layout for `rage::GjkPairDetector`.
///
/// Size: 0x4a8 (low). Bases: DiscreteCollisionDetectorInterface@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageGjkPairDetector {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// Unknown bytes (0x8..0xc).
    pub _pad_0008: [u8; 0x4],
    /// field_c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// field_10 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_24: u32,
    /// field_28 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_28: u32,
    /// field_2c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_30: u32,
    /// field_34 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_44: u32,
    /// field_48 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4c: u32,
    /// field_50 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_50: u32,
    /// field_54 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_74: u32,
    /// field_78 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_78: u32,
    /// field_7c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_7c: u32,
    /// field_80 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_80: u32,
    /// Unknown bytes (0x84..0x90).
    pub _pad_0084: [u8; 0xc],
    /// field_90 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// Unknown bytes (0x94..0xb0).
    pub _pad_0094: [u8; 0x1c],
    /// field_b0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b0: u32,
    /// field_b4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b4: u32,
    /// field_b8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_b8: u32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// field_c0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c0: u32,
    /// Unknown bytes (0xc4..0xe0).
    pub _pad_00c4: [u8; 0x1c],
    /// field_e0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e0: u32,
    /// field_e4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f0: u32,
    /// field_f4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f4: u32,
    /// field_f8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_f8: u32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// field_100 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_100: u32,
    /// field_104 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_104: u32,
    /// field_108 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_108: u32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// field_110 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_110: u32,
    /// field_114 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_114: u32,
    /// field_118 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_118: u32,
    /// Unknown bytes (0x11c..0x120).
    pub _pad_011c: [u8; 0x4],
    /// field_120 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_120: u32,
    /// Unknown bytes (0x124..0x2f0).
    pub _pad_0124: [u8; 0x1cc],
    /// field_2f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f4: u32,
    /// field_2f8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f8: u32,
    /// Unknown bytes (0x2fc..0x300).
    pub _pad_02fc: [u8; 0x4],
    /// field_300 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_300: u32,
    /// field_304 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_304: u32,
    /// field_308 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_308: u32,
    /// field_30c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_30c: u32,
    /// Unknown bytes (0x310..0x4a0).
    pub _pad_0310: [u8; 0x190],
    /// field_4a0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4a0: u32,
    /// field_4a4 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_4a4: Ptr32<u8>,
}
assert_size!(RageGjkPairDetector, 0x4a8); // merged size 0x4a8 rounded to 4
assert_offset!(RageGjkPairDetector, field_4, 0x4);
assert_offset!(RageGjkPairDetector, field_c, 0xc);
assert_offset!(RageGjkPairDetector, field_10, 0x10);
assert_offset!(RageGjkPairDetector, field_14, 0x14);
assert_offset!(RageGjkPairDetector, field_18, 0x18);
assert_offset!(RageGjkPairDetector, field_20, 0x20);
assert_offset!(RageGjkPairDetector, field_24, 0x24);
assert_offset!(RageGjkPairDetector, field_28, 0x28);
assert_offset!(RageGjkPairDetector, field_2c, 0x2c);
assert_offset!(RageGjkPairDetector, field_30, 0x30);
assert_offset!(RageGjkPairDetector, field_34, 0x34);
assert_offset!(RageGjkPairDetector, field_38, 0x38);
assert_offset!(RageGjkPairDetector, field_3c, 0x3c);
assert_offset!(RageGjkPairDetector, field_40, 0x40);
assert_offset!(RageGjkPairDetector, field_44, 0x44);
assert_offset!(RageGjkPairDetector, field_48, 0x48);
assert_offset!(RageGjkPairDetector, field_4c, 0x4c);
assert_offset!(RageGjkPairDetector, field_50, 0x50);
assert_offset!(RageGjkPairDetector, field_54, 0x54);
assert_offset!(RageGjkPairDetector, field_58, 0x58);
assert_offset!(RageGjkPairDetector, field_60, 0x60);
assert_offset!(RageGjkPairDetector, field_64, 0x64);
assert_offset!(RageGjkPairDetector, field_68, 0x68);
assert_offset!(RageGjkPairDetector, field_70, 0x70);
assert_offset!(RageGjkPairDetector, field_74, 0x74);
assert_offset!(RageGjkPairDetector, field_78, 0x78);
assert_offset!(RageGjkPairDetector, field_7c, 0x7c);
assert_offset!(RageGjkPairDetector, field_80, 0x80);
assert_offset!(RageGjkPairDetector, field_90, 0x90);
assert_offset!(RageGjkPairDetector, field_b0, 0xb0);
assert_offset!(RageGjkPairDetector, field_b4, 0xb4);
assert_offset!(RageGjkPairDetector, field_b8, 0xb8);
assert_offset!(RageGjkPairDetector, field_c0, 0xc0);
assert_offset!(RageGjkPairDetector, field_e0, 0xe0);
assert_offset!(RageGjkPairDetector, field_e4, 0xe4);
assert_offset!(RageGjkPairDetector, field_e8, 0xe8);
assert_offset!(RageGjkPairDetector, field_f0, 0xf0);
assert_offset!(RageGjkPairDetector, field_f4, 0xf4);
assert_offset!(RageGjkPairDetector, field_f8, 0xf8);
assert_offset!(RageGjkPairDetector, field_100, 0x100);
assert_offset!(RageGjkPairDetector, field_104, 0x104);
assert_offset!(RageGjkPairDetector, field_108, 0x108);
assert_offset!(RageGjkPairDetector, field_110, 0x110);
assert_offset!(RageGjkPairDetector, field_114, 0x114);
assert_offset!(RageGjkPairDetector, field_118, 0x118);
assert_offset!(RageGjkPairDetector, field_120, 0x120);
assert_offset!(RageGjkPairDetector, field_2f0, 0x2f0);
assert_offset!(RageGjkPairDetector, field_2f4, 0x2f4);
assert_offset!(RageGjkPairDetector, field_2f8, 0x2f8);
assert_offset!(RageGjkPairDetector, field_300, 0x300);
assert_offset!(RageGjkPairDetector, field_304, 0x304);
assert_offset!(RageGjkPairDetector, field_308, 0x308);
assert_offset!(RageGjkPairDetector, field_30c, 0x30c);
assert_offset!(RageGjkPairDetector, field_4a0, 0x4a0);
assert_offset!(RageGjkPairDetector, field_4a4, 0x4a4);

/// Merged layout for `rage::TriangleShape`.
///
/// Size: 0xac (low). Bases: rage::phBound@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageTriangleShape {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_80: u32,
    /// field_84 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_84: u32,
    /// field_88 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_90: u32,
    /// field_94 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_94: u32,
    /// field_98 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// field_a0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a0: u32,
    /// field_a4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a4: u32,
    /// field_a8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a8: u32,
}
assert_size!(RageTriangleShape, 0xac); // merged size 0xac rounded to 4
assert_offset!(RageTriangleShape, field_80, 0x80);
assert_offset!(RageTriangleShape, field_84, 0x84);
assert_offset!(RageTriangleShape, field_88, 0x88);
assert_offset!(RageTriangleShape, field_90, 0x90);
assert_offset!(RageTriangleShape, field_94, 0x94);
assert_offset!(RageTriangleShape, field_98, 0x98);
assert_offset!(RageTriangleShape, field_a0, 0xa0);
assert_offset!(RageTriangleShape, field_a4, 0xa4);
assert_offset!(RageTriangleShape, field_a8, 0xa8);

/// Merged layout for `rage::btAxisSweep3`.
///
/// Size: 0x58 (low). Bases: rage::phBroadPhase@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageBtAxisSweep3 {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// float_10 (confidence: low, kind: float, lanes: c-physics).
    pub float_10: f32,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// float_18 (confidence: low, kind: float, lanes: c-physics).
    pub float_18: f32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// float_20 (confidence: low, kind: float, lanes: c-physics).
    pub float_20: f32,
    /// float_24 (confidence: low, kind: float, lanes: c-physics).
    pub float_24: f32,
    /// float_28 (confidence: low, kind: float, lanes: c-physics).
    pub float_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// float_30 (confidence: medium, kind: float, lanes: c-physics).
    pub float_30: f32,
    /// float_34 (confidence: medium, kind: float, lanes: c-physics).
    pub float_34: f32,
    /// float_38 (confidence: medium, kind: float, lanes: c-physics).
    pub float_38: f32,
    /// float_3c (confidence: low, kind: float, lanes: c-physics).
    pub float_3c: f32,
    /// field_40 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_40: u16,
    /// field_42 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_42: u16,
    /// field_44 (confidence: high, kind: int?, lanes: c-physics).
    pub field_44: u32,
    /// field_48 (confidence: high, kind: int?, lanes: c-physics).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: int?, lanes: c-physics).
    pub field_54: u32,
}
assert_size!(RageBtAxisSweep3, 0x58); // merged size 0x58 rounded to 4
assert_offset!(RageBtAxisSweep3, float_10, 0x10);
assert_offset!(RageBtAxisSweep3, float_18, 0x18);
assert_offset!(RageBtAxisSweep3, float_20, 0x20);
assert_offset!(RageBtAxisSweep3, float_24, 0x24);
assert_offset!(RageBtAxisSweep3, float_28, 0x28);
assert_offset!(RageBtAxisSweep3, float_30, 0x30);
assert_offset!(RageBtAxisSweep3, float_34, 0x34);
assert_offset!(RageBtAxisSweep3, float_38, 0x38);
assert_offset!(RageBtAxisSweep3, float_3c, 0x3c);
assert_offset!(RageBtAxisSweep3, field_40, 0x40);
assert_offset!(RageBtAxisSweep3, field_42, 0x42);
assert_offset!(RageBtAxisSweep3, field_44, 0x44);
assert_offset!(RageBtAxisSweep3, field_48, 0x48);
assert_offset!(RageBtAxisSweep3, field_50, 0x50);
assert_offset!(RageBtAxisSweep3, field_54, 0x54);

/// Merged layout for `rage::cvCurve<rage::Vector3>`.
///
/// Size: 0x4c (low). Bases: none.
/// Lanes: c-physics, via:rage::cvCurveCatRom, via:rage::cvCurveNurbs<rage::Vector3>.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCvCurveRageVector3 {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_4: u32,
    /// Unknown bytes (0x8..0xa).
    pub _pad_0008: [u8; 0x2],
    /// field_a (confidence: low, kind: int16?, lanes: c-physics).
    pub field_a: u16,
    /// field_c (confidence: low, kind: int?, lanes: c-physics).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x40).
    pub _pad_0010: [u8; 0x30],
    /// field_40 (confidence: high, kind: int?, lanes: c-physics,via:rage::cvCurveCatRom,via:rage::cvCurveNurbs<rage::Vector3> moved from siblings:rage::cvCurveCatRom,rage::cvCurveNurbs<rage::Vector3>).
    pub field_40: u32,
    /// field_44 (confidence: high, kind: int?, lanes: c-physics,via:rage::cvCurveCatRom,via:rage::cvCurveNurbs<rage::Vector3> moved from siblings:rage::cvCurveCatRom,rage::cvCurveNurbs<rage::Vector3>).
    pub field_44: u32,
    /// field_48 (confidence: high, kind: int?, lanes: c-physics,via:rage::cvCurveCatRom,via:rage::cvCurveNurbs<rage::Vector3> moved from siblings:rage::cvCurveCatRom,rage::cvCurveNurbs<rage::Vector3>).
    pub field_48: u32,
}
assert_size!(RageCvCurveRageVector3, 0x4c); // merged size 0x4c rounded to 4
assert_offset!(RageCvCurveRageVector3, vfptr, 0x0);
assert_offset!(RageCvCurveRageVector3, field_4, 0x4);
assert_offset!(RageCvCurveRageVector3, field_a, 0xa);
assert_offset!(RageCvCurveRageVector3, field_c, 0xc);
assert_offset!(RageCvCurveRageVector3, field_40, 0x40);
assert_offset!(RageCvCurveRageVector3, field_44, 0x44);
assert_offset!(RageCvCurveRageVector3, field_48, 0x48);

/// Merged layout for `rage::cvCurveNurbs<rage::Vector3>`.
///
/// Size: 0xc4 (low). Bases: rage::cvCurve<rage::Vector3>@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCvCurveNurbsRageVector3 {
    /// Unknown bytes (0x0..0x1c).
    pub _pad_0000: [u8; 0x1c],
    /// field_1c (confidence: high, kind: int?, lanes: c-physics).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_20: u16,
    /// Unknown bytes (0x22..0x24).
    pub _pad_0022: [u8; 0x2],
    /// field_24 (confidence: high, kind: int?, lanes: c-physics).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_28: u16,
    /// Unknown bytes (0x2a..0x2c).
    pub _pad_002a: [u8; 0x2],
    /// float_2c (confidence: high, kind: float, lanes: c-physics).
    pub float_2c: f32,
    /// float_30 (confidence: medium, kind: float, lanes: c-physics).
    pub float_30: f32,
    /// float_34 (confidence: medium, kind: float, lanes: c-physics).
    pub float_34: f32,
    /// float_38 (confidence: medium, kind: float, lanes: c-physics).
    pub float_38: f32,
    /// field_3c (confidence: high, kind: int?, lanes: c-physics).
    pub field_3c: u32,
    /// Unknown bytes (0x40..0x80).
    pub _pad_0040: [u8; 0x40],
    /// field_80 (confidence: low, kind: int?, lanes: c-physics).
    pub field_80: u32,
    /// Unknown bytes (0x84..0xb4).
    pub _pad_0084: [u8; 0x30],
    /// ptr_b4 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_b4: Ptr32<u8>,
    /// field_b8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_b8: u32,
    /// field_bc (confidence: low, kind: int?, lanes: c-physics).
    pub field_bc: u32,
    /// float_c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_c0: f32,
}
assert_size!(RageCvCurveNurbsRageVector3, 0xc4); // merged size 0xc4 rounded to 4
assert_offset!(RageCvCurveNurbsRageVector3, field_1c, 0x1c);
assert_offset!(RageCvCurveNurbsRageVector3, field_20, 0x20);
assert_offset!(RageCvCurveNurbsRageVector3, field_24, 0x24);
assert_offset!(RageCvCurveNurbsRageVector3, field_28, 0x28);
assert_offset!(RageCvCurveNurbsRageVector3, float_2c, 0x2c);
assert_offset!(RageCvCurveNurbsRageVector3, float_30, 0x30);
assert_offset!(RageCvCurveNurbsRageVector3, float_34, 0x34);
assert_offset!(RageCvCurveNurbsRageVector3, float_38, 0x38);
assert_offset!(RageCvCurveNurbsRageVector3, field_3c, 0x3c);
assert_offset!(RageCvCurveNurbsRageVector3, field_80, 0x80);
assert_offset!(RageCvCurveNurbsRageVector3, ptr_b4, 0xb4);
assert_offset!(RageCvCurveNurbsRageVector3, field_b8, 0xb8);
assert_offset!(RageCvCurveNurbsRageVector3, field_bc, 0xbc);
assert_offset!(RageCvCurveNurbsRageVector3, float_c0, 0xc0);

/// Merged layout for `rage::fragInst`.
///
/// Size: 0xa0 (low). Bases: rage::phInstBreakable@0x0, rage::pgStreamableRef<rage::fragType>@0x50.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageFragInst {
    /// Unknown bytes (0x0..0x5c).
    pub _pad_0000: [u8; 0x5c],
    /// field_5c (confidence: high, kind: int?, lanes: c-physics).
    pub field_5c: u32,
    /// Unknown bytes (0x60..0x64).
    pub _pad_0060: [u8; 0x4],
    /// field_64 (confidence: high, kind: int?, lanes: c-physics).
    pub field_64: u32,
    /// float_68 (confidence: high, kind: float, lanes: c-physics).
    pub float_68: f32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// float_70 (confidence: high, kind: float, lanes: c-physics).
    pub float_70: f32,
    /// float_74 (confidence: high, kind: float, lanes: c-physics).
    pub float_74: f32,
    /// float_78 (confidence: high, kind: float, lanes: c-physics).
    pub float_78: f32,
    /// float_7c (confidence: high, kind: float, lanes: c-physics).
    pub float_7c: f32,
    /// float_80 (confidence: high, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: high, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// float_88 (confidence: high, kind: float, lanes: c-physics).
    pub float_88: f32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: high, kind: int?, lanes: c-physics).
    pub field_90: u32,
    /// field_94 (confidence: high, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// field_98 (confidence: low, kind: int?, lanes: c-physics).
    pub field_98: u32,
    /// field_9c (confidence: high, kind: int?, lanes: c-physics).
    pub field_9c: u32,
}
assert_size!(RageFragInst, 0xa0); // merged size 0xa0 rounded to 4
assert_offset!(RageFragInst, field_5c, 0x5c);
assert_offset!(RageFragInst, field_64, 0x64);
assert_offset!(RageFragInst, float_68, 0x68);
assert_offset!(RageFragInst, float_70, 0x70);
assert_offset!(RageFragInst, float_74, 0x74);
assert_offset!(RageFragInst, float_78, 0x78);
assert_offset!(RageFragInst, float_7c, 0x7c);
assert_offset!(RageFragInst, float_80, 0x80);
assert_offset!(RageFragInst, float_84, 0x84);
assert_offset!(RageFragInst, float_88, 0x88);
assert_offset!(RageFragInst, field_90, 0x90);
assert_offset!(RageFragInst, field_94, 0x94);
assert_offset!(RageFragInst, field_98, 0x98);
assert_offset!(RageFragInst, field_9c, 0x9c);

/// Merged layout for `rage::fragManagerTemplate<rage::fragTypeStub>`.
///
/// Size: 0x401 (low). Bases: rage::fragManagerBase@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageFragManagerTemplateRageFragTypeStub {
    /// Unknown bytes (0x0..0x2f0).
    pub _pad_0000: [u8; 0x2f0],
    /// field_2f0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f4: u32,
    /// Unknown bytes (0x2f8..0x3f0).
    pub _pad_02f8: [u8; 0xf8],
    /// field_3f0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_3f0: u32,
    /// Unknown bytes (0x3f4..0x3f6).
    pub _pad_03f4: [u8; 0x2],
    /// field_3f6 (confidence: medium, kind: int16?, lanes: c-physics).
    pub field_3f6: u16,
    /// Unknown bytes (0x3f8..0x3fc).
    pub _pad_03f8: [u8; 0x4],
    /// ptr_3fc (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_3fc: Ptr32<u8>,
    /// bool_400 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_400: u8,
    /// Unknown trailing bytes (0x401..0x404).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageFragManagerTemplateRageFragTypeStub, 0x404); // merged size 0x401 rounded to 4
assert_offset!(RageFragManagerTemplateRageFragTypeStub, field_2f0, 0x2f0);
assert_offset!(RageFragManagerTemplateRageFragTypeStub, field_2f4, 0x2f4);
assert_offset!(RageFragManagerTemplateRageFragTypeStub, field_3f0, 0x3f0);
assert_offset!(RageFragManagerTemplateRageFragTypeStub, field_3f6, 0x3f6);
assert_offset!(RageFragManagerTemplateRageFragTypeStub, ptr_3fc, 0x3fc);
assert_offset!(RageFragManagerTemplateRageFragTypeStub, bool_400, 0x400);

/// Merged layout for `rage::fragType`.
///
/// Size: 0x204 (low). Bases: rage::pgBase@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageFragType {
    /// Unknown bytes (0x0..0x50).
    pub _pad_0000: [u8; 0x50],
    /// field_50 (confidence: medium, kind: int?, lanes: c-misc-b,c-physics).
    pub field_50: u32,
    /// Unknown bytes (0x54..0x70).
    pub _pad_0054: [u8; 0x1c],
    /// field_70 (confidence: medium, kind: int?, lanes: c-misc-b,c-physics).
    pub field_70: u32,
    /// Unknown bytes (0x74..0xc8).
    pub _pad_0074: [u8; 0x54],
    /// field_c8 (confidence: high, kind: int?, lanes: c-misc-b,c-physics).
    pub field_c8: u32,
    /// Unknown bytes (0xcc..0xfc).
    pub _pad_00cc: [u8; 0x30],
    /// field_fc (confidence: medium, kind: int?, lanes: c-misc-b,c-physics).
    pub field_fc: u32,
    /// field_100 (confidence: low, kind: int?, lanes: c-physics).
    pub field_100: u32,
    /// Unknown bytes (0x104..0x108).
    pub _pad_0104: [u8; 0x4],
    /// field_108 (confidence: high, kind: int?, lanes: c-misc-b,c-physics).
    pub field_108: u32,
    /// field_10c (confidence: low, kind: int?, lanes: c-physics).
    pub field_10c: u32,
    /// Unknown bytes (0x110..0x1d8).
    pub _pad_0110: [u8; 0xc8],
    /// field_1d8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1d8: u32,
    /// field_1dc (confidence: low, kind: int?, lanes: c-physics).
    pub field_1dc: u32,
    /// field_1e0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1e0: u32,
    /// field_1e4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: int?, lanes: c-misc-b,c-physics).
    pub field_1e8: u32,
    /// field_1ec (confidence: medium, kind: int?, lanes: c-misc-b,c-physics).
    pub field_1ec: u32,
    /// field_1f0 (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1f0: u8,
    /// Unknown bytes (0x1f1..0x1f3).
    pub _pad_01f1: [u8; 0x2],
    /// field_1f3 (confidence: low, kind: int8/bool?, lanes: c-physics).
    pub field_1f3: u8,
    /// field_1f4 (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1f4: u8,
    /// bool_1f5 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_1f5: u8,
    /// Unknown bytes (0x1f6..0x1f7).
    pub _pad_01f6: [u8; 0x1],
    /// field_1f7 (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1f7: u8,
    /// Unknown bytes (0x1f8..0x1f9).
    pub _pad_01f8: [u8; 0x1],
    /// field_1f9 (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1f9: u8,
    /// field_1fa (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1fa: u8,
    /// field_1fb (confidence: medium, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_1fb: u8,
    /// Unknown bytes (0x1fc..0x200).
    pub _pad_01fc: [u8; 0x4],
    /// float_200 (confidence: medium, kind: float, lanes: c-misc-b,c-physics).
    pub float_200: f32,
}
assert_size!(RageFragType, 0x204); // merged size 0x204 rounded to 4
assert_offset!(RageFragType, field_50, 0x50);
assert_offset!(RageFragType, field_70, 0x70);
assert_offset!(RageFragType, field_c8, 0xc8);
assert_offset!(RageFragType, field_fc, 0xfc);
assert_offset!(RageFragType, field_100, 0x100);
assert_offset!(RageFragType, field_108, 0x108);
assert_offset!(RageFragType, field_10c, 0x10c);
assert_offset!(RageFragType, field_1d8, 0x1d8);
assert_offset!(RageFragType, field_1dc, 0x1dc);
assert_offset!(RageFragType, field_1e0, 0x1e0);
assert_offset!(RageFragType, field_1e4, 0x1e4);
assert_offset!(RageFragType, field_1e8, 0x1e8);
assert_offset!(RageFragType, field_1ec, 0x1ec);
assert_offset!(RageFragType, field_1f0, 0x1f0);
assert_offset!(RageFragType, field_1f3, 0x1f3);
assert_offset!(RageFragType, field_1f4, 0x1f4);
assert_offset!(RageFragType, bool_1f5, 0x1f5);
assert_offset!(RageFragType, field_1f7, 0x1f7);
assert_offset!(RageFragType, field_1f9, 0x1f9);
assert_offset!(RageFragType, field_1fa, 0x1fa);
assert_offset!(RageFragType, field_1fb, 0x1fb);
assert_offset!(RageFragType, float_200, 0x200);

/// Merged layout for `rage::fragTypeChild`.
///
/// Size: 0x310 (low). Bases: rage::datBase@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageFragTypeChild {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// float_04 (confidence: low, kind: float, lanes: c-physics).
    pub float_04: f32,
    /// Unknown bytes (0x8..0x90).
    pub _pad_0008: [u8; 0x88],
    /// field_90 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_90: u32,
    /// field_94 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// field_98 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_98: u32,
    /// field_9c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_9c: u32,
    /// field_a0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_a0: u32,
    /// field_a4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_a4: u32,
    /// field_a8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_a8: u32,
    /// Unknown bytes (0xac..0xae).
    pub _pad_00ac: [u8; 0x2],
    /// field_ae (confidence: low, kind: int16?, lanes: c-physics).
    pub field_ae: u16,
    /// Unknown bytes (0xb0..0x160).
    pub _pad_00b0: [u8; 0xb0],
    /// float_160 (confidence: low, kind: float, lanes: c-physics).
    pub float_160: f32,
    /// Unknown bytes (0x164..0x174).
    pub _pad_0164: [u8; 0x10],
    /// field_174 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_174: u32,
    /// Unknown bytes (0x178..0x17a).
    pub _pad_0178: [u8; 0x2],
    /// field_17a (confidence: low, kind: int16?, lanes: c-physics).
    pub field_17a: u16,
    /// Unknown bytes (0x17c..0x22c).
    pub _pad_017c: [u8; 0xb0],
    /// float_22c (confidence: low, kind: float, lanes: c-physics).
    pub float_22c: f32,
    /// Unknown bytes (0x230..0x240).
    pub _pad_0230: [u8; 0x10],
    /// field_240 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_240: u32,
    /// Unknown bytes (0x244..0x246).
    pub _pad_0244: [u8; 0x2],
    /// field_246 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_246: u16,
    /// Unknown bytes (0x248..0x2f8).
    pub _pad_0248: [u8; 0xb0],
    /// float_2f8 (confidence: low, kind: float, lanes: c-physics).
    pub float_2f8: f32,
    /// Unknown bytes (0x2fc..0x30c).
    pub _pad_02fc: [u8; 0x10],
    /// bool_30c (confidence: low, kind: bool, lanes: c-physics).
    pub bool_30c: u8,
    /// field_30d (confidence: low, kind: int16?, lanes: c-physics).
    pub field_30d: [u8; 2],
    /// field_30f (confidence: low, kind: int8/bool?, lanes: c-physics).
    pub field_30f: u8,
}
assert_size!(RageFragTypeChild, 0x310); // merged size 0x310 rounded to 4
assert_offset!(RageFragTypeChild, float_04, 0x4);
assert_offset!(RageFragTypeChild, field_90, 0x90);
assert_offset!(RageFragTypeChild, field_94, 0x94);
assert_offset!(RageFragTypeChild, field_98, 0x98);
assert_offset!(RageFragTypeChild, field_9c, 0x9c);
assert_offset!(RageFragTypeChild, field_a0, 0xa0);
assert_offset!(RageFragTypeChild, field_a4, 0xa4);
assert_offset!(RageFragTypeChild, field_a8, 0xa8);
assert_offset!(RageFragTypeChild, field_ae, 0xae);
assert_offset!(RageFragTypeChild, float_160, 0x160);
assert_offset!(RageFragTypeChild, field_174, 0x174);
assert_offset!(RageFragTypeChild, field_17a, 0x17a);
assert_offset!(RageFragTypeChild, float_22c, 0x22c);
assert_offset!(RageFragTypeChild, field_240, 0x240);
assert_offset!(RageFragTypeChild, field_246, 0x246);
assert_offset!(RageFragTypeChild, float_2f8, 0x2f8);
assert_offset!(RageFragTypeChild, bool_30c, 0x30c);
assert_offset!(RageFragTypeChild, field_30d, 0x30d);
assert_offset!(RageFragTypeChild, field_30f, 0x30f);

/// Merged layout for `rage::phArchetype`.
///
/// Size: 0x1c (low). Bases: rage::phArchetypeBase@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhArchetype {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// ptr_10 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_10: Ptr32<u8>,
    /// ptr_14 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_14: Ptr32<u8>,
    /// field_18 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_18: u16,
    /// field_1a (confidence: low, kind: int16?, lanes: c-physics).
    pub field_1a: u16,
}
assert_size!(RagePhArchetype, 0x1c); // merged size 0x1c rounded to 4
assert_offset!(RagePhArchetype, field_8, 0x8);
assert_offset!(RagePhArchetype, ptr_10, 0x10);
assert_offset!(RagePhArchetype, ptr_14, 0x14);
assert_offset!(RagePhArchetype, field_18, 0x18);
assert_offset!(RagePhArchetype, field_1a, 0x1a);

/// Merged layout for `rage::phArchetypeDamp`.
///
/// Size: 0xb6 (low). Bases: rage::phArchetypePhys@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhArchetypeDamp {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_4: u32,
    /// Unknown bytes (0x8..0x50).
    pub _pad_0008: [u8; 0x48],
    /// float_50 (confidence: medium, kind: float, lanes: c-physics).
    pub float_50: f32,
    /// ptr_54 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_54: Ptr32<u8>,
    /// float_58 (confidence: medium, kind: float, lanes: c-physics).
    pub float_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// float_60 (confidence: medium, kind: float, lanes: c-physics).
    pub float_60: f32,
    /// float_64 (confidence: medium, kind: float, lanes: c-physics).
    pub float_64: f32,
    /// float_68 (confidence: medium, kind: float, lanes: c-physics).
    pub float_68: f32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// float_70 (confidence: medium, kind: float, lanes: c-physics).
    pub float_70: f32,
    /// float_74 (confidence: medium, kind: float, lanes: c-physics).
    pub float_74: f32,
    /// float_78 (confidence: medium, kind: float, lanes: c-physics).
    pub float_78: f32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// float_80 (confidence: medium, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: medium, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// float_88 (confidence: medium, kind: float, lanes: c-physics).
    pub float_88: f32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// float_90 (confidence: medium, kind: float, lanes: c-physics).
    pub float_90: f32,
    /// float_94 (confidence: medium, kind: float, lanes: c-physics).
    pub float_94: f32,
    /// float_98 (confidence: medium, kind: float, lanes: c-physics).
    pub float_98: f32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// float_a0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_a0: f32,
    /// float_a4 (confidence: medium, kind: float, lanes: c-physics).
    pub float_a4: f32,
    /// float_a8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_a8: f32,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// bool_b0 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b0: u8,
    /// bool_b1 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b1: u8,
    /// bool_b2 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b2: u8,
    /// bool_b3 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b3: u8,
    /// bool_b4 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b4: u8,
    /// bool_b5 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_b5: u8,
    /// Unknown trailing bytes (0xb6..0xb8).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RagePhArchetypeDamp, 0xb8); // merged size 0xb6 rounded to 4
assert_offset!(RagePhArchetypeDamp, field_4, 0x4);
assert_offset!(RagePhArchetypeDamp, float_50, 0x50);
assert_offset!(RagePhArchetypeDamp, ptr_54, 0x54);
assert_offset!(RagePhArchetypeDamp, float_58, 0x58);
assert_offset!(RagePhArchetypeDamp, float_60, 0x60);
assert_offset!(RagePhArchetypeDamp, float_64, 0x64);
assert_offset!(RagePhArchetypeDamp, float_68, 0x68);
assert_offset!(RagePhArchetypeDamp, float_70, 0x70);
assert_offset!(RagePhArchetypeDamp, float_74, 0x74);
assert_offset!(RagePhArchetypeDamp, float_78, 0x78);
assert_offset!(RagePhArchetypeDamp, float_80, 0x80);
assert_offset!(RagePhArchetypeDamp, float_84, 0x84);
assert_offset!(RagePhArchetypeDamp, float_88, 0x88);
assert_offset!(RagePhArchetypeDamp, float_90, 0x90);
assert_offset!(RagePhArchetypeDamp, float_94, 0x94);
assert_offset!(RagePhArchetypeDamp, float_98, 0x98);
assert_offset!(RagePhArchetypeDamp, float_a0, 0xa0);
assert_offset!(RagePhArchetypeDamp, float_a4, 0xa4);
assert_offset!(RagePhArchetypeDamp, float_a8, 0xa8);
assert_offset!(RagePhArchetypeDamp, bool_b0, 0xb0);
assert_offset!(RagePhArchetypeDamp, bool_b1, 0xb1);
assert_offset!(RagePhArchetypeDamp, bool_b2, 0xb2);
assert_offset!(RagePhArchetypeDamp, bool_b3, 0xb3);
assert_offset!(RagePhArchetypeDamp, bool_b4, 0xb4);
assert_offset!(RagePhArchetypeDamp, bool_b5, 0xb5);

/// Merged layout for `rage::phArchetypePhys`.
///
/// Size: 0x4c (low). Bases: rage::phArchetype@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhArchetypePhys {
    /// Unknown bytes (0x0..0x1c).
    pub _pad_0000: [u8; 0x1c],
    /// float_1c (confidence: high, kind: float, lanes: c-physics).
    pub float_1c: f32,
    /// float_20 (confidence: high, kind: float, lanes: c-physics).
    pub float_20: f32,
    /// float_24 (confidence: high, kind: float, lanes: c-physics).
    pub float_24: f32,
    /// float_28 (confidence: low, kind: float, lanes: c-physics).
    pub float_28: f32,
    /// float_2c (confidence: low, kind: float, lanes: c-physics).
    pub float_2c: f32,
    /// float_30 (confidence: high, kind: float, lanes: c-physics).
    pub float_30: f32,
    /// float_34 (confidence: high, kind: float, lanes: c-physics).
    pub float_34: f32,
    /// float_38 (confidence: high, kind: float, lanes: c-physics).
    pub float_38: f32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: high, kind: int?, lanes: c-physics).
    pub field_40: u32,
    /// field_44 (confidence: high, kind: int?, lanes: c-physics).
    pub field_44: u32,
    /// float_48 (confidence: high, kind: float, lanes: c-physics).
    pub float_48: f32,
}
assert_size!(RagePhArchetypePhys, 0x4c); // merged size 0x4c rounded to 4
assert_offset!(RagePhArchetypePhys, float_1c, 0x1c);
assert_offset!(RagePhArchetypePhys, float_20, 0x20);
assert_offset!(RagePhArchetypePhys, float_24, 0x24);
assert_offset!(RagePhArchetypePhys, float_28, 0x28);
assert_offset!(RagePhArchetypePhys, float_2c, 0x2c);
assert_offset!(RagePhArchetypePhys, float_30, 0x30);
assert_offset!(RagePhArchetypePhys, float_34, 0x34);
assert_offset!(RagePhArchetypePhys, float_38, 0x38);
assert_offset!(RagePhArchetypePhys, field_40, 0x40);
assert_offset!(RagePhArchetypePhys, field_44, 0x44);
assert_offset!(RagePhArchetypePhys, float_48, 0x48);

/// Merged layout for `rage::phArticulatedCollider`.
///
/// Size: 0x2f0 (low). Bases: rage::phCollider@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhArticulatedCollider {
    /// Unknown bytes (0x0..0xc8).
    pub _pad_0000: [u8; 0xc8],
    /// float_c8 (confidence: low, kind: float, lanes: c-physics).
    pub float_c8: f32,
    /// float_cc (confidence: low, kind: float, lanes: c-physics).
    pub float_cc: f32,
    /// Unknown bytes (0xd0..0x2a0).
    pub _pad_00d0: [u8; 0x1d0],
    /// field_2a0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_2a0: u32,
    /// Unknown bytes (0x2a4..0x2a8).
    pub _pad_02a4: [u8; 0x4],
    /// field_2a8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2a8: u32,
    /// Unknown bytes (0x2ac..0x2e4).
    pub _pad_02ac: [u8; 0x38],
    /// bool_2e4 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_2e4: u8,
    /// Unknown bytes (0x2e5..0x2ec).
    pub _pad_02e5: [u8; 0x7],
    /// field_2ec (confidence: medium, kind: int?, lanes: c-physics).
    pub field_2ec: u32,
}
assert_size!(RagePhArticulatedCollider, 0x2f0); // merged size 0x2f0 rounded to 4
assert_offset!(RagePhArticulatedCollider, float_c8, 0xc8);
assert_offset!(RagePhArticulatedCollider, float_cc, 0xcc);
assert_offset!(RagePhArticulatedCollider, field_2a0, 0x2a0);
assert_offset!(RagePhArticulatedCollider, field_2a8, 0x2a8);
assert_offset!(RagePhArticulatedCollider, bool_2e4, 0x2e4);
assert_offset!(RagePhArticulatedCollider, field_2ec, 0x2ec);

/// Merged layout for `rage::phBound`.
///
/// Size: 0xfc (low). Bases: rage::phBoundBase@0x0.
/// Lanes: c-misc-b, c-physics, via:rage::TriangleShape, via:rage::phBoundBox, via:rage::phBoundCapsule, via:rage::phBoundComposite, via:rage::phBoundGrid, via:rage::phBoundPolyhedron, via:rage::phBoundRibbon, via:rage::phBoundSphere, via:rage::phBoundTaperedCapsule.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBound {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-misc-b,c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: int8/bool?, lanes: c-misc-b,c-physics).
    pub field_4: u8,
    /// field_5 (confidence: low, kind: int8/bool?, lanes: c-physics).
    pub field_5: u8,
    /// bool_06 (confidence: medium, kind: bool, lanes: c-physics,via:rage::phBoundBox,via:rage::phBoundCapsule,via:rage::phBoundComposite,via:rage::phBoundSphere,via:rage::phBoundTaperedCapsule moved from siblings:rage::phBoundCapsule,rage::phBoundComposite,rage::phBoundPolyhedron,rage::phBoundSphere,rage::phBoundTaperedCapsule).
    pub bool_06: u8,
    /// field_7 (confidence: high, kind: int8/bool?, lanes: c-physics).
    pub field_7: u8,
    /// float_08 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_08: f32,
    /// float_0c (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_0c: f32,
    /// float_10 (confidence: medium, kind: float, lanes: c-misc-b,c-physics).
    pub float_10: f32,
    /// float_14 (confidence: medium, kind: float, lanes: c-misc-b,c-physics).
    pub float_14: f32,
    /// float_18 (confidence: medium, kind: float, lanes: c-misc-b,c-physics).
    pub float_18: f32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// float_20 (confidence: low, kind: float, lanes: c-physics).
    pub float_20: f32,
    /// float_24 (confidence: low, kind: float, lanes: c-physics).
    pub float_24: f32,
    /// float_28 (confidence: low, kind: float, lanes: c-physics).
    pub float_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// float_30 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_30: f32,
    /// float_34 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_34: f32,
    /// float_38 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_38: f32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// float_40 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_40: f32,
    /// float_44 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_44: f32,
    /// float_48 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_48: f32,
    /// float_4c (confidence: medium, kind: float, lanes: c-misc-b,c-physics).
    pub float_4c: f32,
    /// float_50 (confidence: low, kind: float, lanes: c-physics).
    pub float_50: f32,
    /// float_54 (confidence: low, kind: float, lanes: c-physics).
    pub float_54: f32,
    /// float_58 (confidence: low, kind: float, lanes: c-physics).
    pub float_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// float_60 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_60: f32,
    /// float_64 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_64: f32,
    /// float_68 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_68: f32,
    /// float_6c (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_6c: f32,
    /// float_70 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_70: f32,
    /// float_74 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_74: f32,
    /// float_78 (confidence: high, kind: float, lanes: c-misc-b,c-physics).
    pub float_78: f32,
    /// field_7c (confidence: high, kind: int?, lanes: c-misc-b,c-physics).
    pub field_7c: u32,
    /// Unknown bytes (0x80..0x8c).
    pub _pad_0080: [u8; 0xc],
    /// field_8c (confidence: high, kind: int?, lanes: c-misc-b,c-physics,via:rage::TriangleShape,via:rage::phBoundComposite,via:rage::phBoundGrid,via:rage::phBoundPolyhedron,via:rage::phBoundRibbon moved from siblings:rage::TriangleShape,rage::phBoundComposite,rage::phBoundGrid,rage::phBoundPolyhedron,rage::phBoundRibbon).
    pub field_8c: u32,
    /// Unknown bytes (0x90..0x9c).
    pub _pad_0090: [u8; 0xc],
    /// field_9c (confidence: high, kind: int?, lanes: c-physics,via:rage::phBoundGrid,via:rage::phBoundRibbon,via:rage::phBoundSphere moved from siblings:rage::phBoundGrid,rage::phBoundRibbon,rage::phBoundSphere).
    pub field_9c: u32,
    /// Unknown bytes (0xa0..0xcc).
    pub _pad_00a0: [u8; 0x2c],
    /// field_cc (confidence: high, kind: int?, lanes: c-physics,via:rage::phBoundPolyhedron,via:rage::phBoundRibbon moved from siblings:rage::phBoundPolyhedron,rage::phBoundRibbon).
    pub field_cc: u32,
    /// Unknown bytes (0xd0..0xd8).
    pub _pad_00d0: [u8; 0x8],
    /// float_d8 (confidence: high, kind: float, lanes: c-physics,via:rage::phBoundBox,via:rage::phBoundTaperedCapsule moved from siblings:rage::phBoundPolyhedron,rage::phBoundTaperedCapsule).
    pub float_d8: f32,
    /// Unknown bytes (0xdc..0xf0).
    pub _pad_00dc: [u8; 0x14],
    /// float_f0 (confidence: high, kind: float, lanes: c-physics,via:rage::phBoundBox,via:rage::phBoundTaperedCapsule moved from siblings:rage::phBoundPolyhedron,rage::phBoundTaperedCapsule).
    pub float_f0: f32,
    /// float_f4 (confidence: high, kind: float, lanes: c-physics,via:rage::phBoundBox,via:rage::phBoundTaperedCapsule moved from siblings:rage::phBoundPolyhedron,rage::phBoundTaperedCapsule).
    pub float_f4: f32,
    /// float_f8 (confidence: high, kind: float, lanes: c-physics,via:rage::phBoundBox,via:rage::phBoundTaperedCapsule moved from siblings:rage::phBoundPolyhedron,rage::phBoundTaperedCapsule).
    pub float_f8: f32,
}
assert_size!(RagePhBound, 0xfc); // merged size 0xfc rounded to 4
assert_offset!(RagePhBound, vfptr, 0x0);
assert_offset!(RagePhBound, field_4, 0x4);
assert_offset!(RagePhBound, field_5, 0x5);
assert_offset!(RagePhBound, bool_06, 0x6);
assert_offset!(RagePhBound, field_7, 0x7);
assert_offset!(RagePhBound, float_08, 0x8);
assert_offset!(RagePhBound, float_0c, 0xc);
assert_offset!(RagePhBound, float_10, 0x10);
assert_offset!(RagePhBound, float_14, 0x14);
assert_offset!(RagePhBound, float_18, 0x18);
assert_offset!(RagePhBound, float_20, 0x20);
assert_offset!(RagePhBound, float_24, 0x24);
assert_offset!(RagePhBound, float_28, 0x28);
assert_offset!(RagePhBound, float_30, 0x30);
assert_offset!(RagePhBound, float_34, 0x34);
assert_offset!(RagePhBound, float_38, 0x38);
assert_offset!(RagePhBound, float_40, 0x40);
assert_offset!(RagePhBound, float_44, 0x44);
assert_offset!(RagePhBound, float_48, 0x48);
assert_offset!(RagePhBound, float_4c, 0x4c);
assert_offset!(RagePhBound, float_50, 0x50);
assert_offset!(RagePhBound, float_54, 0x54);
assert_offset!(RagePhBound, float_58, 0x58);
assert_offset!(RagePhBound, float_60, 0x60);
assert_offset!(RagePhBound, float_64, 0x64);
assert_offset!(RagePhBound, float_68, 0x68);
assert_offset!(RagePhBound, float_6c, 0x6c);
assert_offset!(RagePhBound, float_70, 0x70);
assert_offset!(RagePhBound, float_74, 0x74);
assert_offset!(RagePhBound, float_78, 0x78);
assert_offset!(RagePhBound, field_7c, 0x7c);
assert_offset!(RagePhBound, field_8c, 0x8c);
assert_offset!(RagePhBound, field_9c, 0x9c);
assert_offset!(RagePhBound, field_cc, 0xcc);
assert_offset!(RagePhBound, float_d8, 0xd8);
assert_offset!(RagePhBound, float_f0, 0xf0);
assert_offset!(RagePhBound, float_f4, 0xf4);
assert_offset!(RagePhBound, float_f8, 0xf8);

/// Merged layout for `rage::phBoundBox`.
///
/// Size: 0x224 (low). Bases: rage::phBoundPolyhedron@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundBox {
    /// Unknown bytes (0x0..0xd0).
    pub _pad_0000: [u8; 0xd0],
    /// float_d0 (confidence: high, kind: float, lanes: c-physics).
    pub float_d0: f32,
    /// float_d4 (confidence: high, kind: float, lanes: c-physics).
    pub float_d4: f32,
    /// Unknown bytes (0xd8..0xe0).
    pub _pad_00d8: [u8; 0x8],
    /// float_e0 (confidence: high, kind: float, lanes: c-physics).
    pub float_e0: f32,
    /// float_e4 (confidence: high, kind: float, lanes: c-physics).
    pub float_e4: f32,
    /// float_e8 (confidence: high, kind: float, lanes: c-physics).
    pub float_e8: f32,
    /// Unknown bytes (0xec..0x100).
    pub _pad_00ec: [u8; 0x14],
    /// float_100 (confidence: high, kind: float, lanes: c-physics).
    pub float_100: f32,
    /// float_104 (confidence: high, kind: float, lanes: c-physics).
    pub float_104: f32,
    /// float_108 (confidence: high, kind: float, lanes: c-physics).
    pub float_108: f32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// float_110 (confidence: high, kind: float, lanes: c-physics).
    pub float_110: f32,
    /// float_114 (confidence: high, kind: float, lanes: c-physics).
    pub float_114: f32,
    /// float_118 (confidence: high, kind: float, lanes: c-physics).
    pub float_118: f32,
    /// Unknown bytes (0x11c..0x120).
    pub _pad_011c: [u8; 0x4],
    /// float_120 (confidence: high, kind: float, lanes: c-physics).
    pub float_120: f32,
    /// float_124 (confidence: high, kind: float, lanes: c-physics).
    pub float_124: f32,
    /// float_128 (confidence: high, kind: float, lanes: c-physics).
    pub float_128: f32,
    /// Unknown bytes (0x12c..0x130).
    pub _pad_012c: [u8; 0x4],
    /// float_130 (confidence: high, kind: float, lanes: c-physics).
    pub float_130: f32,
    /// float_134 (confidence: high, kind: float, lanes: c-physics).
    pub float_134: f32,
    /// float_138 (confidence: high, kind: float, lanes: c-physics).
    pub float_138: f32,
    /// Unknown bytes (0x13c..0x140).
    pub _pad_013c: [u8; 0x4],
    /// float_140 (confidence: high, kind: float, lanes: c-physics).
    pub float_140: f32,
    /// float_144 (confidence: high, kind: float, lanes: c-physics).
    pub float_144: f32,
    /// float_148 (confidence: high, kind: float, lanes: c-physics).
    pub float_148: f32,
    /// Unknown bytes (0x14c..0x150).
    pub _pad_014c: [u8; 0x4],
    /// float_150 (confidence: high, kind: float, lanes: c-physics).
    pub float_150: f32,
    /// float_154 (confidence: high, kind: float, lanes: c-physics).
    pub float_154: f32,
    /// float_158 (confidence: high, kind: float, lanes: c-physics).
    pub float_158: f32,
    /// Unknown bytes (0x15c..0x160).
    pub _pad_015c: [u8; 0x4],
    /// field_160 (confidence: low, kind: int?, lanes: c-physics).
    pub field_160: u32,
    /// field_164 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_164: u32,
    /// Unknown bytes (0x168..0x172).
    pub _pad_0168: [u8; 0xa],
    /// field_172 (confidence: low, kind: int?, lanes: c-physics).
    pub field_172: [u8; 4],
    /// Unknown bytes (0x176..0x17a).
    pub _pad_0176: [u8; 0x4],
    /// field_17a (confidence: low, kind: int?, lanes: c-physics).
    pub field_17a: [u8; 4],
    /// Unknown bytes (0x17e..0x220).
    pub _pad_017e: [u8; 0xa2],
    /// ptr_220 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_220: Ptr32<u8>,
}
assert_size!(RagePhBoundBox, 0x224); // merged size 0x224 rounded to 4
assert_offset!(RagePhBoundBox, float_d0, 0xd0);
assert_offset!(RagePhBoundBox, float_d4, 0xd4);
assert_offset!(RagePhBoundBox, float_e0, 0xe0);
assert_offset!(RagePhBoundBox, float_e4, 0xe4);
assert_offset!(RagePhBoundBox, float_e8, 0xe8);
assert_offset!(RagePhBoundBox, float_100, 0x100);
assert_offset!(RagePhBoundBox, float_104, 0x104);
assert_offset!(RagePhBoundBox, float_108, 0x108);
assert_offset!(RagePhBoundBox, float_110, 0x110);
assert_offset!(RagePhBoundBox, float_114, 0x114);
assert_offset!(RagePhBoundBox, float_118, 0x118);
assert_offset!(RagePhBoundBox, float_120, 0x120);
assert_offset!(RagePhBoundBox, float_124, 0x124);
assert_offset!(RagePhBoundBox, float_128, 0x128);
assert_offset!(RagePhBoundBox, float_130, 0x130);
assert_offset!(RagePhBoundBox, float_134, 0x134);
assert_offset!(RagePhBoundBox, float_138, 0x138);
assert_offset!(RagePhBoundBox, float_140, 0x140);
assert_offset!(RagePhBoundBox, float_144, 0x144);
assert_offset!(RagePhBoundBox, float_148, 0x148);
assert_offset!(RagePhBoundBox, float_150, 0x150);
assert_offset!(RagePhBoundBox, float_154, 0x154);
assert_offset!(RagePhBoundBox, float_158, 0x158);
assert_offset!(RagePhBoundBox, field_160, 0x160);
assert_offset!(RagePhBoundBox, field_164, 0x164);
assert_offset!(RagePhBoundBox, field_172, 0x172);
assert_offset!(RagePhBoundBox, field_17a, 0x17a);
assert_offset!(RagePhBoundBox, ptr_220, 0x220);

/// Merged layout for `rage::phBoundCapsule`.
///
/// Size: 0xe0 (high). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundCapsule {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// float_80 (confidence: high, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: high, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// float_88 (confidence: high, kind: float, lanes: c-physics).
    pub float_88: f32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// float_90 (confidence: high, kind: float, lanes: c-physics).
    pub float_90: f32,
    /// float_94 (confidence: high, kind: float, lanes: c-physics).
    pub float_94: f32,
    /// float_98 (confidence: high, kind: float, lanes: c-physics).
    pub float_98: f32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// float_a0 (confidence: high, kind: float, lanes: c-physics).
    pub float_a0: f32,
    /// float_a4 (confidence: high, kind: float, lanes: c-physics).
    pub float_a4: f32,
    /// float_a8 (confidence: high, kind: float, lanes: c-physics).
    pub float_a8: f32,
    /// float_ac (confidence: medium, kind: float, lanes: c-physics).
    pub float_ac: f32,
    /// float_b0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_b0: f32,
    /// float_b4 (confidence: medium, kind: float, lanes: c-physics).
    pub float_b4: f32,
    /// float_b8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_b8: f32,
    /// float_bc (confidence: medium, kind: float, lanes: c-physics).
    pub float_bc: f32,
    /// float_c0 (confidence: high, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// float_c4 (confidence: high, kind: float, lanes: c-physics).
    pub float_c4: f32,
    /// float_c8 (confidence: high, kind: float, lanes: c-physics).
    pub float_c8: f32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// ptr_d0 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_d0: Ptr32<u8>,
    /// Unknown bytes (0xd4..0xdc).
    pub _pad_00d4: [u8; 0x8],
    /// field_dc (confidence: low, kind: int?, lanes: c-physics).
    pub field_dc: u32,
}
assert_size!(RagePhBoundCapsule, 0xe0); // merged size 0xe0 rounded to 4
assert_offset!(RagePhBoundCapsule, float_80, 0x80);
assert_offset!(RagePhBoundCapsule, float_84, 0x84);
assert_offset!(RagePhBoundCapsule, float_88, 0x88);
assert_offset!(RagePhBoundCapsule, float_90, 0x90);
assert_offset!(RagePhBoundCapsule, float_94, 0x94);
assert_offset!(RagePhBoundCapsule, float_98, 0x98);
assert_offset!(RagePhBoundCapsule, float_a0, 0xa0);
assert_offset!(RagePhBoundCapsule, float_a4, 0xa4);
assert_offset!(RagePhBoundCapsule, float_a8, 0xa8);
assert_offset!(RagePhBoundCapsule, float_ac, 0xac);
assert_offset!(RagePhBoundCapsule, float_b0, 0xb0);
assert_offset!(RagePhBoundCapsule, float_b4, 0xb4);
assert_offset!(RagePhBoundCapsule, float_b8, 0xb8);
assert_offset!(RagePhBoundCapsule, float_bc, 0xbc);
assert_offset!(RagePhBoundCapsule, float_c0, 0xc0);
assert_offset!(RagePhBoundCapsule, float_c4, 0xc4);
assert_offset!(RagePhBoundCapsule, float_c8, 0xc8);
assert_offset!(RagePhBoundCapsule, ptr_d0, 0xd0);
assert_offset!(RagePhBoundCapsule, field_dc, 0xdc);

/// Merged layout for `rage::phBoundComposite`.
///
/// Size: 0xe0 (high). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundComposite {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: high, kind: int?, lanes: c-physics).
    pub field_80: u32,
    /// ptr_84 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_84: Ptr32<u8>,
    /// field_88 (confidence: high, kind: int?, lanes: c-physics).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_90: u16,
    /// field_92 (confidence: high, kind: int16?, lanes: c-physics).
    pub field_92: u16,
    /// field_94 (confidence: high, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// Unknown trailing bytes (0x98..0xe0).
    pub _pad_end: [u8; 0x48],
}
assert_size!(RagePhBoundComposite, 0xe0); // merged size 0xe0 rounded to 4
assert_offset!(RagePhBoundComposite, field_80, 0x80);
assert_offset!(RagePhBoundComposite, ptr_84, 0x84);
assert_offset!(RagePhBoundComposite, field_88, 0x88);
assert_offset!(RagePhBoundComposite, field_90, 0x90);
assert_offset!(RagePhBoundComposite, field_92, 0x92);
assert_offset!(RagePhBoundComposite, field_94, 0x94);

/// Merged layout for `rage::phBoundGrid`.
///
/// Size: 0xf0 (high). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundGrid {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// float_80 (confidence: high, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: high, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// field_88 (confidence: high, kind: int?, lanes: c-physics).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: high, kind: int?, lanes: c-physics).
    pub field_90: u32,
    /// field_94 (confidence: high, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// field_98 (confidence: high, kind: int?, lanes: c-physics).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// field_a0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_a0: u32,
    /// ptr_a4 (confidence: high, kind: int16?, lanes: c-physics).
    pub ptr_a4: u16,
    /// field_a6 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_a6: u16,
    /// field_a8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_a8: u32,
    /// field_ac (confidence: high, kind: int8/bool?, lanes: c-physics).
    pub field_ac: u8,
    /// field_ad (confidence: high, kind: int8/bool?, lanes: c-physics).
    pub field_ad: u8,
    /// Unknown trailing bytes (0xae..0xf0).
    pub _pad_end: [u8; 0x42],
}
assert_size!(RagePhBoundGrid, 0xf0); // merged size 0xf0 rounded to 4
assert_offset!(RagePhBoundGrid, float_80, 0x80);
assert_offset!(RagePhBoundGrid, float_84, 0x84);
assert_offset!(RagePhBoundGrid, field_88, 0x88);
assert_offset!(RagePhBoundGrid, field_90, 0x90);
assert_offset!(RagePhBoundGrid, field_94, 0x94);
assert_offset!(RagePhBoundGrid, field_98, 0x98);
assert_offset!(RagePhBoundGrid, field_a0, 0xa0);
assert_offset!(RagePhBoundGrid, ptr_a4, 0xa4);
assert_offset!(RagePhBoundGrid, field_a6, 0xa6);
assert_offset!(RagePhBoundGrid, field_a8, 0xa8);
assert_offset!(RagePhBoundGrid, field_ac, 0xac);
assert_offset!(RagePhBoundGrid, field_ad, 0xad);

/// Merged layout for `rage::phBoundPolyhedron`.
///
/// Size: 0xd0 (low). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundPolyhedron {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: low, kind: int?, lanes: c-physics).
    pub field_80: u32,
    /// field_84 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_84: u32,
    /// field_88 (confidence: high, kind: int?, lanes: c-physics).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// float_90 (confidence: low, kind: float, lanes: c-physics).
    pub float_90: f32,
    /// float_94 (confidence: low, kind: float, lanes: c-physics).
    pub float_94: f32,
    /// float_98 (confidence: low, kind: float, lanes: c-physics).
    pub float_98: f32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// float_a0 (confidence: low, kind: float, lanes: c-physics).
    pub float_a0: f32,
    /// float_a4 (confidence: low, kind: float, lanes: c-physics).
    pub float_a4: f32,
    /// float_a8 (confidence: low, kind: float, lanes: c-physics).
    pub float_a8: f32,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// field_b0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_b0: u32,
    /// field_b4 (confidence: high, kind: int?, lanes: c-physics).
    pub field_b4: u32,
    /// bool_b8 (confidence: high, kind: bool, lanes: c-physics).
    pub bool_b8: u8,
    /// Unknown bytes (0xb9..0xbc).
    pub _pad_00b9: [u8; 0x3],
    /// field_bc (confidence: medium, kind: int?, lanes: c-physics).
    pub field_bc: u32,
    /// field_c0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_c0: u32,
    /// field_c4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_c4: u32,
    /// field_c8 (confidence: high, kind: int?, lanes: c-physics).
    pub field_c8: u32,
    /// Unknown trailing bytes (0xcc..0xd0).
    pub _pad_end: [u8; 0x4],
}
assert_size!(RagePhBoundPolyhedron, 0xd0); // merged size 0xd0 rounded to 4
assert_offset!(RagePhBoundPolyhedron, field_80, 0x80);
assert_offset!(RagePhBoundPolyhedron, field_84, 0x84);
assert_offset!(RagePhBoundPolyhedron, field_88, 0x88);
assert_offset!(RagePhBoundPolyhedron, float_90, 0x90);
assert_offset!(RagePhBoundPolyhedron, float_94, 0x94);
assert_offset!(RagePhBoundPolyhedron, float_98, 0x98);
assert_offset!(RagePhBoundPolyhedron, float_a0, 0xa0);
assert_offset!(RagePhBoundPolyhedron, float_a4, 0xa4);
assert_offset!(RagePhBoundPolyhedron, float_a8, 0xa8);
assert_offset!(RagePhBoundPolyhedron, field_b0, 0xb0);
assert_offset!(RagePhBoundPolyhedron, field_b4, 0xb4);
assert_offset!(RagePhBoundPolyhedron, bool_b8, 0xb8);
assert_offset!(RagePhBoundPolyhedron, field_bc, 0xbc);
assert_offset!(RagePhBoundPolyhedron, field_c0, 0xc0);
assert_offset!(RagePhBoundPolyhedron, field_c4, 0xc4);
assert_offset!(RagePhBoundPolyhedron, field_c8, 0xc8);

/// Merged layout for `rage::phBoundRibbon`.
///
/// Size: 0xf0 (high). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundRibbon {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: high, kind: int?, lanes: c-physics).
    pub field_80: u32,
    /// ptr_84 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_84: Ptr32<u8>,
    /// field_88 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_90: u32,
    /// field_94 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// field_98 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// field_a0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_a0: u32,
    /// field_a4 (confidence: high, kind: int?, lanes: c-physics).
    pub field_a4: u32,
    /// field_a8 (confidence: high, kind: int?, lanes: c-physics).
    pub field_a8: u32,
    /// field_ac (confidence: high, kind: int?, lanes: c-physics).
    pub field_ac: u32,
    /// field_b0 (confidence: high, kind: int?, lanes: c-physics).
    pub field_b0: u32,
    /// ptr_b4 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_b4: Ptr32<u8>,
    /// field_b8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_b8: u32,
    /// field_bc (confidence: medium, kind: int?, lanes: c-physics).
    pub field_bc: u32,
    /// float_c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// field_c4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_c4: u32,
    /// Unknown bytes (0xc8..0xd0).
    pub _pad_00c8: [u8; 0x8],
    /// field_d0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_d0: u32,
    /// Unknown trailing bytes (0xd4..0xf0).
    pub _pad_end: [u8; 0x1c],
}
assert_size!(RagePhBoundRibbon, 0xf0); // merged size 0xf0 rounded to 4
assert_offset!(RagePhBoundRibbon, field_80, 0x80);
assert_offset!(RagePhBoundRibbon, ptr_84, 0x84);
assert_offset!(RagePhBoundRibbon, field_88, 0x88);
assert_offset!(RagePhBoundRibbon, field_90, 0x90);
assert_offset!(RagePhBoundRibbon, field_94, 0x94);
assert_offset!(RagePhBoundRibbon, field_98, 0x98);
assert_offset!(RagePhBoundRibbon, field_a0, 0xa0);
assert_offset!(RagePhBoundRibbon, field_a4, 0xa4);
assert_offset!(RagePhBoundRibbon, field_a8, 0xa8);
assert_offset!(RagePhBoundRibbon, field_ac, 0xac);
assert_offset!(RagePhBoundRibbon, field_b0, 0xb0);
assert_offset!(RagePhBoundRibbon, ptr_b4, 0xb4);
assert_offset!(RagePhBoundRibbon, field_b8, 0xb8);
assert_offset!(RagePhBoundRibbon, field_bc, 0xbc);
assert_offset!(RagePhBoundRibbon, float_c0, 0xc0);
assert_offset!(RagePhBoundRibbon, field_c4, 0xc4);
assert_offset!(RagePhBoundRibbon, field_d0, 0xd0);

/// Merged layout for `rage::phBoundSphere`.
///
/// Size: 0xf0 (high). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundSphere {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// float_80 (confidence: high, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// float_84 (confidence: high, kind: float, lanes: c-physics).
    pub float_84: f32,
    /// float_88 (confidence: high, kind: float, lanes: c-physics).
    pub float_88: f32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// ptr_90 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: int?, lanes: c-physics).
    pub field_94: [u8; 8],
    /// Unknown trailing bytes (0x9c..0xf0).
    pub _pad_end: [u8; 0x54],
}
assert_size!(RagePhBoundSphere, 0xf0); // merged size 0xf0 rounded to 4
assert_offset!(RagePhBoundSphere, float_80, 0x80);
assert_offset!(RagePhBoundSphere, float_84, 0x84);
assert_offset!(RagePhBoundSphere, float_88, 0x88);
assert_offset!(RagePhBoundSphere, ptr_90, 0x90);
assert_offset!(RagePhBoundSphere, field_94, 0x94);

/// Merged layout for `rage::phBoundTaperedCapsule`.
///
/// Size: 0x104 (low). Bases: rage::phBound@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBoundTaperedCapsule {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// float_80 (confidence: high, kind: float, lanes: c-physics).
    pub float_80: f32,
    /// Unknown bytes (0x84..0x90).
    pub _pad_0084: [u8; 0xc],
    /// float_90 (confidence: high, kind: float, lanes: c-physics).
    pub float_90: f32,
    /// Unknown bytes (0x94..0xa0).
    pub _pad_0094: [u8; 0xc],
    /// float_a0 (confidence: high, kind: float, lanes: c-physics).
    pub float_a0: f32,
    /// float_a4 (confidence: low, kind: float, lanes: c-physics).
    pub float_a4: f32,
    /// float_a8 (confidence: low, kind: float, lanes: c-physics).
    pub float_a8: f32,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// float_b0 (confidence: high, kind: float, lanes: c-physics).
    pub float_b0: f32,
    /// float_b4 (confidence: medium, kind: float, lanes: c-physics).
    pub float_b4: f32,
    /// float_b8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_b8: f32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// float_c0 (confidence: high, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// Unknown bytes (0xc4..0xd0).
    pub _pad_00c4: [u8; 0xc],
    /// float_d0 (confidence: high, kind: float, lanes: c-physics).
    pub float_d0: f32,
    /// float_d4 (confidence: high, kind: float, lanes: c-physics).
    pub float_d4: f32,
    /// Unknown bytes (0xd8..0xdc).
    pub _pad_00d8: [u8; 0x4],
    /// float_dc (confidence: low, kind: float, lanes: c-physics).
    pub float_dc: f32,
    /// float_e0 (confidence: low, kind: float, lanes: c-physics).
    pub float_e0: f32,
    /// float_e4 (confidence: low, kind: float, lanes: c-physics).
    pub float_e4: f32,
    /// float_e8 (confidence: low, kind: float, lanes: c-physics).
    pub float_e8: f32,
    /// float_ec (confidence: low, kind: float, lanes: c-physics).
    pub float_ec: f32,
    /// Unknown bytes (0xf0..0x100).
    pub _pad_00f0: [u8; 0x10],
    /// ptr_100 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_100: Ptr32<u8>,
}
assert_size!(RagePhBoundTaperedCapsule, 0x104); // merged size 0x104 rounded to 4
assert_offset!(RagePhBoundTaperedCapsule, float_80, 0x80);
assert_offset!(RagePhBoundTaperedCapsule, float_90, 0x90);
assert_offset!(RagePhBoundTaperedCapsule, float_a0, 0xa0);
assert_offset!(RagePhBoundTaperedCapsule, float_a4, 0xa4);
assert_offset!(RagePhBoundTaperedCapsule, float_a8, 0xa8);
assert_offset!(RagePhBoundTaperedCapsule, float_b0, 0xb0);
assert_offset!(RagePhBoundTaperedCapsule, float_b4, 0xb4);
assert_offset!(RagePhBoundTaperedCapsule, float_b8, 0xb8);
assert_offset!(RagePhBoundTaperedCapsule, float_c0, 0xc0);
assert_offset!(RagePhBoundTaperedCapsule, float_d0, 0xd0);
assert_offset!(RagePhBoundTaperedCapsule, float_d4, 0xd4);
assert_offset!(RagePhBoundTaperedCapsule, float_dc, 0xdc);
assert_offset!(RagePhBoundTaperedCapsule, float_e0, 0xe0);
assert_offset!(RagePhBoundTaperedCapsule, float_e4, 0xe4);
assert_offset!(RagePhBoundTaperedCapsule, float_e8, 0xe8);
assert_offset!(RagePhBoundTaperedCapsule, float_ec, 0xec);
assert_offset!(RagePhBoundTaperedCapsule, ptr_100, 0x100);

/// Merged layout for `rage::phBroadPhase`.
///
/// Size: 0x16 (low). Bases: none.
/// Lanes: c-physics, via:rage::btImmediateModeBroadphase, via:rage::phLevelBroadPhase.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhBroadPhase {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: int?, lanes: c-physics,via:rage::btImmediateModeBroadphase,via:rage::phLevelBroadPhase moved from siblings:rage::btImmediateModeBroadphase,rage::phLevelBroadPhase).
    pub field_8: u32,
    /// field_c (confidence: high, kind: int?, lanes: c-physics,via:rage::btImmediateModeBroadphase,via:rage::phLevelBroadPhase moved from siblings:rage::btImmediateModeBroadphase,rage::phLevelBroadPhase).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: high, kind: int16?, lanes: c-physics,via:rage::btImmediateModeBroadphase,via:rage::phLevelBroadPhase moved from siblings:rage::btImmediateModeBroadphase,rage::phLevelBroadPhase).
    pub field_14: u16,
    /// Unknown trailing bytes (0x16..0x18).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RagePhBroadPhase, 0x18); // merged size 0x16 rounded to 4
assert_offset!(RagePhBroadPhase, vfptr, 0x0);
assert_offset!(RagePhBroadPhase, field_4, 0x4);
assert_offset!(RagePhBroadPhase, field_8, 0x8);
assert_offset!(RagePhBroadPhase, field_c, 0xc);
assert_offset!(RagePhBroadPhase, field_14, 0x14);

/// Merged layout for `rage::phCollider`.
///
/// Size: 0x2ec (low). Bases: none.
/// Lanes: c-physics, via:rage::phArticulatedCollider, via:rage::phConstrainedCollider.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhCollider {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: high, kind: int?, lanes: c-physics).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: int?, lanes: c-physics,via:rage::phArticulatedCollider,via:rage::phConstrainedCollider moved from siblings:rage::phArticulatedCollider,rage::phConstrainedCollider).
    pub field_14: u32,
    /// ptr_18 (confidence: high, kind: pointer, lanes: c-physics).
    pub ptr_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// ptr_20 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_20: Ptr32<u8>,
    /// ptr_24 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_24: Ptr32<u8>,
    /// field_28 (confidence: low, kind: int?, lanes: c-physics).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// field_30 (confidence: low, kind: int?, lanes: c-physics).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: int?, lanes: c-physics).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: int?, lanes: c-physics).
    pub field_38: u32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// field_40 (confidence: low, kind: int?, lanes: c-physics).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: int?, lanes: c-physics).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: int?, lanes: c-physics).
    pub field_48: u32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// field_50 (confidence: low, kind: int?, lanes: c-physics).
    pub field_50: u32,
    /// field_54 (confidence: low, kind: int?, lanes: c-physics).
    pub field_54: u32,
    /// field_58 (confidence: low, kind: int?, lanes: c-physics).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: low, kind: int?, lanes: c-physics).
    pub field_60: u32,
    /// field_64 (confidence: low, kind: int?, lanes: c-physics).
    pub field_64: u32,
    /// field_68 (confidence: low, kind: int?, lanes: c-physics).
    pub field_68: u32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: low, kind: int?, lanes: c-physics).
    pub field_70: u32,
    /// field_74 (confidence: low, kind: int?, lanes: c-physics).
    pub field_74: u32,
    /// field_78 (confidence: low, kind: int?, lanes: c-physics).
    pub field_78: u32,
    /// Unknown bytes (0x7c..0x80).
    pub _pad_007c: [u8; 0x4],
    /// field_80 (confidence: low, kind: int?, lanes: c-physics).
    pub field_80: u32,
    /// field_84 (confidence: low, kind: int?, lanes: c-physics).
    pub field_84: u32,
    /// field_88 (confidence: low, kind: int?, lanes: c-physics).
    pub field_88: u32,
    /// Unknown bytes (0x8c..0x90).
    pub _pad_008c: [u8; 0x4],
    /// field_90 (confidence: low, kind: int?, lanes: c-physics).
    pub field_90: u32,
    /// field_94 (confidence: low, kind: int?, lanes: c-physics).
    pub field_94: u32,
    /// field_98 (confidence: low, kind: int?, lanes: c-physics).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// float_a0 (confidence: low, kind: float, lanes: c-physics).
    pub float_a0: f32,
    /// float_a4 (confidence: low, kind: float, lanes: c-physics).
    pub float_a4: f32,
    /// float_a8 (confidence: low, kind: float, lanes: c-physics).
    pub float_a8: f32,
    /// Unknown bytes (0xac..0xb0).
    pub _pad_00ac: [u8; 0x4],
    /// float_b0 (confidence: high, kind: float, lanes: c-physics).
    pub float_b0: f32,
    /// float_b4 (confidence: high, kind: float, lanes: c-physics).
    pub float_b4: f32,
    /// float_b8 (confidence: high, kind: float, lanes: c-physics).
    pub float_b8: f32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// float_c0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// float_c4 (confidence: high, kind: float, lanes: c-physics).
    pub float_c4: f32,
    /// Unknown bytes (0xc8..0xd0).
    pub _pad_00c8: [u8; 0x8],
    /// float_d0 (confidence: high, kind: float, lanes: c-physics).
    pub float_d0: f32,
    /// float_d4 (confidence: high, kind: float, lanes: c-physics).
    pub float_d4: f32,
    /// float_d8 (confidence: high, kind: float, lanes: c-physics).
    pub float_d8: f32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// float_e0 (confidence: high, kind: float, lanes: c-physics).
    pub float_e0: f32,
    /// float_e4 (confidence: high, kind: float, lanes: c-physics).
    pub float_e4: f32,
    /// float_e8 (confidence: high, kind: float, lanes: c-physics).
    pub float_e8: f32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// float_f0 (confidence: high, kind: float, lanes: c-physics).
    pub float_f0: f32,
    /// float_f4 (confidence: high, kind: float, lanes: c-physics).
    pub float_f4: f32,
    /// float_f8 (confidence: high, kind: float, lanes: c-physics).
    pub float_f8: f32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// float_100 (confidence: high, kind: float, lanes: c-physics).
    pub float_100: f32,
    /// float_104 (confidence: high, kind: float, lanes: c-physics).
    pub float_104: f32,
    /// float_108 (confidence: high, kind: float, lanes: c-physics).
    pub float_108: f32,
    /// Unknown bytes (0x10c..0x110).
    pub _pad_010c: [u8; 0x4],
    /// float_110 (confidence: high, kind: float, lanes: c-physics).
    pub float_110: f32,
    /// float_114 (confidence: high, kind: float, lanes: c-physics).
    pub float_114: f32,
    /// float_118 (confidence: high, kind: float, lanes: c-physics).
    pub float_118: f32,
    /// Unknown bytes (0x11c..0x120).
    pub _pad_011c: [u8; 0x4],
    /// float_120 (confidence: high, kind: float, lanes: c-physics).
    pub float_120: f32,
    /// float_124 (confidence: high, kind: float, lanes: c-physics).
    pub float_124: f32,
    /// float_128 (confidence: high, kind: float, lanes: c-physics).
    pub float_128: f32,
    /// float_12c (confidence: low, kind: float, lanes: c-physics).
    pub float_12c: f32,
    /// float_130 (confidence: high, kind: float, lanes: c-physics).
    pub float_130: f32,
    /// float_134 (confidence: high, kind: float, lanes: c-physics).
    pub float_134: f32,
    /// float_138 (confidence: high, kind: float, lanes: c-physics).
    pub float_138: f32,
    /// Unknown bytes (0x13c..0x140).
    pub _pad_013c: [u8; 0x4],
    /// float_140 (confidence: high, kind: float, lanes: c-physics).
    pub float_140: f32,
    /// float_144 (confidence: high, kind: float, lanes: c-physics).
    pub float_144: f32,
    /// float_148 (confidence: high, kind: float, lanes: c-physics).
    pub float_148: f32,
    /// float_14c (confidence: low, kind: float, lanes: c-physics).
    pub float_14c: f32,
    /// float_150 (confidence: high, kind: float, lanes: c-physics).
    pub float_150: f32,
    /// float_154 (confidence: high, kind: float, lanes: c-physics).
    pub float_154: f32,
    /// float_158 (confidence: high, kind: float, lanes: c-physics).
    pub float_158: f32,
    /// Unknown bytes (0x15c..0x160).
    pub _pad_015c: [u8; 0x4],
    /// float_160 (confidence: high, kind: float, lanes: c-physics).
    pub float_160: f32,
    /// float_164 (confidence: high, kind: float, lanes: c-physics).
    pub float_164: f32,
    /// float_168 (confidence: high, kind: float, lanes: c-physics).
    pub float_168: f32,
    /// Unknown bytes (0x16c..0x170).
    pub _pad_016c: [u8; 0x4],
    /// float_170 (confidence: high, kind: float, lanes: c-physics).
    pub float_170: f32,
    /// float_174 (confidence: high, kind: float, lanes: c-physics).
    pub float_174: f32,
    /// float_178 (confidence: high, kind: float, lanes: c-physics).
    pub float_178: f32,
    /// Unknown bytes (0x17c..0x180).
    pub _pad_017c: [u8; 0x4],
    /// float_180 (confidence: high, kind: float, lanes: c-physics).
    pub float_180: f32,
    /// float_184 (confidence: high, kind: float, lanes: c-physics).
    pub float_184: f32,
    /// float_188 (confidence: high, kind: float, lanes: c-physics).
    pub float_188: f32,
    /// Unknown bytes (0x18c..0x190).
    pub _pad_018c: [u8; 0x4],
    /// float_190 (confidence: high, kind: float, lanes: c-physics).
    pub float_190: f32,
    /// float_194 (confidence: high, kind: float, lanes: c-physics).
    pub float_194: f32,
    /// float_198 (confidence: high, kind: float, lanes: c-physics).
    pub float_198: f32,
    /// Unknown bytes (0x19c..0x1a0).
    pub _pad_019c: [u8; 0x4],
    /// float_1a0 (confidence: high, kind: float, lanes: c-physics).
    pub float_1a0: f32,
    /// float_1a4 (confidence: high, kind: float, lanes: c-physics).
    pub float_1a4: f32,
    /// float_1a8 (confidence: high, kind: float, lanes: c-physics).
    pub float_1a8: f32,
    /// Unknown bytes (0x1ac..0x1b0).
    pub _pad_01ac: [u8; 0x4],
    /// field_1b0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1b0: u32,
    /// field_1b4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1b4: u32,
    /// field_1b8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1b8: u32,
    /// Unknown bytes (0x1bc..0x1c0).
    pub _pad_01bc: [u8; 0x4],
    /// field_1c0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1c0: u32,
    /// field_1c4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1c4: u32,
    /// field_1c8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_1c8: u32,
    /// Unknown bytes (0x1cc..0x1d0).
    pub _pad_01cc: [u8; 0x4],
    /// float_1d0 (confidence: high, kind: float, lanes: c-physics).
    pub float_1d0: f32,
    /// float_1d4 (confidence: high, kind: float, lanes: c-physics).
    pub float_1d4: f32,
    /// float_1d8 (confidence: high, kind: float, lanes: c-physics).
    pub float_1d8: f32,
    /// Unknown bytes (0x1dc..0x1e0).
    pub _pad_01dc: [u8; 0x4],
    /// float_1e0 (confidence: high, kind: float, lanes: c-physics).
    pub float_1e0: f32,
    /// float_1e4 (confidence: high, kind: float, lanes: c-physics).
    pub float_1e4: f32,
    /// float_1e8 (confidence: high, kind: float, lanes: c-physics).
    pub float_1e8: f32,
    /// Unknown bytes (0x1ec..0x1f0).
    pub _pad_01ec: [u8; 0x4],
    /// float_1f0 (confidence: high, kind: float, lanes: c-physics).
    pub float_1f0: f32,
    /// float_1f4 (confidence: high, kind: float, lanes: c-physics).
    pub float_1f4: f32,
    /// float_1f8 (confidence: high, kind: float, lanes: c-physics).
    pub float_1f8: f32,
    /// Unknown bytes (0x1fc..0x200).
    pub _pad_01fc: [u8; 0x4],
    /// float_200 (confidence: medium, kind: float, lanes: c-physics).
    pub float_200: f32,
    /// float_204 (confidence: medium, kind: float, lanes: c-physics).
    pub float_204: f32,
    /// float_208 (confidence: medium, kind: float, lanes: c-physics).
    pub float_208: f32,
    /// Unknown bytes (0x20c..0x210).
    pub _pad_020c: [u8; 0x4],
    /// float_210 (confidence: high, kind: float, lanes: c-physics).
    pub float_210: f32,
    /// float_214 (confidence: high, kind: float, lanes: c-physics).
    pub float_214: f32,
    /// float_218 (confidence: high, kind: float, lanes: c-physics).
    pub float_218: f32,
    /// Unknown bytes (0x21c..0x220).
    pub _pad_021c: [u8; 0x4],
    /// float_220 (confidence: high, kind: float, lanes: c-physics).
    pub float_220: f32,
    /// float_224 (confidence: high, kind: float, lanes: c-physics).
    pub float_224: f32,
    /// float_228 (confidence: high, kind: float, lanes: c-physics).
    pub float_228: f32,
    /// Unknown bytes (0x22c..0x230).
    pub _pad_022c: [u8; 0x4],
    /// float_230 (confidence: high, kind: float, lanes: c-physics).
    pub float_230: f32,
    /// float_234 (confidence: high, kind: float, lanes: c-physics).
    pub float_234: f32,
    /// float_238 (confidence: high, kind: float, lanes: c-physics).
    pub float_238: f32,
    /// Unknown bytes (0x23c..0x240).
    pub _pad_023c: [u8; 0x4],
    /// float_240 (confidence: medium, kind: float, lanes: c-physics).
    pub float_240: f32,
    /// float_244 (confidence: medium, kind: float, lanes: c-physics).
    pub float_244: f32,
    /// float_248 (confidence: medium, kind: float, lanes: c-physics).
    pub float_248: f32,
    /// Unknown bytes (0x24c..0x250).
    pub _pad_024c: [u8; 0x4],
    /// float_250 (confidence: high, kind: float, lanes: c-physics).
    pub float_250: f32,
    /// float_254 (confidence: high, kind: float, lanes: c-physics).
    pub float_254: f32,
    /// float_258 (confidence: high, kind: float, lanes: c-physics).
    pub float_258: f32,
    /// Unknown bytes (0x25c..0x260).
    pub _pad_025c: [u8; 0x4],
    /// float_260 (confidence: high, kind: float, lanes: c-physics).
    pub float_260: f32,
    /// float_264 (confidence: high, kind: float, lanes: c-physics).
    pub float_264: f32,
    /// float_268 (confidence: high, kind: float, lanes: c-physics).
    pub float_268: f32,
    /// Unknown bytes (0x26c..0x270).
    pub _pad_026c: [u8; 0x4],
    /// float_270 (confidence: high, kind: float, lanes: c-physics).
    pub float_270: f32,
    /// float_274 (confidence: high, kind: float, lanes: c-physics).
    pub float_274: f32,
    /// float_278 (confidence: high, kind: float, lanes: c-physics).
    pub float_278: f32,
    /// Unknown bytes (0x27c..0x280).
    pub _pad_027c: [u8; 0x4],
    /// field_280 (confidence: low, kind: int?, lanes: c-physics).
    pub field_280: u32,
    /// field_284 (confidence: low, kind: int?, lanes: c-physics).
    pub field_284: u32,
    /// field_288 (confidence: low, kind: int?, lanes: c-physics).
    pub field_288: u32,
    /// Unknown bytes (0x28c..0x290).
    pub _pad_028c: [u8; 0x4],
    /// bool_290 (confidence: high, kind: bool, lanes: c-physics).
    pub bool_290: u8,
    /// bool_291 (confidence: medium, kind: bool, lanes: c-physics).
    pub bool_291: u8,
    /// bool_292 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_292: u8,
    /// bool_293 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_293: u8,
    /// bool_294 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_294: u8,
    /// Unknown bytes (0x295..0x298).
    pub _pad_0295: [u8; 0x3],
    /// field_298 (confidence: high, kind: int?, lanes: c-physics).
    pub field_298: u32,
    /// field_29c (confidence: low, kind: int?, lanes: c-physics).
    pub field_29c: u32,
    /// Unknown bytes (0x2a0..0x2ac).
    pub _pad_02a0: [u8; 0xc],
    /// field_2ac (confidence: low, kind: int?, lanes: c-physics).
    pub field_2ac: u32,
    /// Unknown bytes (0x2b0..0x2b2).
    pub _pad_02b0: [u8; 0x2],
    /// field_2b2 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2b2: u16,
    /// field_2b4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2b4: u32,
    /// Unknown bytes (0x2b8..0x2ba).
    pub _pad_02b8: [u8; 0x2],
    /// field_2ba (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2ba: u16,
    /// field_2bc (confidence: low, kind: int?, lanes: c-physics).
    pub field_2bc: u32,
    /// Unknown bytes (0x2c0..0x2c2).
    pub _pad_02c0: [u8; 0x2],
    /// field_2c2 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2c2: u16,
    /// field_2c4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2c4: u32,
    /// Unknown bytes (0x2c8..0x2ca).
    pub _pad_02c8: [u8; 0x2],
    /// field_2ca (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2ca: u16,
    /// field_2cc (confidence: low, kind: int?, lanes: c-physics).
    pub field_2cc: u32,
    /// Unknown bytes (0x2d0..0x2d2).
    pub _pad_02d0: [u8; 0x2],
    /// field_2d2 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2d2: u16,
    /// field_2d4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2d4: u32,
    /// Unknown bytes (0x2d8..0x2da).
    pub _pad_02d8: [u8; 0x2],
    /// field_2da (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2da: u16,
    /// field_2dc (confidence: low, kind: int?, lanes: c-physics).
    pub field_2dc: u32,
    /// Unknown bytes (0x2e0..0x2e2).
    pub _pad_02e0: [u8; 0x2],
    /// field_2e2 (confidence: low, kind: int16?, lanes: c-physics).
    pub field_2e2: u16,
    /// Unknown bytes (0x2e4..0x2e8).
    pub _pad_02e4: [u8; 0x4],
    /// field_2e8 (confidence: high, kind: int?, lanes: c-physics,via:rage::phArticulatedCollider,via:rage::phConstrainedCollider moved from siblings:rage::phArticulatedCollider,rage::phConstrainedCollider).
    pub field_2e8: u32,
}
assert_size!(RagePhCollider, 0x2ec); // merged size 0x2ec rounded to 4
assert_offset!(RagePhCollider, vfptr, 0x0);
assert_offset!(RagePhCollider, field_10, 0x10);
assert_offset!(RagePhCollider, field_14, 0x14);
assert_offset!(RagePhCollider, ptr_18, 0x18);
assert_offset!(RagePhCollider, ptr_20, 0x20);
assert_offset!(RagePhCollider, ptr_24, 0x24);
assert_offset!(RagePhCollider, field_28, 0x28);
assert_offset!(RagePhCollider, field_30, 0x30);
assert_offset!(RagePhCollider, field_34, 0x34);
assert_offset!(RagePhCollider, field_38, 0x38);
assert_offset!(RagePhCollider, field_40, 0x40);
assert_offset!(RagePhCollider, field_44, 0x44);
assert_offset!(RagePhCollider, field_48, 0x48);
assert_offset!(RagePhCollider, field_50, 0x50);
assert_offset!(RagePhCollider, field_54, 0x54);
assert_offset!(RagePhCollider, field_58, 0x58);
assert_offset!(RagePhCollider, field_60, 0x60);
assert_offset!(RagePhCollider, field_64, 0x64);
assert_offset!(RagePhCollider, field_68, 0x68);
assert_offset!(RagePhCollider, field_70, 0x70);
assert_offset!(RagePhCollider, field_74, 0x74);
assert_offset!(RagePhCollider, field_78, 0x78);
assert_offset!(RagePhCollider, field_80, 0x80);
assert_offset!(RagePhCollider, field_84, 0x84);
assert_offset!(RagePhCollider, field_88, 0x88);
assert_offset!(RagePhCollider, field_90, 0x90);
assert_offset!(RagePhCollider, field_94, 0x94);
assert_offset!(RagePhCollider, field_98, 0x98);
assert_offset!(RagePhCollider, float_a0, 0xa0);
assert_offset!(RagePhCollider, float_a4, 0xa4);
assert_offset!(RagePhCollider, float_a8, 0xa8);
assert_offset!(RagePhCollider, float_b0, 0xb0);
assert_offset!(RagePhCollider, float_b4, 0xb4);
assert_offset!(RagePhCollider, float_b8, 0xb8);
assert_offset!(RagePhCollider, float_c0, 0xc0);
assert_offset!(RagePhCollider, float_c4, 0xc4);
assert_offset!(RagePhCollider, float_d0, 0xd0);
assert_offset!(RagePhCollider, float_d4, 0xd4);
assert_offset!(RagePhCollider, float_d8, 0xd8);
assert_offset!(RagePhCollider, float_e0, 0xe0);
assert_offset!(RagePhCollider, float_e4, 0xe4);
assert_offset!(RagePhCollider, float_e8, 0xe8);
assert_offset!(RagePhCollider, float_f0, 0xf0);
assert_offset!(RagePhCollider, float_f4, 0xf4);
assert_offset!(RagePhCollider, float_f8, 0xf8);
assert_offset!(RagePhCollider, float_100, 0x100);
assert_offset!(RagePhCollider, float_104, 0x104);
assert_offset!(RagePhCollider, float_108, 0x108);
assert_offset!(RagePhCollider, float_110, 0x110);
assert_offset!(RagePhCollider, float_114, 0x114);
assert_offset!(RagePhCollider, float_118, 0x118);
assert_offset!(RagePhCollider, float_120, 0x120);
assert_offset!(RagePhCollider, float_124, 0x124);
assert_offset!(RagePhCollider, float_128, 0x128);
assert_offset!(RagePhCollider, float_12c, 0x12c);
assert_offset!(RagePhCollider, float_130, 0x130);
assert_offset!(RagePhCollider, float_134, 0x134);
assert_offset!(RagePhCollider, float_138, 0x138);
assert_offset!(RagePhCollider, float_140, 0x140);
assert_offset!(RagePhCollider, float_144, 0x144);
assert_offset!(RagePhCollider, float_148, 0x148);
assert_offset!(RagePhCollider, float_14c, 0x14c);
assert_offset!(RagePhCollider, float_150, 0x150);
assert_offset!(RagePhCollider, float_154, 0x154);
assert_offset!(RagePhCollider, float_158, 0x158);
assert_offset!(RagePhCollider, float_160, 0x160);
assert_offset!(RagePhCollider, float_164, 0x164);
assert_offset!(RagePhCollider, float_168, 0x168);
assert_offset!(RagePhCollider, float_170, 0x170);
assert_offset!(RagePhCollider, float_174, 0x174);
assert_offset!(RagePhCollider, float_178, 0x178);
assert_offset!(RagePhCollider, float_180, 0x180);
assert_offset!(RagePhCollider, float_184, 0x184);
assert_offset!(RagePhCollider, float_188, 0x188);
assert_offset!(RagePhCollider, float_190, 0x190);
assert_offset!(RagePhCollider, float_194, 0x194);
assert_offset!(RagePhCollider, float_198, 0x198);
assert_offset!(RagePhCollider, float_1a0, 0x1a0);
assert_offset!(RagePhCollider, float_1a4, 0x1a4);
assert_offset!(RagePhCollider, float_1a8, 0x1a8);
assert_offset!(RagePhCollider, field_1b0, 0x1b0);
assert_offset!(RagePhCollider, field_1b4, 0x1b4);
assert_offset!(RagePhCollider, field_1b8, 0x1b8);
assert_offset!(RagePhCollider, field_1c0, 0x1c0);
assert_offset!(RagePhCollider, field_1c4, 0x1c4);
assert_offset!(RagePhCollider, field_1c8, 0x1c8);
assert_offset!(RagePhCollider, float_1d0, 0x1d0);
assert_offset!(RagePhCollider, float_1d4, 0x1d4);
assert_offset!(RagePhCollider, float_1d8, 0x1d8);
assert_offset!(RagePhCollider, float_1e0, 0x1e0);
assert_offset!(RagePhCollider, float_1e4, 0x1e4);
assert_offset!(RagePhCollider, float_1e8, 0x1e8);
assert_offset!(RagePhCollider, float_1f0, 0x1f0);
assert_offset!(RagePhCollider, float_1f4, 0x1f4);
assert_offset!(RagePhCollider, float_1f8, 0x1f8);
assert_offset!(RagePhCollider, float_200, 0x200);
assert_offset!(RagePhCollider, float_204, 0x204);
assert_offset!(RagePhCollider, float_208, 0x208);
assert_offset!(RagePhCollider, float_210, 0x210);
assert_offset!(RagePhCollider, float_214, 0x214);
assert_offset!(RagePhCollider, float_218, 0x218);
assert_offset!(RagePhCollider, float_220, 0x220);
assert_offset!(RagePhCollider, float_224, 0x224);
assert_offset!(RagePhCollider, float_228, 0x228);
assert_offset!(RagePhCollider, float_230, 0x230);
assert_offset!(RagePhCollider, float_234, 0x234);
assert_offset!(RagePhCollider, float_238, 0x238);
assert_offset!(RagePhCollider, float_240, 0x240);
assert_offset!(RagePhCollider, float_244, 0x244);
assert_offset!(RagePhCollider, float_248, 0x248);
assert_offset!(RagePhCollider, float_250, 0x250);
assert_offset!(RagePhCollider, float_254, 0x254);
assert_offset!(RagePhCollider, float_258, 0x258);
assert_offset!(RagePhCollider, float_260, 0x260);
assert_offset!(RagePhCollider, float_264, 0x264);
assert_offset!(RagePhCollider, float_268, 0x268);
assert_offset!(RagePhCollider, float_270, 0x270);
assert_offset!(RagePhCollider, float_274, 0x274);
assert_offset!(RagePhCollider, float_278, 0x278);
assert_offset!(RagePhCollider, field_280, 0x280);
assert_offset!(RagePhCollider, field_284, 0x284);
assert_offset!(RagePhCollider, field_288, 0x288);
assert_offset!(RagePhCollider, bool_290, 0x290);
assert_offset!(RagePhCollider, bool_291, 0x291);
assert_offset!(RagePhCollider, bool_292, 0x292);
assert_offset!(RagePhCollider, bool_293, 0x293);
assert_offset!(RagePhCollider, bool_294, 0x294);
assert_offset!(RagePhCollider, field_298, 0x298);
assert_offset!(RagePhCollider, field_29c, 0x29c);
assert_offset!(RagePhCollider, field_2ac, 0x2ac);
assert_offset!(RagePhCollider, field_2b2, 0x2b2);
assert_offset!(RagePhCollider, field_2b4, 0x2b4);
assert_offset!(RagePhCollider, field_2ba, 0x2ba);
assert_offset!(RagePhCollider, field_2bc, 0x2bc);
assert_offset!(RagePhCollider, field_2c2, 0x2c2);
assert_offset!(RagePhCollider, field_2c4, 0x2c4);
assert_offset!(RagePhCollider, field_2ca, 0x2ca);
assert_offset!(RagePhCollider, field_2cc, 0x2cc);
assert_offset!(RagePhCollider, field_2d2, 0x2d2);
assert_offset!(RagePhCollider, field_2d4, 0x2d4);
assert_offset!(RagePhCollider, field_2da, 0x2da);
assert_offset!(RagePhCollider, field_2dc, 0x2dc);
assert_offset!(RagePhCollider, field_2e2, 0x2e2);
assert_offset!(RagePhCollider, field_2e8, 0x2e8);

/// Merged layout for `rage::phConstrainedCollider`.
///
/// Size: 0x380 (medium). Bases: rage::phCollider@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhConstrainedCollider {
    /// Unknown bytes (0x0..0x2a0).
    pub _pad_0000: [u8; 0x2a0],
    /// float_2a0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2a0: f32,
    /// float_2a4 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2a4: f32,
    /// float_2a8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2a8: f32,
    /// Unknown bytes (0x2ac..0x2e4).
    pub _pad_02ac: [u8; 0x38],
    /// field_2e4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2e4: u32,
    /// Unknown bytes (0x2e8..0x2f0).
    pub _pad_02e8: [u8; 0x8],
    /// field_2f0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f4: u32,
    /// field_2f8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f8: u32,
    /// Unknown bytes (0x2fc..0x300).
    pub _pad_02fc: [u8; 0x4],
    /// field_300 (confidence: low, kind: int?, lanes: c-physics).
    pub field_300: u32,
    /// field_304 (confidence: low, kind: int?, lanes: c-physics).
    pub field_304: u32,
    /// field_308 (confidence: low, kind: int?, lanes: c-physics).
    pub field_308: u32,
    /// Unknown bytes (0x30c..0x310).
    pub _pad_030c: [u8; 0x4],
    /// field_310 (confidence: low, kind: int?, lanes: c-physics).
    pub field_310: u32,
    /// field_314 (confidence: low, kind: int?, lanes: c-physics).
    pub field_314: u32,
    /// field_318 (confidence: low, kind: int?, lanes: c-physics).
    pub field_318: u32,
    /// Unknown bytes (0x31c..0x330).
    pub _pad_031c: [u8; 0x14],
    /// field_330 (confidence: low, kind: int?, lanes: c-physics).
    pub field_330: u32,
    /// field_334 (confidence: low, kind: int?, lanes: c-physics).
    pub field_334: u32,
    /// Unknown bytes (0x338..0x340).
    pub _pad_0338: [u8; 0x8],
    /// float_340 (confidence: medium, kind: float, lanes: c-physics).
    pub float_340: f32,
    /// float_344 (confidence: medium, kind: float, lanes: c-physics).
    pub float_344: f32,
    /// float_348 (confidence: medium, kind: float, lanes: c-physics).
    pub float_348: f32,
    /// Unknown bytes (0x34c..0x350).
    pub _pad_034c: [u8; 0x4],
    /// field_350 (confidence: low, kind: int?, lanes: c-physics).
    pub field_350: u32,
    /// field_354 (confidence: low, kind: int?, lanes: c-physics).
    pub field_354: u32,
    /// field_358 (confidence: low, kind: int?, lanes: c-physics).
    pub field_358: u32,
    /// Unknown bytes (0x35c..0x360).
    pub _pad_035c: [u8; 0x4],
    /// field_360 (confidence: low, kind: int?, lanes: c-physics).
    pub field_360: u32,
    /// field_364 (confidence: low, kind: int?, lanes: c-physics).
    pub field_364: u32,
    /// field_368 (confidence: low, kind: int?, lanes: c-physics).
    pub field_368: u32,
    /// Unknown bytes (0x36c..0x370).
    pub _pad_036c: [u8; 0x4],
    /// field_370 (confidence: low, kind: int?, lanes: c-physics).
    pub field_370: u32,
    /// field_374 (confidence: low, kind: int?, lanes: c-physics).
    pub field_374: u32,
    /// Unknown trailing bytes (0x378..0x380).
    pub _pad_end: [u8; 0x8],
}
assert_size!(RagePhConstrainedCollider, 0x380); // merged size 0x380 rounded to 4
assert_offset!(RagePhConstrainedCollider, float_2a0, 0x2a0);
assert_offset!(RagePhConstrainedCollider, float_2a4, 0x2a4);
assert_offset!(RagePhConstrainedCollider, float_2a8, 0x2a8);
assert_offset!(RagePhConstrainedCollider, field_2e4, 0x2e4);
assert_offset!(RagePhConstrainedCollider, field_2f0, 0x2f0);
assert_offset!(RagePhConstrainedCollider, field_2f4, 0x2f4);
assert_offset!(RagePhConstrainedCollider, field_2f8, 0x2f8);
assert_offset!(RagePhConstrainedCollider, field_300, 0x300);
assert_offset!(RagePhConstrainedCollider, field_304, 0x304);
assert_offset!(RagePhConstrainedCollider, field_308, 0x308);
assert_offset!(RagePhConstrainedCollider, field_310, 0x310);
assert_offset!(RagePhConstrainedCollider, field_314, 0x314);
assert_offset!(RagePhConstrainedCollider, field_318, 0x318);
assert_offset!(RagePhConstrainedCollider, field_330, 0x330);
assert_offset!(RagePhConstrainedCollider, field_334, 0x334);
assert_offset!(RagePhConstrainedCollider, float_340, 0x340);
assert_offset!(RagePhConstrainedCollider, float_344, 0x344);
assert_offset!(RagePhConstrainedCollider, float_348, 0x348);
assert_offset!(RagePhConstrainedCollider, field_350, 0x350);
assert_offset!(RagePhConstrainedCollider, field_354, 0x354);
assert_offset!(RagePhConstrainedCollider, field_358, 0x358);
assert_offset!(RagePhConstrainedCollider, field_360, 0x360);
assert_offset!(RagePhConstrainedCollider, field_364, 0x364);
assert_offset!(RagePhConstrainedCollider, field_368, 0x368);
assert_offset!(RagePhConstrainedCollider, field_370, 0x370);
assert_offset!(RagePhConstrainedCollider, field_374, 0x374);

/// Merged layout for `rage::phInst`.
///
/// Size: 0x54 (low). Bases: rage::datBase@0x0.
/// Lanes: c-animation, c-physics, via:ART::Rockstar::phInstNM, via:phInstGta, via:rage::fragInst.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhInst {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: int?, lanes: c-animation,c-physics).
    pub field_4: u32,
    /// Unknown bytes (0x8..0x48).
    pub _pad_0008: [u8; 0x40],
    /// float_48 (confidence: medium, kind: float, lanes: c-animation,c-physics).
    pub float_48: f32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// ptr_50 (confidence: high, kind: pointer, lanes: c-animation,c-physics,via:ART::Rockstar::phInstNM,via:phInstGta,via:rage::fragInst moved from siblings:ART::Rockstar::phInstNM,phInstGta,rage::phInstBreakable).
    pub ptr_50: Ptr32<u8>,
}
assert_size!(RagePhInst, 0x54); // merged size 0x54 rounded to 4
assert_offset!(RagePhInst, field_4, 0x4);
assert_offset!(RagePhInst, float_48, 0x48);
assert_offset!(RagePhInst, ptr_50, 0x50);

/// Merged layout for `rage::phJoint`.
///
/// Size: 0x31c (low). Bases: none.
/// Lanes: c-physics, via:rage::phJoint1Dof, via:rage::phJoint3Dof, via:rage::phPrismaticJoint.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhJoint {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// float_04 (confidence: low, kind: float, lanes: c-physics).
    pub float_04: f32,
    /// float_08 (confidence: low, kind: float, lanes: c-physics).
    pub float_08: f32,
    /// Unknown bytes (0xc..0x10).
    pub _pad_000c: [u8; 0x4],
    /// field_10 (confidence: medium, kind: int?, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub field_10: u32,
    /// field_14 (confidence: low, kind: int?, lanes: c-physics).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: int?, lanes: c-physics).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: int?, lanes: c-physics).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: int?, lanes: c-physics).
    pub field_20: u32,
    /// Unknown bytes (0x24..0xb0).
    pub _pad_0024: [u8; 0x8c],
    /// field_b0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_b0: u32,
    /// Unknown bytes (0xb4..0x140).
    pub _pad_00b4: [u8; 0x8c],
    /// field_140 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_140: u32,
    /// field_144 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_144: u32,
    /// field_148 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_148: u32,
    /// field_14c (confidence: low, kind: int?, lanes: c-physics).
    pub field_14c: u32,
    /// field_150 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_150: u32,
    /// field_154 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_154: u32,
    /// field_158 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_158: u32,
    /// field_15c (confidence: low, kind: int?, lanes: c-physics).
    pub field_15c: u32,
    /// field_160 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_160: u32,
    /// field_164 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_164: u32,
    /// field_168 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_168: u32,
    /// field_16c (confidence: low, kind: int?, lanes: c-physics).
    pub field_16c: u32,
    /// field_170 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_170: u32,
    /// field_174 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_174: u32,
    /// field_178 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_178: u32,
    /// field_17c (confidence: low, kind: int?, lanes: c-physics).
    pub field_17c: u32,
    /// field_180 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_180: u32,
    /// field_184 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_184: u32,
    /// field_188 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_188: u32,
    /// field_18c (confidence: low, kind: int?, lanes: c-physics).
    pub field_18c: u32,
    /// field_190 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_190: u32,
    /// field_194 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_194: u32,
    /// field_198 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_198: u32,
    /// field_19c (confidence: low, kind: int?, lanes: c-physics).
    pub field_19c: u32,
    /// field_1a0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1a0: u32,
    /// field_1a4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1a4: u32,
    /// field_1a8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1a8: u32,
    /// field_1ac (confidence: low, kind: int?, lanes: c-physics).
    pub field_1ac: u32,
    /// field_1b0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1b0: u32,
    /// field_1b4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1b4: u32,
    /// field_1b8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1b8: u32,
    /// field_1bc (confidence: low, kind: int?, lanes: c-physics).
    pub field_1bc: u32,
    /// Unknown bytes (0x1c0..0x1c8).
    pub _pad_01c0: [u8; 0x8],
    /// float_1c8 (confidence: medium, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_1c8: f32,
    /// Unknown bytes (0x1cc..0x1d8).
    pub _pad_01cc: [u8; 0xc],
    /// float_1d8 (confidence: medium, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_1d8: f32,
    /// Unknown bytes (0x1dc..0x1e8).
    pub _pad_01dc: [u8; 0xc],
    /// float_1e8 (confidence: medium, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_1e8: f32,
    /// Unknown bytes (0x1ec..0x200).
    pub _pad_01ec: [u8; 0x14],
    /// float_200 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_200: f32,
    /// float_204 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_204: f32,
    /// float_208 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_208: f32,
    /// Unknown bytes (0x20c..0x210).
    pub _pad_020c: [u8; 0x4],
    /// float_210 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_210: f32,
    /// float_214 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_214: f32,
    /// float_218 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_218: f32,
    /// field_21c (confidence: medium, kind: int?, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof).
    pub field_21c: u32,
    /// field_220 (confidence: low, kind: int?, lanes: c-physics).
    pub field_220: u32,
    /// bool_224 (confidence: low, kind: bool, lanes: c-physics).
    pub bool_224: u8,
    /// Unknown bytes (0x225..0x238).
    pub _pad_0225: [u8; 0x13],
    /// float_238 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_238: f32,
    /// Unknown bytes (0x23c..0x248).
    pub _pad_023c: [u8; 0xc],
    /// float_248 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_248: f32,
    /// Unknown bytes (0x24c..0x258).
    pub _pad_024c: [u8; 0xc],
    /// float_258 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_258: f32,
    /// Unknown bytes (0x25c..0x270).
    pub _pad_025c: [u8; 0x14],
    /// float_270 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_270: f32,
    /// float_274 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_274: f32,
    /// float_278 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_278: f32,
    /// float_27c (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_27c: f32,
    /// float_280 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_280: f32,
    /// float_284 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_284: f32,
    /// field_288 (confidence: high, kind: int?, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub field_288: u32,
    /// Unknown bytes (0x28c..0x2a0).
    pub _pad_028c: [u8; 0x14],
    /// float_2a0 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2a0: f32,
    /// float_2a4 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2a4: f32,
    /// float_2a8 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2a8: f32,
    /// field_2ac (confidence: high, kind: int?, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub field_2ac: u32,
    /// float_2b0 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2b0: f32,
    /// float_2b4 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2b4: f32,
    /// float_2b8 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2b8: f32,
    /// float_2bc (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phPrismaticJoint).
    pub float_2bc: f32,
    /// float_2c0 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2c0: f32,
    /// float_2c4 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2c4: f32,
    /// Unknown bytes (0x2c8..0x2d0).
    pub _pad_02c8: [u8; 0x8],
    /// float_2d0 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2d0: f32,
    /// float_2d4 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2d4: f32,
    /// float_2d8 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2d8: f32,
    /// Unknown bytes (0x2dc..0x2e0).
    pub _pad_02dc: [u8; 0x4],
    /// float_2e0 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2e0: f32,
    /// float_2e4 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2e4: f32,
    /// float_2e8 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_2e8: f32,
    /// Unknown bytes (0x2ec..0x310).
    pub _pad_02ec: [u8; 0x24],
    /// float_310 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_310: f32,
    /// float_314 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_314: f32,
    /// float_318 (confidence: high, kind: float, lanes: c-physics,via:rage::phJoint1Dof,via:rage::phJoint3Dof,via:rage::phPrismaticJoint moved from siblings:rage::phJoint1Dof,rage::phJoint3Dof,rage::phPrismaticJoint).
    pub float_318: f32,
}
assert_size!(RagePhJoint, 0x31c); // merged size 0x31c rounded to 4
assert_offset!(RagePhJoint, vfptr, 0x0);
assert_offset!(RagePhJoint, float_04, 0x4);
assert_offset!(RagePhJoint, float_08, 0x8);
assert_offset!(RagePhJoint, field_10, 0x10);
assert_offset!(RagePhJoint, field_14, 0x14);
assert_offset!(RagePhJoint, field_18, 0x18);
assert_offset!(RagePhJoint, field_1c, 0x1c);
assert_offset!(RagePhJoint, field_20, 0x20);
assert_offset!(RagePhJoint, field_b0, 0xb0);
assert_offset!(RagePhJoint, field_140, 0x140);
assert_offset!(RagePhJoint, field_144, 0x144);
assert_offset!(RagePhJoint, field_148, 0x148);
assert_offset!(RagePhJoint, field_14c, 0x14c);
assert_offset!(RagePhJoint, field_150, 0x150);
assert_offset!(RagePhJoint, field_154, 0x154);
assert_offset!(RagePhJoint, field_158, 0x158);
assert_offset!(RagePhJoint, field_15c, 0x15c);
assert_offset!(RagePhJoint, field_160, 0x160);
assert_offset!(RagePhJoint, field_164, 0x164);
assert_offset!(RagePhJoint, field_168, 0x168);
assert_offset!(RagePhJoint, field_16c, 0x16c);
assert_offset!(RagePhJoint, field_170, 0x170);
assert_offset!(RagePhJoint, field_174, 0x174);
assert_offset!(RagePhJoint, field_178, 0x178);
assert_offset!(RagePhJoint, field_17c, 0x17c);
assert_offset!(RagePhJoint, field_180, 0x180);
assert_offset!(RagePhJoint, field_184, 0x184);
assert_offset!(RagePhJoint, field_188, 0x188);
assert_offset!(RagePhJoint, field_18c, 0x18c);
assert_offset!(RagePhJoint, field_190, 0x190);
assert_offset!(RagePhJoint, field_194, 0x194);
assert_offset!(RagePhJoint, field_198, 0x198);
assert_offset!(RagePhJoint, field_19c, 0x19c);
assert_offset!(RagePhJoint, field_1a0, 0x1a0);
assert_offset!(RagePhJoint, field_1a4, 0x1a4);
assert_offset!(RagePhJoint, field_1a8, 0x1a8);
assert_offset!(RagePhJoint, field_1ac, 0x1ac);
assert_offset!(RagePhJoint, field_1b0, 0x1b0);
assert_offset!(RagePhJoint, field_1b4, 0x1b4);
assert_offset!(RagePhJoint, field_1b8, 0x1b8);
assert_offset!(RagePhJoint, field_1bc, 0x1bc);
assert_offset!(RagePhJoint, float_1c8, 0x1c8);
assert_offset!(RagePhJoint, float_1d8, 0x1d8);
assert_offset!(RagePhJoint, float_1e8, 0x1e8);
assert_offset!(RagePhJoint, float_200, 0x200);
assert_offset!(RagePhJoint, float_204, 0x204);
assert_offset!(RagePhJoint, float_208, 0x208);
assert_offset!(RagePhJoint, float_210, 0x210);
assert_offset!(RagePhJoint, float_214, 0x214);
assert_offset!(RagePhJoint, float_218, 0x218);
assert_offset!(RagePhJoint, field_21c, 0x21c);
assert_offset!(RagePhJoint, field_220, 0x220);
assert_offset!(RagePhJoint, bool_224, 0x224);
assert_offset!(RagePhJoint, float_238, 0x238);
assert_offset!(RagePhJoint, float_248, 0x248);
assert_offset!(RagePhJoint, float_258, 0x258);
assert_offset!(RagePhJoint, float_270, 0x270);
assert_offset!(RagePhJoint, float_274, 0x274);
assert_offset!(RagePhJoint, float_278, 0x278);
assert_offset!(RagePhJoint, float_27c, 0x27c);
assert_offset!(RagePhJoint, float_280, 0x280);
assert_offset!(RagePhJoint, float_284, 0x284);
assert_offset!(RagePhJoint, field_288, 0x288);
assert_offset!(RagePhJoint, float_2a0, 0x2a0);
assert_offset!(RagePhJoint, float_2a4, 0x2a4);
assert_offset!(RagePhJoint, float_2a8, 0x2a8);
assert_offset!(RagePhJoint, field_2ac, 0x2ac);
assert_offset!(RagePhJoint, float_2b0, 0x2b0);
assert_offset!(RagePhJoint, float_2b4, 0x2b4);
assert_offset!(RagePhJoint, float_2b8, 0x2b8);
assert_offset!(RagePhJoint, float_2bc, 0x2bc);
assert_offset!(RagePhJoint, float_2c0, 0x2c0);
assert_offset!(RagePhJoint, float_2c4, 0x2c4);
assert_offset!(RagePhJoint, float_2d0, 0x2d0);
assert_offset!(RagePhJoint, float_2d4, 0x2d4);
assert_offset!(RagePhJoint, float_2d8, 0x2d8);
assert_offset!(RagePhJoint, float_2e0, 0x2e0);
assert_offset!(RagePhJoint, float_2e4, 0x2e4);
assert_offset!(RagePhJoint, float_2e8, 0x2e8);
assert_offset!(RagePhJoint, float_310, 0x310);
assert_offset!(RagePhJoint, float_314, 0x314);
assert_offset!(RagePhJoint, float_318, 0x318);

/// Merged layout for `rage::phJoint1Dof`.
///
/// Size: 0x31c (low). Bases: rage::phJoint@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhJoint1Dof {
    /// Unknown bytes (0x0..0x230).
    pub _pad_0000: [u8; 0x230],
    /// field_230 (confidence: low, kind: int?, lanes: c-physics).
    pub field_230: u32,
    /// Unknown bytes (0x234..0x290).
    pub _pad_0234: [u8; 0x5c],
    /// float_290 (confidence: high, kind: float, lanes: c-physics).
    pub float_290: f32,
    /// float_294 (confidence: high, kind: float, lanes: c-physics).
    pub float_294: f32,
    /// float_298 (confidence: high, kind: float, lanes: c-physics).
    pub float_298: f32,
    /// field_29c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_29c: u32,
    /// Unknown bytes (0x2a0..0x2f0).
    pub _pad_02a0: [u8; 0x50],
    /// float_2f0 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f0: f32,
    /// float_2f4 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f4: f32,
    /// float_2f8 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f8: f32,
    /// Unknown bytes (0x2fc..0x300).
    pub _pad_02fc: [u8; 0x4],
    /// float_300 (confidence: high, kind: float, lanes: c-physics).
    pub float_300: f32,
    /// float_304 (confidence: high, kind: float, lanes: c-physics).
    pub float_304: f32,
    /// float_308 (confidence: high, kind: float, lanes: c-physics).
    pub float_308: f32,
    /// Unknown trailing bytes (0x30c..0x31c).
    pub _pad_end: [u8; 0x10],
}
assert_size!(RagePhJoint1Dof, 0x31c); // merged size 0x31c rounded to 4
assert_offset!(RagePhJoint1Dof, field_230, 0x230);
assert_offset!(RagePhJoint1Dof, float_290, 0x290);
assert_offset!(RagePhJoint1Dof, float_294, 0x294);
assert_offset!(RagePhJoint1Dof, float_298, 0x298);
assert_offset!(RagePhJoint1Dof, field_29c, 0x29c);
assert_offset!(RagePhJoint1Dof, float_2f0, 0x2f0);
assert_offset!(RagePhJoint1Dof, float_2f4, 0x2f4);
assert_offset!(RagePhJoint1Dof, float_2f8, 0x2f8);
assert_offset!(RagePhJoint1Dof, float_300, 0x300);
assert_offset!(RagePhJoint1Dof, float_304, 0x304);
assert_offset!(RagePhJoint1Dof, float_308, 0x308);

/// Merged layout for `rage::phJoint3Dof`.
///
/// Size: 0x57c (low). Bases: rage::phJoint@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhJoint3Dof {
    /// Unknown bytes (0x0..0x230).
    pub _pad_0000: [u8; 0x230],
    /// field_230 (confidence: low, kind: int?, lanes: c-physics).
    pub field_230: u32,
    /// Unknown bytes (0x234..0x28c).
    pub _pad_0234: [u8; 0x58],
    /// field_28c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_28c: u32,
    /// field_290 (confidence: low, kind: int?, lanes: c-physics).
    pub field_290: u32,
    /// float_294 (confidence: medium, kind: float, lanes: c-physics).
    pub float_294: f32,
    /// field_298 (confidence: low, kind: int?, lanes: c-physics).
    pub field_298: u32,
    /// float_29c (confidence: high, kind: float, lanes: c-physics).
    pub float_29c: f32,
    /// Unknown bytes (0x2a0..0x2c8).
    pub _pad_02a0: [u8; 0x28],
    /// float_2c8 (confidence: medium, kind: float, lanes: c-physics).
    pub float_2c8: f32,
    /// Unknown bytes (0x2cc..0x2f0).
    pub _pad_02cc: [u8; 0x24],
    /// field_2f0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f4: u32,
    /// field_2f8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_2f8: u32,
    /// Unknown bytes (0x2fc..0x300).
    pub _pad_02fc: [u8; 0x4],
    /// field_300 (confidence: low, kind: int?, lanes: c-physics).
    pub field_300: u32,
    /// field_304 (confidence: low, kind: int?, lanes: c-physics).
    pub field_304: u32,
    /// field_308 (confidence: low, kind: int?, lanes: c-physics).
    pub field_308: u32,
    /// Unknown bytes (0x30c..0x31c).
    pub _pad_030c: [u8; 0x10],
    /// field_31c (confidence: low, kind: int?, lanes: c-physics).
    pub field_31c: u32,
    /// float_320 (confidence: high, kind: float, lanes: c-physics).
    pub float_320: f32,
    /// float_324 (confidence: high, kind: float, lanes: c-physics).
    pub float_324: f32,
    /// float_328 (confidence: high, kind: float, lanes: c-physics).
    pub float_328: f32,
    /// field_32c (confidence: low, kind: int?, lanes: c-physics).
    pub field_32c: u32,
    /// float_330 (confidence: high, kind: float, lanes: c-physics).
    pub float_330: f32,
    /// float_334 (confidence: high, kind: float, lanes: c-physics).
    pub float_334: f32,
    /// float_338 (confidence: high, kind: float, lanes: c-physics).
    pub float_338: f32,
    /// field_33c (confidence: low, kind: int?, lanes: c-physics).
    pub field_33c: u32,
    /// float_340 (confidence: high, kind: float, lanes: c-physics).
    pub float_340: f32,
    /// Unknown bytes (0x344..0x350).
    pub _pad_0344: [u8; 0xc],
    /// float_350 (confidence: low, kind: float, lanes: c-physics).
    pub float_350: f32,
    /// float_354 (confidence: low, kind: float, lanes: c-physics).
    pub float_354: f32,
    /// Unknown bytes (0x358..0x360).
    pub _pad_0358: [u8; 0x8],
    /// float_360 (confidence: low, kind: float, lanes: c-physics).
    pub float_360: f32,
    /// float_364 (confidence: low, kind: float, lanes: c-physics).
    pub float_364: f32,
    /// Unknown bytes (0x368..0x370).
    pub _pad_0368: [u8; 0x8],
    /// float_370 (confidence: low, kind: float, lanes: c-physics).
    pub float_370: f32,
    /// float_374 (confidence: low, kind: float, lanes: c-physics).
    pub float_374: f32,
    /// Unknown bytes (0x378..0x390).
    pub _pad_0378: [u8; 0x18],
    /// float_390 (confidence: high, kind: float, lanes: c-physics).
    pub float_390: f32,
    /// float_394 (confidence: high, kind: float, lanes: c-physics).
    pub float_394: f32,
    /// float_398 (confidence: high, kind: float, lanes: c-physics).
    pub float_398: f32,
    /// Unknown bytes (0x39c..0x3a0).
    pub _pad_039c: [u8; 0x4],
    /// float_3a0 (confidence: medium, kind: float, lanes: c-physics).
    pub float_3a0: f32,
    /// float_3a4 (confidence: low, kind: float, lanes: c-physics).
    pub float_3a4: f32,
    /// float_3a8 (confidence: low, kind: float, lanes: c-physics).
    pub float_3a8: f32,
    /// Unknown bytes (0x3ac..0x3b0).
    pub _pad_03ac: [u8; 0x4],
    /// float_3b0 (confidence: low, kind: float, lanes: c-physics).
    pub float_3b0: f32,
    /// float_3b4 (confidence: low, kind: float, lanes: c-physics).
    pub float_3b4: f32,
    /// float_3b8 (confidence: low, kind: float, lanes: c-physics).
    pub float_3b8: f32,
    /// Unknown bytes (0x3bc..0x3c0).
    pub _pad_03bc: [u8; 0x4],
    /// float_3c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_3c0: f32,
    /// float_3c4 (confidence: low, kind: float, lanes: c-physics).
    pub float_3c4: f32,
    /// float_3c8 (confidence: low, kind: float, lanes: c-physics).
    pub float_3c8: f32,
    /// Unknown bytes (0x3cc..0x3d0).
    pub _pad_03cc: [u8; 0x4],
    /// field_3d0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_3d0: u32,
    /// field_3d4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_3d4: u32,
    /// field_3d8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_3d8: u32,
    /// Unknown bytes (0x3dc..0x3e0).
    pub _pad_03dc: [u8; 0x4],
    /// float_3e0 (confidence: low, kind: float, lanes: c-physics).
    pub float_3e0: f32,
    /// float_3e4 (confidence: low, kind: float, lanes: c-physics).
    pub float_3e4: f32,
    /// float_3e8 (confidence: low, kind: float, lanes: c-physics).
    pub float_3e8: f32,
    /// Unknown bytes (0x3ec..0x3f0).
    pub _pad_03ec: [u8; 0x4],
    /// float_3f0 (confidence: low, kind: float, lanes: c-physics).
    pub float_3f0: f32,
    /// float_3f4 (confidence: low, kind: float, lanes: c-physics).
    pub float_3f4: f32,
    /// float_3f8 (confidence: low, kind: float, lanes: c-physics).
    pub float_3f8: f32,
    /// Unknown bytes (0x3fc..0x430).
    pub _pad_03fc: [u8; 0x34],
    /// float_430 (confidence: medium, kind: float, lanes: c-physics).
    pub float_430: f32,
    /// float_434 (confidence: low, kind: float, lanes: c-physics).
    pub float_434: f32,
    /// float_438 (confidence: low, kind: float, lanes: c-physics).
    pub float_438: f32,
    /// Unknown bytes (0x43c..0x440).
    pub _pad_043c: [u8; 0x4],
    /// float_440 (confidence: low, kind: float, lanes: c-physics).
    pub float_440: f32,
    /// float_444 (confidence: low, kind: float, lanes: c-physics).
    pub float_444: f32,
    /// float_448 (confidence: low, kind: float, lanes: c-physics).
    pub float_448: f32,
    /// Unknown bytes (0x44c..0x450).
    pub _pad_044c: [u8; 0x4],
    /// float_450 (confidence: low, kind: float, lanes: c-physics).
    pub float_450: f32,
    /// float_454 (confidence: low, kind: float, lanes: c-physics).
    pub float_454: f32,
    /// float_458 (confidence: low, kind: float, lanes: c-physics).
    pub float_458: f32,
    /// Unknown bytes (0x45c..0x460).
    pub _pad_045c: [u8; 0x4],
    /// float_460 (confidence: low, kind: float, lanes: c-physics).
    pub float_460: f32,
    /// float_464 (confidence: low, kind: float, lanes: c-physics).
    pub float_464: f32,
    /// float_468 (confidence: low, kind: float, lanes: c-physics).
    pub float_468: f32,
    /// Unknown bytes (0x46c..0x470).
    pub _pad_046c: [u8; 0x4],
    /// float_470 (confidence: low, kind: float, lanes: c-physics).
    pub float_470: f32,
    /// float_474 (confidence: low, kind: float, lanes: c-physics).
    pub float_474: f32,
    /// float_478 (confidence: low, kind: float, lanes: c-physics).
    pub float_478: f32,
    /// Unknown bytes (0x47c..0x480).
    pub _pad_047c: [u8; 0x4],
    /// float_480 (confidence: low, kind: float, lanes: c-physics).
    pub float_480: f32,
    /// float_484 (confidence: low, kind: float, lanes: c-physics).
    pub float_484: f32,
    /// float_488 (confidence: low, kind: float, lanes: c-physics).
    pub float_488: f32,
    /// Unknown bytes (0x48c..0x490).
    pub _pad_048c: [u8; 0x4],
    /// float_490 (confidence: low, kind: float, lanes: c-physics).
    pub float_490: f32,
    /// float_494 (confidence: low, kind: float, lanes: c-physics).
    pub float_494: f32,
    /// float_498 (confidence: low, kind: float, lanes: c-physics).
    pub float_498: f32,
    /// Unknown bytes (0x49c..0x4c0).
    pub _pad_049c: [u8; 0x24],
    /// float_4c0 (confidence: high, kind: float, lanes: c-physics).
    pub float_4c0: f32,
    /// float_4c4 (confidence: high, kind: float, lanes: c-physics).
    pub float_4c4: f32,
    /// float_4c8 (confidence: high, kind: float, lanes: c-physics).
    pub float_4c8: f32,
    /// Unknown bytes (0x4cc..0x4d0).
    pub _pad_04cc: [u8; 0x4],
    /// float_4d0 (confidence: high, kind: float, lanes: c-physics).
    pub float_4d0: f32,
    /// float_4d4 (confidence: high, kind: float, lanes: c-physics).
    pub float_4d4: f32,
    /// float_4d8 (confidence: high, kind: float, lanes: c-physics).
    pub float_4d8: f32,
    /// Unknown bytes (0x4dc..0x4e0).
    pub _pad_04dc: [u8; 0x4],
    /// float_4e0 (confidence: high, kind: float, lanes: c-physics).
    pub float_4e0: f32,
    /// float_4e4 (confidence: high, kind: float, lanes: c-physics).
    pub float_4e4: f32,
    /// float_4e8 (confidence: high, kind: float, lanes: c-physics).
    pub float_4e8: f32,
    /// Unknown bytes (0x4ec..0x4f0).
    pub _pad_04ec: [u8; 0x4],
    /// float_4f0 (confidence: high, kind: float, lanes: c-physics).
    pub float_4f0: f32,
    /// float_4f4 (confidence: high, kind: float, lanes: c-physics).
    pub float_4f4: f32,
    /// float_4f8 (confidence: high, kind: float, lanes: c-physics).
    pub float_4f8: f32,
    /// Unknown bytes (0x4fc..0x500).
    pub _pad_04fc: [u8; 0x4],
    /// float_500 (confidence: high, kind: float, lanes: c-physics).
    pub float_500: f32,
    /// float_504 (confidence: high, kind: float, lanes: c-physics).
    pub float_504: f32,
    /// float_508 (confidence: high, kind: float, lanes: c-physics).
    pub float_508: f32,
    /// Unknown bytes (0x50c..0x510).
    pub _pad_050c: [u8; 0x4],
    /// float_510 (confidence: high, kind: float, lanes: c-physics).
    pub float_510: f32,
    /// float_514 (confidence: high, kind: float, lanes: c-physics).
    pub float_514: f32,
    /// float_518 (confidence: high, kind: float, lanes: c-physics).
    pub float_518: f32,
    /// Unknown bytes (0x51c..0x520).
    pub _pad_051c: [u8; 0x4],
    /// float_520 (confidence: high, kind: float, lanes: c-physics).
    pub float_520: f32,
    /// float_524 (confidence: high, kind: float, lanes: c-physics).
    pub float_524: f32,
    /// float_528 (confidence: high, kind: float, lanes: c-physics).
    pub float_528: f32,
    /// Unknown bytes (0x52c..0x530).
    pub _pad_052c: [u8; 0x4],
    /// float_530 (confidence: high, kind: float, lanes: c-physics).
    pub float_530: f32,
    /// float_534 (confidence: high, kind: float, lanes: c-physics).
    pub float_534: f32,
    /// float_538 (confidence: high, kind: float, lanes: c-physics).
    pub float_538: f32,
    /// Unknown bytes (0x53c..0x540).
    pub _pad_053c: [u8; 0x4],
    /// float_540 (confidence: high, kind: float, lanes: c-physics).
    pub float_540: f32,
    /// float_544 (confidence: high, kind: float, lanes: c-physics).
    pub float_544: f32,
    /// float_548 (confidence: high, kind: float, lanes: c-physics).
    pub float_548: f32,
    /// Unknown bytes (0x54c..0x550).
    pub _pad_054c: [u8; 0x4],
    /// float_550 (confidence: high, kind: float, lanes: c-physics).
    pub float_550: f32,
    /// float_554 (confidence: medium, kind: float, lanes: c-physics).
    pub float_554: f32,
    /// float_558 (confidence: high, kind: float, lanes: c-physics).
    pub float_558: f32,
    /// Unknown bytes (0x55c..0x560).
    pub _pad_055c: [u8; 0x4],
    /// float_560 (confidence: medium, kind: float, lanes: c-physics).
    pub float_560: f32,
    /// float_564 (confidence: high, kind: float, lanes: c-physics).
    pub float_564: f32,
    /// float_568 (confidence: medium, kind: float, lanes: c-physics).
    pub float_568: f32,
    /// Unknown bytes (0x56c..0x570).
    pub _pad_056c: [u8; 0x4],
    /// float_570 (confidence: medium, kind: float, lanes: c-physics).
    pub float_570: f32,
    /// float_574 (confidence: medium, kind: float, lanes: c-physics).
    pub float_574: f32,
    /// float_578 (confidence: high, kind: float, lanes: c-physics).
    pub float_578: f32,
}
assert_size!(RagePhJoint3Dof, 0x57c); // merged size 0x57c rounded to 4
assert_offset!(RagePhJoint3Dof, field_230, 0x230);
assert_offset!(RagePhJoint3Dof, field_28c, 0x28c);
assert_offset!(RagePhJoint3Dof, field_290, 0x290);
assert_offset!(RagePhJoint3Dof, float_294, 0x294);
assert_offset!(RagePhJoint3Dof, field_298, 0x298);
assert_offset!(RagePhJoint3Dof, float_29c, 0x29c);
assert_offset!(RagePhJoint3Dof, float_2c8, 0x2c8);
assert_offset!(RagePhJoint3Dof, field_2f0, 0x2f0);
assert_offset!(RagePhJoint3Dof, field_2f4, 0x2f4);
assert_offset!(RagePhJoint3Dof, field_2f8, 0x2f8);
assert_offset!(RagePhJoint3Dof, field_300, 0x300);
assert_offset!(RagePhJoint3Dof, field_304, 0x304);
assert_offset!(RagePhJoint3Dof, field_308, 0x308);
assert_offset!(RagePhJoint3Dof, field_31c, 0x31c);
assert_offset!(RagePhJoint3Dof, float_320, 0x320);
assert_offset!(RagePhJoint3Dof, float_324, 0x324);
assert_offset!(RagePhJoint3Dof, float_328, 0x328);
assert_offset!(RagePhJoint3Dof, field_32c, 0x32c);
assert_offset!(RagePhJoint3Dof, float_330, 0x330);
assert_offset!(RagePhJoint3Dof, float_334, 0x334);
assert_offset!(RagePhJoint3Dof, float_338, 0x338);
assert_offset!(RagePhJoint3Dof, field_33c, 0x33c);
assert_offset!(RagePhJoint3Dof, float_340, 0x340);
assert_offset!(RagePhJoint3Dof, float_350, 0x350);
assert_offset!(RagePhJoint3Dof, float_354, 0x354);
assert_offset!(RagePhJoint3Dof, float_360, 0x360);
assert_offset!(RagePhJoint3Dof, float_364, 0x364);
assert_offset!(RagePhJoint3Dof, float_370, 0x370);
assert_offset!(RagePhJoint3Dof, float_374, 0x374);
assert_offset!(RagePhJoint3Dof, float_390, 0x390);
assert_offset!(RagePhJoint3Dof, float_394, 0x394);
assert_offset!(RagePhJoint3Dof, float_398, 0x398);
assert_offset!(RagePhJoint3Dof, float_3a0, 0x3a0);
assert_offset!(RagePhJoint3Dof, float_3a4, 0x3a4);
assert_offset!(RagePhJoint3Dof, float_3a8, 0x3a8);
assert_offset!(RagePhJoint3Dof, float_3b0, 0x3b0);
assert_offset!(RagePhJoint3Dof, float_3b4, 0x3b4);
assert_offset!(RagePhJoint3Dof, float_3b8, 0x3b8);
assert_offset!(RagePhJoint3Dof, float_3c0, 0x3c0);
assert_offset!(RagePhJoint3Dof, float_3c4, 0x3c4);
assert_offset!(RagePhJoint3Dof, float_3c8, 0x3c8);
assert_offset!(RagePhJoint3Dof, field_3d0, 0x3d0);
assert_offset!(RagePhJoint3Dof, field_3d4, 0x3d4);
assert_offset!(RagePhJoint3Dof, field_3d8, 0x3d8);
assert_offset!(RagePhJoint3Dof, float_3e0, 0x3e0);
assert_offset!(RagePhJoint3Dof, float_3e4, 0x3e4);
assert_offset!(RagePhJoint3Dof, float_3e8, 0x3e8);
assert_offset!(RagePhJoint3Dof, float_3f0, 0x3f0);
assert_offset!(RagePhJoint3Dof, float_3f4, 0x3f4);
assert_offset!(RagePhJoint3Dof, float_3f8, 0x3f8);
assert_offset!(RagePhJoint3Dof, float_430, 0x430);
assert_offset!(RagePhJoint3Dof, float_434, 0x434);
assert_offset!(RagePhJoint3Dof, float_438, 0x438);
assert_offset!(RagePhJoint3Dof, float_440, 0x440);
assert_offset!(RagePhJoint3Dof, float_444, 0x444);
assert_offset!(RagePhJoint3Dof, float_448, 0x448);
assert_offset!(RagePhJoint3Dof, float_450, 0x450);
assert_offset!(RagePhJoint3Dof, float_454, 0x454);
assert_offset!(RagePhJoint3Dof, float_458, 0x458);
assert_offset!(RagePhJoint3Dof, float_460, 0x460);
assert_offset!(RagePhJoint3Dof, float_464, 0x464);
assert_offset!(RagePhJoint3Dof, float_468, 0x468);
assert_offset!(RagePhJoint3Dof, float_470, 0x470);
assert_offset!(RagePhJoint3Dof, float_474, 0x474);
assert_offset!(RagePhJoint3Dof, float_478, 0x478);
assert_offset!(RagePhJoint3Dof, float_480, 0x480);
assert_offset!(RagePhJoint3Dof, float_484, 0x484);
assert_offset!(RagePhJoint3Dof, float_488, 0x488);
assert_offset!(RagePhJoint3Dof, float_490, 0x490);
assert_offset!(RagePhJoint3Dof, float_494, 0x494);
assert_offset!(RagePhJoint3Dof, float_498, 0x498);
assert_offset!(RagePhJoint3Dof, float_4c0, 0x4c0);
assert_offset!(RagePhJoint3Dof, float_4c4, 0x4c4);
assert_offset!(RagePhJoint3Dof, float_4c8, 0x4c8);
assert_offset!(RagePhJoint3Dof, float_4d0, 0x4d0);
assert_offset!(RagePhJoint3Dof, float_4d4, 0x4d4);
assert_offset!(RagePhJoint3Dof, float_4d8, 0x4d8);
assert_offset!(RagePhJoint3Dof, float_4e0, 0x4e0);
assert_offset!(RagePhJoint3Dof, float_4e4, 0x4e4);
assert_offset!(RagePhJoint3Dof, float_4e8, 0x4e8);
assert_offset!(RagePhJoint3Dof, float_4f0, 0x4f0);
assert_offset!(RagePhJoint3Dof, float_4f4, 0x4f4);
assert_offset!(RagePhJoint3Dof, float_4f8, 0x4f8);
assert_offset!(RagePhJoint3Dof, float_500, 0x500);
assert_offset!(RagePhJoint3Dof, float_504, 0x504);
assert_offset!(RagePhJoint3Dof, float_508, 0x508);
assert_offset!(RagePhJoint3Dof, float_510, 0x510);
assert_offset!(RagePhJoint3Dof, float_514, 0x514);
assert_offset!(RagePhJoint3Dof, float_518, 0x518);
assert_offset!(RagePhJoint3Dof, float_520, 0x520);
assert_offset!(RagePhJoint3Dof, float_524, 0x524);
assert_offset!(RagePhJoint3Dof, float_528, 0x528);
assert_offset!(RagePhJoint3Dof, float_530, 0x530);
assert_offset!(RagePhJoint3Dof, float_534, 0x534);
assert_offset!(RagePhJoint3Dof, float_538, 0x538);
assert_offset!(RagePhJoint3Dof, float_540, 0x540);
assert_offset!(RagePhJoint3Dof, float_544, 0x544);
assert_offset!(RagePhJoint3Dof, float_548, 0x548);
assert_offset!(RagePhJoint3Dof, float_550, 0x550);
assert_offset!(RagePhJoint3Dof, float_554, 0x554);
assert_offset!(RagePhJoint3Dof, float_558, 0x558);
assert_offset!(RagePhJoint3Dof, float_560, 0x560);
assert_offset!(RagePhJoint3Dof, float_564, 0x564);
assert_offset!(RagePhJoint3Dof, float_568, 0x568);
assert_offset!(RagePhJoint3Dof, float_570, 0x570);
assert_offset!(RagePhJoint3Dof, float_574, 0x574);
assert_offset!(RagePhJoint3Dof, float_578, 0x578);

/// Merged layout for `rage::phPrismaticJoint`.
///
/// Size: 0x31c (low). Bases: rage::phJoint@0x0.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhPrismaticJoint {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// float_24 (confidence: low, kind: float, lanes: c-physics).
    pub float_24: f32,
    /// float_28 (confidence: low, kind: float, lanes: c-physics).
    pub float_28: f32,
    /// Unknown bytes (0x2c..0x30).
    pub _pad_002c: [u8; 0x4],
    /// float_30 (confidence: low, kind: float, lanes: c-physics).
    pub float_30: f32,
    /// float_34 (confidence: low, kind: float, lanes: c-physics).
    pub float_34: f32,
    /// float_38 (confidence: low, kind: float, lanes: c-physics).
    pub float_38: f32,
    /// Unknown bytes (0x3c..0x40).
    pub _pad_003c: [u8; 0x4],
    /// float_40 (confidence: low, kind: float, lanes: c-physics).
    pub float_40: f32,
    /// float_44 (confidence: low, kind: float, lanes: c-physics).
    pub float_44: f32,
    /// float_48 (confidence: low, kind: float, lanes: c-physics).
    pub float_48: f32,
    /// Unknown bytes (0x4c..0x50).
    pub _pad_004c: [u8; 0x4],
    /// float_50 (confidence: low, kind: float, lanes: c-physics).
    pub float_50: f32,
    /// float_54 (confidence: low, kind: float, lanes: c-physics).
    pub float_54: f32,
    /// float_58 (confidence: low, kind: float, lanes: c-physics).
    pub float_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// float_60 (confidence: low, kind: float, lanes: c-physics).
    pub float_60: f32,
    /// float_64 (confidence: low, kind: float, lanes: c-physics).
    pub float_64: f32,
    /// float_68 (confidence: low, kind: float, lanes: c-physics).
    pub float_68: f32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// float_70 (confidence: low, kind: float, lanes: c-physics).
    pub float_70: f32,
    /// float_74 (confidence: low, kind: float, lanes: c-physics).
    pub float_74: f32,
    /// float_78 (confidence: low, kind: float, lanes: c-physics).
    pub float_78: f32,
    /// Unknown bytes (0x7c..0xb4).
    pub _pad_007c: [u8; 0x38],
    /// float_b4 (confidence: low, kind: float, lanes: c-physics).
    pub float_b4: f32,
    /// float_b8 (confidence: low, kind: float, lanes: c-physics).
    pub float_b8: f32,
    /// Unknown bytes (0xbc..0xc0).
    pub _pad_00bc: [u8; 0x4],
    /// float_c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_c0: f32,
    /// float_c4 (confidence: low, kind: float, lanes: c-physics).
    pub float_c4: f32,
    /// float_c8 (confidence: low, kind: float, lanes: c-physics).
    pub float_c8: f32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// float_d0 (confidence: low, kind: float, lanes: c-physics).
    pub float_d0: f32,
    /// float_d4 (confidence: low, kind: float, lanes: c-physics).
    pub float_d4: f32,
    /// float_d8 (confidence: low, kind: float, lanes: c-physics).
    pub float_d8: f32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// float_e0 (confidence: low, kind: float, lanes: c-physics).
    pub float_e0: f32,
    /// float_e4 (confidence: low, kind: float, lanes: c-physics).
    pub float_e4: f32,
    /// float_e8 (confidence: low, kind: float, lanes: c-physics).
    pub float_e8: f32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// float_f0 (confidence: low, kind: float, lanes: c-physics).
    pub float_f0: f32,
    /// float_f4 (confidence: low, kind: float, lanes: c-physics).
    pub float_f4: f32,
    /// float_f8 (confidence: low, kind: float, lanes: c-physics).
    pub float_f8: f32,
    /// Unknown bytes (0xfc..0x100).
    pub _pad_00fc: [u8; 0x4],
    /// float_100 (confidence: low, kind: float, lanes: c-physics).
    pub float_100: f32,
    /// float_104 (confidence: low, kind: float, lanes: c-physics).
    pub float_104: f32,
    /// float_108 (confidence: low, kind: float, lanes: c-physics).
    pub float_108: f32,
    /// Unknown bytes (0x10c..0x1c0).
    pub _pad_010c: [u8; 0xb4],
    /// float_1c0 (confidence: low, kind: float, lanes: c-physics).
    pub float_1c0: f32,
    /// float_1c4 (confidence: low, kind: float, lanes: c-physics).
    pub float_1c4: f32,
    /// Unknown bytes (0x1c8..0x1d0).
    pub _pad_01c8: [u8; 0x8],
    /// float_1d0 (confidence: low, kind: float, lanes: c-physics).
    pub float_1d0: f32,
    /// float_1d4 (confidence: low, kind: float, lanes: c-physics).
    pub float_1d4: f32,
    /// Unknown bytes (0x1d8..0x1e0).
    pub _pad_01d8: [u8; 0x8],
    /// float_1e0 (confidence: low, kind: float, lanes: c-physics).
    pub float_1e0: f32,
    /// float_1e4 (confidence: low, kind: float, lanes: c-physics).
    pub float_1e4: f32,
    /// Unknown bytes (0x1e8..0x230).
    pub _pad_01e8: [u8; 0x48],
    /// float_230 (confidence: medium, kind: float, lanes: c-physics).
    pub float_230: f32,
    /// float_234 (confidence: low, kind: float, lanes: c-physics).
    pub float_234: f32,
    /// Unknown bytes (0x238..0x240).
    pub _pad_0238: [u8; 0x8],
    /// float_240 (confidence: low, kind: float, lanes: c-physics).
    pub float_240: f32,
    /// float_244 (confidence: low, kind: float, lanes: c-physics).
    pub float_244: f32,
    /// Unknown bytes (0x248..0x250).
    pub _pad_0248: [u8; 0x8],
    /// float_250 (confidence: low, kind: float, lanes: c-physics).
    pub float_250: f32,
    /// float_254 (confidence: low, kind: float, lanes: c-physics).
    pub float_254: f32,
    /// Unknown bytes (0x258..0x290).
    pub _pad_0258: [u8; 0x38],
    /// field_290 (confidence: low, kind: int?, lanes: c-physics).
    pub field_290: u32,
    /// field_294 (confidence: low, kind: int?, lanes: c-physics).
    pub field_294: u32,
    /// field_298 (confidence: low, kind: int?, lanes: c-physics).
    pub field_298: u32,
    /// Unknown bytes (0x29c..0x2f0).
    pub _pad_029c: [u8; 0x54],
    /// float_2f0 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f0: f32,
    /// float_2f4 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f4: f32,
    /// float_2f8 (confidence: high, kind: float, lanes: c-physics).
    pub float_2f8: f32,
    /// Unknown bytes (0x2fc..0x300).
    pub _pad_02fc: [u8; 0x4],
    /// float_300 (confidence: high, kind: float, lanes: c-physics).
    pub float_300: f32,
    /// float_304 (confidence: high, kind: float, lanes: c-physics).
    pub float_304: f32,
    /// float_308 (confidence: high, kind: float, lanes: c-physics).
    pub float_308: f32,
    /// Unknown trailing bytes (0x30c..0x31c).
    pub _pad_end: [u8; 0x10],
}
assert_size!(RagePhPrismaticJoint, 0x31c); // merged size 0x31c rounded to 4
assert_offset!(RagePhPrismaticJoint, float_24, 0x24);
assert_offset!(RagePhPrismaticJoint, float_28, 0x28);
assert_offset!(RagePhPrismaticJoint, float_30, 0x30);
assert_offset!(RagePhPrismaticJoint, float_34, 0x34);
assert_offset!(RagePhPrismaticJoint, float_38, 0x38);
assert_offset!(RagePhPrismaticJoint, float_40, 0x40);
assert_offset!(RagePhPrismaticJoint, float_44, 0x44);
assert_offset!(RagePhPrismaticJoint, float_48, 0x48);
assert_offset!(RagePhPrismaticJoint, float_50, 0x50);
assert_offset!(RagePhPrismaticJoint, float_54, 0x54);
assert_offset!(RagePhPrismaticJoint, float_58, 0x58);
assert_offset!(RagePhPrismaticJoint, float_60, 0x60);
assert_offset!(RagePhPrismaticJoint, float_64, 0x64);
assert_offset!(RagePhPrismaticJoint, float_68, 0x68);
assert_offset!(RagePhPrismaticJoint, float_70, 0x70);
assert_offset!(RagePhPrismaticJoint, float_74, 0x74);
assert_offset!(RagePhPrismaticJoint, float_78, 0x78);
assert_offset!(RagePhPrismaticJoint, float_b4, 0xb4);
assert_offset!(RagePhPrismaticJoint, float_b8, 0xb8);
assert_offset!(RagePhPrismaticJoint, float_c0, 0xc0);
assert_offset!(RagePhPrismaticJoint, float_c4, 0xc4);
assert_offset!(RagePhPrismaticJoint, float_c8, 0xc8);
assert_offset!(RagePhPrismaticJoint, float_d0, 0xd0);
assert_offset!(RagePhPrismaticJoint, float_d4, 0xd4);
assert_offset!(RagePhPrismaticJoint, float_d8, 0xd8);
assert_offset!(RagePhPrismaticJoint, float_e0, 0xe0);
assert_offset!(RagePhPrismaticJoint, float_e4, 0xe4);
assert_offset!(RagePhPrismaticJoint, float_e8, 0xe8);
assert_offset!(RagePhPrismaticJoint, float_f0, 0xf0);
assert_offset!(RagePhPrismaticJoint, float_f4, 0xf4);
assert_offset!(RagePhPrismaticJoint, float_f8, 0xf8);
assert_offset!(RagePhPrismaticJoint, float_100, 0x100);
assert_offset!(RagePhPrismaticJoint, float_104, 0x104);
assert_offset!(RagePhPrismaticJoint, float_108, 0x108);
assert_offset!(RagePhPrismaticJoint, float_1c0, 0x1c0);
assert_offset!(RagePhPrismaticJoint, float_1c4, 0x1c4);
assert_offset!(RagePhPrismaticJoint, float_1d0, 0x1d0);
assert_offset!(RagePhPrismaticJoint, float_1d4, 0x1d4);
assert_offset!(RagePhPrismaticJoint, float_1e0, 0x1e0);
assert_offset!(RagePhPrismaticJoint, float_1e4, 0x1e4);
assert_offset!(RagePhPrismaticJoint, float_230, 0x230);
assert_offset!(RagePhPrismaticJoint, float_234, 0x234);
assert_offset!(RagePhPrismaticJoint, float_240, 0x240);
assert_offset!(RagePhPrismaticJoint, float_244, 0x244);
assert_offset!(RagePhPrismaticJoint, float_250, 0x250);
assert_offset!(RagePhPrismaticJoint, float_254, 0x254);
assert_offset!(RagePhPrismaticJoint, field_290, 0x290);
assert_offset!(RagePhPrismaticJoint, field_294, 0x294);
assert_offset!(RagePhPrismaticJoint, field_298, 0x298);
assert_offset!(RagePhPrismaticJoint, float_2f0, 0x2f0);
assert_offset!(RagePhPrismaticJoint, float_2f4, 0x2f4);
assert_offset!(RagePhPrismaticJoint, float_2f8, 0x2f8);
assert_offset!(RagePhPrismaticJoint, float_300, 0x300);
assert_offset!(RagePhPrismaticJoint, float_304, 0x304);
assert_offset!(RagePhPrismaticJoint, float_308, 0x308);

/// Merged layout for `rage::phSimulator`.
///
/// Size: 0xc0 (low). Bases: none.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhSimulator {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: int?, lanes: c-physics).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: int?, lanes: c-physics).
    pub field_8: u32,
    /// field_c (confidence: high, kind: int?, lanes: c-physics).
    pub field_c: u32,
    /// field_10 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: int?, lanes: c-physics).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: int?, lanes: c-physics).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: int?, lanes: c-physics).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: int?, lanes: c-physics).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_28: u32,
    /// field_2c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: int?, lanes: c-physics).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: int?, lanes: c-physics).
    pub field_38: u32,
    /// field_3c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_3c: u32,
    /// field_40 (confidence: low, kind: int?, lanes: c-physics).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: int?, lanes: c-physics).
    pub field_48: [u8; 8],
    /// field_50 (confidence: low, kind: int?, lanes: c-physics).
    pub field_50: [u8; 8],
    /// field_58 (confidence: low, kind: int?, lanes: c-physics).
    pub field_58: [u8; 8],
    /// field_60 (confidence: low, kind: int?, lanes: c-physics).
    pub field_60: [u8; 8],
    /// field_68 (confidence: low, kind: int?, lanes: c-physics).
    pub field_68: [u8; 8],
    /// field_70 (confidence: low, kind: int?, lanes: c-physics).
    pub field_70: [u8; 8],
    /// field_78 (confidence: low, kind: int?, lanes: c-physics).
    pub field_78: [u8; 8],
    /// field_80 (confidence: low, kind: int?, lanes: c-physics).
    pub field_80: [u8; 8],
    /// field_88 (confidence: low, kind: int?, lanes: c-physics).
    pub field_88: [u8; 8],
    /// field_90 (confidence: low, kind: int?, lanes: c-physics).
    pub field_90: [u8; 8],
    /// field_98 (confidence: low, kind: int?, lanes: c-physics).
    pub field_98: [u8; 8],
    /// field_a0 (confidence: low, kind: int?, lanes: c-physics).
    pub field_a0: [u8; 8],
    /// field_a8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_a8: u32,
    /// field_ac (confidence: low, kind: int?, lanes: c-physics).
    pub field_ac: u32,
    /// field_b0 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_b0: u32,
    /// field_b4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_b4: u32,
    /// field_b8 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_b8: u32,
    /// field_bc (confidence: low, kind: int?, lanes: c-physics).
    pub field_bc: u32,
}
assert_size!(RagePhSimulator, 0xc0); // merged size 0xc0 rounded to 4
assert_offset!(RagePhSimulator, vfptr, 0x0);
assert_offset!(RagePhSimulator, field_4, 0x4);
assert_offset!(RagePhSimulator, field_8, 0x8);
assert_offset!(RagePhSimulator, field_c, 0xc);
assert_offset!(RagePhSimulator, field_10, 0x10);
assert_offset!(RagePhSimulator, field_14, 0x14);
assert_offset!(RagePhSimulator, field_18, 0x18);
assert_offset!(RagePhSimulator, field_1c, 0x1c);
assert_offset!(RagePhSimulator, field_20, 0x20);
assert_offset!(RagePhSimulator, field_24, 0x24);
assert_offset!(RagePhSimulator, field_28, 0x28);
assert_offset!(RagePhSimulator, field_2c, 0x2c);
assert_offset!(RagePhSimulator, field_30, 0x30);
assert_offset!(RagePhSimulator, field_34, 0x34);
assert_offset!(RagePhSimulator, field_38, 0x38);
assert_offset!(RagePhSimulator, field_3c, 0x3c);
assert_offset!(RagePhSimulator, field_40, 0x40);
assert_offset!(RagePhSimulator, field_44, 0x44);
assert_offset!(RagePhSimulator, field_48, 0x48);
assert_offset!(RagePhSimulator, field_50, 0x50);
assert_offset!(RagePhSimulator, field_58, 0x58);
assert_offset!(RagePhSimulator, field_60, 0x60);
assert_offset!(RagePhSimulator, field_68, 0x68);
assert_offset!(RagePhSimulator, field_70, 0x70);
assert_offset!(RagePhSimulator, field_78, 0x78);
assert_offset!(RagePhSimulator, field_80, 0x80);
assert_offset!(RagePhSimulator, field_88, 0x88);
assert_offset!(RagePhSimulator, field_90, 0x90);
assert_offset!(RagePhSimulator, field_98, 0x98);
assert_offset!(RagePhSimulator, field_a0, 0xa0);
assert_offset!(RagePhSimulator, field_a8, 0xa8);
assert_offset!(RagePhSimulator, field_ac, 0xac);
assert_offset!(RagePhSimulator, field_b0, 0xb0);
assert_offset!(RagePhSimulator, field_b4, 0xb4);
assert_offset!(RagePhSimulator, field_b8, 0xb8);
assert_offset!(RagePhSimulator, field_bc, 0xbc);

/// Merged layout for `rage::phSleep`.
///
/// Size: 0x1a0 (low). Bases: none.
/// Lanes: c-physics.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RagePhSleep {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-physics).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_4: u32,
    /// float_08 (confidence: high, kind: float, lanes: c-physics).
    pub float_08: f32,
    /// ptr_0c (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_0c: Ptr32<u8>,
    /// ptr_10 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_10: Ptr32<u8>,
    /// ptr_14 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_14: Ptr32<u8>,
    /// field_18 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: int?, lanes: c-physics).
    pub field_1c: u32,
    /// field_20 (confidence: medium, kind: int?, lanes: c-physics).
    pub field_20: u32,
    /// ptr_24 (confidence: medium, kind: pointer, lanes: c-physics).
    pub ptr_24: Ptr32<u8>,
    /// bool_28 (confidence: low, kind: int?, lanes: c-physics).
    pub bool_28: u32,
    /// ptr_2c (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_2c: Ptr32<u8>,
    /// Unknown bytes (0x30..0x38).
    pub _pad_0030: [u8; 0x8],
    /// field_38 (confidence: low, kind: int?, lanes: c-physics).
    pub field_38: u32,
    /// ptr_3c (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_3c: Ptr32<u8>,
    /// Unknown bytes (0x40..0x44).
    pub _pad_0040: [u8; 0x4],
    /// ptr_44 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_44: Ptr32<u8>,
    /// Unknown bytes (0x48..0xa8).
    pub _pad_0048: [u8; 0x60],
    /// ptr_a8 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_a8: Ptr32<u8>,
    /// ptr_ac (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_ac: Ptr32<u8>,
    /// ptr_b0 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_b0: Ptr32<u8>,
    /// ptr_b4 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_b4: Ptr32<u8>,
    /// ptr_b8 (confidence: low, kind: pointer, lanes: c-physics).
    pub ptr_b8: Ptr32<u8>,
    /// Unknown bytes (0xbc..0xbe).
    pub _pad_00bc: [u8; 0x2],
    /// field_be (confidence: low, kind: int16?, lanes: c-physics).
    pub field_be: u16,
    /// Unknown bytes (0xc0..0xe4).
    pub _pad_00c0: [u8; 0x24],
    /// field_e4 (confidence: low, kind: int?, lanes: c-physics).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: int?, lanes: c-physics).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0x10c).
    pub _pad_00ec: [u8; 0x20],
    /// field_10c (confidence: low, kind: int?, lanes: c-physics).
    pub field_10c: u32,
    /// Unknown bytes (0x110..0x178).
    pub _pad_0110: [u8; 0x68],
    /// field_178 (confidence: low, kind: int?, lanes: c-physics).
    pub field_178: u32,
    /// Unknown bytes (0x17c..0x19c).
    pub _pad_017c: [u8; 0x20],
    /// field_19c (confidence: low, kind: int?, lanes: c-physics).
    pub field_19c: u32,
}
assert_size!(RagePhSleep, 0x1a0); // merged size 0x1a0 rounded to 4
assert_offset!(RagePhSleep, vfptr, 0x0);
assert_offset!(RagePhSleep, field_4, 0x4);
assert_offset!(RagePhSleep, float_08, 0x8);
assert_offset!(RagePhSleep, ptr_0c, 0xc);
assert_offset!(RagePhSleep, ptr_10, 0x10);
assert_offset!(RagePhSleep, ptr_14, 0x14);
assert_offset!(RagePhSleep, field_18, 0x18);
assert_offset!(RagePhSleep, field_1c, 0x1c);
assert_offset!(RagePhSleep, field_20, 0x20);
assert_offset!(RagePhSleep, ptr_24, 0x24);
assert_offset!(RagePhSleep, bool_28, 0x28);
assert_offset!(RagePhSleep, ptr_2c, 0x2c);
assert_offset!(RagePhSleep, field_38, 0x38);
assert_offset!(RagePhSleep, ptr_3c, 0x3c);
assert_offset!(RagePhSleep, ptr_44, 0x44);
assert_offset!(RagePhSleep, ptr_a8, 0xa8);
assert_offset!(RagePhSleep, ptr_ac, 0xac);
assert_offset!(RagePhSleep, ptr_b0, 0xb0);
assert_offset!(RagePhSleep, ptr_b4, 0xb4);
assert_offset!(RagePhSleep, ptr_b8, 0xb8);
assert_offset!(RagePhSleep, field_be, 0xbe);
assert_offset!(RagePhSleep, field_e4, 0xe4);
assert_offset!(RagePhSleep, field_e8, 0xe8);
assert_offset!(RagePhSleep, field_10c, 0x10c);
assert_offset!(RagePhSleep, field_178, 0x178);
assert_offset!(RagePhSleep, field_19c, 0x19c);
