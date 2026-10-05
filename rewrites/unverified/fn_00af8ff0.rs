// original: 0x00AF8FF0 veh_zone_find_0 (proposed)

/// Find the first live box in table 0 containing a 2D point.
///
/// Scans the `COUNT` boxes of the table at `TABLE`, each 32 bytes holding
/// `(lo_x, lo_y, hi_x, hi_y)` at offsets `(-4, 0, +0xC, +0x10)` from the
/// entry cursor. A coordinate fails only by an ordered strict comparison, so
/// a NaN box bound or point coordinate passes that axis, matching the
/// original's above-compare jumps. Returns 1 on the first containing box,
/// 0 when none contains the point.
///
/// Original: 0x00AF8FF0 (cdecl, one stack argument, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF8FF0(point: u32) -> u8 {
    unsafe {
        const COUNT: u32 = 0x15FFBC4;
        const TABLE: u32 = 0x15FFC44;
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
