// original: 0x00939440 stream_table_row_copy (proposed)

/// Copy one two-word row of the streaming table to the caller.
///
/// Reads the table base from its global; row `index` holds two dwords at
/// `base + index * 8`. Copies both to `out` and returns `out`.
lf_checker_rt::export!(cdecl, rw_00939440(out: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_GLOBAL: u32 = 0x11A4EF0;
        const ROW_BYTES: u32 = 8;
        let row = lf_checker_rt::global::<u32>(TABLE_GLOBAL)
            .read_unaligned()
            .wrapping_add(index.wrapping_mul(ROW_BYTES));
        (out as *mut u32).write_unaligned((row as *const u32).read_unaligned());
        ((out + 4) as *mut u32)
            .write_unaligned(((row + 4) as *const u32).read_unaligned());
        out
    }
});
