// original: 0x0098C3C0 audVehicleAudioEntity::compute_occluded_gains

/// Evaluates the vehicle's interior occlusion response, then writes the
/// transformed first response and the second curve response to two output
/// pointers. The helper results travel as single precision values; both
/// pointers are supplied on the stack by the thiscall caller.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_compute_occluded_gains(
    this: u32,
    first_output: u32,
    second_output: u32
) -> () {
    const FIRST_CURVE: u32 = 0x0128_3DF0;
    const SECOND_CURVE: u32 = 0x0128_3AA4;

    unsafe {
        let interior_occlusion: unsafe extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(lf_checker_rt::callee_addr(1));
        let evaluate_curve: unsafe extern "thiscall" fn(u32, f32) -> f32 =
            core::mem::transmute(lf_checker_rt::callee_addr(2));
        let linear_to_db: unsafe extern "cdecl" fn(f32) -> f32 =
            core::mem::transmute(lf_checker_rt::callee_addr(3));

        let raw_occlusion = interior_occlusion(this);
        let first_curve_value = evaluate_curve(
            lf_checker_rt::global::<u8>(FIRST_CURVE) as u32,
            raw_occlusion,
        );
        let transformed_first = linear_to_db(first_curve_value);
        (first_output as *mut f32).write_unaligned(transformed_first);

        let transformed_second = evaluate_curve(
            lf_checker_rt::global::<u8>(SECOND_CURVE) as u32,
            raw_occlusion,
        );
        (second_output as *mut f32).write_unaligned(transformed_second);
    }
});
