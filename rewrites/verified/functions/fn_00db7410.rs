// original: 0x00DB7410 UIFontString::vf118

/// Select a scratch slot by kind, resolve the shared text handle through the
/// registry, store it at `+0x200`, and notify the backend.
///
/// Kind 0 uses size 2 and the last scratch pair, kind 2 size 8 and the
/// middle pair, any other kind size 8 and the first pair (all equality
/// tests on the stack argument, so signedness is moot). The registry
/// (cdecl: buffer pointer, size) returns a pointer whose two words are
/// copied. The scratch pointer is skipped with a 2-word snapshot over
/// zeroed scratch; the backend call matches vf119's shape. Void.
lf_checker_rt::export!(thiscall, rw_00db7410(this_ptr: u32, kind: u32) -> u32 {
    let mut scratch = [0u32; 6];
    let (size, slot) = if kind == 0 {
        (2u32, 4usize)
    } else if kind == 2 {
        (8u32, 2usize)
    } else {
        (8u32, 0usize)
    };
    let buf = unsafe { scratch.as_mut_ptr().add(slot) as u32 };
    let found = lf_checker_rt::callee_cdecl!(1, u32, buf, size);
    unsafe {
        let lo = (found as *const u32).read_unaligned();
        let hi = ((found + 4) as *const u32).read_unaligned();
        ((this_ptr + 0x200) as *mut u32).write_unaligned(lo);
        ((this_ptr + 0x204) as *mut u32).write_unaligned(hi);
    }
    lf_checker_rt::callee_cdecl!(2, u32, 2, 0, this_ptr + 0x200, 0);
    0
});
