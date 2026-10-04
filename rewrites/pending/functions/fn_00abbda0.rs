// original: 0x00abbda0 range_split_call
/// Dispatch a dword range `[a, b)` to worker callees, splitting at `a+0x40`.
///
/// The extent is `(b - a) & !3` compared as a signed value. Ranges longer
/// than 0x40 go through the split path: the head `[a, a+0x40)` is handled
/// by callee 1 and the tail `[a+0x40, b)` by callee 2 (both take the
/// context word as their last argument). Shorter ranges go to callee 1
/// whole. Returns the
/// last callee answer.
lf_checker_rt::export!(cdecl, rw_00abbda0(a: u32, b: u32, c: u32) -> u32 {
    let extent = b.wrapping_sub(a) & 0xFFFF_FFFC;
    if (extent as i32) > 0x40 {
        let mid = a.wrapping_add(0x40);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, a, mid, 0, c);
        lf_checker_rt::callee_cdecl!(2, u32, mid, b, 0, c)
    } else {
        lf_checker_rt::callee_cdecl!(1, u32, a, b, 0, c)
    }
});

