// original: 0x00a252a0 camera_query_and_select_compact_state

/// Query the helper with the first two stack words. Return zero when its low
/// byte is nonzero and the third word is zero; otherwise return one for a
/// zero second word or two for a nonzero second word.
lf_checker_rt::export!(stdcall, rw_00a252a0(first: u32, second: u32, third: u32) -> u32 {
    let helper_result = lf_checker_rt::callee_stdcall!(1, u8, first, second);
    if helper_result != 0 && third == 0 {
        0
    } else {
        u32::from(second != 0) + 1
    }
});
