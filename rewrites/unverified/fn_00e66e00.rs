// original: 0x00E66E00 global_ratio_store_03

/// Divide one global float by another and store the ratio into a third global.
///
/// Reads the two `f32` inputs at `SRC_A` and `SRC_B`, computes `a / b` with
/// single-precision SSE division semantics (division by zero yields infinity,
/// `0 / 0` and `inf / inf` yield a quiet NaN, NaN inputs propagate), and
/// writes the result bits to `DST`. Bit-exact: the quotient is produced by one
/// hardware `divss`, with the original's operand order pinned through
/// `black_box` so the compiler cannot fold or commute it.
///
/// Original: 0x00E66E00 (cdecl, no arguments, no outgoing calls, no return value).
lf_checker_rt::export!(cdecl, rw_00e66e00() -> u32 {
    unsafe {
        const SRC_A: u32 = 0x0103B570;
        const SRC_B: u32 = 0x0103B574;
        const DST: u32 = 0x012B6494;
        let a = f32::from_bits((lf_checker_rt::global::<u32>(SRC_A)).read_unaligned());
        let b = f32::from_bits((lf_checker_rt::global::<u32>(SRC_B)).read_unaligned());
        let q = core::hint::black_box(a) / core::hint::black_box(b);
        (lf_checker_rt::global::<u32>(DST)).write_unaligned(q.to_bits());
    }
    0
});
