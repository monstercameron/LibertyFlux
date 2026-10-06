// original: 0x00b2a4e0 slot_find_tail_next

/// Finds a file slot by id and tail-calls its follow-up, if any.
///
/// Scans the 24 slots in order for the first slot whose flag byte is nonzero
/// and whose id word equals `a0`. On a miss returns 24 without calling.
/// On a hit tail-calls the follow-up with (index, `a1`) and returns its
/// answer. Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b2a4e0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_IDS: u32 = 0x01657650;
        const SLOT_COUNT: u32 = 24;
        const NEXT: u32 = 0;
        let flags = lf_checker_rt::relocated(SLOT_FLAGS);
        let ids = lf_checker_rt::relocated(SLOT_IDS);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let flag = ((flags + i) as *const u8).read();
            let cur = ((ids + i * 4) as *const u32).read_unaligned();
            if flag != 0 && cur == a0 {
                return lf_checker_rt::callee_cdecl!(NEXT, u32, i, a1);
            }
            i += 1;
        }
        SLOT_COUNT
    }
});
