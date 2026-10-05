// original: 0x0093e100 heap_sift_up (proposed)

/// Percolate `val` up the pointer heap from `hole` toward `bottom`.
///
/// While `hole` is above `bottom` (both signed), compares `val`'s key
/// (first object word, unsigned) with the key at parent `(hole - 1) / 2`:
/// a parent not below `val` ends the walk, otherwise the parent moves down
/// and the hole rises. Stores `val` in the final hole. When `hole` starts
/// at or below `bottom` stores immediately. Returns `base` on the store
/// path, else the last parent index probed.
///
/// Original: 0x0093e100 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_0093e100(base: u32, hole: u32, bottom: u32, val: u32) -> u32 {
    unsafe {
        if (hole as i32) <= (bottom as i32) {
            (base.wrapping_add(hole.wrapping_mul(4)) as *mut u32).write_unaligned(val);
            return base;
        }
        let val_key = (val as *const u32).read_unaligned();
        let mut h = hole;
        let mut parent = (((h as i32).wrapping_sub(1)) / 2) as u32;
        loop {
            let cand = (base.wrapping_add(parent.wrapping_mul(4)) as *const u32).read_unaligned();
            if ((cand as *const u32).read_unaligned()) >= val_key {
                (base.wrapping_add(h.wrapping_mul(4)) as *mut u32).write_unaligned(val);
                return parent;
            }
            (base.wrapping_add(h.wrapping_mul(4)) as *mut u32).write_unaligned(cand);
            h = parent;
            parent = (((h as i32).wrapping_sub(1)) / 2) as u32;
            if (h as i32) <= (bottom as i32) {
                (base.wrapping_add(h.wrapping_mul(4)) as *mut u32).write_unaligned(val);
                return parent;
            }
        }
    }
});
