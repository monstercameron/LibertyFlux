// original: 0x00b285a0 slots_rows_reset

/// Clears every file slot and every row back to the empty state.
///
/// For each of the 24 slots calls the slot-clear callee with (index, 0),
/// then zeroes the slot's base word and both flag bytes. Then walks the
/// 2072 sixteen-byte rows, zeroing each row's data word and flag byte.
/// (One dead branch in the original jumps to a call that no iteration can
/// reach: the index is below 24 at every pass, so it is not reproduced.)
/// Returns the rows end address: the original's walk pointer ends exactly
/// there. Cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_00b285a0() -> u32 {
    unsafe {
        const SLOT_BASES: u32 = 0x016576B0;
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_FLAGS2: u32 = 0x016578C0;
        const ROWS: u32 = 0x01657A14;
        const ROWS_END: u32 = 0x0165FB9C;
        const ROW_STRIDE: u32 = 16;
        const SLOT_COUNT: u32 = 24;
        const CLEAR: u32 = 0;
        let bases = lf_checker_rt::relocated(SLOT_BASES);
        let flags = lf_checker_rt::relocated(SLOT_FLAGS);
        let flags2 = lf_checker_rt::relocated(SLOT_FLAGS2);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            lf_checker_rt::callee_cdecl!(CLEAR, u32, i, 0);
            ((bases + i * 4) as *mut u32).write_unaligned(0);
            ((flags + i) as *mut u8).write(0);
            ((flags2 + i) as *mut u8).write(0);
            i += 1;
        }
        let mut p = lf_checker_rt::relocated(ROWS);
        let end = lf_checker_rt::relocated(ROWS_END);
        while p < end {
            (p as *mut u32).write_unaligned(0);
            ((p + 8) as *mut u8).write(0);
            p += ROW_STRIDE;
        }
        end
    }
});
