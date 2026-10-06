// original: 0x008E6BD0 introsort_entry_depth_limit (proposed)

/// Sort entry: derive the depth limit from the range, then sort.
///
/// An empty range (`lo == hi`) returns at once (the original leaves the
/// entry `eax` untouched there, so the contract compares no return value).
/// Otherwise the depth limit is twice the floor binary logarithm of the
/// element count `(hi - lo) / 8` (0 for a single element), the setup
/// routine (callee 1, cdecl) runs as `(lo, hi, 0, limit, ctx)`, and the
/// range sorter (callee 2, cdecl) runs as `(lo, hi, ctx)`.
///
/// Original: 0x008E6BD0 (cdecl, three stack arguments).
lf_checker_rt::export!(cdecl, rw_008E6BD0(lo: u32, hi: u32, ctx: u32) -> u32 {
    unsafe {
        /// Element size in bytes: the count is the byte length over 8.
        const ELEM_SHIFT: u32 = 3;
        /// Setup routine callee id.
        const SETUP: u32 = 1;
        /// Range sorter callee id.
        const SORT: u32 = 2;

        if lo == hi {
            return 0;
        }
        let mut e = ((hi.wrapping_sub(lo)) as i32) >> (ELEM_SHIFT as i32);
        let mut depth: u32 = 0;
        if e != 1 {
            loop {
                e >>= 1;
                depth = depth.wrapping_add(1);
                if e == 1 {
                    break;
                }
            }
        }
        let _: u32 =
            lf_checker_rt::callee_cdecl!(SETUP, u32, lo, hi, 0, depth.wrapping_mul(2), ctx);
        lf_checker_rt::callee_cdecl!(SORT, u32, lo, hi, ctx)
    }
});
