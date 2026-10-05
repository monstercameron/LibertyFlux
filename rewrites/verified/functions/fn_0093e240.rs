// original: 0x0093e240 heap_pop_adjust (proposed)

/// Move the first element to the end slot and re-heapify from the top.
///
/// Saves the contents of the last slot (`a1 - 4`), stores `*a0` there,
/// then calls the heap callee with (`a0`, 0, count, saved, `a2`) where
/// count is `(a1 - a0 - 4) / 4` (signed shift, so -1 for an empty range).
///
/// Original: 0x0093e240 (cdecl, three stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e240(a0: u32, a1: u32, a2: u32) -> u32 {
    const HEAP_CALLEE: u32 = 1;
    unsafe {
        let first = (a0 as *const u32).read_unaligned();
        let last_slot = a1.wrapping_sub(4);
        let saved = (last_slot as *const u32).read_unaligned();
        (last_slot as *mut u32).write_unaligned(first);
        let count = ((a1.wrapping_sub(a0).wrapping_sub(4)) as i32 >> 2) as u32;
        lf_checker_rt::callee_cdecl!(HEAP_CALLEE, u32, a0, 0u32, count, saved, a2)
    }
});
