// original: 0x00D4F7B0 check_target_against_global_matcher
/// Apply the global target matcher to a non-null target and return its low-byte
/// result. The first global word selects the matcher object and the second
/// global word is forwarded as its comparison value. A null target returns
/// false without making either call. Both observed calls are scripted by the
/// checker; no local object fields are read or written.
lf_checker_rt::export!(cdecl, rw_00d4f7b0(target: u32) -> u8 {
    const TARGET_MATCHER_GLOBAL: u32 = 0x016d_d63c;
    const COMPARISON_VALUE_GLOBAL: u32 = 0x0104_96e8;
    const QUERY_MATCHER: u32 = 1;
    const CHECK_TARGET: u32 = 2;

    if target == 0 {
        return 0;
    }

    let matcher = unsafe { lf_checker_rt::global::<u32>(TARGET_MATCHER_GLOBAL).read_unaligned() };
    let query_result: u32 =
        lf_checker_rt::callee_thiscall!(QUERY_MATCHER, u32, matcher, target);
    let comparison_value = unsafe {
        lf_checker_rt::global::<u32>(COMPARISON_VALUE_GLOBAL).read_unaligned()
    };
    let matches: u8 =
        lf_checker_rt::callee_cdecl!(CHECK_TARGET, u8, query_result, comparison_value);
    u8::from(matches != 0)
});
