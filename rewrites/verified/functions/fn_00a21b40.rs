// original: 0x00a21b40 spatial_records_distance_exceeds_radius_sum

/// Compare two three-dimensional records. Subtract the second position from
/// the first, sum squared deltas in the original Y, X, Z order, and compare
/// that sum with the square of the two radii added together. Unordered COMISS
/// results return zero.
lf_checker_rt::export!(thiscall, rw_00a21b40(this: u32, other: u32) -> u32 {
    const POSITION_X: u32 = 0x430;
    const POSITION_Y: u32 = 0x434;
    const POSITION_Z: u32 = 0x438;
    const RADIUS: u32 = 0x490;
    unsafe {
        let dx = core::hint::black_box((this.wrapping_add(POSITION_X) as *const f32).read_unaligned())
            - core::hint::black_box((other.wrapping_add(0x30) as *const f32).read_unaligned());
        let dy = core::hint::black_box((this.wrapping_add(POSITION_Y) as *const f32).read_unaligned())
            - core::hint::black_box((other.wrapping_add(0x34) as *const f32).read_unaligned());
        let dz = core::hint::black_box((this.wrapping_add(POSITION_Z) as *const f32).read_unaligned())
            - core::hint::black_box((other.wrapping_add(0x38) as *const f32).read_unaligned());
        let combined_radius = core::hint::black_box((this.wrapping_add(RADIUS) as *const f32).read_unaligned())
            + core::hint::black_box((other.wrapping_add(8) as *const f32).read_unaligned());
        let y_squared = core::hint::black_box(dy) * core::hint::black_box(dy);
        let x_squared = core::hint::black_box(dx) * core::hint::black_box(dx);
        let z_squared = core::hint::black_box(dz) * core::hint::black_box(dz);
        let partial_distance = core::hint::black_box(y_squared) + core::hint::black_box(x_squared);
        let distance_squared = core::hint::black_box(partial_distance) + core::hint::black_box(z_squared);
        let radius_squared = core::hint::black_box(combined_radius) * core::hint::black_box(combined_radius);
        u32::from(distance_squared > radius_squared)
    }
});
