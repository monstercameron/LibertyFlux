// original: 0x0093f070 Player_GetCurrentPos

/// Fill the caller's buffer through the position callee, return the buffer.
///
/// Calls the position callee with `buf` and returns `buf` unchanged.
///
/// Original: 0x0093f070 (cdecl, one stack word; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093f070(buf: u32) -> u32 {
    const FILL: u32 = 1;
    unsafe {
        lf_checker_rt::callee_cdecl!(FILL, u32, buf);
        buf
    }
});
