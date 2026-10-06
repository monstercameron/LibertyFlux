// original: 0x005B6880 search_keyed_rows (proposed)

/// Find the first row past a start index whose key word matches.
///
/// Reads the target from the search key at +0x80 and the start index minus
/// one from +0x84 (so -1 starts at row 0). Rows past the start are 0x88
/// bytes; the row whose word at +0x80 equals the target is returned, or, if
/// none matches (or the start is already at/past the 16-bit count, *signed*;
/// a start of -1 first compares one word just before the table), the table
/// base + 0x690 is returned instead. Thiscall: table in ECX (`rows` at +0,
/// `count` at +4), key pointer on the stack.
lf_checker_rt::export!(thiscall, rw_005B6880(this: u32, key: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const ROW_STRIDE: i32 = 0x88;
        const KEY_OFF: u32 = 0x80;
        const START_OFF: u32 = 0x84;
        const MISS_OFF: u32 = 0x690;

        let start = ((key.wrapping_add(START_OFF)) as *const i32).read().wrapping_add(1);
        let count = ((this.wrapping_add(COUNT)) as *const u16).read() as i32;
        if start >= count {
            return this.wrapping_add(MISS_OFF);
        }
        let rows = (this.wrapping_add(ROWS) as *const u32).read();
        let target = ((key.wrapping_add(KEY_OFF)) as *const u32).read();
        let mut i = start;
        while i < count {
            let row = (rows as i32).wrapping_add(i.wrapping_mul(ROW_STRIDE)) as u32;
            if ((row.wrapping_add(KEY_OFF)) as *const u32).read() == target {
                return row;
            }
            i += 1;
        }
        this.wrapping_add(MISS_OFF)
    }
});
