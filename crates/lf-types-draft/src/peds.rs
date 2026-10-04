//! Peds and players.
//!
//! Holds 16 draft layouts: `CPed`, `CPlayerPed`, `CDummyPed`, the `CPed*` helper classes
//! (intelligence, formations, attractors, damage response, targeting, model info), `CTargetting`,
//! and the player records `CPlayer`, `CPlayerInfo` and `CWantedChain`. Every layout is Inferred;
//! size confidence (the analysis lanes' own rating) is high for 1, medium for 0 and low for 15. The
//! conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CDummyPed`.
///
/// Size: 0x3b0 (low). Bases: CDynamicEntity@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CDummyPed {
    /// Unknown bytes (0x0..0x110).
    pub _pad_0000: [u8; 0x110],
    /// field_110 (confidence: high, kind: pointer, lanes: c-entities).
    pub field_110: Ptr32<u8>,
    /// field_114 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_114: u8,
    /// Unknown bytes (0x115..0x11c).
    pub _pad_0115: [u8; 0x7],
    /// field_11c (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_11c: u8,
    /// Unknown bytes (0x11d..0x120).
    pub _pad_011d: [u8; 0x3],
    /// field_120 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_120: Ptr32<u8>,
    /// Unknown bytes (0x124..0x138).
    pub _pad_0124: [u8; 0x14],
    /// field_138 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_138: Ptr32<u8>,
    /// Unknown bytes (0x13c..0x14c).
    pub _pad_013c: [u8; 0x10],
    /// field_14c (confidence: low, kind: u32, lanes: c-entities).
    pub field_14c: u32,
    /// Unknown bytes (0x150..0x390).
    pub _pad_0150: [u8; 0x240],
    /// field_390 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_390: u8,
    /// field_391 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_391: u8,
    /// Unknown bytes (0x392..0x394).
    pub _pad_0392: [u8; 0x2],
    /// field_394 (confidence: high, kind: u32, lanes: c-entities).
    pub field_394: u32,
    /// field_398 (confidence: high, kind: u32, lanes: c-entities).
    pub field_398: u32,
    /// field_39c (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_39c: u8,
    /// field_39d (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_39d: u8,
    /// Unknown bytes (0x39e..0x3a0).
    pub _pad_039e: [u8; 0x2],
    /// field_3a0 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_3a0: u8,
    /// field_3a1 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_3a1: [u8; 4],
    /// Unknown bytes (0x3a5..0x3a8).
    pub _pad_03a5: [u8; 0x3],
    /// field_3a8 (confidence: high, kind: u32, lanes: c-entities).
    pub field_3a8: u32,
    /// field_3ac (confidence: high, kind: float, lanes: c-entities).
    pub field_3ac: f32,
}
assert_size!(CDummyPed, 0x3b0); // merged size 0x3b0 rounded to 4
assert_offset!(CDummyPed, field_110, 0x110);
assert_offset!(CDummyPed, field_114, 0x114);
assert_offset!(CDummyPed, field_11c, 0x11c);
assert_offset!(CDummyPed, field_120, 0x120);
assert_offset!(CDummyPed, field_138, 0x138);
assert_offset!(CDummyPed, field_14c, 0x14c);
assert_offset!(CDummyPed, field_390, 0x390);
assert_offset!(CDummyPed, field_391, 0x391);
assert_offset!(CDummyPed, field_394, 0x394);
assert_offset!(CDummyPed, field_398, 0x398);
assert_offset!(CDummyPed, field_39c, 0x39c);
assert_offset!(CDummyPed, field_39d, 0x39d);
assert_offset!(CDummyPed, field_3a0, 0x3a0);
assert_offset!(CDummyPed, field_3a1, 0x3a1);
assert_offset!(CDummyPed, field_3a8, 0x3a8);
assert_offset!(CDummyPed, field_3ac, 0x3ac);

/// Merged layout for `CPed`.
///
/// Size: 0xee0 (low). Bases: CPhysical@0x0.
/// Lanes: c-entities, n-01, n-02, n-04, n-05, n-06, n-07, n-08, n-10, n-11, n-12, n-13, n-14, n-15, n-16, n-18, n-20, n-22, p0-360.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPed {
    /// Unknown bytes (0x0..0x214).
    pub _pad_0000: [u8; 0x214],
    /// m_fPedHealth (confidence: medium, kind: float, lanes: c-entities,p0-360).
    pub ped_health: f32,
    /// m_nPlayerIndex (u8, iv-sdk 1.0.7.0) (confidence: high, kind: u8, lanes: c-entities,n-02,n-16,p0-360).
    pub player_index: u8,
    /// m_bIsPlayer (u8, iv-sdk 1.0.7.0) (confidence: high, kind: float, lanes: c-entities,n-01,n-02,n-10,n-16,p0-360).
    pub is_player: u8,
    /// Unknown bytes (0x21a..0x21c).
    pub _pad_021a: [u8; 0x2],
    /// ped_data_model_extension (confidence: high, kind: u32, lanes: c-entities,n-06,n-07,n-14,n-16,p0-360).
    pub ped_data_model_extension: u32,
    /// Unknown bytes (0x220..0x224).
    pub _pad_0220: [u8; 0x4],
    /// ped_intelligence (confidence: high, kind: pointer, lanes: c-entities,n-02,n-04,n-05,n-06,n-07,n-08,n-10,n-11,n-14,n-16,n-18,p0-360).
    pub ped_intelligence: Ptr32<u8>,
    /// Unknown bytes (0x228..0x230).
    pub _pad_0228: [u8; 0x8],
    /// field_230 (confidence: high, kind: float, lanes: c-entities).
    pub field_230: f32,
    /// field_234 (confidence: high, kind: float, lanes: c-entities).
    pub field_234: f32,
    /// field_238 (confidence: high, kind: u32, lanes: c-entities).
    pub field_238: u32,
    /// Unknown bytes (0x23c..0x268).
    pub _pad_023c: [u8; 0x2c],
    /// visualise_head_damage_bullets (confidence: high, kind: u32, lanes: n-05,n-06,n-11,n-16,n-18).
    pub visualise_head_damage_bullets: u32,
    /// Unknown bytes (0x26c..0x27c).
    pub _pad_026c: [u8; 0x10],
    /// field_27c (confidence: low, kind: u32, lanes: c-entities).
    pub field_27c: u32,
    /// Unknown bytes (0x280..0x2b0).
    pub _pad_0280: [u8; 0x30],
    /// weapon_passed_weapon_helpers (confidence: high, kind: u32, lanes: c-entities,n-05,n-08,n-16).
    pub weapon_passed_weapon_helpers: u32,
    /// Unknown bytes (0x2b4..0x368).
    pub _pad_02b4: [u8; 0xb4],
    /// field_368 (confidence: low, kind: u32, lanes: c-entities).
    pub field_368: u32,
    /// Unknown bytes (0x36c..0x370).
    pub _pad_036c: [u8; 0x4],
    /// field_370 (confidence: medium, kind: u8, lanes: n-16).
    pub field_370: u8,
    /// field_371 (confidence: medium, kind: u8, lanes: n-16).
    pub field_371: u8,
    /// Unknown bytes (0x372..0x398).
    pub _pad_0372: [u8; 0x26],
    /// current_target_entity (confidence: high, kind: pointer, lanes: c-entities,n-11).
    pub current_target_entity: Ptr32<u8>,
    /// Unknown bytes (0x39c..0x3c0).
    pub _pad_039c: [u8; 0x24],
    /// audio_entity_base (confidence: high, kind: u32, lanes: c-entities,n-08).
    pub audio_entity_base: u32,
    /// Unknown bytes (0x3c4..0x570).
    pub _pad_03c4: [u8; 0x1ac],
    /// voice_speech_entity_scream (confidence: high, kind: embedded-object, lanes: c-entities,n-05,n-13).
    pub voice_speech_entity_scream: u32,
    /// Unknown bytes (0x574..0x640).
    pub _pad_0574: [u8; 0xcc],
    /// field_640 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_640: u32,
    /// Unknown bytes (0x644..0x690).
    pub _pad_0644: [u8; 0x4c],
    /// field_690 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_690: u32,
    /// Unknown bytes (0x694..0x6e0).
    pub _pad_0694: [u8; 0x4c],
    /// field_6e0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_6e0: u32,
    /// Unknown bytes (0x6e4..0x6fc).
    pub _pad_06e4: [u8; 0x18],
    /// field_6fc (confidence: medium, kind: u32, lanes: c-entities).
    pub field_6fc: u32,
    /// Unknown bytes (0x700..0x770).
    pub _pad_0700: [u8; 0x70],
    /// drunk (confidence: high, kind: u8, lanes: n-18).
    pub drunk: u8,
    /// blind_raging (confidence: high, kind: u8, lanes: n-18).
    pub blind_raging: u8,
    /// Unknown bytes (0x772..0x780).
    pub _pad_0772: [u8; 0xe],
    /// field_780 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_780: u32,
    /// Unknown bytes (0x784..0x788).
    pub _pad_0784: [u8; 0x4],
    /// field_788 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_788: u32,
    /// Unknown bytes (0x78c..0x798).
    pub _pad_078c: [u8; 0xc],
    /// field_798 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_798: u32,
    /// Unknown bytes (0x79c..0x7ac).
    pub _pad_079c: [u8; 0x10],
    /// field_7ac (confidence: high, kind: u32, lanes: c-entities).
    pub field_7ac: u32,
    /// field_7b0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_7b0: u32,
    /// euphoria_behaviour (confidence: high, kind: u32, lanes: c-entities,n-01,n-11,n-16,n-18).
    pub euphoria_behaviour: u32,
    /// physics_state (confidence: high, kind: u32, lanes: c-entities,n-11).
    pub physics_state: u32,
    /// Unknown bytes (0x7bc..0x7c0).
    pub _pad_07bc: [u8; 0x4],
    /// field_7c0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_7c0: u32,
    /// Unknown bytes (0x7c4..0x820).
    pub _pad_07c4: [u8; 0x5c],
    /// model_change_state_ped (confidence: low, kind: flags, lanes: n-02).
    pub model_change_state_ped: u8,
    /// Unknown bytes (0x821..0xa60).
    pub _pad_0821: [u8; 0x23f],
    /// mission_entity_marker (confidence: medium, kind: u8, lanes: n-10,n-12,n-14,n-16).
    pub mission_entity_marker: u8,
    /// Unknown bytes (0xa61..0xa74).
    pub _pad_0a61: [u8; 0x13],
    /// state_compared (confidence: high, kind: u32, lanes: c-entities,n-02,n-07,n-10,n-14).
    pub state_compared: u32,
    /// last_damage_bone_id (confidence: high, kind: u32, lanes: n-02,n-05).
    pub last_damage_bone_id: u32,
    /// Unknown bytes (0xa7c..0xa80).
    pub _pad_0a7c: [u8; 0x4],
    /// ptr_anim_move_state (confidence: high, kind: float, lanes: c-entities,n-02,n-05,n-06,n-16).
    pub ptr_anim_move_state: f32,
    /// full_health_revive (confidence: high, kind: float, lanes: c-entities,n-15,n-16).
    pub full_health_revive: f32,
    /// Unknown bytes (0xa88..0xaa0).
    pub _pad_0a88: [u8; 0x18],
    /// heading_radians_copy_written (confidence: high, kind: float, lanes: c-entities,n-14,n-16).
    pub heading_radians_copy_written: f32,
    /// heading_radians_copy_written (confidence: high, kind: float, lanes: c-entities,n-14,n-16).
    pub heading_radians_copy_written_2: f32,
    /// Unknown bytes (0xaa8..0xab0).
    pub _pad_0aa8: [u8; 0x8],
    /// field_ab0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ab0: u32,
    /// touched_attached_object_fallback (confidence: low, kind: pointer, lanes: n-10).
    pub touched_attached_object_fallback: Ptr32<u8>,
    /// Unknown bytes (0xab8..0xad0).
    pub _pad_0ab8: [u8; 0x18],
    /// field_ad0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ad0: u32,
    /// field_ad4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ad4: u32,
    /// field_ad8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ad8: u32,
    /// Unknown bytes (0xadc..0xae0).
    pub _pad_0adc: [u8; 0x4],
    /// field_ae0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ae0: u32,
    /// field_ae4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ae4: u32,
    /// field_ae8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ae8: u32,
    /// Unknown bytes (0xaec..0xaf0).
    pub _pad_0aec: [u8; 0x4],
    /// field_af0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_af0: u32,
    /// field_af4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_af4: u32,
    /// field_af8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_af8: u32,
    /// Unknown bytes (0xafc..0xb14).
    pub _pad_0afc: [u8; 0x18],
    /// float_network_resurrect (confidence: low, kind: float, lanes: n-14).
    pub float_network_resurrect: f32,
    /// field_b18 (confidence: low, kind: u32, lanes: c-entities).
    pub field_b18: u32,
    /// Unknown bytes (0xb1c..0xb28).
    pub _pad_0b1c: [u8; 0xc],
    /// field_b28 (confidence: high, kind: u32, lanes: c-entities,n-16).
    pub field_b28: u32,
    /// field_b2c (confidence: medium, kind: u32, lanes: n-16).
    pub field_b2c: u32,
    /// current_vehicle (confidence: high, kind: pointer, lanes: c-entities,n-05,n-06,n-08,n-10,n-11,n-14,n-16,n-20).
    pub current_vehicle: Ptr32<u8>,
    /// field_b34 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_b34: u32,
    /// Unknown bytes (0xb38..0xb84).
    pub _pad_0b38: [u8; 0x4c],
    /// armour_float_truncated_int (confidence: high, kind: float, lanes: n-01,n-05).
    pub armour_float_truncated_int: f32,
    /// money (confidence: high, kind: u32, lanes: n-06).
    pub money: u32,
    /// Unknown bytes (0xb8c..0xb90).
    pub _pad_0b8c: [u8; 0x4],
    /// field_b90 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_b90: Ptr32<u8>,
    /// anim_group_id (confidence: high, kind: u32, lanes: c-entities,n-05,n-15).
    pub anim_group_id: u32,
    /// field_b98 (confidence: medium, kind: u32, lanes: n-16).
    pub field_b98: u32,
    /// Unknown bytes (0xb9c..0xbb0).
    pub _pad_0b9c: [u8; 0x14],
    /// field_bb0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_bb0: u32,
    /// Unknown bytes (0xbb4..0xbe0).
    pub _pad_0bb4: [u8; 0x2c],
    /// field_be0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_be0: u32,
    /// Unknown bytes (0xbe4..0xbf8).
    pub _pad_0be4: [u8; 0x14],
    /// field_bf8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_bf8: f32,
    /// Unknown bytes (0xbfc..0xd32).
    pub _pad_0bfc: [u8; 0x136],
    /// group_index (confidence: high, kind: i32, lanes: n-11).
    pub group_index: u8,
    /// field_d33 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_d33: u8,
    /// field_d34 (confidence: medium, kind: u32, lanes: n-16).
    pub field_d34: u32,
    /// Unknown bytes (0xd38..0xd50).
    pub _pad_0d38: [u8; 0x18],
    /// field_d50 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d50: u32,
    /// field_d54 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d54: u32,
    /// field_d58 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_d58: u32,
    /// Unknown bytes (0xd5c..0xd84).
    pub _pad_0d5c: [u8; 0x28],
    /// field_d84 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_d84: u16,
    /// Unknown bytes (0xd86..0xd88).
    pub _pad_0d86: [u8; 0x2],
    /// field_d88 (confidence: high, kind: u32, lanes: c-entities).
    pub field_d88: u32,
    /// Unknown bytes (0xd8c..0xe04).
    pub _pad_0d8c: [u8; 0x78],
    /// field_e04 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e04: u32,
    /// field_e08 (confidence: high, kind: float, lanes: c-entities).
    pub field_e08: f32,
    /// windy_clothing_scale (confidence: high, kind: u32, lanes: c-entities,n-18).
    pub windy_clothing_scale: u32,
    /// field_e10 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_e10: u8,
    /// Unknown bytes (0xe11..0xe20).
    pub _pad_0e11: [u8; 0xf],
    /// field_e20 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e20: u32,
    /// field_e24 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e24: u32,
    /// field_e28 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e28: u32,
    /// Unknown bytes (0xe2c..0xe30).
    pub _pad_0e2c: [u8; 0x4],
    /// field_e30 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e30: u32,
    /// field_e34 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e34: u32,
    /// field_e38 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e38: u32,
    /// Unknown bytes (0xe3c..0xe40).
    pub _pad_0e3c: [u8; 0x4],
    /// field_e40 (confidence: high, kind: float, lanes: c-entities).
    pub field_e40: f32,
    /// helmet_texture_index (confidence: high, kind: i32, lanes: c-entities,n-18).
    pub helmet_texture_index: u32,
    /// Unknown bytes (0xe48..0xe4c).
    pub _pad_0e48: [u8; 0x4],
    /// helmet_related (confidence: medium, kind: u32, lanes: c-entities,n-08).
    pub helmet_related: u32,
    /// field_e50 (confidence: high, kind: float, lanes: c-entities).
    pub field_e50: f32,
    /// Unknown bytes (0xe54..0xe97).
    pub _pad_0e54: [u8; 0x43],
    /// cop (confidence: high, kind: flags, lanes: c-entities,n-18).
    pub cop: u8,
    /// field_e98 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_e98: u32,
    /// Unknown bytes (0xe9c..0xedc).
    pub _pad_0e9c: [u8; 0x40],
    /// field_edc (confidence: high, kind: u32, lanes: c-entities,n-16).
    pub field_edc: u32,
}
assert_size!(CPed, 0xee0); // merged size 0xee0 rounded to 4
assert_offset!(CPed, ped_health, 0x214);
assert_offset!(CPed, player_index, 0x218);
assert_offset!(CPed, is_player, 0x219);
assert_offset!(CPed, ped_data_model_extension, 0x21c);
assert_offset!(CPed, ped_intelligence, 0x224);
assert_offset!(CPed, field_230, 0x230);
assert_offset!(CPed, field_234, 0x234);
assert_offset!(CPed, field_238, 0x238);
assert_offset!(CPed, visualise_head_damage_bullets, 0x268);
assert_offset!(CPed, field_27c, 0x27c);
assert_offset!(CPed, weapon_passed_weapon_helpers, 0x2b0);
assert_offset!(CPed, field_368, 0x368);
assert_offset!(CPed, field_370, 0x370);
assert_offset!(CPed, field_371, 0x371);
assert_offset!(CPed, current_target_entity, 0x398);
assert_offset!(CPed, audio_entity_base, 0x3c0);
assert_offset!(CPed, voice_speech_entity_scream, 0x570);
assert_offset!(CPed, field_640, 0x640);
assert_offset!(CPed, field_690, 0x690);
assert_offset!(CPed, field_6e0, 0x6e0);
assert_offset!(CPed, field_6fc, 0x6fc);
assert_offset!(CPed, drunk, 0x770);
assert_offset!(CPed, blind_raging, 0x771);
assert_offset!(CPed, field_780, 0x780);
assert_offset!(CPed, field_788, 0x788);
assert_offset!(CPed, field_798, 0x798);
assert_offset!(CPed, field_7ac, 0x7ac);
assert_offset!(CPed, field_7b0, 0x7b0);
assert_offset!(CPed, euphoria_behaviour, 0x7b4);
assert_offset!(CPed, physics_state, 0x7b8);
assert_offset!(CPed, field_7c0, 0x7c0);
assert_offset!(CPed, model_change_state_ped, 0x820);
assert_offset!(CPed, mission_entity_marker, 0xa60);
assert_offset!(CPed, state_compared, 0xa74);
assert_offset!(CPed, last_damage_bone_id, 0xa78);
assert_offset!(CPed, ptr_anim_move_state, 0xa80);
assert_offset!(CPed, full_health_revive, 0xa84);
assert_offset!(CPed, heading_radians_copy_written, 0xaa0);
assert_offset!(CPed, heading_radians_copy_written_2, 0xaa4);
assert_offset!(CPed, field_ab0, 0xab0);
assert_offset!(CPed, touched_attached_object_fallback, 0xab4);
assert_offset!(CPed, field_ad0, 0xad0);
assert_offset!(CPed, field_ad4, 0xad4);
assert_offset!(CPed, field_ad8, 0xad8);
assert_offset!(CPed, field_ae0, 0xae0);
assert_offset!(CPed, field_ae4, 0xae4);
assert_offset!(CPed, field_ae8, 0xae8);
assert_offset!(CPed, field_af0, 0xaf0);
assert_offset!(CPed, field_af4, 0xaf4);
assert_offset!(CPed, field_af8, 0xaf8);
assert_offset!(CPed, float_network_resurrect, 0xb14);
assert_offset!(CPed, field_b18, 0xb18);
assert_offset!(CPed, field_b28, 0xb28);
assert_offset!(CPed, field_b2c, 0xb2c);
assert_offset!(CPed, current_vehicle, 0xb30);
assert_offset!(CPed, field_b34, 0xb34);
assert_offset!(CPed, armour_float_truncated_int, 0xb84);
assert_offset!(CPed, money, 0xb88);
assert_offset!(CPed, field_b90, 0xb90);
assert_offset!(CPed, anim_group_id, 0xb94);
assert_offset!(CPed, field_b98, 0xb98);
assert_offset!(CPed, field_bb0, 0xbb0);
assert_offset!(CPed, field_be0, 0xbe0);
assert_offset!(CPed, field_bf8, 0xbf8);
assert_offset!(CPed, group_index, 0xd32);
assert_offset!(CPed, field_d33, 0xd33);
assert_offset!(CPed, field_d34, 0xd34);
assert_offset!(CPed, field_d50, 0xd50);
assert_offset!(CPed, field_d54, 0xd54);
assert_offset!(CPed, field_d58, 0xd58);
assert_offset!(CPed, field_d84, 0xd84);
assert_offset!(CPed, field_d88, 0xd88);
assert_offset!(CPed, field_e04, 0xe04);
assert_offset!(CPed, field_e08, 0xe08);
assert_offset!(CPed, windy_clothing_scale, 0xe0c);
assert_offset!(CPed, field_e10, 0xe10);
assert_offset!(CPed, field_e20, 0xe20);
assert_offset!(CPed, field_e24, 0xe24);
assert_offset!(CPed, field_e28, 0xe28);
assert_offset!(CPed, field_e30, 0xe30);
assert_offset!(CPed, field_e34, 0xe34);
assert_offset!(CPed, field_e38, 0xe38);
assert_offset!(CPed, field_e40, 0xe40);
assert_offset!(CPed, helmet_texture_index, 0xe44);
assert_offset!(CPed, helmet_related, 0xe4c);
assert_offset!(CPed, field_e50, 0xe50);
assert_offset!(CPed, cop, 0xe97);
assert_offset!(CPed, field_e98, 0xe98);
assert_offset!(CPed, field_edc, 0xedc);

/// Merged layout for `CPedAttractor`.
///
/// Size: 0x94 (low). Bases: none.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedAttractor {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: high, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: u32, lanes: c-entities).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: u32, lanes: c-entities).
    pub field_20: u32,
    /// field_24 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_24: u32,
    /// field_28 (confidence: low, kind: u32, lanes: c-entities).
    pub field_28: u32,
    /// field_2c (confidence: low, kind: u32, lanes: c-entities).
    pub field_2c: u32,
    /// field_30 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32, lanes: c-entities).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: u32, lanes: c-entities).
    pub field_38: u32,
    /// field_3c (confidence: low, kind: u32, lanes: c-entities).
    pub field_3c: u32,
    /// field_40 (confidence: high, kind: float, lanes: c-entities).
    pub field_40: f32,
    /// field_44 (confidence: medium, kind: float, lanes: c-entities).
    pub field_44: f32,
    /// field_48 (confidence: medium, kind: float, lanes: c-entities).
    pub field_48: f32,
    /// field_4c (confidence: medium, kind: float, lanes: c-entities).
    pub field_4c: f32,
    /// field_50 (confidence: medium, kind: float, lanes: c-entities).
    pub field_50: f32,
    /// field_54 (confidence: medium, kind: float, lanes: c-entities).
    pub field_54: f32,
    /// field_58 (confidence: medium, kind: float, lanes: c-entities).
    pub field_58: f32,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// field_60 (confidence: high, kind: float, lanes: c-entities).
    pub field_60: f32,
    /// field_64 (confidence: medium, kind: float, lanes: c-entities).
    pub field_64: f32,
    /// field_68 (confidence: medium, kind: float, lanes: c-entities).
    pub field_68: f32,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: high, kind: float, lanes: c-entities).
    pub field_70: f32,
    /// field_74 (confidence: high, kind: float, lanes: c-entities).
    pub field_74: f32,
    /// field_78 (confidence: medium, kind: float, lanes: c-entities).
    pub field_78: f32,
    /// field_7c (confidence: low, kind: u32, lanes: c-entities).
    pub field_7c: u32,
    /// field_80 (confidence: high, kind: float, lanes: c-entities).
    pub field_80: f32,
    /// field_84 (confidence: medium, kind: float, lanes: c-entities).
    pub field_84: f32,
    /// Unknown bytes (0x88..0x90).
    pub _pad_0088: [u8; 0x8],
    /// field_90 (confidence: low, kind: u32, lanes: c-entities).
    pub field_90: u32,
}
assert_size!(CPedAttractor, 0x94); // merged size 0x94 rounded to 4
assert_offset!(CPedAttractor, vfptr, 0x0);
assert_offset!(CPedAttractor, field_10, 0x10);
assert_offset!(CPedAttractor, field_14, 0x14);
assert_offset!(CPedAttractor, field_18, 0x18);
assert_offset!(CPedAttractor, field_1c, 0x1c);
assert_offset!(CPedAttractor, field_20, 0x20);
assert_offset!(CPedAttractor, field_24, 0x24);
assert_offset!(CPedAttractor, field_28, 0x28);
assert_offset!(CPedAttractor, field_2c, 0x2c);
assert_offset!(CPedAttractor, field_30, 0x30);
assert_offset!(CPedAttractor, field_34, 0x34);
assert_offset!(CPedAttractor, field_38, 0x38);
assert_offset!(CPedAttractor, field_3c, 0x3c);
assert_offset!(CPedAttractor, field_40, 0x40);
assert_offset!(CPedAttractor, field_44, 0x44);
assert_offset!(CPedAttractor, field_48, 0x48);
assert_offset!(CPedAttractor, field_4c, 0x4c);
assert_offset!(CPedAttractor, field_50, 0x50);
assert_offset!(CPedAttractor, field_54, 0x54);
assert_offset!(CPedAttractor, field_58, 0x58);
assert_offset!(CPedAttractor, field_60, 0x60);
assert_offset!(CPedAttractor, field_64, 0x64);
assert_offset!(CPedAttractor, field_68, 0x68);
assert_offset!(CPedAttractor, field_70, 0x70);
assert_offset!(CPedAttractor, field_74, 0x74);
assert_offset!(CPedAttractor, field_78, 0x78);
assert_offset!(CPedAttractor, field_7c, 0x7c);
assert_offset!(CPedAttractor, field_80, 0x80);
assert_offset!(CPedAttractor, field_84, 0x84);
assert_offset!(CPedAttractor, field_90, 0x90);

/// Merged layout for `CPedDamageResponse`.
///
/// Size: 0x39 (low). Bases: none.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedDamageResponse {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_4: u8,
    /// Unknown bytes (0x5..0x8).
    pub _pad_0005: [u8; 0x3],
    /// field_8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_8: u32,
    /// field_c (confidence: low, kind: u32, lanes: c-entities).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x18).
    pub _pad_0010: [u8; 0x8],
    /// field_18 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_18: u32,
    /// field_1c (confidence: low, kind: u32, lanes: c-entities).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: u32, lanes: c-entities).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_28: u8,
    /// Unknown bytes (0x29..0x34).
    pub _pad_0029: [u8; 0xb],
    /// field_34 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_38: u8,
    /// Unknown trailing bytes (0x39..0x3c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CPedDamageResponse, 0x3c); // merged size 0x39 rounded to 4
assert_offset!(CPedDamageResponse, vfptr, 0x0);
assert_offset!(CPedDamageResponse, field_4, 0x4);
assert_offset!(CPedDamageResponse, field_8, 0x8);
assert_offset!(CPedDamageResponse, field_c, 0xc);
assert_offset!(CPedDamageResponse, field_18, 0x18);
assert_offset!(CPedDamageResponse, field_1c, 0x1c);
assert_offset!(CPedDamageResponse, field_20, 0x20);
assert_offset!(CPedDamageResponse, field_24, 0x24);
assert_offset!(CPedDamageResponse, field_28, 0x28);
assert_offset!(CPedDamageResponse, field_34, 0x34);
assert_offset!(CPedDamageResponse, field_38, 0x38);

/// Merged layout for `CPedFormation`.
///
/// Size: 0xd4 (low). Bases: none.
/// Lanes: c-entities, via:CPedFormation_Arrowhead, via:CPedFormation_FollowInLine, via:CPedFormation_LineAbreast, via:CPedFormation_Loose, via:CPedFormation_SurroundFacingAhead, via:CPedFormation_SurroundFacingInwards, via:CPedFormation_V.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedFormation {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x8).
    pub _pad_0004: [u8; 0x4],
    /// field_8 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_8: f32,
    /// Unknown bytes (0xc..0x14).
    pub _pad_000c: [u8; 0x8],
    /// field_14 (confidence: medium, kind: u32, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_14: u32,
    /// field_18 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_Loose,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards moved from siblings:CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_Loose,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards).
    pub field_18: f32,
    /// field_1c (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_Loose,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards moved from siblings:CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_Loose,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards).
    pub field_1c: f32,
    /// field_20 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_Loose,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards moved from siblings:CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_Loose,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards).
    pub field_20: f32,
    /// field_24 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_Loose,via:CPedFormation_SurroundFacingInwards moved from siblings:CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_Loose,CPedFormation_SurroundFacingInwards).
    pub field_24: f32,
    /// field_28 (confidence: high, kind: u32, lanes: c-entities,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_Loose,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards moved from siblings:CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_Loose,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0xc0).
    pub _pad_002c: [u8; 0x94],
    /// field_c0 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_c0: f32,
    /// field_c4 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_c4: f32,
    /// field_c8 (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_c8: f32,
    /// field_cc (confidence: high, kind: float, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_cc: f32,
    /// field_d0 (confidence: medium, kind: u32, lanes: c-entities,via:CPedFormation_Arrowhead,via:CPedFormation_FollowInLine,via:CPedFormation_LineAbreast,via:CPedFormation_SurroundFacingAhead,via:CPedFormation_SurroundFacingInwards,via:CPedFormation_V moved from siblings:CPedFormation_Arrowhead,CPedFormation_FollowInLine,CPedFormation_LineAbreast,CPedFormation_SurroundFacingAhead,CPedFormation_SurroundFacingInwards,CPedFormation_V).
    pub field_d0: u32,
}
assert_size!(CPedFormation, 0xd4); // merged size 0xd4 rounded to 4
assert_offset!(CPedFormation, vfptr, 0x0);
assert_offset!(CPedFormation, field_8, 0x8);
assert_offset!(CPedFormation, field_14, 0x14);
assert_offset!(CPedFormation, field_18, 0x18);
assert_offset!(CPedFormation, field_1c, 0x1c);
assert_offset!(CPedFormation, field_20, 0x20);
assert_offset!(CPedFormation, field_24, 0x24);
assert_offset!(CPedFormation, field_28, 0x28);
assert_offset!(CPedFormation, field_c0, 0xc0);
assert_offset!(CPedFormation, field_c4, 0xc4);
assert_offset!(CPedFormation, field_c8, 0xc8);
assert_offset!(CPedFormation, field_cc, 0xcc);
assert_offset!(CPedFormation, field_d0, 0xd0);

/// Merged layout for `CPedFormation_FollowInLine`.
///
/// Size: 0x1b0 (low). Bases: CPedFormation@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedFormationFollowInLine {
    /// Unknown bytes (0x0..0xd8).
    pub _pad_0000: [u8; 0xd8],
    /// field_d8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_d8: u32,
    /// Unknown bytes (0xdc..0xe0).
    pub _pad_00dc: [u8; 0x4],
    /// field_e0 (confidence: high, kind: float, lanes: c-entities).
    pub field_e0: f32,
    /// field_e4 (confidence: high, kind: float, lanes: c-entities).
    pub field_e4: f32,
    /// field_e8 (confidence: high, kind: float, lanes: c-entities).
    pub field_e8: f32,
    /// field_ec (confidence: high, kind: float, lanes: c-entities).
    pub field_ec: f32,
    /// field_f0 (confidence: high, kind: float, lanes: c-entities).
    pub field_f0: f32,
    /// field_f4 (confidence: high, kind: float, lanes: c-entities).
    pub field_f4: f32,
    /// field_f8 (confidence: high, kind: float, lanes: c-entities).
    pub field_f8: f32,
    /// field_fc (confidence: high, kind: float, lanes: c-entities).
    pub field_fc: f32,
    /// field_100 (confidence: high, kind: float, lanes: c-entities).
    pub field_100: f32,
    /// field_104 (confidence: high, kind: float, lanes: c-entities).
    pub field_104: f32,
    /// field_108 (confidence: high, kind: float, lanes: c-entities).
    pub field_108: f32,
    /// field_10c (confidence: high, kind: float, lanes: c-entities).
    pub field_10c: f32,
    /// field_110 (confidence: high, kind: float, lanes: c-entities).
    pub field_110: f32,
    /// field_114 (confidence: high, kind: float, lanes: c-entities).
    pub field_114: f32,
    /// field_118 (confidence: high, kind: float, lanes: c-entities).
    pub field_118: f32,
    /// field_11c (confidence: high, kind: float, lanes: c-entities).
    pub field_11c: f32,
    /// field_120 (confidence: high, kind: float, lanes: c-entities).
    pub field_120: f32,
    /// field_124 (confidence: high, kind: float, lanes: c-entities).
    pub field_124: f32,
    /// field_128 (confidence: high, kind: float, lanes: c-entities).
    pub field_128: f32,
    /// field_12c (confidence: high, kind: float, lanes: c-entities).
    pub field_12c: f32,
    /// field_130 (confidence: high, kind: float, lanes: c-entities).
    pub field_130: f32,
    /// field_134 (confidence: high, kind: float, lanes: c-entities).
    pub field_134: f32,
    /// field_138 (confidence: high, kind: float, lanes: c-entities).
    pub field_138: f32,
    /// field_13c (confidence: high, kind: float, lanes: c-entities).
    pub field_13c: f32,
    /// field_140 (confidence: high, kind: float, lanes: c-entities).
    pub field_140: f32,
    /// field_144 (confidence: high, kind: float, lanes: c-entities).
    pub field_144: f32,
    /// field_148 (confidence: high, kind: float, lanes: c-entities).
    pub field_148: f32,
    /// field_14c (confidence: high, kind: float, lanes: c-entities).
    pub field_14c: f32,
    /// field_150 (confidence: high, kind: float, lanes: c-entities).
    pub field_150: f32,
    /// field_154 (confidence: high, kind: float, lanes: c-entities).
    pub field_154: f32,
    /// field_158 (confidence: high, kind: float, lanes: c-entities).
    pub field_158: f32,
    /// field_15c (confidence: high, kind: float, lanes: c-entities).
    pub field_15c: f32,
    /// field_160 (confidence: high, kind: float, lanes: c-entities).
    pub field_160: f32,
    /// field_164 (confidence: high, kind: float, lanes: c-entities).
    pub field_164: f32,
    /// field_168 (confidence: high, kind: float, lanes: c-entities).
    pub field_168: f32,
    /// field_16c (confidence: high, kind: float, lanes: c-entities).
    pub field_16c: f32,
    /// field_170 (confidence: high, kind: float, lanes: c-entities).
    pub field_170: f32,
    /// field_174 (confidence: high, kind: float, lanes: c-entities).
    pub field_174: f32,
    /// field_178 (confidence: high, kind: float, lanes: c-entities).
    pub field_178: f32,
    /// field_17c (confidence: high, kind: float, lanes: c-entities).
    pub field_17c: f32,
    /// field_180 (confidence: high, kind: float, lanes: c-entities).
    pub field_180: f32,
    /// field_184 (confidence: high, kind: float, lanes: c-entities).
    pub field_184: f32,
    /// field_188 (confidence: high, kind: float, lanes: c-entities).
    pub field_188: f32,
    /// field_18c (confidence: high, kind: float, lanes: c-entities).
    pub field_18c: f32,
    /// field_190 (confidence: high, kind: float, lanes: c-entities).
    pub field_190: f32,
    /// field_194 (confidence: high, kind: float, lanes: c-entities).
    pub field_194: f32,
    /// field_198 (confidence: high, kind: float, lanes: c-entities).
    pub field_198: f32,
    /// field_19c (confidence: high, kind: float, lanes: c-entities).
    pub field_19c: f32,
    /// field_1a0 (confidence: high, kind: float, lanes: c-entities).
    pub field_1a0: f32,
    /// field_1a4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1a4: f32,
    /// field_1a8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1a8: f32,
    /// field_1ac (confidence: high, kind: float, lanes: c-entities).
    pub field_1ac: f32,
}
assert_size!(CPedFormationFollowInLine, 0x1b0); // merged size 0x1b0 rounded to 4
assert_offset!(CPedFormationFollowInLine, field_d8, 0xd8);
assert_offset!(CPedFormationFollowInLine, field_e0, 0xe0);
assert_offset!(CPedFormationFollowInLine, field_e4, 0xe4);
assert_offset!(CPedFormationFollowInLine, field_e8, 0xe8);
assert_offset!(CPedFormationFollowInLine, field_ec, 0xec);
assert_offset!(CPedFormationFollowInLine, field_f0, 0xf0);
assert_offset!(CPedFormationFollowInLine, field_f4, 0xf4);
assert_offset!(CPedFormationFollowInLine, field_f8, 0xf8);
assert_offset!(CPedFormationFollowInLine, field_fc, 0xfc);
assert_offset!(CPedFormationFollowInLine, field_100, 0x100);
assert_offset!(CPedFormationFollowInLine, field_104, 0x104);
assert_offset!(CPedFormationFollowInLine, field_108, 0x108);
assert_offset!(CPedFormationFollowInLine, field_10c, 0x10c);
assert_offset!(CPedFormationFollowInLine, field_110, 0x110);
assert_offset!(CPedFormationFollowInLine, field_114, 0x114);
assert_offset!(CPedFormationFollowInLine, field_118, 0x118);
assert_offset!(CPedFormationFollowInLine, field_11c, 0x11c);
assert_offset!(CPedFormationFollowInLine, field_120, 0x120);
assert_offset!(CPedFormationFollowInLine, field_124, 0x124);
assert_offset!(CPedFormationFollowInLine, field_128, 0x128);
assert_offset!(CPedFormationFollowInLine, field_12c, 0x12c);
assert_offset!(CPedFormationFollowInLine, field_130, 0x130);
assert_offset!(CPedFormationFollowInLine, field_134, 0x134);
assert_offset!(CPedFormationFollowInLine, field_138, 0x138);
assert_offset!(CPedFormationFollowInLine, field_13c, 0x13c);
assert_offset!(CPedFormationFollowInLine, field_140, 0x140);
assert_offset!(CPedFormationFollowInLine, field_144, 0x144);
assert_offset!(CPedFormationFollowInLine, field_148, 0x148);
assert_offset!(CPedFormationFollowInLine, field_14c, 0x14c);
assert_offset!(CPedFormationFollowInLine, field_150, 0x150);
assert_offset!(CPedFormationFollowInLine, field_154, 0x154);
assert_offset!(CPedFormationFollowInLine, field_158, 0x158);
assert_offset!(CPedFormationFollowInLine, field_15c, 0x15c);
assert_offset!(CPedFormationFollowInLine, field_160, 0x160);
assert_offset!(CPedFormationFollowInLine, field_164, 0x164);
assert_offset!(CPedFormationFollowInLine, field_168, 0x168);
assert_offset!(CPedFormationFollowInLine, field_16c, 0x16c);
assert_offset!(CPedFormationFollowInLine, field_170, 0x170);
assert_offset!(CPedFormationFollowInLine, field_174, 0x174);
assert_offset!(CPedFormationFollowInLine, field_178, 0x178);
assert_offset!(CPedFormationFollowInLine, field_17c, 0x17c);
assert_offset!(CPedFormationFollowInLine, field_180, 0x180);
assert_offset!(CPedFormationFollowInLine, field_184, 0x184);
assert_offset!(CPedFormationFollowInLine, field_188, 0x188);
assert_offset!(CPedFormationFollowInLine, field_18c, 0x18c);
assert_offset!(CPedFormationFollowInLine, field_190, 0x190);
assert_offset!(CPedFormationFollowInLine, field_194, 0x194);
assert_offset!(CPedFormationFollowInLine, field_198, 0x198);
assert_offset!(CPedFormationFollowInLine, field_19c, 0x19c);
assert_offset!(CPedFormationFollowInLine, field_1a0, 0x1a0);
assert_offset!(CPedFormationFollowInLine, field_1a4, 0x1a4);
assert_offset!(CPedFormationFollowInLine, field_1a8, 0x1a8);
assert_offset!(CPedFormationFollowInLine, field_1ac, 0x1ac);

/// Merged layout for `CPedIntelligence`.
///
/// Size: 0x2e4 (low). Bases: none.
/// Lanes: c-entities, n-11.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedIntelligence {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x40).
    pub _pad_0014: [u8; 0x2c],
    /// field_40 (confidence: low, kind: u32, lanes: c-entities).
    pub field_40: u32,
    /// field_44 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_44: u32,
    /// Unknown bytes (0x48..0x84).
    pub _pad_0048: [u8; 0x3c],
    /// field_84 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_84: u32,
    /// Unknown bytes (0x88..0xd0).
    pub _pad_0088: [u8; 0x48],
    /// field_d0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_d0: u32,
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
    /// field_ec (confidence: low, kind: u32, lanes: c-entities).
    pub field_ec: u32,
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
    /// Unknown bytes (0x21f..0x220).
    pub _pad_021f: [u8; 0x1],
    /// field_220 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_220: u8,
    /// Unknown bytes (0x221..0x230).
    pub _pad_0221: [u8; 0xf],
    /// field_230 (confidence: low, kind: u32, lanes: c-entities).
    pub field_230: u32,
    /// Unknown bytes (0x234..0x250).
    pub _pad_0234: [u8; 0x1c],
    /// field_250 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_250: u32,
    /// Unknown bytes (0x254..0x25c).
    pub _pad_0254: [u8; 0x8],
    /// field_25c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_25c: u32,
    /// field_260 (confidence: low, kind: u32, lanes: c-entities).
    pub field_260: u32,
    /// field_264 (confidence: low, kind: u32, lanes: c-entities).
    pub field_264: u32,
    /// field_268 (confidence: low, kind: u32, lanes: c-entities).
    pub field_268: u32,
    /// Unknown bytes (0x26c..0x280).
    pub _pad_026c: [u8; 0x14],
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
    /// pinned_down_counter_compared (confidence: high, kind: i32, lanes: c-entities,n-11).
    pub pinned_down_counter_compared: u8,
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
    /// head_task_chain (confidence: high, kind: u32, lanes: c-entities,n-11).
    pub head_task_chain: u32,
}
assert_size!(CPedIntelligence, 0x2e4); // merged size 0x2e4 rounded to 4
assert_offset!(CPedIntelligence, vfptr, 0x0);
assert_offset!(CPedIntelligence, field_10, 0x10);
assert_offset!(CPedIntelligence, field_40, 0x40);
assert_offset!(CPedIntelligence, field_44, 0x44);
assert_offset!(CPedIntelligence, field_84, 0x84);
assert_offset!(CPedIntelligence, field_d0, 0xd0);
assert_offset!(CPedIntelligence, field_d4, 0xd4);
assert_offset!(CPedIntelligence, field_d8, 0xd8);
assert_offset!(CPedIntelligence, field_dc, 0xdc);
assert_offset!(CPedIntelligence, field_e0, 0xe0);
assert_offset!(CPedIntelligence, field_e4, 0xe4);
assert_offset!(CPedIntelligence, field_e8, 0xe8);
assert_offset!(CPedIntelligence, field_ec, 0xec);
assert_offset!(CPedIntelligence, field_f0, 0xf0);
assert_offset!(CPedIntelligence, field_f4, 0xf4);
assert_offset!(CPedIntelligence, field_150, 0x150);
assert_offset!(CPedIntelligence, field_1ac, 0x1ac);
assert_offset!(CPedIntelligence, field_208, 0x208);
assert_offset!(CPedIntelligence, field_20c, 0x20c);
assert_offset!(CPedIntelligence, field_210, 0x210);
assert_offset!(CPedIntelligence, field_214, 0x214);
assert_offset!(CPedIntelligence, field_21e, 0x21e);
assert_offset!(CPedIntelligence, field_220, 0x220);
assert_offset!(CPedIntelligence, field_230, 0x230);
assert_offset!(CPedIntelligence, field_250, 0x250);
assert_offset!(CPedIntelligence, field_25c, 0x25c);
assert_offset!(CPedIntelligence, field_260, 0x260);
assert_offset!(CPedIntelligence, field_264, 0x264);
assert_offset!(CPedIntelligence, field_268, 0x268);
assert_offset!(CPedIntelligence, field_280, 0x280);
assert_offset!(CPedIntelligence, field_284, 0x284);
assert_offset!(CPedIntelligence, field_288, 0x288);
assert_offset!(CPedIntelligence, field_28c, 0x28c);
assert_offset!(CPedIntelligence, field_290, 0x290);
assert_offset!(CPedIntelligence, field_294, 0x294);
assert_offset!(CPedIntelligence, field_298, 0x298);
assert_offset!(CPedIntelligence, field_2a0, 0x2a0);
assert_offset!(CPedIntelligence, field_2b0, 0x2b0);
assert_offset!(CPedIntelligence, field_2b4, 0x2b4);
assert_offset!(CPedIntelligence, field_2b8, 0x2b8);
assert_offset!(CPedIntelligence, field_2c0, 0x2c0);
assert_offset!(CPedIntelligence, field_2c4, 0x2c4);
assert_offset!(CPedIntelligence, pinned_down_counter_compared, 0x2c8);
assert_offset!(CPedIntelligence, field_2cc, 0x2cc);
assert_offset!(CPedIntelligence, field_2d0, 0x2d0);
assert_offset!(CPedIntelligence, field_2d4, 0x2d4);
assert_offset!(CPedIntelligence, field_2d8, 0x2d8);
assert_offset!(CPedIntelligence, head_task_chain, 0x2e0);

/// Merged layout for `CPedModelInfo`.
///
/// Size: 0x160 (high). Bases: CBaseModelInfo@0x0.
/// Lanes: c-entities, n-07, p0-360.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedModelInfo {
    /// Unknown bytes (0x0..0x60).
    pub _pad_0000: [u8; 0x60],
    /// field_60 (confidence: low, kind: u32, lanes: c-entities).
    pub field_60: u32,
    /// Unknown bytes (0x64..0x6c).
    pub _pad_0064: [u8; 0x8],
    /// field_6c (confidence: low, kind: u32, lanes: c-entities).
    pub field_6c: u32,
    /// Unknown bytes (0x70..0x78).
    pub _pad_0070: [u8; 0x8],
    /// field_78 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_78: u32,
    /// field_7c (confidence: high, kind: pointer, lanes: c-entities).
    pub field_7c: Ptr32<u8>,
    /// m_nGestureAnimIndex (s32, iv-sdk 1.0.7.0) (confidence: low, kind: u32, lanes: c-entities).
    pub gesture_anim_index: u32,
    /// m_nFacialAnimIndex (s32, iv-sdk 1.0.7.0) (confidence: low, kind: u32, lanes: c-entities).
    pub facial_anim_index: u32,
    /// Unknown bytes (0x88..0x8c).
    pub _pad_0088: [u8; 0x4],
    /// m_bStreamedPed (confidence: low, kind: u8, lanes: p0-360).
    pub streamed_ped: u8,
    /// Unknown bytes (0x8d..0x90).
    pub _pad_008d: [u8; 0x3],
    /// m_nPedType (confidence: medium, kind: u32, lanes: c-entities,p0-360).
    pub ped_type: u32,
    /// field_94 (confidence: low, kind: u32, lanes: c-entities).
    pub field_94: u32,
    /// field_98 (confidence: low, kind: u32, lanes: c-entities).
    pub field_98: u32,
    /// Unknown bytes (0x9c..0xa0).
    pub _pad_009c: [u8; 0x4],
    /// field_a0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_a0: u32,
    /// field_a4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_a4: u32,
    /// field_a8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_a8: u32,
    /// field_ac (confidence: low, kind: u32, lanes: c-entities).
    pub field_ac: u32,
    /// Unknown bytes (0xb0..0xb2).
    pub _pad_00b0: [u8; 0x2],
    /// field_b2 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_b2: u8,
    /// Unknown bytes (0xb3..0xb4).
    pub _pad_00b3: [u8; 0x1],
    /// field_b4 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_b4: Ptr32<u8>,
    /// Unknown bytes (0xb8..0xe8).
    pub _pad_00b8: [u8; 0x30],
    /// field_e8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_e8: u32,
    /// field_ec (confidence: low, kind: u32, lanes: c-entities).
    pub field_ec: u32,
    /// field_f0 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_f0: u16,
    /// Unknown bytes (0xf2..0xfc).
    pub _pad_00f2: [u8; 0xa],
    /// field_fc (confidence: low, kind: u32, lanes: c-entities).
    pub field_fc: u32,
    /// Unknown bytes (0x100..0x120).
    pub _pad_0100: [u8; 0x20],
    /// field_120 (confidence: low, kind: u32, lanes: c-entities).
    pub field_120: u32,
    /// Unknown bytes (0x124..0x15c).
    pub _pad_0124: [u8; 0x38],
    /// field_15c (confidence: low, kind: int16?, lanes: c-entities).
    pub field_15c: u16,
    /// Unknown trailing bytes (0x15e..0x160).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CPedModelInfo, 0x160); // merged size 0x160 rounded to 4
assert_offset!(CPedModelInfo, field_60, 0x60);
assert_offset!(CPedModelInfo, field_6c, 0x6c);
assert_offset!(CPedModelInfo, field_78, 0x78);
assert_offset!(CPedModelInfo, field_7c, 0x7c);
assert_offset!(CPedModelInfo, gesture_anim_index, 0x80);
assert_offset!(CPedModelInfo, facial_anim_index, 0x84);
assert_offset!(CPedModelInfo, streamed_ped, 0x8c);
assert_offset!(CPedModelInfo, ped_type, 0x90);
assert_offset!(CPedModelInfo, field_94, 0x94);
assert_offset!(CPedModelInfo, field_98, 0x98);
assert_offset!(CPedModelInfo, field_a0, 0xa0);
assert_offset!(CPedModelInfo, field_a4, 0xa4);
assert_offset!(CPedModelInfo, field_a8, 0xa8);
assert_offset!(CPedModelInfo, field_ac, 0xac);
assert_offset!(CPedModelInfo, field_b2, 0xb2);
assert_offset!(CPedModelInfo, field_b4, 0xb4);
assert_offset!(CPedModelInfo, field_e8, 0xe8);
assert_offset!(CPedModelInfo, field_ec, 0xec);
assert_offset!(CPedModelInfo, field_f0, 0xf0);
assert_offset!(CPedModelInfo, field_fc, 0xfc);
assert_offset!(CPedModelInfo, field_120, 0x120);
assert_offset!(CPedModelInfo, field_15c, 0x15c);

/// Merged layout for `CPedMoveBlendBase`.
///
/// Size: 0x54 (low). Bases: none.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedMoveBlendBase {
    /// __vfptr (confidence: high, kind: vtable_ptr, lanes: c-entities).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_4: u32,
    /// field_8 (confidence: high, kind: float, lanes: c-entities,p0-360).
    pub field_8: f32,
    /// field_c (confidence: low, kind: u32, lanes: c-entities).
    pub field_c: u32,
    /// field_10 (confidence: low, kind: u32, lanes: c-entities).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_14: u32,
    /// field_18 (confidence: low, kind: u32, lanes: c-entities).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: u32, lanes: c-entities,p0-360).
    pub field_1c: u32,
    /// field_20 (confidence: low, kind: u32, lanes: c-entities).
    pub field_20: u32,
    /// field_24 (confidence: high, kind: u32, lanes: c-entities,p0-360).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: u32, lanes: c-entities,p0-360).
    pub field_28: u32,
    /// field_2c (confidence: low, kind: u32, lanes: c-entities).
    pub field_2c: u32,
    /// field_30 (confidence: low, kind: u32, lanes: c-entities).
    pub field_30: u32,
    /// field_34 (confidence: low, kind: u32, lanes: c-entities).
    pub field_34: u32,
    /// field_38 (confidence: low, kind: u32, lanes: c-entities).
    pub field_38: u32,
    /// field_3c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_3c: u32,
    /// field_40 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_40: u32,
    /// field_44 (confidence: low, kind: u32, lanes: c-entities).
    pub field_44: u32,
    /// field_48 (confidence: low, kind: u32, lanes: c-entities).
    pub field_48: u32,
    /// field_4c (confidence: low, kind: u32, lanes: c-entities).
    pub field_4c: u32,
    /// field_50 (confidence: high, kind: u32, lanes: c-entities,p0-360).
    pub field_50: u32,
}
assert_size!(CPedMoveBlendBase, 0x54); // merged size 0x54 rounded to 4
assert_offset!(CPedMoveBlendBase, vfptr, 0x0);
assert_offset!(CPedMoveBlendBase, field_4, 0x4);
assert_offset!(CPedMoveBlendBase, field_8, 0x8);
assert_offset!(CPedMoveBlendBase, field_c, 0xc);
assert_offset!(CPedMoveBlendBase, field_10, 0x10);
assert_offset!(CPedMoveBlendBase, field_14, 0x14);
assert_offset!(CPedMoveBlendBase, field_18, 0x18);
assert_offset!(CPedMoveBlendBase, field_1c, 0x1c);
assert_offset!(CPedMoveBlendBase, field_20, 0x20);
assert_offset!(CPedMoveBlendBase, field_24, 0x24);
assert_offset!(CPedMoveBlendBase, field_28, 0x28);
assert_offset!(CPedMoveBlendBase, field_2c, 0x2c);
assert_offset!(CPedMoveBlendBase, field_30, 0x30);
assert_offset!(CPedMoveBlendBase, field_34, 0x34);
assert_offset!(CPedMoveBlendBase, field_38, 0x38);
assert_offset!(CPedMoveBlendBase, field_3c, 0x3c);
assert_offset!(CPedMoveBlendBase, field_40, 0x40);
assert_offset!(CPedMoveBlendBase, field_44, 0x44);
assert_offset!(CPedMoveBlendBase, field_48, 0x48);
assert_offset!(CPedMoveBlendBase, field_4c, 0x4c);
assert_offset!(CPedMoveBlendBase, field_50, 0x50);

/// Merged layout for `CPedTargetting`.
///
/// Size: 0x270 (low). Bases: CTargetting@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedTargetting {
    /// Unknown bytes (0x0..0x24).
    pub _pad_0000: [u8; 0x24],
    /// field_24 (confidence: medium, kind: float, lanes: c-entities).
    pub field_24: f32,
    /// field_28 (confidence: high, kind: float, lanes: c-entities).
    pub field_28: f32,
    /// field_2c (confidence: low, kind: u32, lanes: c-entities).
    pub field_2c: u32,
    /// Unknown bytes (0x30..0x34).
    pub _pad_0030: [u8; 0x4],
    /// field_34 (confidence: high, kind: u32, lanes: c-entities).
    pub field_34: u32,
    /// Unknown bytes (0x38..0x44).
    pub _pad_0038: [u8; 0xc],
    /// field_44 (confidence: medium, kind: float, lanes: c-entities).
    pub field_44: f32,
    /// field_48 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_48: u32,
    /// field_4c (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_4c: u8,
    /// field_4d (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_4d: u8,
    /// Unknown bytes (0x4e..0x240).
    pub _pad_004e: [u8; 0x1f2],
    /// field_240 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_240: u32,
    /// field_244 (confidence: low, kind: u32, lanes: c-entities).
    pub field_244: u32,
    /// field_248 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_248: u32,
    /// field_24c (confidence: high, kind: pointer, lanes: c-entities).
    pub field_24c: Ptr32<u8>,
    /// Unknown bytes (0x250..0x254).
    pub _pad_0250: [u8; 0x4],
    /// field_254 (confidence: low, kind: u32, lanes: c-entities).
    pub field_254: u32,
    /// field_258 (confidence: low, kind: u32, lanes: c-entities).
    pub field_258: u32,
    /// Unknown bytes (0x25c..0x26c).
    pub _pad_025c: [u8; 0x10],
    /// field_26c (confidence: low, kind: u32, lanes: c-entities).
    pub field_26c: u32,
}
assert_size!(CPedTargetting, 0x270); // merged size 0x270 rounded to 4
assert_offset!(CPedTargetting, field_24, 0x24);
assert_offset!(CPedTargetting, field_28, 0x28);
assert_offset!(CPedTargetting, field_2c, 0x2c);
assert_offset!(CPedTargetting, field_34, 0x34);
assert_offset!(CPedTargetting, field_44, 0x44);
assert_offset!(CPedTargetting, field_48, 0x48);
assert_offset!(CPedTargetting, field_4c, 0x4c);
assert_offset!(CPedTargetting, field_4d, 0x4d);
assert_offset!(CPedTargetting, field_240, 0x240);
assert_offset!(CPedTargetting, field_244, 0x244);
assert_offset!(CPedTargetting, field_248, 0x248);
assert_offset!(CPedTargetting, field_24c, 0x24c);
assert_offset!(CPedTargetting, field_254, 0x254);
assert_offset!(CPedTargetting, field_258, 0x258);
assert_offset!(CPedTargetting, field_26c, 0x26c);

/// Merged layout for `CPedTriggerScriptAttractor`.
///
/// Size: 0xb4 (low). Bases: CPedAttractor@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPedTriggerScriptAttractor {
    /// Unknown bytes (0x0..0xa0).
    pub _pad_0000: [u8; 0xa0],
    /// field_a0 (confidence: medium, kind: float, lanes: c-entities).
    pub field_a0: f32,
    /// field_a4 (confidence: medium, kind: float, lanes: c-entities).
    pub field_a4: f32,
    /// field_a8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_a8: f32,
    /// field_ac (confidence: medium, kind: float, lanes: c-entities).
    pub field_ac: f32,
    /// field_b0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_b0: u32,
}
assert_size!(CPedTriggerScriptAttractor, 0xb4); // merged size 0xb4 rounded to 4
assert_offset!(CPedTriggerScriptAttractor, field_a0, 0xa0);
assert_offset!(CPedTriggerScriptAttractor, field_a4, 0xa4);
assert_offset!(CPedTriggerScriptAttractor, field_a8, 0xa8);
assert_offset!(CPedTriggerScriptAttractor, field_ac, 0xac);
assert_offset!(CPedTriggerScriptAttractor, field_b0, 0xb0);

/// Merged layout for `CPlayer`.
///
/// Size: 0xee8 (low). Bases: none.
/// Lanes: n-02, n-04, n-22.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlayer {
    /// Unknown bytes (0x0..0x11).
    pub _pad_0000: [u8; 0x11],
    /// nonzero_wanted_level_active (confidence: medium, kind: u8, lanes: n-22).
    pub nonzero_wanted_level_active: u8,
    /// Unknown bytes (0x12..0x5c).
    pub _pad_0012: [u8; 0x4a],
    /// nonzero_wanted_stars_flashing (confidence: medium, kind: u32, lanes: n-22).
    pub nonzero_wanted_stars_flashing: u32,
    /// Unknown bytes (0x60..0x560).
    pub _pad_0060: [u8; 0x500],
    /// has_damaged_vehicle (confidence: high, kind: flags, lanes: n-02).
    pub has_damaged_vehicle: u32,
    /// Unknown bytes (0x564..0x578).
    pub _pad_0564: [u8; 0x14],
    /// network_controller_owner_id (confidence: medium, kind: u32, lanes: n-04).
    pub network_controller_owner_id: u32,
    /// Unknown bytes (0x57c..0x598).
    pub _pad_057c: [u8; 0x1c],
    /// sub_object_pickup_candidate (confidence: medium, kind: pointer, lanes: n-02,n-04,n-22).
    pub sub_object_pickup_candidate: Ptr32<u8>,
    /// Unknown bytes (0x59c..0xee4).
    pub _pad_059c: [u8; 0x948],
    /// stats_menu_actions_state (confidence: medium, kind: u32, lanes: n-02).
    pub stats_menu_actions_state: u32,
}
assert_size!(CPlayer, 0xee8); // merged size 0xee8 rounded to 4
assert_offset!(CPlayer, nonzero_wanted_level_active, 0x11);
assert_offset!(CPlayer, nonzero_wanted_stars_flashing, 0x5c);
assert_offset!(CPlayer, has_damaged_vehicle, 0x560);
assert_offset!(CPlayer, network_controller_owner_id, 0x578);
assert_offset!(CPlayer, sub_object_pickup_candidate, 0x598);
assert_offset!(CPlayer, stats_menu_actions_state, 0xee4);

/// Merged layout for `CPlayerInfo`.
///
/// Size: 0x5c8 (low). Bases: none.
/// Lanes: n-07, n-08, n-10, n-11, n-12, n-13, n-14, n-18, p0-360.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlayerInfo {
    /// Unknown bytes (0x0..0x40).
    pub _pad_0000: [u8; 0x40],
    /// voice_talker_data_block (confidence: medium, kind: u32, lanes: n-13).
    pub voice_talker_data_block: u32,
    /// Unknown bytes (0x44..0x48).
    pub _pad_0044: [u8; 0x4],
    /// voice_guid_block_copied (confidence: medium, kind: u32, lanes: n-13).
    pub voice_guid_block_copied: [u8; 16],
    /// Unknown bytes (0x58..0x228).
    pub _pad_0058: [u8; 0x1d0],
    /// rc_buggy_controller (confidence: low, kind: pointer, lanes: n-14).
    pub rc_buggy_controller: Ptr32<u8>,
    /// Unknown bytes (0x22c..0x3f0).
    pub _pad_022c: [u8; 0x1c4],
    /// m_pPlayerPed2 (confidence: low, kind: pointer, lanes: p0-360).
    pub player_ped2: Ptr32<u8>,
    /// Unknown bytes (0x3f4..0x414).
    pub _pad_03f4: [u8; 0x20],
    /// m_fStamina (confidence: low, kind: float, lanes: p0-360).
    pub stamina: f32,
    /// Unknown bytes (0x418..0x494).
    pub _pad_0418: [u8; 0x7c],
    /// m_nLastHitPedTime (confidence: low, kind: u32, lanes: p0-360).
    pub last_hit_ped_time: u32,
    /// Unknown bytes (0x498..0x4a0).
    pub _pad_0498: [u8; 0x8],
    /// timestamp_last_hit_car (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_hit_car: u32,
    /// timestamp_last_hit_ped (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_hit_ped: u32,
    /// timestamp_last_hit_building (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_hit_building: u32,
    /// timestamp_last_hit_object (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_hit_object: u32,
    /// timestamp_last_drove_pavement (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_drove_pavement: u32,
    /// timestamp_last_ran_light (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_ran_light: u32,
    /// timestamp_last_drove_against (confidence: high, kind: u32, lanes: n-08).
    pub timestamp_last_drove_against: u32,
    /// m_nControlFlags (confidence: low, kind: u32, lanes: p0-360).
    pub control_flags: u32,
    /// Unknown bytes (0x4c0..0x4c8).
    pub _pad_04c0: [u8; 0x8],
    /// text_chat_control_claim (confidence: high, kind: flags, lanes: n-10,n-11,n-12,n-14,n-18).
    pub text_chat_control_claim: u32,
    /// Unknown bytes (0x4cc..0x4d0).
    pub _pad_04cc: [u8; 0x4],
    /// resurrect_timestamp (confidence: medium, kind: u32, lanes: n-14).
    pub resurrect_timestamp: u32,
    /// Unknown bytes (0x4d4..0x4da).
    pub _pad_04d4: [u8; 0x6],
    /// m_nPlayerId (confidence: low, kind: u8, lanes: p0-360).
    pub player_id: u8,
    /// Unknown bytes (0x4db..0x4dc).
    pub _pad_04db: [u8; 0x1],
    /// m_nState (confidence: low, kind: u32, lanes: p0-360).
    pub state: u32,
    /// Unknown bytes (0x4e0..0x4e8).
    pub _pad_04e0: [u8; 0x8],
    /// player_state_ready_playing (confidence: high, kind: u32, lanes: n-10,n-11,n-13,n-14,n-18).
    pub player_state_ready_playing: u32,
    /// Unknown bytes (0x4ec..0x514).
    pub _pad_04ec: [u8; 0x28],
    /// wheelie_state (confidence: high, kind: u32, lanes: n-11).
    pub wheelie_state: u32,
    /// Unknown bytes (0x518..0x51c).
    pub _pad_0518: [u8; 0x4],
    /// stoppie_state (confidence: high, kind: u32, lanes: n-11).
    pub stoppie_state: u32,
    /// Unknown bytes (0x520..0x546).
    pub _pad_0520: [u8; 0x26],
    /// m_nNeverTired (confidence: low, kind: u8, lanes: p0-360).
    pub m_nnevertired: u8,
    /// Unknown bytes (0x547..0x54a).
    pub _pad_0547: [u8; 0x3],
    /// m_nMaxHealth (confidence: low, kind: u16, lanes: p0-360).
    pub m_nmaxhealth: u16,
    /// m_nMaxArmor (confidence: low, kind: u16, lanes: p0-360).
    pub max_armor: u16,
    /// Unknown bytes (0x54e..0x550).
    pub _pad_054e: [u8; 0x2],
    /// stat_adjusted_stat_add (confidence: medium, kind: i32, lanes: n-14,p0-360).
    pub stat_adjusted_stat_add: u16,
    /// never_tired (confidence: high, kind: u8, lanes: n-18).
    pub never_tired: u8,
    /// fast_reload (confidence: high, kind: u8, lanes: n-18).
    pub fast_reload: u8,
    /// fireproof (confidence: high, kind: flags, lanes: n-12).
    pub fireproof: u8,
    /// Unknown bytes (0x555..0x556).
    pub _pad_0555: [u8; 0x1],
    /// max_health (confidence: high, kind: float, lanes: n-07,n-14).
    pub max_health: u16,
    /// Unknown bytes (0x558..0x55c).
    pub _pad_0558: [u8; 0x4],
    /// can_do_drive (confidence: high, kind: u8, lanes: n-18,p0-360).
    pub can_do_drive: u8,
    /// can_hassled_gangs (confidence: high, kind: u8, lanes: n-18).
    pub can_hassled_gangs: u8,
    /// Unknown bytes (0x55e..0x560).
    pub _pad_055e: [u8; 0x2],
    /// can_use_cover (confidence: high, kind: u32, lanes: n-18).
    pub can_use_cover: u32,
    /// Unknown bytes (0x564..0x568).
    pub _pad_0564: [u8; 0x4],
    /// pissed_off_until_timestamp (confidence: high, kind: u32, lanes: n-13,n-18).
    pub pissed_off_until_timestamp: u32,
    /// Unknown bytes (0x56c..0x578).
    pub _pad_056c: [u8; 0xc],
    /// network_connection_object (confidence: medium, kind: u32, lanes: n-10,n-11,n-13).
    pub network_connection_object: u32,
    /// player_colour (confidence: high, kind: u32, lanes: n-07,n-08,n-18).
    pub player_colour: u32,
    /// team (confidence: high, kind: u32, lanes: n-08,n-18,p0-360).
    pub team: u32,
    /// Unknown bytes (0x584..0x58c).
    pub _pad_0584: [u8; 0x8],
    /// m_pPlayerPed (confidence: low, kind: pointer, lanes: p0-360).
    pub m_pplayerped: Ptr32<u8>,
    /// Unknown bytes (0x590..0x594).
    pub _pad_0590: [u8; 0x4],
    /// m_pOnlyEnterThisVehicle (confidence: low, kind: pointer, lanes: p0-360).
    pub only_enter_this_vehicle: Ptr32<u8>,
    /// player_ped (confidence: high, kind: u32, lanes: n-07,n-08,n-10,n-11,n-13,n-14,n-18).
    pub player_ped: u32,
    /// Unknown bytes (0x59c..0x5c4).
    pub _pad_059c: [u8; 0x28],
    /// score_money (confidence: high, kind: u32, lanes: n-11).
    pub score_money: u32,
}
assert_size!(CPlayerInfo, 0x5c8); // merged size 0x5c8 rounded to 4
assert_offset!(CPlayerInfo, voice_talker_data_block, 0x40);
assert_offset!(CPlayerInfo, voice_guid_block_copied, 0x48);
assert_offset!(CPlayerInfo, rc_buggy_controller, 0x228);
assert_offset!(CPlayerInfo, player_ped2, 0x3f0);
assert_offset!(CPlayerInfo, stamina, 0x414);
assert_offset!(CPlayerInfo, last_hit_ped_time, 0x494);
assert_offset!(CPlayerInfo, timestamp_last_hit_car, 0x4a0);
assert_offset!(CPlayerInfo, timestamp_last_hit_ped, 0x4a4);
assert_offset!(CPlayerInfo, timestamp_last_hit_building, 0x4a8);
assert_offset!(CPlayerInfo, timestamp_last_hit_object, 0x4ac);
assert_offset!(CPlayerInfo, timestamp_last_drove_pavement, 0x4b0);
assert_offset!(CPlayerInfo, timestamp_last_ran_light, 0x4b4);
assert_offset!(CPlayerInfo, timestamp_last_drove_against, 0x4b8);
assert_offset!(CPlayerInfo, control_flags, 0x4bc);
assert_offset!(CPlayerInfo, text_chat_control_claim, 0x4c8);
assert_offset!(CPlayerInfo, resurrect_timestamp, 0x4d0);
assert_offset!(CPlayerInfo, player_id, 0x4da);
assert_offset!(CPlayerInfo, state, 0x4dc);
assert_offset!(CPlayerInfo, player_state_ready_playing, 0x4e8);
assert_offset!(CPlayerInfo, wheelie_state, 0x514);
assert_offset!(CPlayerInfo, stoppie_state, 0x51c);
assert_offset!(CPlayerInfo, m_nnevertired, 0x546);
assert_offset!(CPlayerInfo, m_nmaxhealth, 0x54a);
assert_offset!(CPlayerInfo, max_armor, 0x54c);
assert_offset!(CPlayerInfo, stat_adjusted_stat_add, 0x550);
assert_offset!(CPlayerInfo, never_tired, 0x552);
assert_offset!(CPlayerInfo, fast_reload, 0x553);
assert_offset!(CPlayerInfo, fireproof, 0x554);
assert_offset!(CPlayerInfo, max_health, 0x556);
assert_offset!(CPlayerInfo, can_do_drive, 0x55c);
assert_offset!(CPlayerInfo, can_hassled_gangs, 0x55d);
assert_offset!(CPlayerInfo, can_use_cover, 0x560);
assert_offset!(CPlayerInfo, pissed_off_until_timestamp, 0x568);
assert_offset!(CPlayerInfo, network_connection_object, 0x578);
assert_offset!(CPlayerInfo, player_colour, 0x57c);
assert_offset!(CPlayerInfo, team, 0x580);
assert_offset!(CPlayerInfo, m_pplayerped, 0x58c);
assert_offset!(CPlayerInfo, only_enter_this_vehicle, 0x594);
assert_offset!(CPlayerInfo, player_ped, 0x598);
assert_offset!(CPlayerInfo, score_money, 0x5c4);

/// Merged layout for `CPlayerPed`.
///
/// Size: 0xee8 (low). Bases: CPed@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlayerPed {
    /// Unknown bytes (0x0..0xb80).
    pub _pad_0000: [u8; 0xb80],
    /// field_b80 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_b80: u32,
    /// Unknown bytes (0xb84..0xe60).
    pub _pad_0b84: [u8; 0x2dc],
    /// field_e60 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_e60: u8,
    /// field_e61 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_e61: u8,
    /// Unknown bytes (0xe62..0xe64).
    pub _pad_0e62: [u8; 0x2],
    /// field_e64 (confidence: low, kind: u32, lanes: c-entities).
    pub field_e64: u32,
    /// field_e68 (confidence: low, kind: u32, lanes: c-entities).
    pub field_e68: u32,
    /// field_e6c (confidence: low, kind: u32, lanes: c-entities).
    pub field_e6c: u32,
    /// field_e70 (confidence: low, kind: u32, lanes: c-entities).
    pub field_e70: u32,
    /// Unknown bytes (0xe74..0xe94).
    pub _pad_0e74: [u8; 0x20],
    /// field_e94 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_e94: u8,
    /// field_e95 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_e95: [u8; 2],
    /// Unknown bytes (0xe97..0xe9c).
    pub _pad_0e97: [u8; 0x5],
    /// field_e9c (confidence: high, kind: float, lanes: c-entities).
    pub field_e9c: f32,
    /// field_ea0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_ea0: u32,
    /// Unknown bytes (0xea4..0xea8).
    pub _pad_0ea4: [u8; 0x4],
    /// field_ea8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_ea8: u32,
    /// field_eac (confidence: high, kind: pointer, lanes: c-entities).
    pub field_eac: Ptr32<u8>,
    /// field_eb0 (confidence: high, kind: float, lanes: c-entities).
    pub field_eb0: f32,
    /// field_eb4 (confidence: high, kind: float, lanes: c-entities).
    pub field_eb4: f32,
    /// field_eb8 (confidence: high, kind: float, lanes: c-entities).
    pub field_eb8: f32,
    /// field_ebc (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ebc: u32,
    /// field_ec0 (confidence: high, kind: float, lanes: c-entities).
    pub field_ec0: f32,
    /// field_ec4 (confidence: high, kind: float, lanes: c-entities).
    pub field_ec4: f32,
    /// field_ec8 (confidence: high, kind: float, lanes: c-entities).
    pub field_ec8: f32,
    /// field_ecc (confidence: low, kind: u32, lanes: c-entities).
    pub field_ecc: u32,
    /// field_ed0 (confidence: high, kind: float, lanes: c-entities).
    pub field_ed0: f32,
    /// field_ed4 (confidence: high, kind: float, lanes: c-entities).
    pub field_ed4: f32,
    /// field_ed8 (confidence: high, kind: float, lanes: c-entities).
    pub field_ed8: f32,
    /// Unknown bytes (0xedc..0xee0).
    pub _pad_0edc: [u8; 0x4],
    /// field_ee0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_ee0: u32,
    /// field_ee4 (confidence: high, kind: u32, lanes: c-entities).
    pub field_ee4: u32,
}
assert_size!(CPlayerPed, 0xee8); // merged size 0xee8 rounded to 4
assert_offset!(CPlayerPed, field_b80, 0xb80);
assert_offset!(CPlayerPed, field_e60, 0xe60);
assert_offset!(CPlayerPed, field_e61, 0xe61);
assert_offset!(CPlayerPed, field_e64, 0xe64);
assert_offset!(CPlayerPed, field_e68, 0xe68);
assert_offset!(CPlayerPed, field_e6c, 0xe6c);
assert_offset!(CPlayerPed, field_e70, 0xe70);
assert_offset!(CPlayerPed, field_e94, 0xe94);
assert_offset!(CPlayerPed, field_e95, 0xe95);
assert_offset!(CPlayerPed, field_e9c, 0xe9c);
assert_offset!(CPlayerPed, field_ea0, 0xea0);
assert_offset!(CPlayerPed, field_ea8, 0xea8);
assert_offset!(CPlayerPed, field_eac, 0xeac);
assert_offset!(CPlayerPed, field_eb0, 0xeb0);
assert_offset!(CPlayerPed, field_eb4, 0xeb4);
assert_offset!(CPlayerPed, field_eb8, 0xeb8);
assert_offset!(CPlayerPed, field_ebc, 0xebc);
assert_offset!(CPlayerPed, field_ec0, 0xec0);
assert_offset!(CPlayerPed, field_ec4, 0xec4);
assert_offset!(CPlayerPed, field_ec8, 0xec8);
assert_offset!(CPlayerPed, field_ecc, 0xecc);
assert_offset!(CPlayerPed, field_ed0, 0xed0);
assert_offset!(CPlayerPed, field_ed4, 0xed4);
assert_offset!(CPlayerPed, field_ed8, 0xed8);
assert_offset!(CPlayerPed, field_ee0, 0xee0);
assert_offset!(CPlayerPed, field_ee4, 0xee4);

/// Merged layout for `CTargetting`.
///
/// Size: 0x23d (low). Bases: CExpensiveProcess@0x10.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTargetting {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-entities,c-misc-b).
    pub vftable: Ptr32<()>,
    /// Unknown bytes (0x4..0x20).
    pub _pad_0004: [u8; 0x1c],
    /// field_20 (confidence: medium, kind: flags, lanes: c-entities,c-misc-b).
    pub field_20: u32,
    /// Unknown bytes (0x24..0x224).
    pub _pad_0024: [u8; 0x200],
    /// field_224 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_224: u32,
    /// field_228 (confidence: low, kind: word?, lanes: c-misc-b).
    pub field_228: u16,
    /// Unknown bytes (0x22a..0x22c).
    pub _pad_022a: [u8; 0x2],
    /// field_22c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_22c: u32,
    /// Unknown bytes (0x230..0x234).
    pub _pad_0230: [u8; 0x4],
    /// field_234 (confidence: medium, kind: word?, lanes: c-entities,c-misc-b).
    pub field_234: u16,
    /// Unknown bytes (0x236..0x238).
    pub _pad_0236: [u8; 0x2],
    /// field_238 (confidence: medium, kind: flags, lanes: c-entities,c-misc-b).
    pub field_238: u32,
    /// field_23c (confidence: medium, kind: bool/byte?, lanes: c-entities,c-misc-b).
    pub field_23c: u8,
    /// Unknown trailing bytes (0x23d..0x240).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CTargetting, 0x240); // merged size 0x23d rounded to 4
assert_offset!(CTargetting, vftable, 0x0);
assert_offset!(CTargetting, field_20, 0x20);
assert_offset!(CTargetting, field_224, 0x224);
assert_offset!(CTargetting, field_228, 0x228);
assert_offset!(CTargetting, field_22c, 0x22c);
assert_offset!(CTargetting, field_234, 0x234);
assert_offset!(CTargetting, field_238, 0x238);
assert_offset!(CTargetting, field_23c, 0x23c);

/// Merged layout for `CWantedChain`.
///
/// Size: 0x3d4 (low). Bases: none.
/// Lanes: n-08.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CWantedChain {
    /// Unknown bytes (0x0..0xc).
    pub _pad_0000: [u8; 0xc],
    /// wanted_level_increment (confidence: high, kind: u32, lanes: n-08).
    pub wanted_level_increment: u32,
    /// Unknown bytes (0x10..0x70).
    pub _pad_0010: [u8; 0x60],
    /// wanted_link (confidence: medium, kind: u32, lanes: n-08).
    pub wanted_link: u32,
    /// Unknown bytes (0x74..0x3d0).
    pub _pad_0074: [u8; 0x35c],
    /// flags (confidence: high, kind: flags, lanes: n-08).
    pub flags: u32,
}
assert_size!(CWantedChain, 0x3d4); // merged size 0x3d4 rounded to 4
assert_offset!(CWantedChain, wanted_level_increment, 0xc);
assert_offset!(CWantedChain, wanted_link, 0x70);
assert_offset!(CWantedChain, flags, 0x3d0);
