// original: 0x008C6C30 table_lookup_or_default
/// Look up a streaming table row, reporting presence through a flag.
///
/// Searches the table rooted at `this` (base at `+0`, bound at `+4`) for
/// `key` via the search callee. When a row is found its first word is
/// returned and `present` is set to 1, otherwise `present` is set to 0 and
/// the result is 0. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_008c6c30(this: u32, key: u32,
                                              present: u32) -> u32 {
    unsafe {
        const SEARCH_CALLEE: u32 = 1;
        let base = (this as *const u32).read_unaligned();
        let bound = ((this + 4) as *const u32).read_unaligned();
        let row: u32 = lf_checker_rt::callee_stdcall!(
            SEARCH_CALLEE, u32, key, base, 0, bound.wrapping_sub(1));
        if row == 0 {
            (present as *mut u8).write(0);
            0
        } else {
            (present as *mut u8).write(1);
            (row as *const u32).read_unaligned()
        }
    }
});
