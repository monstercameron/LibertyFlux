// original: 0x0096D180 append_timing_sample

/// Appends one converted sample to the object's integer timing array. Counts
/// at or above 64 flush the pending sample block first. The sample provider
/// returns f32 in ST0; x87 FISTP semantics truncate toward zero and produce
/// the low 32 bits of its qword result. EAX returns the previous sample count.
/// The input argument is forwarded unchanged to the provider.
lf_checker_rt::export!(thiscall, rw_0096d180(this: u32, sample_time: f32) -> u32 {
    const SAMPLE_COUNT: u32 = 0x233c;
    const SAMPLE_VALUES: u32 = 0x2640;
    const SAMPLE_LIMIT: u32 = 64;

    let count_address = this.wrapping_add(SAMPLE_COUNT);
    let count_before_flush = unsafe { (count_address as *const u32).read_unaligned() };
    if count_before_flush >= SAMPLE_LIMIT {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    }

    let sample = lf_checker_rt::callee_thiscall!(2, f32, this, sample_time.to_bits());
    let extended = f64::from(sample);
    let sample_integer = if !extended.is_finite()
        || extended < i64::MIN as f64
        || extended >= 9_223_372_036_854_775_808.0
    {
        0
    } else {
        (extended as i64) as u32
    };

    unsafe {
        let previous_count = (count_address as *const u32).read_unaligned();
        let sample_address = this
            .wrapping_add(SAMPLE_VALUES)
            .wrapping_add(previous_count.wrapping_mul(4));
        (sample_address as *mut u32).write_unaligned(sample_integer);
        (count_address as *mut u32).write_unaligned(previous_count.wrapping_add(1));
        previous_count
    }
});
