// original: 0x00a25230 camera_record_clamp_scalar_to_interval

/// Clamp the camera record's scalar at offset 0x218 against the interval's
/// upper value at 0x1c and lower value at 0x20. Ordered comparisons preserve
/// the original COMISS behavior for unordered inputs.
lf_checker_rt::export!(thiscall, rw_00a25230(this: u32, interval: u32) -> u32 {
    const CURRENT: u32 = 0x218;
    let current = unsafe { (this.wrapping_add(CURRENT) as *const f32).read_unaligned() };
    let upper = unsafe { (interval.wrapping_add(0x1c) as *const f32).read_unaligned() };
    let lower = unsafe { (interval.wrapping_add(0x20) as *const f32).read_unaligned() };
    let capped = if current > upper { upper } else { current };
    let result = if capped > lower { capped } else { lower };
    unsafe { (this.wrapping_add(CURRENT) as *mut f32).write_unaligned(result); }
    0
});
