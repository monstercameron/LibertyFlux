// original: 0x00a4ff80 vehicle_count_free_slots (proposed)

/// Whether six inactive slots exist among the 256 flags: 1 if so.
///
/// Counts flag bytes at `this` with bit 0 clear, stopping at six (result 1)
/// or after all 256 (result reports none found). Note the not-found path
/// returns 0x100 in eax: only al is cleared while the counter sits at 256.
/// Thiscall, no stack words, result in eax.
lf_checker_rt::export!(thiscall, rw_00a4ff80(this: u32) -> u32 {
    unsafe {
        const NEED: u32 = 6;
        const NSLOTS: u32 = 0x100;
        const NOT_FOUND: u32 = 0x100;
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        let mut clears: u32 = 0;
        let mut i: u32 = 0;
        loop {
            if rd8(this.wrapping_add(i)) & 1 == 0 {
                clears = clears.wrapping_add(1);
                if clears >= NEED {
                    return 1;
                }
            }
            i = i.wrapping_add(1);
            if i < NSLOTS {
                continue;
            }
            return NOT_FOUND;
        }
    }
});
