// original: 0x00b35720 call_with_record28_by_value
/// Call the worker with `(a0, p, p, w0..w6, a2, 0)` where `p = a1 - 28` and
/// `w0..w6` are the seven words at `p` (a 28-byte record passed by value).
lf_checker_rt::export!(cdecl, rw_b35720(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let p = a1.wrapping_sub(0x1c);
        let w = p as *const u32;
        lf_checker_rt::callee_cdecl!(2, u32, a0, p, p,
            w.read(), w.add(1).read(), w.add(2).read(), w.add(3).read(),
            w.add(4).read(), w.add(5).read(), w.add(6).read(), a2, 0)
    }
});
