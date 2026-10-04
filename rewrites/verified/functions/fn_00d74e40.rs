// original: 0x00d74e40 ui_rotor_blend_4tap (proposed)

/// Blend a 4-tap rotation table from two input vectors and emit it through a
/// downstream sink, with data-dependent normalize/clamp/angle stages.
///
/// `this` points at the owner object (dword at `+0xC` selects the first sink
/// call; dword at `+0x10` the second). `a1`/`a2` point at two floats each,
/// `a3` (edi) at floats read at `+0x20/+0x24/+0x40/+0x44`. A lock prologue
/// and two 0-argument query calls (each answer selects one of two global
/// int constants, converted to float) scale a float fetched through a
/// scripted global index table. A 4-argument mix call takes a frame pointer
/// and fills two words. `cos(0)`/`sin(0)` seed rotation loop 1, which folds
/// four (+/-1) constant pairs with the seeds into the 8-word table `t`.
/// When `[this+0xC]` is nonzero, a thiscall plus a 19-argument big call
/// (table by value, nine constants, frame pointer, zero) emit `t`; the big
/// callee only reads through the pointer. The middle stages measure the
/// (a3-vs-sink) distance, clamp it to [0,1], normalize three vector pairs
/// (`x == 0 ? 0 : 1/sqrt(x)`, NaN/negative yield NaN), take an arccosine
/// with a sign branch, and combine everything through sin/cos into fresh
/// seeds for rotation loop 2, which rewrites `t`. When `[this+0x10]` is
/// nonzero the thiscall plus big call emit `t` again. All clamps keep NaN
/// (ordered-comparison form); float operation order is the original's.
/// Returns 32 when the second branch is skipped, else the last answer.
///
/// Original: 0x00d74e40 (thiscall, this + three stack words; returns `eax`).
lf_checker_rt::export!(thiscall, rw_00d74e40(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const Q1: u32 = 2;
        const Q2: u32 = 3;
        const MIX: u32 = 4;
        const COS: u32 = 5;
        const SIN: u32 = 6;
        const REL: u32 = 7;
        const BIG1: u32 = 8;
        const FLT: u32 = 9;
        const ANG: u32 = 10;
        const BIG2: u32 = 11;
        const COOKIE: u32 = 12;
        const ONE_BITS: u32 = 0x3f80_0000;

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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn negate_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() ^ 0x8000_0000)
        }
        /// Ordered clamp with NaN passthrough (the original's comiss/jbe pair).
        #[inline(always)]
        fn clamp1(x: f32, lo: f32, hi: f32) -> f32 {
            if x < lo {
                lo
            } else if x > hi {
                hi
            } else {
                x
            }
        }
        /// The lahf/test/jp idiom: zero (either sign) maps to +0, everything
        /// else goes through 1/sqrt (NaN and negatives yield NaN).
        #[inline(always)]
        fn inv_sqrt_or_zero(x: f32) -> f32 {
            if x == 0.0 {
                0.0
            } else {
                div(1.0, x.sqrt())
            }
        }
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd(a)) }
        }
        #[inline(always)]
        unsafe fn big(id: u32, t: &[f32; 8], frame: *mut u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_cdecl!(id, u32,
                    t[0].to_bits(), t[1].to_bits(), t[2].to_bits(), t[3].to_bits(),
                    t[4].to_bits(), t[5].to_bits(), t[6].to_bits(), t[7].to_bits(),
                    0u32, 0u32, ONE_BITS, 0u32, 0u32, ONE_BITS, ONE_BITS, ONE_BITS, 0u32,
                    frame as u32, 0u32)
            }
        }

        let half = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE8830));
        let quarter = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE87E4));
        let one = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE88E8));
        let neg1 = f32::from_bits(*lf_checker_rt::global::<u32>(0x00FE8D94));
        let k26 = f32::from_bits(*lf_checker_rt::global::<u32>(0x00EEBF0C));

        lf_checker_rt::callee_cdecl!(LOCK, u32, 0xAu32, 0u32);
        lf_checker_rt::callee_cdecl!(LOCK, u32, 7u32, 0u32);

        let a2f0 = rdf(a2);
        let a2f1 = rdf(a2.wrapping_add(4));
        let x1 = mul(a2f0, half);
        let x0 = mul(a2f1, half);
        let seed140 = add(rdf(a1), x1);
        let seed36 = add(rdf(a1.wrapping_add(4)), x0);

        let idx = *lf_checker_rt::global::<u32>(0x0118EA24);
        let tbase = lf_checker_rt::relocated(0x0118E7F8);
        let entry = rd(tbase.wrapping_add(idx.wrapping_mul(4)));
        let e24 = rdf(entry.wrapping_add(0x24));

        let q1: u32 = lf_checker_rt::callee_cdecl!(Q1, u32,);
        let c1 = if q1 & 0xFF != 0 {
            *lf_checker_rt::global::<u32>(0x0105C888)
        } else {
            *lf_checker_rt::global::<u32>(0x0105C884)
        };
        let e68 = mul(c1 as i32 as f32, e24);
        let q2: u32 = lf_checker_rt::callee_cdecl!(Q2, u32,);
        let c2 = if q2 & 0xFF != 0 {
            *lf_checker_rt::global::<u32>(0x0105C87C)
        } else {
            *lf_checker_rt::global::<u32>(0x0105C880)
        };
        let e72 = mul(c2 as i32 as f32, e24);

        let mut mix = [e68.to_bits(), e72.to_bits()];
        lf_checker_rt::callee_cdecl!(MIX, u32, 2u32, 0u32, mix.as_mut_ptr() as u32, 0u32);
        let e68w = f32::from_bits(mix[0]);
        let e72w = f32::from_bits(mix[1]);

        let cos0 = f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, 0u32));
        let sin0 = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, 0u32));
        let e76 = mul(e68w, quarter);
        let m7 = mul(e72w, quarter);

        const CONSTS: [f32; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
        let mut t = [0.0f32; 8];
        for j in 0..4usize {
            let c1v = CONSTS[2 * j];
            let c2v = CONSTS[2 * j + 1];
            let t1 = sub(mul(c1v, cos0), mul(c2v, sin0));
            let t3 = add(mul(c1v, sin0), mul(c2v, cos0));
            t[2 * j] = add(mul(t1, e76), seed140);
            t[2 * j + 1] = add(mul(t3, m7), seed36);
        }

        if rd(this.wrapping_add(0xC)) != 0 {
            lf_checker_rt::callee_thiscall!(REL, u32, this.wrapping_add(0xC));
            let mut frame = [
                0xFFFF_FFFFu32, seed140.to_bits(), this,
                t[0].to_bits(), t[1].to_bits(), t[2].to_bits(), t[3].to_bits(),
                t[4].to_bits(), t[5].to_bits(), t[6].to_bits(), t[7].to_bits(),
            ];
            big(BIG1, &t, frame.as_mut_ptr());
        }

        let gobj = *lf_checker_rt::global::<u32>(0x011F70FC);
        let esi2 = rd(gobj.wrapping_add(0x20));
        let dy = sub(rdf(a3.wrapping_add(0x44)), rdf(esi2.wrapping_add(0x34)));
        let dx = sub(rdf(a3.wrapping_add(0x40)), rdf(esi2.wrapping_add(0x30)));
        let dx2 = mul(dx, dx);
        let len2 = add(mul(dy, dy), dx2);
        let len = len2.sqrt();
        let c30: f32 = lf_checker_rt::callee_cdecl!(FLT, f32,);
        let e48c = clamp1(div(len, c30), 0.0, one);

        let s10 = rdf(esi2.wrapping_add(0x10));
        let s14 = rdf(esi2.wrapping_add(0x14));
        let n1 = inv_sqrt_or_zero(add(mul(s14, s14), mul(s10, s10)));
        let e84 = mul(s14, n1);
        let e88 = mul(s10, n1);
        let e128 = mul(n1, 0.0);

        let d20 = rdf(a3.wrapping_add(0x20));
        let d24 = rdf(a3.wrapping_add(0x24));
        let n2 = inv_sqrt_or_zero(add(mul(d24, d24), mul(d20, d20)));
        let e8b = mul(d20, n2);
        let e76b = mul(d24, n2);
        let e112 = mul(n2, 0.0);

        let dyneg = negate_bits(dy);
        let n3 = inv_sqrt_or_zero(add(mul(dyneg, dyneg), dx2));
        let e40 = mul(n3, dx);
        let e60 = mul(dyneg, n3);

        let z1 = mul(s10, 0.0);
        let z2 = mul(n1, 0.0);
        let c2v = clamp1(add(add(z1, e84), z2), neg1, one);

        let ang = f32::from_bits(lf_checker_rt::callee_cdecl!(ANG, u32, c2v.to_bits()));
        let d0 = sub(e88, mul(e84, 0.0));
        let e80 = if d0 > 0.0 { mul(ang, neg1) } else { ang };

        let sin_a = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, e80.to_bits()));
        let cos_a = f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, e80.to_bits()));

        let i1 = mul(e60, sin_a);
        let i3 = mul(e60, cos_a);
        let i5 = mul(e40, sin_a);
        let i4 = mul(e40, cos_a);
        let k0 = mul(rdf(a2), half);
        let f4 = sub(i4, i1);
        let g1 = mul(e76b, e84);
        let k = mul(k0, e48c);
        let f3 = add(i3, i5);
        let e124b = add(mul(f4, k), seed140);
        let h3 = mul(f3, k);
        let m0 = mul(e8b, e88);
        let e60c = add(h3, seed36);
        let g1c = add(add(g1, m0), mul(e112, e128));

        let ac2in = clamp1(g1c, neg1, one);
        let ac2 = f32::from_bits(lf_checker_rt::callee_cdecl!(ANG, u32, ac2in.to_bits()));
        let dd = sub(mul(e76b, e88), mul(e8b, e84));
        let e36b = if dd > 0.0 { mul(ac2, neg1) } else { ac2 };

        let e112b = mul(e68w, k26);
        let cos_b = f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, e36b.to_bits()));
        let sin_b = f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, e36b.to_bits()));
        let e40c = mul(e72w, k26);

        let mut t2 = [0.0f32; 8];
        for j in 0..4usize {
            let c1v = CONSTS[2 * j];
            let c2v = CONSTS[2 * j + 1];
            let t1 = sub(mul(c1v, cos_b), mul(c2v, sin_b));
            let t3 = add(mul(c1v, sin_b), mul(c2v, cos_b));
            t2[2 * j] = add(mul(t1, e112b), e124b);
            t2[2 * j + 1] = add(mul(t3, e40c), e60c);
        }

        let mut ret = 32u32;
        let b2ptr = this.wrapping_add(0x10);
        if rd(b2ptr) != 0 {
            lf_checker_rt::callee_thiscall!(REL, u32, b2ptr);
            let mut frame = [
                0xFFFF_FFFFu32, seed140.to_bits(), this,
                t2[0].to_bits(), t2[1].to_bits(), t2[2].to_bits(), t2[3].to_bits(),
                t2[4].to_bits(), t2[5].to_bits(), t2[6].to_bits(), t2[7].to_bits(),
            ];
            ret = big(BIG2, &t2, frame.as_mut_ptr());
        }
        lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        ret
    }
});
