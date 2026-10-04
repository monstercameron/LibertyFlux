//! World entities, model information and pickups.
//!
//! Holds 18 draft layouts: The entity hierarchy `CEntity`, `CDynamicEntity`, `CPhysical`,
//! `CBuilding`, `CObject` and `CCutsceneObject`, the scanners and seek-position calculators,
//! interiors and portals, `CBaseModelInfo` and `CModelInfo`, `C2dEffect`, and the pickup records
//! `CPickup` and `PickupPool`. Every layout is Inferred; size confidence (the analysis lanes' own
//! rating) is high for 1, medium for 0 and low for 17. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `C2dEffect`.
///
/// Size: 0xc (low). Bases: none.
/// Lanes: c-misc-a, c-misc-b, via:CParticleAttr, via:CSwayableAttr, via:CWalkDontWalkAttr, via:CWorldPointAttr.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2dEffect {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-misc-a,c-misc-b).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: flags, lanes: c-misc-b,via:CSwayableAttr,via:CWalkDontWalkAttr,via:CWorldPointAttr moved from siblings:CSwayableAttr,CWalkDontWalkAttr,CWorldPointAttr).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: flags, lanes: c-misc-a,c-misc-b,via:CParticleAttr,via:CSwayableAttr,via:CWalkDontWalkAttr,via:CWorldPointAttr moved from siblings:CParticleAttr,CSwayableAttr,CWalkDontWalkAttr,CWorldPointAttr).
    pub field_8: u32,
}
assert_size!(C2dEffect, 0xc); // merged size 0xc rounded to 4
assert_offset!(C2dEffect, vfptr, 0x0);
assert_offset!(C2dEffect, field_4, 0x4);
assert_offset!(C2dEffect, field_8, 0x8);

/// Merged layout for `CBaseModelInfo`.
///
/// Size: 0x60 (high). Bases: CVirtualBase@0x0.
/// Lanes: c-entities, n-06, n-07, p0-360.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBaseModelInfo {
    /// __vfptr (confidence: high, kind: u32, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// m_pArchetype (phArchetypeGta*, iv-sdk 1.0.7.0) (confidence: high, kind: u32, lanes: c-entities,p0-360).
    pub archetype: u32,
    /// Unknown bytes (0x8..0xc).
    pub _pad_0008: [u8; 0x4],
    /// m_pDrawableStruct (tDrawableStruct*, iv-sdk 1.0.7.0) (confidence: high, kind: pointer, lanes: c-entities,p0-360).
    pub drawable_struct: Ptr32<u8>,
    /// field_10 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// bounds_min (confidence: high, kind: vec4, lanes: c-entities,n-07).
    pub bounds_min: [f32; 4],
    /// bounds_max (confidence: high, kind: vec4, lanes: c-entities,n-07).
    pub bounds_max: [f32; 4],
    /// m_nIDEFlags (u32, iv-sdk 1.0.7.0) (confidence: high, kind: u32, lanes: c-entities,p0-360).
    pub ide_flags: u32,
    /// field_44 (confidence: low, kind: u32, lanes: c-entities).
    pub field_44: u32,
    /// m_nTexDictionary (s16, iv-sdk 1.0.7.0) (confidence: medium, kind: int16?, lanes: c-entities).
    pub tex_dictionary: u16,
    /// field_4a (confidence: medium, kind: int16?, lanes: c-entities).
    pub field_4a: u16,
    /// field_4c (confidence: low, kind: int16?, lanes: c-entities).
    pub field_4c: u16,
    /// field_4e (confidence: low, kind: int16?, lanes: c-entities).
    pub field_4e: u16,
    /// field_50 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_50: u16,
    /// field_52 (confidence: high, kind: int16?, lanes: c-entities).
    pub field_52: u16,
    /// field_54 (confidence: medium, kind: int16?, lanes: c-entities).
    pub field_54: u16,
    /// m_nAnimIndex (s8, iv-sdk 1.0.7.0) (confidence: low, kind: int16?, lanes: c-entities).
    pub anim_index: u16,
    /// field_58 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_58: u16,
    /// Unknown bytes (0x5a..0x5b).
    pub _pad_005a: [u8; 0x1],
    /// field_5b (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_5b: u8,
    /// field_5c (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_5c: Ptr32<u8>,
}
assert_size!(CBaseModelInfo, 0x60); // merged size 0x60 rounded to 4
assert_offset!(CBaseModelInfo, vfptr, 0x0);
assert_offset!(CBaseModelInfo, archetype, 0x4);
assert_offset!(CBaseModelInfo, drawable_struct, 0xc);
assert_offset!(CBaseModelInfo, field_10, 0x10);
assert_offset!(CBaseModelInfo, bounds_min, 0x20);
assert_offset!(CBaseModelInfo, bounds_max, 0x30);
assert_offset!(CBaseModelInfo, ide_flags, 0x40);
assert_offset!(CBaseModelInfo, field_44, 0x44);
assert_offset!(CBaseModelInfo, tex_dictionary, 0x48);
assert_offset!(CBaseModelInfo, field_4a, 0x4a);
assert_offset!(CBaseModelInfo, field_4c, 0x4c);
assert_offset!(CBaseModelInfo, field_4e, 0x4e);
assert_offset!(CBaseModelInfo, field_50, 0x50);
assert_offset!(CBaseModelInfo, field_52, 0x52);
assert_offset!(CBaseModelInfo, field_54, 0x54);
assert_offset!(CBaseModelInfo, anim_index, 0x56);
assert_offset!(CBaseModelInfo, field_58, 0x58);
assert_offset!(CBaseModelInfo, field_5b, 0x5b);
assert_offset!(CBaseModelInfo, field_5c, 0x5c);

/// Merged layout for `CBuilding`.
///
/// Size: 0x9c (low). Bases: CEntity@0x0.
/// Lanes: c-entities, via:CInteriorInst, via:CPortalInst.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBuilding {
    /// Unknown bytes (0x0..0x88).
    pub _pad_0000: [u8; 0x88],
    /// field_88 (confidence: high, kind: float, lanes: c-entities,via:CInteriorInst,via:CPortalInst moved from siblings:CInteriorInst,CPortalInst).
    pub field_88: f32,
    /// Unknown bytes (0x8c..0x94).
    pub _pad_008c: [u8; 0x8],
    /// field_94 (confidence: high, kind: float, lanes: c-entities,via:CInteriorInst,via:CPortalInst moved from siblings:CInteriorInst,CPortalInst).
    pub field_94: f32,
    /// field_98 (confidence: high, kind: float, lanes: c-entities,via:CInteriorInst,via:CPortalInst moved from siblings:CInteriorInst,CPortalInst).
    pub field_98: f32,
}
assert_size!(CBuilding, 0x9c); // merged size 0x9c rounded to 4
assert_offset!(CBuilding, field_88, 0x88);
assert_offset!(CBuilding, field_94, 0x94);
assert_offset!(CBuilding, field_98, 0x98);

/// Merged layout for `CCutsceneObject`.
///
/// Size: 0x318 (low). Bases: CObject@0x0.
/// Lanes: c-misc-a.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCutsceneObject {
    /// Unknown bytes (0x0..0x290).
    pub _pad_0000: [u8; 0x290],
    /// field_290 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_290: u32,
    /// field_294 (confidence: high, kind: u32, lanes: c-misc-a).
    pub field_294: u32,
    /// field_298 (confidence: medium, kind: u32, lanes: c-misc-a).
    pub field_298: u32,
    /// Unknown bytes (0x29c..0x2a8).
    pub _pad_029c: [u8; 0xc],
    /// field_2a8 (confidence: low, kind: bool_or_byte, lanes: c-misc-a).
    pub field_2a8: u8,
    /// field_2a9 (confidence: low, kind: word, lanes: c-misc-a).
    pub field_2a9: [u8; 2],
    /// Unknown bytes (0x2ab..0x2ac).
    pub _pad_02ab: [u8; 0x1],
    /// field_2ac (confidence: medium, kind: bool_or_byte, lanes: c-misc-a).
    pub field_2ac: u8,
    /// Unknown bytes (0x2ad..0x2b1).
    pub _pad_02ad: [u8; 0x4],
    /// field_2b1 (confidence: low, kind: bool_or_byte, lanes: c-misc-a).
    pub field_2b1: u8,
    /// Unknown bytes (0x2b2..0x2c0).
    pub _pad_02b2: [u8; 0xe],
    /// field_2c0 (confidence: low, kind: u32, lanes: c-misc-a).
    pub field_2c0: u32,
    /// Unknown bytes (0x2c4..0x2c8).
    pub _pad_02c4: [u8; 0x4],
    /// field_2c8 (confidence: low, kind: u32, lanes: c-misc-a).
    pub field_2c8: u32,
    /// field_2cc (confidence: low, kind: u32, lanes: c-misc-a).
    pub field_2cc: u32,
    /// Unknown bytes (0x2d0..0x2dc).
    pub _pad_02d0: [u8; 0xc],
    /// field_2dc (confidence: low, kind: u32, lanes: c-misc-a).
    pub field_2dc: u32,
    /// Unknown bytes (0x2e0..0x310).
    pub _pad_02e0: [u8; 0x30],
    /// field_310 (confidence: high, kind: u32, lanes: c-misc-a).
    pub field_310: u32,
    /// Unknown trailing bytes (0x314..0x318).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CCutsceneObject, 0x318); // merged size 0x318 rounded to 4
assert_offset!(CCutsceneObject, field_290, 0x290);
assert_offset!(CCutsceneObject, field_294, 0x294);
assert_offset!(CCutsceneObject, field_298, 0x298);
assert_offset!(CCutsceneObject, field_2a8, 0x2a8);
assert_offset!(CCutsceneObject, field_2a9, 0x2a9);
assert_offset!(CCutsceneObject, field_2ac, 0x2ac);
assert_offset!(CCutsceneObject, field_2b1, 0x2b1);
assert_offset!(CCutsceneObject, field_2c0, 0x2c0);
assert_offset!(CCutsceneObject, field_2c8, 0x2c8);
assert_offset!(CCutsceneObject, field_2cc, 0x2cc);
assert_offset!(CCutsceneObject, field_2dc, 0x2dc);
assert_offset!(CCutsceneObject, field_310, 0x310);

/// Merged layout for `CDynamicEntity`.
///
/// Size: 0x158 (low). Bases: CEntity@0x0.
/// Lanes: c-entities, n-07, p0-360, via:CDummyPed, via:CPed, via:CPhysical.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDynamicEntity {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: high, kind: u32, lanes: c-animation,c-entities).
    pub field_80: u32,
    /// Unknown bytes (0x84..0xd0).
    pub _pad_0084: [u8; 0x4c],
    /// field_d0 (confidence: high, kind: u32, lanes: c-animation,c-entities,c-misc-a).
    pub field_d0: u32,
    /// field_d4 (confidence: high, kind: u32, lanes: c-animation,c-entities,c-misc-a).
    pub field_d4: u32,
    /// Unknown bytes (0xd8..0xe0).
    pub _pad_00d8: [u8; 0x8],
    /// field_e0 (confidence: high, kind: float, lanes: c-animation,c-entities,n-05).
    pub field_e0: f32,
    /// field_e4 (confidence: high, kind: float, lanes: c-animation,c-entities).
    pub field_e4: f32,
    /// field_e8 (confidence: high, kind: float, lanes: c-animation,c-entities).
    pub field_e8: f32,
    /// field_ec (confidence: high, kind: float, lanes: c-animation,c-entities).
    pub field_ec: f32,
    /// Unknown bytes (0xf0..0xfc).
    pub _pad_00f0: [u8; 0xc],
    /// field_fc (confidence: high, kind: float, lanes: c-animation,c-entities).
    pub field_fc: f32,
    /// Unknown bytes (0x100..0x108).
    pub _pad_0100: [u8; 0x8],
    /// field_108 (confidence: low, kind: u32, lanes: c-entities).
    pub field_108: u32,
    /// Unknown bytes (0x10c..0x154).
    pub _pad_010c: [u8; 0x48],
    /// field_154 (confidence: medium, kind: u32, lanes: c-entities,via:CDummyPed,via:CPhysical moved from siblings:CDummyPed,CPhysical).
    pub field_154: u32,
}
assert_size!(CDynamicEntity, 0x158); // merged size 0x158 rounded to 4
assert_offset!(CDynamicEntity, field_80, 0x80);
assert_offset!(CDynamicEntity, field_d0, 0xd0);
assert_offset!(CDynamicEntity, field_d4, 0xd4);
assert_offset!(CDynamicEntity, field_e0, 0xe0);
assert_offset!(CDynamicEntity, field_e4, 0xe4);
assert_offset!(CDynamicEntity, field_e8, 0xe8);
assert_offset!(CDynamicEntity, field_ec, 0xec);
assert_offset!(CDynamicEntity, field_fc, 0xfc);
assert_offset!(CDynamicEntity, field_108, 0x108);
assert_offset!(CDynamicEntity, field_154, 0x154);

/// Merged layout for `CEntity`.
///
/// Size: 0x214 (low). Bases: CVirtualBase@0x0.
/// Lanes: c-animation, c-entities, n-01, n-03, n-04, n-05, n-06, n-07, n-10, n-12, n-16, n-18, n-20, p0-360, via:CBuilding, via:CDummyObject, via:CDynamicEntity, via:CInteriorInst, via:CPortalInst.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEntity {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-animation,c-entities,c-misc-a,n-16).
    pub vfptr: Ptr32<()>,
    /// m_nCurrentWeaponSlot (confidence: high, kind: u32, lanes: c-animation,c-entities,p0-360).
    pub current_weapon_slot: u32,
    /// Unknown bytes (0x8..0xc).
    pub _pad_0008: [u8; 0x4],
    /// field_c (confidence: high, kind: u32, lanes: c-animation,c-entities,n-22).
    pub field_c: u32,
    /// m_placement (CSimpleTransform, iv-sdk 1.0.7.0) (confidence: high, kind: float, lanes: c-animation,c-entities,n-10,n-15).
    pub placement: f32,
    /// Unknown bytes (0x14..0x20).
    pub _pad_0014: [u8; 0xc],
    /// placement_matrix34 (confidence: high, kind: pointer, lanes: c-animation,c-entities,c-misc-a,n-01,n-05,n-06,n-07,n-08,n-10,n-11,n-14,n-15,n-16,n-18,n-20,n-22,p0-360).
    pub placement_matrix34: Ptr32<u8>,
    /// visible (confidence: high, kind: u32, lanes: c-animation,c-entities,c-misc-a,n-05,n-06,n-10,n-15,n-16,n-18,n-22).
    pub visible: u32,
    /// m_nEntityFlags2 (u32, iv-sdk 1.0.7.0) (confidence: high, kind: flags, lanes: c-animation,c-entities,c-misc-a,n-04,n-05,n-07,n-08,n-10,n-11,n-18,n-20,p0-360).
    pub entity_flags2: u32,
    /// Unknown bytes (0x2c..0x2e).
    pub _pad_002c: [u8; 0x2],
    /// model_index (confidence: high, kind: i32, lanes: c-animation,c-entities,c-misc-a,n-04,n-05,n-06,n-07,n-10,n-15,n-20).
    pub model_index: u16,
    /// field_30 (confidence: medium, kind: u32, lanes: c-entities,n-02).
    pub field_30: u32,
    /// m_pDrawablePtr (tObjectDrawable*, iv-sdk 1.0.7.0) (confidence: high, kind: u32, lanes: c-animation,c-entities,n-13,p0-360).
    pub drawable_ptr: u32,
    /// physics_rigid_body_record (confidence: high, kind: pointer, lanes: c-animation,c-entities,n-01,n-02,n-04,n-05,n-11,n-16,n-18,p0-360).
    pub physics_rigid_body_record: Ptr32<u8>,
    /// field_3c (confidence: medium, kind: u32, lanes: c-entities,n-02,n-16).
    pub field_3c: u32,
    /// field_40 (confidence: high, kind: bool-or-byte, lanes: c-entities,n-08).
    pub field_40: u8,
    /// Unknown bytes (0x41..0x42).
    pub _pad_0041: [u8; 0x1],
    /// field_42 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_42: u16,
    /// field_44 (confidence: medium, kind: int16?, lanes: c-entities).
    pub field_44: u16,
    /// Unknown bytes (0x46..0x48).
    pub _pad_0046: [u8; 0x2],
    /// m_hInterior (confidence: high, kind: u32, lanes: c-entities,n-08,p0-360).
    pub interior: u32,
    /// field_4c (confidence: high, kind: u32, lanes: c-animation,c-entities).
    pub field_4c: u32,
    /// m_fDrawDistance (f32, iv-sdk 1.0.7.0) (confidence: high, kind: u32, lanes: c-animation,c-entities,p0-360).
    pub draw_distance: u32,
    /// field_54 (confidence: medium, kind: u32, lanes: c-entities,n-16).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: u32, lanes: c-animation,c-entities).
    pub field_58: u32,
    /// field_5c (confidence: low, kind: int16?, lanes: c-entities).
    pub field_5c: u16,
    /// field_5e (confidence: low, kind: int16?, lanes: c-entities).
    pub field_5e: u16,
    /// field_60 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_60: u8,
    /// field_61 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_61: [u8; 2],
    /// alpha (confidence: high, kind: u8, lanes: c-entities,c-misc-a,n-18,p0-360).
    pub alpha: u8,
    /// Unknown bytes (0x64..0x6c).
    pub _pad_0064: [u8; 0x8],
    /// network_object (confidence: high, kind: pointer, lanes: c-entities,n-02,n-07,n-08,n-10,n-11,n-12,n-14,n-15,n-16,n-18,n-22,p0-360).
    pub network_object: Ptr32<u8>,
    /// Unknown bytes (0x70..0x78).
    pub _pad_0070: [u8; 0x8],
    /// anim_state_passed_shared (confidence: high, kind: pointer, lanes: c-animation,c-entities,n-05,n-10,n-13,n-15,n-16,n-18,p0-360).
    pub anim_state_passed_shared: Ptr32<u8>,
    /// Unknown bytes (0x7c..0xf0).
    pub _pad_007c: [u8; 0x74],
    /// field_f0 (confidence: high, kind: u32, lanes: c-animation,c-entities,via:CDynamicEntity,via:CInteriorInst moved from siblings:CBuilding,CDynamicEntity).
    pub field_f0: u32,
    /// Unknown bytes (0xf4..0x100).
    pub _pad_00f4: [u8; 0xc],
    /// field_100 (confidence: high, kind: u32, lanes: c-animation,c-entities,n-01,via:CDynamicEntity,via:CInteriorInst moved from siblings:CBuilding,CDynamicEntity).
    pub field_100: u32,
    /// field_104 (confidence: medium, kind: u32, lanes: c-entities,via:CDynamicEntity,via:CInteriorInst moved from siblings:CBuilding,CDynamicEntity).
    pub field_104: u32,
    /// Unknown bytes (0x108..0x118).
    pub _pad_0108: [u8; 0x10],
    /// proof_script_arg (confidence: high, kind: flags, lanes: c-entities,n-04,n-07,n-10,n-15,n-16,n-18).
    pub proof_script_arg: u32,
    /// Unknown bytes (0x11c..0x150).
    pub _pad_011c: [u8; 0x34],
    /// touching_list_entry_count (confidence: high, kind: i32, lanes: c-entities,n-10).
    pub touching_list_entry_count: u8,
    /// Unknown bytes (0x151..0x1bc).
    pub _pad_0151: [u8; 0x6b],
    /// attached_entity (confidence: high, kind: u32, lanes: c-entities,n-05,n-07,n-10,p0-360).
    pub attached_entity: u32,
    /// Unknown bytes (0x1c0..0x1e2).
    pub _pad_01c0: [u8; 0x22],
    /// attachment_state_low_nibble (confidence: high, kind: u8, lanes: c-entities,n-03,n-05,n-07,n-10).
    pub attachment_state_low_nibble: u8,
    /// Unknown trailing bytes (0x1e3..0x214).
    pub _pad_end: [u8; 0x31],
}
assert_size!(CEntity, 0x214); // merged size 0x214 rounded to 4
assert_offset!(CEntity, vfptr, 0x0);
assert_offset!(CEntity, current_weapon_slot, 0x4);
assert_offset!(CEntity, field_c, 0xc);
assert_offset!(CEntity, placement, 0x10);
assert_offset!(CEntity, placement_matrix34, 0x20);
assert_offset!(CEntity, visible, 0x24);
assert_offset!(CEntity, entity_flags2, 0x28);
assert_offset!(CEntity, model_index, 0x2e);
assert_offset!(CEntity, field_30, 0x30);
assert_offset!(CEntity, drawable_ptr, 0x34);
assert_offset!(CEntity, physics_rigid_body_record, 0x38);
assert_offset!(CEntity, field_3c, 0x3c);
assert_offset!(CEntity, field_40, 0x40);
assert_offset!(CEntity, field_42, 0x42);
assert_offset!(CEntity, field_44, 0x44);
assert_offset!(CEntity, interior, 0x48);
assert_offset!(CEntity, field_4c, 0x4c);
assert_offset!(CEntity, draw_distance, 0x50);
assert_offset!(CEntity, field_54, 0x54);
assert_offset!(CEntity, field_58, 0x58);
assert_offset!(CEntity, field_5c, 0x5c);
assert_offset!(CEntity, field_5e, 0x5e);
assert_offset!(CEntity, field_60, 0x60);
assert_offset!(CEntity, field_61, 0x61);
assert_offset!(CEntity, alpha, 0x63);
assert_offset!(CEntity, network_object, 0x6c);
assert_offset!(CEntity, anim_state_passed_shared, 0x78);
assert_offset!(CEntity, field_f0, 0xf0);
assert_offset!(CEntity, field_100, 0x100);
assert_offset!(CEntity, field_104, 0x104);
assert_offset!(CEntity, proof_script_arg, 0x118);
assert_offset!(CEntity, touching_list_entry_count, 0x150);
assert_offset!(CEntity, attached_entity, 0x1bc);
assert_offset!(CEntity, attachment_state_low_nibble, 0x1e2);

/// Merged layout for `CEntityScanner`.
///
/// Size: 0x12c (low). Bases: CExpensiveProcess@0x4.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEntityScanner {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_8: u32,
    /// field_c (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_c: u8,
    /// Unknown bytes (0xd..0x18).
    pub _pad_000d: [u8; 0xb],
    /// field_18 (confidence: low, kind: double, lanes: c-entities).
    pub field_18: [u8; 8],
    /// field_20 (confidence: low, kind: double, lanes: c-entities).
    pub field_20: [u8; 8],
    /// field_28 (confidence: low, kind: double, lanes: c-entities).
    pub field_28: [u8; 8],
    /// field_30 (confidence: low, kind: double, lanes: c-entities).
    pub field_30: [u8; 8],
    /// field_38 (confidence: low, kind: double, lanes: c-entities).
    pub field_38: [u8; 8],
    /// field_40 (confidence: low, kind: double, lanes: c-entities).
    pub field_40: [u8; 8],
    /// field_48 (confidence: low, kind: double, lanes: c-entities).
    pub field_48: [u8; 8],
    /// field_50 (confidence: low, kind: double, lanes: c-entities).
    pub field_50: [u8; 8],
    /// field_58 (confidence: low, kind: u32, lanes: c-entities).
    pub field_58: u32,
    /// Unknown bytes (0x5c..0xd0).
    pub _pad_005c: [u8; 0x74],
    /// field_d0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d0: u32,
    /// Unknown bytes (0xd4..0xec).
    pub _pad_00d4: [u8; 0x18],
    /// field_ec (confidence: low, kind: u32, lanes: c-entities).
    pub field_ec: u32,
    /// Unknown bytes (0xf0..0x104).
    pub _pad_00f0: [u8; 0x14],
    /// field_104 (confidence: low, kind: u32, lanes: c-entities).
    pub field_104: u32,
    /// Unknown bytes (0x108..0x11c).
    pub _pad_0108: [u8; 0x14],
    /// field_11c (confidence: low, kind: u32, lanes: c-entities).
    pub field_11c: u32,
    /// Unknown bytes (0x120..0x128).
    pub _pad_0120: [u8; 0x8],
    /// field_128 (confidence: low, kind: u32, lanes: c-entities).
    pub field_128: u32,
}
assert_size!(CEntityScanner, 0x12c); // merged size 0x12c rounded to 4
assert_offset!(CEntityScanner, vfptr, 0x0);
assert_offset!(CEntityScanner, field_4, 0x4);
assert_offset!(CEntityScanner, field_8, 0x8);
assert_offset!(CEntityScanner, field_c, 0xc);
assert_offset!(CEntityScanner, field_18, 0x18);
assert_offset!(CEntityScanner, field_20, 0x20);
assert_offset!(CEntityScanner, field_28, 0x28);
assert_offset!(CEntityScanner, field_30, 0x30);
assert_offset!(CEntityScanner, field_38, 0x38);
assert_offset!(CEntityScanner, field_40, 0x40);
assert_offset!(CEntityScanner, field_48, 0x48);
assert_offset!(CEntityScanner, field_50, 0x50);
assert_offset!(CEntityScanner, field_58, 0x58);
assert_offset!(CEntityScanner, field_d0, 0xd0);
assert_offset!(CEntityScanner, field_ec, 0xec);
assert_offset!(CEntityScanner, field_104, 0x104);
assert_offset!(CEntityScanner, field_11c, 0x11c);
assert_offset!(CEntityScanner, field_128, 0x128);

/// Merged layout for `CEntitySeekPosCalculator`.
///
/// Size: 0x78 (low). Bases: none.
/// Lanes: c-entities, via:CEntitySeekPosCalculatorRadiusAngleOffset, via:CEntitySeekPosCalculatorStandard, via:CEntitySeekPosCalculatorXYOffset.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEntitySeekPosCalculator {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x14).
    pub _pad_0004: [u8; 0x10],
    /// field_14 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// field_20 (confidence: high, kind: u32, lanes: c-entities).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x30).
    pub _pad_0024: [u8; 0xc],
    /// field_30 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_30: u32,
    /// field_34 (confidence: high, kind: float, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_34: f32,
    /// field_38 (confidence: high, kind: float, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_38: f32,
    /// field_3c (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard,via:CEntitySeekPosCalculatorXYOffset moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard,CEntitySeekPosCalculatorXYOffset).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard,via:CEntitySeekPosCalculatorXYOffset moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard,CEntitySeekPosCalculatorXYOffset).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x4c).
    pub _pad_0048: [u8; 0x4],
    /// field_4c (confidence: high, kind: float, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_4c: f32,
    /// field_50 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_50: u32,
    /// field_54 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_54: u32,
    /// field_58 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_58: u32,
    /// field_5c (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_5c: u32,
    /// field_60 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_60: u32,
    /// field_64 (confidence: medium, kind: int16?, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_64: u16,
    /// Unknown bytes (0x66..0x68).
    pub _pad_0066: [u8; 0x2],
    /// field_68 (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_68: u32,
    /// field_6c (confidence: medium, kind: u32, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_6c: u32,
    /// field_70 (confidence: medium, kind: int16?, lanes: c-entities,via:CEntitySeekPosCalculatorRadiusAngleOffset,via:CEntitySeekPosCalculatorStandard moved from siblings:CEntitySeekPosCalculatorRadiusAngleOffset,CEntitySeekPosCalculatorStandard).
    pub field_70: u16,
    /// Unknown bytes (0x72..0x74).
    pub _pad_0072: [u8; 0x2],
    /// field_74 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_74: u32,
}
assert_size!(CEntitySeekPosCalculator, 0x78); // merged size 0x78 rounded to 4
assert_offset!(CEntitySeekPosCalculator, vfptr, 0x0);
assert_offset!(CEntitySeekPosCalculator, field_14, 0x14);
assert_offset!(CEntitySeekPosCalculator, field_18, 0x18);
assert_offset!(CEntitySeekPosCalculator, field_20, 0x20);
assert_offset!(CEntitySeekPosCalculator, field_30, 0x30);
assert_offset!(CEntitySeekPosCalculator, field_34, 0x34);
assert_offset!(CEntitySeekPosCalculator, field_38, 0x38);
assert_offset!(CEntitySeekPosCalculator, field_3c, 0x3c);
assert_offset!(CEntitySeekPosCalculator, field_40, 0x40);
assert_offset!(CEntitySeekPosCalculator, field_44, 0x44);
assert_offset!(CEntitySeekPosCalculator, field_4c, 0x4c);
assert_offset!(CEntitySeekPosCalculator, field_50, 0x50);
assert_offset!(CEntitySeekPosCalculator, field_54, 0x54);
assert_offset!(CEntitySeekPosCalculator, field_58, 0x58);
assert_offset!(CEntitySeekPosCalculator, field_5c, 0x5c);
assert_offset!(CEntitySeekPosCalculator, field_60, 0x60);
assert_offset!(CEntitySeekPosCalculator, field_64, 0x64);
assert_offset!(CEntitySeekPosCalculator, field_68, 0x68);
assert_offset!(CEntitySeekPosCalculator, field_6c, 0x6c);
assert_offset!(CEntitySeekPosCalculator, field_70, 0x70);
assert_offset!(CEntitySeekPosCalculator, field_74, 0x74);

/// Merged layout for `CEntitySeekPosCalculatorRadiusAngleOffset`.
///
/// Size: 0x81 (low). Bases: CEntitySeekPosCalculator@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CEntitySeekPosCalculatorRadiusAngleOffset {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: medium, kind: float, lanes: c-entities).
    pub field_4: f32,
    /// field_8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_8: f32,
    /// Unknown bytes (0xc..0x48).
    pub _pad_000c: [u8; 0x3c],
    /// field_48 (confidence: medium, kind: float, lanes: c-entities).
    pub field_48: f32,
    /// Unknown bytes (0x4c..0x78).
    pub _pad_004c: [u8; 0x2c],
    /// field_78 (confidence: low, kind: u32, lanes: c-entities).
    pub field_78: u32,
    /// field_7c (confidence: low, kind: u32, lanes: c-entities).
    pub field_7c: u32,
    /// field_80 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_80: u8,
    /// Unknown trailing bytes (0x81..0x84).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CEntitySeekPosCalculatorRadiusAngleOffset, 0x84); // merged size 0x81 rounded to 4
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_4, 0x4);
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_8, 0x8);
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_48, 0x48);
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_78, 0x78);
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_7c, 0x7c);
assert_offset!(CEntitySeekPosCalculatorRadiusAngleOffset, field_80, 0x80);

/// Merged layout for `CInteriorInst`.
///
/// Size: 0x15a (low). Bases: CBuilding@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CInteriorInst {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_80: u32,
    /// field_84 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_84: Ptr32<u8>,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: high, kind: u32, lanes: c-entities).
    pub field_90: u32,
    /// Unknown bytes (0x94..0x9c).
    pub _pad_0094: [u8; 0x8],
    /// field_9c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_9c: u32,
    /// field_a0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_a0: u32,
    /// Unknown bytes (0xa4..0xc0).
    pub _pad_00a4: [u8; 0x1c],
    /// field_c0 (confidence: high, kind: float, lanes: c-entities).
    pub field_c0: f32,
    /// field_c4 (confidence: high, kind: float, lanes: c-entities).
    pub field_c4: f32,
    /// field_c8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_c8: f32,
    /// Unknown bytes (0xcc..0xd0).
    pub _pad_00cc: [u8; 0x4],
    /// field_d0 (confidence: high, kind: float, lanes: c-entities).
    pub field_d0: f32,
    /// field_d4 (confidence: high, kind: float, lanes: c-entities).
    pub field_d4: f32,
    /// field_d8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_d8: f32,
    /// field_dc (confidence: low, kind: u32, lanes: c-entities).
    pub field_dc: u32,
    /// field_e0 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_e0: u8,
    /// Unknown bytes (0xe1..0xe4).
    pub _pad_00e1: [u8; 0x3],
    /// field_e4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e4: u32,
    /// field_e8 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_e8: Ptr32<u8>,
    /// field_ec (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ec: u32,
    /// Unknown bytes (0xf0..0xfc).
    pub _pad_00f0: [u8; 0xc],
    /// field_fc (confidence: low, kind: u32, lanes: c-entities).
    pub field_fc: u32,
    /// Unknown bytes (0x100..0x108).
    pub _pad_0100: [u8; 0x8],
    /// field_108 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_108: u8,
    /// Unknown bytes (0x109..0x10c).
    pub _pad_0109: [u8; 0x3],
    /// field_10c (confidence: low, kind: u32, lanes: c-entities).
    pub field_10c: u32,
    /// field_110 (confidence: low, kind: u32, lanes: c-entities).
    pub field_110: u32,
    /// field_114 (confidence: low, kind: u32, lanes: c-entities).
    pub field_114: u32,
    /// Unknown bytes (0x118..0x11c).
    pub _pad_0118: [u8; 0x4],
    /// field_11c (confidence: low, kind: u32, lanes: c-entities).
    pub field_11c: u32,
    /// Unknown bytes (0x120..0x154).
    pub _pad_0120: [u8; 0x34],
    /// field_154 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_154: Ptr32<u8>,
    /// Unknown trailing bytes (0x158..0x15c).
    pub _pad_end: [u8; 0x4],
}
assert_size!(CInteriorInst, 0x15c); // merged size 0x15a rounded to 4
assert_offset!(CInteriorInst, field_80, 0x80);
assert_offset!(CInteriorInst, field_84, 0x84);
assert_offset!(CInteriorInst, field_90, 0x90);
assert_offset!(CInteriorInst, field_9c, 0x9c);
assert_offset!(CInteriorInst, field_a0, 0xa0);
assert_offset!(CInteriorInst, field_c0, 0xc0);
assert_offset!(CInteriorInst, field_c4, 0xc4);
assert_offset!(CInteriorInst, field_c8, 0xc8);
assert_offset!(CInteriorInst, field_d0, 0xd0);
assert_offset!(CInteriorInst, field_d4, 0xd4);
assert_offset!(CInteriorInst, field_d8, 0xd8);
assert_offset!(CInteriorInst, field_dc, 0xdc);
assert_offset!(CInteriorInst, field_e0, 0xe0);
assert_offset!(CInteriorInst, field_e4, 0xe4);
assert_offset!(CInteriorInst, field_e8, 0xe8);
assert_offset!(CInteriorInst, field_ec, 0xec);
assert_offset!(CInteriorInst, field_fc, 0xfc);
assert_offset!(CInteriorInst, field_108, 0x108);
assert_offset!(CInteriorInst, field_10c, 0x10c);
assert_offset!(CInteriorInst, field_110, 0x110);
assert_offset!(CInteriorInst, field_114, 0x114);
assert_offset!(CInteriorInst, field_11c, 0x11c);
assert_offset!(CInteriorInst, field_154, 0x154);

/// Merged layout for `CModelInfo`.
///
/// Size: 0x70 (low). Bases: none.
/// Lanes: n-08, n-11, n-14.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CModelInfo {
    /// Unknown bytes (0x0..0xc).
    pub _pad_0000: [u8; 0xc],
    /// vtable_slot_category_query (confidence: medium, kind: u32, lanes: n-08).
    pub vtable_slot_category_query: u32,
    /// Unknown bytes (0x10..0x52).
    pub _pad_0010: [u8; 0x42],
    /// collision_streaming_index (confidence: medium, kind: i32, lanes: n-14).
    pub collision_streaming_index: u16,
    /// Unknown bytes (0x54..0x6c).
    pub _pad_0054: [u8; 0x18],
    /// vehicle_type_id (confidence: medium, kind: u32, lanes: n-08).
    pub vehicle_type_id: u32,
}
assert_size!(CModelInfo, 0x70); // merged size 0x70 rounded to 4
assert_offset!(CModelInfo, vtable_slot_category_query, 0xc);
assert_offset!(CModelInfo, collision_streaming_index, 0x52);
assert_offset!(CModelInfo, vehicle_type_id, 0x6c);

/// Merged layout for `CObject`.
///
/// Size: 0x290 (low). Bases: CPhysical@0x0.
/// Lanes: c-entities, n-02, n-04, n-05, n-06, n-07, n-08, n-10, n-12, n-15, n-18, n-22, p0-360.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CObject {
    /// Unknown bytes (0x0..0x214).
    pub _pad_0000: [u8; 0x214],
    /// stealable (confidence: high, kind: flags, lanes: c-entities,n-18).
    pub stealable: u32,
    /// Unknown bytes (0x218..0x240).
    pub _pad_0218: [u8; 0x28],
    /// door_state_related (confidence: high, kind: float, lanes: c-entities,n-06).
    pub door_state_related: f32,
    /// field_244 (confidence: medium, kind: float, lanes: c-entities).
    pub field_244: f32,
    /// field_248 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_248: u8,
    /// field_249 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_249: u8,
    /// field_24a (confidence: low, kind: int16?, lanes: c-entities).
    pub field_24a: u16,
    /// Unknown bytes (0x24c..0x250).
    pub _pad_024c: [u8; 0x4],
    /// scale (confidence: high, kind: u32, lanes: n-18,p0-360).
    pub scale: u32,
    /// Unknown bytes (0x254..0x280).
    pub _pad_0254: [u8; 0x2c],
    /// field_280 (confidence: high, kind: u32, lanes: c-entities).
    pub field_280: u32,
    /// Unknown trailing bytes (0x284..0x290).
    pub _pad_end: [u8; 0xc],
}
assert_size!(CObject, 0x290); // merged size 0x290 rounded to 4
assert_offset!(CObject, stealable, 0x214);
assert_offset!(CObject, door_state_related, 0x240);
assert_offset!(CObject, field_244, 0x244);
assert_offset!(CObject, field_248, 0x248);
assert_offset!(CObject, field_249, 0x249);
assert_offset!(CObject, field_24a, 0x24a);
assert_offset!(CObject, scale, 0x250);
assert_offset!(CObject, field_280, 0x280);

/// Merged layout for `CObjectScanner`.
///
/// Size: 0x2e4 (low). Bases: CEntityScanner@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CObjectScanner {
    /// Unknown bytes (0x0..0x84).
    pub _pad_0000: [u8; 0x84],
    /// field_84 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_84: u32,
    /// Unknown bytes (0x88..0xd4).
    pub _pad_0088: [u8; 0x4c],
    /// field_d4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d4: u32,
    /// field_d8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d8: u32,
    /// field_dc (confidence: low, kind: u32, lanes: c-entities).
    pub field_dc: u32,
    /// field_e0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e0: u32,
    /// field_e4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e4: u32,
    /// field_e8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_e8: u32,
    /// Unknown bytes (0xec..0xf0).
    pub _pad_00ec: [u8; 0x4],
    /// field_f0 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_f0: u16,
    /// Unknown bytes (0xf2..0xf4).
    pub _pad_00f2: [u8; 0x2],
    /// field_f4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_f4: u32,
    /// Unknown bytes (0xf8..0x150).
    pub _pad_00f8: [u8; 0x58],
    /// field_150 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_150: u32,
    /// Unknown bytes (0x154..0x1ac).
    pub _pad_0154: [u8; 0x58],
    /// field_1ac (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_1ac: u32,
    /// Unknown bytes (0x1b0..0x208).
    pub _pad_01b0: [u8; 0x58],
    /// field_208 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_208: u8,
    /// Unknown bytes (0x209..0x20c).
    pub _pad_0209: [u8; 0x3],
    /// field_20c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_20c: u32,
    /// field_210 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_210: u32,
    /// field_214 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_214: u16,
    /// Unknown bytes (0x216..0x21e).
    pub _pad_0216: [u8; 0x8],
    /// field_21e (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_21e: u8,
    /// Unknown bytes (0x21f..0x280).
    pub _pad_021f: [u8; 0x61],
    /// field_280 (confidence: low, kind: u32, lanes: c-entities).
    pub field_280: u32,
    /// field_284 (confidence: low, kind: u32, lanes: c-entities).
    pub field_284: u32,
    /// field_288 (confidence: low, kind: u32, lanes: c-entities).
    pub field_288: u32,
    /// field_28c (confidence: low, kind: u32, lanes: c-entities).
    pub field_28c: u32,
    /// field_290 (confidence: low, kind: u32, lanes: c-entities).
    pub field_290: u32,
    /// field_294 (confidence: low, kind: u32, lanes: c-entities).
    pub field_294: u32,
    /// field_298 (confidence: low, kind: u32, lanes: c-entities).
    pub field_298: u32,
    /// Unknown bytes (0x29c..0x2a0).
    pub _pad_029c: [u8; 0x4],
    /// field_2a0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2a0: u32,
    /// Unknown bytes (0x2a4..0x2b0).
    pub _pad_02a4: [u8; 0xc],
    /// field_2b0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2b0: u32,
    /// field_2b4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2b4: u32,
    /// field_2b8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2b8: u32,
    /// Unknown bytes (0x2bc..0x2c0).
    pub _pad_02bc: [u8; 0x4],
    /// field_2c0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2c0: u32,
    /// field_2c4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2c4: u32,
    /// field_2c8 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_2c8: u8,
    /// Unknown bytes (0x2c9..0x2cc).
    pub _pad_02c9: [u8; 0x3],
    /// field_2cc (confidence: low, kind: u32, lanes: c-entities).
    pub field_2cc: u32,
    /// field_2d0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2d0: u32,
    /// field_2d4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2d4: u32,
    /// field_2d8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2d8: u32,
    /// Unknown bytes (0x2dc..0x2e0).
    pub _pad_02dc: [u8; 0x4],
    /// field_2e0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_2e0: u32,
}
assert_size!(CObjectScanner, 0x2e4); // merged size 0x2e4 rounded to 4
assert_offset!(CObjectScanner, field_84, 0x84);
assert_offset!(CObjectScanner, field_d4, 0xd4);
assert_offset!(CObjectScanner, field_d8, 0xd8);
assert_offset!(CObjectScanner, field_dc, 0xdc);
assert_offset!(CObjectScanner, field_e0, 0xe0);
assert_offset!(CObjectScanner, field_e4, 0xe4);
assert_offset!(CObjectScanner, field_e8, 0xe8);
assert_offset!(CObjectScanner, field_f0, 0xf0);
assert_offset!(CObjectScanner, field_f4, 0xf4);
assert_offset!(CObjectScanner, field_150, 0x150);
assert_offset!(CObjectScanner, field_1ac, 0x1ac);
assert_offset!(CObjectScanner, field_208, 0x208);
assert_offset!(CObjectScanner, field_20c, 0x20c);
assert_offset!(CObjectScanner, field_210, 0x210);
assert_offset!(CObjectScanner, field_214, 0x214);
assert_offset!(CObjectScanner, field_21e, 0x21e);
assert_offset!(CObjectScanner, field_280, 0x280);
assert_offset!(CObjectScanner, field_284, 0x284);
assert_offset!(CObjectScanner, field_288, 0x288);
assert_offset!(CObjectScanner, field_28c, 0x28c);
assert_offset!(CObjectScanner, field_290, 0x290);
assert_offset!(CObjectScanner, field_294, 0x294);
assert_offset!(CObjectScanner, field_298, 0x298);
assert_offset!(CObjectScanner, field_2a0, 0x2a0);
assert_offset!(CObjectScanner, field_2b0, 0x2b0);
assert_offset!(CObjectScanner, field_2b4, 0x2b4);
assert_offset!(CObjectScanner, field_2b8, 0x2b8);
assert_offset!(CObjectScanner, field_2c0, 0x2c0);
assert_offset!(CObjectScanner, field_2c4, 0x2c4);
assert_offset!(CObjectScanner, field_2c8, 0x2c8);
assert_offset!(CObjectScanner, field_2cc, 0x2cc);
assert_offset!(CObjectScanner, field_2d0, 0x2d0);
assert_offset!(CObjectScanner, field_2d4, 0x2d4);
assert_offset!(CObjectScanner, field_2d8, 0x2d8);
assert_offset!(CObjectScanner, field_2e0, 0x2e0);

/// Merged layout for `CPhysical`.
///
/// Size: 0xea8 (low). Bases: CDynamicEntity@0x0.
/// Lanes: c-entities, c-misc-a, n-02, n-04, n-05, n-06, n-07, n-08, n-10, n-11, n-14, n-16, n-18, n-20, p0-360, via:CCutsceneObject, via:CHeli, via:CObject, via:CPed, via:CPlayerPed, via:CTrain, via:CVehicle.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPhysical {
    /// Unknown bytes (0x0..0x2c).
    pub _pad_0000: [u8; 0x2c],
    /// field_2c (confidence: high, kind: int16?, lanes: c-entities,via:CHeli,via:CPed moved from siblings:CPed,CVehicle).
    pub field_2c: u16,
    /// Unknown bytes (0x2e..0x41).
    pub _pad_002e: [u8; 0x13],
    /// field_41 (confidence: high, kind: u8, lanes: c-misc-a,n-16,via:CCutsceneObject,via:CPed moved from siblings:CObject,CPed).
    pub field_41: u8,
    /// Unknown bytes (0x42..0x110).
    pub _pad_0042: [u8; 0xce],
    /// field_110 (confidence: low, kind: u32, lanes: c-entities).
    pub field_110: u32,
    /// Unknown bytes (0x114..0x11c).
    pub _pad_0114: [u8; 0x8],
    /// field_11c (confidence: medium, kind: int16?, lanes: c-entities,n-15,n-16).
    pub field_11c: u16,
    /// Unknown bytes (0x11e..0x120).
    pub _pad_011e: [u8; 0x2],
    /// m_fPercentSubmerged (confidence: high, kind: float, lanes: c-entities,c-misc-a,p0-360).
    pub percent_submerged: f32,
    /// Unknown bytes (0x124..0x15c).
    pub _pad_0124: [u8; 0x38],
    /// field_15c (confidence: low, kind: u32, lanes: c-entities).
    pub field_15c: u32,
    /// field_160 (confidence: low, kind: u32, lanes: c-entities).
    pub field_160: u32,
    /// Unknown bytes (0x164..0x170).
    pub _pad_0164: [u8; 0xc],
    /// field_170 (confidence: low, kind: u32, lanes: c-entities).
    pub field_170: u32,
    /// field_174 (confidence: low, kind: u32, lanes: c-entities).
    pub field_174: u32,
    /// Unknown bytes (0x178..0x190).
    pub _pad_0178: [u8; 0x18],
    /// field_190 (confidence: low, kind: u32, lanes: c-entities).
    pub field_190: u32,
    /// Unknown bytes (0x194..0x1a0).
    pub _pad_0194: [u8; 0xc],
    /// field_1a0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1a0: u32,
    /// field_1a4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1a4: u32,
    /// field_1a8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1a8: u32,
    /// field_1ac (confidence: high, kind: u32, lanes: c-entities,n-10).
    pub field_1ac: u32,
    /// Unknown bytes (0x1b0..0x1c0).
    pub _pad_01b0: [u8; 0x10],
    /// m_vAttachOffset (CVector, iv-sdk 1.0.7.0) (confidence: high, kind: float, lanes: c-entities).
    pub attach_offset_vector: f32,
    /// field_1c4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1c4: f32,
    /// field_1c8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1c8: f32,
    /// field_1cc (confidence: medium, kind: float, lanes: c-entities).
    pub field_1cc: f32,
    /// m_qAttachOffset (CQuaternion, iv-sdk 1.0.7.0) (confidence: medium, kind: u32, lanes: c-entities).
    pub attach_offset_quaternion: u32,
    /// field_1d4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1d4: u32,
    /// field_1d8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1d8: u32,
    /// field_1dc (confidence: low, kind: u32, lanes: c-entities).
    pub field_1dc: u32,
    /// field_1e0 (confidence: high, kind: int16?, lanes: c-entities).
    pub field_1e0: u16,
    /// Unknown bytes (0x1e2..0x1e4).
    pub _pad_01e2: [u8; 0x2],
    /// m_pLastDamageEntity (confidence: high, kind: pointer, lanes: c-entities,n-04,n-08,p0-360).
    pub last_damage_entity: Ptr32<u8>,
    /// Unknown bytes (0x1e8..0x1ec).
    pub _pad_01e8: [u8; 0x4],
    /// m_nLastDamageWeapon (s32, iv-sdk 1.0.7.0) (confidence: medium, kind: bool-or-byte, lanes: c-entities,n-02,n-18).
    pub last_damage_weapon: u8,
    /// Unknown bytes (0x1ed..0x1f0).
    pub _pad_01ed: [u8; 0x3],
    /// m_fHealth (f32, iv-sdk 1.0.7.0) (confidence: high, kind: float, lanes: c-entities,p0-360).
    pub health: f32,
    /// field_1f4 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1f4: u8,
    /// Unknown bytes (0x1f5..0x1f8).
    pub _pad_01f5: [u8; 0x3],
    /// m_pEntityIgnoredCollision (confidence: medium, kind: pointer, lanes: c-entities,p0-360).
    pub entity_ignored_collision: Ptr32<u8>,
    /// Unknown bytes (0x1fc..0x200).
    pub _pad_01fc: [u8; 0x4],
    /// field_200 (confidence: high, kind: u32, lanes: c-entities).
    pub field_200: u32,
    /// field_204 (confidence: high, kind: float, lanes: c-entities).
    pub field_204: f32,
    /// field_208 (confidence: high, kind: float, lanes: c-entities).
    pub field_208: f32,
    /// field_20c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_20c: u32,
    /// Unknown bytes (0x210..0x260).
    pub _pad_0210: [u8; 0x50],
    /// dies_injured (confidence: high, kind: flags, lanes: c-entities,n-07,n-11,n-16,n-18,via:CObject,via:CPed moved from siblings:CObject,CPed).
    pub dies_injured: u32,
    /// flags (confidence: high, kind: flags, lanes: c-entities,n-02,n-04,n-06,n-08,n-14,n-16,n-18,via:CObject,via:CPed moved from siblings:CObject,CPed).
    pub flags: u32,
    /// Unknown bytes (0x268..0x26c).
    pub _pad_0268: [u8; 0x4],
    /// flags (confidence: high, kind: flags, lanes: c-entities,n-05,n-06,n-07,n-08,n-10,n-11,n-14,n-16,n-20,via:CObject,via:CPed moved from siblings:CObject,CPed).
    pub flags_2: u32,
    /// Unknown bytes (0x270..0x29c).
    pub _pad_0270: [u8; 0x2c],
    /// x8000000_ped_shooting (confidence: high, kind: u32, lanes: c-entities,c-misc-a,n-02,n-10,via:CCutsceneObject,via:CPed moved from siblings:CObject,CPed).
    pub x8000000_ped_shooting: u32,
    /// Unknown bytes (0x2a0..0x2c4).
    pub _pad_02a0: [u8; 0x24],
    /// held_object (confidence: high, kind: u32, lanes: c-entities,c-misc-a,n-05,n-07,n-10,n-16,via:CCutsceneObject,via:CPed moved from siblings:CObject,CPed).
    pub held_object: u32,
    /// Unknown bytes (0x2c8..0xe48).
    pub _pad_02c8: [u8; 0xb80],
    /// helmet_related (confidence: high, kind: u32, lanes: c-entities,n-08,via:CHeli,via:CPed moved from siblings:CPed,CVehicle).
    pub helmet_related: u32,
    /// Unknown bytes (0xe4c..0xea4).
    pub _pad_0e4c: [u8; 0x58],
    /// field_ea4 (confidence: medium, kind: u32, lanes: c-entities,via:CPlayerPed,via:CVehicle moved from siblings:CPed,CVehicle).
    pub field_ea4: u32,
}
assert_size!(CPhysical, 0xea8); // merged size 0xea8 rounded to 4
assert_offset!(CPhysical, field_2c, 0x2c);
assert_offset!(CPhysical, field_41, 0x41);
assert_offset!(CPhysical, field_110, 0x110);
assert_offset!(CPhysical, field_11c, 0x11c);
assert_offset!(CPhysical, percent_submerged, 0x120);
assert_offset!(CPhysical, field_15c, 0x15c);
assert_offset!(CPhysical, field_160, 0x160);
assert_offset!(CPhysical, field_170, 0x170);
assert_offset!(CPhysical, field_174, 0x174);
assert_offset!(CPhysical, field_190, 0x190);
assert_offset!(CPhysical, field_1a0, 0x1a0);
assert_offset!(CPhysical, field_1a4, 0x1a4);
assert_offset!(CPhysical, field_1a8, 0x1a8);
assert_offset!(CPhysical, field_1ac, 0x1ac);
assert_offset!(CPhysical, attach_offset_vector, 0x1c0);
assert_offset!(CPhysical, field_1c4, 0x1c4);
assert_offset!(CPhysical, field_1c8, 0x1c8);
assert_offset!(CPhysical, field_1cc, 0x1cc);
assert_offset!(CPhysical, attach_offset_quaternion, 0x1d0);
assert_offset!(CPhysical, field_1d4, 0x1d4);
assert_offset!(CPhysical, field_1d8, 0x1d8);
assert_offset!(CPhysical, field_1dc, 0x1dc);
assert_offset!(CPhysical, field_1e0, 0x1e0);
assert_offset!(CPhysical, last_damage_entity, 0x1e4);
assert_offset!(CPhysical, last_damage_weapon, 0x1ec);
assert_offset!(CPhysical, health, 0x1f0);
assert_offset!(CPhysical, field_1f4, 0x1f4);
assert_offset!(CPhysical, entity_ignored_collision, 0x1f8);
assert_offset!(CPhysical, field_200, 0x200);
assert_offset!(CPhysical, field_204, 0x204);
assert_offset!(CPhysical, field_208, 0x208);
assert_offset!(CPhysical, field_20c, 0x20c);
assert_offset!(CPhysical, dies_injured, 0x260);
assert_offset!(CPhysical, flags, 0x264);
assert_offset!(CPhysical, flags_2, 0x26c);
assert_offset!(CPhysical, x8000000_ped_shooting, 0x29c);
assert_offset!(CPhysical, held_object, 0x2c4);
assert_offset!(CPhysical, helmet_related, 0xe48);
assert_offset!(CPhysical, field_ea4, 0xea4);

/// Merged layout for `CPickup`.
///
/// Size: 0x104 (low). Bases: none.
/// Lanes: n-07, n-10, n-14, n-18.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPickup {
    /// position_entries (confidence: high, kind: vec3, lanes: n-07,n-14).
    pub position_entries: [f32; 3],
    /// Unknown bytes (0xc..0x28).
    pub _pad_000c: [u8; 0x1c],
    /// pickup_type (confidence: medium, kind: u8, lanes: n-10).
    pub pickup_type: u8,
    /// Unknown bytes (0x29..0x7c).
    pub _pad_0029: [u8; 0x53],
    /// network_regen_timer (confidence: medium, kind: u32, lanes: n-14).
    pub network_regen_timer: u32,
    /// Unknown bytes (0x80..0x100).
    pub _pad_0080: [u8; 0x80],
    /// local_player_weapon_stats (confidence: low, kind: i32, lanes: n-14).
    pub local_player_weapon_stats: u32,
}
assert_size!(CPickup, 0x104); // merged size 0x104 rounded to 4
assert_offset!(CPickup, position_entries, 0x0);
assert_offset!(CPickup, pickup_type, 0x28);
assert_offset!(CPickup, network_regen_timer, 0x7c);
assert_offset!(CPickup, local_player_weapon_stats, 0x100);

/// Merged layout for `CPortalInst`.
///
/// Size: 0xc8 (low). Bases: CBuilding@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPortalInst {
    /// Unknown bytes (0x0..0x80).
    pub _pad_0000: [u8; 0x80],
    /// field_80 (confidence: high, kind: float, lanes: c-entities).
    pub field_80: f32,
    /// field_84 (confidence: high, kind: float, lanes: c-entities).
    pub field_84: f32,
    /// Unknown bytes (0x88..0x8c).
    pub _pad_0088: [u8; 0x4],
    /// field_8c (confidence: high, kind: float, lanes: c-entities).
    pub field_8c: f32,
    /// field_90 (confidence: high, kind: float, lanes: c-entities).
    pub field_90: f32,
    /// Unknown bytes (0x94..0x9c).
    pub _pad_0094: [u8; 0x8],
    /// field_9c (confidence: high, kind: float, lanes: c-entities).
    pub field_9c: f32,
    /// field_a0 (confidence: high, kind: float, lanes: c-entities).
    pub field_a0: f32,
    /// field_a4 (confidence: high, kind: float, lanes: c-entities).
    pub field_a4: f32,
    /// field_a8 (confidence: high, kind: float, lanes: c-entities).
    pub field_a8: f32,
    /// field_ac (confidence: high, kind: float, lanes: c-entities).
    pub field_ac: f32,
    /// Unknown bytes (0xb0..0xb4).
    pub _pad_00b0: [u8; 0x4],
    /// field_b4 (confidence: high, kind: u32, lanes: c-entities).
    pub field_b4: u32,
    /// Unknown bytes (0xb8..0xc0).
    pub _pad_00b8: [u8; 0x8],
    /// field_c0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_c0: u32,
    /// field_c4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_c4: u32,
}
assert_size!(CPortalInst, 0xc8); // merged size 0xc8 rounded to 4
assert_offset!(CPortalInst, field_80, 0x80);
assert_offset!(CPortalInst, field_84, 0x84);
assert_offset!(CPortalInst, field_8c, 0x8c);
assert_offset!(CPortalInst, field_90, 0x90);
assert_offset!(CPortalInst, field_9c, 0x9c);
assert_offset!(CPortalInst, field_a0, 0xa0);
assert_offset!(CPortalInst, field_a4, 0xa4);
assert_offset!(CPortalInst, field_a8, 0xa8);
assert_offset!(CPortalInst, field_ac, 0xac);
assert_offset!(CPortalInst, field_b4, 0xb4);
assert_offset!(CPortalInst, field_c0, 0xc0);
assert_offset!(CPortalInst, field_c4, 0xc4);

/// Merged layout for `CPortalTracker`.
///
/// Size: 0x48 (low). Bases: none.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPortalTracker {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: float, lanes: c-entities).
    pub field_14: f32,
    /// field_18 (confidence: high, kind: float, lanes: c-entities).
    pub field_18: f32,
    /// field_1c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: float, lanes: c-entities).
    pub field_20: f32,
    /// field_24 (confidence: high, kind: float, lanes: c-entities).
    pub field_24: f32,
    /// field_28 (confidence: high, kind: float, lanes: c-entities).
    pub field_28: f32,
    /// field_2c (confidence: high, kind: float, lanes: c-entities).
    pub field_2c: f32,
    /// field_30 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_30: u32,
    /// field_34 (confidence: high, kind: u32, lanes: c-entities).
    pub field_34: u32,
    /// field_38 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_38: u32,
    /// field_3c (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_3c: u8,
    /// Unknown bytes (0x3d..0x3e).
    pub _pad_003d: [u8; 0x1],
    /// field_3e (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_3e: u8,
    /// field_3f (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_3f: u8,
    /// field_40 (confidence: low, kind: u32, lanes: c-entities).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_44: u8,
    /// field_45 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_45: u8,
    /// field_46 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_46: u16,
}
assert_size!(CPortalTracker, 0x48); // merged size 0x48 rounded to 4
assert_offset!(CPortalTracker, vfptr, 0x0);
assert_offset!(CPortalTracker, field_10, 0x10);
assert_offset!(CPortalTracker, field_14, 0x14);
assert_offset!(CPortalTracker, field_18, 0x18);
assert_offset!(CPortalTracker, field_1c, 0x1c);
assert_offset!(CPortalTracker, field_20, 0x20);
assert_offset!(CPortalTracker, field_24, 0x24);
assert_offset!(CPortalTracker, field_28, 0x28);
assert_offset!(CPortalTracker, field_2c, 0x2c);
assert_offset!(CPortalTracker, field_30, 0x30);
assert_offset!(CPortalTracker, field_34, 0x34);
assert_offset!(CPortalTracker, field_38, 0x38);
assert_offset!(CPortalTracker, field_3c, 0x3c);
assert_offset!(CPortalTracker, field_3e, 0x3e);
assert_offset!(CPortalTracker, field_3f, 0x3f);
assert_offset!(CPortalTracker, field_40, 0x40);
assert_offset!(CPortalTracker, field_44, 0x44);
assert_offset!(CPortalTracker, field_45, 0x45);
assert_offset!(CPortalTracker, field_46, 0x46);

/// Merged layout for `PickupPool`.
///
/// Size: 0x31 (low). Bases: none.
/// Lanes: n-13.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PickupPool {
    /// pickup_timer_decremented_dt (confidence: high, kind: u32, lanes: n-13).
    pub pickup_timer_decremented_dt: u32,
    /// Unknown bytes (0x4..0x14).
    pub _pad_0004: [u8; 0x10],
    /// pickup_world_position_pigeon (confidence: medium, kind: u32, lanes: n-13).
    pub pickup_world_position_pigeon: u32,
    /// pickup_world_position_pigeon (confidence: medium, kind: u32, lanes: n-13).
    pub pickup_world_position_pigeon_2: u32,
    /// pickup_world_position_pigeon (confidence: medium, kind: u32, lanes: n-13).
    pub pickup_world_position_pigeon_3: u32,
    /// Unknown bytes (0x20..0x24).
    pub _pad_0020: [u8; 0x4],
    /// pickup_model_id_matched (confidence: medium, kind: u16, lanes: n-13).
    pub pickup_model_id_matched: u16,
    /// Unknown bytes (0x26..0x28).
    pub _pad_0026: [u8; 0x2],
    /// pickup_live (confidence: medium, kind: flags, lanes: n-13).
    pub pickup_live: u8,
    /// Unknown bytes (0x29..0x30).
    pub _pad_0029: [u8; 0x7],
    /// pickup_state (confidence: medium, kind: u8, lanes: n-13).
    pub pickup_state: u8,
    /// Unknown trailing bytes (0x31..0x34).
    pub _pad_end: [u8; 0x3],
}
assert_size!(PickupPool, 0x34); // merged size 0x31 rounded to 4
assert_offset!(PickupPool, pickup_timer_decremented_dt, 0x0);
assert_offset!(PickupPool, pickup_world_position_pigeon, 0x14);
assert_offset!(PickupPool, pickup_world_position_pigeon_2, 0x18);
assert_offset!(PickupPool, pickup_world_position_pigeon_3, 0x1c);
assert_offset!(PickupPool, pickup_model_id_matched, 0x24);
assert_offset!(PickupPool, pickup_live, 0x28);
assert_offset!(PickupPool, pickup_state, 0x30);
