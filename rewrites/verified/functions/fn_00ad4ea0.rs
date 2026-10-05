// original: 0x00AD4EA0 audio_zero_column_strided (proposed)

/// Zero one strided column across the two audio tables.
///
/// From the argument, `half = arg / 2 + 100000` (truncating division) and
/// `start = (half mod 100) * 4`; then 100 words are zeroed in each table at
/// byte offsets `start + i * 0x190` (cdecl/1). The count is a constant 100:
/// the quotient is computed and then overwritten. Arguments giving a
/// negative `half` would index before the tables; tested inputs keep it
/// non-negative. Returns 0.
lf_checker_rt::export!(cdecl, rw_00ad4ea0(arg: u32) -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x01552C78;
        const TABLE_B: u32 = 0x0155C8B8;
        const BIAS: i32 = 100_000;
        const DIVISOR: i32 = 100;
        const COUNT: u32 = 100;
        const STRIDE: u32 = 0x190;
        let half = (arg as i32 / 2).wrapping_add(BIAS);
        let start = (half % DIVISOR * 4) as u32;
        let mut off = start;
        let mut n = COUNT;
        while n != 0 {
            (lf_checker_rt::relocated(TABLE_A).wrapping_add(off) as *mut u32).write(0);
            (lf_checker_rt::relocated(TABLE_B).wrapping_add(off) as *mut u32).write(0);
            off = off.wrapping_add(STRIDE);
            n -= 1;
        }
        0
    }
});
