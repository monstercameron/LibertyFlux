// original: 0x00C0BBF0 NativeImpl_IS_CHAR_IN_TAXI_2

/// Reports whether a value appears in one row of a word table.
///
/// `index` selects one of the rows: the row length comes from the counts
/// table at `COUNTS` (`counts[index]`), the row itself starts at
/// `TABLE + index * index * 70` and holds 16-bit words. When the
/// length is zero or negative as a SIGNED 32-bit value, returns 0
/// without reading the row. Otherwise scans the first `length` words
/// and returns 1 when any word, zero-extended to 32 bits, equals
/// `value` (compared as UNSIGNED 32-bit values), else 0. Only the low
/// byte of the result is defined; the upper bytes of EAX keep
/// scan-pointer leftovers, so the contract compares AL only.
///
/// Original: 0x00C0BBF0 (cdecl, two stack words; counts at 0x0169E320,
/// row table at 0x0169D568).
lf_checker_rt::export!(cdecl, rw_00c0bbf0(index: u32, value: u32) -> u32 {
    unsafe {
        /// Counts table: row length per index.
        const COUNTS: u32 = 0x0169E320;
        /// Row table holding the 16-bit word lists.
        const TABLE: u32 = 0x0169D568;
        /// Bytes per row.
        const STRIDE: u32 = 0x46;

        let count_addr = COUNTS.wrapping_add(index.wrapping_mul(4));
        let count = (lf_checker_rt::relocated(count_addr) as *const u32).read_unaligned();
        if (count as i32) <= 0 {
            return 0;
        }
        let mut slot = TABLE.wrapping_add(index.wrapping_mul(STRIDE));
        let mut done: u32 = 0;
        loop {
            let word = (lf_checker_rt::relocated(slot) as *const u16).read_unaligned() as u32;
            if value == word {
                return 1;
            }
            done = done.wrapping_add(1);
            slot = slot.wrapping_add(2);
            if !((done as i32) < (count as i32)) {
                return 0;
            }
        }
    }
});
