// original: 0x00e62680 store_graphics_scale_ratio_a

/// Store the single-precision quotient of the fixed graphics scale numerator and the global scale denominator. The integer return register is not part of this void operation.
lf_checker_rt::export!(cdecl, rw_00e62680() -> () {
    const NUMERATOR_WORD: u32 = 0x00fe88e8;
    const DENOMINATOR_WORD: u32 = 0x010300a0;
    const RESULT_WORD: u32 = 0x0115a424;
    let numerator_bits = unsafe { lf_checker_rt::global::<u32>(0x00fe88e8).read_volatile() };
    let denominator_bits = unsafe { lf_checker_rt::global::<u32>(0x010300a0).read_volatile() };
    let numerator = core::hint::black_box(f32::from_bits(numerator_bits));
    let denominator = core::hint::black_box(f32::from_bits(denominator_bits));
    let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<u32>(RESULT_WORD).write_volatile(quotient.to_bits()); }
});
