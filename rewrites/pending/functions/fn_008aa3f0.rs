// original: 0x008aa3f0 audio_voice_toggle_bit1
/// Set bit 1 of the voice flag byte from the argument's low bit.
///
/// Same record lookup as its siblings; then bit 1 of the byte at
/// record+0xe8 is set to bit 0 of the argument (low byte only). The 0xff
/// selector path faults at a near-null address exactly like the original.
/// Returns the table base with its low byte replaced by the adjust mask
/// (matching exit EAX).
export!(thiscall, rw_008aa3f0(this: *const u8, arg: u32) -> u32 {
    unsafe {
        const SCALE_GLOB: u32 = 0x0115D968;
        const TABLE_GLOB: u32 = 0x0115D988;
        const STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
        let sel = *this.add(4);
        let (target, table_hi) = if sel == 0xFF {
            (0u32, 0u32)
        } else {
            let sub = *this.add(0x40);
            let scale = *global::<u32>(SCALE_GLOB);
            let table = *global::<u32>(TABLE_GLOB);
            let entry_at = table
                .wrapping_add((sub as u32).wrapping_mul(STRIDE))
                .wrapping_add(TABLE_BIAS);
            let target =
                (*(entry_at as *const u32)).wrapping_add(scale.wrapping_mul(sel as u32));
            (target, table & 0xFFFF_FF00)
        };
        let slot = target.wrapping_add(FLAG_OFF) as *mut u8;
        let doubled = (arg as u8).wrapping_mul(2);
        let adjust = (doubled ^ *slot) & 2;
        *slot ^= adjust;
        table_hi | (adjust as u32)
    }
});
