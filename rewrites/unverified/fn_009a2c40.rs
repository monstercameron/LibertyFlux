// original: 0x009a2c40 GIGGLE

/// Per-frame audio update: pick at most one sound to (re)start, then refresh
/// every live voice.
///
/// `this` points to the voice manager. Six voice slots live at `+SLOT94`,
/// `+SLOT98`, `+SLOT60`, `+SLOT64`, `+SLOTB0` and `+SLOTB4` (null when idle);
/// `+SUB8` points at the shared voice parameters, `+A0` holds a small state
/// value. The entry gates select one of three phases:
///
/// * Phase A (all slots idle, `+A0` negative, globals in the running
///   state): probe the shared object through two predicates, form a float
///   ratio of global gains, confirm it through a third predicate, look up
///   a gain value and, when it falls strictly inside the live band, emit
///   one start request. A failed first ratio retries with the alternate
///   numerator; a failed range check or a refused predicate ends the call.
/// * Phase B (no shared object, or the predicates disagree): check the
///   enable flag, a float threshold and an integer window, then emit one
///   start request.
/// * Phase C (anything else): resolve the listener frame (three scripted
///   blocks), settle `+A0`, then run every non-idle slot: feed the slot's
///   filter, derive its table index from the two tag bytes (0xFF at `+4`
///   forces index 0), feed the mixer, and hand the slot's frame block to
///   the applier. Slots 98/64/B4 first blend the TLS-selected float triple
///   into their block; slots 94/98 additionally re-arm through the
///   follow-up call when the shared object asks for it; slot 60 also
///   pushes the full voice description.
///
/// All float arithmetic is single SSE operations in the original's order;
/// every `jbe`-after-`comiss` continues only on strict greater-than (an
/// unordered NaN comparison exits, as in the original). All sixteen
/// outgoing calls are intercepted and scripted, including the two into the
/// encrypted first megabyte (they never execute). The exit eax is the
/// original's scratch and is not compared.
///
/// Original: 0x009a2c40 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009a2c40(this: u32) -> u32 {
    unsafe {
        const SUB8: u32 = 0x08;
        const SLOT94: u32 = 0x94;
        const SLOT98: u32 = 0x98;
        const SLOT60: u32 = 0x60;
        const SLOT64: u32 = 0x64;
        const SLOTB0: u32 = 0xb0;
        const SLOTB4: u32 = 0xb4;
        const A0: u32 = 0xa0;
        const F1F0: u32 = 0x1f0;
        const F1F4: u32 = 0x1f4;
        const F1F8: u32 = 0x1f8;
        const F1FC: u32 = 0x1fc;
        const EN201: u32 = 0x201;
        const NO_INDEX: u32 = 0xff;
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const TRIPLE_STRIDE: u32 = 64;
        const ONE_BITS: u32 = 0x3f80_0000;

        const G_RUN: u32 = 0x011f_7060;
        const G_GEN_A: u32 = 0x0120_88b4;
        const G_GEN_B: u32 = 0x00f1_c040;
        const G_MODE: u32 = 0x0103_7720;
        const G_GAIN_A: u32 = 0x0103_8dcc;
        const G_GAIN_B: u32 = 0x00fe_8b48;
        const G_GAIN_C: u32 = 0x00fe_88e8;
        const G_GAIN_D: u32 = 0x0103_8dd0;
        const G_BAND_LO: u32 = 0x0103_8dd4;
        const G_BAND_HI: u32 = 0x0103_8dd8;
        const G_EMIT_ARG: u32 = 0x0128_4530;
        const G_ENABLE: u32 = 0x0128_437f;
        const G_THRESH: u32 = 0x0128_31d8;
        const G_THRESH_REF: u32 = 0x0103_8da8;
        const G_WIN_HI: u32 = 0x0117_35b4;
        const G_WIN_LO: u32 = 0x0128_31c8;
        const G_TLS_SLOT: u32 = 0x017a_ba14;
        const G_MULT: u32 = 0x0115_d968;
        const G_TABLE: u32 = 0x0115_d988;
        const G_LOOKUP_OBJ: u32 = 0x0115_def0;
        const G_TRIPLES: u32 = 0x0115_e420;
        const G_BLEND_ARG: u32 = 0x0103_8de0;
        const G_TRACKER: u32 = 0x0128_8780;
        const EMIT_A: u32 = 0x00e9_0980;
        const EMIT_B: u32 = 0x00e9_0988;
        const EMIT_C: u32 = 0x00e9_0990;
        const EMIT_D: u32 = 0x00e9_09a0;

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
        unsafe fn grd(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read() }
        }
        #[inline(always)]
        unsafe fn grf(a: u32) -> f32 {
            unsafe { f32::from_bits(grd(a)) }
        }
        #[inline(always)]
        unsafe fn grb(a: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(a).read() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        /// Look up the gain for the shared object and emit a start request
        /// when it lands strictly inside the live band. Either way the
        /// function returns afterwards.
        unsafe fn emit(this: u32, addr: u32) {
            unsafe {
                let obj8 = rd32(this + SUB8);
                let arg = rd32(obj8 + 0x20).wrapping_add(0x30);
                let x: f32 =
                    lf_checker_rt::callee_thiscall!(4, f32, lf_checker_rt::relocated(G_LOOKUP_OBJ), arg);
                if x > grf(G_BAND_LO) && grf(G_BAND_HI) > x {
                    let g = grd(G_EMIT_ARG);
                    lf_checker_rt::callee_thiscall!(
                        5, u32, this, lf_checker_rt::relocated(addr), 0, 1, g,
                        0xffff_ffff, 0, 0, ONE_BITS, 0, 0
                    );
                }
            }
        }

        /// Phase B: flag, float threshold and integer window, then one
        /// start request with cleared middle arguments.
        unsafe fn phase_b(this: u32) {
            unsafe {
                if rd8(this + EN201) == 0 && grb(G_ENABLE) == 0 {
                    return;
                }
                if !(grf(G_THRESH) > grf(G_THRESH_REF)) {
                    return;
                }
                let hi = grd(G_WIN_HI);
                let lo = grd(G_WIN_LO);
                if !(lo.wrapping_add(0x1f4) < hi) {
                    return;
                }
                if !(lo.wrapping_add(0x230) > hi) {
                    return;
                }
                lf_checker_rt::callee_thiscall!(
                    5, u32, this, lf_checker_rt::relocated(EMIT_D), 0, 0, 0,
                    0xffff_ffff, 0, 0, ONE_BITS, 0, 0
                );
            }
        }

        /// Table index for a slot from its two tag bytes.
        unsafe fn table_index(slot: u32) -> u32 {
            unsafe {
                let tag = rd8(slot + 4);
                if tag as u32 == NO_INDEX {
                    return 0;
                }
                let row = rd8(slot + 0x40) as u32;
                let mult = grd(G_MULT);
                let base = grd(G_TABLE);
                let w = rd32(
                    base
                        .wrapping_add(TABLE_BIAS)
                        .wrapping_add(row.wrapping_mul(ROW_STRIDE)),
                );
                (tag as u32).wrapping_mul(mult).wrapping_add(w)
            }
        }

        /// Blend the TLS-selected triple into a slot's frame block `d`
        /// (three words, contiguous): the per-component differences, then
        /// the blend call, then each difference re-increased by its own
        /// component. (The original stages the three components in frame
        /// slots whose addresses differ between the 98 block and the 64/B4
        /// blocks; the slots are never observed, only `d` is, and the
        /// resulting block is identical, so only the values are modelled.)
        unsafe fn tls_blend(p0: &[u32; 3], d: &mut [u32; 3]) {
            unsafe {
                let slot_idx = grd(G_TLS_SLOT);
                let sv = lf_checker_rt::tls_slot(slot_idx as usize);
                let idx = rd32(sv + 0x70);
                let e = lf_checker_rt::relocated(G_TRIPLES)
                    .wrapping_add(idx.wrapping_mul(TRIPLE_STRIDE));
                let f1 = rdf(e);
                let f2 = rdf(e + 4);
                let f3 = rdf(e + 8);
                let p = [
                    f32::from_bits(p0[0]),
                    f32::from_bits(p0[1]),
                    f32::from_bits(p0[2]),
                ];
                d[0] = fsub(p[0], f1).to_bits();
                d[1] = fsub(p[1], f2).to_bits();
                d[2] = fsub(p[2], f3).to_bits();
                let g = grf(G_BLEND_ARG);
                lf_checker_rt::callee_thiscall!(15, u32, d.as_mut_ptr() as u32, g.to_bits(), 0x7a);
                d[0] = fadd(f32::from_bits(d[0]), f1).to_bits();
                d[1] = fadd(f32::from_bits(d[1]), f2).to_bits();
                d[2] = fadd(f32::from_bits(d[2]), f3).to_bits();
            }
        }

        /// Shared-object gate for slots 94/98, then the follow-up call.
        unsafe fn maybe_follow(this: u32, slot: u32) {
            unsafe {
                let o8 = rd32(this + SUB8);
                if o8 != 0 && (rd32(o8 + 0xa74) == 2 || (rd8(o8 + 0x29c) & 4) != 0) {
                    lf_checker_rt::callee_thiscall!(14, u32, slot, 0);
                }
            }
        }

        // ---- entry gates: all must pass to stay in Phase A ----
        let in_a = grd(G_RUN) != 1
            && grd(G_GEN_A) == grd(G_GEN_B)
            && grd(G_MODE) != 0x12
            && (rd32(this + A0) as i32) < 0
            && rd32(this + SLOT94) == 0
            && rd32(this + SLOT98) == 0
            && rd32(this + SLOT60) == 0
            && rd32(this + SLOT64) == 0
            && rd32(this + SLOTB0) == 0
            && rd32(this + SLOTB4) == 0;
        if in_a {
            let edi = rd32(this + SUB8);
            if edi == 0 {
                phase_b(this);
                return 0;
            }
            let mut side = false;
            let a1: u32 = lf_checker_rt::callee_thiscall!(1, u32, edi);
            if (a1 & 0xff) != 0 {
                side = true;
            } else {
                let b1: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi);
                if (b1 & 0xff) != 0 {
                    side = true;
                }
            }
            if !side {
                let t = fmul(grf(G_GAIN_A), grf(G_GAIN_B));
                let r = fdiv(grf(G_GAIN_C), t);
                let c: u32 = lf_checker_rt::callee_cdecl!(3, u32, r.to_bits());
                if (c & 0xff) != 0 {
                    emit(this, EMIT_A);
                    return 0;
                }
                let t2 = fmul(grf(G_GAIN_D), grf(G_GAIN_B));
                let r2 = fdiv(grf(G_GAIN_C), t2);
                let c2: u32 = lf_checker_rt::callee_cdecl!(3, u32, r2.to_bits());
                if (c2 & 0xff) != 0 {
                    emit(this, EMIT_B);
                    return 0;
                }
                return 0;
            }
            let a: u32 = lf_checker_rt::callee_thiscall!(1, u32, edi);
            if (a & 0xff) == 0 {
                phase_b(this);
                return 0;
            }
            let b: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi);
            if (b & 0xff) != 0 {
                phase_b(this);
                return 0;
            }
            let t = fmul(grf(G_GAIN_D), grf(G_GAIN_B));
            let r = fdiv(grf(G_GAIN_C), t);
            let c: u32 = lf_checker_rt::callee_cdecl!(3, u32, r.to_bits());
            if (c & 0xff) == 0 {
                return 0;
            }
            emit(this, EMIT_C);
            return 0;
        }

        // ---- Phase C ----
        let mut p0 = [0u32; 3];
        let mut p1: u32 = 0x46ab_e000;
        let mut p2: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            6, u32, this, &mut p2 as *mut u32 as u32, &mut p1 as *mut u32 as u32,
            p0.as_mut_ptr() as u32
        );
        let f1c = fadd(rdf(this + F1F0), f32::from_bits(p2)).to_bits();
        let a0 = rd32(this + A0);
        if (a0 as i32) >= 0 && rd32(this + SLOT94) == 0 && rd32(this + SLOT98) == 0 {
            let v1f8 = rd32(this + F1F8);
            let al = if v1f8 != 0 { 1u32 } else { 0u32 };
            lf_checker_rt::callee_thiscall!(7, u32, lf_checker_rt::relocated(G_TRACKER), a0, al, 0);
            wr32(this + A0, 0xffff_ffff);
            if grd(G_RUN) != 1
                && grd(G_GEN_A) == grd(G_GEN_B)
                && grd(G_MODE) != 0x12
                && rd32(this + F1F8) != 0
            {
                let v = rd32(this + F1F8);
                let w = rd32(this + F1FC);
                lf_checker_rt::callee_thiscall!(8, u32, this, v, w);
                let slot_idx = grd(G_TLS_SLOT);
                let sv = lf_checker_rt::tls_slot(slot_idx as usize);
                let flag = rd32(sv + 0x8d0);
                let edi_arg = this + F1F8;
                if ((flag >> 3) & 1) != 0 {
                    lf_checker_rt::callee_cdecl!(9, u32, v, edi_arg);
                } else if v != 0 {
                    lf_checker_rt::callee_thiscall!(10, u32, v, edi_arg);
                }
                wr32(this + F1F8, 0);
            }
        }

        let s94 = rd32(this + SLOT94);
        if s94 != 0 {
            lf_checker_rt::callee_thiscall!(11, u32, s94, p1);
            let edx = table_index(s94);
            lf_checker_rt::callee_thiscall!(12, u32, edx, f1c);
            lf_checker_rt::callee_thiscall!(13, u32, s94, p0.as_mut_ptr() as u32);
            maybe_follow(this, s94);
        }
        let s98 = rd32(this + SLOT98);
        if s98 != 0 {
            let mut d = [0u32; 3];
            tls_blend(&p0, &mut d);
            lf_checker_rt::callee_thiscall!(11, u32, s98, p1);
            let edx = table_index(s98);
            let x = fadd(rdf(this + F1F4), f32::from_bits(f1c));
            lf_checker_rt::callee_thiscall!(12, u32, edx, x.to_bits());
            lf_checker_rt::callee_thiscall!(13, u32, s98, d.as_mut_ptr() as u32);
            maybe_follow(this, s98);
        }
        let o8 = rd32(this + SUB8);
        let s60 = rd32(this + SLOT60);
        if s60 != 0 && o8 != 0 {
            lf_checker_rt::callee_thiscall!(11, u32, s60, p1);
            let edx = table_index(s60);
            lf_checker_rt::callee_thiscall!(12, u32, edx, f1c);
            lf_checker_rt::callee_thiscall!(13, u32, s60, p0.as_mut_ptr() as u32);
            lf_checker_rt::callee_cdecl!(
                16, u32, o8, rd32(this + 0x10), this + 0x14, rd32(this + 0x90),
                rd8(this + 0x54) as u32, rd8(this + 0x55) as u32,
                rd32(this + 0x58), rd32(this + 0x5c)
            );
        }
        let s64 = rd32(this + SLOT64);
        if s64 != 0 && o8 != 0 {
            let mut d = [0u32; 3];
            tls_blend(&p0, &mut d);
            lf_checker_rt::callee_thiscall!(11, u32, s64, p1);
            let edx = table_index(s64);
            let x = fadd(rdf(this + F1F4), f32::from_bits(f1c));
            lf_checker_rt::callee_thiscall!(12, u32, edx, x.to_bits());
            lf_checker_rt::callee_thiscall!(13, u32, s64, p0.as_mut_ptr() as u32);
        }
        let sb0 = rd32(this + SLOTB0);
        if sb0 != 0 {
            lf_checker_rt::callee_thiscall!(11, u32, sb0, p1);
            let edx = table_index(sb0);
            lf_checker_rt::callee_thiscall!(12, u32, edx, f1c);
            lf_checker_rt::callee_thiscall!(13, u32, sb0, p0.as_mut_ptr() as u32);
        }
        let sb4 = rd32(this + SLOTB4);
        if sb4 != 0 {
            let mut d = [0u32; 3];
            tls_blend(&p0, &mut d);
            lf_checker_rt::callee_thiscall!(11, u32, sb4, p1);
            let edx = table_index(sb4);
            let x = fadd(rdf(this + F1F4), f32::from_bits(f1c));
            lf_checker_rt::callee_thiscall!(12, u32, edx, x.to_bits());
            lf_checker_rt::callee_thiscall!(13, u32, sb4, d.as_mut_ptr() as u32);
        }
        0
    }
});
