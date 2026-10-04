//! Cameras and viewports.
//!
//! Holds 5 draft layouts: `CCam` and its subclasses, the script camera instruction record
//! `CCamScriptInstruction`, and `CViewport`. Every layout is Inferred; size confidence (the
//! analysis lanes' own rating) is high for 0, medium for 0 and low for 5. The conventions are those
//! of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CCam`.
///
/// Size: 0x388 (low). Bases: none.
/// Lanes: c-cameras, via:CCamAimWeapon, via:CCamBusted, via:CCamCinematic, via:CCamCinematicCamMan, via:CCamCinematicHeliChase, via:CCamCinematicVehOffset, via:CCamCutscene, via:CCamDebug, via:CCamFinal, via:CCamFollowPed, via:CCamFollowVehicle, via:CCamFpsWeapon, via:CCamFree, via:CCamGame, via:CCamIdle, via:CCamIntermezzo, via:CCamInterp, via:CCamMarket, via:CCamPlayerSettings, via:CCamRadar, via:CCamReplay, via:CCamScript, via:CCamScripted, via:CCamSpline, via:CCamViewFind, via:CCamWasted.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCam {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-cameras).
    pub vfptr: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_10: u32,
    /// field_14 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFree,via:CCamIntermezzo moved from siblings:CCamFree,CCamIntermezzo).
    pub field_14: f32,
    /// field_18 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFree,via:CCamIntermezzo moved from siblings:CCamFree,CCamIntermezzo).
    pub field_18: f32,
    /// Unknown bytes (0x1c..0x24).
    pub _pad_001c: [u8; 0x8],
    /// field_24 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFree,via:CCamInterp moved from siblings:CCamFree,CCamInterp).
    pub field_24: u32,
    /// field_28 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFree,via:CCamInterp moved from siblings:CCamFree,CCamInterp).
    pub field_28: u32,
    /// Unknown bytes (0x2c..0x40).
    pub _pad_002c: [u8; 0x14],
    /// field_40 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFree,via:CCamIdle,via:CCamMarket,via:CCamPlayerSettings,via:CCamRadar,via:CCamScript,via:CCamScripted,via:CCamSpline moved from siblings:CCamFollowVehicle,CCamFree,CCamIdle,CCamMarket,CCamPlayerSettings,CCamRadar,CCamScript,CCamScripted,CCamSpline).
    pub field_40: f32,
    /// field_44 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFree,via:CCamIdle,via:CCamMarket,via:CCamPlayerSettings,via:CCamRadar,via:CCamScript,via:CCamScripted,via:CCamSpline moved from siblings:CCamFollowVehicle,CCamFree,CCamIdle,CCamMarket,CCamPlayerSettings,CCamRadar,CCamScript,CCamScripted,CCamSpline).
    pub field_44: f32,
    /// field_48 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamIdle,via:CCamMarket,via:CCamPlayerSettings,via:CCamRadar,via:CCamScript,via:CCamScripted,via:CCamSpline moved from siblings:CCamFollowVehicle,CCamIdle,CCamMarket,CCamPlayerSettings,CCamRadar,CCamScript,CCamScripted,CCamSpline).
    pub field_48: f32,
    /// field_4c (confidence: medium, kind: float, lanes: c-cameras,via:CCamIdle,via:CCamMarket,via:CCamPlayerSettings,via:CCamRadar,via:CCamScript,via:CCamScripted,via:CCamSpline moved from siblings:CCamIdle,CCamMarket,CCamPlayerSettings,CCamRadar,CCamScript,CCamScripted,CCamSpline).
    pub field_4c: f32,
    /// field_50 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamScripted moved from siblings:CCamFollowVehicle,CCamScripted).
    pub field_50: u32,
    /// field_54 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamScripted moved from siblings:CCamFollowVehicle,CCamScripted).
    pub field_54: f32,
    /// field_58 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamScripted moved from siblings:CCamFollowVehicle,CCamScripted).
    pub field_58: f32,
    /// field_5c (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamScripted moved from siblings:CCamFollowVehicle,CCamScripted).
    pub field_5c: u32,
    /// field_60 (confidence: medium, kind: float, lanes: c-cameras,via:CCamCutscene,via:CCamFollowPed,via:CCamFpsWeapon,via:CCamFree,via:CCamMarket,via:CCamPlayerSettings moved from siblings:CCamCutscene,CCamFollowPed,CCamFpsWeapon,CCamFree,CCamMarket,CCamPlayerSettings).
    pub field_60: f32,
    /// field_64 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamIdle,via:CCamPlayerSettings moved from siblings:CCamIdle,CCamPlayerSettings).
    pub field_64: u32,
    /// Unknown bytes (0x68..0x74).
    pub _pad_0068: [u8; 0xc],
    /// field_74 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamFollowPed,via:CCamFollowVehicle,via:CCamFpsWeapon,via:CCamGame,via:CCamInterp moved from siblings:CCamAimWeapon,CCamFollowPed,CCamFollowVehicle,CCamFpsWeapon,CCamGame,CCamInterp).
    pub field_74: u32,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// field_90 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_90: u32,
    /// Unknown bytes (0x94..0x114).
    pub _pad_0094: [u8; 0x80],
    /// field_114 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamDebug,via:CCamFinal,via:CCamGame,via:CCamRadar moved from siblings:CCamDebug,CCamFinal,CCamGame,CCamRadar).
    pub field_114: u32,
    /// field_118 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamCinematicCamMan,via:CCamCinematicHeliChase,via:CCamCinematicVehOffset,via:CCamFollowPed moved from siblings:CCamAimWeapon,CCamCinematicCamMan,CCamCinematicHeliChase,CCamCinematicVehOffset,CCamFollowPed).
    pub field_118: u32,
    /// Unknown bytes (0x11c..0x12c).
    pub _pad_011c: [u8; 0x10],
    /// field_12c (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamReplay moved from siblings:CCamFollowVehicle,CCamReplay).
    pub field_12c: u32,
    /// field_130 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowPed,via:CCamScripted moved from siblings:CCamFollowPed,CCamScripted).
    pub field_130: u32,
    /// Unknown bytes (0x134..0x13c).
    pub _pad_0134: [u8; 0x8],
    /// field_13c (confidence: high, kind: bool?, lanes: c-cameras,via:CCamIntermezzo,via:CCamPlayerSettings,via:CCamRadar moved from siblings:CCamIntermezzo,CCamPlayerSettings,CCamRadar).
    pub field_13c: u8,
    /// Unknown bytes (0x13d..0x140).
    pub _pad_013d: [u8; 0x3],
    /// field_140 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamBusted,via:CCamCinematicCamMan,via:CCamCinematicHeliChase,via:CCamFollowPed,via:CCamFpsWeapon,via:CCamFree,via:CCamGame,via:CCamIdle,via:CCamMarket,via:CCamScript,via:CCamScripted,via:CCamSpline moved from siblings:CCamBusted,CCamCinematicCamMan,CCamCinematicHeliChase,CCamFollowPed,CCamFpsWeapon,CCamFree,CCamGame,CCamIdle,CCamMarket,CCamScript,CCamScripted,CCamSpline).
    pub field_140: u32,
    /// field_144 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamFpsWeapon,via:CCamFree,via:CCamIntermezzo,via:CCamMarket,via:CCamPlayerSettings,via:CCamReplay,via:CCamScript,via:CCamSpline,via:CCamViewFind,via:CCamWasted moved from siblings:CCamCinematicHeliChase,CCamFpsWeapon,CCamFree,CCamIntermezzo,CCamMarket,CCamPlayerSettings,CCamReplay,CCamScript,CCamSpline,CCamViewFind,CCamWasted).
    pub field_144: u32,
    /// field_148 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamFpsWeapon,via:CCamFree,via:CCamGame,via:CCamIntermezzo,via:CCamInterp,via:CCamMarket,via:CCamPlayerSettings,via:CCamReplay,via:CCamViewFind moved from siblings:CCamCinematicHeliChase,CCamFpsWeapon,CCamFree,CCamGame,CCamIntermezzo,CCamInterp,CCamMarket,CCamPlayerSettings,CCamReplay,CCamViewFind).
    pub field_148: u32,
    /// field_14c (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFpsWeapon,via:CCamFree,via:CCamGame,via:CCamInterp,via:CCamSpline moved from siblings:CCamFpsWeapon,CCamFree,CCamGame,CCamInterp,CCamSpline).
    pub field_14c: u32,
    /// field_150 (confidence: medium, kind: bool?, lanes: c-cameras,via:CCamGame,via:CCamSpline moved from siblings:CCamGame,CCamSpline).
    pub field_150: u8,
    /// Unknown bytes (0x151..0x154).
    pub _pad_0151: [u8; 0x3],
    /// field_154 (confidence: high, kind: float, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamFpsWeapon,via:CCamFree,via:CCamGame,via:CCamInterp,via:CCamSpline moved from siblings:CCamCinematicHeliChase,CCamFpsWeapon,CCamFree,CCamGame,CCamInterp,CCamSpline).
    pub field_154: f32,
    /// field_158 (confidence: high, kind: float, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamFree,via:CCamInterp,via:CCamSpline moved from siblings:CCamCinematicHeliChase,CCamFree,CCamInterp,CCamSpline).
    pub field_158: f32,
    /// field_15c (confidence: high, kind: float, lanes: c-cameras,via:CCamMarket,via:CCamSpline moved from siblings:CCamMarket,CCamSpline).
    pub field_15c: f32,
    /// field_160 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFpsWeapon,via:CCamFree,via:CCamSpline moved from siblings:CCamFollowVehicle,CCamFpsWeapon,CCamFree,CCamSpline).
    pub field_160: u32,
    /// field_164 (confidence: medium, kind: float, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFpsWeapon,via:CCamFree moved from siblings:CCamFollowVehicle,CCamFpsWeapon,CCamFree).
    pub field_164: f32,
    /// field_168 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFpsWeapon,via:CCamFree,via:CCamSpline moved from siblings:CCamFollowVehicle,CCamFpsWeapon,CCamFree,CCamSpline).
    pub field_168: u32,
    /// field_16c (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFree moved from siblings:CCamFollowVehicle,CCamFree).
    pub field_16c: u32,
    /// field_170 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowVehicle,via:CCamFree moved from siblings:CCamFollowVehicle,CCamFree).
    pub field_170: u32,
    /// Unknown bytes (0x174..0x178).
    pub _pad_0174: [u8; 0x4],
    /// field_178 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFree,via:CCamPlayerSettings moved from siblings:CCamFree,CCamPlayerSettings).
    pub field_178: u32,
    /// Unknown bytes (0x17c..0x180).
    pub _pad_017c: [u8; 0x4],
    /// field_180 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamFree moved from siblings:CCamAimWeapon,CCamFree).
    pub field_180: u32,
    /// Unknown bytes (0x184..0x194).
    pub _pad_0184: [u8; 0x10],
    /// field_194 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematic,via:CCamCinematicHeliChase moved from siblings:CCamCinematic,CCamCinematicHeliChase).
    pub field_194: u32,
    /// Unknown bytes (0x198..0x1b0).
    pub _pad_0198: [u8; 0x18],
    /// field_1b0 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamFinal moved from siblings:CCamCinematicHeliChase,CCamFinal).
    pub field_1b0: u32,
    /// Unknown bytes (0x1b4..0x1e0).
    pub _pad_01b4: [u8; 0x2c],
    /// field_1e0 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowPed,via:CCamGame,via:CCamScripted moved from siblings:CCamFollowPed,CCamGame,CCamScripted).
    pub field_1e0: u32,
    /// field_1e4 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowPed,via:CCamGame,via:CCamScripted moved from siblings:CCamFollowPed,CCamGame,CCamScripted).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamGame,via:CCamScripted moved from siblings:CCamGame,CCamScripted).
    pub field_1e8: u32,
    /// Unknown bytes (0x1ec..0x1f0).
    pub _pad_01ec: [u8; 0x4],
    /// field_1f0 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematic,via:CCamCinematicHeliChase,via:CCamReplay,via:CCamScripted moved from siblings:CCamCinematic,CCamCinematicHeliChase,CCamReplay,CCamScripted).
    pub field_1f0: u32,
    /// field_1f4 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematic,via:CCamScripted moved from siblings:CCamCinematic,CCamScripted).
    pub field_1f4: u32,
    /// Unknown bytes (0x1f8..0x204).
    pub _pad_01f8: [u8; 0xc],
    /// field_204 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamScripted moved from siblings:CCamAimWeapon,CCamScripted).
    pub field_204: u32,
    /// Unknown bytes (0x208..0x258).
    pub _pad_0208: [u8; 0x50],
    /// field_258 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamScripted moved from siblings:CCamAimWeapon,CCamScripted).
    pub field_258: u32,
    /// field_25c (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamScripted moved from siblings:CCamAimWeapon,CCamScripted).
    pub field_25c: u32,
    /// Unknown bytes (0x260..0x2a0).
    pub _pad_0260: [u8; 0x40],
    /// field_2a0 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamReplay moved from siblings:CCamCinematicHeliChase,CCamReplay).
    pub field_2a0: u32,
    /// field_2a4 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamCinematicHeliChase,via:CCamReplay moved from siblings:CCamCinematicHeliChase,CCamReplay).
    pub field_2a4: u32,
    /// Unknown bytes (0x2a8..0x2cc).
    pub _pad_02a8: [u8; 0x24],
    /// field_2cc (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamReplay moved from siblings:CCamAimWeapon,CCamReplay).
    pub field_2cc: u32,
    /// Unknown bytes (0x2d0..0x2d4).
    pub _pad_02d0: [u8; 0x4],
    /// field_2d4 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamAimWeapon,via:CCamReplay moved from siblings:CCamAimWeapon,CCamReplay).
    pub field_2d4: u32,
    /// Unknown bytes (0x2d8..0x320).
    pub _pad_02d8: [u8; 0x48],
    /// field_320 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowPed,via:CCamFollowVehicle moved from siblings:CCamFollowPed,CCamFollowVehicle).
    pub field_320: u32,
    /// Unknown bytes (0x324..0x384).
    pub _pad_0324: [u8; 0x60],
    /// field_384 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamFollowPed,via:CCamReplay moved from siblings:CCamFollowPed,CCamReplay).
    pub field_384: u32,
}
assert_size!(CCam, 0x388); // merged size 0x388 rounded to 4
assert_offset!(CCam, vfptr, 0x0);
assert_offset!(CCam, field_10, 0x10);
assert_offset!(CCam, field_14, 0x14);
assert_offset!(CCam, field_18, 0x18);
assert_offset!(CCam, field_24, 0x24);
assert_offset!(CCam, field_28, 0x28);
assert_offset!(CCam, field_40, 0x40);
assert_offset!(CCam, field_44, 0x44);
assert_offset!(CCam, field_48, 0x48);
assert_offset!(CCam, field_4c, 0x4c);
assert_offset!(CCam, field_50, 0x50);
assert_offset!(CCam, field_54, 0x54);
assert_offset!(CCam, field_58, 0x58);
assert_offset!(CCam, field_5c, 0x5c);
assert_offset!(CCam, field_60, 0x60);
assert_offset!(CCam, field_64, 0x64);
assert_offset!(CCam, field_74, 0x74);
assert_offset!(CCam, field_90, 0x90);
assert_offset!(CCam, field_114, 0x114);
assert_offset!(CCam, field_118, 0x118);
assert_offset!(CCam, field_12c, 0x12c);
assert_offset!(CCam, field_130, 0x130);
assert_offset!(CCam, field_13c, 0x13c);
assert_offset!(CCam, field_140, 0x140);
assert_offset!(CCam, field_144, 0x144);
assert_offset!(CCam, field_148, 0x148);
assert_offset!(CCam, field_14c, 0x14c);
assert_offset!(CCam, field_150, 0x150);
assert_offset!(CCam, field_154, 0x154);
assert_offset!(CCam, field_158, 0x158);
assert_offset!(CCam, field_15c, 0x15c);
assert_offset!(CCam, field_160, 0x160);
assert_offset!(CCam, field_164, 0x164);
assert_offset!(CCam, field_168, 0x168);
assert_offset!(CCam, field_16c, 0x16c);
assert_offset!(CCam, field_170, 0x170);
assert_offset!(CCam, field_178, 0x178);
assert_offset!(CCam, field_180, 0x180);
assert_offset!(CCam, field_194, 0x194);
assert_offset!(CCam, field_1b0, 0x1b0);
assert_offset!(CCam, field_1e0, 0x1e0);
assert_offset!(CCam, field_1e4, 0x1e4);
assert_offset!(CCam, field_1e8, 0x1e8);
assert_offset!(CCam, field_1f0, 0x1f0);
assert_offset!(CCam, field_1f4, 0x1f4);
assert_offset!(CCam, field_204, 0x204);
assert_offset!(CCam, field_258, 0x258);
assert_offset!(CCam, field_25c, 0x25c);
assert_offset!(CCam, field_2a0, 0x2a0);
assert_offset!(CCam, field_2a4, 0x2a4);
assert_offset!(CCam, field_2cc, 0x2cc);
assert_offset!(CCam, field_2d4, 0x2d4);
assert_offset!(CCam, field_320, 0x320);
assert_offset!(CCam, field_384, 0x384);

/// Merged layout for `CCamAimWeapon`.
///
/// Size: 0x2dc (low). Bases: CCam@0x0.
/// Lanes: c-cameras.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCamAimWeapon {
    /// Unknown bytes (0x0..0x1a8).
    pub _pad_0000: [u8; 0x1a8],
    /// field_1a8 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_1a8: u8,
    /// Unknown bytes (0x1a9..0x200).
    pub _pad_01a9: [u8; 0x57],
    /// field_200 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_200: u8,
    /// Unknown bytes (0x201..0x210).
    pub _pad_0201: [u8; 0xf],
    /// field_210 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_210: u8,
    /// field_211 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_211: u8,
    /// field_212 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_212: u8,
    /// field_213 (confidence: medium, kind: bool?, lanes: c-cameras).
    pub field_213: u8,
    /// field_214 (confidence: medium, kind: bool?, lanes: c-cameras).
    pub field_214: u8,
    /// field_215 (confidence: medium, kind: bool?, lanes: c-cameras).
    pub field_215: u8,
    /// Unknown bytes (0x216..0x26c).
    pub _pad_0216: [u8; 0x56],
    /// field_26c (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_26c: u32,
    /// field_270 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_270: u32,
    /// Unknown bytes (0x274..0x2c4).
    pub _pad_0274: [u8; 0x50],
    /// field_2c4 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_2c4: u8,
    /// Unknown bytes (0x2c5..0x2d0).
    pub _pad_02c5: [u8; 0xb],
    /// field_2d0 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_2d0: u32,
    /// Unknown bytes (0x2d4..0x2d8).
    pub _pad_02d4: [u8; 0x4],
    /// field_2d8 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_2d8: u8,
    /// field_2d9 (confidence: low, kind: bool?, lanes: c-cameras).
    pub field_2d9: u8,
    /// Unknown trailing bytes (0x2da..0x2dc).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CCamAimWeapon, 0x2dc); // merged size 0x2dc rounded to 4
assert_offset!(CCamAimWeapon, field_1a8, 0x1a8);
assert_offset!(CCamAimWeapon, field_200, 0x200);
assert_offset!(CCamAimWeapon, field_210, 0x210);
assert_offset!(CCamAimWeapon, field_211, 0x211);
assert_offset!(CCamAimWeapon, field_212, 0x212);
assert_offset!(CCamAimWeapon, field_213, 0x213);
assert_offset!(CCamAimWeapon, field_214, 0x214);
assert_offset!(CCamAimWeapon, field_215, 0x215);
assert_offset!(CCamAimWeapon, field_26c, 0x26c);
assert_offset!(CCamAimWeapon, field_270, 0x270);
assert_offset!(CCamAimWeapon, field_2c4, 0x2c4);
assert_offset!(CCamAimWeapon, field_2d0, 0x2d0);
assert_offset!(CCamAimWeapon, field_2d8, 0x2d8);
assert_offset!(CCamAimWeapon, field_2d9, 0x2d9);

/// Merged layout for `CCamScriptInstruction`.
///
/// Size: 0x24 (low). Bases: none.
/// Lanes: c-cameras, via:CCamScriptInstruction_ActivateCam, via:CCamScriptInstruction_ActivateViewport, via:CCamScriptInstruction_AddInterpCustomSpeedGraphMarker, via:CCamScriptInstruction_AddPedToCinematographyAI, via:CCamScriptInstruction_AttachCamToViewport, via:CCamScriptInstruction_CamProcess, via:CCamScriptInstruction_DestroyAllCams, via:CCamScriptInstruction_DestroyCam, via:CCamScriptInstruction_DisableIntermezzoCams, via:CCamScriptInstruction_DoFadeIn_Q, via:CCamScriptInstruction_DoFadeIn_UNHACKED_Q, via:CCamScriptInstruction_DoFadeOut_Q, via:CCamScriptInstruction_DoFadeOut_UNHACKED_Q, via:CCamScriptInstruction_EnableCamCollision, via:CCamScriptInstruction_EnableDebugCam, via:CCamScriptInstruction_ForceTelescopeCam, via:CCamScriptInstruction_InheritRoll, via:CCamScriptInstruction_InsertSplineNode, via:CCamScriptInstruction_InterpolateToGameCam, via:CCamScriptInstruction_InterpolateToScriptCam, via:CCamScriptInstruction_PointCamAtObj_Q, via:CCamScriptInstruction_PointCamAtPed_Q, via:CCamScriptInstruction_PointCamAtVehicle_Q, via:CCamScriptInstruction_PropagateCam, via:CCamScriptInstruction_ResetCamSplineCustomSpeedGraph, via:CCamScriptInstruction_ResetInterpCustomSpeedGraph, via:CCamScriptInstruction_RestoreJumpCut_Q, via:CCamScriptInstruction_Restore_Q, via:CCamScriptInstruction_SequenceEmpty, via:CCamScriptInstruction_SequenceStartProcessing, via:CCamScriptInstruction_SequenceStopProcessing, via:CCamScriptInstruction_SequenceWait, via:CCamScriptInstruction_SetCamBehindPed_Q, via:CCamScriptInstruction_SetCamInFrontPed_Q, via:CCamScriptInstruction_SetCamLookAtPos_Q, via:CCamScriptInstruction_SetCamPos_Q, via:CCamScriptInstruction_SetCamRot_Q, via:CCamScriptInstruction_SetCamSplineCustomSpeedGraph, via:CCamScriptInstruction_SetCamTargetPed, via:CCamScriptInstruction_SetCameraControlsDisabledWithPlayerControls, via:CCamScriptInstruction_SetCinematicButtonEnabled, via:CCamScriptInstruction_SetCinematicCam_Q, via:CCamScriptInstruction_SetCollideWithPeds, via:CCamScriptInstruction_SetDollyZoomLock, via:CCamScriptInstruction_SetDrunkCam, via:CCamScriptInstruction_SetFOV, via:CCamScriptInstruction_SetFar, via:CCamScriptInstruction_SetFarDOF, via:CCamScriptInstruction_SetFollowPedPitchLimitDown, via:CCamScriptInstruction_SetFollowPedPitchLimitUp, via:CCamScriptInstruction_SetFollowVehicleCamOffset, via:CCamScriptInstruction_SetFollowVehiclePitchLimitDown, via:CCamScriptInstruction_SetFollowVehiclePitchLimitUp, via:CCamScriptInstruction_SetGameCameraControlsActive, via:CCamScriptInstruction_SetGameFollowVehicleCamSubMode, via:CCamScriptInstruction_SetHintAdvancedParams, via:CCamScriptInstruction_SetHintFOV, via:CCamScriptInstruction_SetHintMoveInDist, via:CCamScriptInstruction_SetHintMoveInDistDefault, via:CCamScriptInstruction_SetHintTimes, via:CCamScriptInstruction_SetHintTimesDefault, via:CCamScriptInstruction_SetInterpDetail_Rot_Style_Angles, via:CCamScriptInstruction_SetInterpDetail_Rot_Style_Quats, via:CCamScriptInstruction_SetInterpGraphTypePos, via:CCamScriptInstruction_SetInterpGraphTypeRot, via:CCamScriptInstruction_SetInterpStateSrc, via:CCamScriptInstruction_SetInterpStyleCore, via:CCamScriptInstruction_SetInterpStyleDetailed, via:CCamScriptInstruction_SetInterpolationTime_Q, via:CCamScriptInstruction_SetLookDampingParams, via:CCamScriptInstruction_SetLookTargetCam, via:CCamScriptInstruction_SetLookTargetEntity, via:CCamScriptInstruction_SetLookTargetOffset, via:CCamScriptInstruction_SetLookTargetOffsetRelative, via:CCamScriptInstruction_SetLookTargetPos, via:CCamScriptInstruction_SetMotionBlur, via:CCamScriptInstruction_SetName, via:CCamScriptInstruction_SetNear, via:CCamScriptInstruction_SetNearDOF, via:CCamScriptInstruction_SetPos, via:CCamScriptInstruction_SetPosTargetEntity, via:CCamScriptInstruction_SetPosTargetOffset, via:CCamScriptInstruction_SetPosTargetOffsetRelative, via:CCamScriptInstruction_SetProstituteCamActive, via:CCamScriptInstruction_SetRoll, via:CCamScriptInstruction_SetRot, via:CCamScriptInstruction_SetRotOrder, via:CCamScriptInstruction_SetScreenFade, via:CCamScriptInstruction_SetScriptModeActive, via:CCamScriptInstruction_SetShake, via:CCamScriptInstruction_SetSniperZoomFactor, via:CCamScriptInstruction_SetSplineDuration, via:CCamScriptInstruction_SetSplineProgress, via:CCamScriptInstruction_SetSplineSpeedConstant, via:CCamScriptInstruction_SetSplineSpeedGraph, via:CCamScriptInstruction_SetStockShake, via:CCamScriptInstruction_SetTelescopeCamAngleLimits, via:CCamScriptInstruction_SetViewport, via:CCamScriptInstruction_SetViewportDest, via:CCamScriptInstruction_SetViewportMirrored, via:CCamScriptInstruction_SetViewportPriority, via:CCamScriptInstruction_SetViewportShape, via:CCamScriptInstruction_SetWidescreenBorders, via:CCamScriptInstruction_SetWidescreenBordersInstant, via:CCamScriptInstruction_SnapShotCam, via:CCamScriptInstruction_UnAttachCam, via:CCamScriptInstruction_UnAttachCamFromViewport, via:CCamScriptInstruction_UnInheritRollCam, via:CCamScriptInstruction_UnPointCam.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCamScriptInstruction {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-cameras).
    pub vfptr: Ptr32<()>,
    /// flags_or_state? (confidence: high, kind: bool?, lanes: c-cameras,via:CCamScriptInstruction_ActivateCam,via:CCamScriptInstruction_ActivateViewport,via:CCamScriptInstruction_AddInterpCustomSpeedGraphMarker,via:CCamScriptInstruction_AddPedToCinematographyAI,via:CCamScriptInstruction_AttachCamToViewport,via:CCamScriptInstruction_CamProcess,via:CCamScriptInstruction_DestroyAllCams,via:CCamScriptInstruction_DestroyCam,via:CCamScriptInstruction_DisableIntermezzoCams,via:CCamScriptInstruction_DoFadeIn_Q,via:CCamScriptInstruction_DoFadeIn_UNHACKED_Q,via:CCamScriptInstruction_DoFadeOut_Q,via:CCamScriptInstruction_DoFadeOut_UNHACKED_Q,via:CCamScriptInstruction_EnableCamCollision,via:CCamScriptInstruction_EnableDebugCam,via:CCamScriptInstruction_ForceTelescopeCam,via:CCamScriptInstruction_InheritRoll,via:CCamScriptInstruction_InsertSplineNode,via:CCamScriptInstruction_InterpolateToGameCam,via:CCamScriptInstruction_InterpolateToScriptCam,via:CCamScriptInstruction_PointCamAtObj_Q,via:CCamScriptInstruction_PointCamAtPed_Q,via:CCamScriptInstruction_PointCamAtVehicle_Q,via:CCamScriptInstruction_PropagateCam,via:CCamScriptInstruction_ResetCamSplineCustomSpeedGraph,via:CCamScriptInstruction_ResetInterpCustomSpeedGraph,via:CCamScriptInstruction_RestoreJumpCut_Q,via:CCamScriptInstruction_Restore_Q,via:CCamScriptInstruction_SequenceEmpty,via:CCamScriptInstruction_SequenceStartProcessing,via:CCamScriptInstruction_SequenceStopProcessing,via:CCamScriptInstruction_SequenceWait,via:CCamScriptInstruction_SetCamBehindPed_Q,via:CCamScriptInstruction_SetCamInFrontPed_Q,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetCamSplineCustomSpeedGraph,via:CCamScriptInstruction_SetCamTargetPed,via:CCamScriptInstruction_SetCameraControlsDisabledWithPlayerControls,via:CCamScriptInstruction_SetCinematicButtonEnabled,via:CCamScriptInstruction_SetCinematicCam_Q,via:CCamScriptInstruction_SetCollideWithPeds,via:CCamScriptInstruction_SetDollyZoomLock,via:CCamScriptInstruction_SetDrunkCam,via:CCamScriptInstruction_SetFOV,via:CCamScriptInstruction_SetFar,via:CCamScriptInstruction_SetFarDOF,via:CCamScriptInstruction_SetFollowPedPitchLimitDown,via:CCamScriptInstruction_SetFollowPedPitchLimitUp,via:CCamScriptInstruction_SetFollowVehicleCamOffset,via:CCamScriptInstruction_SetFollowVehiclePitchLimitDown,via:CCamScriptInstruction_SetFollowVehiclePitchLimitUp,via:CCamScriptInstruction_SetGameCameraControlsActive,via:CCamScriptInstruction_SetGameFollowVehicleCamSubMode,via:CCamScriptInstruction_SetHintAdvancedParams,via:CCamScriptInstruction_SetHintFOV,via:CCamScriptInstruction_SetHintMoveInDist,via:CCamScriptInstruction_SetHintMoveInDistDefault,via:CCamScriptInstruction_SetHintTimes,via:CCamScriptInstruction_SetHintTimesDefault,via:CCamScriptInstruction_SetInterpDetail_Rot_Style_Angles,via:CCamScriptInstruction_SetInterpDetail_Rot_Style_Quats,via:CCamScriptInstruction_SetInterpGraphTypePos,via:CCamScriptInstruction_SetInterpGraphTypeRot,via:CCamScriptInstruction_SetInterpStateSrc,via:CCamScriptInstruction_SetInterpStyleCore,via:CCamScriptInstruction_SetInterpStyleDetailed,via:CCamScriptInstruction_SetInterpolationTime_Q,via:CCamScriptInstruction_SetLookDampingParams,via:CCamScriptInstruction_SetLookTargetCam,via:CCamScriptInstruction_SetLookTargetEntity,via:CCamScriptInstruction_SetLookTargetOffset,via:CCamScriptInstruction_SetLookTargetOffsetRelative,via:CCamScriptInstruction_SetLookTargetPos,via:CCamScriptInstruction_SetMotionBlur,via:CCamScriptInstruction_SetName,via:CCamScriptInstruction_SetNear,via:CCamScriptInstruction_SetNearDOF,via:CCamScriptInstruction_SetPos,via:CCamScriptInstruction_SetPosTargetEntity,via:CCamScriptInstruction_SetPosTargetOffset,via:CCamScriptInstruction_SetPosTargetOffsetRelative,via:CCamScriptInstruction_SetProstituteCamActive,via:CCamScriptInstruction_SetRoll,via:CCamScriptInstruction_SetRot,via:CCamScriptInstruction_SetRotOrder,via:CCamScriptInstruction_SetScreenFade,via:CCamScriptInstruction_SetScriptModeActive,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetSniperZoomFactor,via:CCamScriptInstruction_SetSplineDuration,via:CCamScriptInstruction_SetSplineProgress,via:CCamScriptInstruction_SetSplineSpeedConstant,via:CCamScriptInstruction_SetSplineSpeedGraph,via:CCamScriptInstruction_SetStockShake,via:CCamScriptInstruction_SetTelescopeCamAngleLimits,via:CCamScriptInstruction_SetViewport,via:CCamScriptInstruction_SetViewportDest,via:CCamScriptInstruction_SetViewportMirrored,via:CCamScriptInstruction_SetViewportPriority,via:CCamScriptInstruction_SetViewportShape,via:CCamScriptInstruction_SetWidescreenBorders,via:CCamScriptInstruction_SetWidescreenBordersInstant,via:CCamScriptInstruction_SnapShotCam,via:CCamScriptInstruction_UnAttachCam,via:CCamScriptInstruction_UnAttachCamFromViewport,via:CCamScriptInstruction_UnInheritRollCam,via:CCamScriptInstruction_UnPointCam moved from siblings:CCamScriptInstruction_ActivateCam,CCamScriptInstruction_ActivateViewport,CCamScriptInstruction_AddInterpCustomSpeedGraphMarker,CCamScriptInstruction_AddPedToCinematographyAI,CCamScriptInstruction_AttachCamToViewport,CCamScriptInstruction_CamProcess,CCamScriptInstruction_DestroyAllCams,CCamScriptInstruction_DestroyCam,CCamScriptInstruction_DisableIntermezzoCams,CCamScriptInstruction_DoFadeIn_Q,CCamScriptInstruction_DoFadeIn_UNHACKED_Q,CCamScriptInstruction_DoFadeOut_Q,CCamScriptInstruction_DoFadeOut_UNHACKED_Q,CCamScriptInstruction_EnableCamCollision,CCamScriptInstruction_EnableDebugCam,CCamScriptInstruction_ForceTelescopeCam,CCamScriptInstruction_InheritRoll,CCamScriptInstruction_InsertSplineNode,CCamScriptInstruction_InterpolateToGameCam,CCamScriptInstruction_InterpolateToScriptCam,CCamScriptInstruction_PointCamAtObj_Q,CCamScriptInstruction_PointCamAtPed_Q,CCamScriptInstruction_PointCamAtVehicle_Q,CCamScriptInstruction_PropagateCam,CCamScriptInstruction_ResetCamSplineCustomSpeedGraph,CCamScriptInstruction_ResetInterpCustomSpeedGraph,CCamScriptInstruction_RestoreJumpCut_Q,CCamScriptInstruction_Restore_Q,CCamScriptInstruction_SequenceEmpty,CCamScriptInstruction_SequenceStartProcessing,CCamScriptInstruction_SequenceStopProcessing,CCamScriptInstruction_SequenceWait,CCamScriptInstruction_SetCamBehindPed_Q,CCamScriptInstruction_SetCamInFrontPed_Q,CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetCamSplineCustomSpeedGraph,CCamScriptInstruction_SetCamTargetPed,CCamScriptInstruction_SetCameraControlsDisabledWithPlayerControls,CCamScriptInstruction_SetCinematicButtonEnabled,CCamScriptInstruction_SetCinematicCam_Q,CCamScriptInstruction_SetCollideWithPeds,CCamScriptInstruction_SetDollyZoomLock,CCamScriptInstruction_SetDrunkCam,CCamScriptInstruction_SetFOV,CCamScriptInstruction_SetFar,CCamScriptInstruction_SetFarDOF,CCamScriptInstruction_SetFollowPedPitchLimitDown,CCamScriptInstruction_SetFollowPedPitchLimitUp,CCamScriptInstruction_SetFollowVehicleCamOffset,CCamScriptInstruction_SetFollowVehiclePitchLimitDown,CCamScriptInstruction_SetFollowVehiclePitchLimitUp,CCamScriptInstruction_SetGameCameraControlsActive,CCamScriptInstruction_SetGameFollowVehicleCamSubMode,CCamScriptInstruction_SetHintAdvancedParams,CCamScriptInstruction_SetHintFOV,CCamScriptInstruction_SetHintMoveInDist,CCamScriptInstruction_SetHintMoveInDistDefault,CCamScriptInstruction_SetHintTimes,CCamScriptInstruction_SetHintTimesDefault,CCamScriptInstruction_SetInterpDetail_Rot_Style_Angles,CCamScriptInstruction_SetInterpDetail_Rot_Style_Quats,CCamScriptInstruction_SetInterpGraphTypePos,CCamScriptInstruction_SetInterpGraphTypeRot,CCamScriptInstruction_SetInterpStateSrc,CCamScriptInstruction_SetInterpStyleCore,CCamScriptInstruction_SetInterpStyleDetailed,CCamScriptInstruction_SetInterpolationTime_Q,CCamScriptInstruction_SetLookDampingParams,CCamScriptInstruction_SetLookTargetCam,CCamScriptInstruction_SetLookTargetEntity,CCamScriptInstruction_SetLookTargetOffset,CCamScriptInstruction_SetLookTargetOffsetRelative,CCamScriptInstruction_SetLookTargetPos,CCamScriptInstruction_SetMotionBlur,CCamScriptInstruction_SetName,CCamScriptInstruction_SetNear,CCamScriptInstruction_SetNearDOF,CCamScriptInstruction_SetPos,CCamScriptInstruction_SetPosTargetEntity,CCamScriptInstruction_SetPosTargetOffset,CCamScriptInstruction_SetPosTargetOffsetRelative,CCamScriptInstruction_SetProstituteCamActive,CCamScriptInstruction_SetRoll,CCamScriptInstruction_SetRot,CCamScriptInstruction_SetRotOrder,CCamScriptInstruction_SetScreenFade,CCamScriptInstruction_SetScriptModeActive,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetSniperZoomFactor,CCamScriptInstruction_SetSplineDuration,CCamScriptInstruction_SetSplineProgress,CCamScriptInstruction_SetSplineSpeedConstant,CCamScriptInstruction_SetSplineSpeedGraph,CCamScriptInstruction_SetStockShake,CCamScriptInstruction_SetTelescopeCamAngleLimits,CCamScriptInstruction_SetViewport,CCamScriptInstruction_SetViewportDest,CCamScriptInstruction_SetViewportMirrored,CCamScriptInstruction_SetViewportPriority,CCamScriptInstruction_SetViewportShape,CCamScriptInstruction_SetWidescreenBorders,CCamScriptInstruction_SetWidescreenBordersInstant,CCamScriptInstruction_SnapShotCam,CCamScriptInstruction_UnAttachCam,CCamScriptInstruction_UnAttachCamFromViewport,CCamScriptInstruction_UnInheritRollCam,CCamScriptInstruction_UnPointCam).
    pub flags_or_state: u8,
    /// Unknown bytes (0x5..0x8).
    pub _pad_0005: [u8; 0x3],
    /// field_8 (confidence: high, kind: bool?, lanes: c-cameras,via:CCamScriptInstruction_EnableDebugCam,via:CCamScriptInstruction_ForceTelescopeCam,via:CCamScriptInstruction_InterpolateToGameCam,via:CCamScriptInstruction_InterpolateToScriptCam,via:CCamScriptInstruction_SetCameraControlsDisabledWithPlayerControls,via:CCamScriptInstruction_SetCinematicButtonEnabled,via:CCamScriptInstruction_SetCollideWithPeds,via:CCamScriptInstruction_SetGameCameraControlsActive,via:CCamScriptInstruction_SetProstituteCamActive,via:CCamScriptInstruction_SetScriptModeActive,via:CCamScriptInstruction_SetWidescreenBorders,via:CCamScriptInstruction_SetWidescreenBordersInstant moved from siblings:CCamScriptInstruction_EnableDebugCam,CCamScriptInstruction_ForceTelescopeCam,CCamScriptInstruction_InterpolateToGameCam,CCamScriptInstruction_InterpolateToScriptCam,CCamScriptInstruction_SetCameraControlsDisabledWithPlayerControls,CCamScriptInstruction_SetCinematicButtonEnabled,CCamScriptInstruction_SetCollideWithPeds,CCamScriptInstruction_SetGameCameraControlsActive,CCamScriptInstruction_SetProstituteCamActive,CCamScriptInstruction_SetScriptModeActive,CCamScriptInstruction_SetWidescreenBorders,CCamScriptInstruction_SetWidescreenBordersInstant).
    pub field_8: u8,
    /// Unknown bytes (0x9..0xc).
    pub _pad_0009: [u8; 0x3],
    /// field_c (confidence: high, kind: bool?, lanes: c-cameras,via:CCamScriptInstruction_ActivateCam,via:CCamScriptInstruction_ActivateViewport,via:CCamScriptInstruction_EnableCamCollision,via:CCamScriptInstruction_PropagateCam,via:CCamScriptInstruction_SetDollyZoomLock,via:CCamScriptInstruction_SetLookTargetOffsetRelative,via:CCamScriptInstruction_SetPosTargetOffsetRelative,via:CCamScriptInstruction_SetSplineSpeedConstant,via:CCamScriptInstruction_SetViewportMirrored moved from siblings:CCamScriptInstruction_ActivateCam,CCamScriptInstruction_ActivateViewport,CCamScriptInstruction_EnableCamCollision,CCamScriptInstruction_PropagateCam,CCamScriptInstruction_SetDollyZoomLock,CCamScriptInstruction_SetLookTargetOffsetRelative,CCamScriptInstruction_SetPosTargetOffsetRelative,CCamScriptInstruction_SetSplineSpeedConstant,CCamScriptInstruction_SetViewportMirrored).
    pub field_c: u8,
    /// Unknown bytes (0xd..0x10).
    pub _pad_000d: [u8; 0x3],
    /// field_10 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamScriptInstruction_SequenceWait,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetDrunkCam,via:CCamScriptInstruction_SetFollowVehicleCamOffset,via:CCamScriptInstruction_SetHintAdvancedParams,via:CCamScriptInstruction_SetHintTimes,via:CCamScriptInstruction_SetInterpStyleCore,via:CCamScriptInstruction_SetInterpStyleDetailed,via:CCamScriptInstruction_SetLookDampingParams,via:CCamScriptInstruction_SetLookTargetOffset,via:CCamScriptInstruction_SetLookTargetPos,via:CCamScriptInstruction_SetPos,via:CCamScriptInstruction_SetPosTargetOffset,via:CCamScriptInstruction_SetRot,via:CCamScriptInstruction_SetScreenFade,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetStockShake,via:CCamScriptInstruction_SetTelescopeCamAngleLimits,via:CCamScriptInstruction_SetViewport,via:CCamScriptInstruction_SetViewportDest moved from siblings:CCamScriptInstruction_SequenceWait,CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetDrunkCam,CCamScriptInstruction_SetFollowVehicleCamOffset,CCamScriptInstruction_SetHintAdvancedParams,CCamScriptInstruction_SetHintTimes,CCamScriptInstruction_SetInterpStyleCore,CCamScriptInstruction_SetInterpStyleDetailed,CCamScriptInstruction_SetLookDampingParams,CCamScriptInstruction_SetLookTargetOffset,CCamScriptInstruction_SetLookTargetPos,CCamScriptInstruction_SetPos,CCamScriptInstruction_SetPosTargetOffset,CCamScriptInstruction_SetRot,CCamScriptInstruction_SetScreenFade,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetStockShake,CCamScriptInstruction_SetTelescopeCamAngleLimits,CCamScriptInstruction_SetViewport,CCamScriptInstruction_SetViewportDest).
    pub field_10: u32,
    /// field_14 (confidence: high, kind: float, lanes: c-cameras,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetFollowVehicleCamOffset,via:CCamScriptInstruction_SetHintAdvancedParams,via:CCamScriptInstruction_SetInterpStyleCore,via:CCamScriptInstruction_SetInterpStyleDetailed,via:CCamScriptInstruction_SetLookDampingParams,via:CCamScriptInstruction_SetLookTargetOffset,via:CCamScriptInstruction_SetLookTargetPos,via:CCamScriptInstruction_SetPos,via:CCamScriptInstruction_SetPosTargetOffset,via:CCamScriptInstruction_SetRot,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetTelescopeCamAngleLimits,via:CCamScriptInstruction_SetViewport,via:CCamScriptInstruction_SetViewportDest moved from siblings:CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetFollowVehicleCamOffset,CCamScriptInstruction_SetHintAdvancedParams,CCamScriptInstruction_SetInterpStyleCore,CCamScriptInstruction_SetInterpStyleDetailed,CCamScriptInstruction_SetLookDampingParams,CCamScriptInstruction_SetLookTargetOffset,CCamScriptInstruction_SetLookTargetPos,CCamScriptInstruction_SetPos,CCamScriptInstruction_SetPosTargetOffset,CCamScriptInstruction_SetRot,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetTelescopeCamAngleLimits,CCamScriptInstruction_SetViewport,CCamScriptInstruction_SetViewportDest).
    pub field_14: f32,
    /// field_18 (confidence: high, kind: float, lanes: c-cameras,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetFollowVehicleCamOffset,via:CCamScriptInstruction_SetInterpStyleCore,via:CCamScriptInstruction_SetInterpStyleDetailed,via:CCamScriptInstruction_SetLookTargetOffset,via:CCamScriptInstruction_SetLookTargetPos,via:CCamScriptInstruction_SetPos,via:CCamScriptInstruction_SetPosTargetOffset,via:CCamScriptInstruction_SetRot,via:CCamScriptInstruction_SetScreenFade,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetTelescopeCamAngleLimits,via:CCamScriptInstruction_SetViewport,via:CCamScriptInstruction_SetViewportDest moved from siblings:CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetFollowVehicleCamOffset,CCamScriptInstruction_SetInterpStyleCore,CCamScriptInstruction_SetInterpStyleDetailed,CCamScriptInstruction_SetLookTargetOffset,CCamScriptInstruction_SetLookTargetPos,CCamScriptInstruction_SetPos,CCamScriptInstruction_SetPosTargetOffset,CCamScriptInstruction_SetRot,CCamScriptInstruction_SetScreenFade,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetTelescopeCamAngleLimits,CCamScriptInstruction_SetViewport,CCamScriptInstruction_SetViewportDest).
    pub field_18: f32,
    /// field_1c (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetFollowVehicleCamOffset,via:CCamScriptInstruction_SetLookTargetOffset,via:CCamScriptInstruction_SetLookTargetPos,via:CCamScriptInstruction_SetPos,via:CCamScriptInstruction_SetPosTargetOffset,via:CCamScriptInstruction_SetRot,via:CCamScriptInstruction_SetScreenFade,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetTelescopeCamAngleLimits,via:CCamScriptInstruction_SetViewportDest moved from siblings:CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetFollowVehicleCamOffset,CCamScriptInstruction_SetLookTargetOffset,CCamScriptInstruction_SetLookTargetPos,CCamScriptInstruction_SetPos,CCamScriptInstruction_SetPosTargetOffset,CCamScriptInstruction_SetRot,CCamScriptInstruction_SetScreenFade,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetTelescopeCamAngleLimits,CCamScriptInstruction_SetViewportDest).
    pub field_1c: u32,
    /// field_20 (confidence: high, kind: u32_or_ptr, lanes: c-cameras,via:CCamScriptInstruction_SetCamLookAtPos_Q,via:CCamScriptInstruction_SetCamRot_Q,via:CCamScriptInstruction_SetScreenFade,via:CCamScriptInstruction_SetShake,via:CCamScriptInstruction_SetViewportDest moved from siblings:CCamScriptInstruction_SetCamLookAtPos_Q,CCamScriptInstruction_SetCamRot_Q,CCamScriptInstruction_SetScreenFade,CCamScriptInstruction_SetShake,CCamScriptInstruction_SetViewportDest).
    pub field_20: u32,
}
assert_size!(CCamScriptInstruction, 0x24); // merged size 0x24 rounded to 4
assert_offset!(CCamScriptInstruction, vfptr, 0x0);
assert_offset!(CCamScriptInstruction, flags_or_state, 0x4);
assert_offset!(CCamScriptInstruction, field_8, 0x8);
assert_offset!(CCamScriptInstruction, field_c, 0xc);
assert_offset!(CCamScriptInstruction, field_10, 0x10);
assert_offset!(CCamScriptInstruction, field_14, 0x14);
assert_offset!(CCamScriptInstruction, field_18, 0x18);
assert_offset!(CCamScriptInstruction, field_1c, 0x1c);
assert_offset!(CCamScriptInstruction, field_20, 0x20);

/// Merged layout for `CCamScripted`.
///
/// Size: 0x268 (low). Bases: CCam@0x0.
/// Lanes: c-cameras.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCamScripted {
    /// Unknown bytes (0x0..0x1ec).
    pub _pad_0000: [u8; 0x1ec],
    /// field_1ec (confidence: low, kind: float, lanes: c-cameras).
    pub field_1ec: f32,
    /// Unknown bytes (0x1f0..0x1f8).
    pub _pad_01f0: [u8; 0x8],
    /// field_1f8 (confidence: low, kind: float, lanes: c-cameras).
    pub field_1f8: f32,
    /// Unknown bytes (0x1fc..0x200).
    pub _pad_01fc: [u8; 0x4],
    /// field_200 (confidence: low, kind: float, lanes: c-cameras).
    pub field_200: f32,
    /// Unknown bytes (0x204..0x208).
    pub _pad_0204: [u8; 0x4],
    /// field_208 (confidence: low, kind: float, lanes: c-cameras).
    pub field_208: f32,
    /// Unknown bytes (0x20c..0x210).
    pub _pad_020c: [u8; 0x4],
    /// field_210 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras).
    pub field_210: u32,
    /// field_214 (confidence: medium, kind: float, lanes: c-cameras).
    pub field_214: f32,
    /// field_218 (confidence: medium, kind: float, lanes: c-cameras).
    pub field_218: f32,
    /// field_21c (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_21c: u32,
    /// field_220 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_220: u32,
    /// field_224 (confidence: low, kind: float, lanes: c-cameras).
    pub field_224: f32,
    /// field_228 (confidence: low, kind: float, lanes: c-cameras).
    pub field_228: f32,
    /// field_22c (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_22c: u32,
    /// field_230 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_230: u32,
    /// field_234 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_234: u32,
    /// field_238 (confidence: medium, kind: u32_or_ptr, lanes: c-cameras).
    pub field_238: u32,
    /// Unknown bytes (0x23c..0x240).
    pub _pad_023c: [u8; 0x4],
    /// field_240 (confidence: high, kind: u32_or_ptr, lanes: c-cameras).
    pub field_240: u32,
    /// field_244 (confidence: high, kind: u32_or_ptr, lanes: c-cameras).
    pub field_244: u32,
    /// field_248 (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_248: u32,
    /// field_24c (confidence: low, kind: u32_or_ptr, lanes: c-cameras).
    pub field_24c: u32,
    /// Unknown bytes (0x250..0x260).
    pub _pad_0250: [u8; 0x10],
    /// field_260 (confidence: medium, kind: float, lanes: c-cameras).
    pub field_260: f32,
    /// field_264 (confidence: high, kind: bool?, lanes: c-cameras).
    pub field_264: u8,
    /// Unknown trailing bytes (0x265..0x268).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CCamScripted, 0x268); // merged size 0x268 rounded to 4
assert_offset!(CCamScripted, field_1ec, 0x1ec);
assert_offset!(CCamScripted, field_1f8, 0x1f8);
assert_offset!(CCamScripted, field_200, 0x200);
assert_offset!(CCamScripted, field_208, 0x208);
assert_offset!(CCamScripted, field_210, 0x210);
assert_offset!(CCamScripted, field_214, 0x214);
assert_offset!(CCamScripted, field_218, 0x218);
assert_offset!(CCamScripted, field_21c, 0x21c);
assert_offset!(CCamScripted, field_220, 0x220);
assert_offset!(CCamScripted, field_224, 0x224);
assert_offset!(CCamScripted, field_228, 0x228);
assert_offset!(CCamScripted, field_22c, 0x22c);
assert_offset!(CCamScripted, field_230, 0x230);
assert_offset!(CCamScripted, field_234, 0x234);
assert_offset!(CCamScripted, field_238, 0x238);
assert_offset!(CCamScripted, field_240, 0x240);
assert_offset!(CCamScripted, field_244, 0x244);
assert_offset!(CCamScripted, field_248, 0x248);
assert_offset!(CCamScripted, field_24c, 0x24c);
assert_offset!(CCamScripted, field_260, 0x260);
assert_offset!(CCamScripted, field_264, 0x264);

/// Merged layout for `CViewport`.
///
/// Size: 0x559 (low). Bases: none.
/// Lanes: c-misc-b, via:CViewport3DScene, via:CViewportFrontend3DScene, via:CViewportMobilePhone, via:CViewportPrimaryOrtho, via:CViewportRadar.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CViewport {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// field_10 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_10: u32,
    /// Unknown bytes (0x14..0x2c0).
    pub _pad_0014: [u8; 0x2ac],
    /// field_2c0 (confidence: medium, kind: flags, lanes: c-misc-b,via:CViewport3DScene,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportPrimaryOrtho,via:CViewportRadar moved from siblings:CViewport3DScene,CViewportFrontend3DScene,CViewportMobilePhone,CViewportPrimaryOrtho,CViewportRadar).
    pub field_2c0: u32,
    /// field_2c4 (confidence: medium, kind: flags, lanes: c-misc-b,via:CViewport3DScene,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportPrimaryOrtho,via:CViewportRadar moved from siblings:CViewport3DScene,CViewportFrontend3DScene,CViewportMobilePhone,CViewportPrimaryOrtho,CViewportRadar).
    pub field_2c4: u32,
    /// field_2c8 (confidence: medium, kind: pointer, lanes: c-misc-b,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportRadar moved from siblings:CViewportFrontend3DScene,CViewportMobilePhone,CViewportRadar).
    pub field_2c8: Ptr32<u8>,
    /// Unknown bytes (0x2cc..0x2d0).
    pub _pad_02cc: [u8; 0x4],
    /// field_2d0 (confidence: medium, kind: pointer, lanes: c-misc-b,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportRadar moved from siblings:CViewportFrontend3DScene,CViewportMobilePhone,CViewportRadar).
    pub field_2d0: Ptr32<u8>,
    /// field_2d4 (confidence: medium, kind: pointer, lanes: c-misc-b,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportRadar moved from siblings:CViewportFrontend3DScene,CViewportMobilePhone,CViewportRadar).
    pub field_2d4: Ptr32<u8>,
    /// Unknown bytes (0x2d8..0x300).
    pub _pad_02d8: [u8; 0x28],
    /// field_300 (confidence: medium, kind: bool/byte?, lanes: c-misc-b,via:CViewportFrontend3DScene,via:CViewportMobilePhone,via:CViewportRadar moved from siblings:CViewportFrontend3DScene,CViewportMobilePhone,CViewportRadar).
    pub field_300: u8,
    /// Unknown bytes (0x301..0x400).
    pub _pad_0301: [u8; 0xff],
    /// field_400 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_400: u32,
    /// Unknown bytes (0x404..0x40c).
    pub _pad_0404: [u8; 0x8],
    /// field_40c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_40c: u32,
    /// Unknown bytes (0x410..0x424).
    pub _pad_0410: [u8; 0x14],
    /// field_424 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_424: u32,
    /// Unknown bytes (0x428..0x46c).
    pub _pad_0428: [u8; 0x44],
    /// field_46c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_46c: u32,
    /// Unknown bytes (0x470..0x4b0).
    pub _pad_0470: [u8; 0x40],
    /// field_4b0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_4b0: u32,
    /// Unknown bytes (0x4b4..0x530).
    pub _pad_04b4: [u8; 0x7c],
    /// field_530 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_530: u32,
    /// field_534 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_534: u32,
    /// field_538 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_538: u32,
    /// field_53c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_53c: u32,
    /// field_540 (confidence: high, kind: pointer, lanes: c-misc-b).
    pub field_540: Ptr32<u8>,
    /// field_544 (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_544: Ptr32<u8>,
    /// field_548 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_548: u32,
    /// field_54c (confidence: low, kind: pointer, lanes: c-misc-b).
    pub field_54c: Ptr32<u8>,
    /// field_550 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_550: u32,
    /// field_554 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_554: u32,
    /// field_558 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_558: u8,
    /// Unknown trailing bytes (0x559..0x55c).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CViewport, 0x55c); // merged size 0x559 rounded to 4
assert_offset!(CViewport, vftable, 0x0);
assert_offset!(CViewport, field_10, 0x10);
assert_offset!(CViewport, field_2c0, 0x2c0);
assert_offset!(CViewport, field_2c4, 0x2c4);
assert_offset!(CViewport, field_2c8, 0x2c8);
assert_offset!(CViewport, field_2d0, 0x2d0);
assert_offset!(CViewport, field_2d4, 0x2d4);
assert_offset!(CViewport, field_300, 0x300);
assert_offset!(CViewport, field_400, 0x400);
assert_offset!(CViewport, field_40c, 0x40c);
assert_offset!(CViewport, field_424, 0x424);
assert_offset!(CViewport, field_46c, 0x46c);
assert_offset!(CViewport, field_4b0, 0x4b0);
assert_offset!(CViewport, field_530, 0x530);
assert_offset!(CViewport, field_534, 0x534);
assert_offset!(CViewport, field_538, 0x538);
assert_offset!(CViewport, field_53c, 0x53c);
assert_offset!(CViewport, field_540, 0x540);
assert_offset!(CViewport, field_544, 0x544);
assert_offset!(CViewport, field_548, 0x548);
assert_offset!(CViewport, field_54c, 0x54c);
assert_offset!(CViewport, field_550, 0x550);
assert_offset!(CViewport, field_554, 0x554);
assert_offset!(CViewport, field_558, 0x558);
