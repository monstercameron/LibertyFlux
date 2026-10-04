// original: 0x00dfb900 wcsncpy
/// Copy at most `count` wide characters from `src` to `dst`.
///
/// Stops after an embedded NUL and zero-fills the rest, exactly like the C
/// `wcsncpy` semantics: precisely `count` wide units are always written.
/// Returns `dst`.
export!(cdecl, rw_00dfb900(dst: *mut u16, src: *const u16, count: u32) -> *mut u16 {
    unsafe {
        let mut i = 0u32;
        while i < count {
            let w = *src.add(i as usize);
            *dst.add(i as usize) = w;
            i += 1;
            if w == 0 {
                break;
            }
        }
        while i < count {
            *dst.add(i as usize) = 0;
            i += 1;
        }
        dst
    }
});
