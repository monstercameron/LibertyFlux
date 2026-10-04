//! Vehicles.
//!
//! Holds 7 draft layouts: `CVehicle` and the classes that name it, directly or through
//! `CAutomobile`, as their base. Every layout is Inferred; size confidence (the analysis lanes' own
//! rating) is high for 0, medium for 0 and low for 7. The conventions are those of the crate root.

use crate::Ptr32;
use lf_core::{assert_offset, assert_size};

/// Merged layout for `CAutomobile`.
///
/// Size: 0x1f0c (low). Bases: CVehicle@0x0.
/// Lanes: c-entities, via:CHeli, via:CPlane.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CAutomobile {
    /// Unknown bytes (0x0..0xe6e).
    pub _pad_0000: [u8; 0xe6e],
    /// field_e6e (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_e6e: u8,
    /// Unknown bytes (0xe6f..0x10ac).
    pub _pad_0e6f: [u8; 0x23d],
    /// field_10ac (confidence: medium, kind: float, lanes: c-entities).
    pub field_10ac: f32,
    /// Unknown bytes (0x10b0..0x1440).
    pub _pad_10b0: [u8; 0x390],
    /// field_1440 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1440: f32,
    /// field_1444 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1444: f32,
    /// field_1448 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1448: f32,
    /// field_144c (confidence: medium, kind: float, lanes: c-entities).
    pub field_144c: f32,
    /// field_1450 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1450: f32,
    /// Unknown bytes (0x1454..0x1460).
    pub _pad_1454: [u8; 0xc],
    /// field_1460 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1460: f32,
    /// field_1464 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1464: f32,
    /// field_1468 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1468: f32,
    /// field_146c (confidence: medium, kind: float, lanes: c-entities).
    pub field_146c: f32,
    /// Unknown bytes (0x1470..0x1480).
    pub _pad_1470: [u8; 0x10],
    /// field_1480 (confidence: high, kind: float, lanes: c-entities).
    pub field_1480: f32,
    /// Unknown bytes (0x1484..0x1490).
    pub _pad_1484: [u8; 0xc],
    /// field_1490 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_1490: Ptr32<u8>,
    /// Unknown bytes (0x1494..0x1498).
    pub _pad_1494: [u8; 0x4],
    /// field_1498 (confidence: high, kind: float, lanes: c-entities).
    pub field_1498: f32,
    /// field_149c (confidence: high, kind: float, lanes: c-entities).
    pub field_149c: f32,
    /// Unknown bytes (0x14a0..0x14b0).
    pub _pad_14a0: [u8; 0x10],
    /// field_14b0 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_14b0: Ptr32<u8>,
    /// Unknown bytes (0x14b4..0x1d70).
    pub _pad_14b4: [u8; 0x8bc],
    /// field_1d70 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1d70: u32,
    /// Unknown bytes (0x1d74..0x1eb4).
    pub _pad_1d74: [u8; 0x140],
    /// field_1eb4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1eb4: u32,
    /// field_1eb8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1eb8: u32,
    /// field_1ebc (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1ebc: u32,
    /// field_1ec0 (confidence: high, kind: u32, lanes: c-entities,n-18).
    pub field_1ec0: u32,
    /// Unknown bytes (0x1ec4..0x1ecc).
    pub _pad_1ec4: [u8; 0x8],
    /// field_1ecc (confidence: high, kind: u32, lanes: c-entities,via:CHeli,via:CPlane moved from siblings:CHeli,CPlane).
    pub field_1ecc: u32,
    /// field_1ed0 (confidence: high, kind: u32, lanes: c-entities,via:CHeli,via:CPlane moved from siblings:CHeli,CPlane).
    pub field_1ed0: u32,
    /// Unknown bytes (0x1ed4..0x1ee0).
    pub _pad_1ed4: [u8; 0xc],
    /// field_1ee0 (confidence: high, kind: u32, lanes: c-entities,via:CHeli,via:CPlane moved from siblings:CHeli,CPlane).
    pub field_1ee0: u32,
    /// Unknown bytes (0x1ee4..0x1efc).
    pub _pad_1ee4: [u8; 0x18],
    /// field_1efc (confidence: high, kind: u32, lanes: c-entities,via:CHeli,via:CPlane moved from siblings:CHeli,CPlane).
    pub field_1efc: u32,
    /// field_1f00 (confidence: medium, kind: u32, lanes: c-entities,via:CHeli,via:CPlane moved from siblings:CHeli,CPlane).
    pub field_1f00: u32,
    /// Unknown bytes (0x1f04..0x1f0b).
    pub _pad_1f04: [u8; 0x7],
    /// field_1f0b (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1f0b: u8,
}
assert_size!(CAutomobile, 0x1f0c); // merged size 0x1f0c rounded to 4
assert_offset!(CAutomobile, field_e6e, 0xe6e);
assert_offset!(CAutomobile, field_10ac, 0x10ac);
assert_offset!(CAutomobile, field_1440, 0x1440);
assert_offset!(CAutomobile, field_1444, 0x1444);
assert_offset!(CAutomobile, field_1448, 0x1448);
assert_offset!(CAutomobile, field_144c, 0x144c);
assert_offset!(CAutomobile, field_1450, 0x1450);
assert_offset!(CAutomobile, field_1460, 0x1460);
assert_offset!(CAutomobile, field_1464, 0x1464);
assert_offset!(CAutomobile, field_1468, 0x1468);
assert_offset!(CAutomobile, field_146c, 0x146c);
assert_offset!(CAutomobile, field_1480, 0x1480);
assert_offset!(CAutomobile, field_1490, 0x1490);
assert_offset!(CAutomobile, field_1498, 0x1498);
assert_offset!(CAutomobile, field_149c, 0x149c);
assert_offset!(CAutomobile, field_14b0, 0x14b0);
assert_offset!(CAutomobile, field_1d70, 0x1d70);
assert_offset!(CAutomobile, field_1eb4, 0x1eb4);
assert_offset!(CAutomobile, field_1eb8, 0x1eb8);
assert_offset!(CAutomobile, field_1ebc, 0x1ebc);
assert_offset!(CAutomobile, field_1ec0, 0x1ec0);
assert_offset!(CAutomobile, field_1ecc, 0x1ecc);
assert_offset!(CAutomobile, field_1ed0, 0x1ed0);
assert_offset!(CAutomobile, field_1ee0, 0x1ee0);
assert_offset!(CAutomobile, field_1efc, 0x1efc);
assert_offset!(CAutomobile, field_1f00, 0x1f00);
assert_offset!(CAutomobile, field_1f0b, 0x1f0b);

/// Merged layout for `CBike`.
///
/// Size: 0x1734 (low). Bases: CVehicle@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBike {
    /// Unknown bytes (0x0..0x13a0).
    pub _pad_0000: [u8; 0x13a0],
    /// field_13a0 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13a0: f32,
    /// field_13a4 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13a4: f32,
    /// field_13a8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13a8: f32,
    /// field_13ac (confidence: medium, kind: float, lanes: c-entities).
    pub field_13ac: f32,
    /// field_13b0 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13b0: f32,
    /// Unknown bytes (0x13b4..0x13c0).
    pub _pad_13b4: [u8; 0xc],
    /// field_13c0 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13c0: f32,
    /// field_13c4 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13c4: f32,
    /// field_13c8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_13c8: f32,
    /// field_13cc (confidence: medium, kind: float, lanes: c-entities).
    pub field_13cc: f32,
    /// field_13d0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_13d0: u32,
    /// Unknown bytes (0x13d4..0x13e0).
    pub _pad_13d4: [u8; 0xc],
    /// field_13e0 (confidence: high, kind: float, lanes: c-entities).
    pub field_13e0: f32,
    /// Unknown bytes (0x13e4..0x13f0).
    pub _pad_13e4: [u8; 0xc],
    /// field_13f0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_13f0: u32,
    /// field_13f4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_13f4: u32,
    /// field_13f8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_13f8: u32,
    /// Unknown bytes (0x13fc..0x1400).
    pub _pad_13fc: [u8; 0x4],
    /// field_1400 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1400: u32,
    /// field_1404 (confidence: high, kind: float, lanes: c-entities).
    pub field_1404: f32,
    /// field_1408 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1408: u32,
    /// Unknown bytes (0x140c..0x1410).
    pub _pad_140c: [u8; 0x4],
    /// field_1410 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1410: f32,
    /// field_1414 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1414: f32,
    /// field_1418 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1418: f32,
    /// Unknown bytes (0x141c..0x1420).
    pub _pad_141c: [u8; 0x4],
    /// field_1420 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1420: u32,
    /// field_1424 (confidence: high, kind: float, lanes: c-entities).
    pub field_1424: f32,
    /// Unknown bytes (0x1428..0x142c).
    pub _pad_1428: [u8; 0x4],
    /// field_142c (confidence: low, kind: u32, lanes: c-entities).
    pub field_142c: u32,
    /// Unknown bytes (0x1430..0x1438).
    pub _pad_1430: [u8; 0x8],
    /// field_1438 (confidence: high, kind: float, lanes: c-entities).
    pub field_1438: f32,
    /// field_143c (confidence: low, kind: u32, lanes: c-entities).
    pub field_143c: u32,
    /// field_1440 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_1440: u8,
    /// Unknown bytes (0x1441..0x1444).
    pub _pad_1441: [u8; 0x3],
    /// field_1444 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1444: u32,
    /// Unknown bytes (0x1448..0x1450).
    pub _pad_1448: [u8; 0x8],
    /// field_1450 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1450: u32,
    /// Unknown bytes (0x1454..0x1730).
    pub _pad_1454: [u8; 0x2dc],
    /// field_1730 (confidence: high, kind: u32, lanes: c-entities).
    pub field_1730: u32,
}
assert_size!(CBike, 0x1734); // merged size 0x1734 rounded to 4
assert_offset!(CBike, field_13a0, 0x13a0);
assert_offset!(CBike, field_13a4, 0x13a4);
assert_offset!(CBike, field_13a8, 0x13a8);
assert_offset!(CBike, field_13ac, 0x13ac);
assert_offset!(CBike, field_13b0, 0x13b0);
assert_offset!(CBike, field_13c0, 0x13c0);
assert_offset!(CBike, field_13c4, 0x13c4);
assert_offset!(CBike, field_13c8, 0x13c8);
assert_offset!(CBike, field_13cc, 0x13cc);
assert_offset!(CBike, field_13d0, 0x13d0);
assert_offset!(CBike, field_13e0, 0x13e0);
assert_offset!(CBike, field_13f0, 0x13f0);
assert_offset!(CBike, field_13f4, 0x13f4);
assert_offset!(CBike, field_13f8, 0x13f8);
assert_offset!(CBike, field_1400, 0x1400);
assert_offset!(CBike, field_1404, 0x1404);
assert_offset!(CBike, field_1408, 0x1408);
assert_offset!(CBike, field_1410, 0x1410);
assert_offset!(CBike, field_1414, 0x1414);
assert_offset!(CBike, field_1418, 0x1418);
assert_offset!(CBike, field_1420, 0x1420);
assert_offset!(CBike, field_1424, 0x1424);
assert_offset!(CBike, field_142c, 0x142c);
assert_offset!(CBike, field_1438, 0x1438);
assert_offset!(CBike, field_143c, 0x143c);
assert_offset!(CBike, field_1440, 0x1440);
assert_offset!(CBike, field_1444, 0x1444);
assert_offset!(CBike, field_1450, 0x1450);
assert_offset!(CBike, field_1730, 0x1730);

/// Merged layout for `CBoat`.
///
/// Size: 0x190c (low). Bases: CVehicle@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBoat {
    /// Unknown bytes (0x0..0x1640).
    pub _pad_0000: [u8; 0x1640],
    /// field_1640 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1640: u32,
    /// field_1644 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1644: u32,
    /// field_1648 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1648: u32,
    /// field_164c (confidence: high, kind: float, lanes: c-entities).
    pub field_164c: f32,
    /// field_1650 (confidence: high, kind: u32, lanes: c-entities).
    pub field_1650: u32,
    /// field_1654 (confidence: high, kind: float, lanes: c-entities).
    pub field_1654: f32,
    /// field_1658 (confidence: high, kind: float, lanes: c-entities).
    pub field_1658: f32,
    /// field_165c (confidence: low, kind: u32, lanes: c-entities).
    pub field_165c: u32,
    /// field_1660 (confidence: high, kind: u32, lanes: c-entities).
    pub field_1660: u32,
    /// field_1664 (confidence: high, kind: float, lanes: c-entities).
    pub field_1664: f32,
    /// field_1668 (confidence: high, kind: float, lanes: c-entities).
    pub field_1668: f32,
    /// field_166c (confidence: low, kind: u32, lanes: c-entities).
    pub field_166c: u32,
    /// field_1670 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1670: u32,
    /// field_1674 (confidence: high, kind: float, lanes: c-entities).
    pub field_1674: f32,
    /// field_1678 (confidence: high, kind: float, lanes: c-entities).
    pub field_1678: f32,
    /// field_167c (confidence: high, kind: u32, lanes: c-entities).
    pub field_167c: u32,
    /// field_1680 (confidence: high, kind: u32, lanes: c-entities).
    pub field_1680: u32,
    /// Unknown bytes (0x1684..0x1908).
    pub _pad_1684: [u8; 0x284],
    /// field_1908 (confidence: high, kind: float, lanes: c-entities).
    pub field_1908: f32,
}
assert_size!(CBoat, 0x190c); // merged size 0x190c rounded to 4
assert_offset!(CBoat, field_1640, 0x1640);
assert_offset!(CBoat, field_1644, 0x1644);
assert_offset!(CBoat, field_1648, 0x1648);
assert_offset!(CBoat, field_164c, 0x164c);
assert_offset!(CBoat, field_1650, 0x1650);
assert_offset!(CBoat, field_1654, 0x1654);
assert_offset!(CBoat, field_1658, 0x1658);
assert_offset!(CBoat, field_165c, 0x165c);
assert_offset!(CBoat, field_1660, 0x1660);
assert_offset!(CBoat, field_1664, 0x1664);
assert_offset!(CBoat, field_1668, 0x1668);
assert_offset!(CBoat, field_166c, 0x166c);
assert_offset!(CBoat, field_1670, 0x1670);
assert_offset!(CBoat, field_1674, 0x1674);
assert_offset!(CBoat, field_1678, 0x1678);
assert_offset!(CBoat, field_167c, 0x167c);
assert_offset!(CBoat, field_1680, 0x1680);
assert_offset!(CBoat, field_1908, 0x1908);

/// Merged layout for `CHeli`.
///
/// Size: 0x2004 (low). Bases: CAutomobile@0x0.
/// Lanes: c-entities.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHeli {
    /// Unknown bytes (0x0..0x1eb0).
    pub _pad_0000: [u8; 0x1eb0],
    /// field_1eb0 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_1eb0: u8,
    /// Unknown bytes (0x1eb1..0x1ec4).
    pub _pad_1eb1: [u8; 0x13],
    /// field_1ec4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ec4: f32,
    /// field_1ec8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ec8: f32,
    /// Unknown bytes (0x1ecc..0x1ed4).
    pub _pad_1ecc: [u8; 0x8],
    /// field_1ed4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ed4: f32,
    /// field_1ed8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ed8: f32,
    /// field_1edc (confidence: high, kind: float, lanes: c-entities).
    pub field_1edc: f32,
    /// Unknown bytes (0x1ee0..0x1ee4).
    pub _pad_1ee0: [u8; 0x4],
    /// field_1ee4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ee4: f32,
    /// field_1ee8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ee8: f32,
    /// field_1eec (confidence: low, kind: u32, lanes: c-entities).
    pub field_1eec: u32,
    /// Unknown bytes (0x1ef0..0x1ef2).
    pub _pad_1ef0: [u8; 0x2],
    /// field_1ef2 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_1ef2: u8,
    /// Unknown bytes (0x1ef3..0x1ef4).
    pub _pad_1ef3: [u8; 0x1],
    /// field_1ef4 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ef4: f32,
    /// field_1ef8 (confidence: high, kind: float, lanes: c-entities).
    pub field_1ef8: f32,
    /// Unknown bytes (0x1efc..0x1f04).
    pub _pad_1efc: [u8; 0x8],
    /// field_1f04 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1f04: u8,
    /// field_1f05 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_1f05: u8,
    /// field_1f06 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1f06: [u8; 4],
    /// Unknown bytes (0x1f0a..0x1f0c).
    pub _pad_1f0a: [u8; 0x2],
    /// field_1f0c (confidence: high, kind: float, lanes: c-entities).
    pub field_1f0c: f32,
    /// field_1f10 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f10: f32,
    /// field_1f14 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f14: f32,
    /// field_1f18 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f18: u32,
    /// field_1f1c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f1c: u32,
    /// field_1f20 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f20: u32,
    /// field_1f24 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f24: f32,
    /// field_1f28 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f28: f32,
    /// field_1f2c (confidence: high, kind: float, lanes: c-entities).
    pub field_1f2c: f32,
    /// field_1f30 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f30: u32,
    /// field_1f34 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f34: u32,
    /// field_1f38 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f38: u32,
    /// field_1f3c (confidence: high, kind: float, lanes: c-entities).
    pub field_1f3c: f32,
    /// field_1f40 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f40: f32,
    /// field_1f44 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f44: f32,
    /// field_1f48 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f48: u32,
    /// field_1f4c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f4c: u32,
    /// field_1f50 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f50: u32,
    /// field_1f54 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1f54: u32,
    /// field_1f58 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f58: f32,
    /// field_1f5c (confidence: high, kind: float, lanes: c-entities).
    pub field_1f5c: f32,
    /// field_1f60 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f60: f32,
    /// field_1f64 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f64: f32,
    /// field_1f68 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1f68: u32,
    /// field_1f6c (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_1f6c: u8,
    /// field_1f6d (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_1f6d: u8,
    /// Unknown bytes (0x1f6e..0x1f70).
    pub _pad_1f6e: [u8; 0x2],
    /// field_1f70 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f70: f32,
    /// field_1f74 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f74: f32,
    /// field_1f78 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f78: f32,
    /// Unknown bytes (0x1f7c..0x1f80).
    pub _pad_1f7c: [u8; 0x4],
    /// field_1f80 (confidence: high, kind: float, lanes: c-entities).
    pub field_1f80: f32,
    /// field_1f84 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f84: f32,
    /// field_1f88 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f88: f32,
    /// Unknown bytes (0x1f8c..0x1f90).
    pub _pad_1f8c: [u8; 0x4],
    /// field_1f90 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f90: f32,
    /// field_1f94 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f94: f32,
    /// field_1f98 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1f98: f32,
    /// Unknown bytes (0x1f9c..0x1fa0).
    pub _pad_1f9c: [u8; 0x4],
    /// field_1fa0 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1fa0: f32,
    /// field_1fa4 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1fa4: f32,
    /// field_1fa8 (confidence: medium, kind: float, lanes: c-entities).
    pub field_1fa8: f32,
    /// Unknown bytes (0x1fac..0x1fd0).
    pub _pad_1fac: [u8; 0x24],
    /// field_1fd0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fd0: u32,
    /// field_1fd4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fd4: u32,
    /// field_1fd8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fd8: u32,
    /// Unknown bytes (0x1fdc..0x1fe0).
    pub _pad_1fdc: [u8; 0x4],
    /// field_1fe0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fe0: u32,
    /// field_1fe4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fe4: u32,
    /// field_1fe8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1fe8: u32,
    /// Unknown bytes (0x1fec..0x2000).
    pub _pad_1fec: [u8; 0x14],
    /// field_2000 (confidence: low, kind: u32, lanes: c-entities).
    pub field_2000: u32,
}
assert_size!(CHeli, 0x2004); // merged size 0x2004 rounded to 4
assert_offset!(CHeli, field_1eb0, 0x1eb0);
assert_offset!(CHeli, field_1ec4, 0x1ec4);
assert_offset!(CHeli, field_1ec8, 0x1ec8);
assert_offset!(CHeli, field_1ed4, 0x1ed4);
assert_offset!(CHeli, field_1ed8, 0x1ed8);
assert_offset!(CHeli, field_1edc, 0x1edc);
assert_offset!(CHeli, field_1ee4, 0x1ee4);
assert_offset!(CHeli, field_1ee8, 0x1ee8);
assert_offset!(CHeli, field_1eec, 0x1eec);
assert_offset!(CHeli, field_1ef2, 0x1ef2);
assert_offset!(CHeli, field_1ef4, 0x1ef4);
assert_offset!(CHeli, field_1ef8, 0x1ef8);
assert_offset!(CHeli, field_1f04, 0x1f04);
assert_offset!(CHeli, field_1f05, 0x1f05);
assert_offset!(CHeli, field_1f06, 0x1f06);
assert_offset!(CHeli, field_1f0c, 0x1f0c);
assert_offset!(CHeli, field_1f10, 0x1f10);
assert_offset!(CHeli, field_1f14, 0x1f14);
assert_offset!(CHeli, field_1f18, 0x1f18);
assert_offset!(CHeli, field_1f1c, 0x1f1c);
assert_offset!(CHeli, field_1f20, 0x1f20);
assert_offset!(CHeli, field_1f24, 0x1f24);
assert_offset!(CHeli, field_1f28, 0x1f28);
assert_offset!(CHeli, field_1f2c, 0x1f2c);
assert_offset!(CHeli, field_1f30, 0x1f30);
assert_offset!(CHeli, field_1f34, 0x1f34);
assert_offset!(CHeli, field_1f38, 0x1f38);
assert_offset!(CHeli, field_1f3c, 0x1f3c);
assert_offset!(CHeli, field_1f40, 0x1f40);
assert_offset!(CHeli, field_1f44, 0x1f44);
assert_offset!(CHeli, field_1f48, 0x1f48);
assert_offset!(CHeli, field_1f4c, 0x1f4c);
assert_offset!(CHeli, field_1f50, 0x1f50);
assert_offset!(CHeli, field_1f54, 0x1f54);
assert_offset!(CHeli, field_1f58, 0x1f58);
assert_offset!(CHeli, field_1f5c, 0x1f5c);
assert_offset!(CHeli, field_1f60, 0x1f60);
assert_offset!(CHeli, field_1f64, 0x1f64);
assert_offset!(CHeli, field_1f68, 0x1f68);
assert_offset!(CHeli, field_1f6c, 0x1f6c);
assert_offset!(CHeli, field_1f6d, 0x1f6d);
assert_offset!(CHeli, field_1f70, 0x1f70);
assert_offset!(CHeli, field_1f74, 0x1f74);
assert_offset!(CHeli, field_1f78, 0x1f78);
assert_offset!(CHeli, field_1f80, 0x1f80);
assert_offset!(CHeli, field_1f84, 0x1f84);
assert_offset!(CHeli, field_1f88, 0x1f88);
assert_offset!(CHeli, field_1f90, 0x1f90);
assert_offset!(CHeli, field_1f94, 0x1f94);
assert_offset!(CHeli, field_1f98, 0x1f98);
assert_offset!(CHeli, field_1fa0, 0x1fa0);
assert_offset!(CHeli, field_1fa4, 0x1fa4);
assert_offset!(CHeli, field_1fa8, 0x1fa8);
assert_offset!(CHeli, field_1fd0, 0x1fd0);
assert_offset!(CHeli, field_1fd4, 0x1fd4);
assert_offset!(CHeli, field_1fd8, 0x1fd8);
assert_offset!(CHeli, field_1fe0, 0x1fe0);
assert_offset!(CHeli, field_1fe4, 0x1fe4);
assert_offset!(CHeli, field_1fe8, 0x1fe8);
assert_offset!(CHeli, field_2000, 0x2000);

/// Merged layout for `CPlane`.
///
/// Size: 0x1f0d (low). Bases: CAutomobile@0x0.
/// Lanes: c-entities, n-07, n-18.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPlane {
    /// Unknown bytes (0x0..0x1eb0).
    pub _pad_0000: [u8; 0x1eb0],
    /// field_1eb0 (confidence: high, kind: float, lanes: c-entities).
    pub field_1eb0: f32,
    /// Unknown bytes (0x1eb4..0x1ec4).
    pub _pad_1eb4: [u8; 0x10],
    /// field_1ec4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ec4: u32,
    /// field_1ec8 (confidence: high, kind: u32, lanes: c-entities).
    pub field_1ec8: u32,
    /// Unknown bytes (0x1ecc..0x1ed4).
    pub _pad_1ecc: [u8; 0x8],
    /// field_1ed4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ed4: u32,
    /// field_1ed8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ed8: u32,
    /// field_1edc (confidence: low, kind: u32, lanes: c-entities).
    pub field_1edc: u32,
    /// Unknown bytes (0x1ee0..0x1ee4).
    pub _pad_1ee0: [u8; 0x4],
    /// field_1ee4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ee4: u32,
    /// field_1ee8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ee8: u32,
    /// field_1eec (confidence: high, kind: float, lanes: c-entities).
    pub field_1eec: f32,
    /// Unknown bytes (0x1ef0..0x1ef4).
    pub _pad_1ef0: [u8; 0x4],
    /// undercarriage_up_amount (confidence: high, kind: u32, lanes: c-entities,n-07,n-18).
    pub undercarriage_up_amount: u32,
    /// field_1ef8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1ef8: u32,
    /// Unknown bytes (0x1efc..0x1f0c).
    pub _pad_1efc: [u8; 0x10],
    /// field_1f0c (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1f0c: u8,
    /// Unknown trailing bytes (0x1f0d..0x1f10).
    pub _pad_end: [u8; 0x3],
}
assert_size!(CPlane, 0x1f10); // merged size 0x1f0d rounded to 4
assert_offset!(CPlane, field_1eb0, 0x1eb0);
assert_offset!(CPlane, field_1ec4, 0x1ec4);
assert_offset!(CPlane, field_1ec8, 0x1ec8);
assert_offset!(CPlane, field_1ed4, 0x1ed4);
assert_offset!(CPlane, field_1ed8, 0x1ed8);
assert_offset!(CPlane, field_1edc, 0x1edc);
assert_offset!(CPlane, field_1ee4, 0x1ee4);
assert_offset!(CPlane, field_1ee8, 0x1ee8);
assert_offset!(CPlane, field_1eec, 0x1eec);
assert_offset!(CPlane, undercarriage_up_amount, 0x1ef4);
assert_offset!(CPlane, field_1ef8, 0x1ef8);
assert_offset!(CPlane, field_1f0c, 0x1f0c);

/// Merged layout for `CTrain`.
///
/// Size: 0x14ea (low). Bases: CVehicle@0x0.
/// Lanes: c-entities, n-04.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CTrain {
    /// Unknown bytes (0x0..0xe6f).
    pub _pad_0000: [u8; 0xe6f],
    /// field_e6f (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_e6f: u8,
    /// Unknown bytes (0xe70..0x1448).
    pub _pad_0e70: [u8; 0x5d8],
    /// field_1448 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1448: u32,
    /// Unknown bytes (0x144c..0x1450).
    pub _pad_144c: [u8; 0x4],
    /// field_1450 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1450: u32,
    /// field_1454 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1454: u32,
    /// Unknown bytes (0x1458..0x14a4).
    pub _pad_1458: [u8; 0x4c],
    /// field_14a4 (confidence: high, kind: float, lanes: c-entities).
    pub field_14a4: f32,
    /// field_14a8 (confidence: high, kind: float, lanes: c-entities).
    pub field_14a8: f32,
    /// field_14ac (confidence: high, kind: float, lanes: c-entities).
    pub field_14ac: f32,
    /// field_14b0 (confidence: high, kind: float, lanes: c-entities).
    pub field_14b0: f32,
    /// Unknown bytes (0x14b4..0x14b8).
    pub _pad_14b4: [u8; 0x4],
    /// field_14b8 (confidence: high, kind: float, lanes: c-entities).
    pub field_14b8: f32,
    /// field_14bc (confidence: high, kind: u32, lanes: c-entities).
    pub field_14bc: u32,
    /// field_14c0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_14c0: u32,
    /// field_14c4 (confidence: high, kind: u32, lanes: c-entities).
    pub field_14c4: u32,
    /// Unknown bytes (0x14c8..0x14dc).
    pub _pad_14c8: [u8; 0x14],
    /// field_14dc (confidence: high, kind: u32, lanes: c-entities).
    pub field_14dc: u32,
    /// field_14e0 (confidence: high, kind: u32, lanes: c-entities).
    pub field_14e0: u32,
    /// train_direction (confidence: high, kind: u8, lanes: c-entities,n-04).
    pub train_direction: u8,
    /// field_14e5 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_14e5: u8,
    /// Unknown bytes (0x14e6..0x14e7).
    pub _pad_14e6: [u8; 0x1],
    /// field_14e7 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_14e7: u8,
    /// field_14e8 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_14e8: u8,
    /// field_14e9 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_14e9: u8,
    /// Unknown trailing bytes (0x14ea..0x14ec).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CTrain, 0x14ec); // merged size 0x14ea rounded to 4
assert_offset!(CTrain, field_e6f, 0xe6f);
assert_offset!(CTrain, field_1448, 0x1448);
assert_offset!(CTrain, field_1450, 0x1450);
assert_offset!(CTrain, field_1454, 0x1454);
assert_offset!(CTrain, field_14a4, 0x14a4);
assert_offset!(CTrain, field_14a8, 0x14a8);
assert_offset!(CTrain, field_14ac, 0x14ac);
assert_offset!(CTrain, field_14b0, 0x14b0);
assert_offset!(CTrain, field_14b8, 0x14b8);
assert_offset!(CTrain, field_14bc, 0x14bc);
assert_offset!(CTrain, field_14c0, 0x14c0);
assert_offset!(CTrain, field_14c4, 0x14c4);
assert_offset!(CTrain, field_14dc, 0x14dc);
assert_offset!(CTrain, field_14e0, 0x14e0);
assert_offset!(CTrain, train_direction, 0x14e4);
assert_offset!(CTrain, field_14e5, 0x14e5);
assert_offset!(CTrain, field_14e7, 0x14e7);
assert_offset!(CTrain, field_14e8, 0x14e8);
assert_offset!(CTrain, field_14e9, 0x14e9);

/// Merged layout for `CVehicle`.
///
/// Size: 0x1ef2 (low). Bases: CPhysical@0x0.
/// Lanes: c-entities, n-02, n-03, n-04, n-05, n-06, n-07, n-08, n-10, n-11, n-12, n-13, n-14, n-15, n-16, n-18, n-19, n-20, n-22, via:CAutomobile, via:CBike, via:CBoat, via:CTrain.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVehicle {
    /// Unknown bytes (0x0..0x820).
    pub _pad_0000: [u8; 0x820],
    /// vehicle (confidence: low, kind: pointer, lanes: n-02).
    pub vehicle: Ptr32<u8>,
    /// Unknown bytes (0x824..0x9e0).
    pub _pad_0824: [u8; 0x1bc],
    /// vehicle (confidence: low, kind: pointer, lanes: n-02).
    pub vehicle_2: Ptr32<u8>,
    /// Unknown bytes (0x9e4..0xd60).
    pub _pad_09e4: [u8; 0x37c],
    /// field_d60 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_d60: u32,
    /// Unknown bytes (0xd64..0xdb8).
    pub _pad_0d64: [u8; 0x54],
    /// field_db8 (confidence: high, kind: embedded-object, lanes: c-entities).
    pub field_db8: u32,
    /// Unknown bytes (0xdbc..0xdc0).
    pub _pad_0dbc: [u8; 0x4],
    /// field_dc0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_dc0: u32,
    /// field_dc4 (confidence: high, kind: u32, lanes: c-entities).
    pub field_dc4: u32,
    /// field_dc8 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_dc8: Ptr32<u8>,
    /// has_hydraulics (confidence: high, kind: flags, lanes: c-entities,n-04).
    pub has_hydraulics: u32,
    /// field_dd0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_dd0: u32,
    /// field_dd4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_dd4: u32,
    /// Unknown bytes (0xdd8..0xe38).
    pub _pad_0dd8: [u8; 0x60],
    /// field_e38 (confidence: medium, kind: pointer, lanes: c-entities).
    pub field_e38: Ptr32<u8>,
    /// Unknown bytes (0xe3c..0xea0).
    pub _pad_0e3c: [u8; 0x64],
    /// field_ea0 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_ea0: u8,
    /// Unknown bytes (0xea1..0xeca).
    pub _pad_0ea1: [u8; 0x29],
    /// field_eca (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_eca: u8,
    /// field_ecb (confidence: low, kind: int16?, lanes: c-entities).
    pub field_ecb: [u8; 2],
    /// Unknown bytes (0xecd..0xef8).
    pub _pad_0ecd: [u8; 0x2b],
    /// field_ef8 (confidence: high, kind: flags, lanes: c-entities,n-16).
    pub field_ef8: u8,
    /// Unknown bytes (0xef9..0xf04).
    pub _pad_0ef9: [u8; 0xb],
    /// distance_ahead_multiplier (confidence: high, kind: u32, lanes: n-15).
    pub distance_ahead_multiplier: u32,
    /// lane_shift (confidence: high, kind: u32, lanes: n-15).
    pub lane_shift: u32,
    /// blocking_entity (confidence: medium, kind: pointer, lanes: n-05).
    pub blocking_entity: Ptr32<u8>,
    /// field_f10 (confidence: high, kind: bool-or-byte, lanes: c-entities,via:CAutomobile,via:CBike moved from siblings:CAutomobile,CBike).
    pub field_f10: u8,
    /// Unknown bytes (0xf11..0xf14).
    pub _pad_0f11: [u8; 0x3],
    /// state (confidence: high, kind: bool-or-byte, lanes: c-entities,n-15).
    pub state: u8,
    /// field_f15 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_f15: u8,
    /// needs_hotwired (confidence: high, kind: flags, lanes: c-entities,n-16,n-18).
    pub needs_hotwired: u8,
    /// damage_wrecked_not_driveable (confidence: high, kind: flags, lanes: c-entities,n-10,n-11).
    pub damage_wrecked_not_driveable: u8,
    /// field_f18 (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_f18: u8,
    /// flags (confidence: high, kind: flags, lanes: c-entities,n-22).
    pub flags: u8,
    /// flags (confidence: high, kind: flags, lanes: c-entities,n-22).
    pub flags_2: u8,
    /// petrol_tank_weakpoint_enabled (confidence: high, kind: u8, lanes: c-entities,n-18).
    pub petrol_tank_weakpoint_enabled: u8,
    /// vehicle_hot (confidence: medium, kind: u8, lanes: c-entities,n-10).
    pub vehicle_hot: u8,
    /// field_f1d (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_f1d: u8,
    /// police_focus_will_track (confidence: high, kind: flags, lanes: c-entities,n-05,n-18).
    pub police_focus_will_track: u8,
    /// x40_emergency_services_vehicle (confidence: high, kind: u8, lanes: c-entities,n-10,n-15).
    pub x40_emergency_services_vehicle: u8,
    /// field_f20 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_f20: u8,
    /// field_f21 (confidence: high, kind: bool-or-byte, lanes: c-entities).
    pub field_f21: u8,
    /// Unknown bytes (0xf22..0xf24).
    pub _pad_0f22: [u8; 0x2],
    /// field_f24 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f24: u32,
    /// field_f28 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_f28: u32,
    /// field_f2c (confidence: low, kind: u32, lanes: c-entities).
    pub field_f2c: u32,
    /// field_f30 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f30: u32,
    /// field_f34 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f34: u32,
    /// field_f38 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f38: u32,
    /// Unknown bytes (0xf3c..0xf48).
    pub _pad_0f3c: [u8; 0xc],
    /// alarm_time_remaining_milliseconds (confidence: high, kind: u16, lanes: c-entities,n-19).
    pub alarm_time_remaining_milliseconds: u16,
    /// field_f4a (confidence: high, kind: u16, lanes: c-entities,n-16).
    pub field_f4a: u16,
    /// field_f4c (confidence: low, kind: int16?, lanes: c-entities).
    pub field_f4c: u16,
    /// Unknown bytes (0xf4e..0xf50).
    pub _pad_0f4e: [u8; 0x2],
    /// ptr_driver_cped (confidence: high, kind: u32, lanes: c-entities,n-06,n-08).
    pub ptr_driver_cped: u32,
    /// occupant_array_base (confidence: high, kind: u32, lanes: c-entities,n-05).
    pub occupant_array_base: u32,
    /// Unknown bytes (0xf58..0xf5c).
    pub _pad_0f58: [u8; 0x4],
    /// field_f5c (confidence: low, kind: u32, lanes: c-entities).
    pub field_f5c: u32,
    /// Unknown bytes (0xf60..0xf74).
    pub _pad_0f60: [u8; 0x14],
    /// field_f74 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f74: u32,
    /// field_f78 (confidence: low, kind: u32, lanes: c-entities).
    pub field_f78: u32,
    /// field_f7c (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_f7c: u8,
    /// Unknown bytes (0xf7d..0xf80).
    pub _pad_0f7d: [u8; 0x3],
    /// wheel_array (confidence: high, kind: pointer, lanes: c-entities,n-14).
    pub wheel_array: Ptr32<u8>,
    /// wheel_count (confidence: high, kind: i32, lanes: c-entities,n-11,n-14).
    pub wheel_count: u32,
    /// component_array_searched_component (confidence: high, kind: u32, lanes: c-entities,n-13).
    pub component_array_searched_component: u32,
    /// component_array_count (confidence: high, kind: i32, lanes: c-entities,n-13).
    pub component_array_count: u32,
    /// field_f90 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_f90: u32,
    /// primary_colour_id (confidence: high, kind: u8, lanes: c-entities,n-02,n-05,n-15).
    pub primary_colour_id: u8,
    /// secondary_colour_id (confidence: high, kind: u8, lanes: c-entities,n-02,n-05).
    pub secondary_colour_id: u8,
    /// extra_colour (confidence: high, kind: u8, lanes: c-entities,n-06).
    pub extra_colour: u8,
    /// extra_colour (confidence: high, kind: u8, lanes: c-entities,n-06).
    pub extra_colour_2: u8,
    /// field_f98 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_f98: u8,
    /// Unknown bytes (0xf99..0xf9c).
    pub _pad_0f99: [u8; 0x3],
    /// field_f9c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_f9c: u32,
    /// Unknown bytes (0xfa0..0xfa4).
    pub _pad_0fa0: [u8; 0x4],
    /// field_fa4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_fa4: u32,
    /// field_fa8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_fa8: u32,
    /// field_fac (confidence: low, kind: u32, lanes: c-entities).
    pub field_fac: u32,
    /// Unknown bytes (0xfb0..0x104c).
    pub _pad_0fb0: [u8; 0x9c],
    /// field_104c (confidence: medium, kind: u32, lanes: c-entities).
    pub field_104c: u32,
    /// field_1050 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1050: u32,
    /// field_1054 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1054: u32,
    /// field_1058 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1058: u32,
    /// Unknown bytes (0x105c..0x1060).
    pub _pad_105c: [u8; 0x4],
    /// field_1060 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1060: u32,
    /// field_1064 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1064: u32,
    /// field_1068 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1068: u32,
    /// Unknown bytes (0x106c..0x1070).
    pub _pad_106c: [u8; 0x4],
    /// max_passengers (confidence: high, kind: u8, lanes: c-entities,n-07).
    pub max_passengers: u8,
    /// field_1071 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_1071: [u8; 2],
    /// Unknown bytes (0x1073..0x1074).
    pub _pad_1073: [u8; 0x1],
    /// field_1074 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_1074: u16,
    /// Unknown bytes (0x1076..0x1078).
    pub _pad_1076: [u8; 0x2],
    /// field_1078 (confidence: high, kind: float, lanes: c-entities).
    pub field_1078: f32,
    /// field_107c (confidence: low, kind: u32, lanes: c-entities).
    pub field_107c: u32,
    /// field_1080 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1080: u32,
    /// field_1084 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1084: u32,
    /// field_1088 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1088: u32,
    /// field_108c (confidence: low, kind: u32, lanes: c-entities).
    pub field_108c: u32,
    /// gear (confidence: high, kind: u32, lanes: c-entities,n-08).
    pub gear: u32,
    /// revs_float (confidence: high, kind: float, lanes: n-08).
    pub revs_float: f32,
    /// Unknown bytes (0x1098..0x10b8).
    pub _pad_1098: [u8; 0x20],
    /// mission_entity_marker (confidence: high, kind: u8, lanes: c-entities,n-12,n-15).
    pub mission_entity_marker: u8,
    /// Unknown bytes (0x10b9..0x10bc).
    pub _pad_10b9: [u8; 0x3],
    /// field_10bc (confidence: low, kind: u32, lanes: c-entities).
    pub field_10bc: u32,
    /// field_10c0 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_10c0: u16,
    /// lights_state_flags_toggled (confidence: high, kind: flags, lanes: c-entities,n-05).
    pub lights_state_flags_toggled: u8,
    /// Unknown bytes (0x10c3..0x10c4).
    pub _pad_10c3: [u8; 0x1],
    /// field_10c4 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_10c4: u16,
    /// Unknown bytes (0x10c6..0x10c8).
    pub _pad_10c6: [u8; 0x2],
    /// dirt_level_float (confidence: high, kind: float, lanes: c-entities,n-08).
    pub dirt_level_float: f32,
    /// field_10cc (confidence: high, kind: float, lanes: c-entities).
    pub field_10cc: f32,
    /// siren_object_passed_siren (confidence: high, kind: u32, lanes: c-entities,n-05).
    pub siren_object_passed_siren: u32,
    /// Unknown bytes (0x10d4..0x10d8).
    pub _pad_10d4: [u8; 0x4],
    /// petrol_tank_health (confidence: high, kind: float, lanes: n-07,n-18).
    pub petrol_tank_health: f32,
    /// Unknown bytes (0x10dc..0x10e0).
    pub _pad_10dc: [u8; 0x4],
    /// field_10e0 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_10e0: u32,
    /// Unknown bytes (0x10e4..0x115e).
    pub _pad_10e4: [u8; 0x7a],
    /// field_115e (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_115e: u8,
    /// field_115f (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_115f: u8,
    /// Unknown bytes (0x1160..0x1164).
    pub _pad_1160: [u8; 0x4],
    /// damage_state_deformation_sampling (confidence: low, kind: pointer, lanes: n-05).
    pub damage_state_deformation_sampling: Ptr32<u8>,
    /// Unknown bytes (0x1168..0x119e).
    pub _pad_1168: [u8; 0x36],
    /// field_119e (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_119e: u8,
    /// field_119f (confidence: medium, kind: bool-or-byte, lanes: c-entities).
    pub field_119f: u8,
    /// Unknown bytes (0x11a0..0x1290).
    pub _pad_11a0: [u8; 0xf0],
    /// field_1290 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_1290: u32,
    /// field_1294 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1294: u32,
    /// field_1298 (confidence: low, kind: u32, lanes: c-entities).
    pub field_1298: u32,
    /// field_129c (confidence: low, kind: u32, lanes: c-entities).
    pub field_129c: u32,
    /// field_12a0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12a0: u32,
    /// field_12a4 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_12a4: u32,
    /// field_12a8 (confidence: medium, kind: u32, lanes: c-entities).
    pub field_12a8: u32,
    /// field_12ac (confidence: low, kind: u32, lanes: c-entities).
    pub field_12ac: u32,
    /// field_12b0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12b0: u32,
    /// field_12b4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12b4: u32,
    /// field_12b8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12b8: u32,
    /// field_12bc (confidence: low, kind: u32, lanes: c-entities).
    pub field_12bc: u32,
    /// field_12c0 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12c0: u32,
    /// field_12c4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12c4: u32,
    /// field_12c8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12c8: u32,
    /// field_12cc (confidence: low, kind: u32, lanes: c-entities).
    pub field_12cc: u32,
    /// door_lock_status (confidence: medium, kind: u32, lanes: c-entities,n-05).
    pub door_lock_status: u32,
    /// field_12d4 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12d4: u32,
    /// field_12d8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12d8: u32,
    /// field_12dc (confidence: low, kind: u32, lanes: c-entities).
    pub field_12dc: u32,
    /// light_multiplier (confidence: high, kind: u32, lanes: c-entities,n-15).
    pub light_multiplier: u32,
    /// Unknown bytes (0x12e4..0x12e5).
    pub _pad_12e4: [u8; 0x1],
    /// field_12e5 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_12e5: [u8; 2],
    /// selected_weapons_secondary_guns (confidence: high, kind: u8, lanes: c-entities,n-15).
    pub selected_weapons_secondary_guns: u8,
    /// field_12e8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12e8: u32,
    /// field_12ec (confidence: low, kind: u32, lanes: c-entities).
    pub field_12ec: u32,
    /// field_12f0 (confidence: low, kind: int16?, lanes: c-entities).
    pub field_12f0: u16,
    /// field_12f2 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_12f2: u8,
    /// Unknown bytes (0x12f3..0x12f4).
    pub _pad_12f3: [u8; 0x1],
    /// secondary_state (confidence: medium, kind: flags, lanes: n-15).
    pub secondary_state: u32,
    /// field_12f8 (confidence: low, kind: u32, lanes: c-entities).
    pub field_12f8: u32,
    /// field_12fc (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_12fc: u8,
    /// Unknown bytes (0x12fd..0x1300).
    pub _pad_12fd: [u8; 0x3],
    /// vehicle_class_id (confidence: high, kind: u32, lanes: c-entities,n-03,n-10,n-11,n-15,n-16,n-19).
    pub vehicle_class_id: u32,
    /// vehicle_class_id (confidence: high, kind: u32, lanes: c-entities,n-05,n-06,n-08,n-10,n-11,n-22).
    pub vehicle_class_id_2: u32,
    /// Unknown bytes (0x1308..0x1310).
    pub _pad_1308: [u8; 0x8],
    /// field_1310 (confidence: high, kind: flags, lanes: c-entities,n-16).
    pub field_1310: u8,
    /// Unknown bytes (0x1311..0x1320).
    pub _pad_1311: [u8; 0xf],
    /// field_1320 (confidence: medium, kind: u32, lanes: c-entities,via:CBike,via:CBoat moved from siblings:CBike,CBoat).
    pub field_1320: u32,
    /// Unknown bytes (0x1324..0x1428).
    pub _pad_1324: [u8; 0x104],
    /// field_1428 (confidence: high, kind: u32, lanes: c-entities,n-16).
    pub field_1428: u32,
    /// Unknown bytes (0x142c..0x1430).
    pub _pad_142c: [u8; 0x4],
    /// field_1430 (confidence: high, kind: u32, lanes: c-entities,via:CAutomobile,via:CBike moved from siblings:CAutomobile,CBike).
    pub field_1430: u32,
    /// field_1434 (confidence: high, kind: u32, lanes: c-entities,via:CAutomobile,via:CBike moved from siblings:CAutomobile,CBike).
    pub field_1434: u32,
    /// Unknown bytes (0x1438..0x1474).
    pub _pad_1438: [u8; 0x3c],
    /// respray_drown_bits (confidence: high, kind: flags, lanes: c-entities,n-15,n-16).
    pub respray_drown_bits: u32,
    /// Unknown bytes (0x1478..0x1484).
    pub _pad_1478: [u8; 0xc],
    /// field_1484 (confidence: medium, kind: u32, lanes: n-16).
    pub field_1484: u32,
    /// Unknown bytes (0x1488..0x14a0).
    pub _pad_1488: [u8; 0x18],
    /// field_14a0 (confidence: high, kind: float, lanes: c-entities,via:CAutomobile,via:CTrain moved from siblings:CAutomobile,CTrain).
    pub field_14a0: f32,
    /// Unknown bytes (0x14a4..0x14d0).
    pub _pad_14a4: [u8; 0x2c],
    /// link_next_unit_train (confidence: medium, kind: u32, lanes: c-entities,n-03).
    pub link_next_unit_train: u32,
    /// next_carriage_link (confidence: high, kind: u32, lanes: c-entities,n-03,n-08).
    pub next_carriage_link: u32,
    /// Unknown bytes (0x14d8..0x1ef0).
    pub _pad_14d8: [u8; 0xa18],
    /// field_1ef0 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1ef0: u8,
    /// field_1ef1 (confidence: low, kind: bool-or-byte, lanes: c-entities).
    pub field_1ef1: u8,
    /// Unknown trailing bytes (0x1ef2..0x1ef4).
    pub _pad_end: [u8; 0x2],
}
assert_size!(CVehicle, 0x1ef4); // merged size 0x1ef2 rounded to 4
assert_offset!(CVehicle, vehicle, 0x820);
assert_offset!(CVehicle, vehicle_2, 0x9e0);
assert_offset!(CVehicle, field_d60, 0xd60);
assert_offset!(CVehicle, field_db8, 0xdb8);
assert_offset!(CVehicle, field_dc0, 0xdc0);
assert_offset!(CVehicle, field_dc4, 0xdc4);
assert_offset!(CVehicle, field_dc8, 0xdc8);
assert_offset!(CVehicle, has_hydraulics, 0xdcc);
assert_offset!(CVehicle, field_dd0, 0xdd0);
assert_offset!(CVehicle, field_dd4, 0xdd4);
assert_offset!(CVehicle, field_e38, 0xe38);
assert_offset!(CVehicle, field_ea0, 0xea0);
assert_offset!(CVehicle, field_eca, 0xeca);
assert_offset!(CVehicle, field_ecb, 0xecb);
assert_offset!(CVehicle, field_ef8, 0xef8);
assert_offset!(CVehicle, distance_ahead_multiplier, 0xf04);
assert_offset!(CVehicle, lane_shift, 0xf08);
assert_offset!(CVehicle, blocking_entity, 0xf0c);
assert_offset!(CVehicle, field_f10, 0xf10);
assert_offset!(CVehicle, state, 0xf14);
assert_offset!(CVehicle, field_f15, 0xf15);
assert_offset!(CVehicle, needs_hotwired, 0xf16);
assert_offset!(CVehicle, damage_wrecked_not_driveable, 0xf17);
assert_offset!(CVehicle, field_f18, 0xf18);
assert_offset!(CVehicle, flags, 0xf19);
assert_offset!(CVehicle, flags_2, 0xf1a);
assert_offset!(CVehicle, petrol_tank_weakpoint_enabled, 0xf1b);
assert_offset!(CVehicle, vehicle_hot, 0xf1c);
assert_offset!(CVehicle, field_f1d, 0xf1d);
assert_offset!(CVehicle, police_focus_will_track, 0xf1e);
assert_offset!(CVehicle, x40_emergency_services_vehicle, 0xf1f);
assert_offset!(CVehicle, field_f20, 0xf20);
assert_offset!(CVehicle, field_f21, 0xf21);
assert_offset!(CVehicle, field_f24, 0xf24);
assert_offset!(CVehicle, field_f28, 0xf28);
assert_offset!(CVehicle, field_f2c, 0xf2c);
assert_offset!(CVehicle, field_f30, 0xf30);
assert_offset!(CVehicle, field_f34, 0xf34);
assert_offset!(CVehicle, field_f38, 0xf38);
assert_offset!(CVehicle, alarm_time_remaining_milliseconds, 0xf48);
assert_offset!(CVehicle, field_f4a, 0xf4a);
assert_offset!(CVehicle, field_f4c, 0xf4c);
assert_offset!(CVehicle, ptr_driver_cped, 0xf50);
assert_offset!(CVehicle, occupant_array_base, 0xf54);
assert_offset!(CVehicle, field_f5c, 0xf5c);
assert_offset!(CVehicle, field_f74, 0xf74);
assert_offset!(CVehicle, field_f78, 0xf78);
assert_offset!(CVehicle, field_f7c, 0xf7c);
assert_offset!(CVehicle, wheel_array, 0xf80);
assert_offset!(CVehicle, wheel_count, 0xf84);
assert_offset!(CVehicle, component_array_searched_component, 0xf88);
assert_offset!(CVehicle, component_array_count, 0xf8c);
assert_offset!(CVehicle, field_f90, 0xf90);
assert_offset!(CVehicle, primary_colour_id, 0xf94);
assert_offset!(CVehicle, secondary_colour_id, 0xf95);
assert_offset!(CVehicle, extra_colour, 0xf96);
assert_offset!(CVehicle, extra_colour_2, 0xf97);
assert_offset!(CVehicle, field_f98, 0xf98);
assert_offset!(CVehicle, field_f9c, 0xf9c);
assert_offset!(CVehicle, field_fa4, 0xfa4);
assert_offset!(CVehicle, field_fa8, 0xfa8);
assert_offset!(CVehicle, field_fac, 0xfac);
assert_offset!(CVehicle, field_104c, 0x104c);
assert_offset!(CVehicle, field_1050, 0x1050);
assert_offset!(CVehicle, field_1054, 0x1054);
assert_offset!(CVehicle, field_1058, 0x1058);
assert_offset!(CVehicle, field_1060, 0x1060);
assert_offset!(CVehicle, field_1064, 0x1064);
assert_offset!(CVehicle, field_1068, 0x1068);
assert_offset!(CVehicle, max_passengers, 0x1070);
assert_offset!(CVehicle, field_1071, 0x1071);
assert_offset!(CVehicle, field_1074, 0x1074);
assert_offset!(CVehicle, field_1078, 0x1078);
assert_offset!(CVehicle, field_107c, 0x107c);
assert_offset!(CVehicle, field_1080, 0x1080);
assert_offset!(CVehicle, field_1084, 0x1084);
assert_offset!(CVehicle, field_1088, 0x1088);
assert_offset!(CVehicle, field_108c, 0x108c);
assert_offset!(CVehicle, gear, 0x1090);
assert_offset!(CVehicle, revs_float, 0x1094);
assert_offset!(CVehicle, mission_entity_marker, 0x10b8);
assert_offset!(CVehicle, field_10bc, 0x10bc);
assert_offset!(CVehicle, field_10c0, 0x10c0);
assert_offset!(CVehicle, lights_state_flags_toggled, 0x10c2);
assert_offset!(CVehicle, field_10c4, 0x10c4);
assert_offset!(CVehicle, dirt_level_float, 0x10c8);
assert_offset!(CVehicle, field_10cc, 0x10cc);
assert_offset!(CVehicle, siren_object_passed_siren, 0x10d0);
assert_offset!(CVehicle, petrol_tank_health, 0x10d8);
assert_offset!(CVehicle, field_10e0, 0x10e0);
assert_offset!(CVehicle, field_115e, 0x115e);
assert_offset!(CVehicle, field_115f, 0x115f);
assert_offset!(CVehicle, damage_state_deformation_sampling, 0x1164);
assert_offset!(CVehicle, field_119e, 0x119e);
assert_offset!(CVehicle, field_119f, 0x119f);
assert_offset!(CVehicle, field_1290, 0x1290);
assert_offset!(CVehicle, field_1294, 0x1294);
assert_offset!(CVehicle, field_1298, 0x1298);
assert_offset!(CVehicle, field_129c, 0x129c);
assert_offset!(CVehicle, field_12a0, 0x12a0);
assert_offset!(CVehicle, field_12a4, 0x12a4);
assert_offset!(CVehicle, field_12a8, 0x12a8);
assert_offset!(CVehicle, field_12ac, 0x12ac);
assert_offset!(CVehicle, field_12b0, 0x12b0);
assert_offset!(CVehicle, field_12b4, 0x12b4);
assert_offset!(CVehicle, field_12b8, 0x12b8);
assert_offset!(CVehicle, field_12bc, 0x12bc);
assert_offset!(CVehicle, field_12c0, 0x12c0);
assert_offset!(CVehicle, field_12c4, 0x12c4);
assert_offset!(CVehicle, field_12c8, 0x12c8);
assert_offset!(CVehicle, field_12cc, 0x12cc);
assert_offset!(CVehicle, door_lock_status, 0x12d0);
assert_offset!(CVehicle, field_12d4, 0x12d4);
assert_offset!(CVehicle, field_12d8, 0x12d8);
assert_offset!(CVehicle, field_12dc, 0x12dc);
assert_offset!(CVehicle, light_multiplier, 0x12e0);
assert_offset!(CVehicle, field_12e5, 0x12e5);
assert_offset!(CVehicle, selected_weapons_secondary_guns, 0x12e7);
assert_offset!(CVehicle, field_12e8, 0x12e8);
assert_offset!(CVehicle, field_12ec, 0x12ec);
assert_offset!(CVehicle, field_12f0, 0x12f0);
assert_offset!(CVehicle, field_12f2, 0x12f2);
assert_offset!(CVehicle, secondary_state, 0x12f4);
assert_offset!(CVehicle, field_12f8, 0x12f8);
assert_offset!(CVehicle, field_12fc, 0x12fc);
assert_offset!(CVehicle, vehicle_class_id, 0x1300);
assert_offset!(CVehicle, vehicle_class_id_2, 0x1304);
assert_offset!(CVehicle, field_1310, 0x1310);
assert_offset!(CVehicle, field_1320, 0x1320);
assert_offset!(CVehicle, field_1428, 0x1428);
assert_offset!(CVehicle, field_1430, 0x1430);
assert_offset!(CVehicle, field_1434, 0x1434);
assert_offset!(CVehicle, respray_drown_bits, 0x1474);
assert_offset!(CVehicle, field_1484, 0x1484);
assert_offset!(CVehicle, field_14a0, 0x14a0);
assert_offset!(CVehicle, link_next_unit_train, 0x14d0);
assert_offset!(CVehicle, next_carriage_link, 0x14d4);
assert_offset!(CVehicle, field_1ef0, 0x1ef0);
assert_offset!(CVehicle, field_1ef1, 0x1ef1);
