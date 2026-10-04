// original: 0x00d6f610 replay_bar_region_hit_test
/// Test whether the point at `p` lies inside the region selected by `mode`.
///
/// Mode 0 uses bounds +0xA8/+0xB0 for `x` and +0xAC/+0xB4 for `y`; mode 1
/// uses +0xBC/+0xC4 and +0xC0/+0xC8. Any other mode, any NaN, or a point
/// outside the strict bounds returns 0; a strict inside hit returns 1.
lf_checker_rt::export!(thiscall, rw_00d6f610(this_ptr: u32, p: u32, mode: u32) -> u8 {
    unsafe {
        let b = this_ptr as *const u8;
        let (x0o, x1o, y0o, y1o) = if mode == 0 {
            (0xa8usize, 0xb0usize, 0xacusize, 0xb4usize)
        } else if mode == 1 {
            (0xbcusize, 0xc4usize, 0xc0usize, 0xc8usize)
        } else {
            return 0;
        };
        let x = f32::from_bits(*(p as *const u32));
        let y = f32::from_bits(*((p as *const u32).add(1)));
        let x0 = f32::from_bits(*((b.add(x0o)) as *const u32));
        let x1 = f32::from_bits(*((b.add(x1o)) as *const u32));
        let y0 = f32::from_bits(*((b.add(y0o)) as *const u32));
        let y1 = f32::from_bits(*((b.add(y1o)) as *const u32));
        if x > x0 && x1 > x && y > y0 && y1 > y {
            1
        } else {
            0
        }
    }
});
