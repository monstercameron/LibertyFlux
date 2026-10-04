// original: 0x008aa3a0 audio_voice_store_e0
/// Store the argument into the voice record at record+0xe0.
///
/// Same record lookup as `rw_008a9eb0` (two audio globals indexed by the
/// bytes at `this+4` and `this+0x40`); writes the full argument dword. The
/// 0xff selector path faults at a near-null address exactly like the
/// original. Returns the stored argument.
export!(thiscall, rw_008aa3a0(this: *const u8, value: u32) -> u32 {
    unsafe {
        const SCALE_GLOB: u32 = 0x0115D968;
        const TABLE_GLOB: u32 = 0x0115D988;
        const STRIDE: u32 = 0x6F40;
        const TABLE_BIAS: u32 = 0x6F14;
        const SLOT_OFF: u32 = 0xE0;
        let sel = *this.add(4);
        if sel == 0xFF {
            *((0xE0u32) as *mut u32) = value;
            return value;
        }
        let sub = *this.add(0x40);
        let scale = *global::<u32>(SCALE_GLOB);
        let table = *global::<u32>(TABLE_GLOB);
        let entry_at = table
            .wrapping_add((sub as u32).wrapping_mul(STRIDE))
            .wrapping_add(TABLE_BIAS);
        let target = (*(entry_at as *const u32)).wrapping_add(scale.wrapping_mul(sel as u32));
        *((target.wrapping_add(SLOT_OFF)) as *mut u32) = value;
        value
    }
});
