// original: 0x0099d610 heap_sift_down_max
/// Sift-down step of a max-heap over 16-byte records keyed by the dword at +12.
///
/// Starting from `idx`, repeatedly moves the larger-keyed child into the hole
/// while the hole's children lie inside `count`, with the equal-count edge
/// case pulling the last record down; then hands the final hole, the original
/// index, the caller's record and the flag word to the sift-up helper
/// (cdecl/8, stubbed by the checker) and returns its answer.
export!(cdecl, rw_0099d610(arr: u32, idx: u32, count: u32, v0: u32, v1: u32, v2: u32, v3: u32, flag: u32) -> u32 {
    unsafe {
        let key = |at: u32| {
            *((arr.wrapping_add(at.wrapping_mul(16)).wrapping_add(12)) as *const u32)
        };
        let copy = |dst: u32, src: u32| {
            let d = arr.wrapping_add(dst.wrapping_mul(16)) as *mut u32;
            let s = arr.wrapping_add(src.wrapping_mul(16)) as *const u32;
            *d = *s;
            *d.add(1) = *s.add(1);
            *d.add(2) = *s.add(2);
            *d.add(3) = *s.add(3);
        };
        let mut hole = idx;
        let mut child = idx.wrapping_mul(2).wrapping_add(2);
        if child < count {
            loop {
                let mut pick = child;
                if key(child) < key(child - 1) {
                    pick = child - 1;
                }
                copy(hole, pick);
                hole = pick;
                child = pick.wrapping_mul(2).wrapping_add(2);
                if child >= count {
                    break;
                }
            }
        }
        if child == count {
            copy(hole, count - 1);
            hole = count - 1;
        }
        callee_cdecl!(1, u32, arr, hole, idx, v0, v1, v2, v3, flag)
    }
});
