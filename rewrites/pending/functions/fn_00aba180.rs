// original: 0x00aba180 dual_lookup_compare
/// Compare `(c, d)` against two validated lookups over `(a, b)`.
///
/// Returns 1 when `c == -1` and `d == 0`; otherwise fetches `t1` and
/// returns 0 when `c >= t1` (signed byte compare); otherwise fetches `t2`
/// (also keyed by `c`) and returns 1 when `d < t2`, else 0. Only the low
/// byte of each callee answer is significant. The first path leaves the
/// upper return bytes undefined, so the contract compares `al`.
lf_checker_rt::export!(cdecl, rw_00aba180(a: u32, b: u32, c: u32, d: u32) -> u32 {
    if c == 0xFFFF_FFFF && d == 0 {
        return 1;
    }
    let t1 = lf_checker_rt::callee_cdecl!(1, u32, a, b) & 0xFF;
    if (c as i32) >= (t1 as i32) {
        return 0;
    }
    let t2 = lf_checker_rt::callee_cdecl!(2, u32, a, b, c) & 0xFF;
    if (d as i32) < (t2 as i32) {
        return 1;
    }
    0
});

