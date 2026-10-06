// original: 0x00DB7470 UIFontString::vf117

/// Resolve the shared text handle for the given key through the registry
/// (cdecl: scratch buffer, key), store its two words at `+0x200`/`+0x204`,
/// and notify the backend (cdecl: 2, 0, pointer, 0).
///
/// `thiscall`, one stack word. The scratch pointer is skipped with a 2-word
/// snapshot over zeroed scratch; the backend pointer likewise with four
/// words. Void.
lf_checker_rt::export!(thiscall, rw_00db7470(this_ptr: u32, key: u32) -> u32 {
    let mut scratch = [0u32; 2];
    let buf = scratch.as_mut_ptr() as u32;
    let found = lf_checker_rt::callee_cdecl!(1, u32, buf, key);
    unsafe {
        let lo = (found as *const u32).read_unaligned();
        let hi = ((found + 4) as *const u32).read_unaligned();
        ((this_ptr + 0x200) as *mut u32).write_unaligned(lo);
        ((this_ptr + 0x204) as *mut u32).write_unaligned(hi);
    }
    lf_checker_rt::callee_cdecl!(2, u32, 2, 0, this_ptr + 0x200, 0);
    0
});
