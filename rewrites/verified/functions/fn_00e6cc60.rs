// original: 0x00e6cc60 veh_ratio_update_17 (proposed)

/// Recompute one stored vehicle ratio from its two source globals.
///
/// Reads the numerator float at `NUM` and the denominator float at `DEN`
/// (an adjacent pair of initialized globals, pristine 1024.0 and 768.0),
/// divides them with single-precision SSE semantics, and stores the quotient
/// at `DST` (a zero-initialized global). No arguments, no return value
/// (`cdecl`, plain `ret`); the quotient is also left in the low lane of
/// `xmm0`, which is caller-saved scratch, not a result channel.
///
/// Edge cases follow the hardware instruction exactly: a zero denominator
/// yields a signed infinity (or NaN for 0/0), infinities and NaNs propagate
/// per IEEE-754, and denormals divide without traps, since the build runs
/// with default SSE control state. The division is written in the original's
/// operand order with both operands behind `black_box` so the compiler keeps
/// the single-precision divide it would otherwise be free to reshape.
lf_checker_rt::export!(cdecl, rw_00e6cc60() -> u32 {
    unsafe {
        /// Numerator global (file VA).
        const NUM: u32 = 0x010556D0;
        /// Denominator global (file VA), immediately after the numerator.
        const DEN: u32 = 0x010556D4;
        /// Destination global (file VA).
        const DST: u32 = 0x01723B98;
        let num = *lf_checker_rt::global::<f32>(NUM);
        let den = *lf_checker_rt::global::<f32>(DEN);
        let q = core::hint::black_box(num) / core::hint::black_box(den);
        *lf_checker_rt::global::<f32>(DST) = q;
    }
    0
});
