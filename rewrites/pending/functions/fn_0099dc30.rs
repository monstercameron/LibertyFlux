// original: 0x0099dc30 pop_front_sift_down
/// Pop the first record of a heap run into the last slot, then sift down.
///
/// Copies the 16-byte record at `begin` over the one at `end - 16` and calls
/// the sift-down helper (cdecl/8, stubbed by the checker) over the shortened
/// run with the displaced last record, returning its answer. The element
/// count uses signed arithmetic exactly as the original.
export!(cdecl, rw_0099dc30(begin: u32, end: u32, flag: u32) -> u32 {
    unsafe {
        let last = end.wrapping_sub(16);
        let b = begin as *mut u32;
        let e = last as *mut u32;
        let l0 = *e;
        let l1 = *e.add(1);
        let l2 = *e.add(2);
        let l3 = *e.add(3);
        *e = *b;
        *e.add(1) = *b.add(1);
        *e.add(2) = *b.add(2);
        *e.add(3) = *b.add(3);
        let count = (end.wrapping_sub(begin).wrapping_sub(16) as i32 >> 4) as u32;
        callee_cdecl!(1, u32, begin, 0, count, l0, l1, l2, l3, flag)
    }
});
