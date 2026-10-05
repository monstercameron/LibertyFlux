// original: 0x008C95A0 stream_handle_get
/// Return the active streaming handle, or null when streaming is off.
///
/// Reads the streaming state: when it is 1 or 2 the handle global holds a
/// live handle and is returned, otherwise the result is 0. Reads globals
/// only. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c95a0() -> u32 {
    unsafe {
        match *lf_checker_rt::global::<u32>(0x11730f8) {
            1 | 2 => *lf_checker_rt::global::<u32>(0x1172fe0),
            _ => 0,
        }
    }
});
