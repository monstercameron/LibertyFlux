// original: 0x00e69cc0 veh_global_ratio_20 (proposed)

/// Ratio of two single-precision vehicle globals stored into a third.
///
/// Loads `NUM` and `DEN`, divides, and stores the quotient at `OUT`.
/// All three are process globals reached by absolute address; the
/// function takes no arguments and returns nothing (cdecl/0). The
/// division is a single hardware divide, so zero, infinite and NaN
/// operands behave exactly as the hardware does, bit for bit:
/// `0/0` and `inf/inf` store quiet NaN, `x/0` stores signed infinity,
/// a NaN operand propagates quieted. Operand order is pinned with
/// `black_box` so the compiler cannot commute it.
///
/// Original: 0x00E69CC0 (cdecl, no arguments, no calls).
lf_checker_rt::export!(cdecl, rw_00e69cc0() -> () {
    unsafe {
        /// Dividend global (file VA).
        const NUM: u32 = 0x01047244;
        /// Divisor global (file VA).
        const DEN: u32 = 0x01047248;
        /// Quotient destination global (file VA).
        const OUT: u32 = 0x0167E2D0;
        let num = f32::from_bits((lf_checker_rt::global::<u32>(NUM) as *const u32).read_unaligned());
        let den = f32::from_bits((lf_checker_rt::global::<u32>(DEN) as *const u32).read_unaligned());
        let quo = core::hint::black_box(num) / core::hint::black_box(den);
        lf_checker_rt::global::<u32>(OUT).write_unaligned(quo.to_bits());
    }
});
