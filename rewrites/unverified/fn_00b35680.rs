// original: 0x00b35680 sort_run_key0_driver (proposed)

/// Sort-driver entry: forward `first`, `last` and the context word to the
/// leading-key sort routine with two trailing zero flags, and return its
/// result. Original: 0x00b35680 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00b35680(first: u32, last: u32, ctx: u32) -> u32 {
    unsafe {
        const SORT: u32 = 1;
        lf_checker_rt::callee_cdecl!(SORT, u32, first, last, ctx, 0, 0)
    }
});
