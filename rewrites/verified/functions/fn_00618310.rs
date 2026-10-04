// original: 0x00618310 view_bounds_commit_rect (proposed)

/// Project the view volume, derive a pixel rectangle from its bounds, and
/// commit it when it differs from the cached one.
///
/// `this` points to the view object (rate input at `+0x84`; resolver input
/// at `+0x10`). The stack argument is a pointer to a matrix block whose
/// projection rows are read at `+0x200` through `+0x23c`.
///
/// Behaviour in order: the resolver callee (setter 1) fills three output
/// triples; scaled by the rate-derived factor they form eight corner points
/// (paired sums and differences around a center). Each corner is projected
/// through the matrix rows (`(1/w)*x`, `(1/w)*y`) and folded into running
/// minimum/maximum bounds for both axes, starting from 0 for the maxima and
/// a large constant for the minima. The minima pass through a round-to-int
/// idiom and are clamped at pairwise-selected global limits (the second of
/// each pair is used when a gate global equals the poll callee's latest
/// answer) to form the rectangle origin; the maxima travel through the
/// double-precision callee (setter 3) and the same clamp to form the corner.
/// A zero corner, or an origin that still equals a freshly polled limit,
/// returns 0. Otherwise setter 4 commits `(x, y, w, h)` and the result is 1.
/// The poll callee (setter 2) is reached through a global function pointer
/// and takes no arguments; the cookie-check callee (setter 5) preserves all
/// registers and is called on both return paths.
///
/// Original: thiscall with one stack word, returns a byte in AL. All float
/// arithmetic is single precision in the original's operation order; the two
/// double-precision calls round exactly once back to single.
lf_checker_rt::export!(thiscall, rw_00618310(this: u32, matrix: u32) -> u32 {
    unsafe {
        const C_RESOLVER: u32 = 1;
        const C_POLL: u32 = 2;
        const C_DBL: u32 = 3;
        const C_COMMIT: u32 = 4;
        const C_COOKIE: u32 = 5;
        const G_FNPTR: u32 = 0x00e731ac;
        const G_GATE: u32 = 0x0110dd14;
        const G_LIM_X: u32 = 0x0105c884;
        const G_LIM_X_ALT: u32 = 0x0105c888;
        const G_LIM_Y: u32 = 0x0105c880;
        const G_LIM_Y_ALT: u32 = 0x0105c87c;
        const F_RATE_A: u32 = 0x00fe8830;
        const F_RATE_B: u32 = 0x00fe86c4;
        const F_MIN_INIT: u32 = 0x00fe8c70;
        const F_ONE: u32 = 0x00fe88e8;
        const F_RND_M: u32 = 0x00fe8d1c;
        const F_RND_E: u32 = 0x00fe8cf8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(g: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(g)) }
        }
        #[inline(always)]
        unsafe fn gf(g: u32) -> f32 {
            unsafe { f32::from_bits(g32(g)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate toward zero with cvttss2si's invalid semantics: NaN and
        /// out-of-range values yield 0x80000000, not saturation.
        #[inline(always)]
        fn cvtt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000
            } else {
                x as i32 as u32
            }
        }
        /// The original's round-to-nearest-integer idiom, bitwise.
        #[inline(always)]
        unsafe fn round_magic(x: f32) -> u32 {
            unsafe {
                let rm = gf(F_RND_M).to_bits();
                let re = gf(F_RND_E);
                let one = gf(F_ONE).to_bits();
                let xb = x.to_bits();
                let b3 = rm & xb;
                let t = xb ^ b3;
                let m1 = if f32::from_bits(t) < re { 0xffff_ffffu32 } else { 0 };
                let b1 = (re.to_bits() & m1) | b3;
                let x2 = sub(add(x, f32::from_bits(b1)), f32::from_bits(b1));
                let t2 = sub(x2, x);
                // CMPSS predicate 6: not-less-or-equal (true for unordered
                // and for greater-than, false for equal), not not-less-than:
                // equal inputs must yield a zero mask.
                let le = f32::from_bits(t2.to_bits()) <= f32::from_bits(b3);
                let m2 = if le { 0u32 } else { 0xffff_ffffu32 };
                cvtt(sub(x2, f32::from_bits(m2 & one)))
            }
        }
        /// Signed clamp at zero from below and `lim` from above.
        #[inline(always)]
        fn clamp0(i: u32, lim: u32) -> u32 {
            if (i as i32) < 0 {
                0
            } else if (i as i32) > (lim as i32) {
                lim
            } else {
                i
            }
        }
        #[inline(always)]
        unsafe fn poll() -> u32 {
            unsafe {
                let tgt = rd32(lf_checker_rt::relocated(G_FNPTR));
                let f: extern "cdecl" fn() -> u32 = core::mem::transmute(tgt as usize);
                f()
            }
        }

        // Resolver outputs: out0/out1/out2 hold the second, third and first
        // triples the original reads (argument order 0, 1, 2).
        let mut out0 = [0u32; 3];
        let mut out1 = [0u32; 3];
        let mut out2 = [0u32; 3];
        lf_checker_rt::callee_thiscall!(
            C_RESOLVER, u32, this.wrapping_add(0x10),
            out0.as_mut_ptr() as u32, out1.as_mut_ptr() as u32, out2.as_mut_ptr() as u32
        );
        let (u0, u1, u2) = (f32::from_bits(out0[0]), f32::from_bits(out0[1]), f32::from_bits(out0[2]));
        let (v0, v1, v2) = (f32::from_bits(out1[0]), f32::from_bits(out1[1]), f32::from_bits(out1[2]));
        let (w0, w1, w2) = (f32::from_bits(out2[0]), f32::from_bits(out2[1]), f32::from_bits(out2[2]));
        let rate = rdf(this.wrapping_add(0x84));
        let mut k = mul(rate, gf(F_RATE_A));
        k = mul(k, rate);
        k = mul(k, gf(F_RATE_B));
        let (w0k, w1k, w2k) = (mul(w0, k), mul(w1, k), mul(w2, k));
        let (s0, s1, s2) = (mul(v0, k), mul(v1, k), mul(v2, k));
        let (a0, a1) = (sub(u0, w0k), sub(u1, w1k));
        let (b0, b1) = (add(u0, w0k), add(u1, w1k));
        let (c_, d) = (sub(u2, w2k), add(u2, w2k));
        // Eight corners as four triples: (a0-s0, a1-s1, c_-s2),
        // (b0-s0, b1-s1, d-s2), (a0+s0, a1+s1, c_+s2), (b0+s0, b1+s1, d+s2).
        let corners = [
            (sub(a0, s0), sub(a1, s1), sub(c_, s2)),
            (sub(b0, s0), sub(b1, s1), sub(d, s2)),
            (add(a0, s0), add(a1, s1), add(c_, s2)),
            (add(b0, s0), add(b1, s1), add(d, s2)),
        ];
        let one = gf(F_ONE);
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        let mut min_x = gf(F_MIN_INIT);
        let mut min_y = min_x;
        for (p0, p1, p2) in corners {
            let t1 = add(add(add(mul(rdf(matrix.wrapping_add(0x200)), p0), mul(rdf(matrix.wrapping_add(0x210)), p1)), mul(rdf(matrix.wrapping_add(0x220)), p2)), rdf(matrix.wrapping_add(0x230)));
            let t2 = add(add(add(mul(rdf(matrix.wrapping_add(0x204)), p0), mul(rdf(matrix.wrapping_add(0x214)), p1)), mul(rdf(matrix.wrapping_add(0x224)), p2)), rdf(matrix.wrapping_add(0x234)));
            let t4 = add(add(add(mul(rdf(matrix.wrapping_add(0x20c)), p0), mul(rdf(matrix.wrapping_add(0x21c)), p1)), mul(rdf(matrix.wrapping_add(0x22c)), p2)), rdf(matrix.wrapping_add(0x23c)));
            let inv = core::hint::black_box(one) / core::hint::black_box(t4);
            let x = mul(inv, t1);
            let y = mul(inv, t2);
            if !(max_x > x) {
                max_x = x;
            }
            if !(max_y > y) {
                max_y = y;
            }
            if !(x > min_x) {
                min_x = x;
            }
            if !(y > min_y) {
                min_y = y;
            }
        }

        let gate_is = |ans: u32| g32(G_GATE) == ans;
        let ox = clamp0(round_magic(min_x), if gate_is(poll()) { g32(G_LIM_X_ALT) } else { g32(G_LIM_X) });
        let oy = clamp0(round_magic(min_y), if gate_is(poll()) { g32(G_LIM_Y_ALT) } else { g32(G_LIM_Y) });
        let lim_cx = if gate_is(poll()) { g32(G_LIM_X_ALT) } else { g32(G_LIM_X) };
        let dd: f64 = {
            let bits = (max_x as f64).to_bits();
            lf_checker_rt::callee_cdecl!(C_DBL, f64, (bits & 0xffff_ffff) as u32, (bits >> 32) as u32)
        };
        let cx = clamp0(cvtt(dd as f32), lim_cx);
        let lim_cy = if gate_is(poll()) { g32(G_LIM_Y_ALT) } else { g32(G_LIM_Y) };
        let dd2: f64 = {
            let bits = (max_y as f64).to_bits();
            lf_checker_rt::callee_cdecl!(C_DBL, f64, (bits & 0xffff_ffff) as u32, (bits >> 32) as u32)
        };
        let cy = clamp0(cvtt(dd2 as f32), lim_cy);
        if cx == 0 || cy == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if ox == (if gate_is(poll()) { g32(G_LIM_X_ALT) } else { g32(G_LIM_X) }) {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        if oy == (if gate_is(poll()) { g32(G_LIM_Y_ALT) } else { g32(G_LIM_Y) }) {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return 0;
        }
        lf_checker_rt::callee_cdecl!(C_COMMIT, u32, ox, oy, cx.wrapping_sub(ox), cy.wrapping_sub(oy));
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        1
    }
});
