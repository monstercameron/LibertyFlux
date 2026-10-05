// original: 0x0061BD20 net_store_grid_byte

/// Store one byte into a grid row.
///
/// `row` selects one of the 0x5580-byte rows of the grid at `[this]`,
/// and the low byte of `value` is stored at row offset 0x5511. The
/// return channel passes the entry accumulator through with its low
/// byte replaced (the contract pins the entry to zero).
/// Original: 0x0061BD20 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_0061BD20(this: u32, row: u32, value: u32) -> u32 {
    unsafe {
        const ROW_BYTES: u32 = 0x5580;
        const CELL_OFF: u32 = 0x5511;
        let grid = (this as *const u32).read_unaligned();
        let at = row
            .wrapping_mul(ROW_BYTES)
            .wrapping_add(grid)
            .wrapping_add(CELL_OFF);
        (at as *mut u8).write(value as u8);
        value & 0xFF
    }
});
