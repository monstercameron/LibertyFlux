// original: 0x0093e1b0 ptr_partition (proposed)

/// Partition the pointer range [`lo`, `hi`) around `pivot`'s key.
///
/// Hoare-style: the left cursor advances past keys below the pivot key,
/// the right cursor retreats (at least one slot) past keys above it; when
/// the cursors have not crossed, the two slots are swapped and the left
/// cursor advances. Returns the split point. Keys are compared unsigned.
///
/// Original: 0x0093e1b0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_0093e1b0(lo: u32, hi: u32, pivot: u32) -> u32 {
    unsafe {
        let pivot_key = (pivot as *const u32).read_unaligned();
        let mut left = lo;
        let mut right = hi;
        loop {
            while (((left as *const u32).read_unaligned() as *const u32).read_unaligned()) < pivot_key {
                left = left.wrapping_add(4);
            }
            loop {
                right = right.wrapping_sub(4);
                let e = (right as *const u32).read_unaligned();
                if pivot_key >= (e as *const u32).read_unaligned() {
                    break;
                }
            }
            if left >= right {
                return left;
            }
            let a = (left as *const u32).read_unaligned();
            let b = (right as *const u32).read_unaligned();
            (left as *mut u32).write_unaligned(b);
            (right as *mut u32).write_unaligned(a);
            left = left.wrapping_add(4);
        }
    }
});
