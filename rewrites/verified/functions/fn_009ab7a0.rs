// original: 0x009AB7A0 audio_directional_voice_mix (proposed)

/// Directional voice mix: probes the entity's range response, resolves a
/// direction from a global lookup, blends it with gated global weights, and
/// drives two curve evaluations whose results feed the outputs.
///
/// `this` is the weather audio entity, `a0` a probe block (three floats),
/// `a1` an output float slot, `a2` an output word slot. Entity fields used
/// are the cap pair at +0xda0/+0xa20 (only to form a stored-but-never-read
/// maximum, which this rewrite omits), the output bias at +0xc74, and the
/// two curve objects at +0xc24/+0xc4c.
///
/// Behaviour: the range gate (the function at 0x9ab530, called with `this`
/// and `a0`) yields R0, which is raised to at least the cap
/// [0xda0]*[0xa20] (an above-or-equal comparison, so NaN and ties keep R0);
/// a global resolver object answers three direction floats O0..O2 for `a0`;
/// N = O0^2+O1^2+O2^2 selects per-lane between a global gain and +0.0
/// through three global thresholds (a comparison that is unordered counts
/// as not above); the normalised direction (1/sqrt(N) times each O) is
/// blended per bit with the selected lane values under a 12-byte global
/// mask; a quadratic form over the blended lanes, scaled by 0.5 then 0.1
/// and added to 1.0, is subtracted from the raised R0 to form D0, which is
/// passed to BOTH curve evaluations (the second call re-reads the same
/// stack slot, not the first call's answer); the first curve's answer
/// drives an exponential-style mapping whose result plus the bias lands in
/// `a1`; the second curve's x87 answer is consumed by the final call, so it
/// is unobserved; the final call's integer answer lands in `a2` and is the
/// return value. Note the mapping call cleans no stack (cdecl), which is
/// why the second curve re-reads D0's slot.
///
/// All float arithmetic keeps the original's operand order; unordered
/// threshold comparisons take the not-above path. The callee out-parameters
/// and answers are scripted by the proof contract.
///
/// Original: 0x009AB7A0 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_009ab7a0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const OUT_BIAS: u32 = 0xc74;
        const CURVE_A: u32 = 0xc24;
        const CURVE_B: u32 = 0xc4c;
        const GAIN: u32 = 0x017AD148;
        const THR_BASE: u32 = 0x0110DAD0;
        const MASK_BASE: u32 = 0x0110DB50;
        const ONE: u32 = 0x00FE88E8;
        const HALF: u32 = 0x00FE8830;
        const TENTH: u32 = 0x01039118;
        const RESOLVER: u32 = 0x01633810;
        const RANGE_GATE: u32 = 1;
        const DIRECTION: u32 = 2;
        const CURVE: u32 = 3;
        const EXP_MAP: u32 = 4;
        const FINALIZE: u32 = 5;

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
        unsafe fn glob(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn glob32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }

        let r0: f32 =
            lf_checker_rt::callee_thiscall!(RANGE_GATE, f32, this, a0);
        // Raised floor: max(cap, r0), where ties and unordered keep r0
        // (the original's `ja` over the pair). Live: a later read through
        // a shifted stack pointer consumes this slot.
        let cap = mul(rdf(this.wrapping_add(0xda0)), rdf(this.wrapping_add(0xa20)));
        let floor = if cap > r0 { cap } else { r0 };
        let mut o = [0.0f32; 3];
        // The resolver object address is an immediate, not a load: the
        // callee receives the relocated address itself.
        lf_checker_rt::callee_thiscall!(
            DIRECTION,
            u32,
            lf_checker_rt::relocated(RESOLVER),
            a0,
            o.as_mut_ptr() as u32
        );
        let n = add(add(mul(o[0], o[0]), mul(o[1], o[1])), mul(o[2], o[2]));
        let gain = glob(GAIN);
        let h0 = glob(THR_BASE.wrapping_add(8));
        let h1 = glob(THR_BASE.wrapping_add(4));
        let h2 = glob(THR_BASE);
        let sel0 = if n > h2 { gain } else { 0.0 };
        let sel1 = if n > h1 { gain } else { 0.0 };
        let sel2 = if n > h0 { gain } else { 0.0 };
        let inv = div(1.0, core::hint::black_box(n).sqrt());
        // NOTE: the original divides the constant 1.0 twice (two identical
        // quotients); both feed the lanes below as the same value.
        let l0 = mul(inv, o[0]);
        let l1 = mul(inv, o[1]);
        let l2 = mul(inv, o[2]);
        let m0 = glob32(MASK_BASE);
        let m1 = glob32(MASK_BASE.wrapping_add(4));
        let m2 = glob32(MASK_BASE.wrapping_add(8));
        // Per-bit select: where a select bit is set the lane bit wins,
        // elsewhere the mask bit wins (`andps`, then `andnps` which is
        // ~dest & src, then `orps`).
        let s0 = sel0.to_bits();
        let s1 = sel1.to_bits();
        let s2 = sel2.to_bits();
        let x0 = (l0.to_bits() & s0) | ((!s0) & m0);
        let x1 = (l1.to_bits() & s1) | ((!s1) & m1);
        let x2 = (l2.to_bits() & s2) | ((!s2) & m2);
        let b0 = f32::from_bits(x0);
        let b1 = f32::from_bits(x1);
        let b2 = f32::from_bits(x2);
        let mut t = mul(b1, 0.0);
        t = add(t, b0);
        t = add(t, mul(b2, 0.0));
        t = add(t, glob(ONE));
        t = mul(t, glob(HALF));
        t = mul(t, glob(TENTH));
        let d0 = sub(floor, t);
        let a_first: f32 = lf_checker_rt::callee_thiscall!(
            CURVE,
            f32,
            this.wrapping_add(CURVE_A),
            d0.to_bits()
        );
        let e1: f32 =
            lf_checker_rt::callee_cdecl!(EXP_MAP, f32, a_first.to_bits());
        wrf(a1, add(e1, rdf(this.wrapping_add(OUT_BIAS))));
        let _: f32 = lf_checker_rt::callee_thiscall!(
            CURVE,
            f32,
            this.wrapping_add(CURVE_B),
            d0.to_bits()
        );
        let r: u32 = lf_checker_rt::callee_cdecl!(FINALIZE, u32,);
        (a2 as *mut u32).write_unaligned(r);
        r
    }
});
