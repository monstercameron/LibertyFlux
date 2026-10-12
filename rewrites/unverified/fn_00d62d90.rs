// original: 0x00D62D90 axis_delta_ratio

/// Return the ratio between corresponding coordinate deltas in two points.
///
/// The first argument points to the earlier point and the second to the
/// later point. Each point stores two `f32` coordinates at byte offsets 0
/// and 4. The result is `(later.x - earlier.x) / (later.y - earlier.y)`,
/// returned through the x87 floating-point return channel. No zero-denominator
/// guard or range clamp is applied; IEEE-754 infinities and NaNs flow through.
lf_checker_rt::export!(cdecl, rw_axis_delta_ratio(earlier: u32, later: u32) -> f32 {
    unsafe {
        const X_OFFSET: u32 = 0;
        const Y_OFFSET: u32 = 4;

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { (address as *const u32).read_unaligned() })
        }

        #[inline(always)]
        fn subtract_in_source_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn divide_in_source_order(numerator: f32, denominator: f32) -> f32 {
            core::hint::black_box(numerator) / core::hint::black_box(denominator)
        }

        let later_x = read_f32(later.wrapping_add(X_OFFSET));
        let earlier_x = read_f32(earlier.wrapping_add(X_OFFSET));
        let later_y = read_f32(later.wrapping_add(Y_OFFSET));
        let earlier_y = read_f32(earlier.wrapping_add(Y_OFFSET));
        let x_delta = subtract_in_source_order(later_x, earlier_x);
        let y_delta = subtract_in_source_order(later_y, earlier_y);
        divide_in_source_order(x_delta, y_delta)
    }
});
