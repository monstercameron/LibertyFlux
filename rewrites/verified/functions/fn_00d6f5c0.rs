// original: 0x00d6f5c0 replay_bar_hit_test
/// Test whether the point at `p` lies strictly inside the bar rectangle.
///
/// Uses bounds +0xB0/+0xBC for `x` and +0xAC/+0xB4 for `y`. Returns 1 on a
/// strict inside hit (any NaN fails), else 0.
lf_checker_rt::export!(thiscall, rw_00d6f5c0(this_ptr: u32, p: u32) -> u8 {
    unsafe {
        let b = this_ptr as *const u8;
        let x = f32::from_bits(*(p as *const u32));
        let y = f32::from_bits(*((p as *const u32).add(1)));
        let x0 = f32::from_bits(*((b.add(0xb0)) as *const u32));
        let x1 = f32::from_bits(*((b.add(0xbc)) as *const u32));
        let y0 = f32::from_bits(*((b.add(0xac)) as *const u32));
        let y1 = f32::from_bits(*((b.add(0xb4)) as *const u32));
        if x > x0 && x1 > x && y > y0 && y1 > y {
            1
        } else {
            0
        }
    }
});
