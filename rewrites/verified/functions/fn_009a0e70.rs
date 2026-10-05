// original: 0x009A0E70 audio_update_large

/// Update one audio emitter for the frame; returns 1 on success, 0 on early-out.
///
/// `this` is the emitter object; the eight stack words are scalar parameters
/// (only the first seven are read: key, aux, mode key, mode aux, blend hint,
/// voice selector, slot tag; the eighth is padding the callee pops).
///
/// Object layout touched here: `+0x08` child pointer (its `+0x20` grandchild
/// pointer with a position at `+0x30`, and flag bytes at `+0x218`/`+0x219`);
/// `+0x94`/`+0x98` voice slots (null-checked; the first one's `+0x3B` mode byte
/// must equal 4); `+0xA0`/`+0xA8`/`+0xAC` result words; `+0x1F0`/`+0x1F4`
/// mix words; `+0x188`/`+0x1A4` flag bytes whose bit 1 is set on the success
/// path; `+0x70`/`+0x80` key-answer slots filled by a four-iteration
/// helper loop; `+0x209` done flag. The second level flag selects the last
/// argument of the second filter call (0 or 1, upper bytes zero).
///
/// Stages: an entry triple of helper calls whose last answer is tested as a
/// SIGNED word (negative means fail immediately); an optional live/voice
/// switch over the mode answer (exact equality against 1, 2, 3, 4, combined
/// with the voice selector: selector 2 always takes arm A, selector 0 picks
/// an arm by mode, anything else takes the default) computing two working
/// levels with ordered-minimum selection (compare-instruction order and NaN
/// semantics: an unordered compare takes the "below" branch); a spatial
/// stage reading the listener position through thread-local state, taking
/// the emitter distance (capped), and running a probe whose answer picks a
/// level adjustment; a context/filter stage passing a caller-side context
/// block through two filter calls; two output-curve evaluations guarded by
/// the voice slots; and a tail storing key answers and setting the done flag.
///
/// The live-handle and gate answers are tested as 8-bit values (only `al`
/// matters); the slot-lookup answer is tested as a SIGNED word (negative
/// fails before the result word is even stored). The mode answer arrives
/// packed with a flag byte in one stubbed helper's odd-addressed out word;
/// the rewrite unpacks it the same way. The entry answers are reused as
/// later call arguments (mode answer to the voice updates, slot answers to
/// the tail updates). One stack slot is read without being written (padding
/// bytes around the flag byte passed to the level resolver); the checker
/// defines unwritten stack as zero and this rewrite uses 0 there.
///
/// Original: 0x009A0E70 (thiscall, eight stack words; true size 2233 bytes,
// the inventory size stops 8 bytes into the epilogue).
lf_checker_rt::export!(thiscall, rw_009A0E70(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, _a7: u32) -> u8 {
    unsafe {
        const OBJ_A: u32 = 0x1288780;
        const OBJ_B: u32 = 0x1165880;
        const OBJ_C: u32 = 0x115DEF0;
        const OBJ_D: u32 = 0x1284A60;
        const G_43B4: u32 = 0x12843B4;
        const G_44E4: u32 = 0x12844E4;
        const G_4408: u32 = 0x1284408;
        const G_GATE: u32 = 0x128437E;
        const G_44B0: u32 = 0x12844B0;
        const G_438C: u32 = 0x128438C;
        const G_735B4: u32 = 0x11735B4;
        const G_LBINIT: u32 = 0x12831D8;
        const G_TLSIDX: u32 = 0x17ABA14;
        const POS_TABLE: u32 = 0x115E420;
        const K_9C: u32 = 0x1038D9C;
        const K_58: u32 = 0x1038D58;
        const K_60: u32 = 0x1038D60;
        const K_A4: u32 = 0x1038DA4;
        const K_70: u32 = 0x1038D70;
        const K_6C: u32 = 0x1038D6C;
        const K_68: u32 = 0x1038D68;
        const K_64: u32 = 0x1038D64;
        const K_54: u32 = 0x1038D54;
        const K_50: u32 = 0x1038D50;
        const K_90: u32 = 0x1038D90;
        const K_94: u32 = 0x1038D94;
        const K_98: u32 = 0x1038D98;
        const K_DE0: u32 = 0x1038DE0;
        const R_ONE: u32 = 0xFE88E8;
        const DIST_CAP: u32 = 0xFE8AFC;
        const ADJ_A: u32 = 0xFE87A4;
        const ADJ_B: u32 = 0xFE8A94;
        const KEYS: u32 = 0x1038E04;
        const CHILD: u32 = 0x08;
        const VOICE_A: u32 = 0x94;
        const VOICE_B: u32 = 0x98;
        const RES_A: u32 = 0xA0;
        const RES_B: u32 = 0xA8;
        const RES_C: u32 = 0xAC;
        const MIX_A: u32 = 0x1F0;
        const MIX_B: u32 = 0x1F4;
        const FLAG_A: u32 = 0x188;
        const FLAG_B: u32 = 0x1A4;
        const FLAG_BIT: u8 = 2;
        const DONE_FLAG: u32 = 0x209;
        const LOOP_BASE: u32 = 0x80;
        const MODE_OK: u8 = 4;
        const CHILD_GC: u32 = 0x20;
        const CHILD_F0: u32 = 0x218;
        const CHILD_HANDLE_BIAS: u32 = 0x3C0;
        const TLS_SLOT: usize = 5;
        const TLS_STRUCT_INDEX: u32 = 0x70;
        const POS_RECORD_LEN: u32 = 64;
        const PROBE_ARG: u32 = 0x7A;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn gf(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(addr))) }
        }
        #[inline(always)]
        unsafe fn gd(addr: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(addr)) }
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

        let obj_a = lf_checker_rt::relocated(OBJ_A);
        // Entry triple.
        let s1: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj_a, a2);
        let w88: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, a3, a1);
        let s3: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj_a, this);
        if (s3 as i32) < 0 {
            return 0;
        }
        wr32(this.wrapping_add(RES_A), s3);
        // Caller-side context block (18 words) for the two filter calls.
        let mut ctx = [0u32; 18];
        lf_checker_rt::callee_thiscall!(4, u32, ctx.as_mut_ptr() as u32);
        let w17 = ctx[17];
        let tb = (((w17 >> 16) & 0xFF) as u8 & 0xDF) | 8;
        ctx[17] = (w17 & 0xFF00FFFF) | ((tb as u32) << 16);
        ctx[7] = gd(G_43B4);
        ctx[10] = a4;
        let mut w60 = gd(G_44E4);
        let child = rd32(this.wrapping_add(CHILD));
        wr32(this.wrapping_add(MIX_A), 0);
        let mut la = 0.0f32;
        let mut lb = gf(G_LBINIT);
        let mut f84 = 0u8;
        let mut f92 = 0u8;
        let r1 = gf(R_ONE);

        // Live/voice dispatch.
        let mut w56 = 0.0f32;
        if child != 0 {
            let c0 = rd8(child.wrapping_add(CHILD_F0));
            let c1 = rd8(child.wrapping_add(CHILD_F0).wrapping_add(1));
            let f30 = if c0 == 0 && c1 != 0 { 1u8 } else { 0u8 };
            let live: u32 = lf_checker_rt::callee_stdcall!(5, u32, child);
            let gate = rd8(lf_checker_rt::relocated(G_GATE));
            if (live as u8) != 0 && gate == 0 {
                // Curve path: straight to the level clamp.
                la = add(mul(gf(K_9C), lb), gf(K_58));
                w60 = gd(G_4408);
            } else if a5 == 3 {
                // Voice selector 3 takes the curve path too.
                la = add(mul(gf(K_9C), lb), gf(K_58));
                w60 = gd(G_4408);
            } else {
                // Mode switch.
                let mut w56b = 0u32;
                let mut comb = (s1 & 0x00FFFFFF) << 8;
                lf_checker_rt::callee_thiscall!(6, u32,
                    lf_checker_rt::relocated(OBJ_B),
                    &mut w56b as *mut u32 as u32,
                    &mut comb as *mut u32 as u32);
                w56 = f32::from_bits(w56b);
                let flag31 = (comb & 0xFF) as u8;
                let sw = (comb >> 8) | (s1 & 0xFF000000);
                let id7: u32 = lf_checker_rt::callee_thiscall!(7, u32,
                    child.wrapping_add(CHILD_HANDLE_BIAS));
                ctx[8] = id7;
                let esi = a5;
                // Min-block operands; set by arms B and default.
                let mut m_x3 = 0.0f32;
                let mut m_x5 = 0.0f32;
                let mut m_x1 = 0.0f32;
                let mut m_x6 = 0.0f32;
                let mut run_min = false;
                if esi == 2 || (esi == 0 && sw == 1) {
                    // Arm A.
                    let x2 = lb;
                    let mut x0 = mul(gf(K_9C), x2);
                    let mut x1 = sub(r1, x2);
                    x0 = add(x0, gf(K_58));
                    x1 = mul(x1, gf(K_60));
                    la = x0;
                    x0 = mul(gf(K_A4), x2);
                    x1 = add(x1, x0);
                    lb = x1;
                } else if esi == 0 && sw == 2 {
                    // Arm B.
                    let x2 = lb;
                    let mut x1 = gf(K_9C);
                    let x5 = gf(K_70);
                    let mut x0 = gf(K_A4);
                    let mut x6 = gf(K_6C);
                    let mut x3 = sub(r1, x2);
                    x1 = mul(x1, x2);
                    x0 = mul(x0, x2);
                    x3 = mul(x3, x5);
                    x1 = add(x1, x6);
                    f84 = 1;
                    x3 = add(x3, x0);
                    la = x1;
                    lb = x3;
                    m_x3 = x3;
                    m_x5 = x5;
                    m_x1 = x1;
                    m_x6 = x6;
                    if flag31 == 0 {
                        // Block C.
                        let mut c0 = mul(gf(K_68), w56);
                        let mut c1 = mul(gf(K_64), w56);
                        c0 = add(c0, m_x6);
                        m_x6 = c0;
                        c0 = sub(r1, w56);
                        c1 = add(c1, c0);
                        c1 = mul(c1, m_x5);
                        m_x5 = c1;
                        m_x1 = la;
                    }
                    run_min = true;
                } else if esi == 0 && sw == 3 {
                    // Arm D (= A plus both flags).
                    let x2 = lb;
                    let mut x0 = mul(gf(K_9C), x2);
                    let mut x1 = sub(r1, x2);
                    x0 = add(x0, gf(K_58));
                    f84 = 1;
                    f92 = 1;
                    x1 = mul(x1, gf(K_60));
                    la = x0;
                    x0 = mul(gf(K_A4), x2);
                    x1 = add(x1, x0);
                    lb = x1;
                } else if esi == 0 && sw == 4 {
                    // Arm E.
                    let mut x0 = mul(gf(K_9C), lb);
                    f84 = 1;
                    f92 = 1;
                    x0 = add(x0, gf(K_94));
                    la = x0;
                    lb = gf(K_98);
                } else {
                    // Default arm.
                    let x2 = lb;
                    let mut x1 = gf(K_9C);
                    let x5 = gf(K_54);
                    let mut x0 = gf(K_A4);
                    let mut x6 = gf(K_50);
                    let mut x3 = sub(r1, x2);
                    x1 = mul(x1, x2);
                    x0 = mul(x0, x2);
                    x3 = mul(x3, x5);
                    x1 = add(x1, x6);
                    x3 = add(x3, x0);
                    la = x1;
                    lb = x3;
                    m_x3 = x3;
                    m_x5 = x5;
                    m_x1 = x1;
                    m_x6 = x6;
                    if flag31 == 0 {
                        // Block G.
                        let mut c1 = mul(gf(K_64), w56);
                        let mut c0 = sub(r1, w56);
                        c1 = add(c1, c0);
                        c0 = mul(gf(K_68), w56);
                        c0 = add(c0, m_x6);
                        m_x6 = c0;
                        c1 = mul(c1, m_x5);
                        m_x5 = c1;
                        m_x1 = la;
                    }
                    run_min = true;
                }
                if run_min {
                    if !(m_x3 > m_x5) {
                        lb = m_x5;
                    }
                    if !(m_x1 > m_x6) {
                        m_x1 = m_x6;
                        la = m_x6;
                    }
                    if f30 != 0 {
                        la = add(gf(K_90), m_x1);
                        if la > 0.0 {
                            la = 0.0;
                        }
                    }
                }
                // Spatial stage.
                debug_assert_eq!(
                    rd32(lf_checker_rt::relocated(G_TLSIDX)) as usize, TLS_SLOT);
                let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
                let index = rd32(tls_base.wrapping_add(TLS_STRUCT_INDEX));
                let entry = lf_checker_rt::relocated(POS_TABLE)
                    .wrapping_add(index.wrapping_mul(POS_RECORD_LEN));
                let t0 = rdf(entry);
                let t1 = rdf(entry.wrapping_add(4));
                let t2 = rdf(entry.wrapping_add(8));
                let gc = rd32(child.wrapping_add(CHILD_GC));
                let mut b8 = 0u32;
                lf_checker_rt::callee_thiscall!(8, u32,
                    lf_checker_rt::relocated(OBJ_C),
                    &mut b8 as *mut u32 as u32,
                    gc.wrapping_add(0x30), 0u32);
                let dx = sub(t0, rdf(gc.wrapping_add(0x30)));
                let dy = sub(t1, rdf(gc.wrapping_add(0x34)));
                let dz = sub(t2, rdf(gc.wrapping_add(0x38)));
                let mut dist =
                    add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
                let cap = gf(DIST_CAP);
                if !(cap > dist) {
                    dist = cap;
                }
                let mut b9 = [f92 as u32, 0u32, 0u32];
                lf_checker_rt::callee_thiscall!(9, u32, b9.as_mut_ptr() as u32);
                let f0 = f32::from_bits(b9[0]);
                let f1 = f32::from_bits(b9[1]);
                let f2 = f32::from_bits(b9[2]);
                let x = add(mul(add(f0, f1), 0.0), f2);
                if x < 0.0 {
                    let t = mul(dist, gf(ADJ_A));
                    let u = sub(r1, t);
                    let v = mul(u, gf(ADJ_B));
                    la = sub(la, v);
                }
                let obj_d = lf_checker_rt::relocated(OBJ_D);
                let a10: u32 = lf_checker_rt::callee_thiscall!(10, u32, obj_d);
                if (a10 as u8) != 0 {
                    let a11: u32 = lf_checker_rt::callee_thiscall!(11, u32, obj_d);
                    if (a11 as u8) == 0 {
                        la = sub(la, gf(ADJ_B));
                    }
                }
                let l12: u32 = lf_checker_rt::callee_stdcall!(12, u32, lb.to_bits());
                ctx[7] = l12;
            }
            // Level clamp: keep only a negative level.
            if !(0.0 > la) {
                la = 0.0;
            }
            // Filter call on voice slot A.
            lf_checker_rt::callee_thiscall!(13, u32, this, w60,
                this.wrapping_add(VOICE_A), ctx.as_mut_ptr() as u32,
                0xFFFFFFFFu32, 0u32, 0u32);
            let o94t = rd32(this.wrapping_add(VOICE_A));
            if o94t != 0 {
                lf_checker_rt::callee_thiscall!(14, u32, o94t, la.to_bits());
            }
        }
        // Voice slot A gate (a null child joins here, past the filter call).
        let o94 = rd32(this.wrapping_add(VOICE_A));
        if o94 != 0 {
            let o94b = rd32(this.wrapping_add(VOICE_A));
            if rd8(o94b.wrapping_add(0x3B)) == MODE_OK {
                let a15: u32 = lf_checker_rt::callee_thiscall!(15, u32, o94b,
                    w88, a1, a6, 0u32);
                if (a15 as u8) == 0 {
                    lf_checker_rt::callee_thiscall!(16, u32, o94b, 0u32);
                    lf_checker_rt::callee_thiscall!(17, u32, this, 1u32);
                    return 0;
                }
            } else {
                lf_checker_rt::callee_thiscall!(16, u32, o94b, 0u32);
                lf_checker_rt::callee_thiscall!(17, u32, this, 1u32);
                return 0;
            }
        } else {
            lf_checker_rt::callee_thiscall!(17, u32, this, 1u32);
            return 0;
        }
        // Success path: mark both flag bytes, then rise and voice updates.
        wr8(this.wrapping_add(FLAG_A), rd8(this.wrapping_add(FLAG_A)) | FLAG_BIT);
        wr8(this.wrapping_add(FLAG_B), rd8(this.wrapping_add(FLAG_B)) | FLAG_BIT);
        let o94c = rd32(this.wrapping_add(VOICE_A));
        let mut o1 = 0u32;
        let mut o2 = 0x46ABE000u32;
        let mut opos = [0u32; 4];
        lf_checker_rt::callee_thiscall!(18, u32, this,
            &mut o1 as *mut u32 as u32,
            &mut o2 as *mut u32 as u32,
            opos.as_mut_ptr() as u32);
        lb = f32::from_bits(o1);
        w56 = f32::from_bits(o2);
        lf_checker_rt::callee_thiscall!(19, u32, o94c, w56.to_bits());
        let s = add(w56, rdf(this.wrapping_add(MIX_A)));
        lf_checker_rt::callee_thiscall!(20, u32, o94c, s.to_bits());
        lf_checker_rt::callee_thiscall!(21, u32, o94c, opos.as_mut_ptr() as u32);
        if w60 == gd(G_44E4) {
            w60 = 0;
            lf_checker_rt::callee_thiscall!(22, u32, this, f84 as u32,
                this.wrapping_add(MIX_B), &mut w60 as *mut u32 as u32, f92 as u32);
            let mut e = w60;
            if (a4 as i32) >= 0 {
                e = e.wrapping_add(a4);
            }
            ctx[10] = e;
            let o98 = rd32(this.wrapping_add(VOICE_B));
            lf_checker_rt::callee_thiscall!(23, u32, this, gd(G_44E4),
                this.wrapping_add(VOICE_B),
                ctx.as_mut_ptr() as u32, 0xFFFFFFFFu32, 0u32, 0u32);
            if o98 != 0 {
                lf_checker_rt::callee_thiscall!(24, u32, o98, la.to_bits());
                let a25: u32 = lf_checker_rt::callee_thiscall!(25, u32, o98,
                    w88, a1, a6, 0u32);
                if (a25 as u8) != 0 {
                    lf_checker_rt::callee_thiscall!(26, u32, o98, w56.to_bits());
                    let s2 = add(w56, rdf(this.wrapping_add(MIX_B)));
                    lf_checker_rt::callee_thiscall!(27, u32, o98, s2.to_bits());
                    let p28: u32 = lf_checker_rt::callee_thiscall!(28, u32,
                        lf_checker_rt::relocated(OBJ_C), 0u32);
                    let q0 = rdf(p28);
                    let q1 = rdf(p28.wrapping_add(4));
                    let q2 = rdf(p28.wrapping_add(8));
                    let opos0 = f32::from_bits(opos[0]);
                    let opos1 = f32::from_bits(opos[1]);
                    let opos2 = f32::from_bits(opos[2]);
                    let d0 = sub(opos0, q0);
                    let d1 = sub(opos1, q1);
                    let d2 = sub(opos2, q2);
                    let mut v96 = [d0.to_bits(), d1.to_bits(), d2.to_bits()];
                    lf_checker_rt::callee_thiscall!(29, u32,
                        v96.as_mut_ptr() as u32,
                        rd32(lf_checker_rt::relocated(K_DE0)), PROBE_ARG);
                    let sum0 = add(d0, q0);
                    v96[0] = sum0.to_bits();
                    let sum1 = add(d1, q1);
                    v96[1] = sum1.to_bits();
                    let sum2 = add(d2, q2);
                    v96[2] = sum2.to_bits();
                    lf_checker_rt::callee_thiscall!(30, u32, o98, v96.as_mut_ptr() as u32);
                } else {
                    lf_checker_rt::callee_thiscall!(31, u32, o98, 0u32);
                }
            }
        }
        // Tail: slot updates, key loop, done flag.
        let esi = a6;
        let s32: u32 = lf_checker_rt::callee_thiscall!(32, u32, obj_a,
            rd32(this.wrapping_add(RES_A)));
        lf_checker_rt::callee_thiscall!(33, u32,
            rd32(this.wrapping_add(VOICE_A)), s32, 1u32, 0xFFFFFFFFu32);
        let o98b = rd32(this.wrapping_add(VOICE_B));
        if o98b != 0 {
            lf_checker_rt::callee_thiscall!(34, u32, o98b, s32, 1u32, 0xFFFFFFFFu32);
        }
        lf_checker_rt::callee_thiscall!(35, u32, obj_a, s3, w88, a1, esi);
        wr32(this.wrapping_add(RES_B), a0);
        wr32(this.wrapping_add(RES_C), esi);
        if a0 == gd(G_44B0) {
            let child2 = rd32(this.wrapping_add(CHILD));
            if child2 != 0 {
                let a36: u32 = lf_checker_rt::callee_thiscall!(36, u32, child2);
                let old = gd(G_438C);
                let new = if (a36 as u8) != 0 { gd(G_735B4) } else { old };
                wr32(lf_checker_rt::relocated(G_438C), new);
            }
        }
        let base = this.wrapping_add(LOOP_BASE);
        let keys = lf_checker_rt::relocated(KEYS);
        let mut k = 0u32;
        while k < 4 {
            let key = rd32(keys.wrapping_add(k.wrapping_mul(4)));
            let ans: u32 = lf_checker_rt::callee_cdecl!(37, u32, key, 0u32);
            wr32(base.wrapping_sub(0x10).wrapping_add(k.wrapping_mul(4)), ans);
            wr32(base.wrapping_add(k.wrapping_mul(4)), 0xFFFFFFFFu32);
            k += 1;
        }
        wr8(this.wrapping_add(DONE_FLAG), 0);
        1
    }
});
