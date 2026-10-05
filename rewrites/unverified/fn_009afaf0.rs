// original: 0x009AFAF0 WIND_MID_LEFT (symbols, low confidence)

/// Wind voice update (mid band, left channel): recompute one wind sound's
/// mix parameters from global tables, the listener state and scripted audio
/// helpers, then drive its six sub-voices and three spatial blend passes.
///
/// `this` is the voice object (ring buffer of 30 recent filter outputs at
/// `+0xBA8`, cursor at `+0xC20`, seven mix slots at `+0xC74..+0xDA0`);
/// `arg0` is an opaque voice/handle id passed through to two helpers.
/// The routine is straight-line float code with small data-dependent
/// branches: two audibility gates up front, a global-override flag, a
/// listener-flag branch, six conditional sub-voice starts, and two
/// NaN-guarded spatial calls per blend pass. Integer comparisons: the two
/// inner loop bounds use signed less-than (`jl`), the blend-pass counters
/// use unsigned below (`jb`); the TLS slot, table indices and ring cursors
/// are unsigned. All float arithmetic is scalar single precision in the
/// original's operand order (pinned with `black_box`); the one 4-lane
/// select is bitwise-exact with its dead fourth lane dropped (it reads an
/// uninitialised frame slot in the original and is never used).
///
/// Original: 0x009AFAF0 (thiscall, ECX = `this`, one stack word `arg0`,
/// callee cleans 4). Returns the last helper's answer word.
lf_checker_rt::export!(thiscall, rw_009afaf0(this: u32, arg0: u32) -> u32 {
    unsafe {
        run_wind_mid_left(this, arg0, false)
    }
});

/// Behavioural mutant: identical except the seven mix-slot stores are
/// omitted, so the heap diff must catch it.
lf_checker_rt::export!(thiscall, mut_009afaf0(this: u32, arg0: u32) -> u32 {
    unsafe {
        run_wind_mid_left(this, arg0, true)
    }
});

#[allow(clippy::too_many_arguments)]
unsafe fn run_wind_mid_left(this: u32, arg0: u32, mutant_skip_mix: bool) -> u32 {
    unsafe {
        // ---- globals ----
        const COOKIE: u32 = 0x1057FB4;
        const OBJ_A: u32 = 0x1165880;
        const OBJ_B: u32 = 0x1284A60;
        const FCONST_1: u32 = 0xFE88E8;
        const IDX0: u32 = 0x12DDE80;
        const IDX1: u32 = 0x12DDE84;
        const WEIGHT_G: u32 = 0x12DDE94;
        const FLAG1: u32 = 0x1289186;
        const TLS_SLOT_G: u32 = 0x17ABA14;
        const TLS_TAB: u32 = 0x115E420;
        const VEC_C: u32 = 0x17AD148;
        const TH0: u32 = 0x110DAD8;
        const TH1: u32 = 0x110DAD4;
        const TH2: u32 = 0x110DAD0;
        const FLAG2: u32 = 0x11687D0;
        const FCONST_2: u32 = 0xFE879C;
        const MASK16: u32 = 0x110DB50;
        const RING_MUL: u32 = 0x1039100;
        const RING_MUL2: u32 = 0x1039140;
        const RING_SUB: u32 = 0x1039110;
        const RING_SUB2: u32 = 0x1039114;
        const OBJ_C: u32 = 0x12891B8;
        const OBJ_D: u32 = 0x128A8B0;
        const OBJ_E: u32 = 0x115DEF0;
        const OBJ_F: u32 = 0x115DB1C;
        const F2_128: u32 = 0x1039128;
        const WINIT_C: u32 = 0xFE8AB8;
        const STRIDE_G: u32 = 0x115D968;
        const TABBASE_G: u32 = 0x115D988;
        const INIT_FLAG: u32 = 0x128A90C;
        const INIT_PTR: u32 = 0x128A908;
        const INIT_C: u32 = 0xE92BB4;
        const RING_LEN: u32 = 30;
        // ---- this offsets ----
        const RING: u32 = 0xBA8;
        const CURSOR: u32 = 0xC20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gu32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gu8(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
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

        // ---- audibility gates ----
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, OBJ_A, 0u32, 0u32);
        let f_sel: f32 = if (r1 as u8) != 0 {
            0.0
        } else {
            let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, OBJ_B);
            if (r2 as u8) == 0 { gf(FCONST_1) } else { 0.0 }
        };
        // ---- first filter chain ----
        let f1: f32 = lf_checker_rt::callee_thiscall!(3, f32, this.wrapping_add(0x9E8), f_sel.to_bits(), arg0);
        let f2: f32 = lf_checker_rt::callee_cdecl!(4, f32, f1.to_bits());
        // ---- table lerp of the seven mix slots ----
        let idx0 = gu32(IDX0);
        let idx1 = gu32(IDX1);
        let w3 = sub(gf(FCONST_1), gf(WEIGHT_G));
        let w4 = sub(gf(FCONST_1), w3);
        let lerp = |base: u32| {
            let t1 = rdf(lf_checker_rt::relocated(base).wrapping_add(idx1.wrapping_mul(4)));
            let t0 = rdf(lf_checker_rt::relocated(base).wrapping_add(idx0.wrapping_mul(4)));
            add(mul(t1, w4), mul(t0, w3))
        };
        if !mutant_skip_mix {
            wrf(this.wrapping_add(0xC74), lerp(0xE926B8));
            wrf(this.wrapping_add(0xC78), lerp(0xE92708));
            wrf(this.wrapping_add(0xD8C), lerp(0xE926E0));
            wrf(this.wrapping_add(0xD54), lerp(0xE92730));
            wrf(this.wrapping_add(0xD94), lerp(0xE92780));
            wrf(this.wrapping_add(0xD9C), lerp(0xE927A8));
            wrf(this.wrapping_add(0xDA0), lerp(0xE92758));
        }
        if gu8(FLAG1) != 0 && !mutant_skip_mix {
            wrf(this.wrapping_add(0xC74), gf(0x1289188));
            wrf(this.wrapping_add(0xC78), gf(0x103912C));
            wrf(this.wrapping_add(0xD8C), gf(0x103913C));
            wrf(this.wrapping_add(0xD54), gf(0x1039130));
            wrf(this.wrapping_add(0xD94), gf(0x1039134));
            wrf(this.wrapping_add(0xD9C), gf(0x1039138));
            wrf(this.wrapping_add(0xDA0), gf(0x1039124));
        }
        // ---- second filter pair ----
        let t68_1 = rdf(lf_checker_rt::relocated(0xE92668).wrapping_add(idx1.wrapping_mul(4)));
        let t68_0 = rdf(lf_checker_rt::relocated(0xE92668).wrapping_add(idx0.wrapping_mul(4)));
        let t90_1 = rdf(lf_checker_rt::relocated(0xE92690).wrapping_add(idx1.wrapping_mul(4)));
        let x0 = mul(t68_0, w3);
        let mut x1 = mul(t68_1, w4);
        let mut x2 = mul(t90_1, w4);
        x1 = add(x1, x0);
        let t90_0 = rdf(lf_checker_rt::relocated(0xE92690).wrapping_add(idx0.wrapping_mul(4)));
        x2 = add(x2, mul(t90_0, w3));
        lf_checker_rt::callee_thiscall!(5, u32, this.wrapping_add(0x20), x1.to_bits(), x1.to_bits());
        lf_checker_rt::callee_thiscall!(6, u32, this.wrapping_add(0x20), x2.to_bits(), x2.to_bits());
        let f3: f32 = lf_checker_rt::callee_thiscall!(7, f32, this.wrapping_add(0x20));
        // ---- ring push ----
        let cur = rd32(this.wrapping_add(CURSOR));
        let slot = cur.wrapping_add(1) % RING_LEN;
        wr32(this.wrapping_add(CURSOR), slot);
        wrf(this.wrapping_add(RING).wrapping_add(slot.wrapping_mul(4)), f3);
        // ---- TLS listener lookup ----
        let back = rd32(this.wrapping_add(CURSOR)).wrapping_sub(gu32(RING_SUB)).wrapping_add(RING_LEN) % RING_LEN;
        let tls_slot_val = gu32(TLS_SLOT_G);
        let tls_ptr = lf_checker_rt::tls_slot(tls_slot_val as usize);
        let tls_idx = rd32(tls_ptr.wrapping_add(0x70));
        let tls_tab = lf_checker_rt::relocated(TLS_TAB).wrapping_add(tls_idx.wrapping_shl(6));
        let delayed = mul(rdf(this.wrapping_add(RING).wrapping_add(back.wrapping_mul(4))), gf(RING_MUL));
        let l0 = rdf(tls_tab);
        let l1 = rdf(tls_tab.wrapping_add(4));
        let l2 = rdf(tls_tab.wrapping_add(8));
        // ---- direction solve ----
        let mut p1 = [0u32; 3];
        let p2 = [gu32(0x1168810), gu32(0x1168814), gu32(0x1168818)];
        lf_checker_rt::callee_thiscall!(8, u32, 0x1633810u32, p2.as_ptr() as u32, p1.as_mut_ptr() as u32);
        let mut dx = f32::from_bits(p1[0]);
        let mut dy = f32::from_bits(p1[1]);
        let mut dz = f32::from_bits(p1[2]);
        let vc = gf(VEC_C);
        // norm order is exactly (dx*dx + dy*dy) + dz*dz.
        let norm_sq = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let g6 = if !(norm_sq > gf(TH0)) { 0.0 } else { vc };
        let g2 = if !(norm_sq > gf(TH1)) { 0.0 } else { vc };
        let mut g0 = vc;
        if !(norm_sq > gf(TH2)) {
            g0 = 0.0;
        }
        let flag2 = gu8(FLAG2);
        let s = core::hint::black_box(norm_sq).sqrt();
        let q = div(gf(FCONST_1), s);
        dy = mul(dy, q);
        dx = mul(dx, q);
        dz = mul(dz, q);
        // lane select: lanes 0..2 blend the direction with per-lane gates,
        // lane 3 is dead in the original (uninitialised frame).
        let m0 = rd32(lf_checker_rt::relocated(MASK16));
        let m1 = rd32(lf_checker_rt::relocated(MASK16).wrapping_add(4));
        let m2 = rd32(lf_checker_rt::relocated(MASK16).wrapping_add(8));
        let d0 = g0.to_bits();
        let d1 = g2.to_bits();
        let d2 = g6.to_bits();
        let s0 = (dx.to_bits() & d0) | (!d0 & m0);
        let s1 = (dy.to_bits() & d1) | (!d1 & m1);
        let s2 = (dz.to_bits() & d2) | (!d2 & m2);
        dx = f32::from_bits(s0);
        dy = f32::from_bits(s1);
        dz = f32::from_bits(s2);
        // listener-flag scale: 0x1039104 when the flag is clear, else the
        // whistle constant (the compare's flags survive to the branch).
        let k2 = if flag2 == 0 { gf(0x1039104) } else { gf(FCONST_2) };
        dx = mul(dx, k2);
        dy = mul(dy, k2);
        dz = mul(dz, k2);
        // sum/difference pairs against the delayed sample and TLS listener.
        let pair0_a = add(l2, dz);
        let pair0_b = add(delayed, dx);
        let pair0_c = add(l1, dy);
        let pair1_a = sub(l2, dz);
        let pair1_b = sub(delayed, dx);
        let pair1_c = sub(l1, dy);
        let _ = (pair0_a, pair0_b, pair0_c, pair1_a, pair1_b, pair1_c, l0, f2);
