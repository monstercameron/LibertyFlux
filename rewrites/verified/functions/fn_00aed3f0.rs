// original: 0x00AED3F0 kv_pop_last_to_first (proposed)

/// Move the first pair over the last entry and notify the sort callee.
///
/// `first` and `last` delimit 8-byte entries. The pair at `first` is copied
/// over the entry ending at `last` (`[last-8]`, `[last-4]`), then the
/// notify callee is called with (`first`, 0, count, old key, old value,
/// `ctx`) where count is `(last - first - 8) / 8` (arithmetic shift, so an
/// empty range passes -1). Returns the callee's result.
///
/// Original: 0x00AED3F0 (cdecl, three stack words, one direct callee).
lf_checker_rt::export!(cdecl, rw_00aed3f0(first: u32, last: u32, ctx: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 8;
        const NOTIFY_CALLEE: u32 = 1;
        let new_key = ((first) as *const u32).read_unaligned();
        let new_val = ((first.wrapping_add(4)) as *const u32).read_unaligned();
        let slot = last.wrapping_sub(ENTRY);
        let old_key = ((slot) as *const u32).read_unaligned();
        let old_val = ((slot.wrapping_add(4)) as *const u32).read_unaligned();
        ((slot) as *mut u32).write_unaligned(new_key);
        ((slot.wrapping_add(4)) as *mut u32).write_unaligned(new_val);
        let count = (last.wrapping_sub(first).wrapping_sub(ENTRY) as i32 >> 3) as u32;
        lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, first, 0, count, old_key, old_val, ctx)
    }
});
