// original: 0x00a90c60 stream_row_store

/// Stores a value into a strided row-pointer table.
///
/// `this` holds a row-pointer array at `+0xE4`, a 16-bit row count at
/// `+0xE8`, a signed row index at `+0xFC` and a signed column index at
/// `+0x100`. When `index < count` (signed), writes `val` to
/// `rows[index * 160][column]` and returns `val`; otherwise returns the
/// count unchanged. No calls, no globals.
/// Original: 0x00A90C60 (thiscall, ECX + one stack word), 50 bytes.
lf_checker_rt::export!(thiscall, rw_00a90c60(this: u32, val: u32) -> u32 {
    unsafe {
        const ROWS_OFF: u32 = 0xE4;
        const COUNT_OFF: u32 = 0xE8;
        const INDEX_OFF: u32 = 0xFC;
        const COL_OFF: u32 = 0x100;
        const ROW_STRIDE: u32 = 160;
        let count = (this.wrapping_add(COUNT_OFF) as *const u16).read_unaligned() as u32;
        let index = (this.wrapping_add(INDEX_OFF) as *const i32).read_unaligned();
        if index >= count as i32 {
            return count;
        }
        let rows = (this.wrapping_add(ROWS_OFF) as *const u32).read_unaligned();
        let row = (rows.wrapping_add((index as u32).wrapping_mul(ROW_STRIDE)) as *const u32)
            .read_unaligned();
        let col = (this.wrapping_add(COL_OFF) as *const i32).read_unaligned();
        (row.wrapping_add((col.wrapping_mul(4)) as u32) as *mut u32).write_unaligned(val);
        val
    }
});
