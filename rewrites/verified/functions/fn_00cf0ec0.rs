// original: 0x00CF0EC0 stat_is_frontend_visible

/// Decide whether a statistic is visible in the frontend.
///
/// The cdecl argument is a statistic identifier. All identifiers except
/// `0x2b` return true without a call. Identifier `0x2b` asks the intercepted
/// visibility helper with constant `0x294` and returns whether its full EAX
/// answer is nonzero. The public result is the Boolean byte in AL.
lf_checker_rt::export!(cdecl, rw_fn_00cf0ec0(stat_id: u32) -> u32 {
    const SPECIAL_STAT_ID: u32 = 0x2b;
    const VISIBILITY_NATIVE_ID: u32 = 0x294;
    const VISIBILITY_CALLEE: u32 = 1;
    if stat_id != SPECIAL_STAT_ID {
        return 1;
    }
    let answer: u32 = lf_checker_rt::callee_cdecl!(VISIBILITY_CALLEE, u32, VISIBILITY_NATIVE_ID);
    u32::from(answer != 0)
});
