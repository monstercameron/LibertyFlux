// original: 0x00c95c50 sphere_contact_point (proposed)
//
// Contact-style point between two spheres `a` and `b` with radii `r1`/`r2`,
// oriented by a third point `c`. `a`, `b`, `c` point at three-float
// positions. Writes three floats to `out` and returns `out`.
//
// Let d = a - b and dist = |d|. When dist is strictly greater than
// r1 + r2, `out` is the midpoint (a + b) / 2. Otherwise (overlap, touch,
// or NaN anywhere) let
// t = (r2^2 - r1^2 + dist^2) / (2*dist) and p = b + (t/dist)*d; m is
// sqrt(r2^2 - t^2), or zero when that is not positive; u is d normalised
// (or zero when dist is zero); w is u x c normalised; v is w x u
// normalised; `out` = p + m*v. Each normalise-or-zero step yields zero for
// a zero-length input (including negative zero) and NaN for NaN, matching
// the original's flag-based tests exactly.
//
// Original: stdcall, six stack words, returns the out pointer in eax. The
// float operation order below is the original's, including which operand of
// each multiply comes first.
lf_checker_rt::export!(stdcall, rw_00c95c50(out: u32, a: u32, b: u32, r1bits: u32, r2bits: u32, c: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let half: f32 = (lf_checker_rt::global::<f32>(0x00fe8830) as *const f32).read();
        let one: f32 = (lf_checker_rt::global::<f32>(0x00fe88e8) as *const f32).read();
        let two: f32 = (lf_checker_rt::global::<f32>(0x00fe8a24) as *const f32).read();
        let norm_or_zero = |len2: f32| -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                fdiv(one, len2.sqrt())
            }
        };
        let r1 = f32::from_bits(r1bits);
        let r2 = f32::from_bits(r2bits);
        let (ax, ay, az) = (rdf(a), rdf(a + 4), rdf(a + 8));
        let (bx, by, bz) = (rdf(b), rdf(b + 4), rdf(b + 8));
        let dx = fsub(ax, bx);
        let dy = fsub(ay, by);
        let dz = fsub(az, bz);
        let dx2 = fmul(dx, dx);
        let dy2 = fmul(dy, dy);
        let mut dist2 = fadd(dx2, dy2);
        let dz2 = fmul(dz, dz);
        dist2 = fadd(dist2, dz2);
        let sum = fadd(r1, r2);
        let dist = dist2.sqrt();
        if dist > sum {
            let sx = fadd(bx, ax);
            let sy = fadd(by, ay);
            let sz = fadd(bz, az);
            wrf(out, fmul(sx, half));
            wrf(out + 4, fmul(sy, half));
            wrf(out + 8, fmul(sz, half));
            return out;
        }
        let r1sq = fmul(r1, r1);
        let r2sq = fmul(r2, r2);
        let mut t = fsub(r2sq, r1sq);
        let d0 = fmul(dist2.sqrt(), dist);
        t = fadd(t, d0);
        let d1 = fmul(dist2.sqrt(), two);
        t = fdiv(t, d1);
        let dx_t = fmul(dx, t);
        let dy_t = fmul(dy, t);
        let dz_t = fmul(dz, t);
        let t2 = fmul(t, t);
        let s = fdiv(one, dist);
        let dxn = fmul(dx_t, s);
        let dzn = fmul(dz_t, s);
        let dyn_ = fmul(dy_t, s);
        let px = fadd(dxn, bx);
        let h = fsub(r2sq, t2);
        let py = fadd(dyn_, by);
        let pz = fadd(dzn, bz);
        let m = if h > 0.0 || h.is_nan() { h.sqrt() } else { 0.0 };
        let n = norm_or_zero(dist2);
        let ux = fmul(n, dx);
        let uy = fmul(n, dy);
        let uz = fmul(dz, n);
        let (cx, cy, cz) = (rdf(c), rdf(c + 4), rdf(c + 8));
        let cz_ux = fmul(cz, ux);
        let mut wx = fmul(cz, uy);
        let mut wz = fmul(cy, ux);
        let cy_uz = fmul(cy, uz);
        wx = fsub(wx, cy_uz);
        let cx_uy = fmul(cx, uy);
        let mut wy = fmul(cx, uz);
        wz = fsub(wz, cx_uy);
        wy = fsub(wy, cz_ux);
        let wy2 = fmul(wy, wy);
        let wx2 = fmul(wx, wx);
        let mut wlen2 = fadd(wy2, wx2);
        let wz2 = fmul(wz, wz);
        wlen2 = fadd(wlen2, wz2);
        let m2 = norm_or_zero(wlen2);
        wz = fmul(wz, m2);
        wy = fmul(wy, m2);
        wx = fmul(wx, m2);
        let wz_uy = fmul(wz, uy);
        let mut vy = fmul(wz, ux);
        let wy_ux = fmul(wy, ux);
        let mut vx = fmul(wy, uz);
        vx = fsub(vx, wz_uy);
        let mut vz = fmul(wx, uy);
        let wx_uz = fmul(wx, uz);
        vz = fsub(vz, wy_ux);
        vy = fsub(vy, wx_uz);
        let vy2 = fmul(vy, vy);
        let vx2 = fmul(vx, vx);
        let mut vlen2 = fadd(vy2, vx2);
        let vz2 = fmul(vz, vz);
        vlen2 = fadd(vlen2, vz2);
        let m3 = norm_or_zero(vlen2);
        let ox = fadd(fmul(fmul(m3, vx), m), px);
        let oy = fadd(fmul(fmul(m3, vy), m), py);
        let oz = fadd(fmul(fmul(vz, m3), m), pz);
        wrf(out, ox);
        wrf(out + 4, oy);
        wrf(out + 8, oz);
        out
    }
});
