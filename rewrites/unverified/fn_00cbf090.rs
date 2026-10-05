// original: 0x00cbf090 CPedMoveBlendOnFoot::vf5

/// Blend the on-foot movement direction into an output vector.
///
/// `this` is the move-blender object; its state block is reached through
/// `+0x24`. `out` receives three floats, `mat` is the blend basis (three
/// rows of three plus a translation row at `+0x20`).
///
/// The state block holds a 3-vector axis at `+0xB00` and three scalars at
/// `+0xE0`/`+0xE4`/`+0xE8`. Two scale factors are formed as
/// `sqrt(max(1 - dot^2, 0))` where the dots project the axis onto the
/// second and first basis rows; a negative radicand clamps to zero while a
/// NaN stays NaN (the original compares `0.0 > t`, which is false for NaN).
/// The output starts from a three-float global triple, then accumulates
/// the basis rows scaled by a callee-driven weight and by the first
/// scalar, and, when bit `0x40` of the flag byte at `this+0x50` is set,
/// the translation row scaled by the third scalar.
///
/// The single callee (a getter on the object at state `+0x78`) is polled
/// up to five times with the same argument: a null first answer, or a
/// second answer whose float at `+0x48` is not above zero, selects the
/// fallback weight (the second scalar); otherwise the weight combines the
/// floats at `+0x48`/`+0x50`/`+0x54` of the later answers with the first
/// two scalars. Returns `out`.
///
/// Original: 0x00cbf090 (thiscall, two stack words). All float arithmetic
/// is in the original's operand order.
lf_checker_rt::export!(thiscall, rw_00cbf090(this: u32, out: u32, mat: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x24;
        const FLAGS_OFF: u32 = 0x50;
        const EXTRA_ROW_BIT: u8 = 0x40;
        const AXIS_X: u32 = 0xb00;
        const AXIS_Y: u32 = 0xb04;
        const AXIS_Z: u32 = 0xb08;
        const S0_OFF: u32 = 0xe0;
        const S1_OFF: u32 = 0xe4;
        const S2_OFF: u32 = 0xe8;
        const INNER_OFF: u32 = 0x78;
        const W_A_OFF: u32 = 0x48;
        const W_B_OFF: u32 = 0x50;
        const W_C_OFF: u32 = 0x54;
        const SEED0: u32 = 0x01b4_b2a0;
        const SEED1: u32 = 0x01b4_b2a4;
        const SEED2: u32 = 0x01b4_b2a8;
        const ONE: f32 = 1.0;
        const ZERO: f32 = 0.0;
        const CALLEE: u32 = 1;

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
        /// sqrt(max(1 - s*s, 0)) with the original's NaN behaviour: only a
        /// strictly negative radicand clamps; NaN passes through to sqrt.
        #[inline(always)]
        fn scale(s: f32) -> f32 {
            let sq = mul(s, s);
            let t = sub(ONE, sq);
            let c = if ZERO > t { ZERO } else { t };
            c.sqrt()
        }

        let state = rd32(this + STATE_OFF);
        let ax = rdf(state + AXIS_X);
        let ay = rdf(state + AXIS_Y);
        let az = rdf(state + AXIS_Z);
        // Projection on the second basis row (mat +0x10/0x14/0x18).
        let t1 = mul(ay, rdf(mat + 0x14));
        let t2 = mul(ax, rdf(mat + 0x10));
        let t1 = add(t1, t2);
        let s1 = add(t1, mul(az, rdf(mat + 0x18)));
        // Projection on the first basis row (mat +0x0/0x4/0x8).
        let u1 = mul(ay, rdf(mat + 4));
        let u2 = mul(ax, rdf(mat));
        let u1 = add(u1, u2);
        let s2 = add(u1, mul(az, rdf(mat + 8)));
        let c1 = scale(s1);
        let c2 = scale(s2);

        wrf(out, (lf_checker_rt::global::<f32>(SEED0) as *const f32).read());
        wrf(out + 4, (lf_checker_rt::global::<f32>(SEED1) as *const f32).read());
        wrf(out + 8, (lf_checker_rt::global::<f32>(SEED2) as *const f32).read());

        let mut e0 = rdf(state + S0_OFF);
        let e1 = rdf(state + S1_OFF);
        let e2 = rdf(state + S2_OFF);
        let inner = rd32(state + INNER_OFF);
        let p1: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, inner);
        let deep = if p1 == 0 {
            false
        } else {
            let p2: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, inner);
            rdf(p2 + W_A_OFF) > ZERO
        };
        let mut k: f32;
        if deep {
            let p3: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, inner);
            let w = rdf(p3 + W_A_OFF);
            let p4: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, inner);
            let rest = sub(ONE, w);
            let m = mul(rdf(p4 + W_B_OFF), w);
            e0 = add(m, mul(rest, e0));
            let p5: u32 = lf_checker_rt::callee_thiscall!(CALLEE, u32, inner);
            k = add(mul(rdf(p5 + W_C_OFF), w), mul(rest, e1));
        } else {
            k = e1;
        }
        k = mul(k, c1);
        let j = mul(e0, c2);

        let seed0 = rdf(out);
        let seed1 = rdf(out + 4);
        let seed2 = rdf(out + 8);
        let k10 = mul(k, rdf(mat + 0x10));
        let k14 = mul(k, rdf(mat + 0x14));
        let k18 = mul(k, rdf(mat + 0x18));
        let mut ox = add(seed0, k10);
        let mut oy = add(seed1, k14);
        let mut oz = add(seed2, k18);
        let j0 = mul(j, rdf(mat));
        let j4 = mul(j, rdf(mat + 4));
        let j8 = mul(j, rdf(mat + 8));
        ox = add(ox, j0);
        oy = add(oy, j4);
        oz = add(oz, j8);
        wrf(out, ox);
        wrf(out + 4, oy);
        wrf(out + 8, oz);

        let flags = ((this + FLAGS_OFF) as *const u8).read();
        if flags & EXTRA_ROW_BIT != 0 {
            let t0 = mul(rdf(mat + 0x20), e2);
            let t1 = mul(rdf(mat + 0x24), e2);
            let t2 = mul(rdf(mat + 0x28), e2);
            ox = add(ox, t0);
            oy = add(oy, t1);
            oz = add(oz, t2);
            wrf(out, ox);
            wrf(out + 4, oy);
            wrf(out + 8, oz);
        }
        out
    }
});
