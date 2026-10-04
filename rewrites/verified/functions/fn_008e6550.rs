// original: 0x008e6550 siftdown_then_push_heap
/// Sift-down over 8-byte entries ordered by the float key at +4: move the
/// larger-keyed child into the hole while the hole index is below `count`,
/// take the single-child step when exactly one child remains, then hand the
/// final hole plus the caller's value to the sift-up helper (cdecl/6,
/// stubbed). Returns the helper's answer.
export!(cdecl, rw_008e6550(
    base: *mut u8,
    hole: u32,
    count: u32,
    _ignored: u32,
    val0: u32,
    val1: u32,
) -> u32 {
    unsafe {
        let first = hole;
        let mut hole = hole;
        let mut child = hole.wrapping_mul(2).wrapping_add(2);
        if child < count {
            loop {
                let a = entry_at(base, child - 1);
                let b = entry_at(base, child);
                // jbe order: keep the right child unless it is strictly greater.
                if f32::from_bits(b.1) > f32::from_bits(a.1) {
                    child -= 1;
                }
                set_entry(base, hole, entry_at(base, child));
                hole = child;
                child = child.wrapping_mul(2).wrapping_add(2);
                if child >= count {
                    break;
                }
            }
        }
        if child == count {
            set_entry(base, hole, entry_at(base, count - 1));
            hole = count - 1;
        }
        callee_cdecl!(1, u32, base as u32, hole, first, _ignored, val0, val1)
    }
});
