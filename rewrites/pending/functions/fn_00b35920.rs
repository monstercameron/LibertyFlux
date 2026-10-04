// original: 0x00b35920 visit_range28_top_down
/// While more than one 28-byte record remains in `[a, end)`, call the
/// worker with `(a, end, c)` and shrink `end` by one record from the top.
lf_checker_rt::export!(cdecl, rw_b35920(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let mut d = b.wrapping_sub(a);
        let mut e = b;
        let mut n = ((d as i32) / 28);
        while n > 1 {
            lf_checker_rt::callee_cdecl!(2, u32, a, e, c);
            d = d.wrapping_sub(28);
            n = ((d as i32) / 28);
            e = e.wrapping_sub(28);
        }
        n as u32
    }
});
