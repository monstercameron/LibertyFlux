// original: 0x00C224E0 point_inside_sphere_strict (proposed)
/// Return true when the squared radius at `this + 0x10` is strictly greater
/// than the squared distance between the three coordinates at `this` and the
/// supplied point. Subtractions are `this - point`; squared terms are added
/// in Y, X, then Z order to preserve the original single precision result.
/// An unordered final comparison returns false. `this` and the point pointer
/// are passed in ECX and one stack word respectively; the method cleans that
/// word and returns a zero-extended boolean in EAX.
lf_checker_rt::export!(thiscall, rw_c224e0(this: u32, point: u32) -> u32 {
    unsafe {
        const X: u32 = 0x00;
        const Y: u32 = 0x04;
        const Z: u32 = 0x08;
        const RADIUS_SQUARED: u32 = 0x10;
        let point_x = (point.wrapping_add(X) as *const u32).read_unaligned();
        let point_y = (point.wrapping_add(Y) as *const u32).read_unaligned();
        let point_z = (point.wrapping_add(Z) as *const u32).read_unaligned();
        let center_x = (this.wrapping_add(X) as *const u32).read_unaligned();
        let center_y = (this.wrapping_add(Y) as *const u32).read_unaligned();
        let center_z = (this.wrapping_add(Z) as *const u32).read_unaligned();
        let dx = core::hint::black_box(f32::from_bits(center_x)) - core::hint::black_box(f32::from_bits(point_x));
        let dy = core::hint::black_box(f32::from_bits(center_y)) - core::hint::black_box(f32::from_bits(point_y));
        let dz = core::hint::black_box(f32::from_bits(center_z)) - core::hint::black_box(f32::from_bits(point_z));
        let dy2 = core::hint::black_box(dy) * core::hint::black_box(dy);
        let dx2 = core::hint::black_box(dx) * core::hint::black_box(dx);
        let xy2 = core::hint::black_box(dy2) + core::hint::black_box(dx2);
        let dz2 = core::hint::black_box(dz) * core::hint::black_box(dz);
        let distance_squared = core::hint::black_box(xy2) + core::hint::black_box(dz2);
        let radius_bits = (this.wrapping_add(RADIUS_SQUARED) as *const u32).read_unaligned();
        let radius_squared = f32::from_bits(radius_bits);
        u32::from(radius_squared > distance_squared)
    }
});
