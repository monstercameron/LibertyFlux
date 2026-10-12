// original: 0x00A66950 sort_frag_pairs

/// Sort a nonempty range of eight-byte fragment pairs by forwarding it to the
/// introsort helper with twice the floor-log2 depth, then to the insertion
/// finish helper. An empty range returns without calls. `begin` and `end` are
/// cdecl stack arguments, and both helpers are cdecl calls whose returns are
/// ignored. The function computes the depth from the signed pair count.
lf_checker_rt::export!(cdecl, rw_00a66950(begin: u32, end: u32, context: u32) -> u32 {
    const INTROSORT_CALLEE: u32 = 1;
    const INSERTION_FINISH_CALLEE: u32 = 2;

    let signed_pair_count = (end.wrapping_sub(begin) as i32) >> 3;
    if begin == end {
        return 0;
    }

    let mut remaining = signed_pair_count;
    let mut depth = 0u32;
    while remaining != 1 {
        remaining >>= 1;
        depth = depth.wrapping_add(1);
    }
    let depth_limit = depth.wrapping_mul(2);

    let _ignored_sort_result = lf_checker_rt::callee_cdecl!(
        INTROSORT_CALLEE,
        u32,
        begin,
        end,
        0,
        depth_limit,
        context,
    );
    let _ignored_finish_result = lf_checker_rt::callee_cdecl!(
        INSERTION_FINISH_CALLEE,
        u32,
        begin,
        end,
        context,
    );
    0
});
