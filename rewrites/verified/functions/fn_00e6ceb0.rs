// original: 0x00E6CEB0 vehicle_tuning_ratio_init_38
/// Reads two image-resident f32 values, divides the numerator by the denominator in that operand order, stores the quotient in the output global, and returns the same f32 in XMM0. It takes no arguments and uses the cdecl convention.
const NUMERATOR_VA: u32 = 0x01056A38;
const DENOMINATOR_VA: u32 = 0x01056A3C;
const OUTPUT_VA: u32 = 0x0179767C;
lf_checker_rt::export!(cdecl, rw_00e6ceb0() -> f32 {
    let numerator = unsafe { lf_checker_rt::global::<f32>(NUMERATOR_VA).read() };
    let denominator = unsafe { lf_checker_rt::global::<f32>(DENOMINATOR_VA).read() };
    let quotient = core::hint::black_box(numerator) / core::hint::black_box(denominator);
    unsafe { lf_checker_rt::global::<f32>(OUTPUT_VA).write(quotient) };
    quotient
});
