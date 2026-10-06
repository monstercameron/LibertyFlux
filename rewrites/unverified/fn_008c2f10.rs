// original: 0x008c2f10 audio_mix_update (proposed)

/// Advance one audio mixing/update step.
///
/// Takes no arguments and no object (cdecl, balanced frame, callee-saved
/// registers preserved). Reads several dozen float and flag globals, drives
/// thirty-seven intercepted callees, and writes exactly three globals: a
/// level float, a state byte, and an output-level float. The return value is
/// whatever the final tail callee answers (scripted zero under the checker).
///
/// Behaviour in order: scale a seed float and truncate it to an integer
/// (x87 truncate control word, replicated exactly including NaN and
/// out-of-range giving zero); fetch an auxiliary float through an
/// out-pointer; combine them into a clamped level and store it; smooth the
/// level and, when an object lookup succeeds, push two parameters and an
/// apply call through it. Read a tap float and a factor byte (the byte
/// becomes a float), fold a mode byte and two compare bytes into a starting
/// mix value, and select between that value and the tick constant through a
/// chain of three polled status bytes. Take the maximum with the tap,
/// normalise by a divisor (the quotient of a quotient, with the zero and
/// unordered cases falling back to fixed values), smooth the result, and
/// gate two mix buses on further flag bytes and a status call. Sum a group
/// of buses through one of two factor sets chosen by the mode triple
/// (mode word, id pair, stage word), and push the sums through object
/// setters, with a polled fallback value when the triple selects the idle
/// path. Run a two-argument pair call whose arguments depend on one more
/// poll, convert two integer globals to scaled floats, and either push them
/// or push a fixed near-one constant pair depending on the same triple.
/// When two mixer objects are present, scan for a tap object, resolve a
/// count, fetch an entry for an index below the count (unsigned compare),
/// and fold table bytes into the buses; run a short gate (ordered-equal on
/// an unordered-aware compare, or a flag byte) that either zeroes three
/// buses or recomputes them from two more smoothed values. Run a state
/// check with a table-byte fallback, an optional two-level boost, and two
/// truncated-scale power stages whose double arguments cross the callee in
/// vector registers; mix, scale, smooth and push the results into the two
/// mixer objects and commit the output level. Past the join, run a second
/// unordered-aware gate selecting between a summed push and a fixed
/// negative push, resolve a named object and repeat the state/pow/mix/smooth
/// sequence through it, then run a final power stage, two mix/smooth pairs,
/// and a probe call with two frame out-pointers (a float and a flag byte)
/// whose results steer one more smooth call, four gated pushes, and the
/// choice of the last mix input. When three final objects are present, run
/// two pair smooths and three tick-relative clamps (each limited to the
/// range 0 to tick, NaN passing through as itself) pushed through the
/// objects. The tail pushes a fixed unity constant when its flag is set,
/// otherwise polls once more and pushes either zero or a table float
/// depending on two flag bytes and an object-table lookup (the index is
/// never negative in practice: that path reads address zero and faults).
///
/// Float order follows the original operation by operation through the
/// small ordered helpers; integer to float conversions are exact and the
/// truncate helper reproduces the x87 store including its edge cases.
/// All comparisons against callee answers keep the original's signedness:
/// the entry-index compare is unsigned, the boost-level compare is signed.
/// The pow helper takes its double arguments exactly as the original holds
/// them (upper halves zero) and narrows the double answer like the
/// original's convert instruction.
///
/// Two quirks of the original are reproduced: one frame word is read before
/// anything writes it (the checker fills it with zero), and one stale
/// frame slot is reused as a mix argument when the big middle block is
/// skipped (also zero under the checker). Both are documented in the
/// proof record.
lf_checker_rt::export!(cdecl, rw_008c2f10() -> u32 {
    unsafe {
        const F_TICK: u32 = 0x00FE88E8;
        const F_MIX: u32 = 0x01292444;
        const F_MIX_K: u32 = 0x01030CCC;
        const F_SCALE_A: u32 = 0x011735BC;
        const F_SCALE_B: u32 = 0x00FE8C58;
        const G_LEVEL: u32 = 0x01030CC4;
        const F_CLAMP_K: u32 = 0x01030CC8;
        const G_LOOKUP_ARG: u32 = 0x0129243C;
        const B_FLAG0: u32 = 0x011618F0;
        const B_MODE09: u32 = 0x011609F6;
        const F_FACTOR_K: u32 = 0x00FE86E8;
        const B_OBJ_ALIVE: u32 = 0x01161518;
        const B_CMP_A: u32 = 0x01161547;
        const B_CMP_B: u32 = 0x01161543;
        const B_STATE: u32 = 0x011618FA;
        const F_DIV: u32 = 0x01030C58;
        const F_ALT: u32 = 0x00FE8DF8;
        const B_GATE1: u32 = 0x01162523;
        const B_GATE2: u32 = 0x0116252D;
        const B_ALT_FLAG: u32 = 0x011618FB;
        const B_X2GATE: u32 = 0x01160C3D;
        const G_OBJ3C: u32 = 0x0116253C;
        const G_MODE: u32 = 0x011F7060;
        const G_ID_A: u32 = 0x012088B4;
        const G_ID_B: u32 = 0x00F1C040;
        const G_STAGE: u32 = 0x01037720;
        const STAGE_IDLE: u32 = 0x12;
        const F_SUM_A: u32 = 0x01176D24;
        const F_SUM_B: u32 = 0x01176D10;
        const F_SUM_C: u32 = 0x01176D3C;
        const G_OBJB0: u32 = 0x012843B0;
        const G_OBJ4C: u32 = 0x0116254C;
        const B_SEL1: u32 = 0x01295864;
        const B_SEL2: u32 = 0x011D7629;
        const B_SEL3: u32 = 0x0128465A;
        const B_SEL4: u32 = 0x0115DBFC;
        const F_PAIR_A: u32 = 0x01030C4C;
        const F_PAIR_K: u32 = 0x00E7E86C;
        const F_PAIR_B: u32 = 0x01030C50;
        const G_MOVD_A: u32 = 0x01160CA8;
        const G_MOVD_B: u32 = 0x01160CA4;
        const F_INT_K: u32 = 0x00FE8798;
        const G_MOVD_C: u32 = 0x01160CAC;
        const G_OBJ40: u32 = 0x01162540;
        const G_OBJ44: u32 = 0x01162544;
        const G_INDEX: u32 = 0x01030090;
        const F_IDX_A: u32 = 0x01030C38;
        const F_IDX_B: u32 = 0x01030C34;
        const G_EDI_SRC: u32 = 0x01284644;
        const B_CMP_C: u32 = 0x011D7634;
        const B_CMP_D: u32 = 0x011D7638;
        const F_F20: u32 = 0x01030C48;
        const F_F18: u32 = 0x01030C40;
        const F_ESI6: u32 = 0x01030C44;
        const F_UCMP: u32 = 0x0115DEDC;
        const F_S1ARG: u32 = 0x0115DED8;
        const F_POST54: u32 = 0x01030C54;
        const B_UCMP_GATE: u32 = 0x0128465A;
        const B_STATE_B: u32 = 0x01162521;
        const G_TBL_SEL: u32 = 0x0128AADC;
        const TBL_SEL_ON: u32 = 2;
        const G_TBL_IDX: u32 = 0x0128AAC0;
        const G_TBL_BASE: u32 = 0x0128AC92;
        const B_SLOT_A_GATE: u32 = 0x01176BC8;
        const G_BIG_GATE: u32 = 0x01160D80;
        const G_LEVEL_OBJ: u32 = 0x0118F4A8;
        const F_ADD_C: u32 = 0x01030C3C;
        const F_TRUNC_A: u32 = 0x0115DBE8;
        const F_TRUNC_B: u32 = 0x00FE8C58;
        const F_POW_K: u32 = 0x00FE876C;
        const F_DBL: u32 = 0x00FE8A78;
        const F_POST_MUL: u32 = 0x00FE8628;
        const F_UCMP2: u32 = 0x0115DED8;
        const F_UCMP2B: u32 = 0x00FE8628;
        const B_U2_A: u32 = 0x011609F6;
        const B_U2_B: u32 = 0x0116252C;
        const B_U2_C: u32 = 0x0116252D;
        const G_OBJ48: u32 = 0x01162548;
        const G_OUT_LEVEL: u32 = 0x01161900;
        const G_MIX_ARG: u32 = 0x011618FC;
        const B_TAIL_FLAG: u32 = 0x0128E3E1;
        const F_TAIL_BASE: u32 = 0x01030C60;
        const F_TAIL_CMP_A: u32 = 0x00FE88BC;
        const F_TAIL_CMP_B: u32 = 0x0103234C;
        const F_TAIL_SUB: u32 = 0x00FE8B00;
        const G_OBJ50: u32 = 0x01162550;
        const G_OBJ54: u32 = 0x01162554;
        const G_OBJ64: u32 = 0x01162564;
        const G_OBJ68: u32 = 0x01162568;
        const G_OBJ70: u32 = 0x01162570;
        const G_OBJ74: u32 = 0x01162574;
        const F_SEL_ALT: u32 = 0x01030C64;
        const G_OBJ58: u32 = 0x01162558;
        const G_OBJ5C: u32 = 0x0116255C;
        const G_OBJ60: u32 = 0x01162560;
        const F_P0A: u32 = 0x01168264;
        const F_P0B: u32 = 0x0116825C;
        const F_P1A: u32 = 0x01168268;
        const F_P1B: u32 = 0x01168260;
        const F_PK: u32 = 0x00FE8830;
        const F_C_6C: u32 = 0x0116826C;
        const F_C_70: u32 = 0x01168270;
        const F_C_74: u32 = 0x01168274;
        const F_C_78: u32 = 0x01168278;
        const F_CK_E4: u32 = 0x00FE87E4;
        const F_CK_30: u32 = 0x00FE8830;
        const B_TAIL: u32 = 0x01283049;
        const B_TAIL_B: u32 = 0x0129576E;
        const B_TAIL_C: u32 = 0x01030C13;
        const G_TAIL_IDX: u32 = 0x01036F14;
        const G_TAIL_TABLE: u32 = 0x011A8808;
        const F_TAIL_F: u32 = 0x01030C5C;
        const OBJ_MAIN: u32 = 0x0115D9A0;
        const OBJ_GEN: u32 = 0x01165880;
        const OBJ_TAP: u32 = 0x0128E310;
        const OBJ_FAC: u32 = 0x01161518;
        const OBJ_TAP2: u32 = 0x01176888;
        const OBJ_PAIR: u32 = 0x01168B20;
        const OBJ_STATE: u32 = 0x01284A60;
        const OBJ_SUB: u32 = 0x018E51E8;
        const OBJ_MIX_B: u32 = 0x01168ABC;
        const OBJ_MIX_C: u32 = 0x01168AA0;
        const OBJ_MIX_D: u32 = 0x01162614;
        const OBJ_MIX_E: u32 = 0x01165864;
        const OBJ_MIX_F: u32 = 0x01168AD8;
        const OBJ_MIXCHK: u32 = 0x0128E94C;
        const OBJ_PAIR2: u32 = 0x011625EC;
        const OBJ_TAIL: u32 = 0x0115DEF0;
        const OBJ_TAILCHK: u32 = 0x0128E400;
        const STR_NAME: u32 = 0x00E7E838;
        const NEG_HUNDRED: u32 = 0xC2C80000;
        const UNITY: u32 = 0x3F800000;
        const NEAR_ONE: u32 = 0x3F666666;
        const C_AUX: u32 = 1;
        const C_SMOOTH: u32 = 2;
        const C_LOOKUP: u32 = 3;
        const C_SET: u32 = 4;
        const C_SET2: u32 = 5;
        const C_GEN: u32 = 6;
        const C_APPLY: u32 = 7;
        const C_TAP: u32 = 8;
        const C_FACTOR: u32 = 9;
        const C_POLL: u32 = 10;
        const C_POLL2: u32 = 11;
        const C_CHECK: u32 = 12;
        const C_GATE: u32 = 13;
        const C_TAP2: u32 = 14;
        const C_CHAIN1: u32 = 15;
        const C_ALT_PATH: u32 = 16;
        const C_PAIR: u32 = 17;
        const C_SETB: u32 = 18;
        const C_SETC: u32 = 19;
        const C_SCAN: u32 = 20;
        const C_TAP3: u32 = 21;
        const C_COUNT: u32 = 22;
        const C_FETCH: u32 = 23;
        const C_STATE: u32 = 24;
        const C_LEVEL: u32 = 25;
        const C_SUB_A: u32 = 26;
        const C_SUB_B: u32 = 27;
        const C_POW: u32 = 28;
        const C_MIX2: u32 = 29;
        const C_MIX_CHECK: u32 = 30;
        const C_TAP3B: u32 = 31;
        const C_TAP4: u32 = 32;
        const C_RESOLVE: u32 = 33;
        const C_PROBE: u32 = 34;
        const C_PAIR2: u32 = 35;
        const C_TAIL: u32 = 36;
        const C_TAIL_CHECK: u32 = 37;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (lf_checker_rt::global::<u8>(a) as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v.to_bits()) }
        }
        /// Read a word at an already-relocated runtime address (heap object
        /// reached through a callee answer or a filled table): no relocation.
        #[inline(always)]
        unsafe fn rd32r(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Read a byte at an already-relocated runtime address.
        #[inline(always)]
        unsafe fn rd8r(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn div_f(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Unsigned int to float by way of double, as the original's
        /// convert, bias-add and narrow sequence computes it.
        #[inline(always)]
        fn u2f(v: u32) -> f32 {
            (v as f64) as f32
        }
        /// Truncate a float to a 64-bit integer like the x87 store with the
        /// truncate control word, returning the low 32 bits. NaN and
        /// out-of-range values store the integer indefinite, whose low word
        /// is zero.
        #[inline(always)]
        fn f32_to_i64_low(v: f32) -> u32 {
            const LIM: f32 = f32::from_bits(0x5F000000); // 2^63
            if v.is_nan() || v >= LIM || v < -LIM {
                0
            } else {
                (v as i64) as u32
            }
        }
        /// The mode triple selecting the live path: mode word not 1, id pair
        /// equal, stage word not idle.
        #[inline(always)]
        unsafe fn live_triple() -> bool {
            unsafe {
                rd32(G_MODE) != 1 && rd32(G_ID_A) == rd32(G_ID_B) && rd32(G_STAGE) != STAGE_IDLE
            }
        }
        /// Limit a sum to the range 0 to tick, passing NaN through as
        /// itself, as the original's compare-and-select sequence does.
        #[inline(always)]
        fn clamp_tick(x2: f32, tick: f32) -> f32 {
            if 0.0 > x2 {
                0.0
            } else if x2 > tick {
                tick
            } else {
                x2
            }
        }
        /// Call the double helper with the constant double in the first
        /// vector register slot and the widened float in the second, as the
        /// original holds them (upper halves zero), returning the raw double
        /// answer bits.
        #[inline(always)]
        unsafe fn pow_call(x1f: f32) -> u64 {
            unsafe {
                let lo0 = rd32(F_DBL);
                let hi0 = rd32(F_DBL + 4);
                let b1 = (x1f as f64).to_bits();
                lf_checker_rt::callee_cdecl!(
                    C_POW, u64, lo0, hi0, (b1 & 0xFFFF_FFFF) as u32, (b1 >> 32) as u32
                )
            }
        }

        // Entry: scaled truncate plus an auxiliary float through an out-pointer.
        let tick = rdf(F_TICK);
        let scale = mul(rdf(F_SCALE_A), rdf(F_SCALE_B));
        let base = u2f(f32_to_i64_low(scale));
        let mut aux_out: u32 = 0;
        lf_checker_rt::callee_stdcall!(C_AUX, u32, (&mut aux_out as *mut u32) as u32, 0);
        let aux = f32::from_bits(aux_out);
        let mix2 = add(mul(rdf(F_MIX), rdf(F_MIX_K)), sub(tick, rdf(F_MIX)));
        let mut x1 = add(mul(sub(tick, aux), mix2), aux);
        let lim = rdf(G_LEVEL);
        if !(x1 >= lim) {
            // Below the level: raise towards it but not past a scaled bound.
            let t = sub(lim, mul(rdf(F_CLAMP_K), base));
            if !(x1 > t) {
                x1 = t;
            }
        }
        wrf(G_LEVEL, x1);
        let ans_a: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, x1.to_bits());
        let esi0 = lf_checker_rt::callee_thiscall!(
            C_LOOKUP, u32, lf_checker_rt::relocated(OBJ_MAIN), rd32(G_LOOKUP_ARG)
        );
        if esi0 != 0 {
            if rd8(B_FLAG0) != 0 {
                lf_checker_rt::callee_thiscall!(C_SET, u32, esi0, NEG_HUNDRED);
            }
            lf_checker_rt::callee_thiscall!(C_SET2, u32, esi0, UNITY);
            let f6: f32 =
                lf_checker_rt::callee_thiscall!(C_GEN, f32, lf_checker_rt::relocated(OBJ_GEN));
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, esi0, f6.to_bits());
        }
        // Tap float, factor byte as float, and the starting mix value.
        let tb: f32 =
            lf_checker_rt::callee_thiscall!(C_TAP, f32, lf_checker_rt::relocated(OBJ_TAP));
        let a9 = lf_checker_rt::callee_thiscall!(C_FACTOR, u32, lf_checker_rt::relocated(OBJ_FAC));
        let f9 = mul((a9 & 0xFF) as f32, rdf(F_FACTOR_K));
        let mut t0 = 0.0f32;
        if rd8(B_MODE09) == 0 && rd8(B_OBJ_ALIVE) != 0 {
            // Unsigned byte compare.
            if rd8(B_CMP_A) <= rd8(B_CMP_B) {
                t0 = f9;
            } else {
                t0 = sub(tick, f9);
            }
        }
        let bl0 = lf_checker_rt::callee_cdecl!(C_POLL, u32,) as u8;
        wr8(B_STATE, bl0);
        // Select between the mix value and the tick through polled bytes.
        let sel = if bl0 != 0 {
            t0
        } else if lf_checker_rt::callee_cdecl!(C_POLL2, u32,) as u8 != 0 {
            t0
        } else if rd8(B_MODE09) != 0 {
            tick
        } else if lf_checker_rt::callee_thiscall!(C_CHECK, u32, lf_checker_rt::relocated(OBJ_FAC))
            as u8
            == 0
        {
            t0
        } else {
            tick
        };
        let m1 = if sel > tb { sel } else { tb };
        let div = rdf(F_DIV);
        let q0 = if 0.0 < m1 {
            if m1 < div {
                div_f(m1, div)
            } else {
                tick
            }
        } else {
            0.0
        };
        let q = div_f(q0, div);
        let ans_c: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, sub(tick, q).to_bits());
        // Two mix buses gated on flag bytes and a status call.
        let mut f1cv: f32;
        let mut f18v: f32;
        if rd8(B_GATE1) == 0
            && rd8(B_GATE2) == 0
            && lf_checker_rt::callee_cdecl!(C_GATE, u32,) as u8 == 0
        {
            f1cv = 0.0;
        } else {
            f1cv = rdf(F_ALT);
        }
        f18v = rdf(F_ALT);
        if rd8(B_ALT_FLAG) == 0 && bl0 == 0 {
            f18v = 0.0;
        }
        let td: f32 =
            lf_checker_rt::callee_thiscall!(C_TAP2, f32, lf_checker_rt::relocated(OBJ_TAP2));
        let x2gate = if rd8(B_X2GATE) != 0 { rdf(F_ALT) } else { 0.0 };
        let mut v18 = ans_c;
        let mut v1c = ans_a;
        let obj3c = rd32(G_OBJ3C);
        if obj3c != 0 {
            let mut s = td;
            s = add(s, v18);
            s = add(s, f1cv);
            s = add(s, f18v);
            s = add(s, v1c);
            if live_triple() {
                s = add(s, x2gate);
            } else {
                s = add(s, rdf(F_SUM_A));
                s = add(s, rdf(F_SUM_B));
            }
            s = add(s, rdf(F_SUM_C));
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj3c, s.to_bits());
        }
        let obj_b0 = rd32(G_OBJB0);
        if obj_b0 != 0 {
            if live_triple() {
                lf_checker_rt::callee_thiscall!(C_SET, u32, obj_b0, 0);
            } else {
                let a16 = lf_checker_rt::callee_thiscall!(
                    C_ALT_PATH, u32, lf_checker_rt::relocated(OBJ_TAP2)
                );
                lf_checker_rt::callee_thiscall!(
                    C_SET, u32, obj_b0, if (a16 as u8) == 0 { 0 } else { NEG_HUNDRED }
                );
            }
        }
        let obj_4c = rd32(G_OBJ4C);
        if obj_4c != 0 {
            let sel1 = rd8(B_SEL1);
            let sel1b = if sel1 != 0 { rd8(B_SEL2) } else { 0 };
            let use_chain = if sel1 != 0 && sel1b != 0 {
                false
            } else if rd8(B_SEL3) == 0 {
                true
            } else if rd8(B_SEL4) == 0 {
                false
            } else {
                true
            };
            let mut g0 = 0.0f32;
            if use_chain {
                let c15 = lf_checker_rt::callee_cdecl!(C_CHAIN1, u32,) as u8;
                if c15 == 0 {
                    let c11 = lf_checker_rt::callee_cdecl!(C_POLL2, u32,) as u8;
                    if c11 == 0 {
                        let c13 = lf_checker_rt::callee_cdecl!(C_GATE, u32,) as u8;
                        if c13 != 0 {
                            g0 = rdf(F_ALT);
                        }
                    }
                }
            }
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj_4c, add(g0, v18).to_bits());
        }
        // Pair call with poll-dependent arguments, then scaled int factors.
        let a11 = lf_checker_rt::callee_cdecl!(C_POLL2, u32,) as u8;
        let (pair0, pair1, movd_src) = if a11 != 0 {
            let sc = mul(rdf(F_PAIR_A), rdf(F_PAIR_K));
            (sc, sc, rd32(G_MOVD_A))
        } else {
            (rdf(F_PAIR_B), rdf(F_PAIR_A), rd32(G_MOVD_B))
        };
        lf_checker_rt::callee_thiscall!(
            C_PAIR, u32, lf_checker_rt::relocated(OBJ_PAIR), pair0.to_bits(), pair1.to_bits()
        );
        let mut k0 = mul((movd_src as i32) as f32, rdf(F_INT_K));
        if !(live_triple() && rd8(B_X2GATE) == 0) {
            lf_checker_rt::callee_thiscall!(
                C_SETB, u32, lf_checker_rt::relocated(OBJ_MAIN), NEAR_ONE
            );
            lf_checker_rt::callee_thiscall!(
                C_SETC, u32, lf_checker_rt::relocated(OBJ_MAIN), NEAR_ONE
            );
        } else {
            lf_checker_rt::callee_thiscall!(
                C_SETB, u32, lf_checker_rt::relocated(OBJ_MAIN), k0.to_bits()
            );
            k0 = mul((rd32(G_MOVD_C) as i32) as f32, rdf(F_INT_K));
            lf_checker_rt::callee_thiscall!(
                C_SETC, u32, lf_checker_rt::relocated(OBJ_MAIN), k0.to_bits()
            );
        }
        // Big middle block, present only when both mixer objects exist.
        // The stale mix slot below keeps zero when this block is skipped:
        // the original then reads a frame word nothing wrote (the checker
        // fills it with zero).
        let mut stale_mix = 0.0f32;
        let obj40 = rd32(G_OBJ40);
        let obj44 = rd32(G_OBJ44);
        if obj40 != 0 && obj44 != 0 {
            let r20 = lf_checker_rt::callee_cdecl!(C_SCAN, u32, 0);
            if r20 != 0 {
                let te: f32 =
                    lf_checker_rt::callee_thiscall!(C_TAP3, f32, r20.wrapping_add(0x210));
                v18 = te;
                v1c = te;
            }
            // Signed index compare against 3.
            let idx = rd32(G_INDEX);
            if (idx as i32) >= 3 {
                f1cv = rdf(F_IDX_B);
            } else {
                f1cv = rdf(F_IDX_A);
            }
            // Entry fetch for an index below the count (unsigned compare).
            // The incoming register value the original passes here is pinned
            // to zero by the proof (see the proof record).
            let edi0 = rd32(G_EDI_SRC);
            let r22 = lf_checker_rt::callee_cdecl!(C_COUNT, u32, 0);
            let mut esi1: u32 = 0;
            if edi0 < r22 {
                esi1 = lf_checker_rt::callee_cdecl!(C_FETCH, u32, edi0);
            }
            if edi0 == rd8(B_CMP_C) as u32 || edi0 == rd8(B_CMP_D) as u32 {
                v18 = rdf(F_F20);
            }
            f18v = rdf(F_F18);
            if esi1 != 0 {
                if rd32r(esi1.wrapping_add(0x1904)) == 6 {
                    f1cv = rdf(F_ESI6);
                } else {
                    let off = (rd8r(esi1.wrapping_add(0x1917)) as u32)
                        .wrapping_mul(0xBD0)
                        .wrapping_add(esi1);
                    if off != 0
                        && rd8r(off.wrapping_add(0xBCA)) != 0
                        && rd8r(off.wrapping_add(0xBC0)) != 2
                    {
                        t0 = 0.0;
                    }
                }
            }
            // Unordered-aware gate: ordered-equal (or the flag byte) zeroes
            // three buses, otherwise they are recomputed. The scratch bus
            // starts from a frame word nothing wrote (zero under the checker).
            v1c = 0.0;
            let u1 = rdf(F_UCMP);
            let mut slot_c = 0.0f32;
            if u1 == 0.0 || rd8(B_UCMP_GATE) != 0 {
                t0 = 0.0;
                f1cv = 0.0;
                slot_c = 0.0;
            } else {
                let s1: f32 =
                    lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, rdf(F_S1ARG).to_bits());
                let s38a = add(s1, v18);
                let s2: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, u1.to_bits());
                let x3 = sub(s38a, s2);
                let x2 = sub(add(t0, f1cv), x3);
                t0 = if 0.0 > x2 { x2 } else { 0.0 };
                f1cv = sub(rdf(F_POST54), x3);
                let x1b = sub(slot_c, x3);
                slot_c = if 0.0 > x1b { x1b } else { 0.0 };
            }
            // State check with table-byte fallback feeding one slot.
            let a24 = lf_checker_rt::callee_thiscall!(
                C_STATE, u32, lf_checker_rt::relocated(OBJ_STATE)
            );
            let al24 = a24 as u8;
            let mut slot_a = 0.0f32;
            let take_side = if al24 != 0 {
                true
            } else if rd8(B_STATE_B) != al24 {
                true
            } else if rd32(G_TBL_SEL) != TBL_SEL_ON {
                false
            } else {
                let ti = rd32(G_TBL_IDX).wrapping_mul(0x1A0);
                rd8(G_TBL_BASE.wrapping_add(ti)) != 0
            };
            if take_side {
                slot_a = t0;
                v1c = slot_c;
            } else if rd8(B_SLOT_A_GATE) != 0 {
                slot_a = f1cv;
            }
            // Optional two-level boost.
            let mut x1mix: f32;
            if rd32(G_BIG_GATE) != 0 {
                x1mix = slot_a;
            } else {
                let r25a = lf_checker_rt::callee_thiscall!(
                    C_LEVEL, u32, rd32(G_LEVEL_OBJ), 0x66
                );
                // Signed compare: zero and negative skip the boost.
                if (r25a as i32) <= 0 {
                    x1mix = slot_a;
                } else {
                    let r25b = lf_checker_rt::callee_thiscall!(
                        C_LEVEL, u32, rd32(G_LEVEL_OBJ), 0x64
                    );
                    if r25b == 0 {
                        x1mix = slot_a;
                    } else {
                        let a26 = lf_checker_rt::callee_thiscall!(
                            C_SUB_A, u32, lf_checker_rt::relocated(OBJ_SUB)
                        );
                        let go_b = if (a26 as u8) != 0 {
                            true
                        } else {
                            (lf_checker_rt::callee_thiscall!(
                                C_SUB_B, u32, lf_checker_rt::relocated(OBJ_SUB)
                            ) as u8)
                                != 0
                        };
                        if go_b {
                            x1mix = add(rdf(F_ADD_C), slot_a);
                        } else {
                            x1mix = slot_a;
                        }
                    }
                }
            }
            // Two truncated-scale power stages with mix calls.
            x1mix = add(x1mix, v18);
            let trunc2 = f32_to_i64_low(mul(rdf(F_TRUNC_A), rdf(F_TRUNC_B)));
            let x0b = sub(x1mix, rdf(F_ALT));
            let x2b = if x0b < 0.0 {
                0.0
            } else {
                x1mix = mul(x1mix, rdf(F_POW_K));
                f64::from_bits(pow_call(x1mix)) as f32
            };
            let s29a: f32 = lf_checker_rt::callee_thiscall!(
                C_MIX2, f32, lf_checker_rt::relocated(OBJ_PAIR), x2b.to_bits(), trunc2
            );
            v18 = s29a;
            let mut x1c = v1c;
            let trunc3 = f32_to_i64_low(mul(rdf(F_TRUNC_A), rdf(F_TRUNC_B)));
            let x0c = sub(x1c, rdf(F_ALT));
            let x2c = if x0c < 0.0 {
                0.0
            } else {
                x1c = mul(x1c, rdf(F_POW_K));
                f64::from_bits(pow_call(x1c)) as f32
            };
            let s29b: f32 = lf_checker_rt::callee_thiscall!(
                C_MIX2, f32, lf_checker_rt::relocated(OBJ_MIX_B), x2c.to_bits(), trunc3
            );
            stale_mix = s29b;
            let a30 = lf_checker_rt::callee_thiscall!(
                C_MIX_CHECK, u32, lf_checker_rt::relocated(OBJ_MIXCHK)
            );
            if (a30 as u8) != 0 {
                v1c = mul(v18, rdf(F_POST_MUL));
            } else {
                v1c = v18;
            }
            let s31: f32 =
                lf_checker_rt::callee_thiscall!(C_TAP3B, f32, lf_checker_rt::relocated(OBJ_TAP2));
            v18 = s31;
            let s2p: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, v1c.to_bits());
            v1c = add(s2p, s31);
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj40, v1c.to_bits());
            let s2q: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, s29b.to_bits());
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj44, add(s2q, s31).to_bits());
            wrf(G_OUT_LEVEL, v1c);
        }
        // Second unordered-aware gate: a summed push or a fixed negative one.
        let s32: f32 =
            lf_checker_rt::callee_thiscall!(C_TAP4, f32, lf_checker_rt::relocated(OBJ_TAP2));
        let take_sum = rdf(F_UCMP2) != rdf(F_UCMP2B)
            && rd8(B_U2_A) == 0
            && rd8(B_U2_B) == 0
            && {
                let a = lf_checker_rt::callee_thiscall!(
                    C_MIX_CHECK, u32, lf_checker_rt::relocated(OBJ_MIXCHK)
                ) as u8;
                a == 0 && rd8(B_U2_C) == 0
            };
        let obj48 = rd32(G_OBJ48);
        if take_sum {
            if obj48 != 0 {
                let s = add(add(td, f18v), s32);
                lf_checker_rt::callee_thiscall!(C_SET, u32, obj48, s.to_bits());
            }
        } else if obj48 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj48, NEG_HUNDRED);
        }
        // Named-object state/pow/mix/smooth sequence.
        let r33 = lf_checker_rt::callee_cdecl!(
            C_RESOLVE, u32, lf_checker_rt::relocated(STR_NAME), 0
        );
        let esi3 =
            lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, lf_checker_rt::relocated(OBJ_MAIN), r33);
        if esi3 != 0 {
            let a24b = lf_checker_rt::callee_thiscall!(
                C_STATE, u32, lf_checker_rt::relocated(OBJ_STATE)
            );
            let x1d = if (a24b as u8) != 0 { rdf(F_IDX_B) } else { 0.0 };
            let x0d = sub(x1d, rdf(F_ALT));
            let x2d = if x0d < 0.0 {
                0.0
            } else {
                f64::from_bits(pow_call(mul(x1d, rdf(F_POW_K)))) as f32
            };
            let s29c: f32 = lf_checker_rt::callee_thiscall!(
                C_MIX2, f32, lf_checker_rt::relocated(OBJ_MIX_C), x2d.to_bits(), rd32(G_MIX_ARG)
            );
            let s2b: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, s29c.to_bits());
            lf_checker_rt::callee_thiscall!(C_SET, u32, esi3, s2b.to_bits());
        }
        // Trailing mix bus, final power stage and two mix/smooth pairs.
        let mut v1c2 = 0.0f32;
        if rd8(B_TAIL_FLAG) & 0x40 != 0 {
            v1c2 = rdf(F_TAIL_BASE);
        }
        if rdf(F_TAIL_CMP_A) > rdf(F_TAIL_CMP_B) {
            v1c2 = sub(v1c2, rdf(F_TAIL_SUB));
        }
        lf_checker_rt::callee_thiscall!(C_STATE, u32, lf_checker_rt::relocated(OBJ_STATE));
        let x0e = f64::from_bits(pow_call(0.0)) as f32;
        let s29d: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX2, f32, lf_checker_rt::relocated(OBJ_MIX_D), x0e.to_bits(), rd32(G_MIX_ARG)
        );
        let s2c: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, s29d.to_bits());
        // The original stores the next sum into its frame and never reads it
        // (the following mix call takes the stale slot instead), so only the
        // stale slot matters here.
        let _dead_sum = add(s2c, v1c2);
        let s29e: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX2, f32, lf_checker_rt::relocated(OBJ_MIX_E), stale_mix.to_bits(), rd32(G_MIX_ARG)
        );
        let s2d: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, s29e.to_bits());
        let obj50 = rd32(G_OBJ50);
        if obj50 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj50, s2d.to_bits());
        }
        let obj54 = rd32(G_OBJ54);
        if obj54 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj54, stale_mix.to_bits());
        }
        // Probe call with two frame out-pointers: a float and a flag byte.
        // The stub writes both; the float feeds one smooth call and the
        // byte's low bit steers the last mix input.
        let mut out_a: u32 = 0;
        let mut out_b: u32 = 0;
        let bl1 = lf_checker_rt::callee_thiscall!(
            C_PROBE, u32, lf_checker_rt::relocated(OBJ_GEN),
            (&mut out_b as *mut u32) as u32, (&mut out_a as *mut u32) as u32
        ) as u8;
        let out_a_f = f32::from_bits(out_a);
        let _s2e: f32 = lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, sub(tick, out_a_f).to_bits());
        let obj64 = rd32(G_OBJ64);
        if obj64 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj64, f1cv.to_bits());
        }
        let obj68 = rd32(G_OBJ68);
        if obj68 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj68, f1cv.to_bits());
        }
        let obj70 = rd32(G_OBJ70);
        if obj70 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj70, f1cv.to_bits());
        }
        let obj74 = rd32(G_OBJ74);
        if obj74 != 0 {
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj74, f1cv.to_bits());
        }
        let sel_f = if bl1 != 0 && (out_b & 0xFF) as u8 != 0 {
            rdf(F_SEL_ALT)
        } else {
            tick
        };
        let s29f: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX2, f32, lf_checker_rt::relocated(OBJ_MIX_F), sel_f.to_bits(), rd32(G_MIX_ARG)
        );
        let obj50b = rd32(G_OBJ50);
        if obj50b != 0 {
            lf_checker_rt::callee_thiscall!(C_SET2, u32, obj50b, s29f.to_bits());
        }
        // Final pair smooths and three tick-relative clamps.
        let obj58 = rd32(G_OBJ58);
        let obj5c = rd32(G_OBJ5C);
        let obj60 = rd32(G_OBJ60);
        if obj58 != 0 && obj5c != 0 && obj60 != 0 {
            let p0 = mul(add(rdf(F_P0A), rdf(F_P0B)), rdf(F_PK));
            let p1 = add(rdf(F_P1A), rdf(F_P1B));
            let s35a: f32 = lf_checker_rt::callee_thiscall!(
                C_PAIR2, f32, lf_checker_rt::relocated(OBJ_PAIR2), p0.to_bits()
            );
            let s35b: f32 = lf_checker_rt::callee_thiscall!(
                C_PAIR2, f32, lf_checker_rt::relocated(OBJ_PAIR2), mul(p1, rdf(F_PK)).to_bits()
            );
            let mut k1 = sub(tick, rdf(F_C_6C));
            k1 = add(k1, sub(tick, rdf(F_C_70)));
            k1 = add(k1, sub(tick, rdf(F_C_74)));
            k1 = add(k1, sub(tick, rdf(F_C_78)));
            k1 = mul(k1, rdf(F_CK_E4));
            let s2m: f32 =
                lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, clamp_tick(k1, tick).to_bits());
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj58, s2m.to_bits());
            let mut k2 = sub(tick, rdf(F_C_6C));
            k2 = add(k2, sub(tick, rdf(F_C_74)));
            k2 = mul(k2, rdf(F_CK_30));
            let s2m2: f32 =
                lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, clamp_tick(k2, tick).to_bits());
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj5c, add(s2m2, s35a).to_bits());
            let mut k3 = sub(tick, rdf(F_C_70));
            k3 = add(k3, sub(tick, rdf(F_C_78)));
            k3 = mul(k3, rdf(F_CK_30));
            let s2m3: f32 =
                lf_checker_rt::callee_cdecl!(C_SMOOTH, f32, clamp_tick(k3, tick).to_bits());
            lf_checker_rt::callee_thiscall!(C_SET, u32, obj60, add(s2m3, s35b).to_bits());
        }
        // Tail: fixed unity push, or a polled choice between zero and a
        // table float. A negative table index would fault the original and
        // is excluded by the proof; the branch is kept for shape.
        let tail_obj = lf_checker_rt::relocated(OBJ_TAIL);
        if rd8(B_TAIL) != 0 {
            return lf_checker_rt::callee_thiscall!(C_TAIL, u32, tail_obj, UNITY);
        }
        let a37 = lf_checker_rt::callee_thiscall!(
            C_TAIL_CHECK, u32, lf_checker_rt::relocated(OBJ_TAILCHK)
        );
        if (a37 as u8) != 0 {
            return lf_checker_rt::callee_thiscall!(C_TAIL, u32, tail_obj, rdf(F_TAIL_F).to_bits());
        }
        if rd8(B_TAIL_B) == 0 || rd8(B_TAIL_C) == 0 {
            return lf_checker_rt::callee_thiscall!(C_TAIL, u32, tail_obj, 0);
        }
        let tidx = rd32(G_TAIL_IDX);
        let entry = if tidx == 0xFFFF_FFFF {
            0
        } else {
            rd32(G_TAIL_TABLE.wrapping_add(tidx.wrapping_mul(4)))
        };
        if rd32r(entry.wrapping_add(0x4C8)) != 0 {
            lf_checker_rt::callee_thiscall!(C_TAIL, u32, tail_obj, rdf(F_TAIL_F).to_bits())
        } else {
            lf_checker_rt::callee_thiscall!(C_TAIL, u32, tail_obj, 0)
        }
    }
});
