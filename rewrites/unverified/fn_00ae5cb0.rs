// original: 0X00AE5CB0 streaming_grid_rectangle_or

/// OR every dword in an inclusive rectangle of the 48-column grid.
///
/// This is a thiscall method with four signed coordinate arguments. The object
/// stores x and y origins at `+0x00` and `+0x04`, and dword cells begin at
/// `+0x08` in row-major order with a 48-cell row stride. X coordinates are
/// translated by the origin and clamped to 0 through 47; y coordinates
/// are translated by the origin and clamped to the signed limits at
/// `+0x2590` and `+0x2594`. A reversed vertical interval returns
/// zero. A reversed horizontal interval still reads its first column, matching
/// the original's inclusive do-while scan. The result is the bitwise OR of the
/// selected grid words. The checker exercises the full `0x2600`-byte object.
export!(thiscall, rw_00ae5cb0(this: u32, left: u32, top: u32, right: u32, bottom: u32) -> u32 {
    unsafe {
        const GRID_ORIGIN_X: u32 = 0x00;
        const GRID_ORIGIN_Y: u32 = 0x04;
        const GRID_DATA: u32 = 0x08;
        const MIN_ROW_FIELD: u32 = 0x2590;
        const MAX_ROW_FIELD: u32 = 0x2594;
        const GRID_WIDTH: u32 = 48;
        const MAX_COLUMN: i32 = 0x2f;

        #[inline(always)]
        unsafe fn read_u32(base: u32, offset: u32) -> u32 {
            core::ptr::read_unaligned(base.wrapping_add(offset) as *const u32)
        }

        let origin_x = read_u32(this, GRID_ORIGIN_X);
        let origin_y = read_u32(this, GRID_ORIGIN_Y);
        let left = left.wrapping_sub(origin_x) as i32;
        let right = right.wrapping_sub(origin_x) as i32;
        let min_row = read_u32(this, MIN_ROW_FIELD).wrapping_sub(origin_y) as i32;
        let max_row = read_u32(this, MAX_ROW_FIELD).wrapping_sub(origin_y) as i32;
        let mut first_row = top.wrapping_sub(origin_y) as i32;
        let mut last_row = bottom.wrapping_sub(origin_y) as i32;

        let first_column = (if left < 0 { 0 } else if left > MAX_COLUMN { MAX_COLUMN } else { left }) as u32;
        let last_column = (if right < 0 { 0 } else if right > MAX_COLUMN { MAX_COLUMN } else { right }) as u32;
        if first_row < min_row { first_row = min_row; }
        if first_row > max_row { first_row = max_row; }
        if last_row < min_row { last_row = min_row; }
        if last_row > max_row { last_row = max_row; }
        if first_row > last_row { return 0; }

        let mut result = 0u32;
        let mut row = first_row as u32;
        loop {
            let row_index = row.wrapping_mul(GRID_WIDTH);
            let mut cell = this
                .wrapping_add(GRID_DATA)
                .wrapping_add(row_index.wrapping_add(first_column).wrapping_mul(4));
            let mut remaining_columns = last_column.wrapping_sub(first_column) as i32;
            loop {
                result |= read_u32(cell, 0);
                cell = cell.wrapping_add(4);
                remaining_columns = remaining_columns.wrapping_sub(1);
                if remaining_columns < 0 { break; }
            }
            if row == last_row as u32 { break; }
            row = row.wrapping_add(1);
        }
        result
    }
});
