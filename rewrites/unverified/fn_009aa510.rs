// original: 0x009aa510 table_value_fetch
/// Fetch a row's value word and report whether the row is live.
///
/// Looks up row `index` in the Stride25 table (`this+0x9d0`, count at
/// table `+0x0a`). When the table is missing, the index is out of range,
/// or the row's signed status word at row start `+0x10` is negative,
/// stores 0 into `*out` and 0 into `*found` and returns. Otherwise stores
/// the value word at row start `+0x21` into `*out` and 1 into `*found`.
/// Thiscall, three stack words, no result.
export!(thiscall, rw_009AA510(this: u32, index: u32, out: u32, found: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const ROW_COUNT: u32 = 0x0a;
        const ROW_STRIDE: u32 = 25;
        const STATUS_OFF: u32 = 0x10;
        const VALUE_OFF: u32 = 0x21;
        let ok = (|| -> Option<u32> {
            let table = ((this + TABLE_PTR) as *const u32).read_unaligned();
            if table == 0 {
                return None;
            }
            if (index as i32) < 0 {
                return None;
            }
            let count = ((table + ROW_COUNT) as *const u16).read_unaligned() as u32;
            if index >= count {
                return None;
            }
            let row = table + index * ROW_STRIDE;
            let status = (row + STATUS_OFF) as *const i32;
            if status.read_unaligned() < 0 {
                return None;
            }
            Some(((row + VALUE_OFF) as *const u32).read_unaligned())
        })();
        match ok {
            Some(v) => {
                (out as *mut u32).write_unaligned(v);
                (found as *mut u8).write(1);
            }
            None => {
                (out as *mut u32).write_unaligned(0);
                (found as *mut u8).write(0);
            }
        }
        0
    }
});
