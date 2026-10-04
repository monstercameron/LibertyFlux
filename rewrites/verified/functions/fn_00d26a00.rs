// original: 0x00d26a00 ped_stat_to_count (proposed)

/// Convert a callee-provided float into a counter: `trunc(f * 20.0 + 10.0)`.
///
/// Calls the stat helper (intercepted) with `arg`, scales by 20 and adds 10
/// in that order, then converts with the x87 integer-store semantics the
/// original gets by switching the control word to chop: truncation toward
/// zero, and 0 in the low word when the value is NaN, infinite or out of the
/// 64-bit range (the original reads only the low 32 bits of the store, which
/// are 0 in the indefinite case and for exactly -2^63).
///
/// Original: 0x00D26A00 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00d26a00(arg: u32) -> u32 {
    unsafe {
        const SCALE_BITS: u32 = 0x41a0_0000; // 20.0f
        const BIAS_BITS: u32 = 0x4120_0000; // 10.0f
        const TWO63: f32 = 9.223_372e18; // 2^63, exact in f32
        let f: f32 = lf_checker_rt::callee_cdecl!(1, f32, arg);
        let scaled = core::hint::black_box(f) * core::hint::black_box(f32::from_bits(SCALE_BITS));
        let x = core::hint::black_box(scaled) + core::hint::black_box(f32::from_bits(BIAS_BITS));
        let t = x.trunc();
        if t.is_nan() || t >= TWO63 || t < -TWO63 {
            0
        } else {
            (t as i64) as u32
        }
    }
});
