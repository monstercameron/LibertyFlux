// original: 0x005BB980 set_global_flag_bit_5
/// Set global flag bit 5 when the low input byte is nonzero. For a zero low
/// byte, clear bit 5 only when it is currently set. Other bits are preserved.
///
/// The original is cdecl with one 32-bit argument and no meaningful return.
lf_checker_rt::export!(cdecl, rw_005bb980(enabled: u32) -> u32 {
    const FLAGS_VA: u32 = 0x018b_6ea4;
    const FLAG: u32 = 0x20;
    unsafe {
        let flags = lf_checker_rt::global::<u32>(FLAGS_VA);
        let current = flags.read();
        let next = if enabled as u8 != 0 {
            current | FLAG
        } else if current & FLAG != 0 {
            current & !FLAG
        } else {
            current
        };
        if next != current {
            flags.write(next);
        }
    }
    0
});
