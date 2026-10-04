// original: 0x0097b190 audio_table_lookup_entry
/// Look up one entry of the strided global audio table.
///
/// Combines the voice index at +0x4 with the scaled table row for the
/// bank index at +0x40 and stores the entry pointer's payload to `out`.
/// Index 0xff dereferences a null base and faults, like the original.
export!(thiscall, rw_0097b190(this: *const u8, out: *mut u32) -> u32 {
    unsafe {
        let idx = *this.add(4);
        if idx == 0xff {
            *out = core::ptr::read_volatile(0xcc as *const u32);
            return 0;
        }
        let j = (*this.add(0x40)) as u32;
        let g2 = *global::<u32>(0x115D968);
        let base = *global::<u32>(0x115D988);
        let w = *((base
            .wrapping_add(j.wrapping_mul(0x6f40))
            .wrapping_add(0x6f14)) as *const u32);
        let p = (idx as u32).wrapping_mul(g2).wrapping_add(w);
        *out = *((p.wrapping_add(0xcc)) as *const u32);
        0
    }
});
