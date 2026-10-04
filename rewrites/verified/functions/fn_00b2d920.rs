// original: 0x00b2d920 garage_point_in_range_r
// 0xB2D920 garage_point_in_range_r (thiscall/2 -> al).
//
// Same range test with a tolerance radius widening every span. The negative
// radius is an exact sign flip, matching the original's xor with -0.0.
export!(thiscall, rw_00b2d920(rec: *const f32, pt: *const f32, r: f32) -> u8 {
    unsafe {
        let py = *pt.add(2);
        if *rec.add(2) - r > py {
            return 0;
        }
        if py > *rec.add(7) + r {
            return 0;
        }
        let dx = *pt.add(1) - *rec.add(1);
        let dz = *pt.add(0) - *rec.add(0);
        let neg = -r;
        let t0 = *rec.add(4) * dx;
        let t1 = *rec.add(3) * dz;
        let d = t1 + t0;
        if neg > d {
            return 0;
        }
        if d > *rec.add(8) + r {
            return 0;
        }
        let u0 = *rec.add(6) * dx;
        let u1 = *rec.add(5) * dz;
        let e = u1 + u0;
        if neg > e {
            return 0;
        }
        if e > *rec.add(9) + r {
            return 0;
        }
        1
    }
});
