// original: 0x00a4f130 sort_heapsort_wrap (proposed)

/// Heap sort over (first, last) with the middle argument fixed at zero.
///
/// Forwards first, last and the limit to the heap-sort callee with a zero
/// word inserted before the trailing argument. Cdecl, four stack words, one
/// callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4f130(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const SORT: u32 = 1;
        lf_checker_rt::callee_cdecl!(SORT, u32, a, b, c, 0, d);
        0
    }
});
