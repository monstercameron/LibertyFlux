// original: 0x00CF1FD0 playstat_append_pair

/// Forward two play-stat values and their constant key to the append helper.
///
/// The fourth source argument is not read. The five cdecl helper arguments
/// are `(first, second, key, third, fifth)` and its EAX value is returned.
lf_checker_rt::export!(cdecl, rw_fn_00cf1fd0(first: u32, second: u32, third: u32, unused_fourth: u32, fifth: u32) -> u32 {
    const PAIR_KEY_VA: u32 = 0x00ed_e464;
    const APPEND_CALLEE: u32 = 1;
    let _ = unused_fourth;
    lf_checker_rt::callee_cdecl!(APPEND_CALLEE, u32, first, second,
        lf_checker_rt::relocated(PAIR_KEY_VA), third, fifth)
});
