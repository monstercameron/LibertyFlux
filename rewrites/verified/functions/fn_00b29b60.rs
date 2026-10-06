// original: 0x00b29b60 slot_find_tail_nearest

/// Finds a file slot by id and tail-calls the nearest-record search.
///
/// Scans the 24 slots in order for the first slot whose flag byte is nonzero
/// and whose id word equals `a0`, taking slot 24 when none matches, then
/// tail-calls the row search with (index, `a1`) and returns its answer.
/// Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b29b60(a0: u32, a1: u32) -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_IDS: u32 = 0x01657650;
        const SLOT_COUNT: u32 = 24;
        const SEARCH: u32 = 0;
        let flags = lf_checker_rt::relocated(SLOT_FLAGS);
        let ids = lf_checker_rt::relocated(SLOT_IDS);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let flag = ((flags + i) as *const u8).read();
            let cur = ((ids + i * 4) as *const u32).read_unaligned();
            if flag != 0 && cur == a0 {
                break;
            }
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(SEARCH, u32, i, a1)
    }
});
