// original: 0x009a99a0 table_flag_test_b
/// Test whether the row after `index` in the Stride25 table is flagged.
///
/// Same shape as `table_flag_test_a`, but the flag word read is at
/// `table + (index+1)*25 + 0x00`: the head word of the following row.
/// False for a missing table, a negative or out-of-range index, or a
/// zero flag word. Thiscall, one stack word, byte result.
export!(thiscall, rw_009A99A0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const ROW_COUNT: u32 = 0x0a;
        const ROW_STRIDE: u32 = 25;
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
        let flag = (table + (index + 1) * ROW_STRIDE) as *const u32;
        (flag.read_unaligned() != 0) as u32
    }
});
