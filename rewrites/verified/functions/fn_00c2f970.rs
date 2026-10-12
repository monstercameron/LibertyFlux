// original: 0x00C2F970 vehicle_dynamics_get_handling_ratio

/// Read a signed 16-bit handling index from `this`, follow the vehicle's
/// model-info pointer at byte offset `0xdc8`, then return the single-precision
/// quotient of the model value at `0x4c` divided by the indexed value beginning
/// at `0x54`. The value is returned through the x87 single-precision result
/// channel. The thiscall method takes one vehicle pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00c2f970(
    handling_index: *const u8,
    vehicle: *const u8,
) -> f32 {
    const MODEL_INFO_POINTER_OFFSET: usize = 0xdc8;
    const HANDLING_VALUE_OFFSET: usize = 0x4c;
    const FIRST_RATIO_OFFSET: isize = 0x54 / 4;

    #[inline(never)]
    fn div_f32(numerator: f32, denominator: f32) -> f32 {
        core::hint::black_box(numerator) / core::hint::black_box(denominator)
    }

    unsafe {
        let signed_index = handling_index.cast::<i16>().read_unaligned() as isize;
        let model_info = vehicle
            .add(MODEL_INFO_POINTER_OFFSET)
            .cast::<u32>()
            .read_unaligned() as *const u8;
        let numerator = model_info
            .add(HANDLING_VALUE_OFFSET)
            .cast::<f32>()
            .read_unaligned();
        let denominator = model_info
            .cast::<f32>()
            .offset(FIRST_RATIO_OFFSET + signed_index)
            .read_unaligned();
        div_f32(numerator, denominator)
    }
});
