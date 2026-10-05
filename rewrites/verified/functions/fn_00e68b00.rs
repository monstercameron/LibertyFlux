// original: 0x00E68B00 veh_ratio_recompute (proposed)
/// Recompute one vehicle tuning ratio from two static words.
///
/// Reads the single-precision numerator at `NUM_ADDR` and the denominator at
/// `DEN_ADDR`, divides them, and stores the quotient at `OUT_ADDR`. The
/// division is one IEEE-754 single operation in the original's operand order
/// (pinned against reassociation), so edge cases match bit for bit: zero
/// over zero yields a quiet NaN, a finite value over zero yields a signed
/// infinity, and infinities and NaNs propagate as the hardware defines.
///
/// Original: 0x00E68B00 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e68b00() -> u32 {
    unsafe {
        const NUM_ADDR: u32 = 0x0103F970;
        const DEN_ADDR: u32 = 0x0103F974;
        const OUT_ADDR: u32 = 0x015DBDD0;
        let n = f32::from_bits(
            (lf_checker_rt::global::<u32>(NUM_ADDR) as *const u32)
                .read_unaligned(),
        );
        let d = f32::from_bits(
            (lf_checker_rt::global::<u32>(DEN_ADDR) as *const u32)
                .read_unaligned(),
        );
        let q = core::hint::black_box(n) / core::hint::black_box(d);
        lf_checker_rt::global::<u32>(OUT_ADDR).write_unaligned(q.to_bits());
        0
    }
});
