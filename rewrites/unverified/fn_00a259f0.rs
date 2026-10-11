// original: 0x00a259f0 camera_distance_interpolate_default_range

/// Forward two input words to the distance interpolator with the fixed binary32 bounds 0x40000000 and 0x40a00000, a null output pointer, and return its ST0 result unchanged.
lf_checker_rt::export!(stdcall, rw_00a259f0(first: u32, second: u32) -> f32 {
    lf_checker_rt::callee_stdcall!(1, f32, first, second, 0x40000000, 0x40a00000, 0)
});
