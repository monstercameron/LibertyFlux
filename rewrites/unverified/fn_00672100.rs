// original: 0x00672100 streaming_is_empty_string

/// Return one when the optional C-string pointer is null or points to a
/// NUL byte; otherwise return zero. The function reads only the first byte.
///
/// Original: 0x00672100 (cdecl, one stack argument; boolean in AL).
lf_checker_rt::export!(cdecl, rw_00672100(text: u32) -> u32 {
    unsafe {
        if text == 0 || (text as *const u8).read() == 0 { 1 } else { 0 }
    }
});
