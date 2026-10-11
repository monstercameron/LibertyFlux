// original: 0x00a29d10 camera_record_scalar_not_above_zero

/// Return the SETBE result for the record's scalar at offset 0xed4 compared
/// with zero. This includes unordered NaN inputs, which also set the result.
lf_checker_rt::export!(thiscall, rw_00a29d10(this: u32) -> u32 {
    const VALUE: u32 = 0xed4;
    const THRESHOLD: f32 = 0.0;
    let value = unsafe { (this.wrapping_add(VALUE) as *const f32).read_unaligned() };
    u32::from(!(value > THRESHOLD))
});
