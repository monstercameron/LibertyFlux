// original: 0x0089b590 aud_entry_set_params
/// Stores five parameter words into an indexed voice entry.
///
/// Resolves the voice entry like slot 9 above, then writes the stack
/// arguments into the entry's parameter block: two 16-bit values (low
/// halves kept), one byte, two full words, and a ready flag. Returns the
/// last argument.
export!(thiscall, rw_0089b590(
    this: *const u8,
    w1: u32,
    w2: u32,
    b3: u32,
    d4: u32,
    d5: u32,
) -> u32 {
    unsafe {
        let key = *this.add(0x40) as u32;
        let slot = *this.add(0xF7) as u32;
        let stride = *global::<u32>(0x0115D968);
        let table = *global::<u32>(0x0115D988);
        let row = table
            .wrapping_add(key.wrapping_mul(VOICE_ROW_STRIDE))
            .wrapping_add(VOICE_ROW_BIAS);
        let entry = *(row as *const u32);
        let target = (entry as usize)
            .wrapping_add((slot as usize).wrapping_mul(stride as usize))
            as *mut u8;
        core::ptr::write_unaligned(target.add(0x9C) as *mut u16, w1 as u16);
        core::ptr::write_unaligned(target.add(0x9E) as *mut u16, w2 as u16);
        *target.add(0xA0) = b3 as u8;
        *(target.add(0x90) as *mut u32) = d4;
        *(target.add(0x94) as *mut u32) = d5;
        *target.add(0xA1) = 1;
        d5
    }
});

