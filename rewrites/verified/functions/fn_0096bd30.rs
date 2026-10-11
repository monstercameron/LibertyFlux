// original: 0x0096BD30 contains_timing_pair

/// Searches six rows of parallel 32-bit values: the first column starts at
/// `0x31e0`, the second at `0x31f8`. Both arguments must match the same row;
/// the result is returned in AL.
lf_checker_rt::export!(thiscall, rw_0096bd30(this: u32, first_key: u32, second_key: u32) -> u32 {
    unsafe {
        const FIRST_COLUMN: u32 = 0x31e0;
        const SECOND_COLUMN: u32 = 0x31f8;
        for row in 0..6u32 {
            let first = (this.wrapping_add(FIRST_COLUMN).wrapping_add(row * 4) as *const u32)
                .read_unaligned();
            let second = (this.wrapping_add(SECOND_COLUMN).wrapping_add(row * 4) as *const u32)
                .read_unaligned();
            if first == first_key && second == second_key {
                return 1;
            }
        }
    }
    0
});
