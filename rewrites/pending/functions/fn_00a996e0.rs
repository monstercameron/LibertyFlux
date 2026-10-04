// original: 0x00a996e0 gated_plane_proximity_test
/// Tests whether point `a` lies within 0.1 of the plane through `b`, `c`,
/// `d`, behind three orientation gates.
///
/// Each argument points to three consecutive `f32` values (x, y, z). The
/// test first requires three scalar triple products to be non-negative: the
/// edges from `c`, then from `b`, then the pair (`c` - `b`, `d` - `b`) must
/// all have the same handedness relative to the matching edge toward `a`.
/// Any negative (or NaN) triple product returns 0 immediately. When all
/// three gates pass, the cross product of (`c` - `b`) and (`d` - `b`) is
/// scaled by the reciprocal of its own length (a zero-length cross product
/// scales by zero instead of dividing), dotted with (`a` - `b`), and the
/// function returns 1 when the absolute value of that distance is below 0.1.
///
/// NaN inputs fail closed: every gate treats NaN like a negative result and
/// the final comparison returns 0 for NaN.
export!(cdecl, rw_a996e0(p0: u32, p1: u32, p2: u32, p3: u32) -> u8 {
    unsafe {
        let ax: f32 = (p0 as *const f32).read_unaligned();
        let ay: f32 = (p0 as *const f32).add(1).read_unaligned();
        let az: f32 = (p0 as *const f32).add(2).read_unaligned();
        let bx: f32 = (p1 as *const f32).read_unaligned();
        let by: f32 = (p1 as *const f32).add(1).read_unaligned();
        let bz: f32 = (p1 as *const f32).add(2).read_unaligned();
        let cx: f32 = (p2 as *const f32).read_unaligned();
        let cy: f32 = (p2 as *const f32).add(1).read_unaligned();
        let cz: f32 = (p2 as *const f32).add(2).read_unaligned();
        let dx: f32 = (p3 as *const f32).read_unaligned();
        let dy: f32 = (p3 as *const f32).add(1).read_unaligned();
        let dz: f32 = (p3 as *const f32).add(2).read_unaligned();

        // Gate 1: edges out of c must agree in handedness.
        let ex = dx - cx;
        let ey = dy - cy;
        let ez = dz - cz;
        let e1x = ax - cx;
        let e1y = ay - cy;
        let e1z = az - cz;
        let n1x = ey * e1z - ez * e1y;
        let n1y = ez * e1x - ex * e1z;
        let n1z = ex * e1y - ey * e1x;
        let fx = bx - cx;
        let fy = by - cy;
        let fz = bz - cz;
        let mut u = ey * fz - ez * fy;
        u *= n1x;
        let mut v = ez * fx - ex * fz;
        v *= n1y;
        let mut w = ex * fy - ey * fx;
        w *= n1z;
        let t1 = (u + v) + w;
        if !(t1 >= 0.0) {
            return 0;
        }

        // Gate 2: edges out of b must agree in handedness.
        let gx = dx - bx;
        let gy = dy - by;
        let gz = dz - bz;
        let hx = ax - bx;
        let hy = ay - by;
        let hz = az - bz;
        let kx = cx - bx;
        let ky = cy - by;
        let kz = cz - bz;
        let n2x = gy * hz - gz * hy;
        let n2y = gz * hx - gx * hz;
        let n2z = gx * hy - gy * hx;
        let mut u2 = gy * kz - gz * ky;
        u2 *= n2x;
        let mut v2 = gz * kx - gx * kz;
        v2 *= n2y;
        let mut w2 = gx * ky - gy * kx;
        w2 *= n2z;
        let t2 = (v2 + u2) + w2;
        if !(t2 >= 0.0) {
            return 0;
        }

        // Gate 3: the (k, g) pair against h.
        let n3x = ky * hz - kz * hy;
        let n3y = kz * hx - kx * hz;
        let n3z = kx * hy - ky * hx;
        let mut u3 = ky * gz - kz * gy;
        u3 *= n3x;
        let mut v3 = kz * gx - kx * gz;
        v3 *= n3y;
        let mut w3 = kx * gy - ky * gx;
        w3 *= n3z;
        let t3 = (v3 + u3) + w3;
        if !(t3 >= 0.0) {
            return 0;
        }

        // Distance of a from the plane through b, c, d.
        let qx = ky * gz - kz * gy;
        let qy = kz * gx - kx * gz;
        let qz = kx * gy - ky * gx;
        let len2 = (qy * qy + qx * qx) + qz * qz;
        let s = if len2 == 0.0 { 0.0 } else { 1.0 / len2.sqrt() };
        let d = (hy * (qy * s) + hx * (qx * s)) + hz * (qz * s);
        (0.1f32 > d.abs()) as u8
    }
});
