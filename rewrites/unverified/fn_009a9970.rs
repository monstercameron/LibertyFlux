// original: 0x009a9970 table_flag_test_a
/// Test whether row `index` of the Stride25 table carries a non-zero flag.
///
/// The table pointer lives at `this+0x9d0` (null means no table) and its
/// row count is the 16-bit word at table `+0x0a`. Rows are 25 bytes; the
/// flag word of row `index` sits at `table + index*25 + 0x1d`. The answer
/// is false when the table is missing, the index is negative or past the
/// end, or the flag word is zero. Thiscall, one stack word, byte result.
export!(thiscall, rw_009A9970(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const ROW_COUNT: u32 = 0x0a;
        const ROW_STRIDE: u32 = 25;
        const FLAG_OFF: u32 = 0x1d;
        let table = ((this + TABLE_PTR) as *const u32).read_unaligned();
        if table == 0 {
            return 0;
        }
        if (index as i32) < 0 {
            return 0;
        }
        let count = ((table + ROW_COUNT) as *const u16).read_unaligned() as u32;
        if index >= count {
            return 0;
        }
        let flag = (table + index * ROW_STRIDE + FLAG_OFF) as *const u32;
        (flag.read_unaligned() != 0) as u32
    }
});
