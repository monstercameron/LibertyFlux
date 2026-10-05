// original: 0x00B83CF0 parse_or_format
/// Resolve `key` through callee 1, else format it through callee 2.
///
/// A scratch word starts at 2. Callee 1 tries the direct table; on
/// success the scratch is returned as is, otherwise callee 2 writes the
/// scratch through a frame pointer and the updated word is returned.
///
/// Original: 0x00B83CF0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00B83CF0(key: u32) -> u32 {
    unsafe {
        let mut scratch = 2u32;
        let r = lf_checker_rt::callee_cdecl!(1, u32, key, 0x00EB4320, 0x0B);
        if r != 0 {
            return scratch;
        }
        lf_checker_rt::callee_cdecl!(2, u32, &mut scratch as *mut u32 as u32, 0x00EB432C, key);
        scratch
    }
});
