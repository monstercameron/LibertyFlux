// original: 0x00c2cc00 spheres_overlap
/// True when two spheres overlap.
///
/// Same squared-distance test as point_in_radius, but the radius is the
/// sum of both objects' radii (argument first, preserving float order).
export!(thiscall, rw_00c2cc00(this: *const f32, arg: *const f32) -> u32 {
    unsafe {
        let dx = *this.add(0) - *arg.add(0);
        let dy = *this.add(1) - *arg.add(1);
        let dz = *this.add(2) - *arg.add(2);
        let r = *arg.add(4) + *this.add(4);
        let dist2 = dy * dy + dx * dx + dz * dz;
        ((r * r) >= dist2) as u32
    }
});
