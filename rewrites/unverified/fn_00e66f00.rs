// original: 0x00E66F00 task_ratio_store_00E66F00
/// Divide two task tuning floats and store the ratio.
///
/// Loads the dividend from the global `DIVIDEND` and the divisor from
/// the global `DIVISOR` (both single-precision), divides in the
/// original's operand order (pinned against reassociation with
/// `black_box`), and stores the quotient to the global `QUOTIENT`.
/// Edge cases (zero divisor, infinities, NaNs, denormals) follow the
/// hardware divide bit for bit.
///
/// Original: 0x00E66F00 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E66F00() -> u32 {
    unsafe {
        const DIVIDEND: u32 = 0x0103B6D4;
        const DIVISOR: u32 = 0x0103B6D8;
        const QUOTIENT: u32 = 0x012BD098;
        let dividend: f32 = (lf_checker_rt::global::<f32>(DIVIDEND)).read_unaligned();
        let divisor: f32 = (lf_checker_rt::global::<f32>(DIVISOR)).read_unaligned();
        let q = core::hint::black_box(dividend) / core::hint::black_box(divisor);
        (lf_checker_rt::global::<f32>(QUOTIENT)).write_unaligned(q);
        0
    }
});
