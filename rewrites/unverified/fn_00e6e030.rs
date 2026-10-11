// original: 0x00E6E030 store_global_float_ratio_00e6e030

/// Read one f32 numerator from 0x010597A8 and one f32
/// denominator from 0x010597AC, divide numerator by denominator in
/// that operand order, and store the exact result bits at 0x017ACCBC. IEEE
/// zero, infinity and NaN behavior is preserved by one f32 division. The original
/// does not define EAX; the proof compares the global store and leaves EAX out.

lf_checker_rt::export!(cdecl, rw_store_global_float_ratio_00e6e030() -> u32 {
    unsafe {
        const NUMERATOR_VA: u32 = 0x010597A8;
        const DENOMINATOR_VA: u32 = 0x010597AC;
        const OUTPUT_VA: u32 = 0x017ACCBC;
        let numerator_bits = lf_checker_rt::global::<u32>(NUMERATOR_VA).read_unaligned();
        let denominator_bits = lf_checker_rt::global::<u32>(DENOMINATOR_VA).read_unaligned();
        let numerator = f32::from_bits(numerator_bits);
        let denominator = f32::from_bits(denominator_bits);
        let quotient = core::hint::black_box(numerator)
            / core::hint::black_box(denominator);
        lf_checker_rt::global::<u32>(OUTPUT_VA).write_unaligned(quotient.to_bits());
        0
    }
});
