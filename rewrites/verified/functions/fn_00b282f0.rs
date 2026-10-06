// original: 0x00b282f0 slot_find_write_value

/// Finds a file slot by id and copies its value to the caller.
///
/// Scans the 24 slots in order for the first slot whose flag byte is nonzero
/// and whose id word equals `id`. On a hit writes that slot's value word to
/// `*out` and returns `out` with its low byte set to 1 (the original moves
/// the pointer into EAX and then sets AL). On a miss returns `id` with its
/// low byte cleared (the original clears only AL, leaving the argument's
/// upper bytes). Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b282f0(id: u32, out: u32) -> u32 {
    unsafe {
        const SLOT_FLAGS: u32 = 0x01657890;
        const SLOT_IDS: u32 = 0x01657650;
        const SLOT_VALS: u32 = 0x01657830;
        const SLOT_COUNT: u32 = 24;
        let flags = lf_checker_rt::relocated(SLOT_FLAGS);
        let ids = lf_checker_rt::relocated(SLOT_IDS);
        let vals = lf_checker_rt::relocated(SLOT_VALS);
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let flag = ((flags + i) as *const u8).read();
            let cur = ((ids + i * 4) as *const u32).read_unaligned();
            if flag != 0 && cur == id {
                let v = ((vals + i * 4) as *const u32).read_unaligned();
                (out as *mut u32).write_unaligned(v);
                return (out & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        id & 0xFFFF_FF00
    }
});
