// original: 0x00e686d0 veh_ratio_86d0

/// Refresh one vehicle coefficient as the ratio of two stored floats.
///
/// Reads the numerator from `NUM_ADDR` and the denominator from `DEN_ADDR`
/// (both single-precision floats in static storage), divides them with SSE
/// scalar semantics (`divss`: no fault on a zero denominator; IEEE-754
/// result including infinities and NaNs), and stores the quotient at
/// `DST_ADDR`. Takes no arguments, makes no calls and returns nothing: the
/// one stored word is the whole observable effect. The division order is
/// pinned so the compiler cannot commute the operands.
///
/// Original: 0x00E686D0 (cdecl/0, three absolute globals).
lf_checker_rt::export!(cdecl, rw_00e686d0() -> () {
    unsafe {
        /// Numerator slot (file VA).
        const NUM_ADDR: u32 = 0x0103ED80;
        /// Denominator slot (file VA).
        const DEN_ADDR: u32 = 0x0103ED84;
        /// Quotient slot (file VA).
        const DST_ADDR: u32 = 0x013BAB98;
        let a = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(NUM_ADDR)));
        let b = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(DEN_ADDR)));
        let q = core::hint::black_box(a) / core::hint::black_box(b);
        core::ptr::write_unaligned(lf_checker_rt::global::<u32>(DST_ADDR), q.to_bits());
    }
});
