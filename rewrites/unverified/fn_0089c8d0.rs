// original: 0x0089c8d0 fetch_word_or_missing
/// Forward two words to a six-word helper and return its out-word.
///
/// Calls the helper with the two incoming words first, then a pointer to a
/// fresh out-slot, then two zero words and one scratch word. The helper
/// stores its answer through the out-slot pointer; this function returns
/// that word. The helper's own return value is ignored.
export!(cdecl, rw_0089c8d0(first: u32, second: u32) -> u32 {
    unsafe {
        let mut out: u32 = 0;
        callee_cdecl!(1, u32, first, second, &mut out as *mut u32 as u32, 0, 0, 0);
        out
    }
});
