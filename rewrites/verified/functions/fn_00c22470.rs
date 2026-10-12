// original: 0x00C22470 point_inside_halfspace_quad (proposed)
/// Test a point against a four-plane 2D boundary with a separate Z interval.
/// Each plane score is evaluated in the original single-precision order as
/// `x*normal_x + y*normal_y + z*0 + distance`; any ordered positive score or
/// an ordered Z bound violation returns false. An unordered comparison does
/// not take the original's `JA` branch. The method takes `this` in ECX and
/// the point pointer as one stack word, cleans that word, and returns in AL.
lf_checker_rt::export!(thiscall, rw_c22470(this: u32, point: u32) -> u32 {
    unsafe {
        const Z_MIN: u32 = 0x50;
        const Z_MAX: u32 = 0x54;
        const PLANE_BASE: u32 = 0x20;
        const PLANE_STRIDE: u32 = 8;
        const DISTANCE_BASE: u32 = 0x40;
        const ZERO: f32 = 0.0;

        let point_x = f32::from_bits((point as *const u32).read_unaligned());
        let point_y = f32::from_bits((point.wrapping_add(4) as *const u32).read_unaligned());
        let point_z = f32::from_bits((point.wrapping_add(8) as *const u32).read_unaligned());
        let min_z = f32::from_bits((this.wrapping_add(Z_MIN) as *const u32).read_unaligned());
        let max_z = f32::from_bits((this.wrapping_add(Z_MAX) as *const u32).read_unaligned());
        if min_z > point_z || point_z > max_z {
            return 0;
        }

        let mut plane = 0u32;
        while plane < 4 {
            let normal_x = f32::from_bits(
                (this.wrapping_add(PLANE_BASE + plane * PLANE_STRIDE) as *const u32)
                    .read_unaligned(),
            );
            let normal_y = f32::from_bits(
                (this.wrapping_add(PLANE_BASE + plane * PLANE_STRIDE + 4) as *const u32)
                    .read_unaligned(),
            );
            let distance = f32::from_bits(
                (this.wrapping_add(DISTANCE_BASE + plane * 4) as *const u32)
                    .read_unaligned(),
            );
            let x_term = core::hint::black_box(point_x) * core::hint::black_box(normal_x);
            let y_term = core::hint::black_box(point_y) * core::hint::black_box(normal_y);
            let z_term = core::hint::black_box(point_z) * core::hint::black_box(ZERO);
            let xy_score = core::hint::black_box(x_term) + core::hint::black_box(y_term);
            let xyz_score = core::hint::black_box(xy_score) + core::hint::black_box(z_term);
            let score = core::hint::black_box(xyz_score) + core::hint::black_box(distance);
            if score > 0.0 {
                return 0;
            }
            plane += 1;
        }
        1
    }
});
