// original: 0x0099e1e0 audio_scaled_table_lookup
/// Scaled table lookup keyed by two object bytes.
///
/// Returns 0 when the byte at +0x48 is 0xFF; otherwise returns the global
/// scale times that byte plus the table dword selected by the byte at +0x40
/// (stride 0x6F40 past a 0x6F10 header from a global base). All arithmetic
/// wraps.
export!(thiscall, rw_0099e1e0(obj: u32) -> u32 {
    unsafe {
        let hi = *((obj.wrapping_add(0x48)) as *const u8);
        if hi == 0xFF {
            return 0;
        }
        let lo = *((obj.wrapping_add(0x40)) as *const u8) as u32;
        let scale = *global::<u32>(0x115D964);
        let tbase = *global::<u32>(0x115D988);
        let cell = *(tbase
            .wrapping_add(lo.wrapping_mul(0x6F40))
            .wrapping_add(0x6F10) as *const u32);
        scale.wrapping_mul(hi as u32).wrapping_add(cell)
    }
});
