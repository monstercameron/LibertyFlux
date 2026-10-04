// original: 0x00e3fce0 row_pair_order_scan
// adjacent-pair scan over a 0x2b0-stride row array.
// Walks indices 1..count-1; for each adjacent pair whose marker words are
// both live and whose key words are out of order, invokes the handler with
// (prev, curr) and re-examines the previous index, else advances. Returns
// nothing meaningful (exit EAX is an internal address or entry residue).
export!(thiscall, rw_00e3fce0(this_obj: u32, base: u32, count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        const KEY_OFF: u32 = 0x10;
        const MARK_OFF: u32 = 0x14;
        const LIVE: u32 = 0xFFFF_FFFF;
        let bound = count.wrapping_sub(1) as i32;
        if bound < 1 {
            return 0;
        }
        let mut current: i32 = 1;
        let mut next: i32 = 2;
        loop {
            let elem = base.wrapping_add((current as u32).wrapping_mul(STRIDE));
            let prev = elem.wrapping_sub(STRIDE);
            let prev_mark = *((prev.wrapping_add(MARK_OFF)) as *const u32);
            let curr_mark = *((elem.wrapping_add(MARK_OFF)) as *const u32);
            let prev_key = *((prev.wrapping_add(KEY_OFF)) as *const u32);
            let curr_key = *((elem.wrapping_add(KEY_OFF)) as *const u32);
            if prev_mark != LIVE && curr_mark != LIVE && prev_key > curr_key {
                callee_thiscall!(1, u32, this_obj, prev, elem);
                current = current.wrapping_sub(1);
                if current == 0 {
                    current = 1;
                }
            } else {
                current = next;
                next = next.wrapping_add(1);
            }
            if current > bound {
                break;
            }
        }
        0
    }
});
