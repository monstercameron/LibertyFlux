// original: 0x008CEA10 stream_first_idle_bank (proposed)

/// Finds the first idle bank among banks 0, 1 and 2: returns the lowest
/// index whose enable byte in `FLAGS` is 0. Returns 0 both when bank 0 is
/// idle and when all three banks are busy.
///
/// No arguments (cdecl); the result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CEA10() -> u32 {
    unsafe {
        /// Base of the per-bank enable bytes.
        const FLAGS: u32 = 0x117354C;
        /// Number of banks scanned.
        const BANKS: u32 = 3;
        let base = lf_checker_rt::relocated(FLAGS);
        let mut i = 0u32;
        while i < BANKS {
            if ((base + i) as *const u8).read() == 0 {
                return i;
            }
            i += 1;
        }
        0
    }
});
