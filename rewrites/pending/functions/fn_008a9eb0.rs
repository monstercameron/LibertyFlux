// original: 0x008a9eb0 audio_voice_set_flag8
/// Set bit 3 of the voice record's flag byte at record+0xe8.
///
/// Resolves the voice record through the two audio globals (a scale dword
/// and a table base) indexed by the bytes at `this+4` and `this+0x40`, then
/// ORs 8 into the flag byte. A selector byte of 0xff takes the original's
/// faulting path (an access violation at a near-null address), reproduced
/// here so the fault behaviour matches too. Returns the table base.
export!(thiscall, rw_008a9eb0(this: *const u8) -> u32 {
    unsafe {
        const SCALE_GLOB: u32 = 0x0115D968;
        const TABLE_GLOB: u32 = 0x0115D988;
        const STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
        const FLAG_BIT: u8 = 8;
        let sel = *this.add(4);
        if sel == 0xFF {
            *((0xE8u32) as *mut u8) |= FLAG_BIT;
            return 0;
        }
        let sub = *this.add(0x40);
        let scale = *global::<u32>(SCALE_GLOB);
        let table = *global::<u32>(TABLE_GLOB);
        let entry_at = table
            .wrapping_add((sub as u32).wrapping_mul(STRIDE))
            .wrapping_add(TABLE_BIAS);
        let target = (*(entry_at as *const u32)).wrapping_add(scale.wrapping_mul(sel as u32));
        *((target.wrapping_add(FLAG_OFF)) as *mut u8) |= FLAG_BIT;
        table
    }
});
