// original: 0x009687E0 sample_grid9_blend_rows

/// Sample a 3x3 grid of taps around an index and blend them into four output
/// rows (thiscall: `this` in ECX, three stack words, callee pops 12).
///
/// `a1` is an unsigned index, decomposed by unsigned magic division into
/// `q1 = a1 / 120` and `r1 = a1 % 120`. Nine taps (callees 1-9, stdcall/3,
/// each filling a 6-word buffer) sample the grid `(r1-1..r1+1,
/// q1+1..q1-1)`, one row of X per row of Y. `a2` is a float multiplier applied
/// to the last tap's contribution. `a3` points 12 bytes into the output area:
/// four rows of three floats at stride six words, plus the division remainder
/// stored 12 bytes before each row's successor.
///
/// The head reads five floats from `this` (`+0x2F60 b`, `+0x2F64 a`,
/// `+0x2F68 c`, `+0x2F90 centre-x`, `+0x2F94` centre-y) and calls callee 10 (cdecl, one float, answer in ST0) on
/// `(a*a + b*b) + c*c`. With that answer `s`, `x0 = ((b*s)*0 + a*s) +
/// ((c*s)*0)`: at most `-1` or unordered it becomes pi, at least `+1` it
/// becomes `0`, otherwise callee 11 answers in XMM0 (transported as two stack
/// words on this side, read back as a u64 whose low half is the float).
/// `x0` is scaled to degrees; a second combination picks it or `360 - x0`
/// the truncation of the pick feeds an unsigned divide by 360
/// whose quotient is returned and whose remainder is stored per row.
///
/// Each of the four rows then: takes the even taps' words 4-5 as a centre
/// pair, weights them against `(centre-x, centre-y)` (`1, 0` below an epsilon,
/// `K/sqrt(d)` otherwise, with an exact-zero check first that a filtered
/// zero can never reach); runs nine inner taps over every buffer's words
/// 4-5 the same way, clamping each `dot` weight into `[0, 1]` (NaN passes
/// either clamp through, matching the unordered-jump-taken behaviour) except
/// the middle tap, whose weight is `1`; normalises the nine weights by
/// `1/sqrt(sum of squares)`; and accumulates each buffer's words 0-2 times
/// its weight into the row, the last tap's whole row times `a2`. The last
/// three weights scale by the current normaliser like the first six (the
/// original spills it through the dead a3 slot while xmm1 is reused).
///
/// Omitted with no observable effect: the
/// security-cookie prologue/epilogue (the check runs natively on the original
/// side only, per the r-b09 precedent; the slot is never written), and all
/// below-ESP scratch layout (the stack check is off; every scratch value that
/// matters is observed downstream in heap writes or call arguments).
lf_checker_rt::export!(thiscall, rw_009687E0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const OFF_B: u32 = 0x2f60;
        const OFF_A: u32 = 0x2f64;
        const OFF_C: u32 = 0x2f68;
        const OFF_TA: u32 = 0x2f90;
        const OFF_TB: u32 = 0x2f94;
        const K_NEG1: u32 = 0xfe8d94;
        const K_ONE: u32 = 0xfe88e8;
        const K_PI: u32 = 0xfe8aa0;
        const K_DEG: u32 = 0xe7c2a8;
        const K_360: u32 = 0xfe8c1c;
        const K_EPS: u32 = 0xfe863c;
        const K_W: u32 = 0x1038834;
        const MAGIC: u32 = 0x88888889;
        const TRUNC_BASE: u32 = 0x1c2;
        const DIV_STEP: u32 = 0x5a;
        const DIVISOR: u32 = 0x168;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn fmul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn fadd(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn fsub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn fdiv(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn fsqrt(x: f32) -> f32 {
            core::hint::black_box(x).sqrt()
        }
        /// Exact `cvttss2si`: truncate; NaN, infinities and out-of-range give
        /// `i32::MIN` (a plain `as` cast saturates instead, which differs).
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// The loop weight: exact zero maps to zero (unreachable after the
        /// epsilon filter, kept for fidelity), anything else to `k/sqrt(d)`.
        #[inline(always)]
        fn loop_w(d: f32, k: f32) -> f32 {
            if d == 0.0 {
                0.0
            } else {
                fdiv(k, fsqrt(d))
            }
        }
        /// Clamp into `[0, k]` the way the original's two ordered jumps do:
        /// NaN passes both through.
        #[inline(always)]
        fn clamp01k(v: f32, k: f32) -> f32 {
            let mut z = v;
            if z < 0.0 {
                z = 0.0;
            }
            if z > k {
                z = k;
            }
            z
        }

        // Unsigned divide decomposition of a1 by 120, exactly the original's
        // two magic multiplies (the second re-divides q1*120 to get q1 back).
        let hi1 = (((a1 as u64) * (MAGIC as u64)) >> 32) as u32;
        let q1a = hi1 >> 6;
        let t120 = q1a.wrapping_shl(4).wrapping_sub(q1a).wrapping_shl(3);
        let r1 = a1.wrapping_sub(t120);
        let s = a1.wrapping_sub(r1);
        let hi2 = (((s as u64) * (MAGIC as u64)) >> 32) as u32;
        let q1 = hi2 >> 6;

        // Nine grid taps; buffers start zeroed (fresh below-ESP scratch
        // reads the stack fill on the original side, and the contract
        // snapshots the pre-call words).
        let mut b1 = [0u32; 6];
        let mut b2 = [0u32; 6];
        let mut b3 = [0u32; 6];
        let mut b4 = [0u32; 6];
        let mut b5 = [0u32; 6];
        let mut b6 = [0u32; 6];
        let mut b7 = [0u32; 6];
        let mut b8 = [0u32; 6];
        let mut b9 = [0u32; 6];
        let y_hi = q1.wrapping_add(1);
        let y_lo = q1.wrapping_sub(1);
        let x_lo = r1.wrapping_sub(1);
        let x_hi = r1.wrapping_add(1);
        lf_checker_rt::callee_stdcall!(1, u32, x_lo, y_hi, b1.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(2, u32, r1, y_hi, b2.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(3, u32, x_hi, y_hi, b3.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(4, u32, x_lo, q1, b4.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(5, u32, r1, q1, b5.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(6, u32, x_hi, q1, b6.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(7, u32, x_lo, y_lo, b7.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(8, u32, r1, y_lo, b8.as_mut_ptr() as u32);
        lf_checker_rt::callee_stdcall!(9, u32, x_hi, y_lo, b9.as_mut_ptr() as u32);
        let f1 = [
            f32::from_bits(b1[0]),
            f32::from_bits(b1[1]),
            f32::from_bits(b1[2]),
            f32::from_bits(b1[4]),
            f32::from_bits(b1[5]),
        ];
        let f2 = [
            f32::from_bits(b2[0]),
            f32::from_bits(b2[1]),
            f32::from_bits(b2[2]),
            f32::from_bits(b2[4]),
            f32::from_bits(b2[5]),
        ];
        let f3 = [
            f32::from_bits(b3[0]),
            f32::from_bits(b3[1]),
            f32::from_bits(b3[2]),
            f32::from_bits(b3[4]),
            f32::from_bits(b3[5]),
        ];
        let f4 = [
            f32::from_bits(b4[0]),
            f32::from_bits(b4[1]),
            f32::from_bits(b4[2]),
            f32::from_bits(b4[4]),
            f32::from_bits(b4[5]),
        ];
        let f5 = [
            f32::from_bits(b5[0]),
            f32::from_bits(b5[1]),
            f32::from_bits(b5[2]),
            f32::from_bits(b5[4]),
            f32::from_bits(b5[5]),
        ];
        let f6 = [
            f32::from_bits(b6[0]),
            f32::from_bits(b6[1]),
            f32::from_bits(b6[2]),
            f32::from_bits(b6[4]),
            f32::from_bits(b6[5]),
        ];
        let f7 = [
            f32::from_bits(b7[0]),
            f32::from_bits(b7[1]),
            f32::from_bits(b7[2]),
            f32::from_bits(b7[4]),
            f32::from_bits(b7[5]),
        ];
        let f8 = [
            f32::from_bits(b8[0]),
            f32::from_bits(b8[1]),
            f32::from_bits(b8[2]),
            f32::from_bits(b8[4]),
            f32::from_bits(b8[5]),
        ];
        let f9 = [
            f32::from_bits(b9[0]),
            f32::from_bits(b9[1]),
            f32::from_bits(b9[2]),
            f32::from_bits(b9[4]),
            f32::from_bits(b9[5]),
        ];

        // Float head.
        let ta = rdf(this.wrapping_add(OFF_TA));
        let tb = rdf(this.wrapping_add(OFF_TB));
        let a = rdf(this.wrapping_add(OFF_A));
        let b = rdf(this.wrapping_add(OFF_B));
        let c = rdf(this.wrapping_add(OFF_C));
        let c2arg = fadd(fadd(fmul(a, a), fmul(b, b)), fmul(c, c));
        let ans: f32 = lf_checker_rt::callee_cdecl!(10, f32, c2arg.to_bits());
        let bans = fmul(b, ans);
        let aans = fmul(a, ans);
        let bans0 = fmul(bans, 0.0);
        let cans0 = fmul(fmul(c, ans), 0.0);
        let x0 = fadd(fadd(bans0, aans), cans0);
        let k1 = rdf(lf_checker_rt::relocated(K_NEG1));
        let k_one = rdf(lf_checker_rt::relocated(K_ONE));
        let x0v: f32;
        if !(x0 > k1) {
            x0v = rdf(lf_checker_rt::relocated(K_PI));
        } else if x0 >= k_one {
            x0v = 0.0;
        } else {
            let got: u64 = lf_checker_rt::callee_cdecl!(11, u64, x0.to_bits(), 0u32);
            x0v = f32::from_bits(got as u32);
        }
        let deg = fmul(x0v, rdf(lf_checker_rt::relocated(K_DEG)));
        // The callee-11 reloads read these same slots (verified on the esp map).
        let x1in = fadd(fadd(fmul(aans, 0.0), bans), cans0);
        let x1 = if 0.0 < x1in {
            deg
        } else {
            fsub(rdf(lf_checker_rt::relocated(K_360)), deg)
        };
        let trunc = cvtt(x1);
        let mut divbase = TRUNC_BASE.wrapping_sub(trunc as u32);
        let mut out = a3.wrapping_add(8);
        let mut last_q = 0u32;
        let k_eps = rdf(lf_checker_rt::relocated(K_EPS));
        let k_w = rdf(lf_checker_rt::relocated(K_W));
        let a2f = f32::from_bits(a2);
        // Centre pairs copied from the even taps' words 4-5.
        let pairs = [(f2[3], f2[4]), (f4[3], f4[4]), (f6[3], f6[4]), (f8[3], f8[4])];
        let inn = [
            (f1[3], f1[4]),
            (f2[3], f2[4]),
            (f3[3], f3[4]),
            (f4[3], f4[4]),
            (f5[3], f5[4]),
            (f6[3], f6[4]),
            (f7[3], f7[4]),
            (f8[3], f8[4]),
            (f9[3], f9[4]),
        ];
        for oi in 0..4u32 {
            let (px, py) = pairs[oi as usize];
            let dx7 = fsub(ta, px);
            let dy6 = fsub(tb, py);
            let d = fadd(fmul(dy6, dy6), fmul(dx7, dx7));
            let (w7, w6): (f32, f32);
            if d < k_eps {
                w7 = 1.0;
                w6 = 0.0;
            } else {
                let w = loop_w(d, k_one);
                w7 = fmul(dx7, w);
                w6 = fmul(dy6, w);
            }
            let mut w = [0.0f32; 9];
            for (ii, (qx, qy)) in inn.iter().enumerate() {
                let ddx = fsub(ta, *qx);
                let ddy = fsub(tb, *qy);
                let d2 = fadd(fmul(ddy, ddy), fmul(ddx, ddx));
                let (s3, s0): (f32, f32);
                if d2 < k_eps {
                    s3 = 1.0;
                    s0 = 0.0;
                } else {
                    let v = loop_w(d2, k_one);
                    s3 = fmul(ddx, v);
                    s0 = fmul(ddy, v);
                }
                let o = if ii == 4 {
                    k_one
                } else {
                    clamp01k(fadd(fmul(s0, w6), fmul(s3, w7)), k_one)
                };
                let t2 = fmul(ddx, k_w);
                let t1 = fmul(ddy, k_w);
                let n = fadd(fmul(t1, t1), fmul(t2, t2));
                let q = clamp01k(fdiv(k_one, n), k_one);
                w[ii] = fmul(q, o);
            }
            let mut s = fmul(w[0], w[0]);
            s = fadd(s, fmul(w[1], w[1]));
            s = fadd(s, fmul(w[2], w[2]));
            s = fadd(s, fmul(w[3], w[3]));
            s = fadd(s, fmul(w[4], w[4]));
            s = fadd(s, fmul(w[5], w[5]));
            s = fadd(s, fmul(w[6], w[6]));
            s = fadd(s, fmul(w[7], w[7]));
            s = fadd(s, fmul(w[8], w[8]));
            let nrm = fdiv(k_one, fsqrt(s));
            // All nine weights scale by this row's normaliser (the original
            // spills it through the dead a3 slot while xmm1 is reused).
            let w0p = fmul(w[0], nrm);
            let w1p = fmul(w[1], nrm);
            let w2p = fmul(w[2], nrm);
            let w3p = fmul(w[3], nrm);
            let w4p = fmul(w[4], nrm);
            let w5p = fmul(w[5], nrm);
            let w6p = fmul(w[6], nrm);
            let w7p = fmul(w[7], nrm);
            let w8p = fmul(w[8], nrm);
            let mut c0 = fmul(f1[0], w0p);
            let mut c1 = fmul(f1[1], w0p);
            let mut c2v = fmul(f1[2], w0p);
            c1 = fadd(c1, fmul(f2[1], w1p));
            c2v = fadd(c2v, fmul(f2[2], w1p));
            c0 = fadd(c0, fmul(f2[0], w1p));
            c1 = fadd(c1, fmul(f3[1], w2p));
            c2v = fadd(c2v, fmul(f3[2], w2p));
            c0 = fadd(c0, fmul(f3[0], w2p));
            c1 = fadd(c1, fmul(f4[1], w3p));
            c2v = fadd(c2v, fmul(f4[2], w3p));
            c0 = fadd(c0, fmul(f4[0], w3p));
            c1 = fadd(c1, fmul(f5[1], w4p));
            c2v = fadd(c2v, fmul(f5[2], w4p));
            c0 = fadd(c0, fmul(f5[0], w4p));
            c1 = fadd(c1, fmul(f6[1], w5p));
            c2v = fadd(c2v, fmul(f6[2], w5p));
            c0 = fadd(c0, fmul(f6[0], w5p));
            c1 = fadd(c1, fmul(f7[1], w6p));
            c2v = fadd(c2v, fmul(f7[2], w6p));
            c0 = fadd(c0, fmul(f7[0], w6p));
            c1 = fadd(c1, fmul(f8[1], w7p));
            c2v = fadd(c2v, fmul(f8[2], w7p));
            c0 = fadd(c0, fmul(f8[0], w7p));
            c1 = fmul(fadd(c1, fmul(f9[1], w8p)), a2f);
            c2v = fmul(fadd(c2v, fmul(f9[2], w8p)), a2f);
            c0 = fmul(fadd(c0, fmul(f9[0], w8p)), a2f);
            (out.wrapping_sub(8) as *mut u32).write_unaligned(c0.to_bits());
            (out.wrapping_sub(4) as *mut u32).write_unaligned(c1.to_bits());
            (out as *mut u32).write_unaligned(c2v.to_bits());
            // Unsigned divide of the running base by 360 (exact, no fault:
            // the high word is zero and the divisor is constant).
            let q = divbase / DIVISOR;
            let r = divbase % DIVISOR;
            last_q = q;
            divbase = divbase.wrapping_add(DIV_STEP);
            out = out.wrapping_add(0x18);
            (out.wrapping_sub(0x14) as *mut u32).write_unaligned(r);
        }
        last_q
    }
});
