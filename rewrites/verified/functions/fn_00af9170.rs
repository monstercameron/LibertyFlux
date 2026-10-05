// original: 0x00AF9170 veh_point_in_box (proposed)

/// Test whether a point lies inside an integer bounding box.
///
/// `point` holds three floats (x, y, z); `bounds` holds six signed words at
/// `+0x10` through `+0x1A` as (lo_x, lo_y, lo_z, hi_x, hi_y, hi_z), each
/// converted exactly to float. Returns 1 only when every coordinate lies
/// within its closed interval with all comparisons ordered: a NaN coordinate
/// fails, matching the original's unordered-compare branch behaviour.
///
/// Original: 0x00AF9170 (cdecl, two stack arguments, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF9170(point: u32, bounds: u32) -> u8 {
    unsafe {
        #[inline(always)]
        unsafe fn coord(p: u32, i: u32) -> f32 {
            unsafe { f32::from_bits(((p + i * 4) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn bound(b: u32, i: u32) -> f32 {
            unsafe { ((b + 0x10 + i * 2) as *const i16).read_unaligned() as f32 }
        }
        let (x, y, z) = (coord(point, 0), coord(point, 1), coord(point, 2));
        // Each guard is written as !(a >= b) so NaN takes the failing side,
        // exactly like the original's `comiss` + `jb`.
        if !(x >= bound(bounds, 0)) {
            return 0;
        }
        if !(bound(bounds, 3) >= x) {
            return 0;
        }
        if !(y >= bound(bounds, 1)) {
            return 0;
        }
        if !(bound(bounds, 4) >= y) {
            return 0;
        }
        if !(z >= bound(bounds, 2)) {
            return 0;
        }
        if !(bound(bounds, 5) >= z) {
            return 0;
        }
        1
    }
});
