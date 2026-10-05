// original: 0x009AB530 audio_range_gated_response (proposed)

/// Range-gated audio response: combines the entity's spatial parameters with
/// a two-float probe point, gates the result through two threshold chains,
/// and on the open path feeds a derived value to a curve evaluator.
///
/// `this` points to the weather audio entity; the probe `p` points to two
/// floats (P0, P1). The entity fields used are the base pair at +0xd20/+0xd24
/// (V0, V1), the weight triple at +0xd30/+0xd34/+0xd38 (W0, W1, W2), the bias
/// at +0xd28 (B), the second triple at +0xd40/+0xd44/+0xd48 (K0, K1, K2), the
/// output scale at +0xd50 (S), the divisor pair at +0xd90/+0xd98, and the
/// enable byte at +0xd84.
///
/// Behaviour: when the enable byte is clear the result is +0.0. Otherwise
/// the signed energy E = W0*(V0-P0) + W1*(V1-P1) + W2*B is formed and
/// negated; three global thresholds (G0..G2) are each compared against -E
/// and the path closes (result +0.0) unless one of them is below. On the
/// open path a second chain compares the same thresholds against
/// (W0^2+W1^2+W2^2)+E, closing again unless one is below. When both chains
/// stay open, normalised coordinates are derived with three divisions by
/// the recovered sum of squares, a quadratic form over them selects a
/// divisor, and the square root of that form divided by the divisor (or,
/// on the other branch, divided then negated) is passed with `this`+0xd5c
/// to the curve evaluator; the answer times S is the result.
///
/// All floating-point comparisons treat unordered (NaN) as below, matching
/// the original's `comiss`+`jb`/`jbe` pairs, and every operation keeps the
/// original's operand order. The result is returned on the x87 stack.
///
/// Original: 0x009AB530 (thiscall, one stack argument, float result).
lf_checker_rt::export!(thiscall, rw_009ab530(this: u32, p: u32) -> f32 {
    unsafe {
        const ENABLE: u32 = 0xd84;
        const CURVE_THIS: u32 = 0xd5c;
        const G0: u32 = 0x0110DB40;
        const G1: u32 = 0x0110DB44;
        const G2: u32 = 0x0110DB48;
        const SIGN: u32 = 0x8000_0000;
        const CURVE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }
        /// `comiss(a, b)` followed by `jb`: true when a is below b or the
        /// two are unordered.
        #[inline(always)]
        fn below(a: f32, b: f32) -> bool {
            !(a >= b)
        }
        /// `comiss(a, 0.0)` followed by `jbe`: true when a is not positive.
        #[inline(always)]
        fn not_positive(a: f32) -> bool {
            !(a > 0.0)
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
        }

        if (this.wrapping_add(ENABLE) as *const u8).read() == 0 {
            return 0.0;
        }
        let p0 = rdf(p);
        let p1 = rdf(p.wrapping_add(4));
        let dx = sub(rdf(this.wrapping_add(0xd20)), p0);
        let dy = sub(rdf(this.wrapping_add(0xd24)), p1);
        let w0 = rdf(this.wrapping_add(0xd30));
        let w1 = rdf(this.wrapping_add(0xd34));
        let w2 = rdf(this.wrapping_add(0xd38));
        let bias = rdf(this.wrapping_add(0xd28));
        // Accumulated as ((w1*dy + w0*dx) + w2*bias), in this order: for
        // NaN inputs the destination operand's payload wins.
        let energy = add(add(mul(w1, dy), mul(w0, dx)), mul(w2, bias));
        let neg_energy = neg(energy);
        if below(glob(G0), neg_energy)
            || below(glob(G1), neg_energy)
            || below(glob(G2), neg_energy)
        {
            let sq = add(add(mul(w1, w1), mul(w0, w0)), mul(w2, w2));
            let dist = sub(sq, neg_energy);
            if below(glob(G0), dist) || below(glob(G1), dist) || below(glob(G2), dist) {
                let rec = add(dist, neg_energy);
                let q0 = div(neg_energy, rec);
                let q1 = div(neg_energy, rec);
                let q2 = div(neg_energy, rec);
                let c4 = add(mul(q0, w0), dx);
                let c3 = add(mul(q1, w1), dy);
                let mut c5 = mul(w2, q2);
                c5 = add(c5, bias);
                let j4 = add(c4, p0);
                let j3 = add(c3, p1);
                // Quadratic form selecting the divisor.
                let s0 = sub(p1, rdf(this.wrapping_add(0xd24)));
                let t3 = sub(j3, p1);
                let mut s1 = rdf(this.wrapping_add(0xd44));
                let nbias = neg(bias);
                let mut s2 = sub(p0, rdf(this.wrapping_add(0xd20)));
                s1 = mul(s1, s0);
                let mut s0b = rdf(this.wrapping_add(0xd48));
                let t4 = sub(j4, p0);
                s2 = mul(s2, rdf(this.wrapping_add(0xd40)));
                let t3sq = mul(t3, t3);
                let t4sq = mul(t4, t4);
                s0b = mul(s0b, nbias);
                s1 = add(s1, s2);
                let t34 = add(t3sq, t4sq);
                c5 = mul(c5, c5);
                s1 = add(s1, s0b);
                let form = add(t34, c5);
                let root = core::hint::black_box(form).sqrt();
                let arg = if not_positive(s1) {
                    neg(div(root, rdf(this.wrapping_add(0xd98))))
                } else {
                    div(root, rdf(this.wrapping_add(0xd90)))
                };
                let ans: f32 = lf_checker_rt::callee_thiscall!(
                    CURVE_CALLEE,
                    f32,
                    this.wrapping_add(CURVE_THIS),
                    arg.to_bits()
                );
                return mul(ans, rdf(this.wrapping_add(0xd50)));
            }
        }
        0.0
    }
});
