// original: 0x00e60e80 fill_sentinel_table
/// fill_sentinel_table: fill a 128-entry table with a sentinel pair.
///
/// Writes 0xFFFF_FFFF then a zero word into each 8-byte entry, clears
/// one flag byte just below the table, and returns the table's end
/// address (the loop pointer the original leaves in EAX).
lf_checker_rt::export!(cdecl, rw_00e60e80() -> u32 {
    unsafe {
        const ENTRIES: u32 = 128;
        const STRIDE: usize = 8;
        const FLAG: u32 = 0x01BB4035;
        let base = lf_checker_rt::global::<u8>(0x01BB403C);
        let mut i = 0u32;
        while i < ENTRIES {
            let elem = base.wrapping_add((i as usize) * STRIDE);
            (elem as *mut u32).write(0xFFFF_FFFF);
            (elem.wrapping_add(4) as *mut u16).write(0);
            i += 1;
        }
        lf_checker_rt::global::<u8>(FLAG).write(0);
        base.wrapping_add((ENTRIES as usize) * STRIDE) as u32
    }
});
