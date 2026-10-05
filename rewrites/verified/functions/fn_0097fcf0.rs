// original: 0x0097FCF0 MOLOTOV_FUSE_FIRE_LOOP_IN_HAND (symbols, low confidence)

/// Refresh one ped's task/effect state and (re)create its missing pieces.
///
/// `this` points to the ped-task object. The object holds four nullable
/// phase slots (`+0x14/+0x18/+0x20/+0x198`, each a small record with live
/// fields at `+0xa4/+0xa8/+0xac`), a pointer to a larger state object at
/// `+0x120`, several flags and timers, and one float at `+0x40` that is
/// rescaled on every call. The function walks the slots in a fixed order:
/// for each slot it either runs the slot's effect chain directly (slot
/// non-null) or runs a creation sequence (a family of `0xE3...` calls) that
/// fills the slot and then runs the chain. Effect chains mix scripted float
/// stages (values arrive through the x87 return channel of small helpers
/// and are combined with `comiss`-gated min/max blends) with record
/// updates. Two globals cache created descriptors (`0x1231738/0x1231740`,
/// guarded by flag bits in `0x123173c`). Three early gates at the top (a
/// mode global, a version pair, a state id) skip everything and run a short
/// teardown path instead.
///
/// Calling convention: thiscall, no stack arguments, returns in `eax` (the
/// last helper's value on the teardown paths, zero on most others). The
/// original keeps a 0xD8-byte aligned frame; the slots below mirror it:
/// temps at `0x08..0x14`, a three-word descriptor triple at `0x18`, a small
/// scratch struct at `0x24`, a second scratch struct at `0x3c`, and the
/// main record at `0x84` (12 words, the bytes at `+0x2c..0x2e` are flag
/// bits set along the way). Callee ids are the checker's, not the game's.
lf_checker_rt::export!(thiscall, rw_0097FCF0(this: u32) -> u32 {
    unsafe {
        // ---- globals (file VAs) ----
        const G_COOKIE: u32 = 0x01057FB4;
        const G_MODE: u32 = 0x011F7060;
        const G_VERA: u32 = 0x012088B4;
        const G_VERB: u32 = 0x00F1C040;
        const G_STATE: u32 = 0x01037720;
        const G_TIMER: u32 = 0x011735B4;
        const G_TIMERF: u32 = 0x011735BC;
        const G_MUL: u32 = 0x0115D968;
        const G_TBL: u32 = 0x0115D988;
        const G_ARR: u32 = 0x01231360;
        const G_D20: u32 = 0x01231738;
        const G_FLAGS: u32 = 0x0123173C;
        const G_D198: u32 = 0x01231740;
        const F_8670: u32 = 0x00FE8670;
        const F_88E8: u32 = 0x00FE88E8;
        const F_8BB0: u32 = 0x00FE8BB0;
        const F_8B38: u32 = 0x00FE8B38;
        const F_8AB8: u32 = 0x00FE8AB8;
        const F_8C10: u32 = 0x00FE8C10;
        const F_8B00: u32 = 0x00FE8B00;
        const F_88B0: u32 = 0x00FE88B0;
        const F_E8D714: u32 = 0x00E8D714;
        // ---- this + field offsets ----
        const T_PTR8: u32 = 0x08;
        const T_PH14: u32 = 0x14;
        const T_PH18: u32 = 0x18;
        const T_PH20: u32 = 0x20;
        const T_F40: u32 = 0x40;
        const T_IDX: u32 = 0x7C;
        const T_OBJ: u32 = 0x120;
        const T_F129: u32 = 0x129;
        const T_P130: u32 = 0x130;
        const T_T138: u32 = 0x138;
        const T_W144: u32 = 0x144;
        const T_T170: u32 = 0x170;
        const T_B174: u32 = 0x174;
        const T_PH198: u32 = 0x198;
        // ---- obj+0x120 field offsets ----
        const O_VT: u32 = 0x00;
        const O_B218: u32 = 0x218;
        const O_B219: u32 = 0x219;
        const O_W224: u32 = 0x224;
        const O_B26C: u32 = 0x26C;
        const O_SUB: u32 = 0x2B0;
        const O_W2C4: u32 = 0x2C4;
        const O_S780: u32 = 0x780;
        const O_PB30: u32 = 0xB30;
        // ---- frame slots (mirror of the original's aligned frame) ----
        const F_TA: usize = 0x08;
        const F_TB: usize = 0x0C;
        const F_TC: usize = 0x10;
        const F_TD: usize = 0x14;
        const F_TR0: usize = 0x18;
        const F_TR1: usize = 0x1C;
        const F_TR2: usize = 0x20;
        const F_IS: usize = 0x24;
        const F_S48: usize = 0x3C;
        const F_5C: usize = 0x5C;
        const F_M: usize = 0x84;
        const F_M90: usize = 0x90;
        const F_M94: usize = 0x94;
        const F_M98: usize = 0x98;
        const F_M9C: usize = 0x9C;
        const F_MA0: usize = 0xA0;
        const F_MA4: usize = 0xA4;
        const F_MA8: usize = 0xA8;
        const F_MB0: usize = 0xB0;
        const F_MB1: usize = 0xB1;
        const F_MB2: usize = 0xB2;
        const F_COOKIE: usize = 0xD0;
        // ---- callee ids (checker contract) ----
        const C_OK: u32 = 1;
        const C_CLR: u32 = 2;
        const C_NEW: u32 = 3;
        const C_NEW2: u32 = 43;
        const C_FIN: u32 = 4;
        const C_MK1: u32 = 5;
        const C_FMT: u32 = 6;
        const C_RUN: u32 = 7;
        const C_ALT: u32 = 8;
        const C_BYE: u32 = 9;
        const C_RST: u32 = 10;
        const C_F1A: u32 = 11;
        const C_F1B: u32 = 40;
        const C_F1C: u32 = 41;
        const C_F2A: u32 = 12;
        const C_F2B: u32 = 44;
        const C_F2C: u32 = 45;
        const C_MK18: u32 = 13;
        const C_MK20: u32 = 36;
        const C_ATT: u32 = 14;
        const C_F3A: u32 = 15;
        const C_F3B: u32 = 46;
        const C_F3C: u32 = 47;
        const C_USE: u32 = 16;
        const C_SEQ: u32 = 17;
        const C_CFG: u32 = 18;
        const C_EMIT: u32 = 19;
        const C_ONE: u32 = 20;
        const C_ZERO: u32 = 21;
        const C_SUB: u32 = 22;
        const C_MK14: u32 = 23;
        const C_PREP: u32 = 24;
        const C_SET: u32 = 25;
        const C_K3: u32 = 26;
        const C_K1: u32 = 27;
        const C_LIM: u32 = 28;
        const C_SEL: u32 = 29;
        const C_F4A: u32 = 30;
        const C_F4B: u32 = 37;
        const C_F4C: u32 = 38;
        const C_IDX: u32 = 31;
        const C_K7: u32 = 32;
        const C_MK198: u32 = 33;
        const C_CKY: u32 = 34;
        const C_VT: u32 = 35;
        const C_MID: u32 = 39;
        const VT_SLOT: u32 = 0xEC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 { unsafe { (a as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) { unsafe { (a as *mut u32).write_unaligned(v) } }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) { unsafe { (a as *mut u8).write(v) } }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 { unsafe { f32::from_bits(rd32(a)) } }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) { unsafe { wr32(a, v.to_bits()) } }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gset(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 { unsafe { f32::from_bits(g32(va)) } }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Exact `cvttss2si` (truncate; NaN, infinities and out-of-range
        /// magnitudes give 0x80000000, unlike Rust's saturating `as`).
        #[inline(always)]
        fn cvt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000
            } else {
                x as i32 as u32
            }
        }
        /// Manager object behind a possibly-null slot: 0xff tag means absent.
        #[inline(always)]
        unsafe fn resolve(tagged: u32) -> u32 {
            unsafe {
                let tag = rd8(tagged.wrapping_add(4)) as u32;
                if tag == 0xff {
                    return 0;
                }
                let row = rd8(tagged.wrapping_add(0x40)) as u32;
                let col = tag.wrapping_mul(g32(G_MUL));
                let tbl = g32(G_TBL);
                col.wrapping_add(rd32(
                    tbl.wrapping_add(row.wrapping_mul(0x6f40)).wrapping_add(0x6f14),
                ))
            }
        }
        let mut f = [0u8; 0xD8];
        let fbase = unsafe { f.as_mut_ptr() as u32 };
        let fp = |off: usize| fbase.wrapping_add(off as u32);
        let mut eax: u32 = 0;
        wr32(fp(F_COOKIE), g32(G_COOKIE));

        // Entry gates: any one skips the refresh and runs teardown.
        if g32(G_MODE) == 1 || g32(G_VERA) != g32(G_VERB) || g32(G_STATE) == 0x12 {
            if rd8(this.wrapping_add(T_F129)) != 0 {
                eax = lf_checker_rt::callee_thiscall!(C_BYE, u32, this);
                wr8(this.wrapping_add(T_F129), 0);
            }
            eax = lf_checker_rt::callee_thiscall!(C_RST, u32, this);
            let p8 = rd32(this.wrapping_add(T_PTR8));
            if p8 != 0 {
                let obj = rd32(this.wrapping_add(T_OBJ));
                let lo: u32 =
                    if rd8(obj.wrapping_add(O_B219)) != 0 { 0xc8 } else { 0x3e8 };
                eax = lf_checker_rt::callee_thiscall!(C_LIM, u32, p8, lo, 0xfa0, 0x3f000000);
            }
            let p8b = rd32(this.wrapping_add(T_PTR8));
            if p8b != 0 {
                let obj = rd32(this.wrapping_add(T_OBJ));
                let pushed = if obj != 0
                    && rd8(obj.wrapping_add(O_B26C)) & 4 != 0
                    && rd32(obj.wrapping_add(O_PB30)) != 0
                {
                    // The re-test below repeats the flag test above, so the
                    // null arm is dead; it is kept for fidelity.
                    let b = if rd8(obj.wrapping_add(O_B26C)) & 4 != 0 {
                        rd32(obj.wrapping_add(O_PB30))
                    } else {
                        0
                    };
                    rd32(b.wrapping_add(0xc30))
                } else {
                    0
                };
                eax = lf_checker_rt::callee_thiscall!(C_SEL, u32, p8b, pushed);
            }
            eax = rd32(this.wrapping_add(T_PTR8));
            if eax == 0 {
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return 0;
            }
            let obj = rd32(this.wrapping_add(T_OBJ));
            if rd8(obj.wrapping_add(O_B218)) == 0 && rd8(obj.wrapping_add(O_B219)) != 0 {
                wr8(eax.wrapping_add(0xaa), 1);
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return eax;
            }
            wr8(eax.wrapping_add(0xaa), 0);
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eax;
        }
        // Main path.
        let obj0 = rd32(this.wrapping_add(T_OBJ));
        let timer = g32(G_TIMER);
        let ok: u8 =
            lf_checker_rt::callee_thiscall!(C_OK, u8, rd32(obj0.wrapping_add(O_W224)));
        wr32(fp(F_TD), timer);
        if ok == 0
            && rd8(this.wrapping_add(T_B174)) != ok
            && rd32(this.wrapping_add(T_T170)).wrapping_add(0x3e8) < timer
        {
            lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
            wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
            wr32(fp(F_M90), obj0.wrapping_add(O_S780));
            let nid: u32 = lf_checker_rt::callee_thiscall!(C_NEW, u32, fp(F_M));
            let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, nid);
            let made: u8 = lf_checker_rt::callee_thiscall!(
                C_MK1, u8, this, lf_checker_rt::relocated(0xe8cc24), fp(F_M), nid, fin, 0u32);
            if made != 0 {
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x37);
                let h: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc34), 0u32);
                eax = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, h, 0u32, 0u32, 1u32, fp(F_M), fp(F_TR0),
                    rd32(this.wrapping_add(T_OBJ)), nid);
            } else {
                eax = lf_checker_rt::callee_cdecl!(C_ALT, u32, nid);
            }
            wr32(this.wrapping_add(T_T170), rd32(fp(F_TD)));
        }

        let obj1 = rd32(this.wrapping_add(T_OBJ));
        let ok2: u8 =
            lf_checker_rt::callee_thiscall!(C_OK, u8, rd32(obj1.wrapping_add(O_W224)));
        wr8(this.wrapping_add(T_B174), ok2);
        if rd8(this.wrapping_add(T_F129)) != 0 {
            eax = lf_checker_rt::callee_thiscall!(C_BYE, u32, this);
            wr8(this.wrapping_add(T_F129), 0);
        }
        eax = lf_checker_rt::callee_thiscall!(C_RST, u32, this);

        // Gate slot: indexed descriptor or skip to the sub-object check.
        let arr_base = lf_checker_rt::relocated(G_ARR);
        let elem = rd32(
            arr_base.wrapping_add(rd32(this.wrapping_add(T_IDX)).wrapping_mul(4)),
        );
        wr32(fp(F_TB), elem);
        if elem != 0 {
            let a: f32 = lf_checker_rt::callee_thiscall!(C_F1A, f32, this, 1u32);
            wrf(fp(F_TC), a);
            let b: f32 = lf_checker_rt::callee_thiscall!(C_F1B, f32, this, 0u32);
            wrf(fp(F_TA), b);
            // max, NaN-tolerant the way `comiss`+`ja` is (NaN takes b).
            let mut m = rdf(fp(F_TC));
            if !(m > rdf(fp(F_TA))) {
                m = rdf(fp(F_TA));
            }
            wrf(fp(F_TC), m);
            let c: f32 =
                lf_checker_rt::callee_thiscall!(C_F2A, f32, lf_checker_rt::relocated(0x1231478), m.to_bits());
            wrf(fp(F_TA), c);
            // min against the ceiling, NaN keeps the value (`jbe`).
            let mut d = rdf(fp(F_TA));
            if d > gf(F_88E8) {
                d = gf(F_88E8);
            }
            wrf(fp(F_TA), d);
            let e: f32 = lf_checker_rt::callee_thiscall!(
                C_F2B, f32, lf_checker_rt::relocated(0x12315ec), rdf(fp(F_TC)).to_bits());
            wrf(fp(F_TC), e);
            let scaled = mul(rdf(fp(F_TC)), gf(F_8BB0));
            wr32(fp(F_TC), cvt(scaled));
            if rdf(fp(F_TA)) > gf(F_8670) {
                let slot18 = this.wrapping_add(T_PH18);
                if rd32(this.wrapping_add(T_PH18)) == 0 {
                    lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
                    wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
                    wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
                    let e12 = rd32(rd32(fp(F_TB)).wrapping_add(0x12));
                    lf_checker_rt::callee_thiscall!(
                        C_MK18, u32, this, e12, slot18, fp(F_M),
                        0xffffffffu32, 0u32, 0u32);
                    if rd32(slot18) != 0 {
                        wr32(fp(F_TR0), 0);
                        wr32(fp(F_TR1), 0xffffffff);
                        wr32(fp(F_TR2), 0x58);
                        let e12b = rd32(rd32(fp(F_TB)).wrapping_add(0x12));
                        let id: u32 = lf_checker_rt::callee_cdecl!(
                            C_RUN, u32, e12b, 0u32, 1u32, 1u32, fp(F_M),
                            fp(F_TR0), rd32(this.wrapping_add(T_OBJ)),
                            0xffffffffu32);
                        eax = id;
                        let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                        eax = fin;
                        let ph = rd32(this.wrapping_add(T_PH18));
                        wr32(ph.wrapping_add(0xa4), id);
                        wr32(ph.wrapping_add(0xa8), fin);
                        wr32(ph.wrapping_add(0xac), 0);
                        lf_checker_rt::callee_thiscall!(
                            C_ATT, u32, rd32(slot18), 0u32, 0u32, 0u32);
                    }
                }
                if rd32(slot18) != 0 {
                    let g: f32 = lf_checker_rt::callee_cdecl!(
                        C_F3A, f32, rdf(fp(F_TA)).to_bits());
                    wrf(fp(F_TB), g);
                    let ph = rd32(slot18);
                    lf_checker_rt::callee_thiscall!(
                        C_USE, u32, ph, rdf(fp(F_TB)).to_bits());
                    lf_checker_rt::callee_thiscall!(C_SEQ, u32, ph, rd32(fp(F_TC)));
                    lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x58u32);
                    let ph2 = rd32(slot18);
                    let x0 = rdf(fp(F_TB));
                    let ec = rd32(ph2.wrapping_add(0xa4));
                    let ed = rd32(fp(F_TC));
                    wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2 | 8);
                    wrf(fp(F_M90), x0);
                    wr32(fp(F_MA0), ed);
                    eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
                }
            } else {
                let ph = rd32(this.wrapping_add(T_PH18));
                if ph != 0 {
                    eax = lf_checker_rt::callee_cdecl!(
                        C_ONE, u32, rd32(ph.wrapping_add(0xa4)));
                    eax = lf_checker_rt::callee_thiscall!(
                        C_ZERO, u32, rd32(this.wrapping_add(T_PH18)), 0u32);
                }
            }
        }
        // Sub-object must be live and in state 5 for the +0x14 chain.
        let ob2 = rd32(this.wrapping_add(T_OBJ));
        let mut ph14_ok = false;
        if rd32(ob2.wrapping_add(O_W2C4)) != 0 {
            let s1: u32 = lf_checker_rt::callee_thiscall!(
                C_SUB, u32, rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_SUB));
            eax = s1;
            if s1 != 0 {
                let s2: u32 = lf_checker_rt::callee_thiscall!(
                    C_SUB, u32, rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_SUB));
                eax = s2;
                if rd32(s2.wrapping_add(0x18)) == 5 {
                    ph14_ok = true;
                }
            }
        }
        if ph14_ok {
            let slot14 = this.wrapping_add(T_PH14);
            // The s3 chain runs only right after creation, never on the
            // direct-run path (which jumps straight to the hook below).
            let was_null = rd32(this.wrapping_add(T_PH14)) == 0;
            if was_null {
                lf_checker_rt::callee_thiscall!(
                    C_MK14, u32, this, lf_checker_rt::relocated(0xe8cc44), slot14, 1u32, 0u32,
                    rd32(this.wrapping_add(T_PTR8)), 0xffffffffu32, 0u32, 0u32);
            }
            if was_null && rd32(slot14) != 0 {
                lf_checker_rt::callee_thiscall!(C_PREP, u32, this, 3u32, fp(F_IS));
                lf_checker_rt::callee_thiscall!(C_SET, u32, rd32(slot14), fp(F_IS));
                lf_checker_rt::callee_thiscall!(C_K3, u32, rd32(slot14), 3u32);
                lf_checker_rt::callee_thiscall!(C_K1, u32, rd32(slot14), 1u32);
                lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_S48));
                wr32(fp(F_5C), rd32(this.wrapping_add(T_PTR8)));
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x4f);
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x4fu32);
                wr8(fp(F_MB0), rd8(fp(F_MB0)) | 4);
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 4 | 0x20);
                wrf(fp(F_M94), rdf(fp(F_IS)));
                wrf(fp(F_M98), rdf(fp(F_IS).wrapping_add(4)));
                wrf(fp(F_M9C), rdf(fp(F_IS).wrapping_add(8)));
                wr32(fp(F_MA8), 3);
                wr8(fp(F_MB2), 1);
                let h: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc64), 0u32);
                let id0: u32 = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, h, 0u32, 1u32, 0u32, fp(F_S48), fp(F_TR0),
                    rd32(this.wrapping_add(T_OBJ)), 0xffffffffu32);
                eax = id0;
                let id: u32 = lf_checker_rt::callee_cdecl!(
                    C_MID, u32, id0, fp(F_M), rd32(this.wrapping_add(T_OBJ)));
                eax = id;
                let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                eax = fin;
                let ph = rd32(this.wrapping_add(T_PH14));
                wr32(ph.wrapping_add(0xa4), id);
                wr32(ph.wrapping_add(0xa8), fin);
                wr32(ph.wrapping_add(0xac), 0);
                lf_checker_rt::callee_thiscall!(
                    C_ATT, u32, rd32(slot14), 0u32, 0u32, 0u32);
            }
            if rd32(slot14) != 0 {
                let ob3 = rd32(this.wrapping_add(T_OBJ));
                let timer2 = g32(G_TIMER);
                let hook: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(
                        rd32(ob3.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                    ) as usize);
                let got = hook(ob3, fp(F_IS));
                let y = rdf(got.wrapping_add(4));
                let mut x = rdf(got);
                let mut z = rdf(got.wrapping_add(8));
                x = mul(x, x);
                let yy = mul(y, y);
                z = mul(z, z);
                x = add(x, yy);
                let cap = gf(F_88E8);
                x = add(x, z);
                let mut n = x.sqrt();
                n = mul(n, gf(G_TIMERF));
                n = mul(n, gf(F_8B38));
                wrf(fp(F_TB), n);
                if !(cap > n) {
                    wrf(fp(F_TB), cap);
                }
                let q: f32 = lf_checker_rt::callee_thiscall!(C_F1C, f32, this, 2u32);
                wrf(fp(F_TC), q);
                let bl = mul(rdf(fp(F_TC)), rdf(fp(F_TB)));
                let r: f32 = lf_checker_rt::callee_thiscall!(
                    C_F4A, f32, this.wrapping_add(0x90), bl.to_bits(), timer2);
                wrf(fp(F_TC), r);
                let s: f32 =
                    lf_checker_rt::callee_thiscall!(C_K7, f32, lf_checker_rt::relocated(0x1289230), 7u32);
                wrf(fp(F_TB), s);
                let mut x2 = rdf(fp(F_TB));
                let mut x1 = rdf(fp(F_TC));
                let mut x3 = x2;
                x3 = mul(x3, gf(F_8AB8));
                x2 = mul(x2, gf(F_8C10));
                let mut x0 = x1;
                x0 = mul(x0, gf(F_8B00));
                x1 = mul(x1, gf(F_E8D714));
                x3 = add(x3, x0);
                let ph = rd32(this.wrapping_add(T_PH14));
                x2 = add(x2, x1);
                wrf(fp(F_TB), x3);
                let cnt = cvt(x2);
                lf_checker_rt::callee_thiscall!(C_USE, u32, ph, x3.to_bits());
                lf_checker_rt::callee_thiscall!(
                    C_SEQ, u32, rd32(this.wrapping_add(T_PH14)), cnt);
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x4fu32);
                let ph2 = rd32(this.wrapping_add(T_PH14));
                let y0 = rdf(fp(F_TB));
                let ec = rd32(ph2.wrapping_add(0xa4));
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2 | 8);
                wrf(fp(F_M90), y0);
                wr32(fp(F_MA0), cnt);
                eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
            }
        } else {
            let ph = rd32(this.wrapping_add(T_PH14));
            if ph != 0 {
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(ph.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(this.wrapping_add(T_PH14)), 0u32);
            }
        }

        // Optional slot at +0x130, live only before its deadline.
        let p130 = rd32(this.wrapping_add(T_P130));
        if p130 != 0 {
            let dl = rd32(this.wrapping_add(T_T138));
            if dl != 0 && dl < g32(G_TIMER) {
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(p130.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(this.wrapping_add(T_P130)), 0u32);
            }
        }

        // Limit + selector on the +0x8 object.
        let p8 = rd32(this.wrapping_add(T_PTR8));
        if p8 != 0 {
            let obj = rd32(this.wrapping_add(T_OBJ));
            let lo: u32 = if rd8(obj.wrapping_add(O_B219)) != 0 { 0xc8 } else { 0x3e8 };
            eax = lf_checker_rt::callee_thiscall!(C_LIM, u32, p8, lo, 0xfa0, 0x3f000000);
        }
        let p8b = rd32(this.wrapping_add(T_PTR8));
        if p8b != 0 {
            let obj = rd32(this.wrapping_add(T_OBJ));
            let pushed = if obj != 0
                && rd8(obj.wrapping_add(O_B26C)) & 4 != 0
                && rd32(obj.wrapping_add(O_PB30)) != 0
            {
                let b = if rd8(obj.wrapping_add(O_B26C)) & 4 != 0 {
                    rd32(obj.wrapping_add(O_PB30))
                } else {
                    0
                };
                rd32(b.wrapping_add(0xc30))
            } else {
                0
            };
            eax = lf_checker_rt::callee_thiscall!(C_SEL, u32, p8b, pushed);
        }
        let p8c = rd32(this.wrapping_add(T_PTR8));
        if p8c != 0 {
            let base = rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_B218);
            let v: u8 =
                if rd8(base) == 0 && rd8(base.wrapping_add(1)) != 0 { 1 } else { 0 };
            wr8(p8c.wrapping_add(0xaa), v);
        }
        // Blend the +0x40 float, then the +0x20 phase.
        let td = rd32(fp(F_TD));
        let f40 = rdf(this.wrapping_add(T_F40));
        let h2: f32 = lf_checker_rt::callee_thiscall!(
            C_F4B, f32, this.wrapping_add(0x24), f40.to_bits(), td);
        wrf(fp(F_TA), h2);
        let p20 = rd32(this.wrapping_add(T_PH20));
        let f40b = rdf(this.wrapping_add(T_F40));
        wrf(this.wrapping_add(T_F40), mul(f40b, gf(F_88B0)));
        let h2b = rdf(fp(F_TA));
        let slot20 = this.wrapping_add(T_PH20);
        if p20 == 0 && h2b > gf(F_8670) {
            let mut fl = g32(G_FLAGS);
            if fl & 1 == 0 {
                fl |= 1;
                gset(G_FLAGS, fl);
                let d: u32 = lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8cc94), 0u32);
                eax = d;
                gset(G_D20, d);
            }
        }
        if p20 == 0 && h2b > gf(F_8670) {
            let dcur = g32(G_D20);
            lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
            wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
            wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
            lf_checker_rt::callee_thiscall!(
                C_MK20, u32, this, dcur, slot20, fp(F_M),
                0xffffffffu32, 0u32, 0u32);
            if rd32(this.wrapping_add(T_PH20)) != 0 {
                wr32(fp(F_TR0), 0);
                wr32(fp(F_TR1), 0xffffffff);
                wr32(fp(F_TR2), 0x5f);
                let id: u32 = lf_checker_rt::callee_cdecl!(
                    C_RUN, u32, g32(G_D20), 0u32, 1u32, 1u32, fp(F_M),
                    fp(F_TR0), rd32(this.wrapping_add(T_OBJ)), 0xffffffffu32);
                eax = id;
                let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, id);
                eax = fin;
                let ph = rd32(this.wrapping_add(T_PH20));
                wr32(ph.wrapping_add(0xa4), id);
                wr32(ph.wrapping_add(0xa8), fin);
                wr32(ph.wrapping_add(0xac), 0);
                lf_checker_rt::callee_thiscall!(
                    C_ATT, u32, rd32(slot20), 0u32, 0u32, 0u32);
            }
        }
        // Run the +0x20 slot, then the tail chain.
        let xh = h2b;
        eax = rd32(slot20);
        if eax != 0 {
            let g: f32 = lf_checker_rt::callee_cdecl!(C_F3B, f32, xh.to_bits());
            wrf(fp(F_TB), g);
            eax = rd32(slot20);
            let mgr = resolve(eax);
            eax = lf_checker_rt::callee_thiscall!(
                C_IDX, u32, mgr, rdf(fp(F_TB)).to_bits());
            if gf(F_8670) >= rdf(fp(F_TA)) {
                eax = rd32(slot20);
                eax = lf_checker_rt::callee_cdecl!(
                    C_ONE, u32, rd32(eax.wrapping_add(0xa4)));
                eax = lf_checker_rt::callee_thiscall!(
                    C_ZERO, u32, rd32(slot20), 0u32);
            }
        }
        let ob4 = rd32(this.wrapping_add(T_OBJ));
        if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 && rd32(ob4.wrapping_add(O_PB30)) != 0 {
            wr32(this.wrapping_add(T_W144), 0);
        }
        if rd8(ob4.wrapping_add(O_B218)) != 0 || rd8(ob4.wrapping_add(O_B219)) == 0 {
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eax;
        }
        // Second hook site: through the sub-object when flagged, else
        // direct; a mismatched sub-state skips to the +0x198 single-shot.
        let mut skip_hook = false;
        let mut got2 = 0u32;
        let mut hconst = 0u32;
        if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 && rd32(ob4.wrapping_add(O_PB30)) != 0 {
            let b = if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 {
                rd32(ob4.wrapping_add(O_PB30))
            } else {
                0
            };
            if rd32(b.wrapping_add(0x1304)) != 1 {
                skip_hook = true;
            } else {
                let ob = if rd8(ob4.wrapping_add(O_B26C)) & 4 != 0 {
                    rd32(ob4.wrapping_add(O_PB30))
                } else {
                    0
                };
                let hook: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(
                        rd32(ob.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                    ) as usize);
                got2 = hook(ob, fp(F_IS));
                hconst = lf_checker_rt::relocated(0x12314cc);
            }
        } else {
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(
                    rd32(ob4.wrapping_add(O_VT)).wrapping_add(VT_SLOT),
                ) as usize);
            got2 = hook(ob4, fp(F_IS));
            hconst = lf_checker_rt::relocated(0x1231548);
        }
        if !skip_hook {
            let y = rdf(got2.wrapping_add(4));
            let mut x = rdf(got2);
            let mut z = rdf(got2.wrapping_add(8));
            x = mul(x, x);
            let yy = mul(y, y);
            z = mul(z, z);
            x = add(x, yy);
            x = add(x, z);
            let n = x.sqrt();
            let r: f32 =
                lf_checker_rt::callee_thiscall!(C_F2C, f32, hconst, n.to_bits());
            let tdv = rd32(fp(F_TD));
            let r2: f32 = lf_checker_rt::callee_thiscall!(
                C_F4C, f32, this.wrapping_add(0x17c), r.to_bits(), tdv);
            wrf(fp(F_TD), r2);
            if rdf(fp(F_TD)) > gf(F_8670) {
                let slot198 = this.wrapping_add(T_PH198);
                if rd32(this.wrapping_add(T_PH198)) == 0 {
                    let mut fl = g32(G_FLAGS);
                    if fl & 2 == 0 {
                        fl |= 2;
                        gset(G_FLAGS, fl);
                        let d: u32 =
                            lf_checker_rt::callee_cdecl!(C_FMT, u32, lf_checker_rt::relocated(0xe8ccc0), 0u32);
                        eax = d;
                        gset(G_D198, d);
                    }
                    lf_checker_rt::callee_thiscall!(C_CLR, u32, fp(F_M));
                    wr32(fp(F_M90), rd32(this.wrapping_add(T_OBJ)).wrapping_add(O_S780));
                    wr32(fp(F_MA4), rd32(this.wrapping_add(T_PTR8)));
                    let nid: u32 = lf_checker_rt::callee_thiscall!(C_NEW2, u32, fp(F_M));
                    wr32(fp(F_TA), nid);
                    let fin: u32 = lf_checker_rt::callee_cdecl!(C_FIN, u32, nid);
                    let made: u8 = lf_checker_rt::callee_thiscall!(
                        C_MK198, u8, this, g32(G_D198), slot198, fp(F_M),
                        rd32(fp(F_TA)), fin, 0u32);
                    if made != 0 {
                        wr32(fp(F_TR0), 0);
                        wr32(fp(F_TR1), 0xffffffff);
                        wr32(fp(F_TR2), 0x38);
                        eax = lf_checker_rt::callee_cdecl!(
                            C_RUN, u32, g32(G_D198), 0u32, 1u32, 1u32, fp(F_M),
                            fp(F_TR0), rd32(this.wrapping_add(T_OBJ)),
                            rd32(fp(F_TA)));
                    } else {
                        eax = lf_checker_rt::callee_cdecl!(C_ALT, u32, rd32(fp(F_TA)));
                    }
                }
                let x0 = rdf(fp(F_TD));
                let edi2 = rd32(slot198);
                if edi2 == 0 {
                    lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                    return eax;
                }
                let g: f32 = lf_checker_rt::callee_cdecl!(C_F3C, f32, x0.to_bits());
                wrf(fp(F_TB), g);
                let x0b = rdf(fp(F_TB));
                lf_checker_rt::callee_thiscall!(C_USE, u32, edi2, x0b.to_bits());
                lf_checker_rt::callee_thiscall!(C_CFG, u32, fp(F_M), 0x38u32);
                let ph = rd32(slot198);
                let y0 = rdf(fp(F_TB));
                let ec = rd32(ph.wrapping_add(0xa4));
                wr8(fp(F_MB1), rd8(fp(F_MB1)) | 2);
                wrf(fp(F_M90), y0);
                eax = lf_checker_rt::callee_cdecl!(C_EMIT, u32, ec, fp(F_M));
                lf_checker_rt::callee_cdecl!(C_CKY, u32,);
                return eax;
            }
        }
        // Single-shot fallback for the +0x198 slot.
        let eph = rd32(this.wrapping_add(T_PH198));
        if eph == 0 {
            lf_checker_rt::callee_cdecl!(C_CKY, u32,);
            return eph;
        }
        eax = lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(eph.wrapping_add(0xa4)));
        eax = lf_checker_rt::callee_thiscall!(
            C_ZERO, u32, rd32(this.wrapping_add(T_PH198)), 0u32);
        lf_checker_rt::callee_cdecl!(C_CKY, u32,);
        eax
    }
});