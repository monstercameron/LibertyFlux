// original: 0x00D9F8C0 stream_value_in_wrapped_range
/// Calls the float normalization helper, then checks the returned value against two bounds. Normal bounds form an inclusive interval; reversed or unordered bounds use the game's wrap-around predicate.
lf_checker_rt::export!(thiscall, rw_s675_00d9f8c0(this: u32, input_bits: u32) -> u32 {
    const LOWER_OFFSET: u32 = 0x10;
    const UPPER_OFFSET: u32 = 0x14;
    let normalized: f32 = lf_checker_rt::callee_cdecl!(1, f32, input_bits);
    let lower_bits = unsafe { (this.wrapping_add(LOWER_OFFSET) as *const u32).read_unaligned() };
    let upper_bits = unsafe { (this.wrapping_add(UPPER_OFFSET) as *const u32).read_unaligned() };
    let lower = f32::from_bits(lower_bits);
    let upper = f32::from_bits(upper_bits);
    let wrap = !(upper >= lower);
    let in_range = if wrap { normalized >= lower || upper >= normalized } else { normalized >= lower && upper >= normalized };
    in_range as u32
});
