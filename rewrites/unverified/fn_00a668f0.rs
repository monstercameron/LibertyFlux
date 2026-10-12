// original: 0x00A668F0 heapsort_frag_pairs_default

/// Forward four caller words to the heap sort helper in the order preserved
/// by the original's pushes, inserting a zero fourth argument. The helper is
/// called with cdecl cleanup and its return value is ignored.
lf_checker_rt::export!(cdecl, rw_00a668f0(begin: u32, end: u32, context: u32, options: u32) -> u32 {
    const HEAP_SORT_CALLEE: u32 = 1;
    let _ignored = lf_checker_rt::callee_cdecl!(
        HEAP_SORT_CALLEE, (), begin, end, context, 0, options,
    );
    0
});
