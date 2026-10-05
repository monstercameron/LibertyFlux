// original: 0x00E675B0 task_zero_pairs_00E675B0
/// Zero the task pair table.
///
/// Writes two zero words at `TABLE`, spaced `STRIDE` bytes apart,
/// for `PAIRS` pairs (256: the original counts ecx down from 0xFF
/// while non-negative).
///
/// Original: 0x00E675B0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E675B0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012F9538;
        const STRIDE: u32 = 0x8;
        const PAIRS: u32 = 256;
        let mut row = 0u32;
        while row < PAIRS {
            let at = TABLE.wrapping_add(row.wrapping_mul(STRIDE));
            (lf_checker_rt::global::<u32>(at)).write_unaligned(0);
            (lf_checker_rt::global::<u32>(at.wrapping_add(4))).write_unaligned(0);
            row += 1;
        }
        0
    }
});
