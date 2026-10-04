// original: 0x00c625e0 canimplayer_init_fields
/// Field initializer: writes the player's default field values.
///
/// Stores zero, all-ones words, unit and negated-unit floats, a code address
/// and a negative float across the object, and returns 0.
export!(thiscall, rw_00c625e0(this: u32) -> u32 {
    unsafe {
        let w = |off: u32| (this + off) as *mut u32;
        *w(0x04) = 0;
        *w(0x08) = 0;
        *w(0x0C) = 0xFFFF_FFFF;
        *w(0x10) = 0xFFFF_FFFF;
        *w(0x14) = 0xFFFF_FFFF;
        *w(0x18) = 0;
        *w(0x1C) = 0;
        *w(0x20) = relocated(0x4016A0);
        *w(0x24) = 0;
        *w(0x28) = 0xFFFF_FFFF;
        *w(0x2C) = 0xBF80_0000;
        *w(0x30) = 0xBF80_0000;
        *w(0x74) = 0;
        *w(0x64) = 0;
        *w(0x68) = 0x3F80_0000;
        *w(0x6C) = 0;
        *w(0x70) = 0x3F80_0000;
        *w(0x34) = 0;
        *w(0x38) = 0;
        *w(0x80) = 0;
        *w(0x48) = 0;
        *w(0x4C) = 0;
        *w(0x44) = 0;
        *w(0x50) = 0;
        *w(0x54) = 0x3F80_0000;
        *w(0x58) = 0x3F80_0000;
        *w(0x5C) = 0;
        *w(0x60) = 0;
        *w(0x3C) = 0xC080_0000;
        0
    }
});
