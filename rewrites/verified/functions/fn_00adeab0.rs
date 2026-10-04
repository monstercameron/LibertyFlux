// original: 0x00adeab0 unguarded_insert (proposed)

/// Insert a value into a sorted run without a lower-bound check.
///
/// `pos` points just past the run's end, `val` is the value, `comp` a
/// `cdecl(a, b) -> bool` comparator. While `comp(val, previous)`
/// holds, the previous word moves up one slot and the position steps
/// down; the value is then stored at the final position. The caller
/// guarantees a stopping element below, so no bound is tested.
///
/// Edge cases: none in this function; at least one comparison always
/// runs, and the value is always stored exactly once.
///
/// Original: cdecl, three stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments), no direct callees. Returns nothing.
lf_checker_rt::export!(cdecl, rw_00adeab0(pos: u32, val: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        let mut p = pos;
        loop {
            let prev = rd(p.wrapping_sub(4));
            if cmp(val, prev) == 0 {
                break;
            }
            wr(p, prev);
            p = p.wrapping_sub(4);
        }
        wr(p, val);
    }
    0
});
