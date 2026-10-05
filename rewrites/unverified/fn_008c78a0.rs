// original: 0x008C78A0 stream_request_forward
/// Forward three caller words to the request callee with fixed framing.
///
/// Calls the request callee with seven words: a leading 0, `first`, the
/// request-base address, `second`, `third`, and two out-pointers to fresh
/// zeroed scratch words (the original passes addresses into its own frame
/// and zeroes the pointed-to words; only the addresses' contents are
/// behaviour, compared through the contract's snapshots). Returns nothing.
/// Original: cdecl, three stack words.
lf_checker_rt::export!(cdecl, rw_008c78a0(first: u32, second: u32,
                                           third: u32) -> u32 {
    unsafe {
        const REQ_CALLEE: u32 = 1;
        const REQ_BASE_FILE_VA: u32 = 0x1173100;
        let mut scratch = [0u32; 2];
        let lo = scratch.as_mut_ptr() as u32;
        let hi = scratch.as_mut_ptr().add(1) as u32;
        let base = lf_checker_rt::relocated(REQ_BASE_FILE_VA);
        lf_checker_rt::callee_cdecl!(
            REQ_CALLEE, u32, 0, first, base, second, third, hi, lo);
        0
    }
});
