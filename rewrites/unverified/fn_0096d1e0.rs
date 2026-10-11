// original: 0x0096D1E0 build_timing_curve_sample

/// Forwards the four stack inputs to the curve-sample helper, preserving the
/// float both as the outgoing XMM0 value and as its matching stack word. A
/// three-word zeroed local record is passed by pointer and returned helper EAX
/// is preserved. The helper's pointer address is stack-local; its contents are
/// the observable call-time value.
lf_checker_rt::export!(thiscall, rw_0096d1e0(
    _this: u32,
    sample_kind: u32,
    sample_time: f32,
    lower_bound: u32,
    upper_bound: u32,
) -> u32 {
    let mut local_record = [0u32; 3];
    lf_checker_rt::callee_cdecl!(
        1,
        u32,
        sample_kind,
        sample_time.to_bits(),
        local_record.as_mut_ptr() as u32,
        lower_bound,
        upper_bound,
    )
});
