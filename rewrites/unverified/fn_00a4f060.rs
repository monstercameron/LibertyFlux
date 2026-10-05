// original: 0x00a4f060 sort_linear_insert (proposed)

/// Unguarded linear insert: shift predecessors right until one fits `val`.
///
/// `hole` points at the free slot, `val` is the element to insert. The
/// element just below the hole is read; while its key (float at +0x80) is
/// strictly greater than `val`'s key, the predecessor moves up one slot and
/// the scan continues downward. The first key that is less, equal or
/// unordered (NaN on either side ends the scan) stops it then `val` lands in
/// the final slot. The third argument is unused. Cdecl, three stack words,
/// no result.
lf_checker_rt::export!(cdecl, rw_00a4f060(hole: u32, val: u32, _extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        let want = key(val);
        let mut pred = rd(hole.wrapping_sub(4));
        let mut slot = hole;
        let mut below = hole.wrapping_sub(4);
        if key(pred) > want {
            loop {
                wr(slot, pred);
                pred = rd(below.wrapping_sub(4));
                slot = below;
                below = below.wrapping_sub(4);
                if !(key(pred) > want) {
                    break;
                }
            }
        }
        wr(slot, val);
        0
    }
});
