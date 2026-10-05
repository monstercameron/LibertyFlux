// original: 0x00680B70 euphoria_solve_blend (proposed)

/// Solve a two-stage blend: pick a path from the step parameter, run the
/// power helper and the quaternion helper, and combine the results.
///
/// `this`+0x0c holds a limit `c`. The stack words are two opaque integers
/// `b`, `a` (in that order), a quaternion pointer `q` and a 3-vector
/// pointer `v`; the entry vector registers carry the step `t` (XMM1 low)
/// and a bound `lim` (XMM2 low) with zeroed upper words. `q` and `v` are
/// initialised first (identity quaternion, zero vector).
///
/// Paths: `lim > 0` goes to A, otherwise to B. On A, `t < lim` (A1) runs the
/// power helper (callee id 1, cdecl, `pow(c, -(t-lim))` from two doubles on
/// the x87 stack, double answer on x87) and the quaternion helper (id 11)
/// with vector arguments (`c`, pow1); when `-(t-lim) > c` it also runs the
/// helper again (id 12) with (`c`, `c`), rounds `-(t-lim)/c` to an integer
/// count and repeats the combine step that many times. `t >= lim` (A2)
/// skips straight to the helper (id 13) with (`t`, min(`lim`,`t`)) and the
/// tail math. On B, `lim >= 0` (B2) returns 1 with no calls; otherwise
/// (B1) when `(t-lim) > c` it runs the power helper (id 2, `pow(c, t-lim)`)
/// and the quaternion helper (id 14) with (0, -pow2), then, when
/// `(t-lim) > 2c`, the helper again (id 15) with (0, `-c`) and a rounded
/// count of `(t-lim)/c` minus one combine repeats; when `(t-lim) <= c`
/// (B1b) it skips all three and goes straight to the helper a last time
/// (id 16) with (`t`, max(`t-c`,`lim`)) and the tail math. Any helper
/// answer of 0 returns 0; otherwise 1. The first helper call on each side
/// reuses the power call's four pushed words (`b`, `a`, `q`, `v`) as its
/// own stack arguments.
///
/// The power helper's x87 argument pair is invisible to the checker (no
/// x87-argument logging exists); only its scripted double answer is
/// observed downstream. The quaternion helper takes four stack words plus
/// XMM1 and XMM2. Where both vector registers are clean (sites 11, 12)
/// the rewrite carries them in slots 0 and 1 and the integers pass
/// unverified; sites 14 and 15 carry XMM1 (= 0) in slot 1 and keep the
/// real `b` in slot 0, while their XMM2 (low word over three `-0.0`
/// words) is uncompared;
/// sites 13 and 16 keep the real integers and skip the frame-pointer slots,
/// while their vector registers are uncompared (their upper words hold the
/// quotient and combine leftovers the transport cannot reproduce). Loop
/// counts come from an exact round-and-fixup idiom (the fixup test is a
/// greater-than, `cmpnless` = predicate 6; for the positive quotients here
/// the idiom equals floor) plus truncation; a negative count would spin
/// the dec/jne loop forever, so the contract keeps every count in [0, 8]
/// (see the input pins there).
///
/// Original: 0x00680B70 (thiscall, ECX + four stack words, callee pops 16,
/// returns AL).
lf_checker_rt::export!(thiscall, rw_00680b70(ecx: u32, b: u32, a: u32, q: u32, v: u32) -> u32 {
    unsafe {
        const POW1: u32 = 1;
        const POW2: u32 = 2;
        const F1: u32 = 11;
        const F2: u32 = 12;
        const F3: u32 = 13;
        const F4: u32 = 14;
        const F5: u32 = 15;
        const F6: u32 = 16;
        const SIGN_BIT: u32 = 0x8000_0000;
        const F23: u32 = 0x4b00_0000;
        const ONE_F: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(x: u32, val: u32) {
            unsafe { (x as *mut u32).write_unaligned(val) }
        }
        #[inline(always)]
        unsafe fn rdf(x: u32) -> f32 {
            unsafe { f32::from_bits(rd32(x)) }
        }
        #[inline(always)]
        unsafe fn wrf(x: u32, val: f32) {
            unsafe { wr32(x, val.to_bits()) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        /// Exact cvttss2si: out-of-range, infinite and NaN give 0x80000000.
        #[inline(always)]
        fn cvtt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000
            } else {
                (x as i32) as u32
            }
        }
        /// The round-and-fixup idiom (sign/abs/magic/greater-than-fixup), exactly.
        #[inline(always)]
        fn round_ne(x: f32) -> f32 {
            let bits = x.to_bits();
            let sign = bits & SIGN_BIT;
            let abs = f32::from_bits(bits ^ sign);
            let lt = if abs < f32::from_bits(F23) { 0xffff_ffff } else { 0 };
            let mut magic = f32::from_bits(F23 & lt);
            magic = f32::from_bits(magic.to_bits() | sign);
            let mut r = add(x, magic);
            r = sub(r, magic);
            let d = sub(r, x);
            let fix = if d > f32::from_bits(sign) {
                f32::from_bits(ONE_F)
            } else {
                0.0
            };
            sub(r, fix)
        }

        /// One combine repeat: fold frame quat `f0..f3` and vec `f4..f6`
        /// into the outputs. Reads the current outputs, writes them back.
        #[inline(always)]
        unsafe fn combine(q: u32, v: u32, f: &[f32; 8]) {
            unsafe {
                let q0 = rdf(q);
                let q1 = rdf(q + 4);
                let q2 = rdf(q + 8);
                let q3 = rdf(q + 12);
                let mut x5 = mul(q3, f[3]);
                x5 = sub(x5, mul(q0, f[0]));
                x5 = sub(x5, mul(q1, f[1]));
                x5 = sub(x5, mul(q2, f[2]));
                let mut x2 = mul(q0, f[3]);
                x2 = add(x2, mul(q3, f[0]));
                x2 = add(x2, mul(q2, f[1]));
                x2 = sub(x2, mul(q1, f[2]));
                let mut x1 = mul(q1, f[3]);
                x1 = add(x1, mul(q3, f[1]));
                x1 = add(x1, mul(q0, f[2]));
                x1 = sub(x1, mul(q2, f[0]));
                let mut x6 = mul(q2, f[3]);
                x6 = add(x6, mul(q3, f[2]));
                wrf(q + 12, x5);
                wrf(q, x2);
                wrf(q + 4, x1);
                x6 = add(x6, mul(q1, f[0]));
                x6 = sub(x6, mul(q0, f[1]));
                wrf(q + 8, x6);
                wrf(v, add(rdf(v), f[4]));
                wrf(v + 4, add(rdf(v + 4), f[5]));
                wrf(v + 8, add(f[6], rdf(v + 8)));
            }
        }

        /// Tail math after sites F3/F6: core plus the shared tail stores.
        #[inline(always)]
        unsafe fn math_tail(q: u32, v: u32, f: &[f32; 8]) {
            unsafe {
                let q0 = rdf(q);
                let q1 = rdf(q + 4);
                let q2 = rdf(q + 8);
                let q3 = rdf(q + 12);
                let mut x3 = mul(q3, f[3]);
                x3 = sub(x3, mul(q0, f[0]));
                x3 = sub(x3, mul(q1, f[1]));
                x3 = sub(x3, mul(q2, f[2]));
                let x3saved = x3;
                let mut x3b = mul(q0, f[3]);
                x3b = add(x3b, mul(q3, f[0]));
                x3b = add(x3b, mul(q2, f[1]));
                x3b = sub(x3b, mul(q1, f[2]));
                let mut x1 = mul(q1, f[3]);
                x1 = add(x1, mul(q3, f[1]));
                x1 = add(x1, mul(q0, f[2]));
                x1 = sub(x1, mul(q2, f[0]));
                let mut x2 = mul(q2, f[3]);
                x2 = add(x2, mul(q3, f[2]));
                wrf(q, x3b);
                wrf(q + 4, x1);
                x2 = add(x2, mul(q1, f[0]));
                x2 = sub(x2, mul(q0, f[1]));
                wrf(q + 12, x3saved);
                wrf(q + 8, x2);
                wrf(v, add(rdf(v), f[4]));
                wrf(v + 4, add(rdf(v + 4), f[5]));
                wrf(v + 8, add(f[6], rdf(v + 8)));
            }
        }

        let this = ecx;
        let t = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        let lim = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        wr32(q, 0);
        wr32(q + 4, 0);
        wr32(q + 8, 0);
        wr32(q + 12, ONE_F);
        wr32(v, 0);
        wr32(v + 4, 0);
        wr32(v + 8, 0);
        let c = rdf(this + 0x0c);
        let d1 = sub(t, lim);

        if lim > 0.0 {
            if d1 < 0.0 {
                // Path A1.
                let neg = f32::from_bits(d1.to_bits() ^ SIGN_BIT);
                let ans1: f64 = lf_checker_rt::callee_cdecl!(POW1, f64, b, a, q, v);
                let f1 = ans1 as f32;
                let ok1: u8 = lf_checker_rt::callee_thiscall!(
                    F1, u8, this, f1.to_bits(), c.to_bits(), q, v
                );
                if ok1 == 0 {
                    return 0;
                }
                if neg > c {
                    let ab = ans1.to_bits();
                    let mut q2 = [(ab & 0xffff_ffff) as u32,
                                  ((ab >> 32) & 0xffff_ffff) as u32, 0, 0];
                    let mut v2 = [0u32; 4];
                    let ok2: u8 = lf_checker_rt::callee_thiscall!(
                        F2, u8, this, c.to_bits(), c.to_bits(),
                        q2.as_mut_ptr() as u32, v2.as_mut_ptr() as u32
                    );
                    if ok2 == 0 {
                        return 0;
                    }
                    let quot = div(d1, c);
                    let nq = f32::from_bits(quot.to_bits() ^ SIGN_BIT);
                    let n = cvtt(round_ne(nq));
                    let f = [f32::from_bits(q2[0]), f32::from_bits(q2[1]),
                             f32::from_bits(q2[2]), f32::from_bits(q2[3]),
                             f32::from_bits(v2[0]), f32::from_bits(v2[1]),
                             f32::from_bits(v2[2]), f32::from_bits(v2[3])];
                    let mut i = n;
                    while i != 0 {
                        combine(q, v, &f);
                        i = i.wrapping_sub(1);
                    }
                }
            }
            // Paths A1 (after F2 block/skip) and A2 rejoin here.
            let mut x2 = lim;
            if lim > t {
                x2 = t;
            }
            // x2 (XMM2) and t (XMM1) reach site 13 in vector registers the
            // checker cannot observe (dirty upper words); the integers pass.
            let _ = (x2, t);
            let mut q3 = [0u32; 4];
            let mut v3 = [0u32; 4];
            let ok3: u8 = lf_checker_rt::callee_thiscall!(
                F3, u8, this, b, a,
                q3.as_mut_ptr() as u32, v3.as_mut_ptr() as u32
            );
            if ok3 == 0 {
                return 0;
            }
            let f = [f32::from_bits(q3[0]), f32::from_bits(q3[1]),
                     f32::from_bits(q3[2]), f32::from_bits(q3[3]),
                     f32::from_bits(v3[0]), f32::from_bits(v3[1]),
                     f32::from_bits(v3[2]), f32::from_bits(v3[3])];
            math_tail(q, v, &f);
            return 1;
        }

        // Path B.
        if !(lim < 0.0) {
            return 1;
        }
        // B1a runs the power/helper pair; B1b skips straight to site 16.
        if d1 > c {
            let ans2: f64 = lf_checker_rt::callee_cdecl!(POW2, f64, b, a, q, v);
            let f2 = ans2 as f32;
            // neg2 reaches site 14 in XMM2 over three -0.0 words, which the
            // checker cannot observe; site 14 carries XMM1 (= 0) in slot 1.
            let _neg2 = f32::from_bits(f2.to_bits() ^ SIGN_BIT);
            let ok4: u8 = lf_checker_rt::callee_thiscall!(
                F4, u8, this, b, 0u32, q, v
            );
            if ok4 == 0 {
                return 0;
            }
            if sub(d1, c) > c {
                let ab = ans2.to_bits();
                let mut q5 = [(ab & 0xffff_ffff) as u32,
                              ((ab >> 32) & 0xffff_ffff) as u32, 0, 0];
                let mut v5 = [0u32; 4];
                let _negc = f32::from_bits(c.to_bits() ^ SIGN_BIT);
                let ok5: u8 = lf_checker_rt::callee_thiscall!(
                    F5, u8, this, b, 0u32,
                    q5.as_mut_ptr() as u32, v5.as_mut_ptr() as u32
                );
                if ok5 == 0 {
                    return 0;
                }
                let n = cvtt(sub(round_ne(div(d1, c)), 1.0));
                let f = [f32::from_bits(q5[0]), f32::from_bits(q5[1]),
                         f32::from_bits(q5[2]), f32::from_bits(q5[3]),
                         f32::from_bits(v5[0]), f32::from_bits(v5[1]),
                         f32::from_bits(v5[2]), f32::from_bits(v5[3])];
                let mut i = n;
                while i != 0 {
                    combine(q, v, &f);
                    i = i.wrapping_sub(1);
                }
            }
        }
        let mut x0 = sub(t, c);
        if !(x0 > lim) {
            x0 = lim;
        }
        // x0 (XMM0) and t (XMM1) reach site 16 in vector registers the
        // checker cannot observe; the integers pass.
        let _ = (x0, t);
        let mut q6 = [0u32; 4];
        let mut v6 = [0u32; 4];
        let ok6: u8 = lf_checker_rt::callee_thiscall!(
            F6, u8, this, b, a,
            q6.as_mut_ptr() as u32, v6.as_mut_ptr() as u32
        );
        if ok6 == 0 {
            return 0;
        }
        let f = [f32::from_bits(q6[0]), f32::from_bits(q6[1]),
                 f32::from_bits(q6[2]), f32::from_bits(q6[3]),
                 f32::from_bits(v6[0]), f32::from_bits(v6[1]),
                 f32::from_bits(v6[2]), f32::from_bits(v6[3])];
        math_tail(q, v, &f);
        1
    }
});
