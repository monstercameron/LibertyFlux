// original: 0x00ab5d00 stream_table_reset (proposed)

/// Reset an 11-entry streaming table in place and return it.
///
/// For each lane `i` in `0..11` clears the word at `+4*i` and writes all-ones
/// to the word at `+0x2c+4*i`; then clears the word at `+0x58` and the byte
/// at `+0x5c`. Returns the table pointer.
///
/// Original: 0x00ab5d00 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5d00(tab: u32) -> u32 {
    unsafe {
        const LANES: u32 = 11;
        const SECOND_ROW: u32 = 0x2C;
        for i in 0..LANES {
            ((tab + 4 * i) as *mut u32).write_unaligned(0);
            ((tab + SECOND_ROW + 4 * i) as *mut u32).write_unaligned(0xFFFF_FFFF);
        }
        ((tab + 0x58) as *mut u32).write_unaligned(0);
        ((tab + 0x5C) as *mut u8).write(0);
        tab
    }
});
