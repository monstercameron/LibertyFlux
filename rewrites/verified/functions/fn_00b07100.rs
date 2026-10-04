// original: 0x00b07100 cam_pose_integrate (proposed)

/// Camera pose integration step (thiscall, `this` in ECX, no stack arguments,
/// no meaningful return: the single known caller ignores EAX). While a global
/// singleton exists and is active, and unless the early path triggers (which
/// negates two global step/cap words and zeroes five angle/rate slots),
/// advances a clamped blend slot, optionally runs a target-seeking first
/// block, clamps a second angle, applies three chained axis rotations driven
/// by scripted cosine/sine callees, and clamps the result scale.
/// The first block runs only when the tracked slot is non-empty and a status
/// callee is either null or reports zero: it saturates the +0x1DC slot at a
/// global ceiling, forms a direction from published globals minus the position
/// block, normalises it (exact zero yields a zero factor, otherwise the
/// ceiling over the length, NaN propagating), scales it and hands four frame
/// slots to two transform callees (frame pointers skipped, pre-values
/// snapshotted, one float argument compared by value; the transform outputs
/// are never read back).
/// Each rotation block takes cos/sin of one angle slot through xmm0-argument
/// callees (transport-compared) and folds them into two rows of the pose
/// matrix with the original's exact operand order. The tail scales a residual
/// by the +0x60 slot and clamps the slot between two global bounds. Float
/// operation order throughout is the original's.
lf_checker_rt::export!(thiscall, rw_00b07100(this: u32) -> u32 {
    unsafe {
        const P_M10: u32 = 0x10;
        const P_M20: u32 = 0x20;
        const P_M30: u32 = 0x30;
        const P_M40: u32 = 0x40;
        const P_M60: u32 = 0x60;
        const P_TRK: u32 = 0x154;
        const P_A_E0: u32 = 0x1E0;
        const P_A_DC: u32 = 0x1DC;
        const P_A_CC: u32 = 0x1CC;
        const P_A_C8: u32 = 0x1C8;
        const P_A_D0: u32 = 0x1D0;
        const G_CEIL: u32 = 0x00FE88E8;
        const G_DC_STEP: u32 = 0x00FE870C;
        const G_PUB0: u32 = 0x016154C0;
        const G_PUB1: u32 = 0x016154C4;
        const G_PUB2: u32 = 0x016154C8;
        const G_E0_STEP: u32 = 0x0104006C;
        const G_E0_CAP: u32 = 0x01040068;
        const G_K1: u32 = 0x00FE8BB0;
        const G_K2: u32 = 0x00FE8AF4;
        const G_SIGN: u32 = 0x00FE8FA0;
        const SING_ACTIVE: u32 = 0x210;
        const ST_SEL: u32 = 0x328D;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let sing: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if sing == 0 || rd8(sing + SING_ACTIVE) == 0 {
            // Early path: negate the shared step/cap words, park the angles.
            let mask = rd32(lf_checker_rt::relocated(G_SIGN));
            wr32(lf_checker_rt::relocated(G_E0_CAP), rd32(lf_checker_rt::relocated(G_E0_CAP)) ^ mask);
            wr32(lf_checker_rt::relocated(G_E0_STEP), rd32(lf_checker_rt::relocated(G_E0_STEP)) ^ mask);
            wr32(this + P_A_E0, 0);
            wr32(this + P_A_DC, 0);
            wr32(this + P_A_C8, 0);
            wr32(this + P_A_CC, 0);
            wr32(this + P_A_D0, 0);
            return 0;
        }
        let st: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0, 1);
        let ceil = rdf(lf_checker_rt::relocated(G_CEIL));
        if rd32(this + P_TRK) != 0 && (st == 0 || rd8(st + ST_SEL) == 0) {
            // Target-seeking first block.
            let mut x = add(rdf(this + P_A_DC), rdf(lf_checker_rt::relocated(G_DC_STEP)));
            wrf(this + P_A_DC, x);
            if x > ceil {
                x = ceil;
            }
            wrf(this + P_A_DC, x);
            let d3 = sub(rdf(lf_checker_rt::relocated(G_PUB0)), rdf(this + P_M40));
            let m20 = rdf(this + P_M20);
            let d4 = sub(rdf(lf_checker_rt::relocated(G_PUB1)), rdf(this + P_M40 + 4));
            let d5 = sub(rdf(lf_checker_rt::relocated(G_PUB2)), rdf(this + P_M40 + 8));
            let len2 = add(add(mul(d4, d4), mul(d3, d3)), mul(d5, d5));
            // The lahf/test/jp idiom: exact zero (never NaN) takes the
            // zero-factor path.
            let f = if len2 == 0.0 { 0.0 } else { div(ceil, len2.sqrt()) };
            let v20 = mul(d3, f);
            // Buffer in call-argument order: the last-pushed slot (saved
            // +0x20) is argument 0; argument 2 is the clamped slot passed
            // by value; the other pre-values feed slots no call observes.
            let buf = [m20.to_bits(), v20.to_bits(), v20.to_bits()];
            let p0 = (&buf[0] as *const u32) as u32;
            let p1 = (&buf[1] as *const u32) as u32;
            let p3 = (&buf[2] as *const u32) as u32;
            lf_checker_rt::callee_cdecl!(3, u32, p0, p1, x.to_bits(), p3);
            lf_checker_rt::callee_thiscall!(4, u32, this + 0x10, p1);
        }
        // Clamp the +0x1E0 angle after stepping it.
        let e0 = add(rdf(this + P_A_E0), rdf(lf_checker_rt::relocated(G_E0_STEP)));
        wrf(this + P_A_E0, e0);
        let cap = rdf(lf_checker_rt::relocated(G_E0_CAP));
        if e0 > 0.0 {
            if e0 > cap {
                wrf(this + P_A_E0, cap);
            }
        } else if e0 < 0.0 {
            if cap > e0 {
                wrf(this + P_A_E0, cap);
            }
        }
        // Rotation 1: angle +0x1E0 over rows +0x10/+0x30.
        let c1 = f32::from_bits(lf_checker_rt::callee_cdecl!(5, u32, rd32(this + P_A_E0)));
        let s1 = f32::from_bits(lf_checker_rt::callee_cdecl!(6, u32, rd32(this + P_A_E0)));
        let m10 = rdf(this + P_M10);
        let m14 = rdf(this + P_M10 + 4);
        let m18 = rdf(this + P_M10 + 8);
        let m30 = rdf(this + P_M30);
        let m34 = rdf(this + P_M30 + 4);
        let m38 = rdf(this + P_M30 + 8);
        let n10 = sub(mul(m10, c1), mul(m30, s1));
        let n14 = sub(mul(m14, c1), mul(m34, s1));
        let n18 = sub(mul(m18, c1), mul(m38, s1));
        let t30 = mul(m30, c1);
        let t34 = mul(m34, c1);
        let t38 = mul(m38, c1);
        wrf(this + P_M30, add(mul(m10, s1), t30));
        wrf(this + P_M30 + 4, add(mul(m14, s1), t34));
        wrf(this + P_M30 + 8, add(mul(s1, m18), t38));
        wrf(this + P_M10, n10);
        wrf(this + P_M10 + 4, n14);
        wrf(this + P_M10 + 8, n18);
        // Rotation 2: angle +0x1CC over rows +0x10/+0x20.
        let c2 = f32::from_bits(lf_checker_rt::callee_cdecl!(5, u32, rd32(this + P_A_CC)));
        let s2 = f32::from_bits(lf_checker_rt::callee_cdecl!(6, u32, rd32(this + P_A_CC)));
        let m20 = rdf(this + P_M20);
        let m24 = rdf(this + P_M20 + 4);
        let m28 = rdf(this + P_M20 + 8);
        let o10 = add(mul(m20, s2), mul(n10, c2));
        let o14 = add(mul(m24, s2), mul(n14, c2));
        let o18 = add(mul(m28, s2), mul(n18, c2));
        let u20 = mul(m20, c2);
        let u24 = mul(m24, c2);
        let u28 = mul(m28, c2);
        wrf(this + P_M20, sub(u20, mul(n10, s2)));
        wrf(this + P_M20 + 4, sub(u24, mul(n14, s2)));
        wrf(this + P_M20 + 8, sub(u28, mul(s2, n18)));
        wrf(this + P_M10, o10);
        wrf(this + P_M10 + 4, o14);
        wrf(this + P_M10 + 8, o18);
        // Rotation 3: angle +0x1C8 over rows +0x20/+0x30.
        let c3 = f32::from_bits(lf_checker_rt::callee_cdecl!(5, u32, rd32(this + P_A_C8)));
        let s3 = f32::from_bits(lf_checker_rt::callee_cdecl!(6, u32, rd32(this + P_A_C8)));
        let r20 = rdf(this + P_M20);
        let r24 = rdf(this + P_M20 + 4);
        let r28 = rdf(this + P_M20 + 8);
        let r30 = rdf(this + P_M30);
        let r34 = rdf(this + P_M30 + 4);
        let r38 = rdf(this + P_M30 + 8);
        let o20 = add(mul(s3, r30), mul(r20, c3));
        let o24 = add(mul(r34, s3), mul(r24, c3));
        let o28 = add(mul(r38, s3), mul(r28, c3));
        let v30 = mul(c3, r30);
        let v34 = mul(r34, c3);
        let v38 = mul(r38, c3);
        wrf(this + P_M30, sub(v30, mul(r20, s3)));
        wrf(this + P_M30 + 4, sub(v34, mul(r24, s3)));
        wrf(this + P_M30 + 8, sub(v38, mul(r28, s3)));
        let k1 = rdf(lf_checker_rt::relocated(G_K1));
        let k2 = rdf(lf_checker_rt::relocated(G_K2));
        let c0 = rdf(lf_checker_rt::relocated(G_CEIL));
        wrf(this + P_M20, o20);
        wrf(this + P_M20 + 4, o24);
        wrf(this + P_M20 + 8, o28);
        let x = mul(sub(c0, rdf(this + P_A_D0)), rdf(this + P_M60));
        if k1 > x {
            if x > k2 {
                wrf(this + P_M60, x);
            } else {
                wrf(this + P_M60, k2);
            }
        } else {
            wrf(this + P_M60, k1);
        }
        0
    }
});
