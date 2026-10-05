// original: 0x00a4f030 sort_insertion_guarded (proposed)

/// Insertion sort over [first, last) through the linear-insert callee.
///
/// Each slot's element is handed to the linear-insert callee together with
/// the slot itself and `extra`; empty ranges do nothing. The third argument
/// is unused. Cdecl, four stack words, one callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4f030(first: u32, last: u32, _u: u32, extra: u32) -> u32 {
    unsafe {
        const INSERT: u32 = 1;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        if first == last {
            return 0;
        }
        let mut cur = first;
        loop {
            lf_checker_rt::callee_cdecl!(INSERT, u32, cur, rd(cur), extra);
            cur = cur.wrapping_add(4);
            if cur == last {
                break;
            }
        }
        0
    }
});
