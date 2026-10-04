// original: 0x00b35810 dispatch_range28_by_log2
/// If `a == b` do nothing; else with `n = (b - a) / 28` call worker1 with
/// `(a, b, 0, 2 * floor(log2(n)), c)` then worker2 with `(a, b, c)`.
/// Returns nothing meaningful (the original leaves entry garbage or a
/// callee answer in eax); the contract compares the calls, not eax.
lf_checker_rt::export!(cdecl, rw_b35810(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        if a == b {
            return 0;
        }
        let n = ((b.wrapping_sub(a) as i32) / 28) as u32;
        let k = 2 * (31 - n.leading_zeros());
        lf_checker_rt::callee_cdecl!(2, u32, a, b, 0, k, c);
        lf_checker_rt::callee_cdecl!(3, u32, a, b, c)
    }
});
