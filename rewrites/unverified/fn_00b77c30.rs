// original: 0x00b77c30 slot_find_by_key (proposed)

/// Find the first of three table slots whose object key matches.
///
/// Scans the three pointers in the slot table, in order, and returns the
/// first non-null pointer whose object holds `key` in its key field. Returns
/// 0 when no slot matches. Null slots are skipped, never dereferenced.
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b77c30(key: u32) -> u32 {
    unsafe {
        const SLOT_TABLE: u32 = 0x0167CCC4;
        const KEY_FIELD: u32 = 0x1A0;
        const SLOTS: u32 = 3;
        let base = lf_checker_rt::relocated(SLOT_TABLE);
        let mut i = 0u32;
        while i < SLOTS {
            let p = ((base + i * 4) as *const u32).read_unaligned();
            if p != 0 && ((p + KEY_FIELD) as *const u32).read_unaligned() == key {
                return p;
            }
            i += 1;
        }
        0
    }
});
