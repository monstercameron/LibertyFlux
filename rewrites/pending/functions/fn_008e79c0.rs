// original: 0x008e79c0 project_point_onto_segment
/// Project point `p` onto the segment `a + t * (b - a)` (endpoints read from
/// `this`), clamp `t` into `[0, 1]`, store the closest point into `out`, and
/// return `t`.
export!(thiscall, rw_008e79c0(this: *const u8, p: *const u8, out: *mut u8) -> f32 {
    unsafe {
        let f = |base: *const u8, off: usize| {
            f32::from_bits(read_unaligned(base.add(off) as *const u32))
        };
        let ax = f(this, 0);
        let ay = f(this, 4);
        let az = f(this, 8);
        let abx = f(this, 0x10) - ax;
        let aby = f(this, 0x14) - ay;
        let abz = f(this, 0x18) - az;
        let apx = f(p, 0) - ax;
        let apy = f(p, 4) - ay;
        let apz = f(p, 8) - az;
        // Dot products in the original's exact operation order.
        let mut num = apy * aby;
        num += apx * abx;
        num += apz * abz;
        let mut den = aby * aby;
        den += abx * abx;
        den += abz * abz;
        let mut t = num / den;
        if t < 0.0 {
            t = 0.0;
        } else if t > 1.0 {
            t = 1.0;
        }
        write_unaligned(out as *mut u32, (abx * t + ax).to_bits());
        write_unaligned(out.add(4) as *mut u32, (aby * t + ay).to_bits());
        write_unaligned(out.add(8) as *mut u32, (abz * t + az).to_bits());
        t
    }
});
