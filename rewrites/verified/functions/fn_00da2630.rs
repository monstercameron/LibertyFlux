// original: 0x00DA2630 stream_mode_policy
/// Applies a selector policy to a context mode stored in the flags word at offset 0x28. Selectors zero and one always succeed; selectors two through seven require mode three, selectors eight through fifteen require mode two, and out-of-range selectors or a null context fail.
lf_checker_rt::export!(cdecl, rw_s675_00da2630(selector: u32, context: u32) -> u32 {
    const MODE_FLAGS_OFFSET: u32 = 0x28;
    if context == 0 {
        return 0;
    }
    let flags = unsafe { (context.wrapping_add(MODE_FLAGS_OFFSET) as *const u32).read_unaligned() };
    let mode = (flags >> 6) & 0x0F;
    match selector {
        0..=1 => 1,
        2..=7 => (mode == 3) as u32,
        8..=15 => (mode == 2) as u32,
        _ => 0,
    }
});
