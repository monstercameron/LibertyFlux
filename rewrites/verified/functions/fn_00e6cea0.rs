// original: 0x00E6CEA0 convert_vehicle_tuning_float_to_int
/// Reads an f32 value, converts it to signed i32 by truncating toward zero, stores and returns the integer. CVTTSS2SI yields i32::MIN for NaN, values at or above 2^31, or values below -2^31; the Rust cast is used only after that bound check.
const INPUT_VA: u32 = 0x00EAA3D4;
const OUTPUT_VA: u32 = 0x01797674;
const I32_LOWER: f32 = -2_147_483_648.0;
const I32_UPPER_EXCLUSIVE: f32 = 2_147_483_648.0;
lf_checker_rt::export!(cdecl, rw_00e6cea0() -> i32 {
    let value = unsafe { lf_checker_rt::global::<f32>(INPUT_VA).read() };
    let converted = if value.is_nan() || value < I32_LOWER || value >= I32_UPPER_EXCLUSIVE {
        i32::MIN
        } else {
        // The bounds above ensure this truncating cast is representable.
        value as i32
    };
    unsafe { lf_checker_rt::global::<i32>(OUTPUT_VA).write(converted) };
    converted
});
