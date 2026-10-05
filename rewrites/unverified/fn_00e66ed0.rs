// original: 0x00E66ED0 task_zero_flags_00E66ED0
/// Zero the task flag table and its trailing word.
///
/// Writes zero to `WORDS` words starting at `TABLE`, spaced
/// `STRIDE` bytes apart (17: the original counts ecx down from
/// 0x10 while non-negative), then zeroes the global `EXTRA`.
///
/// Original: 0x00E66ED0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E66ED0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012BCD40;
        const STRIDE: u32 = 0x2C;
        const WORDS: u32 = 17;
        const EXTRA: u32 = 0x012BD004;
        let mut row = 0u32;
        while row < WORDS {
            (lf_checker_rt::global::<u32>(TABLE.wrapping_add(row.wrapping_mul(STRIDE)))).write_unaligned(0);
            row += 1;
        }
        (lf_checker_rt::global::<u32>(EXTRA)).write_unaligned(0);
        0
    }
});
