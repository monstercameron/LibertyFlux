// original: 0x00b1ab70 row_active_check (proposed)

/// Decides whether a table row counts as active from two tables.
///
/// Stdcall of one stack word: an index. Reads a flag byte from the row
/// table (stride 0xE2) and a state word pair from a dense table (stride 4).
/// When flag bit 0 is clear, the row is active if either state word is
/// nonzero; when set, bit 1 selects which check applies to the second
/// state word: nonzero low byte (bit set) or nonzero high byte (bit clear).
/// Returns 1 or 0 in AL.
lf_checker_rt::export!(stdcall, rw_00b1ab70(index: u32) -> u32 {
    unsafe {
        const FLAG_TABLE: u32 = 0x010401f1;
        const ROW_STRIDE: u32 = 0xe2;
        const STATE_TABLE: u32 = 0x016334c8;
        let flag = (lf_checker_rt::relocated(FLAG_TABLE.wrapping_add(index.wrapping_mul(ROW_STRIDE)))
            as *const u8)
            .read();
        let state = lf_checker_rt::relocated(STATE_TABLE.wrapping_add(index.wrapping_mul(4)));
        let active = if (flag & 1) == 0 {
            (state as *const u8).read() != 0
                || ((state + 2) as *const u16).read_unaligned() != 0
        } else if (flag & 2) != 0 {
            ((state + 2) as *const u8).read() != 0
        } else {
            ((state + 2) as *const u16).read_unaligned() & 0xff00 != 0
        };
        u32::from(active)
    }
});
