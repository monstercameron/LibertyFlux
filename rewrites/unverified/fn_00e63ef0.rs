// original: 0x00E63EF0 store_resource_unit_ratio
/// Divides the configured numerator by its denominator as an IEEE-754 single-precision operation and stores the exact result bits in the output global.
lf_checker_rt::export!(cdecl, rw_00E63EF0() -> () {
    const NUMERATOR_VA: u32 = 0x01037744;
    const DENOMINATOR_VA: u32 = 0x01037748;
    const OUTPUT_VA: u32 = 0x0120896C;
    let numerator_bits = unsafe { lf_checker_rt::global::<u32>(NUMERATOR_VA).read() };
    let denominator_bits = unsafe { lf_checker_rt::global::<u32>(DENOMINATOR_VA).read() };
    let numerator = f32::from_bits(numerator_bits);
    let denominator = f32::from_bits(denominator_bits);
    let value = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<u32>(OUTPUT_VA).write(value.to_bits()); }
});
