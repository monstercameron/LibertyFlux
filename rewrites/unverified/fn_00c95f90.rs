// original: 0x00c95f90 guided_projection_point (proposed)
//
// Projects `p2` onto the line through `p1` in the direction `p3 - p1`,
// scaled by a radius solve over a second point pair `q1`/`q2`, and offsets
// the result by that solve's centre. All six points are three-float
// positions; the sixth stack word is never read. Writes three floats to
// `out` and returns `out`.
//
// Let qd = q2 - q1 and qdist = |qd|. When qdist is strictly greater than
// r1 + r2, `out` is q1 advanced toward q2 by r1 (normalised, or zero when
// qdist is zero). Otherwise the function calls its sphere-pair helper with
// (q1, q2, r1, r2) into two temporaries: a four-word centre `tc` and a
// scalar `td`. Only tc[0..3] and td[0] are read; the three global words
// the original stores into the temporaries first are dead (the call
// overwrites them) and are not reproduced here. k is td[0] clamped to zero
// when below 0.01 (NaN passes through).
// Let e = m*(p3 - p1) with m normalising (or zero for a zero length),
// j = p1 + ((e . (p2 - p1)) * e), r = p2 - j, and n2 normalising r.
// `out` = tc[0..3] + k*n2*r.
//
// Original: stdcall, nine stack words, returns the out pointer in eax.
// The float operation order is the original's.
lf_checker_rt::export!(stdcall, rw_00c95f90(out: u32, p1: u32, p2: u32, p3: u32, q1: u32, _unused: u32, q2: u32, r1bits: u32, r2bits: u32) -> u32 {
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
        let one: f32 = (lf_checker_rt::global::<f32>(0x00fe88e8) as *const f32).read();
        let kth: f32 = (lf_checker_rt::global::<f32>(0x00fe870c) as *const f32).read();
        let norm_or_zero = |len2: f32| -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                fdiv(one, len2.sqrt())
            }
        };
        let r1 = f32::from_bits(r1bits);
        let r2 = f32::from_bits(r2bits);
        let (q1x, q1y, q1z) = (rdf(q1), rdf(q1 + 4), rdf(q1 + 8));
        let dx = fsub(rdf(q2), q1x);
        let dy = fsub(rdf(q2 + 4), q1y);
        let dz = fsub(rdf(q2 + 8), q1z);
        let dy2 = fmul(dy, dy);
        let dx2 = fmul(dx, dx);
        let mut dist2 = fadd(dy2, dx2);
        let dz2 = fmul(dz, dz);
        dist2 = fadd(dist2, dz2);
        let sum = fadd(r1, r2);
        let dist = dist2.sqrt();
        if dist > sum {
            let n = norm_or_zero(dist2);
            let ox = fadd(q1x, fmul(fmul(n, dx), r1));
            let oy = fadd(q1y, fmul(fmul(dy, n), r1));
            let oz = fadd(q1z, fmul(fmul(dz, n), r1));
            wrf(out, ox);
            wrf(out + 4, oy);
            wrf(out + 8, oz);
            return out;
        }
        let mut tmp_c = [0u32; 4];
        let mut tmp_d = [0u32; 1];
        lf_checker_rt::callee_stdcall!(0, u32, q1, q2, r1bits, r2bits, tmp_c.as_mut_ptr() as u32, tmp_d.as_mut_ptr() as u32);
        let (c0, c1, c2) = (
            f32::from_bits(tmp_c[0]),
            f32::from_bits(tmp_c[1]),
            f32::from_bits(tmp_c[2]),
        );
        let mut k = f32::from_bits(tmp_d[0]);
        if kth > k {
            k = 0.0;
        }
        let (p1x, p1y, p1z) = (rdf(p1), rdf(p1 + 4), rdf(p1 + 8));
        let e1 = fsub(rdf(p2), p1x);
        let e2 = fsub(rdf(p3 + 4), p1y);
        let e3 = fsub(rdf(p3 + 8), p1z);
        let e4 = fsub(rdf(p2 + 8), p1z);
        let e5 = fsub(rdf(p3), p1x);
        let e6 = fsub(rdf(p2 + 4), p1y);
        let e2sq = fmul(e2, e2);
        let e5sq = fmul(e5, e5);
        let mut len2 = fadd(e2sq, e5sq);
        let e3sq = fmul(e3, e3);
        len2 = fadd(len2, e3sq);
        let m = norm_or_zero(len2);
        let s1 = fmul(e2, m);
        let s2 = fmul(e5, m);
        let s3 = fmul(e3, m);
        let mut dot = fmul(s1, e6);
        let t1 = fmul(s2, e1);
        dot = fadd(dot, t1);
        let t2 = fmul(s3, e4);
        dot = fadd(dot, t2);
        let jx = fadd(fmul(s1, dot), p1y);
        let jy = fadd(fmul(s2, dot), p1x);
        let jz = fadd(fmul(s3, dot), p1z);
        let rx = fsub(rdf(p2), jy);
        let ry = fsub(rdf(p2 + 4), jx);
        let rz = fsub(rdf(p2 + 8), jz);
        let ry2 = fmul(ry, ry);
        let rx2 = fmul(rx, rx);
        let mut rlen2 = fadd(ry2, rx2);
        let rz2 = fmul(rz, rz);
        rlen2 = fadd(rlen2, rz2);
        let n2 = norm_or_zero(rlen2);
        let ox = fadd(fmul(fmul(rx, n2), k), c0);
        let oy = fadd(fmul(fmul(ry, n2), k), c1);
        let oz = fadd(fmul(fmul(rz, n2), k), c2);
        wrf(out, ox);
        wrf(out + 4, oy);
        wrf(out + 8, oz);
        out
    }
});
