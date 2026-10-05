// original: 0x009733A0 audio_spatial_update (proposed)

/// Refresh the listener-relative reverb description of an audio object.
///
/// `this` points to a large audio object. After asking the sibling update
/// (callee 1) to refresh it, the function reads the calling thread's audio
/// row selector from thread-local storage, loads one 64-byte row of nine
/// matrix coefficients plus a reference point from the audio table, and
/// transforms each of the `count` sources at `this + COUNT` through it.
///
/// For every source `i` the function forms the offset of the source position
/// (`SRC_POS`, stride 16) from the row's reference point, multiplies it by
/// the row's 3x3 matrix (rows `ROW_M0..M2`, add order `(m1*y + m0*x) + m2*z`
/// per output), normalises the result by its length, and blends each lane
/// with the constant vector at `BLEND_VEC` under a per-lane mask that is
/// `ONE_ALT` when the squared length exceeds `LENGTH_EPS` and zero otherwise
/// (unordered comparisons take the zero side). The blended direction is
/// normalised a second time under the same rule, the dot product of the two
/// directions is clamped to `[-0.01, 1.01]` (anything outside, or unordered,
/// becomes zero), and the source's distance attenuation is formed as the
/// product of two filter answers (callee 2): one for the first length and one
/// for the source gain at `SRC_GAIN`.
///
/// The second phase runs four passes over the diagonal directions
/// `(+-S, +-S, 0)` with `S = 0.7071068`. Each pass scores every direction by
/// its dot with the pass direction (negative scores become zero, the first
/// source scores exactly one), mixes in the clamped dot above as
/// `(0.25 * (1 - dot) + dot * score)` times the source's attenuation
/// product, accumulates the mix weights, then adds
/// the `VEC` positions (`VEC_BASE`, stride 12) weighted by mix over total
/// into one triple of the twelve output accumulators. Each pass then pushes
/// its triple through the shaping filter (callee 3, second argument the
/// `FILTER_ARG` global) scaled by `OUT_SCALE` into `OUT_BASE`. The function
/// finally notifies the global reverb registry (callee 4) and returns its
/// answer.
///
/// Two stack slots hold indeterminate values (one never-written word and the
/// high lanes of three mask vectors); the worker fills them with zero, and
/// they reach only dead stores, so the rewrite does not model them.
///
/// Original: 0x009733A0 (thiscall, no stack arguments, returns callee 4's
/// answer in eax; the trailing security-cookie call is intercepted as
/// callee 5 and preserves every register).
unsafe fn audio_spatial_body(this: u32, do_clamp: bool) -> u32 {
    unsafe {
        const COUNT: u32 = 0x1E80;
        const SRC_GAIN: u32 = 0x1EC4;
        const SRC_POS: u32 = 0x1F10;
        const VEC_BASE: u32 = 0x2010;
        const OUT_BASE: u32 = 0x20E4;
        const FILTER_THIS_A: u32 = 0x21B4;
        const FILTER_THIS_B: u32 = 0x218C;
        const SHAPE_THIS: u32 = 0x21DC;
        const SHAPE_STRIDE: u32 = 0x1C;
        const AUDIO_TABLE: u32 = 0x0115_E3F0;
        const TLS_INDEX_GV: u32 = 0x017A_BA14;
        const TLS_AUDIO_OFF: u32 = 0x70;
        const ONE_GV: u32 = 0x00FE_88E8;
        const ONE_ALT_GV: u32 = 0x017A_D148;
        const LENGTH_EPS_A: u32 = 0x0110_DAD8;
        const LENGTH_EPS_B: u32 = 0x0110_DAD4;
        const LENGTH_EPS_C: u32 = 0x0110_DAD0;
        const BLEND_VEC_GV: u32 = 0x0110_DB50;
        const FILTER_ARG_GV: u32 = 0x0116_18FC;
        const OUT_SCALE_GV: u32 = 0x0103_7A04;
        const REGISTRY_THIS: u32 = 0x0115_DEF0;
        const NEG_HUNDREDTH: f32 = f32::from_bits(0xBC23_D70A); // -0.01
        const ONE_POINT_OH_ONE: f32 = f32::from_bits(0x3F81_47AE); // 1.01
        const QUARTER: f32 = 0.25;
        const DIAG: f32 = f32::from_bits(0x3F35_0481); // 0.7071068
        const CALLEE_SIBLING: u32 = 1;
        const CALLEE_FILTER: u32 = 2;
        const CALLEE_SHAPE: u32 = 3;
        const CALLEE_NOTIFY: u32 = 4;
        const CALLEE_COOKIE: u32 = 5;

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
        unsafe fn gv_f(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(addr))) }
        }
        #[inline(always)]
        unsafe fn gv_32(addr: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(addr)) }
        }

        // Callee 1 refreshes the object, including the source count.
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SIBLING, u32, this);
        let count = rd32(this.wrapping_add(COUNT)) as usize;

        // Thread-local audio row selector.
        let tls_index = gv_32(TLS_INDEX_GV);
        let tls = lf_checker_rt::tls_slot(tls_index as usize);
        let sel = rd32(tls.wrapping_add(TLS_AUDIO_OFF));
        let row = lf_checker_rt::relocated(AUDIO_TABLE).wrapping_add(sel.wrapping_shl(6));
        let rn = |w: u32| rdf(row.wrapping_add(w.wrapping_mul(4)));
        let (m0, m1, m2) = (rn(0), rn(1), rn(2));
        let (m4, m5, m6) = (rn(4), rn(5), rn(6));
        let (m8, m9, m10) = (rn(8), rn(9), rn(10));
        let (rx, ry, rz) = (rn(12), rn(13), rn(14));

        let one = gv_f(ONE_GV);
        let one_alt = gv_f(ONE_ALT_GV);
        let eps_a = gv_f(LENGTH_EPS_A);
        let eps_b = gv_f(LENGTH_EPS_B);
        let eps_c = gv_f(LENGTH_EPS_C);
        let blend0 = gv_32(BLEND_VEC_GV);
        let blend1 = gv_32(BLEND_VEC_GV.wrapping_add(4));
        let blend2 = gv_32(BLEND_VEC_GV.wrapping_add(8));
        // `jbe` after `comiss v, t`: taken (zero side) unless v is ordered
        // and above t.
        let pick = |v: f32, t: f32| {
            if !(v > t) {
                0.0f32
            } else {
                one_alt
            }
        };

        let mut dir = [[0.0f32; 3]; 16];
        let mut dot = [0.0f32; 16];
        let mut prod = [0.0f32; 16];

        for i in 0..count {
            let s = this.wrapping_add(SRC_POS).wrapping_add((i as u32).wrapping_mul(16));
            let dx = sub(rdf(s), rx);
            let dy = sub(rdf(s.wrapping_add(4)), ry);
            let dz = sub(rdf(s.wrapping_add(8)), rz);
            let o0 = add(add(mul(m1, dy), mul(m0, dx)), mul(m2, dz));
            let o1 = add(add(mul(m5, dy), mul(m4, dx)), mul(m6, dz));
            let o2 = add(add(mul(m9, dy), mul(m8, dx)), mul(m10, dz));
            let n2 = add(add(mul(o1, o1), mul(o0, o0)), mul(o2, o2));
            let r = core::hint::black_box(n2).sqrt();
            let (sa, sb, sc) = (pick(n2, eps_a), pick(n2, eps_b), pick(n2, eps_c));
            let inv = div(one, r);
            let (n0, n1, n2v) = (mul(o0, inv), mul(o1, inv), mul(o2, inv));
            let (ma, mb, mc) = (sa.to_bits(), sb.to_bits(), sc.to_bits());
            let l0 = f32::from_bits((n0.to_bits() & mc) | (!mc & blend0));
            let l1 = f32::from_bits((n1.to_bits() & mb) | (!mb & blend1));
            let l2 = f32::from_bits((n2v.to_bits() & ma) | (!ma & blend2));
            let m2 = add(add(mul(l1, l1), mul(l0, l0)), mul(l2, l2));
            let (sd, se) = (pick(m2, eps_a), pick(m2, eps_b));
            // `ja` after `comiss m2, t`: taken (one side) only when ordered
            // and above.
            let sf = if m2 > eps_c { one_alt } else { 0.0f32 };
            let r2 = core::hint::black_box(m2).sqrt();
            let inv2 = div(one, r2);
            let (g0v, g1v, g2v) = (mul(l0, inv2), mul(l1, inv2), mul(l2, inv2));
            let (md, me, mf) = (sd.to_bits(), se.to_bits(), sf.to_bits());
            let g0 = f32::from_bits((g0v.to_bits() & mf) | (!mf & blend0));
            let g1 = f32::from_bits((g1v.to_bits() & me) | (!me & blend1));
            let g2 = f32::from_bits((g2v.to_bits() & md) | (!md & blend2));
            let d = add(add(mul(g1, l1), mul(g0, l0)), mul(g2, l2));
            dot[i] = if do_clamp {
                if NEG_HUNDREDTH > d {
                    0.0
                } else if d <= ONE_POINT_OH_ONE {
                    d
                } else {
                    0.0
                }
            } else {
                d
            };
            dir[i] = [l0, l1, l2];
            let c1: f32 = lf_checker_rt::callee_thiscall!(
                CALLEE_FILTER,
                f32,
                this.wrapping_add(FILTER_THIS_A),
                r.to_bits()
            );
            let gain = rdf(this.wrapping_add(SRC_GAIN).wrapping_add((i as u32).wrapping_mul(4)));
            let c2: f32 = lf_checker_rt::callee_thiscall!(
                CALLEE_FILTER,
                f32,
                this.wrapping_add(FILTER_THIS_B),
                gain.to_bits()
            );
            prod[i] = mul(c2, c1);
        }

        let dirs = [
            (DIAG, f32::from_bits(DIAG.to_bits() ^ 0x8000_0000), 0.0f32),
            (DIAG, DIAG, 0.0f32),
            (
                f32::from_bits(DIAG.to_bits() ^ 0x8000_0000),
                f32::from_bits(DIAG.to_bits() ^ 0x8000_0000),
                0.0f32,
            ),
            (f32::from_bits(DIAG.to_bits() ^ 0x8000_0000), DIAG, 0.0f32),
        ];
        let mut acc = [0.0f32; 12];
        let mut mix = [0.0f32; 16];
        let filter_arg = gv_32(FILTER_ARG_GV);
        let out_scale = gv_f(OUT_SCALE_GV);
        let mut shape_this = this.wrapping_add(SHAPE_THIS);
        let mut out_ptr = this.wrapping_add(OUT_BASE);

        for (k, &(ddx, ddy, ddz)) in dirs.iter().enumerate() {
            let mut total = 0.0f32;
            for j in 0..count {
                let d = dir[j];
                let mut t = add(add(mul(d[1], ddx), mul(d[0], ddy)), mul(d[2], ddz));
                if 0.0 > t {
                    t = 0.0;
                }
                if j == 0 {
                    t = one;
                }
                let w = dot[j];
                let e = mul(add(mul(sub(one, w), QUARTER), mul(w, t)), prod[j]);
                mix[j] = e;
                total = add(total, e);
            }
            if count > 0 {
                let q = div(one, total);
                for j in 0..count {
                    let f = mix[j];
                    let v = this.wrapping_add(VEC_BASE).wrapping_add((j as u32).wrapping_mul(12));
                    let b = 3 * k;
                    acc[b] = add(acc[b], mul(mul(rdf(v), f), q));
                    acc[b + 1] = add(acc[b + 1], mul(mul(f, rdf(v.wrapping_add(4))), q));
                    acc[b + 2] = add(acc[b + 2], mul(mul(rdf(v.wrapping_add(8)), f), q));
                }
            }
            for m in 0..3 {
                let shaped: f32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_SHAPE,
                    f32,
                    shape_this,
                    acc[3 * k + m].to_bits(),
                    filter_arg
                );
                out_ptr = out_ptr.wrapping_add(4);
                wrf(out_ptr.wrapping_sub(4), mul(shaped, out_scale));
                shape_this = shape_this.wrapping_add(SHAPE_STRIDE);
            }
        }

        let rv: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_NOTIFY,
            u32,
            lf_checker_rt::relocated(REGISTRY_THIS),
            this.wrapping_add(OUT_BASE)
        );
        // Trailing security-cookie check: preserves every register.
        let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        rv
    }
}

lf_checker_rt::export!(thiscall, rw_009733A0(this: u32) -> u32 {
    unsafe { audio_spatial_body(this, true) }
});
