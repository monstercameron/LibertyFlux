// original: 0x00E675E0 task_ratio_store_00E675E0
/// Divide two task tuning floats and store the ratio.
///
/// Loads the dividend from the global `DIVIDEND` and the divisor from
/// the global `DIVISOR` (both single-precision), divides in the
/// original's operand order (pinned against reassociation with
/// `black_box`), and stores the quotient to the global `QUOTIENT`.
/// Edge cases (zero divisor, infinities, NaNs, denormals) follow the
/// hardware divide bit for bit.
///
/// Original: 0x00E675E0 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E675E0() -> u32 {
    unsafe {
        const DIVIDEND: u32 = 0x0103CDD0;
        const DIVISOR: u32 = 0x0103CDD4;
        const QUOTIENT: u32 = 0x012F9D38;
        let dividend: f32 = (lf_checker_rt::global::<f32>(DIVIDEND)).read_unaligned();
        let divisor: f32 = (lf_checker_rt::global::<f32>(DIVISOR)).read_unaligned();
        let q = core::hint::black_box(dividend) / core::hint::black_box(divisor);
        (lf_checker_rt::global::<f32>(QUOTIENT)).write_unaligned(q);
        0
    }
});
