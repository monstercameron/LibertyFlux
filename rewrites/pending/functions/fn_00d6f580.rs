// original: 0x00d6f580 point_in_rect_loose
/// Test whether the point at `p` lies strictly inside the given rectangle.
///
/// Returns 1 when `x` is strictly between `x0` and `x1` and `y` strictly
/// between `y0` and `y1` (any NaN fails the test), else 0.
lf_checker_rt::export!(stdcall, rw_00d6f580(
    p: u32,
    x0_bits: u32,
    y0_bits: u32,
    x1_bits: u32,
    y1_bits: u32,
) -> u8 {
    unsafe {
        let x = f32::from_bits(*(p as *const u32));
        let y = f32::from_bits(*((p as *const u32).add(1)));
        let x0 = f32::from_bits(x0_bits);
        let y0 = f32::from_bits(y0_bits);
        let x1 = f32::from_bits(x1_bits);
        let y1 = f32::from_bits(y1_bits);
        if x > x0 && x1 > x && y > y0 && y1 > y {
            1
        } else {
            0
        }
    }
});
