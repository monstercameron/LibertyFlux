// original: 0x00cbea40 ped_move_blend_resolve (proposed)

/// Resolve the active move-blend target and finish the output vector.
///
/// `this` is the blender object with its state block at `+0x24`; `a0` is a
/// parameter block. Returns the resolved target, or null when the blend
/// call answers null.
///
/// Behaviour: an angle from a callee is clamped to `[-3.10668, 3.10668]`
/// (NaN clamps to the lower bound). A mode bit in `this+0x50` picks one of
/// two thresholds for `this+0x10`, seeding a 0/1 latch that a virtual check
/// on the state object may force to 1 and a parameter/mode gate may clear.
/// A second 0/1 flag is picked by a four-way switch on the mode bit and the
/// latch over four global enable bytes, then masked by two parameter bits.
/// Ten channels (`0x12`-`0x1B`) are scanned for the first live target.
///
/// With a live target whose value is non-negative, whose flag bit 4 is set
/// and whose mode bit 2 is clear, the start-up global seeds the target
/// rate, an id outside `0x12`/`0x17` latches a state bit, and a
/// sine/cosine-style refinement may tail out through a magnitude check;
/// otherwise the output is the start-up global times `this+0x2c` clamped
/// to `[-pi, pi]`. Without such a target a selector over the clamped angle
/// and the two flags picks one of five channel pairs (the latch bit
/// choosing the alternate), a second virtual call on the blender picks a
/// blend weight, and the blend call resolves the target; a zero selector
/// phase then copies `this+0x28`, otherwise the clamped angle minus the
/// phase, divided by a callee-provided total, is clamped symmetrically
/// around a scaled state value. The output finishes as
/// `this+0x90` (or `this+0x30` on the early path) times the start-up
/// global.
///
/// The original reuses its dead incoming-argument slot as scratch; the
/// rewrite uses locals, and the contract switches the stack comparison off
/// for this reason. Two calls go through object function tables; the
/// contract plants stub addresses there and both sides call through them.
///
/// Original: 0x00cbea40 (thiscall, one stack word). Float arithmetic is in
/// the original's operand order; ordered comparisons reproduce the
/// original's NaN behaviour.
lf_checker_rt::export!(thiscall, rw_00cbea40(this: u32, a0: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x24;
        const INNER_OFF: u32 = 0x78;
        const FLAGS_OFF: u32 = 0x50;
        const MODE_SHIFT: u32 = 13;
        const WIDE_BIT: u32 = 0x80000;
        const LAT_BIT: u32 = 0x200000;
        const IN10: u32 = 0x10;
        const IN70: u32 = 0x70;
        const IN2C: u32 = 0x2c;
        const IN28: u32 = 0x28;
        const IN94: u32 = 0x94;
        const OUT90: u32 = 0x90;
        const OUT30: u32 = 0x30;
        const CLAMP_FLAG: u32 = 0x219;
        const VT_OBJ: u32 = 0x2c4;
        const VT_KEY: u32 = 0x2e;
        const VT_SLOT: u32 = 0x12c;
        const SEL_SLOT: u32 = 0x58;
        const DIFF_HI: u32 = 0xaa4;
        const DIFF_LO: u32 = 0xaa0;
        const SCALE_OFF: u32 = 0xaa8;
        const P378: u32 = 0x378;
        const TGT_FLAGS: u32 = 0x4;
        const TGT_ID: u32 = 0xc;
        const TGT_MODE: u32 = 0x46;
        const TGT_RATE: u32 = 0x54;
        const G_B21: u32 = 0x0105_1421;
        const G_B22: u32 = 0x0105_1422;
        const G_B23: u32 = 0x0105_1423;
        const G_B40: u32 = 0x0105_1440;
        const G_W44: u32 = 0x0105_1444;
        const G_STARTUP2: u32 = 0x0171_bf8c;
        const G_STARTUP: u32 = 0x0117_35bc;
        const LO3: f32 = f32::from_bits(0xc046_d3d8);
        const HI3: f32 = f32::from_bits(0x4046_d3d8);
        const T16: f32 = f32::from_bits(0x3fcc_cccd);
        const TWO: f32 = 2.0;
        const ZERO: f32 = 0.0;
        const FOUR: f32 = 4.0;
        const C148: f32 = f32::from_bits(0x3fbd_e44f);
        const C209: f32 = f32::from_bits(0x4006_0a92);
        const C055: f32 = f32::from_bits(0x3f0e_fa35);
        const C0017: f32 = f32::from_bits(0x3c8e_fa35);
        const C50: f32 = 50.0;
        const PI: f32 = f32::from_bits(0x4049_0fdb);
        const NPI: f32 = f32::from_bits(0xc049_0fdb);
        const PI2: f32 = f32::from_bits(0x3fc9_0fdb);
        const NPI2: f32 = f32::from_bits(0xbfc9_0fdb);
        const A_ANGLE: u32 = 1;
        const A_PROBE: u32 = 2;
        const A_VT: u32 = 3;
        const A_CHECK: u32 = 4;
        const A_GET: u32 = 5;
        const A_VALUE: u32 = 6;
        const A_TRIG: u32 = 7;
        const A_PHASE: u32 = 8;
        const A_BLEND: u32 = 9;
        const A_TOTAL: u32 = 10;
        const A_SEL: u32 = 11;

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
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fabs(x: f32) -> f32 {
            f32::from_bits(x.to_bits() & 0x7fff_ffff)
        }
        /// Selector path: pick a channel pair from the clamped angle and
        /// the flags, resolve the target through the blend call, and finish.
        #[inline(always)]
        unsafe fn ee_path(
            this: u32,
            a0: u32,
            state: u32,
            inner: u32,
            l2: f32,
            l3: u32,
            l6: u32,
            bss2: f32,
            startup: f32,
        ) -> u32 {
            unsafe {
                let abs2 = fabs(l2);
                // Each selector arm also stores a phase float; the default
                // arm leaves the zero stored on entry.
                let (base, alt, l7f) = if C148 > abs2 {
                    (0x12u32, 0x17u32, ZERO)
                } else if (l6 & 0xff) == 0 {
                    (0x12u32, 0x17u32, ZERO)
                } else if C209 > abs2 {
                    if l2 > ZERO {
                        (0x13u32, 0x18u32, PI2)
                    } else {
                        (0x14u32, 0x19u32, NPI2)
                    }
                } else if l2 > ZERO {
                    (0x15u32, 0x1au32, HI3)
                } else {
                    (0x16u32, 0x1bu32, LO3)
                };
                let sel = if (l3 & 0xff) != 0 { alt } else { base };
                let vt = rd32(this);
                let slot = rd32(vt + SEL_SLOT);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let kr = f(this, inner);
                let x = if kr & 0xff != 0 { FOUR } else { gf(G_W44) };
                let p: u32 = lf_checker_rt::callee_cdecl!(
                    A_BLEND, u32, inner, rd32(a0), sel, x.to_bits(), rd32(a0 + 4));
                if p == 0 {
                    return 0;
                }
                wrf(p + TGT_RATE, bss2);
                let a3: f32 =
                    lf_checker_rt::callee_cdecl!(A_ANGLE, f32, rdf(state + DIFF_HI).to_bits());
                wrf(this + IN94, a3);
                if l7f != 0.0 {
                    let nl1 = mul(mul(rdf(state + SCALE_OFF), C0017), C50);
                    let dv: f32 = lf_checker_rt::callee_thiscall!(A_TOTAL, f32, p);
                    let q = div(sub(l2, l7f), dv);
                    let neg = -nl1;
                    let r = if neg > q { neg } else { q };
                    let r2 = if r > nl1 { nl1 } else { r };
                    wrf(this + OUT90, r2);
                } else {
                    wr32(this + OUT90, rd32(this + IN28));
                }
                wrf(this + OUT30, mul(rdf(this + OUT90), startup));
                p
            }
        }

        let state = rd32(this + STATE_OFF);
        let diff = sub(rdf(state + DIFF_HI), rdf(state + DIFF_LO));
        let s: f32 = lf_checker_rt::callee_cdecl!(A_ANGLE, f32, diff.to_bits());
        let mut l2 = s;
        if !(s > LO3) {
            l2 = LO3;
        } else if !(HI3 > s) {
            l2 = HI3;
        }
        let bit13 = (rd32(this + FLAGS_OFF) >> MODE_SHIFT) & 1;
        let m10 = rdf(this + IN10);
        let thresh = if bit13 != 0 { T16 } else { TWO };
        let mut l3: u32 = if m10 >= thresh { 1 } else { 0 };
        if bit13 != 0 && rd8(state + CLAMP_FLAG) != 0 {
            let t: f32 = lf_checker_rt::callee_thiscall!(A_PROBE, f32, state, 0u32);
            if t > ZERO {
                let q = rd32(state + VT_OBJ);
                let mut run_d = false;
                if q == 0 {
                    run_d = true;
                } else {
                    let key = ((q + VT_KEY) as *const u16).read_unaligned() as i16 as i32 as u32;
                    let vt = rd32(state);
                    let slot = rd32(vt + VT_SLOT);
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    if key != f(state) {
                        run_d = true;
                    }
                }
                if run_d {
                    let dr: u32 = lf_checker_rt::callee_cdecl!(A_CHECK, u32, state);
                    if dr & 0xff == 0 {
                        l3 = 1;
                    }
                }
            }
        }
        let inner = rd32(state + INNER_OFF);
        let m378 = rd32(a0 + P378);
        let keep_l3 = ((m378 >> 2) & 1) != 0
            && rd32(this + FLAGS_OFF) & WIDE_BIT == 0
            && !(TWO > rdf(this + IN70));
        if !keep_l3 {
            l3 = 0;
        }
        let mut l6: u32 = 1;
        let dl = (l3 & 0xff) != 0;
        if bit13 != 0 {
            if dl {
                if g8(G_B22) == 0 {
                    l6 = 0;
                }
            } else if g8(G_B21) == 0 {
                l6 = 0;
            }
        } else if dl {
            if g8(G_B40) == 0 {
                l6 = 0;
            }
        } else if g8(G_B23) == 0 {
            l6 = 0;
        }
        if m378 & 1 != 0 && (m378 >> 1) & 1 == 0 {
            l6 = 0;
        }
        let bss2 = gf(G_STARTUP2);
        let startup = gf(G_STARTUP);
        let mut esi = 0u32;
        for ch in [0x12u32, 0x13, 0x15, 0x14, 0x16, 0x17, 0x18, 0x1a, 0x19, 0x1b] {
            esi = lf_checker_rt::callee_thiscall!(A_GET, u32, inner, ch);
            if esi != 0 {
                break;
            }
        }
        if esi == 0 {
            return ee_path(this, a0, state, inner, l2, l3, l6, bss2, startup);
        }
        let f6: f32 = lf_checker_rt::callee_thiscall!(A_VALUE, f32, esi);
        if f6 < ZERO || (rd32(esi + TGT_FLAGS) >> 4) & 1 == 0 || (rd8(esi + TGT_MODE) >> 2) & 1 != 0
        {
            return ee_path(this, a0, state, inner, l2, l3, l6, bss2, startup);
        }
        wrf(esi + TGT_RATE, bss2);
        let eid = rd32(esi + TGT_ID);
        if eid != 0x12 && eid != 0x17 {
            or32(this + FLAGS_OFF, LAT_BIT);
        }
        let a2: f32 = lf_checker_rt::callee_cdecl!(A_ANGLE, f32, rdf(state + DIFF_HI).to_bits());
        if rd32(this + FLAGS_OFF) & LAT_BIT != 0 {
            let g1: f32 = lf_checker_rt::callee_cdecl!(A_TRIG, f32, a2.to_bits());
            let g2: f32 = lf_checker_rt::callee_cdecl!(A_TRIG, f32, rdf(this + IN94).to_bits());
            let _ = g2;
            if !g1.is_nan() {
                let h: f32 = lf_checker_rt::callee_cdecl!(
                    A_PHASE, f32, rdf(this + IN94).to_bits(), a2.to_bits());
                if C055 > fabs(h) {
                    wrf(this + OUT30, mul(rdf(this + OUT90), startup));
                    return esi;
                }
            }
        }
        let m2c = rdf(this + IN2C);
        let x1 = if NPI > m2c {
            NPI
        } else if m2c > PI {
            PI
        } else {
            m2c
        };
        wrf(this + OUT30, mul(startup, x1));
        esi
    }
});
