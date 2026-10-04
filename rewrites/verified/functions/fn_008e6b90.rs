// original: 0x008e6b90 stash_last_then_siftdown
/// Copy the first entry over the last, then call the sift-down helper
/// (cdecl/6, stubbed) with the saved last entry and the element count.
/// Returns the helper's answer.
export!(cdecl, rw_008e6b90(begin: *mut u8, end: *mut u8, extra: u32) -> u32 {
    unsafe {
        let first = entry_at(begin, 0);
        let lastp = end.sub(8);
        let last = entry_at(lastp, 0);
        set_entry(lastp, 0, first);
        let count =
            (((end as u32).wrapping_sub(begin as u32).wrapping_sub(8) as i32)
                >> 3) as u32;
        callee_cdecl!(1, u32, begin as u32, 0, count, last.0, last.1, extra)
    }
});
