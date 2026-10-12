// original: 0x008C4CC0 path_is_relative

/// Classify a null-terminated path using its first byte and a scripted colon search. Null input
/// returns one. A leading slash, backslash, or zero byte returns EAX with its low byte cleared;
/// otherwise the intercepted cdecl search receives the path and colon, with zero meaning relative.
lf_checker_rt::export!(cdecl, rw_008c4cc0(path: u32) -> u32 {
    const SEARCH_CALLEE: u32 = 33;
    if path == 0 {
        return 1;
    }
    unsafe {
        let first = core::ptr::read_volatile(path as *const u8);
        if first == b'/' {
            return path & !0xff;
        }
        if first == b'\\' || first == 0 {
            return path & !0xff;
        }
        let colon = lf_checker_rt::callee_cdecl!(SEARCH_CALLEE, u32, path, 0x3a);
        if colon == 0 { 1 } else { colon & !0xff }
    }
});
