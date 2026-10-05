// original: 0x00C9E000 frame_from_lookup_pair (proposed)

/// Look up two objects by one key, difference their trailing triples, and
/// rebuild the first object's leading frame from the normalized difference.
///
/// `this` points to an object whose word at `+0x40` is the lookup key. Each
/// stack argument is looked up with that key through the same callee (which
/// takes the key in `ecx` and pops nothing); call one yields `r1`, call two
/// `r2`. The difference `d` of the triples at `+0x30` in `r2` and `r1` is
/// normalized by the reciprocal of its length, then blended lane-wise with a
/// selector triple: each selector lane is the global gain when the squared
/// length strictly exceeds that lane's threshold global, else zero. The
/// blend is `(v & sel) | (~sel & mask)` over four lanes with the 16-byte
/// mask global, of which lanes 0-2 feed two lane shuffles (all-lane-2 and
/// all-lane-1) crossed with the triple at `r1+0x20` and the pair at
/// `r1+0x28`, `r1+0x2c`. Two normalization guards follow the original's flag
/// tests exactly: a zero squared length normalizes to zero instead of
/// dividing, and the second scale is recomputed only when its squared length
/// differs from the running value (an unordered comparison recomputes).
/// Twelve words are stored into `r1` at `+0x00`..`+0x2c`. The value left in
/// `eax` is `r2` with bits 8-15 replaced by the flags byte the second
/// comparison leaves in `ah` (`0x47` unordered, `0x02` above, `0x03` below,
/// `0x42` equal); that clobbered word is what the function returns.
///
/// Two words the original reads were never written by it (scratch below its
/// frame); the proof fills uninitialized stack with zero, so they read as
/// `0.0` here: the word stored to `r1+0x0c` and the one stored to `r1+0x1c`.
/// A third such word feeds only an unobserved shuffle lane. The float
/// operation order is the original's.
///
/// Original: 0x00C9E000 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00C9E000(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x40;
        const G_GAIN: u32 = 0x17AD148;
        const G_T1: u32 = 0x110DAD8;
        const G_T2: u32 = 0x110DAD4;
        const G_T3: u32 = 0x110DAD0;
        const G_MASK: u32 = 0x110DB50;
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrt(x: f32) -> f32 {
            core::hint::black_box(x).sqrt()
        }

        // Look up the pair.
        let key = rd32(this + KEY_OFF);
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, key, arg1);
        let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, key, arg2);

        // Difference of the trailing triples and its squared length.
        let d3 = sub(rdf(r2 + 0x30), rdf(r1 + 0x30));
        let d4 = sub(rdf(r2 + 0x34), rdf(r1 + 0x34));
        let d6 = sub(rdf(r2 + 0x38), rdf(r1 + 0x38));
        let qlen = add(add(mul(d4, d4), mul(d3, d3)), mul(d6, d6));

        // Selector triple: gain where the squared length strictly exceeds
        // the lane threshold, else zero (NaN length selects zero).
        let gain = rdf(lf_checker_rt::relocated(G_GAIN));
        let sel0 = if qlen > rdf(lf_checker_rt::relocated(G_T3)) {
            gain
        } else {
            0.0
        };
        let sel1 = if qlen > rdf(lf_checker_rt::relocated(G_T2)) {
            gain
        } else {
            0.0
        };
        let sel2 = if qlen > rdf(lf_checker_rt::relocated(G_T1)) {
            gain
        } else {
            0.0
        };

        // Normalized difference.
        let inv = div(ONE, sqrt(qlen));
        let n0 = mul(inv, d3);
        let n1 = mul(inv, d4);
        let n2 = mul(inv, d6);

        // Lane blend of the normalized triple with the selector triple.
        let m0 = rd32(lf_checker_rt::relocated(G_MASK));
        let m1 = rd32(lf_checker_rt::relocated(G_MASK) + 4);
        let m2 = rd32(lf_checker_rt::relocated(G_MASK) + 8);
        let m3 = rd32(lf_checker_rt::relocated(G_MASK) + 12);
        let s0b = sel0.to_bits();
        let s1b = sel1.to_bits();
        let s2b = sel2.to_bits();
        let b0 = (n0.to_bits() & s0b) | (!s0b & m0);
        let b1 = (n1.to_bits() & s1b) | (!s1b & m1);
        let b2 = (n2.to_bits() & s2b) | (!s2b & m2);
        let _b3: u32 = m3; // Lane 3: unwritten scratch feeds it; unobserved.
        let v40 = f32::from_bits(b0);
        let v41 = f32::from_bits(b1);
        let v42 = f32::from_bits(b2);

        // Cross the base triple and pair with the shuffled lanes.
        let f6 = rdf(r1 + 0x20);
        let f1 = rdf(r1 + 0x24);
        let s0 = rdf(r1 + 0x28);
        let t0 = rdf(r1 + 0x2C);
        let mut m7 = sub(mul(f1, v42), mul(s0, v41));
        let m1v = mul(f1, v40);
        let x5 = sub(mul(s0, v40), mul(f6, v42));
        let mut r6 = sub(mul(f6, v41), m1v);
        let r1b = sub(mul(r6, v41), mul(x5, v42));
        let mut q2 = sub(mul(m7, v42), mul(r6, v40));
        let z0 = mul(m7, v41);
        let w2 = sub(mul(x5, v40), z0);
        let w4 = add(add(mul(q2, q2), mul(r1b, r1b)), mul(w2, w2));

        // First guard: zero length normalizes to zero.
        let n1s = if w4 == 0.0 { 0.0 } else { div(ONE, sqrt(w4)) };
        let l0 = mul(w2, n1s);
        q2 = mul(q2, n1s);
        let mut l4 = mul(x5, l0);
        let l2 = mul(r1b, n1s);
        l4 = sub(l4, mul(r6, q2));
        let j0 = mul(m7, l0);
        r6 = mul(r6, l2);
        m7 = mul(m7, q2);
        r6 = sub(r6, j0);
        m7 = sub(m7, mul(x5, l2));

        // Second guard: recompute the scale unless it already equals.
        let sq = add(add(mul(l4, l4), mul(r6, r6)), mul(m7, m7));
        let x5n = if sq == x5 { x5 } else { div(ONE, sqrt(sq)) };
        r6 = mul(r6, x5n);
        m7 = mul(m7, x5n);
        let f0 = mul(l0, r6);
        l4 = mul(l4, x5n);
        let q2f = sub(mul(q2, m7), f0);
        let mut d2 = mul(l0, l4);
        d2 = sub(d2, mul(l2, m7));
        let mut o1 = mul(l2, r6);
        o1 = sub(o1, mul(q2, l4));

        // Store the rebuilt frame into r1.
        wrf(r1, l4);
        wrf(r1 + 4, r6);
        wrf(r1 + 8, m7);
        wrf(r1 + 0x0C, 0.0); // Uninitialized scratch under the proof's fill.
        wrf(r1 + 0x10, q2f);
        wrf(r1 + 0x14, d2);
        wrf(r1 + 0x18, o1);
        wrf(r1 + 0x1C, 0.0); // Uninitialized scratch under the proof's fill.
        wrf(r1 + 0x20, l2);
        wrf(r1 + 0x24, q2);
        wrf(r1 + 0x28, l0);
        wrf(r1 + 0x2C, t0);
        // The second comparison's flags byte survives in ah over the return.
        let ah: u32 = if sq.is_nan() || x5.is_nan() {
            0x47
        } else if sq > x5 {
            0x02
        } else if sq < x5 {
            0x03
        } else {
            0x42
        };
        (r2 & 0xFFFF_00FF) | (ah << 8)
    }
});
