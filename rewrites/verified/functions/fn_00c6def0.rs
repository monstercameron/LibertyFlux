// original: 0x00c6def0 unguarded_partition
/// Hoare-style partition of `[first, last)` around `pivot` (signed key).
/// Scans in from both ends without bounds checks and swaps out-of-place
/// pairs. Returns the split pointer: elements before it are `< pivot`.
export!(cdecl, rw_00c6def0(first: u32, last: u32, pivot: u32) -> u32 {
    unsafe {
        let mut lo = first;
        let mut hi = last;
        let p = pivot as i32;
        loop {
            while (*(lo as *const u32) as i32) < p {
                lo = lo.wrapping_add(8);
            }
            hi = hi.wrapping_sub(8);
            while p < (*(hi as *const u32) as i32) {
                hi = hi.wrapping_sub(8);
            }
            if lo >= hi {
                return lo;
            }
            let a0 = *(lo as *const u32);
            let a1 = *((lo + 4) as *const u32);
            *(lo as *mut u32) = *(hi as *const u32);
            *((lo + 4) as *mut u32) = *((hi + 4) as *const u32);
            *(hi as *mut u32) = a0;
            *((hi + 4) as *mut u32) = a1;
            lo = lo.wrapping_add(8);
        }
    }
});
