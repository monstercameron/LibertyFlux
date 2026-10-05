// original: 0x00a4ed40 sort_insertion_dispatch (proposed)

/// Insertion sort, splitting ranges over sixteen elements in two.
///
/// Ranges of sixteen elements or fewer (signed byte-span compare) go to the
/// unguarded insertion callee whole; longer ranges sort their first sixteen
/// elements unguarded and the remainder through the guarded insertion
/// callee, both receiving a zero word and `extra`. Cdecl, three stack
/// words, two callees, no result.
lf_checker_rt::export!(cdecl, rw_00a4ed40(first: u32, last: u32, extra: u32) -> u32 {
    unsafe {
        const UNGUARDED: u32 = 1;
        const GUARDED: u32 = 2;
        const SPLIT_SPAN: u32 = 0x40;
        let span = (last.wrapping_sub(first)) & !3;
        if (span as i32) <= (SPLIT_SPAN as i32) {
            lf_checker_rt::callee_cdecl!(UNGUARDED, u32, first, last, 0, extra);
        } else {
            let mid = first.wrapping_add(SPLIT_SPAN);
            lf_checker_rt::callee_cdecl!(UNGUARDED, u32, first, mid, 0, extra);
            lf_checker_rt::callee_cdecl!(GUARDED, u32, mid, last, 0, extra);
        }
        0
    }
});
