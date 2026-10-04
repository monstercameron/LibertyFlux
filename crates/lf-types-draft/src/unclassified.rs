//! Types whose subsystem their names do not show.
//!
//! Holds 2 draft layouts: `rage::spdApical` and `rage::spdShaft`. Their names and fields give no
//! subsystem; they stay here until evidence places them. Every layout is Inferred; size confidence
//! (the analysis lanes' own rating) is high for 0, medium for 0 and low for 2. The conventions are
//! those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `rage::spdApical`.
///
/// Size: 0x401 (low). Bases: none.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSpdApical {
    /// vftable (confidence: high, kind: vtable_ptr, lanes: c-misc-b).
    pub vftable: Ptr32<()>,
    /// field_4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_4: u32,
    /// Unknown bytes (0x8..0x10).
    pub _pad_0008: [u8; 0x8],
    /// embedded_vftable? (confidence: medium, kind: vtable_ptr, lanes: c-misc-b).
    pub embedded_vftable: Ptr32<()>,
    /// field_14 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_14: u32,
    /// field_18 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_18: u32,
    /// field_1c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1c: u32,
    /// Unknown bytes (0x20..0xa0).
    pub _pad_0020: [u8; 0x80],
    /// field_a0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_a0: u32,
    /// Unknown bytes (0xa4..0xe0).
    pub _pad_00a4: [u8; 0x3c],
    /// field_e0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_e0: u32,
    /// field_e4 (confidence: medium, kind: flags, lanes: c-misc-b).
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
    /// field_114 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_114: u32,
    /// field_118 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_118: u32,
    /// Unknown bytes (0x11c..0x130).
    pub _pad_011c: [u8; 0x14],
    /// field_130 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_130: u32,
    /// field_134 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_134: u32,
    /// field_138 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_138: u32,
    /// field_13c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_13c: u32,
    /// field_140 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_140: u32,
    /// field_144 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_144: u32,
    /// field_148 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_148: u32,
    /// field_14c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_14c: u32,
    /// Unknown bytes (0x150..0x2f0).
    pub _pad_0150: [u8; 0x1a0],
    /// field_2f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f0: u32,
    /// field_2f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2f4: u32,
    /// Unknown bytes (0x2f8..0x3f0).
    pub _pad_02f8: [u8; 0xf8],
    /// field_3f0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3f0: u32,
    /// field_3f4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3f4: u32,
    /// Unknown bytes (0x3f8..0x3fc).
    pub _pad_03f8: [u8; 0x4],
    /// field_3fc (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_3fc: u32,
    /// field_400 (confidence: low, kind: bool/byte?, lanes: c-misc-b).
    pub field_400: u8,
    /// Unknown trailing bytes (0x401..0x404).
    pub _pad_end: [u8; 0x3],
}
assert_size!(RageSpdApical, 0x404); // merged size 0x401 rounded to 4
assert_offset!(RageSpdApical, vftable, 0x0);
assert_offset!(RageSpdApical, field_4, 0x4);
assert_offset!(RageSpdApical, embedded_vftable, 0x10);
assert_offset!(RageSpdApical, field_14, 0x14);
assert_offset!(RageSpdApical, field_18, 0x18);
assert_offset!(RageSpdApical, field_1c, 0x1c);
assert_offset!(RageSpdApical, field_a0, 0xa0);
assert_offset!(RageSpdApical, field_e0, 0xe0);
assert_offset!(RageSpdApical, field_e4, 0xe4);
assert_offset!(RageSpdApical, field_e8, 0xe8);
assert_offset!(RageSpdApical, field_f0, 0xf0);
assert_offset!(RageSpdApical, field_f4, 0xf4);
assert_offset!(RageSpdApical, field_f8, 0xf8);
assert_offset!(RageSpdApical, field_100, 0x100);
assert_offset!(RageSpdApical, field_104, 0x104);
assert_offset!(RageSpdApical, field_108, 0x108);
assert_offset!(RageSpdApical, field_110, 0x110);
assert_offset!(RageSpdApical, field_114, 0x114);
assert_offset!(RageSpdApical, field_118, 0x118);
assert_offset!(RageSpdApical, field_130, 0x130);
assert_offset!(RageSpdApical, field_134, 0x134);
assert_offset!(RageSpdApical, field_138, 0x138);
assert_offset!(RageSpdApical, field_13c, 0x13c);
assert_offset!(RageSpdApical, field_140, 0x140);
assert_offset!(RageSpdApical, field_144, 0x144);
assert_offset!(RageSpdApical, field_148, 0x148);
assert_offset!(RageSpdApical, field_14c, 0x14c);
assert_offset!(RageSpdApical, field_2f0, 0x2f0);
assert_offset!(RageSpdApical, field_2f4, 0x2f4);
assert_offset!(RageSpdApical, field_3f0, 0x3f0);
assert_offset!(RageSpdApical, field_3f4, 0x3f4);
assert_offset!(RageSpdApical, field_3fc, 0x3fc);
assert_offset!(RageSpdApical, field_400, 0x400);

/// Merged layout for `rage::spdShaft`.
///
/// Size: 0x2c3 (low). Bases: rage::spdApical@0x0.
/// Lanes: c-misc-b.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RageSpdShaft {
    /// Unknown bytes (0x0..0x8).
    pub _pad_0000: [u8; 0x8],
    /// field_8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_8: u32,
    /// Unknown bytes (0xc..0x150).
    pub _pad_000c: [u8; 0x144],
    /// field_150 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_150: u32,
    /// field_154 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_154: u32,
    /// field_158 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_158: u32,
    /// field_15c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_15c: u32,
    /// field_160 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_160: u32,
    /// field_164 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_164: u32,
    /// field_168 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_168: u32,
    /// field_16c (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_16c: u32,
    /// field_170 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_170: u32,
    /// field_174 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_174: u32,
    /// field_178 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_178: u32,
    /// field_17c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_17c: u32,
    /// field_180 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_180: u32,
    /// field_184 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_184: u32,
    /// field_188 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_188: u32,
    /// field_18c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_18c: u32,
    /// field_190 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_190: u32,
    /// field_194 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_194: u32,
    /// field_198 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_198: u32,
    /// field_19c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_19c: u32,
    /// field_1a0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1a0: u32,
    /// field_1a4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1a4: u32,
    /// field_1a8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1a8: u32,
    /// field_1ac (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1ac: u32,
    /// field_1b0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1b0: u32,
    /// field_1b4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1b4: u32,
    /// field_1b8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1b8: u32,
    /// field_1bc (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1bc: u32,
    /// field_1c0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c0: u32,
    /// field_1c4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c4: u32,
    /// field_1c8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1c8: u32,
    /// field_1cc (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_1cc: u32,
    /// field_1d0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1d0: u32,
    /// field_1d4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1d4: u32,
    /// field_1d8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1d8: u32,
    /// field_1dc (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1dc: u32,
    /// field_1e0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1e0: u32,
    /// field_1e4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1e4: u32,
    /// field_1e8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1e8: u32,
    /// field_1ec (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1ec: u32,
    /// field_1f0 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1f0: u32,
    /// field_1f4 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1f4: u32,
    /// field_1f8 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1f8: u32,
    /// field_1fc (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_1fc: u32,
    /// field_200 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_200: u32,
    /// field_204 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_204: u32,
    /// field_208 (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_208: u32,
    /// field_20c (confidence: medium, kind: flags, lanes: c-misc-b).
    pub field_20c: u32,
    /// field_210 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_210: u32,
    /// field_214 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_214: u32,
    /// field_218 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_218: u32,
    /// Unknown bytes (0x21c..0x220).
    pub _pad_021c: [u8; 0x4],
    /// field_220 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_220: u32,
    /// field_224 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_224: u32,
    /// field_228 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_228: u32,
    /// Unknown bytes (0x22c..0x230).
    pub _pad_022c: [u8; 0x4],
    /// field_230 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_230: u32,
    /// field_234 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_234: u32,
    /// field_238 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_238: u32,
    /// Unknown bytes (0x23c..0x240).
    pub _pad_023c: [u8; 0x4],
    /// field_240 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_240: u32,
    /// field_244 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_244: u32,
    /// field_248 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_248: u32,
    /// Unknown bytes (0x24c..0x250).
    pub _pad_024c: [u8; 0x4],
    /// field_250 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_250: u32,
    /// field_254 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_254: u32,
    /// field_258 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_258: u32,
    /// Unknown bytes (0x25c..0x260).
    pub _pad_025c: [u8; 0x4],
    /// field_260 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_260: u32,
    /// field_264 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_264: u32,
    /// field_268 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_268: u32,
    /// Unknown bytes (0x26c..0x270).
    pub _pad_026c: [u8; 0x4],
    /// field_270 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_270: u32,
    /// field_274 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_274: u32,
    /// field_278 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_278: u32,
    /// Unknown bytes (0x27c..0x280).
    pub _pad_027c: [u8; 0x4],
    /// field_280 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_280: u32,
    /// field_284 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_284: u32,
    /// field_288 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_288: u32,
    /// Unknown bytes (0x28c..0x290).
    pub _pad_028c: [u8; 0x4],
    /// field_290 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_290: u32,
    /// field_294 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_294: u32,
    /// field_298 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_298: u32,
    /// Unknown bytes (0x29c..0x2a0).
    pub _pad_029c: [u8; 0x4],
    /// field_2a0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2a0: u32,
    /// field_2a4 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2a4: u32,
    /// field_2a8 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2a8: u32,
    /// Unknown bytes (0x2ac..0x2b0).
    pub _pad_02ac: [u8; 0x4],
    /// field_2b0 (confidence: low, kind: flags, lanes: c-misc-b).
    pub field_2b0: u32,
    /// Unknown bytes (0x2b4..0x2c0).
    pub _pad_02b4: [u8; 0xc],
    /// field_2c0 (confidence: high, kind: bool/byte?, lanes: c-misc-b).
    pub field_2c0: u8,
    /// Unknown bytes (0x2c1..0x2c2).
    pub _pad_02c1: [u8; 0x1],
    /// field_2c2 (confidence: high, kind: bool/byte?, lanes: c-misc-b).
    pub field_2c2: u8,
    /// Unknown trailing bytes (0x2c3..0x2c4).
    pub _pad_end: [u8; 0x1],
}
assert_size!(RageSpdShaft, 0x2c4); // merged size 0x2c3 rounded to 4
assert_offset!(RageSpdShaft, field_8, 0x8);
assert_offset!(RageSpdShaft, field_150, 0x150);
assert_offset!(RageSpdShaft, field_154, 0x154);
assert_offset!(RageSpdShaft, field_158, 0x158);
assert_offset!(RageSpdShaft, field_15c, 0x15c);
assert_offset!(RageSpdShaft, field_160, 0x160);
assert_offset!(RageSpdShaft, field_164, 0x164);
assert_offset!(RageSpdShaft, field_168, 0x168);
assert_offset!(RageSpdShaft, field_16c, 0x16c);
assert_offset!(RageSpdShaft, field_170, 0x170);
assert_offset!(RageSpdShaft, field_174, 0x174);
assert_offset!(RageSpdShaft, field_178, 0x178);
assert_offset!(RageSpdShaft, field_17c, 0x17c);
assert_offset!(RageSpdShaft, field_180, 0x180);
assert_offset!(RageSpdShaft, field_184, 0x184);
assert_offset!(RageSpdShaft, field_188, 0x188);
assert_offset!(RageSpdShaft, field_18c, 0x18c);
assert_offset!(RageSpdShaft, field_190, 0x190);
assert_offset!(RageSpdShaft, field_194, 0x194);
assert_offset!(RageSpdShaft, field_198, 0x198);
assert_offset!(RageSpdShaft, field_19c, 0x19c);
assert_offset!(RageSpdShaft, field_1a0, 0x1a0);
assert_offset!(RageSpdShaft, field_1a4, 0x1a4);
assert_offset!(RageSpdShaft, field_1a8, 0x1a8);
assert_offset!(RageSpdShaft, field_1ac, 0x1ac);
assert_offset!(RageSpdShaft, field_1b0, 0x1b0);
assert_offset!(RageSpdShaft, field_1b4, 0x1b4);
assert_offset!(RageSpdShaft, field_1b8, 0x1b8);
assert_offset!(RageSpdShaft, field_1bc, 0x1bc);
assert_offset!(RageSpdShaft, field_1c0, 0x1c0);
assert_offset!(RageSpdShaft, field_1c4, 0x1c4);
assert_offset!(RageSpdShaft, field_1c8, 0x1c8);
assert_offset!(RageSpdShaft, field_1cc, 0x1cc);
assert_offset!(RageSpdShaft, field_1d0, 0x1d0);
assert_offset!(RageSpdShaft, field_1d4, 0x1d4);
assert_offset!(RageSpdShaft, field_1d8, 0x1d8);
assert_offset!(RageSpdShaft, field_1dc, 0x1dc);
assert_offset!(RageSpdShaft, field_1e0, 0x1e0);
assert_offset!(RageSpdShaft, field_1e4, 0x1e4);
assert_offset!(RageSpdShaft, field_1e8, 0x1e8);
assert_offset!(RageSpdShaft, field_1ec, 0x1ec);
assert_offset!(RageSpdShaft, field_1f0, 0x1f0);
assert_offset!(RageSpdShaft, field_1f4, 0x1f4);
assert_offset!(RageSpdShaft, field_1f8, 0x1f8);
assert_offset!(RageSpdShaft, field_1fc, 0x1fc);
assert_offset!(RageSpdShaft, field_200, 0x200);
assert_offset!(RageSpdShaft, field_204, 0x204);
assert_offset!(RageSpdShaft, field_208, 0x208);
assert_offset!(RageSpdShaft, field_20c, 0x20c);
assert_offset!(RageSpdShaft, field_210, 0x210);
assert_offset!(RageSpdShaft, field_214, 0x214);
assert_offset!(RageSpdShaft, field_218, 0x218);
assert_offset!(RageSpdShaft, field_220, 0x220);
assert_offset!(RageSpdShaft, field_224, 0x224);
assert_offset!(RageSpdShaft, field_228, 0x228);
assert_offset!(RageSpdShaft, field_230, 0x230);
assert_offset!(RageSpdShaft, field_234, 0x234);
assert_offset!(RageSpdShaft, field_238, 0x238);
assert_offset!(RageSpdShaft, field_240, 0x240);
assert_offset!(RageSpdShaft, field_244, 0x244);
assert_offset!(RageSpdShaft, field_248, 0x248);
assert_offset!(RageSpdShaft, field_250, 0x250);
assert_offset!(RageSpdShaft, field_254, 0x254);
assert_offset!(RageSpdShaft, field_258, 0x258);
assert_offset!(RageSpdShaft, field_260, 0x260);
assert_offset!(RageSpdShaft, field_264, 0x264);
assert_offset!(RageSpdShaft, field_268, 0x268);
assert_offset!(RageSpdShaft, field_270, 0x270);
assert_offset!(RageSpdShaft, field_274, 0x274);
assert_offset!(RageSpdShaft, field_278, 0x278);
assert_offset!(RageSpdShaft, field_280, 0x280);
assert_offset!(RageSpdShaft, field_284, 0x284);
assert_offset!(RageSpdShaft, field_288, 0x288);
assert_offset!(RageSpdShaft, field_290, 0x290);
assert_offset!(RageSpdShaft, field_294, 0x294);
assert_offset!(RageSpdShaft, field_298, 0x298);
assert_offset!(RageSpdShaft, field_2a0, 0x2a0);
assert_offset!(RageSpdShaft, field_2a4, 0x2a4);
assert_offset!(RageSpdShaft, field_2a8, 0x2a8);
assert_offset!(RageSpdShaft, field_2b0, 0x2b0);
assert_offset!(RageSpdShaft, field_2c0, 0x2c0);
assert_offset!(RageSpdShaft, field_2c2, 0x2c2);
