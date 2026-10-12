// original: 0x00CF1FF0 playstat_append_opt_field

/// Append an optional string field, or its empty-value fallback.
///
/// If `optional_text` is non-null and its first byte is nonzero, the helper
/// receives `(first, second, OPTIONAL_KEY, third, optional_text)`. Otherwise
/// it receives `(first, second, EMPTY_KEY, third, fourth)`. The helper result
/// is returned. The string byte is read only after the null check.
lf_checker_rt::export!(cdecl, rw_fn_00cf1ff0(first: u32, second: u32, third: u32, fourth: u32, optional_text: u32) -> u32 {
    const OPTIONAL_KEY_VA: u32 = 0x00ed_e46c;
    const EMPTY_KEY_VA: u32 = 0x00ed_e478;
    const APPEND_CALLEE: u32 = 1;
    let has_optional_text = optional_text != 0 && unsafe { (optional_text as *const u8).read() != 0 };
    if has_optional_text {
        lf_checker_rt::callee_cdecl!(APPEND_CALLEE, u32, first, second,
            lf_checker_rt::relocated(OPTIONAL_KEY_VA), third, optional_text)
    } else {
        lf_checker_rt::callee_cdecl!(APPEND_CALLEE, u32, first, second,
            lf_checker_rt::relocated(EMPTY_KEY_VA), third, fourth)
    }
});
