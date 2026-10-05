// original: 0x00c96270 sphere_pair_point (proposed)
//
// Solves the two-sphere interpolation of point `b` relative to point `a`.
// `a` and `b` point at three-float positions; `f1`/`f2` are radii-like
// scalars. Writes a four-float record to `out_c` and one float to `out_d`,
// and returns `out_d`.
//
// Let d = b - a, dist = |d|, t = (f1^2 - f2^2 + dist^2) / (2*dist).
// out_c[0..3] = a + (t/dist)*d, the point along a->b at parameter t/dist.
// out_c[3] is whatever word sits in the original's stack frame at that
// moment (an uninitialised read); the contract pins the stack fill to zero,
// so the rewrite stores zero there.
// h = f1^2 - t^2; out_d[0] = sqrt(h) when h is positive or NaN, else zero.
//
// The original also spills f1^2 into its own incoming f1 argument slot (a
// word the stdcall return then pops, so no caller can observe it); the
// rewrite cannot address that slot, so the contract compares everything
// except the incoming-argument area. Original: stdcall, six stack words,
// returns the out_d pointer in eax.
lf_checker_rt::export!(stdcall, rw_00c96270(a: u32, b: u32, f1bits: u32, f2bits: u32, out_c: u32, out_d: u32) -> u32 {
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
        let two: f32 = (lf_checker_rt::global::<f32>(0x00fe8a24) as *const f32).read();
        let one: f32 = (lf_checker_rt::global::<f32>(0x00fe88e8) as *const f32).read();
        let f1 = f32::from_bits(f1bits);
        let f2 = f32::from_bits(f2bits);
        let dx = fsub(rdf(b), rdf(a));
        let dy = fsub(rdf(b + 4), rdf(a + 4));
        let dz = fsub(rdf(b + 8), rdf(a + 8));
        let f1sq = fmul(f1, f1);
        let dx2 = fmul(dx, dx);
        let dy2 = fmul(dy, dy);
        let mut dist2 = fadd(dx2, dy2);
        let dz2 = fmul(dz, dz);
        dist2 = fadd(dist2, dz2);
        let f2sq = fmul(f2, f2);
        let dist = dist2.sqrt();
        let mut t = fsub(f1sq, f2sq);
        let dist_sq = fmul(dist, dist);
        t = fadd(t, dist_sq);
        let denom = fmul(dist, two);
        t = fdiv(t, denom);
        let dx_t = fmul(dx, t);
        let dy_t = fmul(dy, t);
        let s = fdiv(one, dist);
        let mut nx = fmul(dx_t, s);
        let dz_t = fmul(dz, t);
        nx = fadd(nx, rdf(a));
        let mut ny = fmul(dy_t, s);
        let mut nz = fmul(dz_t, s);
        ny = fadd(ny, rdf(a + 4));
        nz = fadd(nz, rdf(a + 8));
        wrf(out_c, nx);
        // Uninitialised stack word in the original; zero under this contract.
        wr32(out_c + 12, 0);
        wrf(out_c + 4, ny);
        let t2 = fmul(t, t);
        let h = fsub(f1sq, t2);
        wrf(out_c + 8, nz);
        let hd = if h > 0.0 || h.is_nan() { h.sqrt() } else { 0.0 };
        wrf(out_d, hd);
        out_d
    }
});
