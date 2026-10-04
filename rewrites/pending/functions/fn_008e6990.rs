// original: 0x008e6990 siftup_heap_entry
/// Sift-up: bubble the hole from `hole` toward `top` while the parent entry
/// has a strictly greater float key, then store the caller's value there.
/// Returns the value's key word.
export!(cdecl, rw_008e6990(
    base: *mut u8,
    hole: u32,
    top: u32,
    val0: u32,
    val1: u32,
    _unused: u32,
) -> u32 {
    unsafe {
        let mut hole = hole;
        if hole > top {
            let key = f32::from_bits(val1);
            loop {
                let parent = ((hole as i32 - 1) >> 1) as u32;
                let p = entry_at(base, parent);
                if !(f32::from_bits(p.1) > key) {
                    break;
                }
                set_entry(base, hole, p);
                hole = parent;
                if hole <= top {
                    break;
                }
            }
        }
        set_entry(base, hole, (val0, val1));
        val1
    }
});
