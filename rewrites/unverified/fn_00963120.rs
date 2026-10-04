// original: 0x00963120 record_table_find
/// Find a record in the 0x400-entry table at 0x120F2B8.
///
/// Scans the 0x14-byte records for the first one whose flag word (+0) is
/// nonzero, whose key word (+8) equals `key` (the first argument) and whose
/// sum word (+12) equals `a.wrapping_add(b)` (the other two added). Marks
/// the found record's tag byte (+0x10) with 1.
/// Returns five times the record index, or 5 * 0x3FF when no record matches.
export!(cdecl, rw_00963120(key: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x120F2B8;
        const COUNT: u32 = 0x400;
        const NOT_FOUND: u32 = 0x3FF * 5;
        let want_sum = a.wrapping_add(b);
        let mut idx = 0u32;
        loop {
            let row = (relocated(BASE) as *const u8).add((idx * 0x14) as usize);
            let flag = *(row as *const u32);
            let f1 = *(row.add(8) as *const u32);
            let f2 = *(row.add(12) as *const u32);
            if flag != 0 && f1 == key && f2 == want_sum {
                *(row.add(0x10) as *mut u8) = 1;
                return idx * 5;
            }
            idx += 1;
            if idx >= COUNT {
                return NOT_FOUND;
            }
        }
    }
});
