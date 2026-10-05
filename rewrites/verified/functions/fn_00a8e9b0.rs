// original: 0x00A8E9B0 pool_cell_iterator (proposed)

/// Step the pool's (row, cell) cursor and fetch the current cell.
///
/// `this+0xE8` is the 16-bit row count, `+0xFC` the current row and `+0x100`
/// the cell within it; the row table at `+0xE4` holds 160-byte rows whose
/// first word points at the cell array and whose `+4` word is the 16-bit
/// cell count. The cell cursor advances first; when it reaches the row's
/// count (signed compare) the row advances with the cell reset, skipping
/// empty rows (unsigned compare against zero). On success the cell value is
/// written to `*out` and 1 returned; when the rows run out 0 is returned
/// and `*out` is untouched. Only the low byte of the return is set on the
/// success path. No calls.
///
/// Original: thiscall, one stack word (output pointer), low byte in AL.
lf_checker_rt::export!(thiscall, rw_00A8E9B0(this: u32, out: u32) -> u32 {
    unsafe {
        const ROW_TABLE_OFF: u32 = 0xe4;
        const ROW_COUNT_OFF: u32 = 0xe8;
        const CUR_ROW_OFF: u32 = 0xfc;
        const CUR_CELL_OFF: u32 = 0x100;
        const ROW_STRIDE: u32 = 160;
        const CELLS_OFF: u32 = 0;
        const COUNT_OFF: u32 = 4;
        let rows = ((this + ROW_COUNT_OFF) as *const u16).read_unaligned() as i32;
        let table = ((this + ROW_TABLE_OFF) as *const u32).read_unaligned();
        let mut row = ((this + CUR_ROW_OFF) as *const i32).read_unaligned();
        if row >= rows {
            return 0;
        }
        loop {
            let cell = ((this + CUR_CELL_OFF) as *const u32)
                .read_unaligned()
                .wrapping_add(1);
            ((this + CUR_CELL_OFF) as *mut u32).write_unaligned(cell);
            let limit = (table
                .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
                .wrapping_add(COUNT_OFF) as *const u16)
                .read_unaligned() as u32;
            if (cell as i32) < limit as i32 {
                break;
            }
            row = ((this + CUR_ROW_OFF) as *const i32)
                .read_unaligned()
                .wrapping_add(1);
            ((this + CUR_ROW_OFF) as *mut i32).write_unaligned(row);
            ((this + CUR_CELL_OFF) as *mut u32).write_unaligned(0);
            if row >= rows {
                return 0;
            }
            let first = (table
                .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
                .wrapping_add(COUNT_OFF) as *const u16)
                .read_unaligned();
            if 0u16 >= first {
                continue;
            }
            break;
        }
        let row = ((this + CUR_ROW_OFF) as *const u32).read_unaligned();
        let cell = ((this + CUR_CELL_OFF) as *const u32).read_unaligned();
        let cells = (table.wrapping_add(row.wrapping_mul(ROW_STRIDE)).wrapping_add(CELLS_OFF)
            as *const u32)
            .read_unaligned();
        let value =
            (cells.wrapping_add(cell.wrapping_mul(4)) as *const u32).read_unaligned();
        (out as *mut u32).write_unaligned(value);
        1
    }
});
