// original: 0x00d87ef0 ui_span_measure (proposed)

/// Measure the closest approach over four orientations, returning the best
/// span as an x87 float.
///
/// `obj1` (`+0x20` anchor with the reference point at `+0x30`/`+0x34`) and
/// `obj2` (same layout) are the two tracked objects; `f1`/`f2` are scale
/// factors passed as bit patterns; `p`/`q` point at two-float vectors. Three
/// virtual hooks supply direction pairs: slot `+0x64` on each object and
/// slot `+0x60` on `obj1`. The initial best (2.0) is read from the image's
/// read-only constants.
///
/// Algorithm: combine the anchor points with the hook answers and `p` into
/// four accumulators, then run four orientation cases (selected by a loop
/// counter 0..3) that each build a candidate point from `q`. A candidate
/// closer than the running best in the projected metric replaces it, subject
/// to three ordered guards (positive projection, negative cross term,
/// negative product term); unordered (NaN) inputs fail every guard and keep
/// the old best. The two stack slots above the frame that the original
/// reloads are never consumed on a live path and are not read here. The
/// result travels back in ST0.
///
/// Original: 0x00D87EF0 (cdecl, six stack words; float ST0 result).
lf_checker_rt::export!(cdecl, rw_00d87ef0(obj1: u32, obj2: u32, f1b: u32, f2b: u32, p: u32, q: u32) -> f32 {
    unsafe {
        const OBJ_ANCHOR: u32 = 0x20;
        const ANCH_X: u32 = 0x30;
        const ANCH_Y: u32 = 0x34;
        const VT_DIR: u32 = 0x64;
        const VT_ALT: u32 = 0x60;
        const C_TWO: u32 = 0x00fe8a24;
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
        let o2a = rd32(obj2 + OBJ_ANCHOR);
        let o1a = rd32(obj1 + OBJ_ANCHOR);
        let o2ax = rdf(o2a + ANCH_X);
        let o2ay = rdf(o2a + ANCH_Y);
        let o1ax = rdf(o1a + ANCH_X);
        let o1ay = rdf(o1a + ANCH_Y);
        let r0 = vcall(obj2, VT_DIR);
        let r0x = rdf(r0);
        let r0y = rdf(r0 + 4);
        let r1 = vcall(obj1, VT_DIR);
        let r1x = rdf(r1);
        let r1y = rdf(r1 + 4);
        let r2 = vcall(obj1, VT_ALT);
        let nr2y = neg(rdf(r2 + 4));
        let p0 = rdf(p);
        let p1 = rdf(p + 4);
        let q0 = rdf(q);
        let q1 = rdf(q + 4);

        let t6 = add(mul(p0, r0y), o2ax);
        let t1 = mul(p1, r0x);
        let s60 = add(t1, t6);
        let s90 = sub(t6, t1);
        let u0 = add(mul(p1, r0y), o2ay);
        let u2 = mul(p0, r0x);
        let s40 = sub(u0, u2);
        let s80 = add(u2, u0);

        let mut best = rdf(lf_checker_rt::relocated(C_TWO));
        let mut case = 0u32;
        loop {
            let (x6, x5) = match case {
                0 => {
                    let x0 = mul(q1, r1x);
                    let x1t = mul(q0, r1x);
                    let mut x6 = mul(q0, r1y);
                    let mut x5 = mul(q1, r1y);
                    x6 = add(x6, o1ax);
                    x5 = add(x5, o1ay);
                    x6 = add(x6, x0);
                    x5 = sub(x5, x1t);
                    (x6, x5)
                }
                1 => {
                    let x0 = mul(q1, r1x);
                    let mut x5 = mul(q1, r1y);
                    let x1t = mul(q0, r1x);
                    x5 = add(x5, o1ay);
                    let mut x6 = mul(q0, r1y);
                    x6 = add(x6, o1ax);
                    x5 = add(x5, x1t);
                    x6 = sub(x6, x0);
                    (x6, x5)
                }
                2 => {
                    let x0 = mul(q0, nr2y);
                    let x2t = mul(q0, r1x);
                    let mut x6 = sub(o1ax, x0);
                    let x1t = mul(q1, nr2y);
                    let x0b = mul(q1, r1x);
                    let mut x5 = sub(o1ay, x1t);
                    x6 = add(x6, x0b);
                    x5 = sub(x5, x2t);
                    (x6, x5)
                }
                _ => {
                    let x0 = mul(q0, nr2y);
                    let x2t = mul(q0, r1x);
                    let mut x6 = sub(o1ax, x0);
                    let x1t = mul(q1, nr2y);
                    let x0b = mul(q1, r1x);
                    let mut x5 = sub(o1ay, x1t);
                    x5 = add(x5, x2t);
                    x6 = sub(x6, x0b);
                    (x6, x5)
                }
            };
            let mut t0 = sub(x6, s60);
            let mut t3 = sub(x5, s40);
            let mut t1 = add(x6, f1);
            t0 = mul(t0, p0);
            t3 = mul(t3, p1);
            t1 = sub(t1, s60);
            let mut t4 = add(x5, f2);
            t3 = add(t3, t0);
            t1 = mul(t1, p0);
            t4 = sub(t4, s40);
            if !(bb(t3) > bb(0.0)) {
                case += 1;
                if case >= 4 {
                    break;
                }
                continue;
            }
            t4 = mul(t4, p1);
            t4 = add(t4, t1);
            if !(bb(0.0) > bb(t4)) {
                case += 1;
                if case >= 4 {
                    break;
                }
                continue;
            }
            let mut u0 = sub(s80, x5);
            let mut u2 = sub(s90, x6);
            let mut u1 = sub(s60, x6);
            u0 = mul(u0, f1);
            u2 = mul(u2, f2);
            u1 = mul(u1, f2);
            u2 = sub(u2, u0);
            u0 = sub(s40, x5);
            u0 = mul(u0, f1);
            u1 = sub(u1, u0);
            u2 = mul(u2, u1);
            if !(bb(0.0) > bb(u2)) {
                case += 1;
                if case >= 4 {
                    break;
                }
                continue;
            }
            let d = sub(t3, t4);
            let cand = div(t3, d);
            if !(bb(cand) > bb(best)) {
                best = cand;
            }
            case += 1;
            if case >= 4 {
                break;
            }
        }
        best
    }
});
