// original: 0x00b05570 rotate_last_pair_and_dispatch

/// Copies the first record over the last slot, then dispatches the worker.
///
/// `base` points at the first 8-byte record and `past_end` one past the last.
/// The first record's two words are stored into the last slot; the worker is
/// then called with the base, a zero word, the record count (an arithmetic
/// shift of the byte span), the overwritten last-slot words, and `tag`.
export!(cdecl, rw_00b05570(base: *mut u32, past_end: *mut u32, tag: u32) -> u32 {
    unsafe {
        let first = *base;
        let second = *base.add(1);
        let old_last0 = *past_end.sub(2);
        let old_last1 = *past_end.sub(1);
        *past_end.sub(2) = first;
        *past_end.sub(1) = second;
        let span = (past_end as u32).wrapping_sub(base as u32).wrapping_sub(8);
        let count = ((span as i32) >> 3) as u32;
        callee_cdecl!(1, u32, base as u32, 0, count, old_last0, old_last1, tag)
    }
});
