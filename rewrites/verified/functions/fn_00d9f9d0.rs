// original: 0x00D9F9D0 stream_sample_in_range
/// Returns whether the input float is inclusively between the values at object offsets 0x0 and 0x4; unordered comparisons follow the x86 COMISS branch behavior.
lf_checker_rt::export!(thiscall, rw_s675_00d9f9d0(this: u32, input_bits: u32) -> u32 {
    const LOWER_BOUND: u32 = 0x0;
    const UPPER_BOUND: u32 = 0x4;
    let lower_bits = unsafe { (this.wrapping_add(LOWER_BOUND) as *const u32).read_unaligned() };
    let upper_bits = unsafe { (this.wrapping_add(UPPER_BOUND) as *const u32).read_unaligned() };
    let value = f32::from_bits(input_bits);
    let lower = f32::from_bits(lower_bits);
    let upper = f32::from_bits(upper_bits);
    ((! (lower > value || value > upper)) as u8) as u32
});
