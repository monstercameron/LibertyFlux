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
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ_A), 0u32, 0u32);
        let f_sel: f32 = if (r1 as u8) != 0 {
            0.0
        } else {
            let r2: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(OBJ_B));
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
        let back = rd32(this.wrapping_add(CURSOR)).wrapping_sub(gu32(RING_SUB)) % RING_LEN;
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
        lf_checker_rt::callee_thiscall!(8, u32, lf_checker_rt::relocated(0x1633810), p2.as_ptr() as u32, p1.as_mut_ptr() as u32);
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
        // pre-scale blend outputs: the filter pair below reads these frame
        // slots, not the scaled direction.
        let (s0, s1, s2) = (dx, dy, dz);
        // listener-flag scale: 0x1039104 when the flag is clear, else the
        // whistle constant (the compare's flags survive to the branch).
        let k2 = if flag2 == 0 { gf(0x1039104) } else { gf(FCONST_2) };
        dx = mul(dx, k2);
        dy = mul(dy, k2);
        dz = mul(dz, k2);
        // sum/difference pairs against the TLS listener channels.
        // (the frame slot copied alongside them holds dead stack fill in the
        // original and is omitted: it reaches only a dead store.)
        let pair0_a = add(l2, dz);
        let pair0_b = add(l0, dx);
        let pair0_c = add(l1, dy);
        let pair1_a = sub(l2, dz);
        let pair1_b = sub(l0, dx);
        let pair1_c = sub(l1, dy);
        // ---- proximity pair ----
        let mut out64 = 0u32;
        let in136 = [pair1_b, pair1_c, pair1_a];
        let a9: f32 = lf_checker_rt::callee_thiscall!(9, f32, lf_checker_rt::relocated(OBJ_A), in136.as_ptr() as u32, &mut out64 as *mut u32 as u32);
        let f64o = f32::from_bits(out64);
        let mut out156 = 0u32;
        let in104 = [pair0_b, pair0_c, pair0_a];
        let a10: f32 = lf_checker_rt::callee_thiscall!(10, f32, lf_checker_rt::relocated(OBJ_A), in104.as_ptr() as u32, &mut out156 as *mut u32 as u32);
        let _ = out156;
        // ---- gain staging ----
        let a11_1: f32 = lf_checker_rt::callee_thiscall!(11, f32, lf_checker_rt::relocated(OBJ_C), a9.to_bits());
        let g1 = gf(FCONST_1);
        let a4_2: f32 = lf_checker_rt::callee_cdecl!(4, f32, mul(a11_1, sub(g1, f64o)).to_bits());
        let a11_2: f32 = lf_checker_rt::callee_thiscall!(11, f32, lf_checker_rt::relocated(OBJ_C), a10.to_bits());
        let a4_3: f32 = lf_checker_rt::callee_cdecl!(4, f32, mul(a11_2, sub(g1, f64o)).to_bits());
        // ---- filter pair over the blend outputs ----
        let k3 = gf(RING_MUL2);
        let in168a = [sub(l0, mul(s0, k3)), sub(l1, mul(s1, k3)), sub(l2, mul(s2, k3))];
        let a12_1: f32 = lf_checker_rt::callee_thiscall!(12, f32, this, in168a.as_ptr() as u32);
        let in168b = [add(mul(s0, k3), l0), add(mul(s1, k3), l1), add(mul(s2, k3), l2)];
        let a12_2: f32 = lf_checker_rt::callee_thiscall!(12, f32, this, in168b.as_ptr() as u32);
        // ---- maxima ----
        let m1 = gf(F2_128);
        let o1 = {
            let p = mul(m1, f3);
            if p > a12_1 { p } else { a12_1 }
        };
        let m1delayed = mul(m1, delayed);
        let o2 = if m1delayed > a12_2 { m1delayed } else { a12_2 };
        let a13: f32 = lf_checker_rt::callee_thiscall!(13, f32, lf_checker_rt::relocated(OBJ_E));
        let a11_4: f32 = lf_checker_rt::callee_thiscall!(11, f32, this.wrapping_add(0x9C0), a13.to_bits());
        let a11_5: f32 = lf_checker_rt::callee_thiscall!(11, f32, lf_checker_rt::relocated(OBJ_D), l2.to_bits());
        let o3 = if a11_4 > a11_5 { a11_4 } else { a11_5 };
        // ---- sub-voice loop nest ----
        let w = sub(g1, o3);
        let mut ptr_a = this.wrapping_add(0x948);
        let mut ptr_b = this.wrapping_add(idx1.wrapping_mul(15).wrapping_add(0xA2).wrapping_mul(8));
        let mut ptr_c = this.wrapping_add(0x510).wrapping_add(idx0.wrapping_mul(15).wrapping_mul(8));
        let f128 = [m1delayed, o2];
        let mut inner_out = [0.0f32; 6];
        let mut outer_out = [0.0f32; 3];
        let mut esi = 0u32;
        let mut oi = 0u32;
        while (esi as i32) < 0x18 {
            let snap_eax = ptr_c.wrapping_sub(0x4B0);
            let mut edi = 0u32;
            while (edi as i32) < 2 {
                let f = f128[edi as usize];
                let b0: f32 = lf_checker_rt::callee_thiscall!(11, f32, snap_eax, f.to_bits());
                let b1: f32 = lf_checker_rt::callee_thiscall!(11, f32, ptr_b.wrapping_sub(0x4B0), f.to_bits());
                let b2: f32 = lf_checker_rt::callee_thiscall!(11, f32, ptr_a.wrapping_sub(0x4B0), f.to_bits());
                let acc = add(mul(add(mul(b0, w3), mul(b1, w4)), w), mul(b2, o3));
                esi = esi.wrapping_add(4);
                inner_out[esi.wrapping_sub(4).wrapping_div(4) as usize] = acc;
                edi = edi.wrapping_add(1);
            }
            let c0: f32 = lf_checker_rt::callee_thiscall!(11, f32, ptr_c, o1.to_bits());
            let c1: f32 = lf_checker_rt::callee_thiscall!(11, f32, ptr_b, o1.to_bits());
            let c2: f32 = lf_checker_rt::callee_thiscall!(11, f32, ptr_a, o1.to_bits());
            outer_out[oi as usize] = add(mul(add(mul(c0, w3), mul(c1, w4)), w), mul(c2, o3));
            oi += 1;
            ptr_c = ptr_c.wrapping_add(0x28);
            ptr_b = ptr_b.wrapping_add(0x28);
            ptr_a = ptr_a.wrapping_add(0x28);
        }
        // ---- conditional sub-voice starts ----
        let esi = this;
        let starts: [(u32, u32); 6] = [
            (8, lf_checker_rt::relocated(0xE92B4C)), (0xC, lf_checker_rt::relocated(0xE92B5C)), (0x10, lf_checker_rt::relocated(0xE92B6C)),
            (0x14, lf_checker_rt::relocated(0xE92B80)), (0x18, lf_checker_rt::relocated(0xE92B90)), (0x1C, lf_checker_rt::relocated(0xE92BA0)),
        ];
        for (off, tag) in starts {
            if rd32(esi.wrapping_add(off)) == 0 {
                lf_checker_rt::callee_thiscall!(14, u32, esi, tag, esi.wrapping_add(off),
                    0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32,
                    0u32, 0u32, 0u32, 0xFFFF_FFFFu32, 0u32, 0u32);
            }
        }
        let x1v = mul(a11_5, gf(WINIT_C));
        let run_blend = rd32(this.wrapping_add(8)) != 0
            && rd32(this.wrapping_add(0xC)) != 0
            && rd32(this.wrapping_add(0x10)) != 0
            && rd32(this.wrapping_add(0x14)) != 0
            && rd32(this.wrapping_add(0x18)) != 0
            && rd32(this.wrapping_add(0x1C)) != 0;
        if run_blend {
            let stride = gu32(STRIDE_G);
            let tabbase = gu32(TABBASE_G);
            let spread = |obj: u32| {
                let b4 = rd8(obj.wrapping_add(4));
                if b4 == 0xFF {
                    0u32
                } else {
                    let c40 = rd8(obj.wrapping_add(0x40));
                    (b4 as u32).wrapping_mul(stride).wrapping_add(rd32(
                        (c40 as u32).wrapping_mul(0x6F40).wrapping_add(tabbase).wrapping_add(0x6F14),
                    ))
                }
            };
            let a4pair = [a4_2, a4_3];
            // middle entry reads an uninitialised frame slot in the original
            // (no writer: the loop outputs around it); the rewrite uses +0.0,
            // matching the checker's zero stack fill.
            let outer_idx = [outer_out[2], 0.0f32, inner_out[0]];
            // edi serves double duty: inner-voice pointer in the pair loop,
            // reloaded from its frame slot (f4) for the spatial pair, then
            // advanced for the next pass.
            let mut edi = this.wrapping_add(8);
            let mut f4 = this.wrapping_add(8);
            let mut eap_idx = 0usize;
            for oc in 0..3u32 {
                for ic in 0..2u32 {
                    let v0 = add(add(add(f2, inner_out[eap_idx]), a4pair[ic as usize]), x1v);
                    let obj0 = rd32(edi);
                    lf_checker_rt::callee_thiscall!(15, u32, spread(obj0), v0.to_bits());
                    let obj1 = rd32(edi);
                    lf_checker_rt::callee_thiscall!(16, u32, obj1, outer_idx[oc as usize].to_bits());
                    edi = edi.wrapping_add(0xC);
                    eap_idx += 1;
                }
                edi = f4;
                let n0 = add(add(mul(in136[1], in136[1]), mul(in136[0], in136[0])), mul(in136[2], in136[2]));
                if (n0.to_bits() & 0x7F80_0000) != 0x7F80_0000 {
                    let p = [in136[0].to_bits(), in136[1].to_bits(), in136[2].to_bits(), 0u32];
                    lf_checker_rt::callee_thiscall!(17, u32, spread(rd32(edi)), p.as_ptr() as u32);
                }
                let n1 = add(add(mul(in104[1], in104[1]), mul(in104[0], in104[0])), mul(in104[2], in104[2]));
                if (n1.to_bits() & 0x7F80_0000) != 0x7F80_0000 {
                    let p = [in104[0].to_bits(), in104[1].to_bits(), in104[2].to_bits(), 0u32];
                    lf_checker_rt::callee_thiscall!(17, u32, spread(rd32(edi.wrapping_add(0xC))), p.as_ptr() as u32);
                }
                edi = edi.wrapping_add(4);
                f4 = edi;
            }
        }
        // ---- tail: voice commit ----
        lf_checker_rt::callee_thiscall!(18, u32, esi, arg0);
        let back2 = rd32(this.wrapping_add(CURSOR)).wrapping_sub(gu32(RING_SUB2)).wrapping_add(RING_LEN) % RING_LEN;
        let f = mul(rdf(this.wrapping_add(RING).wrapping_add(back2.wrapping_mul(4))), f1);
        let _tail_ans: f32 = lf_checker_rt::callee_thiscall!(3, f32, this.wrapping_add(0xA04), f.to_bits(), arg0);
        wrf(this.wrapping_add(0xA20), _tail_ans);
        let fl = gu32(INIT_FLAG);
        let ptrv = if fl & 1 == 0 {
            let fl1 = fl | 1;
            wr32(lf_checker_rt::relocated(INIT_FLAG), fl1);
            let p: u32 = lf_checker_rt::callee_cdecl!(19, u32, lf_checker_rt::relocated(INIT_C), 0u32);
            wr32(lf_checker_rt::relocated(INIT_PTR), p);
            p
        } else {
            gu32(INIT_PTR)
        };
        let back3 = rd32(this.wrapping_add(CURSOR)).wrapping_add(0x16) % RING_LEN;
        let fb = rdf(this.wrapping_add(RING).wrapping_add(back3.wrapping_mul(4)));
        let ans20: u32 = lf_checker_rt::callee_thiscall!(20, u32, lf_checker_rt::relocated(OBJ_F), ptrv, fb.to_bits());
        let cookie = gu32(COOKIE);
        lf_checker_rt::callee_thiscall!(21, u32, cookie);
        ans20
    }
}
