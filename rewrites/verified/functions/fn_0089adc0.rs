// original: 0x0089adc0 aud_envelope_vf9_release
/// Virtual slot 9 of the envelope sound: releases an indexed voice entry.
///
/// Looks up the voice entry from the two key bytes of the object and the
/// global voice table, then marks the entry's state word released. The two
/// stack arguments are accepted and ignored. Returns the looked-up entry
/// address. Address arithmetic is 32-bit by nature of the game's tables.
export!(thiscall, rw_0089adc0(this: *const u8, _a: u32, _b: u32) -> u32 {
    unsafe {
        let key = *this.add(0x40) as u32;
        let slot = *this.add(0xF7) as u32;
        let stride = *global::<u32>(0x0115D968);
        let table = *global::<u32>(0x0115D988);
        let row = table
            .wrapping_add(key.wrapping_mul(VOICE_ROW_STRIDE))
            .wrapping_add(VOICE_ROW_BIAS);
        let entry = *(row as *const u32);
        let state = (entry as usize)
            .wrapping_add((slot as usize).wrapping_mul(stride as usize))
            .wrapping_add(0x98) as *mut u32;
        *state = 0xFFFF_FFFF;
        entry
    }
});

