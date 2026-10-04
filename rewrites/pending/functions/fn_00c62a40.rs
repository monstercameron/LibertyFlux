// original: 0x00c62a40 anim_player_reinit
/// Player re-init: clears the playback fields, binds a new animation, sets flags.
///
/// Tears down existing state when the tag word is set, binds the given
/// animation, zeroes the counters, restores unit scales, then sets the active
/// flag and the option flag from the mode byte. Always returns 1.
export!(thiscall, rw_00c62a40(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        if *((this + 0x44) as *const u16) != 0 {
            let _: u32 = callee_thiscall!(0, u32, this);
        }
        *((this + 0x44) as *mut u32) = 0;
        *((this + 0x4C) as *mut u32) = 0;
        *((this + 0x50) as *mut u32) = 0;
        *((this + 0x54) as *mut u32) = 0x3F80_0000;
        *((this + 0x58) as *mut u32) = 0x3F80_0000;
        *((this + 0x5C) as *mut u32) = 0;
        *((this + 0x60) as *mut u32) = 0;
        let _: u32 = callee_thiscall!(1, u32, this, a0);
        let w = (this + 0x46) as *mut u16;
        *w |= 1;
        let mut bits = (*w) as u32;
        if a1 & 0xFF != 0 {
            bits |= 8;
        } else {
            bits &= 0xFFF7;
        }
        *w = bits as u16;
        1
    }
});
