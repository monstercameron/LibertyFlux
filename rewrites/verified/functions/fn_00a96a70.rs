// original: 0x00a96a70 fade_reset
/// Resets a channel to its default levels, bounds and flags.
///
/// Clears the low flag nibble, retires the cursor, state and span to -1,
/// and restores the default bound pair, opaque word, link and unity ratio.
export!(thiscall, rw_00a96a70(this: u32) -> u32 {
    unsafe {
        *((this + 0x20) as *mut u8) &= 0xF0;
        *((this + 4) as *mut u32) = 0xFFFF_FFFF;
        *(this as *mut u32) = 0xFFFF_FFFF;
        *((this + 8) as *mut u32) = 0xFFFF_FFFF;
        *((this + 0x0C) as *mut u32) = 0x3EAA_AA9F;
        *((this + 0x10) as *mut u32) = 0x3F2A_AA9F;
        *((this + 0x18) as *mut u32) = 0xFF00_0000;
        *((this + 0x1C) as *mut u32) = 0;
        *((this + 0x14) as *mut u32) = 0x3F80_0000;
        0
    }
});
