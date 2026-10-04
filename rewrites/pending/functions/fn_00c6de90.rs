// original: 0x00c6de90 unguarded_insertion_sort
/// Insertion-sort `[first, last)` by inserting each element into the already
/// sorted prefix through the single-element helper. The third argument is
/// unused, the fourth is forwarded untouched.
export!(cdecl, rw_00c6de90(first: u32, last: u32, _u: u32, extra: u32) -> () {
    unsafe {
        let mut p = first;
        while p != last {
            let k = *(p as *const u32);
            let v = *((p + 4) as *const u32);
            callee_cdecl!(1, u32, p, k, v, extra);
            p = p.wrapping_add(8);
        }
    }
});
