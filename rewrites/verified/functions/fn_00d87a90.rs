// original: 0x00d87a90 ui_pair_measure (proposed)

/// Measure a pair of tracked objects over two orientations, returning the
/// best span as an x87 float.
///
/// `obj1`/`obj2` are trackers whose anchors (`+0x20`, reference point at
/// `+0x30`/`+0x34`) supply the base delta; `f1`/`f2` are scale factors as
/// bit patterns; `p`/`q` point at two-float vectors. Three virtual hooks
/// supply direction pairs: slot `+0x64` on each object and slot `+0x60` on
/// `obj2`. The constants 2.0 (initial best), 1.0 and the sign mask are read
/// from the image's read-only data.
///
/// Algorithm: two loop passes (counter 0 and 1) build a candidate point
/// from `q` with the add/sub roles swapped between passes. Each pass runs
/// two guarded diamonds: the first compares the projected span against the
/// first hook answer (upper branch) or its negation (lower branch) and
/// yields a limit pair; the second does the same for the accumulated span
/// against the second hook answer. A pass whose ordered maximum clears both
/// limits and improves the running best (starting at 2.0) stores it.
/// Unordered (NaN) inputs fail every guard and keep the old best. The two
/// stack slots the original reloads above its frame feed only the dead
/// counter path and are not read here. The result travels back in ST0.
///
/// Original: 0x00D87A90 (cdecl, six stack words; float ST0 result).
lf_checker_rt::export!(cdecl, rw_00d87a90(obj1: u32, obj2: u32, f1b: u32, f2b: u32, p: u32, q: u32) -> f32 {
    unsafe {
        const OBJ_ANCHOR: u32 = 0x20;
        const ANCH_X: u32 = 0x30;
        const ANCH_Y: u32 = 0x34;
        const VT_DIR: u32 = 0x64;
        const VT_ALT: u32 = 0x60;
        const C_TWO: u32 = 0x00fe8a24;
        const C_ONE: u32 = 0x00fe88e8;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn bb(x: f32) -> f32 {
            core::hint::black_box(x)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            bb(a) * bb(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            bb(a) + bb(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            bb(a) - bb(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            bb(a) / bb(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        /// Ordered maximum that keeps the first only when ordered-above.
        #[inline(always)]
        fn pick_max(keep: f32, other: f32) -> f32 {
            if bb(keep) > bb(other) {
                keep
            } else {
                other
            }
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let addr = rd32(rd32(obj) + slot);
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                hook(obj)
            }
        }

        let f1 = f32::from_bits(f1b);
        let f2 = f32::from_bits(f2b);
        let one = rdf(lf_checker_rt::relocated(C_ONE));
        let two = rdf(lf_checker_rt::relocated(C_TWO));
        let a1 = rd32(obj1 + OBJ_ANCHOR);
        let a2 = rd32(obj2 + OBJ_ANCHOR);
        let s40 = sub(rdf(a1 + ANCH_X), rdf(a2 + ANCH_X));
        let s68 = sub(rdf(a1 + ANCH_Y), rdf(a2 + ANCH_Y));
        let r0 = vcall(obj2, VT_DIR);
        let s10 = rdf(r0);
        let s2c = rdf(r0 + 4);
        let r1 = vcall(obj1, VT_DIR);
        let s50 = rdf(r1);
        let s3c = rdf(r1 + 4);
        let r2 = vcall(obj2, VT_ALT);
        let s30 = neg(rdf(r2 + 4));
        let p0 = rdf(p);
        let p1 = rdf(p + 4);
        let q0 = rdf(q);
        let q1 = rdf(q + 4);
        let s34 = sub(mul(p1, f1), mul(p0, f2));
        let s0c = add(mul(p0, f1), mul(p1, f2));

        let mut best = two;
        for pass in 0..2u32 {
            // Candidate point; pass 1 swaps the add/sub roles of pass 0.
            let t6 = mul(q1, s3c);
            let t0 = mul(q1, s50);
            let t6 = add(t6, s68);
            let mut t7 = mul(q0, s3c);
            let t1 = mul(q0, s50);
            t7 = add(t7, s40);
            let (x6, x7) = if pass == 0 {
                (sub(t6, t1), add(t7, t0))
            } else {
                (add(t6, t1), sub(t7, t0))
            };
            // First diamond: projected span vs the first hook answer.
            let t = sub(mul(p1, x7), mul(p0, x6));
            let mut x5 = one;
            let mut x1;
            if bb(t) > bb(s10) {
                x1 = s34;
                if bb(0.0) > bb(s34) {
                    let q = div(one, s34);
                    x1 = neg(mul(sub(t, s10), q));
                    if bb(one) > bb(x1) {
                        let mut x3 = mul(s10, two);
                        x3 = mul(x3, q);
                        x3 = sub(x1, x3);
                        if bb(one) > bb(x3) {
                            x5 = x3;
                        }
                    } else {
                        x1 = one;
                    }
                } else {
                    x5 = one;
                    x1 = one;
                }
            } else if bb(neg(s10)) > bb(t) {
                x1 = s34;
                if bb(s34) > bb(0.0) {
                    let q = div(one, s34);
                    x1 = neg(mul(add(t, s10), q));
                    if bb(one) > bb(x1) {
                        let mut x0 = mul(s10, two);
                        x0 = mul(x0, q);
                        x0 = add(x0, x1);
                        if bb(one) > bb(x0) {
                            x5 = x0;
                        }
                    } else {
                        x1 = one;
                    }
                } else {
                    x5 = one;
                    x1 = one;
                }
            } else if bb(s34) > bb(0.0) {
                x5 = div(sub(s10, t), s34);
                x1 = 0.0;
            } else if bb(0.0) > bb(s34) {
                x5 = neg(div(add(t, s10), s34));
                x1 = 0.0;
            } else {
                x1 = 0.0;
            }
            // Second diamond: accumulated span vs the second hook answer.
            let u = add(mul(p0, x7), mul(p1, x6));
            let mut x4 = one;
            let mut x3 = 0.0;
            if bb(u) > bb(s2c) {
                // The original sets x3 to one before this split.
                x3 = one;
                if !(bb(0.0) > bb(s0c)) {
                    x4 = one;
                } else {
                    let q2 = div(one, s0c);
                    x3 = neg(mul(sub(u, s2c), q2));
                    if bb(one) > bb(x3) {
                        let x0 = mul(add(s30, s2c), q2);
                        let x2 = sub(x3, x0);
                        if bb(one) > bb(x2) {
                            x4 = x2;
                        }
                    } else {
                        x3 = one;
                    }
                }
            } else if bb(neg(s30)) > bb(u) {
                if bb(s0c) > bb(x3) {
                    let q3 = div(one, s0c);
                    x3 = neg(mul(add(u, s30), q3));
                    if bb(one) > bb(x3) {
                        let x0 = add(mul(add(s30, s2c), q3), x3);
                        if bb(one) > bb(x0) {
                            x4 = x0;
                        }
                    } else {
                        x3 = one;
                    }
                } else {
                    x4 = one;
                    x3 = one;
                }
            } else if bb(s0c) > bb(0.0) {
                x4 = div(sub(s2c, u), s0c);
            } else if bb(0.0) > bb(s0c) {
                x4 = neg(div(add(u, s30), s0c));
            }
            // Combine: ordered max against both limits and the best.
            x1 = pick_max(x1, x3);
            if bb(x5) > bb(x1) && bb(x4) > bb(x1) && !(bb(x1) > bb(best)) {
                best = x1;
            }
        }
        best
    }
});
