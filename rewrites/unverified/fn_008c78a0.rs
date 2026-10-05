// original: 0x008C78A0 stream_request_forward
/// Forward three caller words to the request callee with fixed framing.
///
/// Calls the request callee with seven words: a leading 0, `first`, the
/// request-base constant, `second`, `third`, and two trailing 0 words (the
/// original computes two scratch addresses and zeroes them in place, so
/// the callee always sees 0 there). Returns nothing.
/// Original: cdecl, three stack words.
lf_checker_rt::export!(cdecl, rw_008c78a0(first: u32, second: u32,
                                           third: u32) -> u32 {
    unsafe {
        const REQ_CALLEE: u32 = 1;
        const REQ_BASE: u32 = 0x1173100;
        lf_checker_rt::callee_cdecl!(
            REQ_CALLEE, u32, 0, first, REQ_BASE, second, third, 0, 0);
        0
    }
});
