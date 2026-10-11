// original: 0x00e62c60 store_audio_scale_ratio_c

/// Store the single-precision quotient of the third audio scale pair. The integer return register is not part of this void operation.
lf_checker_rt::export!(cdecl, rw_00e62c60() -> () {
    const NUMERATOR_WORD: u32 = 0x01030d30;
    const DENOMINATOR_WORD: u32 = 0x01030d34;
    const RESULT_WORD: u32 = 0x0116bb60;
    let numerator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030d30).read_volatile() };
    let denominator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030d34).read_volatile() };
    let numerator = core::hint::black_box(f32::from_bits(numerator_bits));
    let denominator = core::hint::black_box(f32::from_bits(denominator_bits));
    let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<u32>(RESULT_WORD).write_volatile(quotient.to_bits()); }
});
