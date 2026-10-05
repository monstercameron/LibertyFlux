// original: 0x00a24c90 task_pose_integrate (proposed)

/// Integrate the pose triple at `this + 0x140` from sine/cosine pairs.
///
/// `dt` is the step (float bits); the first stack word is unread. Two data
/// bytes steer the shape: when the fast flag is set the function only adds
/// `dt` to `this + 0x148` and returns with no calls. Otherwise it forms
/// `v0 = [0x218] * 0.5` and:
///
/// * when the alt flag is set, composes one rotation from `sin`/`cos` of `v0`
///   and of `this + 0x21c` and adds the result into `0x140`/`0x144`/`0x148`;
/// * otherwise runs a longer composition (mixing `0x140`..`0x148` with the
///   `0x1b0` block), hands three frame triples to a solve callee along with
///   a generator answer, lets a post callee overwrite one word, then clamps
///   `this + 0x1e4` towards [0, 1] by 0.1 according to the post word's sign
///   (up when non-positive, down when positive, with a flag-gated reset)
///   and runs a final sine/cosine composition scaled by `v0 * [0x1e4]`.
///
/// The sine/cosine callees take their argument in XMM0 (the rewrite passes
/// it on the stack and the stub transports it) and answer in XMM0 and EAX;
/// the rewrite reads the EAX bits. Products with literal `+0.0` are kept as
/// the original computes them (NaN and signed-zero behaviour). The EAX
/// return is entry EAX on the fast path and stub scratch otherwise, so it is
/// not compared.
///
/// Original: 0x00a24c90 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a24c90(base: u32, _a1: u32, dt: u32) -> u32 {
    unsafe {
        const HALF: f32 = f32::from_bits(0x3f00_0000);
        const TENTH: f32 = f32::from_bits(0x3dcc_cccd);
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const ZERO: f32 = 0.0;
        const SIN: u32 = 1;
        const COS: u32 = 2;
        const GEN: u32 = 3;
        const BIG: u32 = 4;
        const POST: u32 = 5;
        const G_FAST: u32 = 0x012d_d5e1;
        const G_ALT: u32 = 0x012d_d5e0;
        const G_THIR: u32 = 0x012b_9c78;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        unsafe fn sin_of(x: f32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::callee_cdecl!(SIN, u32, x.to_bits())) }
        }
        #[inline(always)]
        unsafe fn cos_of(x: f32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::callee_cdecl!(COS, u32, x.to_bits())) }
        }

        let dtf = f32::from_bits(dt);
        // NOTE: the data-flag bytes live in the worker's relocated image;
        // read them through the runtime so the trial fills apply.
        let fast: u8 = lf_checker_rt::global::<u8>(G_FAST).read();
        if fast != 0 {
            wrf(base.wrapping_add(0x148), add(rdf(base.wrapping_add(0x148)), dtf));
            return 0;
        }
        let v0 = mul(rdf(base.wrapping_add(0x218)), HALF);
        let alt: u8 = lf_checker_rt::global::<u8>(G_ALT).read();
        if alt != 0 {
            let s0 = sin_of(v0);
            let c0 = cos_of(v0);
            let t5 = add(mul(c0, dtf), mul(s0, ZERO));
            let t3 = sub(mul(c0, ZERO), mul(s0, dtf));
            let v21c = rdf(base.wrapping_add(0x21c));
            let s1 = sin_of(v21c);
            let c1 = cos_of(v21c);
            let u3 = add(mul(c1, t3), mul(s1, ZERO));
            let u2 = sub(mul(c1, ZERO), mul(s1, t3));
            wrf(base.wrapping_add(0x140), add(rdf(base.wrapping_add(0x140)), u2));
            wrf(base.wrapping_add(0x144), add(rdf(base.wrapping_add(0x144)), u3));
            wrf(base.wrapping_add(0x148), add(rdf(base.wrapping_add(0x148)), t5));
            return 0;
        }
        let s0 = sin_of(v0);
        let c0 = cos_of(v0);
        let u3 = add(mul(c0, dtf), mul(s0, ZERO));
        let u2 = sub(mul(c0, ZERO), mul(s0, dtf));
        let v21c = rdf(base.wrapping_add(0x21c));
        let s1 = sin_of(v21c);
        let c1 = cos_of(v21c);
        let q1 = mul(s1, u2);
        let q5 = mul(c1, u2);
        let z0 = mul(s1, ZERO);
        let z2 = mul(c1, ZERO);
        let q6 = add(u3, rdf(base.wrapping_add(0x148)));
        let r2 = sub(z2, q1);
        let r1 = rdf(base.wrapping_add(0x140));
        let r5 = add(q5, z0);
        let r0 = add(rdf(base.wrapping_add(0x1b0)), r1);
        let r4 = add(r1, r2);
        let r2b = rdf(base.wrapping_add(0x1b4));
        let s0b = add(r2b, rdf(base.wrapping_add(0x144)));
        let r5b = add(r5, rdf(base.wrapping_add(0x144)));
        let r3 = add(rdf(base.wrapping_add(0x1b0)), r4);
        let t0 = add(rdf(base.wrapping_add(0x1b8)), rdf(base.wrapping_add(0x148)));
        let r2c = add(r2b, r5b);
        let r1b = add(rdf(base.wrapping_add(0x1b8)), q6);
        let genv: u32 = lf_checker_rt::callee_cdecl!(GEN, u32,);
        let esi = lf_checker_rt::global::<u32>(G_THIR).read();
        let mut slot20 = [r0.to_bits(), s0b.to_bits(), t0.to_bits()];
        let mut slot30 = [r3.to_bits(), r2c.to_bits(), r1b.to_bits()];
        let mut slot40 = [0u32; 1];
        let big: u32 = lf_checker_rt::callee_thiscall!(
            BIG, u32, esi,
            slot20.as_mut_ptr() as u32,
            slot30.as_mut_ptr() as u32,
            ONE.to_bits() - 0x0080_0000, // 0.5
            slot40.as_mut_ptr() as u32,
            0, genv, 0xffff_ffff, 7, 0, 1, 0
        );
        let mut post_arg0 = [0u32; 1];
        let mut post_slot = big;
        lf_checker_rt::callee_cdecl!(
            POST, u32,
            post_arg0.as_mut_ptr() as u32,
            core::ptr::addr_of_mut!(post_slot) as u32
        );
        if (post_slot as i32) > 0 {
            if rd8(base.wrapping_add(0x216)) & 1 != 0 {
                wrf(base.wrapping_add(0x1e4), ZERO);
            }
            let t = sub(rdf(base.wrapping_add(0x1e4)), TENTH);
            wrf(base.wrapping_add(0x1e4), if t > ZERO { t } else { ZERO });
        } else {
            let t = add(rdf(base.wrapping_add(0x1e4)), TENTH);
            wrf(base.wrapping_add(0x1e4), if ONE > t { t } else { ONE });
        }
        let vt = mul(v0, rdf(base.wrapping_add(0x1e4)));
        let s0t = sin_of(vt);
        let c0t = cos_of(vt);
        let w3 = add(mul(c0t, dtf), mul(s0t, ZERO));
        let w2 = sub(mul(c0t, ZERO), mul(s0t, dtf));
        let v21cb = rdf(base.wrapping_add(0x21c));
        let s1t = sin_of(v21cb);
        let c1t = cos_of(v21cb);
        let p1 = mul(w2, s1t);
        let p2 = mul(c1t, ZERO);
        let p4 = mul(w2, c1t);
        let p3 = mul(s1t, ZERO);
        wrf(
            base.wrapping_add(0x140),
            add(rdf(base.wrapping_add(0x140)), sub(p2, p1)),
        );
        wrf(
            base.wrapping_add(0x144),
            add(rdf(base.wrapping_add(0x144)), add(p4, p3)),
        );
        wrf(base.wrapping_add(0x148), add(rdf(base.wrapping_add(0x148)), w3));
        0
    }
});
