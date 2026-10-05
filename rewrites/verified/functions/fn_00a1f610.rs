// original: 0x00A1F610 cam_follow_ped_blend_update (proposed)

/// Blend two externally held 4-word vectors toward per-object base vectors by
/// a countdown-driven factor, then copy the results back into the object.
///
/// `this` points to a camera object. `src` is an opaque source object passed
/// to two intercepted callees (`src+0x44` to the first). `dst1` and `dst2`
/// point at two 4-word vectors that are updated in place and then copied to
/// `this+0x2a0` and `this+0x2b0`. The first stack word is never read.
///
/// Behaviour: flag-gated reloads of three countdowns (`this+0x338/0x344`
/// from globals when bits 0x20/0x10 of `this+0x38e` are set, `this+0x340`
/// when `this+0x34c` matches a counter global), a flag-gated clearing of the
/// first two (bit 0x40), then a blend factor that starts at -1.0 (0.3 when
/// `this+0x33c` matches its global) and is overwritten by `1 - value/global`
/// for each positive countdown in the order 0x344, 0x338, 0x340. A probe
/// callee may run a hook callee when it answers a tagged object; a status
/// callee decides whether the 0x33c ratio uses the countdown form. When the
/// factor is not negative and any of the three base words at `this+0x2a0`
/// is nonzero (each is compared against +0.0 with ucomiss and the lahf bits
/// tested so that only an equal result falls through; NaN counts as
/// nonzero), each vector moves `base + (vec - base) * factor` and its fourth word
/// is overwritten with the scratch word the original copies from its own
/// uninitialised stack (zero under the contract's stack fill). Finally the
/// positive countdowns decrement and both vectors are copied into the object.
/// Returns the fourth word of `dst2`.
///
/// Original: 0x00A1F610 (thiscall, four stack words; the first is not read).
/// The `this+0x38c & 0x80` early exit returns whatever entry garbage is in
/// eax, which a Rust rewrite cannot observe; the contract pins that bit off.
lf_checker_rt::export!(thiscall, rw_00A1F610(this: u32, _unused: u32, src: u32, dst1: u32, dst2: u32) -> u32 {
    unsafe {
        const BASE_A: u32 = 0x2a0;
        const BASE_B: u32 = 0x2b0;
        const COUNT_A: u32 = 0x338;
        const MATCH_A: u32 = 0x33c;
        const COUNT_C: u32 = 0x340;
        const COUNT_B: u32 = 0x344;
        const MATCH_C: u32 = 0x34c;
        const FLAG_EXIT: u32 = 0x38c;
        const FLAG_MODE: u32 = 0x38e;
        const HOOK_ARG_OFF: u32 = 0x10;
        const PROBE_THIS_OFF: u32 = 0x44;
        const PROBE_TAG: u32 = 0x120;
        const PROBE_TAG_OFF: u32 = 0x18;
        const PROBE_TAG_WANT: u32 = 3;
        const G_MATCH_A: u32 = 0x0103bfe8;
        const G_COUNT_B: u32 = 0x0103bfec;
        const G_COUNT_A: u32 = 0x0103bff0;
        const G_COUNT_C: u32 = 0x0103bff4;
        const G_HOOK_ARG: u32 = 0x0103c084;
        const G_COUNTER: u32 = 0x011735c4;
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_HOOK: u32 = 2;
        const CALLEE_STATUS: u32 = 3;
        const NEG_ONE: f32 = -1.0;
        const LOW_DEFAULT: f32 = f32::from_bits(0x3e99999a); // 0.3
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g16(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u16>(va) as *const u16).read_unaligned() as u32 }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn ratio(value: u32, denom: u32) -> f32 {
            sub(ONE, div(value as i32 as f32, denom as i32 as f32))
        }

        if rd8(this + FLAG_EXIT) & 0x80 != 0 {
            // Unreachable under the contract (see doc comment): the original
            // returns entry-eax garbage here.
            return 0;
        }
        let mode = rd8(this + FLAG_MODE);
        if mode & 0x20 != 0 {
            wr32(this + COUNT_A, g32(G_COUNT_A));
        }
        if mode & 0x10 != 0 {
            wr32(this + COUNT_B, g32(G_COUNT_B));
        }
        if rd32(this + MATCH_C) == g32(G_COUNTER) {
            wr32(this + COUNT_C, g32(G_COUNT_C));
        }
        if rd32(this + COUNT_A) != 0 || rd32(this + COUNT_B) != 0 {
            if mode & 0x40 != 0 {
                wr32(this + COUNT_A, 0);
                wr32(this + COUNT_B, 0);
            }
        }
        let mut factor = if rd32(this + MATCH_A) == g32(G_MATCH_A) {
            LOW_DEFAULT
        } else {
            NEG_ONE
        };
        let probe: u32 =
            lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, src.wrapping_add(PROBE_THIS_OFF), PROBE_TAG);
        if probe != 0 && rd32(probe.wrapping_add(PROBE_TAG_OFF)) == PROBE_TAG_WANT {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_HOOK,
                u32,
                this,
                this.wrapping_add(HOOK_ARG_OFF),
                g16(G_HOOK_ARG),
                1,
                1
            );
        }
        if (rd32(this + MATCH_A) as i32) > 0 {
            let status: u32 = lf_checker_rt::callee_thiscall!(CALLEE_STATUS, u32, src);
            if (status & 0xff) == 0 {
                factor = ratio(rd32(this + MATCH_A), g32(G_MATCH_A));
            }
        }
        if (rd32(this + COUNT_B) as i32) > 0 {
            factor = ratio(rd32(this + COUNT_B), g32(G_COUNT_B));
        }
        if (rd32(this + COUNT_A) as i32) > 0 {
            factor = ratio(rd32(this + COUNT_A), g32(G_COUNT_A));
        }
        if (rd32(this + COUNT_C) as i32) > 0 {
            factor = ratio(rd32(this + COUNT_C), g32(G_COUNT_C));
        }
        // `comiss factor, 0.0; jb` skips on negative or NaN. The three
        // `ucomiss; lahf; (an instruction of the original); jp` gates each fall through only
        // on an equal-to-zero result (the test's parity is odd only for
        // 0x40), so the blend runs when any base word is nonzero.
        let blend = (factor >= 0.0)
            && (rdf(this + BASE_A) != 0.0
                || rdf(this + BASE_A + 4) != 0.0
                || rdf(this + BASE_A + 8) != 0.0);
        if blend {
            // dst1 toward BASE_A, dst2 toward BASE_B; the fourth word of
            // each is the original's uninitialised-stack scratch word,
            // which is zero under the contract's stack fill.
            let b0 = rdf(this + BASE_A);
            let b1 = rdf(this + BASE_A + 4);
            let b2 = rdf(this + BASE_A + 8);
            let d0 = mul(sub(rdf(dst1), b0), factor);
            let d1 = mul(sub(rdf(dst1 + 4), b1), factor);
            let d2 = mul(sub(rdf(dst1 + 8), b2), factor);
            wr32(dst1 + 12, 0);
            wrf(dst1 + 8, add(b2, d2));
            wrf(dst1, add(b0, d0));
            wrf(dst1 + 4, add(b1, d1));
            let c0 = rdf(this + BASE_B);
            let c1 = rdf(this + BASE_B + 4);
            let c2 = rdf(this + BASE_B + 8);
            let e0 = mul(sub(rdf(dst2), c0), factor);
            let e1 = mul(sub(rdf(dst2 + 4), c1), factor);
            let e2 = mul(sub(rdf(dst2 + 8), c2), factor);
            wr32(dst2 + 12, 0);
            wrf(dst2 + 8, add(c2, e2));
            wrf(dst2, add(c0, e0));
            wrf(dst2 + 4, add(c1, e1));
        }
        for off in [COUNT_C, COUNT_B, COUNT_A] {
            let v = rd32(this + off);
            if (v as i32) > 0 {
                wr32(this + off, v.wrapping_sub(1));
            }
        }
        wr32(this + BASE_A, rd32(dst1));
        wrf(this + BASE_A + 4, rdf(dst1 + 4));
        wrf(this + BASE_A + 8, rdf(dst1 + 8));
        wr32(this + BASE_A + 12, rd32(dst1 + 12));
        wr32(this + BASE_B, rd32(dst2));
        wrf(this + BASE_B + 4, rdf(dst2 + 4));
        wrf(this + BASE_B + 8, rdf(dst2 + 8));
        wr32(this + BASE_B + 12, rd32(dst2 + 12));
        rd32(dst2 + 12)
    }
});
