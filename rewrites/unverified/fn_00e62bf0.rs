// original: 0x00e62bf0 store_audio_scale_ratio_a

/// Store the single-precision quotient of the first audio scale word by the second audio scale word. The integer return register is not part of this void operation.
lf_checker_rt::export!(cdecl, rw_00e62bf0() -> () {
    const NUMERATOR_WORD: u32 = 0x01030c70;
    const DENOMINATOR_WORD: u32 = 0x01030c74;
    const RESULT_WORD: u32 = 0x01168b1c;
    let numerator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030c70).read_volatile() };
    let denominator_bits = unsafe { lf_checker_rt::global::<u32>(0x01030c74).read_volatile() };
    let numerator = core::hint::black_box(f32::from_bits(numerator_bits));
    let denominator = core::hint::black_box(f32::from_bits(denominator_bits));
    let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<u32>(RESULT_WORD).write_volatile(quotient.to_bits()); }
});
