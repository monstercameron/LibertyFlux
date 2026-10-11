// original: 0x00ABC3A0 input_ui_forward_five_values

/// Forward four cdecl arguments and a zero word to a helper.
///
/// The fifth helper argument is always zero, inserted before the original
/// fourth argument. The helper's EAX result is returned unchanged. Both
/// functions use caller cleanup; the checker contract intercepts the helper
/// and compares all five forwarded words and the scripted result.
lf_checker_rt::export!(cdecl, rw_00abc3a0(first: u32, second: u32, third: u32, fourth: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, first, second, third, 0u32, fourth)
});
