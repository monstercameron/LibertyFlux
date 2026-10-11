// original: 0x00a25960 camera_distance_interpolate_short_range

/// Forward two input words to the distance interpolator with the fixed binary32 bounds 0x3f800000 and 0x41700000, a null output pointer, and return its ST0 result unchanged.
lf_checker_rt::export!(stdcall, rw_00a25960(first: u32, second: u32) -> f32 {
    lf_checker_rt::callee_stdcall!(1, f32, first, second, 0x3f800000, 0x41700000, 0)
});
