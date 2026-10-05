// original: 0x00CB8020 peds_task_state_init (proposed)

/// Build the shared ped-task state block from one object's eight floats.
///
/// `this` points to a task object holding eight floats at `+0x40`..`+0x5C`
/// (a, c, f, raw4, b, e, d, raw7). The function writes the shared state
/// block: the raw floats are copied to their slots, the three midpoints
/// `(a+b)/2`, `(c+e)/2`, `(f+d)/2` feed an enumeration call, and the
/// differences `(b-a)`, `(e-c)`, `(d-f)` are scaled by a multiplier that is
/// `1/sqrt(S)` when the sum of squared differences `S` is exactly zero and
/// `0.0` otherwise (the original tests this with `ucomiss`+`lahf`, which is
/// exactly `== 0.0`, NaN included), then stored as vector A. Two dot
/// products with sign flips, one square root and two words that read
/// unwritten scratch (reproduced as zero under the checker's defined stack
/// fill) complete the block. It then issues the enumeration call over the
/// midpoint block with this module's consumer as the callback address, copies
/// one global dword into the object, stamps the object id `2000` and flag `1`,
/// clears the shared flag byte and returns whether the flag reads back clear
/// (always true against the stub, which never re-enters). Original: thiscall
/// with one unread stack word.
lf_checker_rt::export!(thiscall, rw_00cb8020(this: u32, _unused: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x40;
        const F1: u32 = 0x44;
        const F2: u32 = 0x48;
        const F3: u32 = 0x4C;
        const F4: u32 = 0x50;
        const F5: u32 = 0x54;
        const F6: u32 = 0x58;
        const F7: u32 = 0x5C;
        const OBJ_DWORD: u32 = 0x2C;
        const OBJ_ID: u32 = 0x30;
        const OBJ_FLAG: u32 = 0x34;
        const OBJ_ID_VALUE: u32 = 2000;
        const ST_FLAG: u32 = 0x0171_BBE0;
        const ST_OUT_BIAS: u32 = 0x0171_BBE4;
        const ST_GATE_BIAS: u32 = 0x0171_BBE8;
        const ST_RANGE_HI: u32 = 0x0171_BBEC;
        const ST_A0: u32 = 0x0171_BC10;
        const ST_A1: u32 = 0x0171_BC14;
        const ST_A2: u32 = 0x0171_BC18;
        const ST_A_PAD: u32 = 0x0171_BC1C;
        const ST_B0: u32 = 0x0171_BF20;
        const ST_B1: u32 = 0x0171_BF24;
        const ST_B2: u32 = 0x0171_BF28;
        const ST_RAW4: u32 = 0x0171_BF00;
        const ST_RAW5: u32 = 0x0171_BF04;
        const ST_RAW6: u32 = 0x0171_BF08;
        const ST_RAW7: u32 = 0x0171_BF0C;
        const ST_RAW0: u32 = 0x0171_BF10;
        const ST_RAW1: u32 = 0x0171_BF14;
        const ST_GATE_REF: u32 = 0x0171_BF18;
        const ST_RAW3: u32 = 0x0171_BF1C;
        const ST_EXTRA: u32 = 0x0171_BF2C;
        const ST_SRC_DWORD: u32 = 0x0117_35B4;
        const CONSUMER: u32 = 0x00CB_7CF0;
        const HALF: f32 = 0.5;
        const RADIUS: f32 = 20.0;
        const SIGN: u32 = 0x8000_0000;
        const CALLEE_ENUM: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wf(va: u32, v: f32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v.to_bits()) }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        let a = rdf(this + F0);
        let c = rdf(this + F1);
        let f = rdf(this + F2);
        let b = rdf(this + F4);
        let e = rdf(this + F5);
        let d = rdf(this + F6);
        // Midpoints and differences.
        let m0 = mul(add(a, b), HALF);
        let m1 = mul(add(c, e), HALF);
        let m2 = mul(add(f, d), HALF);
        let d0 = sub(b, a);
        let d1 = sub(e, c);
        let d2 = sub(d, f);
        // Sum of squared differences, then the conditional multiplier.
        let sumsq = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
        let mult = if sumsq == 0.0 {
            core::hint::black_box(1.0f32) / core::hint::black_box(sumsq.sqrt())
        } else {
            0.0
        };
        let s0 = mul(d0, mult);
        let s1 = mul(d1, mult);
        let s2 = mul(d2, mult);
        // Cross-like terms against zero.
        let t2 = sub(mul(s0, 0.0), mul(s1, 0.0));
        let t0 = sub(s1, mul(s2, 0.0));
        let t1 = sub(mul(s2, 0.0), s0);
        // Raw copies.
        wf(ST_RAW0, a);
        wf(ST_RAW1, c);
        wf(ST_GATE_REF, f);
        wf(ST_RAW3, rdf(this + F3));
        wf(ST_RAW4, b);
        wf(ST_RAW5, e);
        wf(ST_RAW6, d);
        wf(ST_RAW7, rdf(this + F7));
        // Vector A from the scaled differences; the pad word reads unwritten
        // scratch, which is the defined stack fill (zero) on both sides.
        wf(ST_A_PAD, 0.0);
        wf(ST_A0, s0);
        wf(ST_A1, s1);
        wf(ST_A2, s2);
        // Gate bias: negated dot of (a, c, f) with the scaled differences.
        let gdot = add(add(mul(c, s1), mul(a, s0)), mul(f, s2));
        wf(ST_GATE_BIAS, neg(gdot));
        // B vector and range: the cross-like terms and the 2-D length.
        wf(ST_B0, t0);
        wf(ST_B1, t1);
        let len = add(mul(sub(e, c), sub(e, c)), mul(sub(b, a), sub(b, a))).sqrt();
        wf(ST_B2, t2);
        wf(ST_RANGE_HI, len);
        // Extra word: unwritten scratch again, so zero under the fill.
        wf(ST_EXTRA, 0.0);
        // Out bias: negated dot of (a, c, f) with the cross-like terms.
        let odot = add(add(mul(c, t1), mul(a, t0)), mul(f, t2));
        lf_checker_rt::global::<u8>(ST_FLAG).write(0);
        wf(ST_OUT_BIAS, neg(odot));
        // Enumeration call over [m0, m1, m2, 0, 20.0] with the consumer address.
        let mut block = [m0.to_bits(), m1.to_bits(), m2.to_bits(), 0u32, RADIUS.to_bits()];
        lf_checker_rt::callee_cdecl!(
            CALLEE_ENUM,
            u32,
            block.as_mut_ptr() as u32,
            lf_checker_rt::relocated(CONSUMER),
            0u32,
            4u32,
            5u32,
        );
        wr32(this + OBJ_DWORD, lf_checker_rt::global::<u32>(ST_SRC_DWORD).read_unaligned());
        wr32(this + OBJ_ID, OBJ_ID_VALUE);
        ((this + OBJ_FLAG) as *mut u8).write(1);
        if lf_checker_rt::global::<u8>(ST_FLAG).read() == 0 {
            1
        } else {
            0
        }
    }
});
