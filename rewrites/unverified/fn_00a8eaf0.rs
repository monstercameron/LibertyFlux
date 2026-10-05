// original: 0x00A8EAF0 pool_row_cell (proposed)

/// Fetch one cell from a two-level pool row table.
///
/// `row_table` at `this+0xE4` holds pointers to row blocks spaced 160 bytes
/// apart (`a*160`); cell `b` is the 32-bit word at row `+b*4`. Pure loads,
/// no calls; out-of-range indexes fault like the original.
///
/// Original: thiscall, two stack words (row index, cell index), returns u32.
lf_checker_rt::export!(thiscall, rw_00A8EAF0(this: u32, row: u32, cell: u32) -> u32 {
    unsafe {
        const ROW_TABLE_OFF: u32 = 0xe4;
        const ROW_STRIDE: u32 = 160;
        let table = ((this + ROW_TABLE_OFF) as *const u32).read_unaligned();
        let block = (table.wrapping_add(row.wrapping_mul(ROW_STRIDE)) as *const u32)
            .read_unaligned();
        (block.wrapping_add(cell.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
