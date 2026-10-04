// original: 0x00b01e10 heap_sift_down
/// Sift-down in a binary max-heap of 8-byte records keyed by the f32 at
/// offset 0, then hand the hole to the sift-up routine. `base` points at
/// record 0, `i` is the hole, `j` the heap size; (c, d, e) pass through.
/// The child choice uses comiss+jbe, i.e. decrement unless strictly
/// greater (NaN counts as not-greater). Returns the sift-up result.
export!(cdecl, rw_00b01e10(base: u32, i: i32, j: i32, c: u32, d: u32, e: u32) -> u32 {
    fn at(base: u32, idx: i32) -> u32 {
        base.wrapping_add((idx as u32).wrapping_mul(8))
    }
    let mut hole = i;
    let mut child = i.wrapping_mul(2).wrapping_add(2);
    if child < j {
        loop {
            let a = unsafe { (at(base, child).wrapping_sub(8) as *const f32).read_unaligned() };
            let b = unsafe { (at(base, child) as *const f32).read_unaligned() };
            if a > b {
                child = child.wrapping_sub(1);
            }
            let w = unsafe { (at(base, child) as *const u64).read_unaligned() };
            unsafe { (at(base, hole) as *mut u64).write_unaligned(w) };
            hole = child;
            child = child.wrapping_mul(2).wrapping_add(2);
            if !(child < j) {
                break;
            }
        }
    }
    if child == j {
        let w = unsafe { (at(base, child).wrapping_sub(8) as *const u64).read_unaligned() };
        unsafe { (at(base, hole) as *mut u64).write_unaligned(w) };
        hole = child.wrapping_sub(1);
    }
    callee_cdecl!(1, u32, base, hole as u32, i as u32, c, d, e)
});
