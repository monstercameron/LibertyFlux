// original: 0x00a4f110 sort_make_heap_wrap (proposed)

/// Make-heap over (first, last) with default trailing arguments.
///
/// Forwards its three arguments to the make-heap callee with two zero words
/// appended. Cdecl, three stack words, one callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4f110(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const MAKE: u32 = 1;
        lf_checker_rt::callee_cdecl!(MAKE, u32, a, b, c, 0, 0);
        0
    }
});
