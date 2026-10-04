//! Audio entities, sounds, voices, effects and stream readers.
//!
//! Holds 17 draft layouts: The game's `aud*AudioEntity` classes and `audSpeechManager`, RAGE's
//! `rage::aud*` sounds, voices and DSP effects, the stream readers `rage::CBaseStreamReader` and
//! `rage::CWMStreamReader` (the second names the first as its base), and the `AudioStream` record
//! seen by the native-handler lanes. Every layout is Inferred; size confidence (the analysis lanes'
//! own rating) is high for 1, medium for 9 and low for 7. The conventions are those of the crate
//! root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `AudioStream`.
///
/// Size: 0x3ac8 (low). Bases: none.
/// Lanes: n-08.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioStream {
    /// Unknown bytes (0x0..0x3b).
    pub _pad_0000: [u8; 0x3b],
    /// state (confidence: medium, kind: u8, lanes: n-08).
    pub state: u8,
    /// Unknown bytes (0x3c..0xb8).
    pub _pad_003c: [u8; 0x7c],
    /// playtime_ms (confidence: medium, kind: u32, lanes: n-08).
    pub playtime_ms: u32,
    /// Unknown bytes (0xbc..0x3ac4).
    pub _pad_00bc: [u8; 0x3a08],
    /// stream (confidence: medium, kind: pointer, lanes: n-08).
    pub stream: Ptr32<u8>,
}
assert_size!(AudioStream, 0x3ac8); // merged size 0x3ac8 rounded to 4
assert_offset!(AudioStream, state, 0x3b);
assert_offset!(AudioStream, playtime_ms, 0xb8);
assert_offset!(AudioStream, stream, 0x3ac4);

/// Merged layout for `audFrontendAudioEntity`.
///
/// Size: 0x4b8 (medium). Bases: audGtaAudioEntity@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudFrontendAudioEntity {
    /// Unknown bytes (0x0..0x4c).
    pub _pad_0000: [u8; 0x4c],
    /// flag_0x4c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4c: u32,
    /// flag_0x50? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x50: u32,
    /// field_54 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_54: Ptr32<u8>,
    /// Unknown bytes (0x58..0x7c).
    pub _pad_0058: [u8; 0x24],
    /// flag_0x7c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x7c: u32,
    /// Unknown bytes (0x80..0xe8).
    pub _pad_0080: [u8; 0x68],
    /// field_e8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_e8: Ptr32<u8>,
    /// Unknown bytes (0xec..0x200).
    pub _pad_00ec: [u8; 0x114],
    /// field_200 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_200: Ptr32<u8>,
    /// field_204 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_204: Ptr32<u8>,
    /// flag_0x208 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x208: u8,
    /// Unknown bytes (0x209..0x20c).
    pub _pad_0209: [u8; 0x3],
    /// field_20c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_20c: Ptr32<u8>,
    /// field_210 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_210: Ptr32<u8>,
    /// field_214 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_214: Ptr32<u8>,
    /// flag_0x218? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x218: u32,
    /// field_21c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_21c: Ptr32<u8>,
    /// Unknown bytes (0x220..0x228).
    pub _pad_0220: [u8; 0x8],
    /// flag_0x228? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x228: u32,
    /// field_22c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_22c: Ptr32<u8>,
    /// field_230 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_230: Ptr32<u8>,
    /// field_234 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_234: Ptr32<u8>,
    /// Unknown bytes (0x238..0x33c).
    pub _pad_0238: [u8; 0x104],
    /// flag_0x33c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x33c: u32,
    /// flag_0x340 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x340: u8,
    /// Unknown bytes (0x341..0x344).
    pub _pad_0341: [u8; 0x3],
    /// flag_0x344? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x344: u32,
    /// field_348 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_348: Ptr32<u8>,
    /// flag_0x34c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x34c: u32,
    /// Unknown bytes (0x350..0x3f8).
    pub _pad_0350: [u8; 0xa8],
    /// field_3f8 (confidence: low, kind: int16, lanes: c-audio).
    pub field_3f8: u16,
    /// flag_0x3fa (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x3fa: u8,
    /// Unknown bytes (0x3fb..0x3fc).
    pub _pad_03fb: [u8; 0x1],
    /// field_3fc (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_3fc: Ptr32<u8>,
    /// flag_0x400 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x400: u8,
    /// flag_0x401 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x401: u8,
    /// field_402 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_402: u8,
    /// Unknown bytes (0x403..0x404).
    pub _pad_0403: [u8; 0x1],
    /// flag_0x404? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x404: u32,
    /// flag_0x408? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x408: u32,
    /// flag_0x40c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x40c: u32,
    /// flag_0x410? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x410: u32,
    /// field_414 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_414: Ptr32<u8>,
    /// flag_0x418? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x418: u32,
    /// flag_0x41c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x41c: u32,
    /// Unknown bytes (0x420..0x43c).
    pub _pad_0420: [u8; 0x1c],
    /// field_43c (confidence: low, kind: int16, lanes: c-audio).
    pub field_43c: u16,
    /// flag_0x43e (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x43e: u8,
    /// flag_0x43f (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x43f: u8,
    /// flag_0x440 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x440: u8,
    /// Unknown bytes (0x441..0x444).
    pub _pad_0441: [u8; 0x3],
    /// flag_0x444? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x444: u32,
    /// flag_0x448? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x448: u32,
    /// field_44c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_44c: Ptr32<u8>,
    /// Unknown bytes (0x450..0x468).
    pub _pad_0450: [u8; 0x18],
    /// flag_0x468? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x468: u32,
    /// flag_0x46c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x46c: u32,
    /// flag_0x470? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x470: u32,
    /// flag_0x474? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x474: u32,
    /// flag_0x478? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x478: u32,
    /// flag_0x47c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x47c: u32,
    /// flag_0x480 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x480: u8,
    /// Unknown bytes (0x481..0x484).
    pub _pad_0481: [u8; 0x3],
    /// field_484 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_484: Ptr32<u8>,
    /// flag_0x488? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x488: u32,
    /// flag_0x48c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x48c: u32,
    /// Unknown bytes (0x490..0x49c).
    pub _pad_0490: [u8; 0xc],
    /// flag_0x49c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x49c: u32,
    /// field_4a0 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4a0: Ptr32<u8>,
    /// field_4a4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4a4: Ptr32<u8>,
    /// field_4a8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4a8: Ptr32<u8>,
    /// field_4ac (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4ac: Ptr32<u8>,
    /// field_4b0 (confidence: low, kind: int16, lanes: c-audio).
    pub field_4b0: u16,
    /// Unknown bytes (0x4b2..0x4b4).
    pub _pad_04b2: [u8; 0x2],
    /// flag_0x4b4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4b4: u32,
}
assert_size!(AudFrontendAudioEntity, 0x4b8); // merged size 0x4b8 rounded to 4
assert_offset!(AudFrontendAudioEntity, flag_0x4c, 0x4c);
assert_offset!(AudFrontendAudioEntity, flag_0x50, 0x50);
assert_offset!(AudFrontendAudioEntity, field_54, 0x54);
assert_offset!(AudFrontendAudioEntity, flag_0x7c, 0x7c);
assert_offset!(AudFrontendAudioEntity, field_e8, 0xe8);
assert_offset!(AudFrontendAudioEntity, field_200, 0x200);
assert_offset!(AudFrontendAudioEntity, field_204, 0x204);
assert_offset!(AudFrontendAudioEntity, flag_0x208, 0x208);
assert_offset!(AudFrontendAudioEntity, field_20c, 0x20c);
assert_offset!(AudFrontendAudioEntity, field_210, 0x210);
assert_offset!(AudFrontendAudioEntity, field_214, 0x214);
assert_offset!(AudFrontendAudioEntity, flag_0x218, 0x218);
assert_offset!(AudFrontendAudioEntity, field_21c, 0x21c);
assert_offset!(AudFrontendAudioEntity, flag_0x228, 0x228);
assert_offset!(AudFrontendAudioEntity, field_22c, 0x22c);
assert_offset!(AudFrontendAudioEntity, field_230, 0x230);
assert_offset!(AudFrontendAudioEntity, field_234, 0x234);
assert_offset!(AudFrontendAudioEntity, flag_0x33c, 0x33c);
assert_offset!(AudFrontendAudioEntity, flag_0x340, 0x340);
assert_offset!(AudFrontendAudioEntity, flag_0x344, 0x344);
assert_offset!(AudFrontendAudioEntity, field_348, 0x348);
assert_offset!(AudFrontendAudioEntity, flag_0x34c, 0x34c);
assert_offset!(AudFrontendAudioEntity, field_3f8, 0x3f8);
assert_offset!(AudFrontendAudioEntity, flag_0x3fa, 0x3fa);
assert_offset!(AudFrontendAudioEntity, field_3fc, 0x3fc);
assert_offset!(AudFrontendAudioEntity, flag_0x400, 0x400);
assert_offset!(AudFrontendAudioEntity, flag_0x401, 0x401);
assert_offset!(AudFrontendAudioEntity, field_402, 0x402);
assert_offset!(AudFrontendAudioEntity, flag_0x404, 0x404);
assert_offset!(AudFrontendAudioEntity, flag_0x408, 0x408);
assert_offset!(AudFrontendAudioEntity, flag_0x40c, 0x40c);
assert_offset!(AudFrontendAudioEntity, flag_0x410, 0x410);
assert_offset!(AudFrontendAudioEntity, field_414, 0x414);
assert_offset!(AudFrontendAudioEntity, flag_0x418, 0x418);
assert_offset!(AudFrontendAudioEntity, flag_0x41c, 0x41c);
assert_offset!(AudFrontendAudioEntity, field_43c, 0x43c);
assert_offset!(AudFrontendAudioEntity, flag_0x43e, 0x43e);
assert_offset!(AudFrontendAudioEntity, flag_0x43f, 0x43f);
assert_offset!(AudFrontendAudioEntity, flag_0x440, 0x440);
assert_offset!(AudFrontendAudioEntity, flag_0x444, 0x444);
assert_offset!(AudFrontendAudioEntity, flag_0x448, 0x448);
assert_offset!(AudFrontendAudioEntity, field_44c, 0x44c);
assert_offset!(AudFrontendAudioEntity, flag_0x468, 0x468);
assert_offset!(AudFrontendAudioEntity, flag_0x46c, 0x46c);
assert_offset!(AudFrontendAudioEntity, flag_0x470, 0x470);
assert_offset!(AudFrontendAudioEntity, flag_0x474, 0x474);
assert_offset!(AudFrontendAudioEntity, flag_0x478, 0x478);
assert_offset!(AudFrontendAudioEntity, flag_0x47c, 0x47c);
assert_offset!(AudFrontendAudioEntity, flag_0x480, 0x480);
assert_offset!(AudFrontendAudioEntity, field_484, 0x484);
assert_offset!(AudFrontendAudioEntity, flag_0x488, 0x488);
assert_offset!(AudFrontendAudioEntity, flag_0x48c, 0x48c);
assert_offset!(AudFrontendAudioEntity, flag_0x49c, 0x49c);
assert_offset!(AudFrontendAudioEntity, field_4a0, 0x4a0);
assert_offset!(AudFrontendAudioEntity, field_4a4, 0x4a4);
assert_offset!(AudFrontendAudioEntity, field_4a8, 0x4a8);
assert_offset!(AudFrontendAudioEntity, field_4ac, 0x4ac);
assert_offset!(AudFrontendAudioEntity, field_4b0, 0x4b0);
assert_offset!(AudFrontendAudioEntity, flag_0x4b4, 0x4b4);

/// Merged layout for `audGtaAudioEntity`.
///
/// Size: 0x60 (high). Bases: rage::audEntity@0x0.
/// Lanes: c-audio, via:audFrontendAudioEntity, via:audPedAudioEntity, via:audPoliceScanner, via:audRadioAudioEntity.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudGtaAudioEntity {
    /// Unknown bytes (0x0..0x1c).
    pub _pad_0000: [u8; 0x1c],
    /// flag_0x1c? (confidence: medium, kind: bool?, lanes: c-audio,via:audPedAudioEntity,via:audRadioAudioEntity moved from siblings:audPedAudioEntity,audRadioAudioEntity).
    pub flag_0x1c: u32,
    /// Unknown bytes (0x20..0x28).
    pub _pad_0020: [u8; 0x8],
    /// flag_0x28? (confidence: medium, kind: bool?, lanes: c-audio,via:audPoliceScanner,via:audRadioAudioEntity moved from siblings:audPoliceScanner,audRadioAudioEntity).
    pub flag_0x28: u32,
    /// flag_0x2c? (confidence: medium, kind: bool?, lanes: c-audio,via:audPoliceScanner,via:audRadioAudioEntity moved from siblings:audPoliceScanner,audRadioAudioEntity).
    pub flag_0x2c: u32,
    /// Unknown bytes (0x30..0x48).
    pub _pad_0030: [u8; 0x18],
    /// flag_0x48? (confidence: medium, kind: bool?, lanes: c-audio,via:audFrontendAudioEntity,via:audPoliceScanner moved from siblings:audFrontendAudioEntity,audPoliceScanner).
    pub flag_0x48: u32,
    /// Unknown trailing bytes (0x4c..0x60).
    pub _pad_end: [u8; 0x14],
}
assert_size!(AudGtaAudioEntity, 0x60); // merged size 0x60 rounded to 4
assert_offset!(AudGtaAudioEntity, flag_0x1c, 0x1c);
assert_offset!(AudGtaAudioEntity, flag_0x28, 0x28);
assert_offset!(AudGtaAudioEntity, flag_0x2c, 0x2c);
assert_offset!(AudGtaAudioEntity, flag_0x48, 0x48);

/// Merged layout for `audPedAudioEntity`.
///
/// Size: 0x1a4 (medium). Bases: audGtaAudioEntity@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudPedAudioEntity {
    /// Unknown bytes (0x0..0xc).
    pub _pad_0000: [u8; 0xc],
    /// ptr_0xc (confidence: medium, kind: pointer, lanes: c-audio).
    pub ptr_0xc: Ptr32<u8>,
    /// flag_0x10 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x10: u8,
    /// Unknown bytes (0x11..0x14).
    pub _pad_0011: [u8; 0x3],
    /// flag_0x14? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x14: u32,
    /// flag_0x18? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x18: u32,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// flag_0x20? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x20: u32,
    /// field_24 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_24: Ptr32<u8>,
    /// Unknown bytes (0x28..0x44).
    pub _pad_0028: [u8; 0x1c],
    /// field_44 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_44: Ptr32<u8>,
    /// Unknown bytes (0x48..0x74).
    pub _pad_0048: [u8; 0x2c],
    /// flag_0x74? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x74: u32,
    /// Unknown bytes (0x78..0x90).
    pub _pad_0078: [u8; 0x18],
    /// field_90 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_90: Ptr32<u8>,
    /// Unknown bytes (0x94..0xac).
    pub _pad_0094: [u8; 0x18],
    /// flag_0xac? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xac: u32,
    /// flag_0xb0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xb0: u32,
    /// flag_0xb4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xb4: u32,
    /// Unknown bytes (0xb8..0x120).
    pub _pad_00b8: [u8; 0x68],
    /// ptr_0x120 (confidence: medium, kind: pointer, lanes: c-audio).
    pub ptr_0x120: Ptr32<u8>,
    /// flag_0x124? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x124: u32,
    /// Unknown bytes (0x128..0x138).
    pub _pad_0128: [u8; 0x10],
    /// flag_0x138? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x138: u32,
    /// flag_0x13c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x13c: u32,
    /// flag_0x140? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x140: u32,
    /// flag_0x144? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x144: u32,
    /// Unknown bytes (0x148..0x17c).
    pub _pad_0148: [u8; 0x34],
    /// field_17c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_17c: Ptr32<u8>,
    /// Unknown bytes (0x180..0x198).
    pub _pad_0180: [u8; 0x18],
    /// flag_0x198? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x198: u32,
    /// flag_0x19c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x19c: u32,
    /// flag_0x1a0 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x1a0: u8,
    /// Unknown trailing bytes (0x1a1..0x1a4).
    pub _pad_end: [u8; 0x3],
}
assert_size!(AudPedAudioEntity, 0x1a4); // merged size 0x1a4 rounded to 4
assert_offset!(AudPedAudioEntity, ptr_0xc, 0xc);
assert_offset!(AudPedAudioEntity, flag_0x10, 0x10);
assert_offset!(AudPedAudioEntity, flag_0x14, 0x14);
assert_offset!(AudPedAudioEntity, flag_0x18, 0x18);
assert_offset!(AudPedAudioEntity, flag_0x20, 0x20);
assert_offset!(AudPedAudioEntity, field_24, 0x24);
assert_offset!(AudPedAudioEntity, field_44, 0x44);
assert_offset!(AudPedAudioEntity, flag_0x74, 0x74);
assert_offset!(AudPedAudioEntity, field_90, 0x90);
assert_offset!(AudPedAudioEntity, flag_0xac, 0xac);
assert_offset!(AudPedAudioEntity, flag_0xb0, 0xb0);
assert_offset!(AudPedAudioEntity, flag_0xb4, 0xb4);
assert_offset!(AudPedAudioEntity, ptr_0x120, 0x120);
assert_offset!(AudPedAudioEntity, flag_0x124, 0x124);
assert_offset!(AudPedAudioEntity, flag_0x138, 0x138);
assert_offset!(AudPedAudioEntity, flag_0x13c, 0x13c);
assert_offset!(AudPedAudioEntity, flag_0x140, 0x140);
assert_offset!(AudPedAudioEntity, flag_0x144, 0x144);
assert_offset!(AudPedAudioEntity, field_17c, 0x17c);
assert_offset!(AudPedAudioEntity, flag_0x198, 0x198);
assert_offset!(AudPedAudioEntity, flag_0x19c, 0x19c);
assert_offset!(AudPedAudioEntity, flag_0x1a0, 0x1a0);

/// Merged layout for `audRadioAudioEntity`.
///
/// Size: 0x9c (medium). Bases: audGtaAudioEntity@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudRadioAudioEntity {
    /// Unknown bytes (0x0..0xc).
    pub _pad_0000: [u8; 0xc],
    /// flag_0xc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xc: u32,
    /// flag_0x10? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x10: u32,
    /// flag_0x14? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x14: u32,
    /// field_18 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x20).
    pub _pad_001c: [u8; 0x4],
    /// flag_0x20? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x20: u32,
    /// Unknown bytes (0x24..0x30).
    pub _pad_0024: [u8; 0xc],
    /// field_30 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_30: Ptr32<u8>,
    /// Unknown bytes (0x34..0x4c).
    pub _pad_0034: [u8; 0x18],
    /// field_4c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4c: Ptr32<u8>,
    /// Unknown bytes (0x50..0x68).
    pub _pad_0050: [u8; 0x18],
    /// field_68 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_68: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// flag_0x70? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x70: u32,
    /// field_74 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_78: Ptr32<u8>,
    /// field_7c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_7c: Ptr32<u8>,
    /// field_80 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_80: Ptr32<u8>,
    /// field_84 (confidence: low, kind: int16, lanes: c-audio).
    pub field_84: u16,
    /// flag_0x86 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x86: u8,
    /// flag_0x87 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x87: u8,
    /// field_88 (confidence: low, kind: int16, lanes: c-audio).
    pub field_88: u16,
    /// flag_0x8a (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x8a: u8,
    /// flag_0x8b (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x8b: u8,
    /// field_8c (confidence: low, kind: int16, lanes: c-audio).
    pub field_8c: u16,
    /// field_8e (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_8e: u8,
    /// Unknown bytes (0x8f..0x90).
    pub _pad_008f: [u8; 0x1],
    /// flag_0x90? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x90: u32,
    /// Unknown bytes (0x94..0x95).
    pub _pad_0094: [u8; 0x1],
    /// flag_0x95 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x95: u8,
    /// field_96 (confidence: low, kind: int16, lanes: c-audio).
    pub field_96: u16,
    /// field_98 (confidence: low, kind: int16, lanes: c-audio).
    pub field_98: u16,
    /// Unknown trailing bytes (0x9a..0x9c).
    pub _pad_end: [u8; 0x2],
}
assert_size!(AudRadioAudioEntity, 0x9c); // merged size 0x9c rounded to 4
assert_offset!(AudRadioAudioEntity, flag_0xc, 0xc);
assert_offset!(AudRadioAudioEntity, flag_0x10, 0x10);
assert_offset!(AudRadioAudioEntity, flag_0x14, 0x14);
assert_offset!(AudRadioAudioEntity, field_18, 0x18);
assert_offset!(AudRadioAudioEntity, flag_0x20, 0x20);
assert_offset!(AudRadioAudioEntity, field_30, 0x30);
assert_offset!(AudRadioAudioEntity, field_4c, 0x4c);
assert_offset!(AudRadioAudioEntity, field_68, 0x68);
assert_offset!(AudRadioAudioEntity, flag_0x70, 0x70);
assert_offset!(AudRadioAudioEntity, field_74, 0x74);
assert_offset!(AudRadioAudioEntity, field_78, 0x78);
assert_offset!(AudRadioAudioEntity, field_7c, 0x7c);
assert_offset!(AudRadioAudioEntity, field_80, 0x80);
assert_offset!(AudRadioAudioEntity, field_84, 0x84);
assert_offset!(AudRadioAudioEntity, flag_0x86, 0x86);
assert_offset!(AudRadioAudioEntity, flag_0x87, 0x87);
assert_offset!(AudRadioAudioEntity, field_88, 0x88);
assert_offset!(AudRadioAudioEntity, flag_0x8a, 0x8a);
assert_offset!(AudRadioAudioEntity, flag_0x8b, 0x8b);
assert_offset!(AudRadioAudioEntity, field_8c, 0x8c);
assert_offset!(AudRadioAudioEntity, field_8e, 0x8e);
assert_offset!(AudRadioAudioEntity, flag_0x90, 0x90);
assert_offset!(AudRadioAudioEntity, flag_0x95, 0x95);
assert_offset!(AudRadioAudioEntity, field_96, 0x96);
assert_offset!(AudRadioAudioEntity, field_98, 0x98);

/// Merged layout for `audSpeechManager`.
///
/// Size: 0x540 (medium). Bases: rage::audEntity@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudSpeechManager {
    /// Unknown bytes (0x0..0xc).
    pub _pad_0000: [u8; 0xc],
    /// flag_0xc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xc: u32,
    /// flag_0x10? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x10: u32,
    /// flag_0x14? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x14: u32,
    /// Unknown bytes (0x18..0x58).
    pub _pad_0018: [u8; 0x40],
    /// field_58 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_58: Ptr32<u8>,
    /// Unknown bytes (0x5c..0x60).
    pub _pad_005c: [u8; 0x4],
    /// flag_0x60? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x60: u32,
    /// flag_0x64 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x64: u8,
    /// Unknown bytes (0x65..0x68).
    pub _pad_0065: [u8; 0x3],
    /// flag_0x68 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x68: u8,
    /// Unknown bytes (0x69..0x70).
    pub _pad_0069: [u8; 0x7],
    /// flag_0x70? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x70: u32,
    /// flag_0x74? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x74: u32,
    /// Unknown bytes (0x78..0xbc).
    pub _pad_0078: [u8; 0x44],
    /// flag_0xbc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xbc: u32,
    /// flag_0xc0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xc0: u32,
    /// flag_0xc4 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0xc4: u8,
    /// Unknown bytes (0xc5..0xc8).
    pub _pad_00c5: [u8; 0x3],
    /// flag_0xc8 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0xc8: u8,
    /// Unknown bytes (0xc9..0xcc).
    pub _pad_00c9: [u8; 0x3],
    /// flag_0xcc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xcc: u32,
    /// flag_0xd0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xd0: u32,
    /// flag_0xd4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xd4: u32,
    /// Unknown bytes (0xd8..0x118).
    pub _pad_00d8: [u8; 0x40],
    /// field_118 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_118: Ptr32<u8>,
    /// flag_0x11c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x11c: u32,
    /// flag_0x120? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x120: u32,
    /// flag_0x124 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x124: u8,
    /// Unknown bytes (0x125..0x12c).
    pub _pad_0125: [u8; 0x7],
    /// flag_0x12c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x12c: u32,
    /// Unknown bytes (0x130..0x178).
    pub _pad_0130: [u8; 0x48],
    /// field_178 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_178: Ptr32<u8>,
    /// flag_0x17c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x17c: u32,
    /// flag_0x180? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x180: u32,
    /// flag_0x184 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x184: u8,
    /// Unknown bytes (0x185..0x188).
    pub _pad_0185: [u8; 0x3],
    /// flag_0x188 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x188: u8,
    /// Unknown bytes (0x189..0x18c).
    pub _pad_0189: [u8; 0x3],
    /// flag_0x18c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x18c: u32,
    /// flag_0x190? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x190: u32,
    /// flag_0x194? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x194: u32,
    /// Unknown bytes (0x198..0x1d8).
    pub _pad_0198: [u8; 0x40],
    /// field_1d8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_1d8: Ptr32<u8>,
    /// flag_0x1dc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x1dc: u32,
    /// flag_0x1e0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x1e0: u32,
    /// flag_0x1e4 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x1e4: u8,
    /// Unknown bytes (0x1e5..0x1e8).
    pub _pad_01e5: [u8; 0x3],
    /// flag_0x1e8 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x1e8: u8,
    /// Unknown bytes (0x1e9..0x1ec).
    pub _pad_01e9: [u8; 0x3],
    /// flag_0x1ec? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x1ec: u32,
    /// flag_0x1f0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x1f0: u32,
    /// flag_0x1f4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x1f4: u32,
    /// Unknown bytes (0x1f8..0x23c).
    pub _pad_01f8: [u8; 0x44],
    /// flag_0x23c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x23c: u32,
    /// flag_0x240? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x240: u32,
    /// flag_0x244 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x244: u8,
    /// Unknown bytes (0x245..0x248).
    pub _pad_0245: [u8; 0x3],
    /// flag_0x248 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x248: u8,
    /// Unknown bytes (0x249..0x24c).
    pub _pad_0249: [u8; 0x3],
    /// flag_0x24c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x24c: u32,
    /// flag_0x250? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x250: u32,
    /// flag_0x254? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x254: u32,
    /// Unknown bytes (0x258..0x298).
    pub _pad_0258: [u8; 0x40],
    /// field_298 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_298: Ptr32<u8>,
    /// flag_0x29c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x29c: u32,
    /// flag_0x2a0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2a0: u32,
    /// flag_0x2a4 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x2a4: u8,
    /// Unknown bytes (0x2a5..0x2a8).
    pub _pad_02a5: [u8; 0x3],
    /// flag_0x2a8 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x2a8: u8,
    /// Unknown bytes (0x2a9..0x2ac).
    pub _pad_02a9: [u8; 0x3],
    /// flag_0x2ac? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2ac: u32,
    /// flag_0x2b0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2b0: u32,
    /// flag_0x2b4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2b4: u32,
    /// Unknown bytes (0x2b8..0x2f8).
    pub _pad_02b8: [u8; 0x40],
    /// field_2f8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_2f8: Ptr32<u8>,
    /// flag_0x2fc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2fc: u32,
    /// flag_0x300? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x300: u32,
    /// flag_0x304 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x304: u8,
    /// Unknown bytes (0x305..0x308).
    pub _pad_0305: [u8; 0x3],
    /// flag_0x308 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x308: u8,
    /// Unknown bytes (0x309..0x30c).
    pub _pad_0309: [u8; 0x3],
    /// flag_0x30c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x30c: u32,
    /// flag_0x310? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x310: u32,
    /// flag_0x314? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x314: u32,
    /// Unknown bytes (0x318..0x35c).
    pub _pad_0318: [u8; 0x44],
    /// flag_0x35c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x35c: u32,
    /// flag_0x360? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x360: u32,
    /// flag_0x364 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x364: u8,
    /// Unknown bytes (0x365..0x36c).
    pub _pad_0365: [u8; 0x7],
    /// flag_0x36c (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x36c: u8,
    /// Unknown bytes (0x36d..0x370).
    pub _pad_036d: [u8; 0x3],
    /// flag_0x370? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x370: u32,
    /// flag_0x374? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x374: u32,
    /// flag_0x378? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x378: u32,
    /// Unknown bytes (0x37c..0x3bc).
    pub _pad_037c: [u8; 0x40],
    /// field_3bc (confidence: low, kind: pointer, lanes: c-audio).
    pub field_3bc: Ptr32<u8>,
    /// flag_0x3c0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x3c0: u32,
    /// flag_0x3c4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x3c4: u32,
    /// flag_0x3c8 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x3c8: u8,
    /// Unknown bytes (0x3c9..0x3cc).
    pub _pad_03c9: [u8; 0x3],
    /// flag_0x3cc (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x3cc: u8,
    /// Unknown bytes (0x3cd..0x3d0).
    pub _pad_03cd: [u8; 0x3],
    /// flag_0x3d0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x3d0: u32,
    /// flag_0x3d4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x3d4: u32,
    /// flag_0x3d8? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x3d8: u32,
    /// Unknown bytes (0x3dc..0x41c).
    pub _pad_03dc: [u8; 0x40],
    /// field_41c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_41c: Ptr32<u8>,
    /// flag_0x420? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x420: u32,
    /// flag_0x424? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x424: u32,
    /// flag_0x428 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x428: u8,
    /// Unknown bytes (0x429..0x42c).
    pub _pad_0429: [u8; 0x3],
    /// flag_0x42c (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x42c: u8,
    /// Unknown bytes (0x42d..0x430).
    pub _pad_042d: [u8; 0x3],
    /// flag_0x430? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x430: u32,
    /// flag_0x434? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x434: u32,
    /// Unknown bytes (0x438..0x47c).
    pub _pad_0438: [u8; 0x44],
    /// field_47c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_47c: Ptr32<u8>,
    /// flag_0x480? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x480: u32,
    /// flag_0x484? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x484: u32,
    /// flag_0x488 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x488: u8,
    /// Unknown bytes (0x489..0x48c).
    pub _pad_0489: [u8; 0x3],
    /// flag_0x48c (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x48c: u8,
    /// Unknown bytes (0x48d..0x494).
    pub _pad_048d: [u8; 0x7],
    /// flag_0x494? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x494: u32,
    /// flag_0x498? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x498: u32,
    /// Unknown bytes (0x49c..0x4dc).
    pub _pad_049c: [u8; 0x40],
    /// field_4dc (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4dc: Ptr32<u8>,
    /// flag_0x4e0? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4e0: u32,
    /// flag_0x4e4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4e4: u32,
    /// flag_0x4e8 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x4e8: u8,
    /// Unknown bytes (0x4e9..0x4f8).
    pub _pad_04e9: [u8; 0xf],
    /// field_4f8 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_4f8: u8,
    /// Unknown bytes (0x4f9..0x4fc).
    pub _pad_04f9: [u8; 0x3],
    /// flag_0x4fc? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4fc: u32,
    /// field_500 (confidence: low, kind: int16, lanes: c-audio).
    pub field_500: u16,
    /// Unknown bytes (0x502..0x504).
    pub _pad_0502: [u8; 0x2],
    /// field_504 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_504: u8,
    /// Unknown bytes (0x505..0x508).
    pub _pad_0505: [u8; 0x3],
    /// field_508 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_508: Ptr32<u8>,
    /// field_50c (confidence: low, kind: int16, lanes: c-audio).
    pub field_50c: u16,
    /// Unknown bytes (0x50e..0x510).
    pub _pad_050e: [u8; 0x2],
    /// field_510 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_510: u8,
    /// Unknown bytes (0x511..0x514).
    pub _pad_0511: [u8; 0x3],
    /// field_514 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_514: Ptr32<u8>,
    /// field_518 (confidence: low, kind: int16, lanes: c-audio).
    pub field_518: u16,
    /// Unknown bytes (0x51a..0x51c).
    pub _pad_051a: [u8; 0x2],
    /// field_51c (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_51c: u8,
    /// Unknown bytes (0x51d..0x520).
    pub _pad_051d: [u8; 0x3],
    /// field_520 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_520: Ptr32<u8>,
    /// field_524 (confidence: low, kind: int16, lanes: c-audio).
    pub field_524: u16,
    /// Unknown bytes (0x526..0x528).
    pub _pad_0526: [u8; 0x2],
    /// field_528 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_528: u8,
    /// Unknown bytes (0x529..0x52c).
    pub _pad_0529: [u8; 0x3],
    /// field_52c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_52c: Ptr32<u8>,
    /// field_530 (confidence: low, kind: int16, lanes: c-audio).
    pub field_530: u16,
    /// Unknown bytes (0x532..0x534).
    pub _pad_0532: [u8; 0x2],
    /// field_534 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_534: u8,
    /// Unknown bytes (0x535..0x538).
    pub _pad_0535: [u8; 0x3],
    /// field_538 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_538: Ptr32<u8>,
    /// field_53c (confidence: low, kind: int16, lanes: c-audio).
    pub field_53c: u16,
    /// Unknown trailing bytes (0x53e..0x540).
    pub _pad_end: [u8; 0x2],
}
assert_size!(AudSpeechManager, 0x540); // merged size 0x540 rounded to 4
assert_offset!(AudSpeechManager, flag_0xc, 0xc);
assert_offset!(AudSpeechManager, flag_0x10, 0x10);
assert_offset!(AudSpeechManager, flag_0x14, 0x14);
assert_offset!(AudSpeechManager, field_58, 0x58);
assert_offset!(AudSpeechManager, flag_0x60, 0x60);
assert_offset!(AudSpeechManager, flag_0x64, 0x64);
assert_offset!(AudSpeechManager, flag_0x68, 0x68);
assert_offset!(AudSpeechManager, flag_0x70, 0x70);
assert_offset!(AudSpeechManager, flag_0x74, 0x74);
assert_offset!(AudSpeechManager, flag_0xbc, 0xbc);
assert_offset!(AudSpeechManager, flag_0xc0, 0xc0);
assert_offset!(AudSpeechManager, flag_0xc4, 0xc4);
assert_offset!(AudSpeechManager, flag_0xc8, 0xc8);
assert_offset!(AudSpeechManager, flag_0xcc, 0xcc);
assert_offset!(AudSpeechManager, flag_0xd0, 0xd0);
assert_offset!(AudSpeechManager, flag_0xd4, 0xd4);
assert_offset!(AudSpeechManager, field_118, 0x118);
assert_offset!(AudSpeechManager, flag_0x11c, 0x11c);
assert_offset!(AudSpeechManager, flag_0x120, 0x120);
assert_offset!(AudSpeechManager, flag_0x124, 0x124);
assert_offset!(AudSpeechManager, flag_0x12c, 0x12c);
assert_offset!(AudSpeechManager, field_178, 0x178);
assert_offset!(AudSpeechManager, flag_0x17c, 0x17c);
assert_offset!(AudSpeechManager, flag_0x180, 0x180);
assert_offset!(AudSpeechManager, flag_0x184, 0x184);
assert_offset!(AudSpeechManager, flag_0x188, 0x188);
assert_offset!(AudSpeechManager, flag_0x18c, 0x18c);
assert_offset!(AudSpeechManager, flag_0x190, 0x190);
assert_offset!(AudSpeechManager, flag_0x194, 0x194);
assert_offset!(AudSpeechManager, field_1d8, 0x1d8);
assert_offset!(AudSpeechManager, flag_0x1dc, 0x1dc);
assert_offset!(AudSpeechManager, flag_0x1e0, 0x1e0);
assert_offset!(AudSpeechManager, flag_0x1e4, 0x1e4);
assert_offset!(AudSpeechManager, flag_0x1e8, 0x1e8);
assert_offset!(AudSpeechManager, flag_0x1ec, 0x1ec);
assert_offset!(AudSpeechManager, flag_0x1f0, 0x1f0);
assert_offset!(AudSpeechManager, flag_0x1f4, 0x1f4);
assert_offset!(AudSpeechManager, flag_0x23c, 0x23c);
assert_offset!(AudSpeechManager, flag_0x240, 0x240);
assert_offset!(AudSpeechManager, flag_0x244, 0x244);
assert_offset!(AudSpeechManager, flag_0x248, 0x248);
assert_offset!(AudSpeechManager, flag_0x24c, 0x24c);
assert_offset!(AudSpeechManager, flag_0x250, 0x250);
assert_offset!(AudSpeechManager, flag_0x254, 0x254);
assert_offset!(AudSpeechManager, field_298, 0x298);
assert_offset!(AudSpeechManager, flag_0x29c, 0x29c);
assert_offset!(AudSpeechManager, flag_0x2a0, 0x2a0);
assert_offset!(AudSpeechManager, flag_0x2a4, 0x2a4);
assert_offset!(AudSpeechManager, flag_0x2a8, 0x2a8);
assert_offset!(AudSpeechManager, flag_0x2ac, 0x2ac);
assert_offset!(AudSpeechManager, flag_0x2b0, 0x2b0);
assert_offset!(AudSpeechManager, flag_0x2b4, 0x2b4);
assert_offset!(AudSpeechManager, field_2f8, 0x2f8);
assert_offset!(AudSpeechManager, flag_0x2fc, 0x2fc);
assert_offset!(AudSpeechManager, flag_0x300, 0x300);
assert_offset!(AudSpeechManager, flag_0x304, 0x304);
assert_offset!(AudSpeechManager, flag_0x308, 0x308);
assert_offset!(AudSpeechManager, flag_0x30c, 0x30c);
assert_offset!(AudSpeechManager, flag_0x310, 0x310);
assert_offset!(AudSpeechManager, flag_0x314, 0x314);
assert_offset!(AudSpeechManager, flag_0x35c, 0x35c);
assert_offset!(AudSpeechManager, flag_0x360, 0x360);
assert_offset!(AudSpeechManager, flag_0x364, 0x364);
assert_offset!(AudSpeechManager, flag_0x36c, 0x36c);
assert_offset!(AudSpeechManager, flag_0x370, 0x370);
assert_offset!(AudSpeechManager, flag_0x374, 0x374);
assert_offset!(AudSpeechManager, flag_0x378, 0x378);
assert_offset!(AudSpeechManager, field_3bc, 0x3bc);
assert_offset!(AudSpeechManager, flag_0x3c0, 0x3c0);
assert_offset!(AudSpeechManager, flag_0x3c4, 0x3c4);
assert_offset!(AudSpeechManager, flag_0x3c8, 0x3c8);
assert_offset!(AudSpeechManager, flag_0x3cc, 0x3cc);
assert_offset!(AudSpeechManager, flag_0x3d0, 0x3d0);
assert_offset!(AudSpeechManager, flag_0x3d4, 0x3d4);
assert_offset!(AudSpeechManager, flag_0x3d8, 0x3d8);
assert_offset!(AudSpeechManager, field_41c, 0x41c);
assert_offset!(AudSpeechManager, flag_0x420, 0x420);
assert_offset!(AudSpeechManager, flag_0x424, 0x424);
assert_offset!(AudSpeechManager, flag_0x428, 0x428);
assert_offset!(AudSpeechManager, flag_0x42c, 0x42c);
assert_offset!(AudSpeechManager, flag_0x430, 0x430);
assert_offset!(AudSpeechManager, flag_0x434, 0x434);
assert_offset!(AudSpeechManager, field_47c, 0x47c);
assert_offset!(AudSpeechManager, flag_0x480, 0x480);
assert_offset!(AudSpeechManager, flag_0x484, 0x484);
assert_offset!(AudSpeechManager, flag_0x488, 0x488);
assert_offset!(AudSpeechManager, flag_0x48c, 0x48c);
assert_offset!(AudSpeechManager, flag_0x494, 0x494);
assert_offset!(AudSpeechManager, flag_0x498, 0x498);
assert_offset!(AudSpeechManager, field_4dc, 0x4dc);
assert_offset!(AudSpeechManager, flag_0x4e0, 0x4e0);
assert_offset!(AudSpeechManager, flag_0x4e4, 0x4e4);
assert_offset!(AudSpeechManager, flag_0x4e8, 0x4e8);
assert_offset!(AudSpeechManager, field_4f8, 0x4f8);
assert_offset!(AudSpeechManager, flag_0x4fc, 0x4fc);
assert_offset!(AudSpeechManager, field_500, 0x500);
assert_offset!(AudSpeechManager, field_504, 0x504);
assert_offset!(AudSpeechManager, field_508, 0x508);
assert_offset!(AudSpeechManager, field_50c, 0x50c);
assert_offset!(AudSpeechManager, field_510, 0x510);
assert_offset!(AudSpeechManager, field_514, 0x514);
assert_offset!(AudSpeechManager, field_518, 0x518);
assert_offset!(AudSpeechManager, field_51c, 0x51c);
assert_offset!(AudSpeechManager, field_520, 0x520);
assert_offset!(AudSpeechManager, field_524, 0x524);
assert_offset!(AudSpeechManager, field_528, 0x528);
assert_offset!(AudSpeechManager, field_52c, 0x52c);
assert_offset!(AudSpeechManager, field_530, 0x530);
assert_offset!(AudSpeechManager, field_534, 0x534);
assert_offset!(AudSpeechManager, field_538, 0x538);
assert_offset!(AudSpeechManager, field_53c, 0x53c);

/// Merged layout for `rage::CBaseStreamReader`.
///
/// Size: 0x20 (low). Bases: none.
/// Lanes: c-audio, c-misc-b, via:rage::CWMStreamReader, via:rage::audQTStreamReader.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCBaseStreamReader {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-audio,c-misc-b).
    pub vftable: Ptr32<()>,
    /// Unknown bytes (0x4..0x10).
    pub _pad_0004: [u8; 0xc],
    /// flag_0x10? (confidence: high, kind: flags, lanes: c-audio,c-misc-b,via:rage::CWMStreamReader,via:rage::audQTStreamReader moved from siblings:rage::CWMStreamReader,rage::audQTStreamReader).
    pub flag_0x10: u32,
    /// Unknown bytes (0x14..0x18).
    pub _pad_0014: [u8; 0x4],
    /// flag_0x18? (confidence: high, kind: flags, lanes: c-audio,c-misc-b,via:rage::CWMStreamReader,via:rage::audQTStreamReader moved from siblings:rage::CWMStreamReader,rage::audQTStreamReader).
    pub flag_0x18: u32,
    /// flag_0x1c? (confidence: high, kind: flags, lanes: c-audio,c-misc-b,via:rage::CWMStreamReader,via:rage::audQTStreamReader moved from siblings:rage::CWMStreamReader,rage::audQTStreamReader).
    pub flag_0x1c: u32,
}
assert_size!(RageCBaseStreamReader, 0x20); // merged size 0x20 rounded to 4
assert_offset!(RageCBaseStreamReader, vftable, 0x0);
assert_offset!(RageCBaseStreamReader, flag_0x10, 0x10);
assert_offset!(RageCBaseStreamReader, flag_0x18, 0x18);
assert_offset!(RageCBaseStreamReader, flag_0x1c, 0x1c);

/// Merged layout for `rage::CWMStreamReader`.
///
/// Size: 0x24 (low). Bases: rage::CBaseStreamReader@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageCWMStreamReader {
    /// Unknown bytes (0x0..0x4).
    pub _pad_0000: [u8; 0x4],
    /// field_4 (confidence: high, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// field_8 (confidence: medium, kind: word?, lanes: c-misc-b).
    pub field_8: u16,
    /// field_a (confidence: high, kind: bool/byte?, lanes: c-misc-b).
    pub field_a: u8,
    /// Unknown bytes (0xb..0xc).
    pub _pad_000b: [u8; 0x1],
    /// field_c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_c: u32,
    /// Unknown bytes (0x10..0x14).
    pub _pad_0010: [u8; 0x4],
    /// field_14 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// Unknown bytes (0x18..0x20).
    pub _pad_0018: [u8; 0x8],
    /// field_20 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_20: u32,
}
assert_size!(RageCWMStreamReader, 0x24); // merged size 0x24 rounded to 4
assert_offset!(RageCWMStreamReader, field_4, 0x4);
assert_offset!(RageCWMStreamReader, field_8, 0x8);
assert_offset!(RageCWMStreamReader, field_a, 0xa);
assert_offset!(RageCWMStreamReader, field_c, 0xc);
assert_offset!(RageCWMStreamReader, field_14, 0x14);
assert_offset!(RageCWMStreamReader, field_20, 0x20);

/// Merged layout for `rage::audCompressorEffectPc`.
///
/// Size: 0x298 (medium). Bases: rage::audDspEffect@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudCompressorEffectPc {
    /// Unknown bytes (0x0..0x10).
    pub _pad_0000: [u8; 0x10],
    /// field_10 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_10: Ptr32<u8>,
    /// Unknown bytes (0x14..0x1f0).
    pub _pad_0014: [u8; 0x1dc],
    /// field_1f0 (confidence: low, kind: double, lanes: c-audio).
    pub field_1f0: [u8; 8],
    /// field_1f8 (confidence: low, kind: double, lanes: c-audio).
    pub field_1f8: [u8; 8],
    /// field_200 (confidence: low, kind: double, lanes: c-audio).
    pub field_200: [u8; 8],
    /// flag_0x208? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x208: u32,
    /// flag_0x20c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x20c: u32,
    /// flag_0x210? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x210: u32,
    /// flag_0x214? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x214: u32,
    /// flag_0x218? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x218: u32,
    /// flag_0x21c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x21c: u32,
    /// flag_0x220? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x220: u32,
    /// flag_0x224? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x224: u32,
    /// flag_0x228? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x228: u32,
    /// flag_0x22c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x22c: u32,
    /// flag_0x230? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x230: u32,
    /// flag_0x234? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x234: u32,
    /// flag_0x238? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x238: u32,
    /// flag_0x23c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x23c: u32,
    /// flag_0x240? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x240: u32,
    /// flag_0x244? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x244: u32,
    /// flag_0x248? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x248: u32,
    /// flag_0x24c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x24c: u32,
    /// field_250 (confidence: low, kind: float, lanes: c-audio).
    pub field_250: f32,
    /// field_254 (confidence: low, kind: float, lanes: c-audio).
    pub field_254: f32,
    /// field_258 (confidence: low, kind: float, lanes: c-audio).
    pub field_258: f32,
    /// field_25c (confidence: low, kind: float, lanes: c-audio).
    pub field_25c: f32,
    /// field_260 (confidence: low, kind: float, lanes: c-audio).
    pub field_260: f32,
    /// field_264 (confidence: medium, kind: float, lanes: c-audio).
    pub field_264: f32,
    /// field_268 (confidence: low, kind: float, lanes: c-audio).
    pub field_268: f32,
    /// field_26c (confidence: low, kind: float, lanes: c-audio).
    pub field_26c: f32,
    /// flag_0x270? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x270: u32,
    /// field_274 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_274: Ptr32<u8>,
    /// field_278 (confidence: low, kind: float, lanes: c-audio).
    pub field_278: f32,
    /// field_27c (confidence: low, kind: float, lanes: c-audio).
    pub field_27c: f32,
    /// field_280 (confidence: low, kind: float, lanes: c-audio).
    pub field_280: f32,
    /// field_284 (confidence: low, kind: float, lanes: c-audio).
    pub field_284: f32,
    /// field_288 (confidence: low, kind: float, lanes: c-audio).
    pub field_288: f32,
    /// field_28c (confidence: low, kind: float, lanes: c-audio).
    pub field_28c: f32,
    /// flag_0x290 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x290: u8,
    /// Unknown bytes (0x291..0x294).
    pub _pad_0291: [u8; 0x3],
    /// field_294 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_294: u8,
    /// flag_0x295 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x295: u8,
    /// field_296 (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_296: u8,
    /// Unknown trailing bytes (0x297..0x298).
    pub _pad_end: [u8; 0x1],
}
assert_size!(RageAudCompressorEffectPc, 0x298); // merged size 0x298 rounded to 4
assert_offset!(RageAudCompressorEffectPc, field_10, 0x10);
assert_offset!(RageAudCompressorEffectPc, field_1f0, 0x1f0);
assert_offset!(RageAudCompressorEffectPc, field_1f8, 0x1f8);
assert_offset!(RageAudCompressorEffectPc, field_200, 0x200);
assert_offset!(RageAudCompressorEffectPc, flag_0x208, 0x208);
assert_offset!(RageAudCompressorEffectPc, flag_0x20c, 0x20c);
assert_offset!(RageAudCompressorEffectPc, flag_0x210, 0x210);
assert_offset!(RageAudCompressorEffectPc, flag_0x214, 0x214);
assert_offset!(RageAudCompressorEffectPc, flag_0x218, 0x218);
assert_offset!(RageAudCompressorEffectPc, flag_0x21c, 0x21c);
assert_offset!(RageAudCompressorEffectPc, flag_0x220, 0x220);
assert_offset!(RageAudCompressorEffectPc, flag_0x224, 0x224);
assert_offset!(RageAudCompressorEffectPc, flag_0x228, 0x228);
assert_offset!(RageAudCompressorEffectPc, flag_0x22c, 0x22c);
assert_offset!(RageAudCompressorEffectPc, flag_0x230, 0x230);
assert_offset!(RageAudCompressorEffectPc, flag_0x234, 0x234);
assert_offset!(RageAudCompressorEffectPc, flag_0x238, 0x238);
assert_offset!(RageAudCompressorEffectPc, flag_0x23c, 0x23c);
assert_offset!(RageAudCompressorEffectPc, flag_0x240, 0x240);
assert_offset!(RageAudCompressorEffectPc, flag_0x244, 0x244);
assert_offset!(RageAudCompressorEffectPc, flag_0x248, 0x248);
assert_offset!(RageAudCompressorEffectPc, flag_0x24c, 0x24c);
assert_offset!(RageAudCompressorEffectPc, field_250, 0x250);
assert_offset!(RageAudCompressorEffectPc, field_254, 0x254);
assert_offset!(RageAudCompressorEffectPc, field_258, 0x258);
assert_offset!(RageAudCompressorEffectPc, field_25c, 0x25c);
assert_offset!(RageAudCompressorEffectPc, field_260, 0x260);
assert_offset!(RageAudCompressorEffectPc, field_264, 0x264);
assert_offset!(RageAudCompressorEffectPc, field_268, 0x268);
assert_offset!(RageAudCompressorEffectPc, field_26c, 0x26c);
assert_offset!(RageAudCompressorEffectPc, flag_0x270, 0x270);
assert_offset!(RageAudCompressorEffectPc, field_274, 0x274);
assert_offset!(RageAudCompressorEffectPc, field_278, 0x278);
assert_offset!(RageAudCompressorEffectPc, field_27c, 0x27c);
assert_offset!(RageAudCompressorEffectPc, field_280, 0x280);
assert_offset!(RageAudCompressorEffectPc, field_284, 0x284);
assert_offset!(RageAudCompressorEffectPc, field_288, 0x288);
assert_offset!(RageAudCompressorEffectPc, field_28c, 0x28c);
assert_offset!(RageAudCompressorEffectPc, flag_0x290, 0x290);
assert_offset!(RageAudCompressorEffectPc, field_294, 0x294);
assert_offset!(RageAudCompressorEffectPc, flag_0x295, 0x295);
assert_offset!(RageAudCompressorEffectPc, field_296, 0x296);

/// Merged layout for `rage::audEffect`.
///
/// Size: 0x78 (low). Bases: none.
/// Lanes: c-audio, via:rage::audBiquadFilterEffect, via:rage::audCompressorEffect, via:rage::audConvolutionEffect, via:rage::audDelayEffect, via:rage::audWaveshaperEffect.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudEffect {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-audio).
    pub vfptr: Ptr32<()>,
    /// flag_0x4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x4: u32,
    /// ptr_0x8 (confidence: high, kind: pointer, lanes: c-audio).
    pub ptr_0x8: Ptr32<u8>,
    /// Unknown bytes (0xc..0x20).
    pub _pad_000c: [u8; 0x14],
    /// flag_0x20? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x20: u32,
    /// flag_0x24? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x24: u32,
    /// flag_0x28? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x28: u32,
    /// field_2c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_2c: Ptr32<u8>,
    /// field_30 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_30: Ptr32<u8>,
    /// Unknown bytes (0x34..0x70).
    pub _pad_0034: [u8; 0x3c],
    /// field_70 (confidence: low, kind: int16, lanes: c-audio).
    pub field_70: u16,
    /// Unknown bytes (0x72..0x74).
    pub _pad_0072: [u8; 0x2],
    /// field_74 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audBiquadFilterEffect,via:rage::audCompressorEffect,via:rage::audConvolutionEffect,via:rage::audDelayEffect,via:rage::audWaveshaperEffect moved from siblings:rage::audBiquadFilterEffect,rage::audCompressorEffect,rage::audConvolutionEffect,rage::audDelayEffect,rage::audWaveshaperEffect).
    pub field_74: Ptr32<u8>,
}
assert_size!(RageAudEffect, 0x78); // merged size 0x78 rounded to 4
assert_offset!(RageAudEffect, vfptr, 0x0);
assert_offset!(RageAudEffect, flag_0x4, 0x4);
assert_offset!(RageAudEffect, ptr_0x8, 0x8);
assert_offset!(RageAudEffect, flag_0x20, 0x20);
assert_offset!(RageAudEffect, flag_0x24, 0x24);
assert_offset!(RageAudEffect, flag_0x28, 0x28);
assert_offset!(RageAudEffect, field_2c, 0x2c);
assert_offset!(RageAudEffect, field_30, 0x30);
assert_offset!(RageAudEffect, field_70, 0x70);
assert_offset!(RageAudEffect, field_74, 0x74);

/// Merged layout for `rage::audEntity`.
///
/// Size: 0x494 (low). Bases: none.
/// Lanes: c-audio, via:audFrontendAudioEntity, via:audPedAudioEntity, via:audPoliceScanner, via:audRadioAudioEntity, via:audScriptAudioEntity, via:audSpeechManager.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudEntity {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-audio).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: int16, lanes: c-audio).
    pub field_4: u16,
    /// Unknown bytes (0x6..0x8).
    pub _pad_0006: [u8; 0x2],
    /// flag_0x8 (confidence: high, kind: bool, lanes: c-audio,via:audScriptAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x8: u8,
    /// Unknown bytes (0x9..0x5c).
    pub _pad_0009: [u8; 0x53],
    /// flag_0x5c? (confidence: medium, kind: bool?, lanes: c-audio,via:audPoliceScanner,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x5c: u32,
    /// Unknown bytes (0x60..0x6c).
    pub _pad_0060: [u8; 0xc],
    /// flag_0x6c? (confidence: medium, kind: bool?, lanes: c-audio,via:audRadioAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x6c: u32,
    /// Unknown bytes (0x70..0xb8).
    pub _pad_0070: [u8; 0x48],
    /// field_b8 (confidence: medium, kind: pointer, lanes: c-audio,via:audPedAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub field_b8: Ptr32<u8>,
    /// Unknown bytes (0xbc..0x128).
    pub _pad_00bc: [u8; 0x6c],
    /// flag_0x128 (confidence: high, kind: bool, lanes: c-audio,via:audPedAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x128: u8,
    /// Unknown bytes (0x129..0x130).
    pub _pad_0129: [u8; 0x7],
    /// flag_0x130? (confidence: medium, kind: bool?, lanes: c-audio,via:audPedAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x130: u32,
    /// flag_0x134? (confidence: medium, kind: bool?, lanes: c-audio,via:audPedAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x134: u32,
    /// Unknown bytes (0x138..0x238).
    pub _pad_0138: [u8; 0x100],
    /// field_238 (confidence: medium, kind: pointer, lanes: c-audio,via:audFrontendAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub field_238: Ptr32<u8>,
    /// Unknown bytes (0x23c..0x358).
    pub _pad_023c: [u8; 0x11c],
    /// field_358 (confidence: medium, kind: pointer, lanes: c-audio,via:audFrontendAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub field_358: Ptr32<u8>,
    /// Unknown bytes (0x35c..0x438).
    pub _pad_035c: [u8; 0xdc],
    /// flag_0x438? (confidence: medium, kind: bool?, lanes: c-audio,via:audFrontendAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x438: u32,
    /// Unknown bytes (0x43c..0x490).
    pub _pad_043c: [u8; 0x54],
    /// flag_0x490? (confidence: medium, kind: bool?, lanes: c-audio,via:audFrontendAudioEntity,via:audSpeechManager moved from siblings:audGtaAudioEntity,audSpeechManager).
    pub flag_0x490: u32,
}
assert_size!(RageAudEntity, 0x494); // merged size 0x494 rounded to 4
assert_offset!(RageAudEntity, vfptr, 0x0);
assert_offset!(RageAudEntity, field_4, 0x4);
assert_offset!(RageAudEntity, flag_0x8, 0x8);
assert_offset!(RageAudEntity, flag_0x5c, 0x5c);
assert_offset!(RageAudEntity, flag_0x6c, 0x6c);
assert_offset!(RageAudEntity, field_b8, 0xb8);
assert_offset!(RageAudEntity, flag_0x128, 0x128);
assert_offset!(RageAudEntity, flag_0x130, 0x130);
assert_offset!(RageAudEntity, flag_0x134, 0x134);
assert_offset!(RageAudEntity, field_238, 0x238);
assert_offset!(RageAudEntity, field_358, 0x358);
assert_offset!(RageAudEntity, flag_0x438, 0x438);
assert_offset!(RageAudEntity, flag_0x490, 0x490);

/// Merged layout for `rage::audForLoopSound`.
///
/// Size: 0xe8 (medium). Bases: rage::audSound@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudForLoopSound {
    /// Unknown bytes (0x0..0xc8).
    pub _pad_0000: [u8; 0xc8],
    /// field_c8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_c8: Ptr32<u8>,
    /// field_cc (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_cc: Ptr32<u8>,
    /// field_d0 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_d0: Ptr32<u8>,
    /// field_d4 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_d4: Ptr32<u8>,
    /// field_d8 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_d8: Ptr32<u8>,
    /// field_dc (confidence: low, kind: pointer, lanes: c-audio).
    pub field_dc: Ptr32<u8>,
    /// Unknown bytes (0xe0..0xe4).
    pub _pad_00e0: [u8; 0x4],
    /// field_e4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_e4: Ptr32<u8>,
}
assert_size!(RageAudForLoopSound, 0xe8); // merged size 0xe8 rounded to 4
assert_offset!(RageAudForLoopSound, field_c8, 0xc8);
assert_offset!(RageAudForLoopSound, field_cc, 0xcc);
assert_offset!(RageAudForLoopSound, field_d0, 0xd0);
assert_offset!(RageAudForLoopSound, field_d4, 0xd4);
assert_offset!(RageAudForLoopSound, field_d8, 0xd8);
assert_offset!(RageAudForLoopSound, field_dc, 0xdc);
assert_offset!(RageAudForLoopSound, field_e4, 0xe4);

/// Merged layout for `rage::audReverbEffectPc`.
///
/// Size: 0xc0 (medium). Bases: rage::audDspEffect@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudReverbEffectPc {
    /// Unknown bytes (0x0..0x64).
    pub _pad_0000: [u8; 0x64],
    /// field_64 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_64: Ptr32<u8>,
    /// field_68 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_68: Ptr32<u8>,
    /// Unknown bytes (0x6c..0x70).
    pub _pad_006c: [u8; 0x4],
    /// field_70 (confidence: medium, kind: pointer, lanes: c-audio).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_78: Ptr32<u8>,
    /// field_7c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_7c: Ptr32<u8>,
    /// field_80 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_80: Ptr32<u8>,
    /// flag_0x84? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x84: u32,
    /// field_88 (confidence: low, kind: int16, lanes: c-audio).
    pub field_88: u16,
    /// Unknown bytes (0x8a..0x8c).
    pub _pad_008a: [u8; 0x2],
    /// field_8c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_8c: Ptr32<u8>,
    /// field_90 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_94: Ptr32<u8>,
    /// flag_0x98? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x98: u32,
    /// field_9c (confidence: low, kind: int16, lanes: c-audio).
    pub field_9c: u16,
    /// Unknown bytes (0x9e..0xa0).
    pub _pad_009e: [u8; 0x2],
    /// field_a0 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_a0: Ptr32<u8>,
    /// field_a4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_a4: Ptr32<u8>,
    /// field_a8 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_a8: Ptr32<u8>,
    /// flag_0xac? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xac: u32,
    /// field_b0 (confidence: low, kind: int16, lanes: c-audio).
    pub field_b0: u16,
    /// Unknown bytes (0xb2..0xb4).
    pub _pad_00b2: [u8; 0x2],
    /// flag_0xb4? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xb4: u32,
    /// flag_0xb8? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xb8: u32,
    /// field_bc (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_bc: u8,
    /// flag_0xbd (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0xbd: u8,
    /// field_be (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_be: u8,
    /// Unknown trailing bytes (0xbf..0xc0).
    pub _pad_end: [u8; 0x1],
}
assert_size!(RageAudReverbEffectPc, 0xc0); // merged size 0xc0 rounded to 4
assert_offset!(RageAudReverbEffectPc, field_64, 0x64);
assert_offset!(RageAudReverbEffectPc, field_68, 0x68);
assert_offset!(RageAudReverbEffectPc, field_70, 0x70);
assert_offset!(RageAudReverbEffectPc, field_74, 0x74);
assert_offset!(RageAudReverbEffectPc, field_78, 0x78);
assert_offset!(RageAudReverbEffectPc, field_7c, 0x7c);
assert_offset!(RageAudReverbEffectPc, field_80, 0x80);
assert_offset!(RageAudReverbEffectPc, flag_0x84, 0x84);
assert_offset!(RageAudReverbEffectPc, field_88, 0x88);
assert_offset!(RageAudReverbEffectPc, field_8c, 0x8c);
assert_offset!(RageAudReverbEffectPc, field_90, 0x90);
assert_offset!(RageAudReverbEffectPc, field_94, 0x94);
assert_offset!(RageAudReverbEffectPc, flag_0x98, 0x98);
assert_offset!(RageAudReverbEffectPc, field_9c, 0x9c);
assert_offset!(RageAudReverbEffectPc, field_a0, 0xa0);
assert_offset!(RageAudReverbEffectPc, field_a4, 0xa4);
assert_offset!(RageAudReverbEffectPc, field_a8, 0xa8);
assert_offset!(RageAudReverbEffectPc, flag_0xac, 0xac);
assert_offset!(RageAudReverbEffectPc, field_b0, 0xb0);
assert_offset!(RageAudReverbEffectPc, flag_0xb4, 0xb4);
assert_offset!(RageAudReverbEffectPc, flag_0xb8, 0xb8);
assert_offset!(RageAudReverbEffectPc, field_bc, 0xbc);
assert_offset!(RageAudReverbEffectPc, flag_0xbd, 0xbd);
assert_offset!(RageAudReverbEffectPc, field_be, 0xbe);

/// Merged layout for `rage::audSound`.
///
/// Size: 0xe4 (low). Bases: none.
/// Lanes: c-audio, via:rage::audEnvelopeSound, via:rage::audEnvironmentSound, via:rage::audForLoopSound, via:rage::audLoopingSound, via:rage::audMultitrackSound, via:rage::audRandomizedSound, via:rage::audRetriggeredOverlappedSound, via:rage::audSequentialSound, via:rage::audSimpleSound, via:rage::audSpeechSound, via:rage::audStreamingSound, via:rage::audSwitchSound.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudSound {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-audio).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4: Ptr32<u8>,
    /// Unknown bytes (0x8..0xc).
    pub _pad_0008: [u8; 0x4],
    /// field_c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_c: Ptr32<u8>,
    /// field_10 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_14: Ptr32<u8>,
    /// field_18 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_18: Ptr32<u8>,
    /// Unknown bytes (0x1c..0x22).
    pub _pad_001c: [u8; 0x6],
    /// field_22 (confidence: low, kind: int16, lanes: c-audio).
    pub field_22: u16,
    /// field_24 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_24: Ptr32<u8>,
    /// Unknown bytes (0x28..0x38).
    pub _pad_0028: [u8; 0x10],
    /// field_38 (confidence: low, kind: int16, lanes: c-audio).
    pub field_38: u16,
    /// field_3a (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_3a: u8,
    /// field_3b (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_3b: u8,
    /// field_3c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_3c: Ptr32<u8>,
    /// field_40 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_40: Ptr32<u8>,
    /// field_44 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_44: Ptr32<u8>,
    /// field_48 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_48: Ptr32<u8>,
    /// field_4c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4c: Ptr32<u8>,
    /// field_50 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_50: Ptr32<u8>,
    /// field_54 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_54: Ptr32<u8>,
    /// Unknown bytes (0x58..0x5c).
    pub _pad_0058: [u8; 0x4],
    /// field_5c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_5c: Ptr32<u8>,
    /// field_60 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_60: Ptr32<u8>,
    /// field_64 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_64: Ptr32<u8>,
    /// Unknown bytes (0x68..0x6c).
    pub _pad_0068: [u8; 0x4],
    /// field_6c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_6c: Ptr32<u8>,
    /// field_70 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audRandomizedSound,via:rage::audRetriggeredOverlappedSound,via:rage::audStreamingSound moved from siblings:rage::audRandomizedSound,rage::audRetriggeredOverlappedSound,rage::audStreamingSound).
    pub field_70: Ptr32<u8>,
    /// field_74 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_74: Ptr32<u8>,
    /// field_78 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_78: Ptr32<u8>,
    /// field_7c (confidence: low, kind: int16, lanes: c-audio).
    pub field_7c: u16,
    /// field_7e (confidence: low, kind: int16, lanes: c-audio).
    pub field_7e: u16,
    /// flag_0x80? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x80: u32,
    /// field_84 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_84: Ptr32<u8>,
    /// field_88 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_88: Ptr32<u8>,
    /// field_8c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_8c: Ptr32<u8>,
    /// field_90 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_90: Ptr32<u8>,
    /// field_94 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_94: Ptr32<u8>,
    /// field_98 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_98: Ptr32<u8>,
    /// Unknown bytes (0x9c..0xa4).
    pub _pad_009c: [u8; 0x8],
    /// field_a4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_a4: Ptr32<u8>,
    /// flag_0xa8? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xa8: u32,
    /// flag_0xac? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0xac: u32,
    /// field_b0 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audEnvelopeSound,via:rage::audEnvironmentSound,via:rage::audMultitrackSound,via:rage::audSequentialSound,via:rage::audSimpleSound,via:rage::audSpeechSound,via:rage::audStreamingSound,via:rage::audSwitchSound moved from siblings:rage::audEnvelopeSound,rage::audEnvironmentSound,rage::audMultitrackSound,rage::audSequentialSound,rage::audSimpleSound,rage::audSpeechSound,rage::audStreamingSound,rage::audSwitchSound).
    pub field_b0: Ptr32<u8>,
    /// flag_0xb4 (confidence: high, kind: byte-or-bool, lanes: c-audio,via:rage::audMultitrackSound,via:rage::audSpeechSound moved from siblings:rage::audMultitrackSound,rage::audSpeechSound).
    pub flag_0xb4: u8,
    /// Unknown bytes (0xb5..0xb8).
    pub _pad_00b5: [u8; 0x3],
    /// field_b8 (confidence: medium, kind: double, lanes: c-audio,via:rage::audForLoopSound,via:rage::audLoopingSound,via:rage::audRetriggeredOverlappedSound moved from siblings:rage::audForLoopSound,rage::audLoopingSound,rage::audRetriggeredOverlappedSound).
    pub field_b8: [u8; 8],
    /// field_c0 (confidence: medium, kind: double, lanes: c-audio,via:rage::audForLoopSound,via:rage::audLoopingSound,via:rage::audRetriggeredOverlappedSound moved from siblings:rage::audForLoopSound,rage::audLoopingSound,rage::audRetriggeredOverlappedSound).
    pub field_c0: [u8; 8],
    /// Unknown bytes (0xc8..0xe3).
    pub _pad_00c8: [u8; 0x1b],
    /// flag_0xe3 (confidence: high, kind: bool, lanes: c-audio,via:rage::audRetriggeredOverlappedSound,via:rage::audStreamingSound moved from siblings:rage::audRetriggeredOverlappedSound,rage::audStreamingSound).
    pub flag_0xe3: u8,
}
assert_size!(RageAudSound, 0xe4); // merged size 0xe4 rounded to 4
assert_offset!(RageAudSound, vfptr, 0x0);
assert_offset!(RageAudSound, field_4, 0x4);
assert_offset!(RageAudSound, field_c, 0xc);
assert_offset!(RageAudSound, field_10, 0x10);
assert_offset!(RageAudSound, field_14, 0x14);
assert_offset!(RageAudSound, field_18, 0x18);
assert_offset!(RageAudSound, field_22, 0x22);
assert_offset!(RageAudSound, field_24, 0x24);
assert_offset!(RageAudSound, field_38, 0x38);
assert_offset!(RageAudSound, field_3a, 0x3a);
assert_offset!(RageAudSound, field_3b, 0x3b);
assert_offset!(RageAudSound, field_3c, 0x3c);
assert_offset!(RageAudSound, field_40, 0x40);
assert_offset!(RageAudSound, field_44, 0x44);
assert_offset!(RageAudSound, field_48, 0x48);
assert_offset!(RageAudSound, field_4c, 0x4c);
assert_offset!(RageAudSound, field_50, 0x50);
assert_offset!(RageAudSound, field_54, 0x54);
assert_offset!(RageAudSound, field_5c, 0x5c);
assert_offset!(RageAudSound, field_60, 0x60);
assert_offset!(RageAudSound, field_64, 0x64);
assert_offset!(RageAudSound, field_6c, 0x6c);
assert_offset!(RageAudSound, field_70, 0x70);
assert_offset!(RageAudSound, field_74, 0x74);
assert_offset!(RageAudSound, field_78, 0x78);
assert_offset!(RageAudSound, field_7c, 0x7c);
assert_offset!(RageAudSound, field_7e, 0x7e);
assert_offset!(RageAudSound, flag_0x80, 0x80);
assert_offset!(RageAudSound, field_84, 0x84);
assert_offset!(RageAudSound, field_88, 0x88);
assert_offset!(RageAudSound, field_8c, 0x8c);
assert_offset!(RageAudSound, field_90, 0x90);
assert_offset!(RageAudSound, field_94, 0x94);
assert_offset!(RageAudSound, field_98, 0x98);
assert_offset!(RageAudSound, field_a4, 0xa4);
assert_offset!(RageAudSound, flag_0xa8, 0xa8);
assert_offset!(RageAudSound, flag_0xac, 0xac);
assert_offset!(RageAudSound, field_b0, 0xb0);
assert_offset!(RageAudSound, flag_0xb4, 0xb4);
assert_offset!(RageAudSound, field_b8, 0xb8);
assert_offset!(RageAudSound, field_c0, 0xc0);
assert_offset!(RageAudSound, flag_0xe3, 0xe3);

/// Merged layout for `rage::audVoicePcAdpcm`.
///
/// Size: 0x154 (medium). Bases: rage::audVoicePhysical@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudVoicePcAdpcm {
    /// Unknown bytes (0x0..0x12c).
    pub _pad_0000: [u8; 0x12c],
    /// field_12c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_12c: Ptr32<u8>,
    /// Unknown bytes (0x130..0x134).
    pub _pad_0130: [u8; 0x4],
    /// ptr_0x134 (confidence: medium, kind: pointer, lanes: c-audio).
    pub ptr_0x134: Ptr32<u8>,
    /// field_138 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_138: Ptr32<u8>,
    /// Unknown bytes (0x13c..0x140).
    pub _pad_013c: [u8; 0x4],
    /// ptr_CEventCommunicateEvent (confidence: medium, kind: pointer, lanes: c-audio).
    pub ptr_ceventcommunicateevent: Ptr32<u8>,
    /// flag_0x144? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x144: u32,
    /// flag_0x148? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x148: u32,
    /// flag_0x14c (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x14c: u8,
    /// Unknown bytes (0x14d..0x14e).
    pub _pad_014d: [u8; 0x1],
    /// field_14e (confidence: low, kind: pointer, lanes: c-audio).
    pub field_14e: [u8; 4],
    /// Unknown trailing bytes (0x152..0x154).
    pub _pad_end: [u8; 0x2],
}
assert_size!(RageAudVoicePcAdpcm, 0x154); // merged size 0x154 rounded to 4
assert_offset!(RageAudVoicePcAdpcm, field_12c, 0x12c);
assert_offset!(RageAudVoicePcAdpcm, ptr_0x134, 0x134);
assert_offset!(RageAudVoicePcAdpcm, field_138, 0x138);
assert_offset!(RageAudVoicePcAdpcm, ptr_ceventcommunicateevent, 0x140);
assert_offset!(RageAudVoicePcAdpcm, flag_0x144, 0x144);
assert_offset!(RageAudVoicePcAdpcm, flag_0x148, 0x148);
assert_offset!(RageAudVoicePcAdpcm, flag_0x14c, 0x14c);
assert_offset!(RageAudVoicePcAdpcm, field_14e, 0x14e);

/// Merged layout for `rage::audVoicePhysical`.
///
/// Size: 0x134 (low). Bases: none.
/// Lanes: c-audio, via:rage::audVoiceDSound, via:rage::audVoiceDSoundAdpcm, via:rage::audVoicePcAdpcm, via:rage::audVoiceSoft.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudVoicePhysical {
    /// vfptr (confidence: high, kind: vtable_ptr, lanes: c-audio).
    pub vfptr: Ptr32<()>,
    /// field_4 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_4: Ptr32<u8>,
    /// flag_0x8? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x8: u32,
    /// field_c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_c: Ptr32<u8>,
    /// ptr_0x10 (confidence: medium, kind: pointer, lanes: c-audio).
    pub ptr_0x10: Ptr32<u8>,
    /// field_14 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_14: Ptr32<u8>,
    /// flag_0x18? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x18: u32,
    /// field_1c (confidence: low, kind: pointer, lanes: c-audio).
    pub field_1c: Ptr32<u8>,
    /// flag_0x20? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x20: u32,
    /// flag_0x24? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x24: u32,
    /// flag_0x28? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x28: u32,
    /// flag_0x2c? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x2c: u32,
    /// flag_0x30? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x30: u32,
    /// flag_0x34? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x34: u32,
    /// Unknown bytes (0x38..0x44).
    pub _pad_0038: [u8; 0xc],
    /// field_44 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_44: Ptr32<u8>,
    /// Unknown bytes (0x48..0x88).
    pub _pad_0048: [u8; 0x40],
    /// field_88 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_88: Ptr32<u8>,
    /// field_8c (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_8c: u8,
    /// Unknown bytes (0x8d..0x90).
    pub _pad_008d: [u8; 0x3],
    /// ptr_0x90 (confidence: high, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm,rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub ptr_0x90: Ptr32<u8>,
    /// ptr_0x94 (confidence: high, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub ptr_0x94: Ptr32<u8>,
    /// field_98 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_98: Ptr32<u8>,
    /// field_9c (confidence: medium, kind: int16, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_9c: u16,
    /// Unknown bytes (0x9e..0xb4).
    pub _pad_009e: [u8; 0x16],
    /// field_b4 (confidence: high, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_b4: Ptr32<u8>,
    /// field_b8 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_b8: Ptr32<u8>,
    /// field_bc (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_bc: Ptr32<u8>,
    /// field_c0 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_c0: Ptr32<u8>,
    /// field_c4 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_c4: Ptr32<u8>,
    /// field_c8 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_c8: Ptr32<u8>,
    /// Unknown bytes (0xcc..0xd4).
    pub _pad_00cc: [u8; 0x8],
    /// field_d4 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_d4: Ptr32<u8>,
    /// field_d8 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_d8: Ptr32<u8>,
    /// ptr_0xdc (confidence: high, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub ptr_0xdc: Ptr32<u8>,
    /// field_e0 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_e0: Ptr32<u8>,
    /// field_e4 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_e4: Ptr32<u8>,
    /// field_e8 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoiceDSound,via:rage::audVoiceDSoundAdpcm moved from siblings:rage::audVoiceDSound,rage::audVoiceDSoundAdpcm).
    pub field_e8: Ptr32<u8>,
    /// Unknown bytes (0xec..0x110).
    pub _pad_00ec: [u8; 0x24],
    /// flag_0x110? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x110: u32,
    /// flag_0x114? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x114: u32,
    /// flag_0x118? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x118: u32,
    /// flag_0x11c? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x11c: u32,
    /// field_120 (confidence: medium, kind: pointer, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub field_120: Ptr32<u8>,
    /// flag_0x124? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x124: u32,
    /// flag_0x128? (confidence: medium, kind: bool?, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub flag_0x128: u32,
    /// Unknown bytes (0x12c..0x130).
    pub _pad_012c: [u8; 0x4],
    /// ptr_sysMemBuddyAllocator (confidence: high, kind: pointer, lanes: c-audio,via:rage::audVoicePcAdpcm,via:rage::audVoiceSoft moved from siblings:rage::audVoicePcAdpcm,rage::audVoiceSoft).
    pub ptr_sysmembuddyallocator: Ptr32<u8>,
}
assert_size!(RageAudVoicePhysical, 0x134); // merged size 0x134 rounded to 4
assert_offset!(RageAudVoicePhysical, vfptr, 0x0);
assert_offset!(RageAudVoicePhysical, field_4, 0x4);
assert_offset!(RageAudVoicePhysical, flag_0x8, 0x8);
assert_offset!(RageAudVoicePhysical, field_c, 0xc);
assert_offset!(RageAudVoicePhysical, ptr_0x10, 0x10);
assert_offset!(RageAudVoicePhysical, field_14, 0x14);
assert_offset!(RageAudVoicePhysical, flag_0x18, 0x18);
assert_offset!(RageAudVoicePhysical, field_1c, 0x1c);
assert_offset!(RageAudVoicePhysical, flag_0x20, 0x20);
assert_offset!(RageAudVoicePhysical, flag_0x24, 0x24);
assert_offset!(RageAudVoicePhysical, flag_0x28, 0x28);
assert_offset!(RageAudVoicePhysical, flag_0x2c, 0x2c);
assert_offset!(RageAudVoicePhysical, flag_0x30, 0x30);
assert_offset!(RageAudVoicePhysical, flag_0x34, 0x34);
assert_offset!(RageAudVoicePhysical, field_44, 0x44);
assert_offset!(RageAudVoicePhysical, field_88, 0x88);
assert_offset!(RageAudVoicePhysical, field_8c, 0x8c);
assert_offset!(RageAudVoicePhysical, ptr_0x90, 0x90);
assert_offset!(RageAudVoicePhysical, ptr_0x94, 0x94);
assert_offset!(RageAudVoicePhysical, field_98, 0x98);
assert_offset!(RageAudVoicePhysical, field_9c, 0x9c);
assert_offset!(RageAudVoicePhysical, field_b4, 0xb4);
assert_offset!(RageAudVoicePhysical, field_b8, 0xb8);
assert_offset!(RageAudVoicePhysical, field_bc, 0xbc);
assert_offset!(RageAudVoicePhysical, field_c0, 0xc0);
assert_offset!(RageAudVoicePhysical, field_c4, 0xc4);
assert_offset!(RageAudVoicePhysical, field_c8, 0xc8);
assert_offset!(RageAudVoicePhysical, field_d4, 0xd4);
assert_offset!(RageAudVoicePhysical, field_d8, 0xd8);
assert_offset!(RageAudVoicePhysical, ptr_0xdc, 0xdc);
assert_offset!(RageAudVoicePhysical, field_e0, 0xe0);
assert_offset!(RageAudVoicePhysical, field_e4, 0xe4);
assert_offset!(RageAudVoicePhysical, field_e8, 0xe8);
assert_offset!(RageAudVoicePhysical, flag_0x110, 0x110);
assert_offset!(RageAudVoicePhysical, flag_0x114, 0x114);
assert_offset!(RageAudVoicePhysical, flag_0x118, 0x118);
assert_offset!(RageAudVoicePhysical, flag_0x11c, 0x11c);
assert_offset!(RageAudVoicePhysical, field_120, 0x120);
assert_offset!(RageAudVoicePhysical, flag_0x124, 0x124);
assert_offset!(RageAudVoicePhysical, flag_0x128, 0x128);
assert_offset!(RageAudVoicePhysical, ptr_sysmembuddyallocator, 0x130);

/// Merged layout for `rage::audWaveshaperEffectPc`.
///
/// Size: 0x28 (medium). Bases: rage::audDspEffect@0x0.
/// Lanes: c-audio.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageAudWaveshaperEffectPc {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// flag_0x8? (confidence: low, kind: bool?, lanes: c-audio).
    pub flag_0x8: u32,
    /// field_c (confidence: medium, kind: float, lanes: c-audio).
    pub field_c: f32,
    /// field_10 (confidence: medium, kind: float, lanes: c-audio).
    pub field_10: f32,
    /// field_14 (confidence: medium, kind: float, lanes: c-audio).
    pub field_14: f32,
    /// Unknown bytes (0x18..0x1c).
    pub _pad_0018: [u8; 0x4],
    /// field_1c (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_1c: u8,
    /// field_1d (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_1d: u8,
    /// flag_0x1e (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x1e: u8,
    /// field_1f (confidence: low, kind: byte-or-bool, lanes: c-audio).
    pub field_1f: u8,
    /// flag_0x20 (confidence: medium, kind: bool, lanes: c-audio).
    pub flag_0x20: u8,
    /// Unknown bytes (0x21..0x24).
    pub _pad_0021: [u8; 0x3],
    /// field_24 (confidence: low, kind: pointer, lanes: c-audio).
    pub field_24: Ptr32<u8>,
}
assert_size!(RageAudWaveshaperEffectPc, 0x28); // merged size 0x28 rounded to 4
assert_offset!(RageAudWaveshaperEffectPc, flag_0x8, 0x8);
assert_offset!(RageAudWaveshaperEffectPc, field_c, 0xc);
assert_offset!(RageAudWaveshaperEffectPc, field_10, 0x10);
assert_offset!(RageAudWaveshaperEffectPc, field_14, 0x14);
assert_offset!(RageAudWaveshaperEffectPc, field_1c, 0x1c);
assert_offset!(RageAudWaveshaperEffectPc, field_1d, 0x1d);
assert_offset!(RageAudWaveshaperEffectPc, flag_0x1e, 0x1e);
assert_offset!(RageAudWaveshaperEffectPc, field_1f, 0x1f);
assert_offset!(RageAudWaveshaperEffectPc, flag_0x20, 0x20);
assert_offset!(RageAudWaveshaperEffectPc, field_24, 0x24);
