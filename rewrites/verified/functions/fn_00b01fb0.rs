// original: 0x00b01fb0 heap_sift_up
/// Sift-up in a binary heap of 8-byte records keyed by the f32 at offset
/// 0. `base` points at record 0, `i` is the hole, `j` the stop index, and
/// (key0, key1) is the record being inserted. Parent of hole h is
/// trunc((h-1)/2); records move down while key0 is strictly greater than
/// the parent key (NaN key compares as not-greater, matching comiss+jbe).
/// Returns key1.
export!(cdecl, rw_00b01fb0(base: *mut u8, i: i32, j: i32, key0: f32, key1: u32) -> u32 {
    fn parent_of(h: i32) -> i32 {
        h.wrapping_sub(1) / 2 // i32 division truncates toward zero, as cdq/sub/sar
    }
    let mut hole = i;
    let mut parent = parent_of(i);
    while hole > j {
        let paddr = (base as u32).wrapping_add((parent as u32).wrapping_mul(8));
        let pkey = unsafe { (paddr as *const f32).read_unaligned() };
        if !(key0 > pkey) {
            break;
        }
        let haddr = (base as u32).wrapping_add((hole as u32).wrapping_mul(8));
        unsafe {
            let w = (paddr as *const u64).read_unaligned();
            (haddr as *mut u64).write_unaligned(w);
        }
        hole = parent;
        parent = parent_of(parent);
    }
    let haddr = (base as u32).wrapping_add((hole as u32).wrapping_mul(8));
    unsafe {
        (haddr as *mut f32).write_unaligned(key0);
        ((haddr.wrapping_add(4)) as *mut u32).write_unaligned(key1);
    }
    key1
});
