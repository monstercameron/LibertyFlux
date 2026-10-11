// original: 0x00e62c40 store_audio_scale_ratio_b

/// Store the single-precision quotient of the second audio scale pair. The integer return register is not part of this void operation.
lf_checker_rt::export!(cdecl, rw_00e62c40() -> () {
    const NUMERATOR_WORD: u32 = 0x01030d28;
    const DENOMINATOR_WORD: u32 = 0x01030d2c;
    const RESULT_WORD: u32 = 0x01168bb8;
    let numerator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030d28).read_volatile() };
    let denominator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030d2c).read_volatile() };
    let numerator = core::hint::black_box(f32::from_bits(numerator_bits));
    let denominator = core::hint::black_box(f32::from_bits(denominator_bits));
    let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<u32>(RESULT_WORD).write_volatile(quotient.to_bits()); }
});
