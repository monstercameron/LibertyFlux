// original: 0x00b2d8a0 garage_point_in_range
// 0xB2D8A0 garage_point_in_range (thiscall/1 -> al).
//
// Range test of a point against a record's vertical span and two projected
// extents. Every early exit is a strict `>` comparison, so NaN inputs fall
// through to the final test, which answers "not greater" (true on NaN).
export!(thiscall, rw_00b2d8a0(rec: *const f32, pt: *const f32) -> u8 {
    unsafe {
        let low = *rec.add(2);
        let py = *pt.add(2);
        if low > py {
            return 0;
        }
        if py > *rec.add(7) {
            return 0;
        }
        let dx = *pt.add(1) - *rec.add(1);
        let dz = *pt.add(0) - *rec.add(0);
        let t0 = *rec.add(4) * dx;
        let t1 = *rec.add(3) * dz;
        let d = t1 + t0;
        if 0.0 > d {
            return 0;
        }
        if d > *rec.add(8) {
            return 0;
        }
        let u0 = *rec.add(6) * dx;
        let u1 = *rec.add(5) * dz;
        let e = u1 + u0;
        if 0.0 > e {
            return 0;
        }
        if e > *rec.add(9) {
            return 0;
        }
        1
    }
});
