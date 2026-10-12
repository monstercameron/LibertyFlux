// original: 0x00A668D0 make_heap_frag_pairs_default

/// Forward the first three caller words to the heap builder, followed by two
/// zero options. The target is called with the cdecl convention. Its return
/// value is ignored, and this wrapper returns no meaningful value.
lf_checker_rt::export!(cdecl, rw_00a668d0(begin: u32, end: u32, context: u32) -> u32 {
    const MAKE_HEAP_CALLEE: u32 = 1;
    let _ignored = lf_checker_rt::callee_cdecl!(
        MAKE_HEAP_CALLEE, (), begin, end, context, 0, 0,
    );
    0
});
