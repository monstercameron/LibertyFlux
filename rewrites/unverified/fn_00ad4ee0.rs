// original: 0x00AD4EE0 audio_zero_block_100 (proposed)

/// Zero a 100-word block in each of the two audio tables.
///
/// From the argument, `half = arg / 2 + 100000` (truncating division) and
/// `start = (half mod 100) * 0x190`; then 100 consecutive words are zeroed
/// in each table starting at `start` (cdecl/1). Arguments giving a negative
/// `half` would index before the tables; tested inputs keep it
/// non-negative. Returns 0.
lf_checker_rt::export!(cdecl, rw_00ad4ee0(arg: u32) -> u32 {
    unsafe {
        const TABLE_A: u32 = 0x01552C78;
        const TABLE_B: u32 = 0x0155C8B8;
        const BIAS: i32 = 100_000;
        const DIVISOR: i32 = 100;
        const COUNT: usize = 100;
        const STRIDE: i32 = 0x190;
        let half = (arg as i32 / 2).wrapping_add(BIAS);
        let start = (half % DIVISOR * STRIDE) as u32;
        for i in 0..COUNT {
            let off = start.wrapping_add((i as u32).wrapping_mul(4));
            (lf_checker_rt::relocated(TABLE_B).wrapping_add(off) as *mut u32).write(0);
            (lf_checker_rt::relocated(TABLE_A).wrapping_add(off) as *mut u32).write(0);
        }
        0
    }
});
