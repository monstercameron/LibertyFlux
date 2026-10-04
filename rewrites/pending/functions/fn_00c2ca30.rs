// original: 0x00c2ca30 point_in_radius
/// True when the point is within radius of the centre.
///
/// Squared-distance test in float: (r*r) >= (dx*dx + dy*dy + dz*dz) with
/// the exact accumulation order of the original, so boundary and NaN
/// cases agree bit for bit.
export!(thiscall, rw_00c2ca30(this: *const f32, arg: *const f32) -> u32 {
    unsafe {
        let dx = *this.add(0) - *arg.add(0);
        let dy = *this.add(1) - *arg.add(1);
        let dz = *this.add(2) - *arg.add(2);
        let r = *this.add(4);
        let dist2 = dy * dy + dx * dx + dz * dz;
        ((r * r) >= dist2) as u32
    }
});
