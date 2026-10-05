// original: 0x00AF90B0 veh_zone_find_2 (proposed)

/// Find the first live box in table 2 containing a 2D point.
///
/// Same scan as table 0 (`veh_zone_find_0`), over the `COUNT` boxes of the
/// table at `TABLE` with the same 32-byte entry layout and the same
/// ordered-comparison semantics (NaN passes an axis). Returns 1 on the first
/// containing box, 0 when none contains the point.
///
/// Original: 0x00AF90B0 (cdecl, one stack argument, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF90B0(point: u32) -> u8 {
    unsafe {
        const COUNT: u32 = 0x15FFBD0;
        const TABLE: u32 = 0x1600004;
        const ENTRY: u32 = 0x20;
        #[inline(always)]
        unsafe fn f(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        let x = f(point);
        let y = f(point + 4);
        let n = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read_unaligned();
        let mut e = lf_checker_rt::relocated(TABLE);
        for _ in 0..n {
            let inside = !(f(e.wrapping_sub(4)) > x)
                && !(x > f(e + 0x0C))
                && !(f(e) > y)
                && !(y > f(e + 0x10));
            if inside {
                return 1;
            }
            e = e.wrapping_add(ENTRY);
        }
        0
    }
});
