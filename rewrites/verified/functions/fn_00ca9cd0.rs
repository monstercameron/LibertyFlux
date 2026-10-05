// original: 0x00ca9cd0 CEventHandler::vf46
/// Vet and dispatch one event by code, time and subject kind: build a
/// response through one of many constructors, or forward unknown codes to
/// the handler's parent slot.
///
/// `this` is the handler: word at `+0x04` points to the owner, whose word at
/// `+0xb30` is the context and whose word at `+0x224` is the inner object.
/// `ev` is the event: its time at `+0x24` (float), subject at `+0x1c`,
/// context echo at `+0x18` and code at `+0x10`. The second and third stack
/// arguments are not read. (thiscall: `this` in ecx, three stack words.)
///
/// The event is fresh only when its time is positive or NaN; the context must
/// be nonzero and match the echo. Codes: `0xc8` returns zero; `0x2d9` joins
/// the shared subject check below; `0x2c2` builds through id 12; `0x2d6`
/// builds through id 14 (kind 3) or id 13 (other kinds) and then joins the
/// shared check; `0x407`/`0x76c` run the deep check; anything else calls the
/// parent slot at `[this]+0x148` (id 16, fed code and subject).
///
/// Shared check: a null subject exits; kind bits `(subject[0x28]>>6)&0xf`
/// must be 2 or 3. The event's own slot at `[ev]+0x34` (id 8) must agree
/// with the lookup (id 7), the tell check (id 9) must pass, the owner's
/// flag byte at `+0xa60` must be `1` and `ev[0x20]` must be `0x31`, else
/// the fallback builds through id 11; on success id 10 builds. A failed
/// allocation stores zero and returns zero.
///
/// Deep check with kind 3: when the state word, the subject flag, the id-17
/// check and a positive id-19 reading all pass, two calls (ids 18, 19) run
/// first; then id 14 builds, flag `0x4000` is set at `response+0x60` and
/// id 15 forwards (its answer is returned). With kind 2 the id-20/21/22
/// chain runs and id 23 (ten arguments) builds and returns; otherwise the
/// id-26 slot must stay quiet, the flag byte must not be `2`, the id-24/25
/// pair resolves, id 27 runs, and either id 28 builds from the event time
/// or — with the second flag bit set and a big enough counter, a quiet
/// id-30 check and a resolved fetch — the final chain builds through ids
/// 14, 15 and 33/34 and stores the result at `this+8`.
///
/// Early exits return whatever eax still holds (the caller's incoming eax);
/// the contract pins it to `INCOMING_EAX` and the rewrite returns that.
/// Floats are only compared or carried as bits, never computed, so no
/// operation order needs pinning.
lf_checker_rt::export!(thiscall, rw_00ca9cd0(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const CONTEXT: u32 = 0xb30;
        const RESPONSE: u32 = 0x0c;
        const EVT_TIME: u32 = 0x24;
        const EVT_SUBJECT: u32 = 0x1c;
        const EVT_ECHO: u32 = 0x18;
        const EVT_CODE: u32 = 0x10;
        const EVT_WORD: u32 = 0x20;
        const EVT_SLOT_OFF: u32 = 0x34;
        const PARENT_SLOT: u32 = 0x148;
        const KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const SUBJ_STATE: u32 = 0x21c;
        const SUBJ_STATE_WANT: u32 = 2;
        const SUBJ_FLAG: u32 = 0x219;
        const SUBJ_EXT: u32 = 0x228;
        const EXT_ADVANCE: u32 = 0x70;
        const READING: u32 = 0x1c;
        const FLAG_BYTE: u32 = 0xa60;
        const FLAG2_BYTE: u32 = 0x26c;
        const FLAG2_BIT: u8 = 4;
        const FLAG3_BYTE: u32 = 0x272;
        const FLAG3_BIT: u8 = 1;
        const VT3_SLOT: u32 = 0x128;
        const BIG_OFF: u32 = 0x1304;
        const BIG_WANT: u32 = 4;
        const CHECK_WORD: u32 = 0xf14;
        const SLOT_ADVANCE: u32 = 0x2e0;
        const FETCH_OFF: u32 = 0xf50;
        const COUNTER: u32 = 0xe04;
        const RESP_FLAG: u32 = 0x60;
        const RESP_BIT: u32 = 8;
        const RESP_BIT_BIG: u32 = 0x4000;
        const RESP_BYTE: u32 = 0x39;
        const STORE8: u32 = 0x08;
        const WANT_76C: u32 = 0x76c;
        const INCOMING_EAX: u32 = 0x1357_9BDF;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const POS_LIMIT: u32 = 0x00fe_8628;
        const F_60: u32 = 0x00ee_f940;
        const F_1M: u32 = 0x00ee_f944;
        const F_1K: u32 = 0x00ee_f948;
        const F_1: u32 = 0x00ee_f94c;
        const CALL9B_THIS: u32 = 0x0128_aa90;
        const TABLE_E: u32 = 0x00ed_79e8;
        const INNER_ADVANCE: u32 = 0x570;
        const A_2D6: u32 = 1;
        const A_JOIN: u32 = 2;
        const A_BIG3: u32 = 3;
        const A_DEEP: u32 = 4;
        const A_VD1: u32 = 5;
        const A_VD2: u32 = 6;
        const LOOKUP: u32 = 7;
        const EVT_SLOT: u32 = 8;
        const TELL0: u32 = 9;
        const BUILD_B: u32 = 10;
        const BUILD_BB: u32 = 11;
        const BUILD_2C2: u32 = 12;
        const BUILD_ALT: u32 = 13;
        const BUILD: u32 = 14;
        const FORWARD: u32 = 15;
        const PARENT: u32 = 16;
        const CHECK_A: u32 = 17;
        const PRE_A: u32 = 18;
        const CALL_9B: u32 = 19;
        const CHECK_B: u32 = 20;
        const CHECK_C: u32 = 21;
        const CHECK_D: u32 = 22;
        const CALL_E: u32 = 23;
        const CHECK_FA: u32 = 24;
        const CHECK_FB: u32 = 25;
        const VT3CALL: u32 = 26;
        const CALL_G: u32 = 27;
        const CALL_H: u32 = 28;
        const RND: u32 = 29;
        const CHECK_I: u32 = 30;
        const FETCH: u32 = 31;
        const CHECK_J: u32 = 32;
        const BUILD_K: u32 = 33;
        const BUILD_L: u32 = 34;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let time = rdf(ev + EVT_TIME);
        if time <= 0.0 {
            return INCOMING_EAX;
        }
        let subj = rd32(ev + EVT_SUBJECT);
        let owner = rd32(this + OWNER);
        let b30 = rd32(owner + CONTEXT);
        if b30 == 0 {
            return INCOMING_EAX;
        }
        if b30 != rd32(ev + EVT_ECHO) {
            return INCOMING_EAX;
        }
        let code = rd32(ev + EVT_CODE);
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let inner = rd32(owner + INNER);

        // Shared pieces, as macros because they return from the function.
        macro_rules! shared_check {
            () => {{
                if subj == 0 {
                    return INCOMING_EAX;
                }
                let kind2 = (rd32(subj + KIND) >> 6) & 0xf;
                if kind2 != 3 && kind2 != 2 {
                    return kind2;
                }
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(ev) + EVT_SLOT_OFF) as usize);
                let m = slot(ev);
                let t: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
                if m != t {
                    return build_bb!();
                }
                let q: u32 = lf_checker_rt::callee_cdecl!(TELL0, u32,);
                if q & 0xff == 0 {
                    return build_bb!();
                }
                if rd8(owner + FLAG_BYTE) != 1 {
                    return build_bb!();
                }
                if rd32(ev + EVT_WORD) != 0x31 {
                    return build_bb!();
                }
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_JOIN, u32, global);
                if obj == 0 {
                    wr32(this + RESPONSE, 0);
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_B, u32, obj, rd32(owner + CONTEXT), 0
                );
                wr32(this + RESPONSE, r);
                return r;
            }};
        }
        macro_rules! build_bb {
            () => {{
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_JOIN, u32, global);
                if obj == 0 {
                    wr32(this + RESPONSE, 0);
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_BB, u32, obj, rd32(owner + CONTEXT), subj
                );
                wr32(this + RESPONSE, r);
                return r;
            }};
        }
        // Deep tail: id 27, then id 28 or the very-deep chain.
        macro_rules! deep_tail {
            () => {{
                let _: u32 = lf_checker_rt::callee_thiscall!(CALL_G, u32, owner);
                if rd8(owner + FLAG3_BYTE) & FLAG3_BIT == 0 {
                    let obj: u32 = lf_checker_rt::callee_thiscall!(A_DEEP, u32, global);
                    if obj == 0 {
                        wr32(this + RESPONSE, 0);
                        return 0;
                    }
                    let fbits = rd32(ev + EVT_TIME);
                    let p = ev.wrapping_add(0x30);
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        CALL_H, u32, obj, rd32(owner + CONTEXT), subj, fbits, p
                    );
                    wr32(this + RESPONSE, r);
                    return r;
                }
                let n = rd32(owner + COUNTER) as i32;
                let m4: u32 = lf_checker_rt::callee_cdecl!(RND, u32, 5u32, 0xau32);
                if n <= m4 as i32 {
                    return m4;
                }
                let ci: u32 = lf_checker_rt::callee_thiscall!(
                    CHECK_I, u32, inner.wrapping_add(SLOT_ADVANCE), WANT_76C, 0
                );
                if ci & 0xff != 0 {
                    return ci;
                }
                let mut s2 = rd32(subj + FETCH_OFF);
                if s2 == 0 {
                    s2 = lf_checker_rt::callee_thiscall!(FETCH, u32, subj);
                    if s2 == 0 {
                        return 0;
                    }
                }
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_VD1, u32, global);
                let r: u32 = if obj == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD, u32, obj, s2, 0)
                };
                wr32(this + RESPONSE, r);
                let f: u32 = lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, s2, 1);
                if rd32(this + STORE8) != 0 {
                    return f;
                }
                let j: u32 =
                    lf_checker_rt::callee_cdecl!(CHECK_J, u32, owner, ev);
                if j & 0xff == 0 {
                    return j;
                }
                let obj2: u32 = lf_checker_rt::callee_thiscall!(A_VD2, u32, global);
                if obj2 == 0 {
                    wr32(this + STORE8, 0);
                    return 0;
                }
                let t3: u32 =
                    lf_checker_rt::callee_stdcall!(BUILD_K, u32, owner, s2, 0u32);
                let r2: u32 =
                    lf_checker_rt::callee_thiscall!(BUILD_L, u32, obj2, 0u32, 1u32, t3);
                wr32(this + STORE8, r2);
                return r2;
            }};
        }

        if code == 0x2d9 {
            shared_check!();
        }
        if code == 0xc8 {
            return 0;
        }
        if code == 0x2c2 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(A_JOIN, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let d2 = rd32(owner + CONTEXT);
            let p = rd32(d2 + 0x20).wrapping_add(0x30);
            let r: u32 =
                lf_checker_rt::callee_thiscall!(BUILD_2C2, u32, obj, d2, p, 0, 0, 0);
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x2d6 {
            if rd32(subj + KIND) & KIND_MASK == KIND_WANT {
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_2D6, u32, global);
                let r: u32 = if obj == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD, u32, obj, subj, 0)
                };
                wr32(this + RESPONSE, r);
                wr32(r + RESP_FLAG, rd32(r + RESP_FLAG) | RESP_BIT);
                lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subj, 1);
            } else {
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_2D6, u32, global);
                if obj == 0 {
                    wr32(this + RESPONSE, 0);
                    ((RESP_BYTE) as *mut u8).write(1);
                    return 0;
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_ALT, u32, obj, subj, 1,
                    rd32(lf_checker_rt::relocated(F_60)),
                    rd32(lf_checker_rt::relocated(F_1M)),
                    rd32(lf_checker_rt::relocated(F_1K)),
                    rd32(lf_checker_rt::relocated(F_1)),
                    0
                );
                wr32(this + RESPONSE, r);
                ((r + RESP_BYTE) as *mut u8).write(1);
            }
            shared_check!();
        }
        if code == 0x407 || code == 0x76c {
            if subj == 0 {
                return INCOMING_EAX;
            }
            let kind2 = (rd32(subj + KIND) >> 6) & 0xf;
            if kind2 == 3 {
                let w = rd32(owner + SUBJ_STATE);
                if rd32(w + 0x12c) == SUBJ_STATE_WANT
                    && rd8(subj + SUBJ_FLAG) != 0
                {
                    let p = rd32(subj + SUBJ_EXT);
                    let q = if p == 0 { 0 } else { p.wrapping_add(EXT_ADVANCE) };
                    let a: u32 = lf_checker_rt::callee_thiscall!(CHECK_A, u32, q);
                    if a == 0 {
                        let p2 = rd32(subj + SUBJ_EXT);
                        let rr = if p2 == 0 { 0 } else { p2.wrapping_add(EXT_ADVANCE) };
                        let g = rdf(rr + READING);
                        let c = rdf(lf_checker_rt::relocated(POS_LIMIT));
                        if g > c {
                            lf_checker_rt::callee_thiscall!(PRE_A, u32, subj, 1, 0x3e8);
                            let u = rd32(subj + 0x20).wrapping_add(0x30);
                            lf_checker_rt::callee_thiscall!(
                                CALL_9B, u32,
                                lf_checker_rt::relocated(CALL9B_THIS), u, 0xc, 0x3e8
                            );
                        }
                    }
                }
                let obj: u32 = lf_checker_rt::callee_thiscall!(A_BIG3, u32, global);
                let r: u32 = if obj == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD, u32, obj, subj, 0)
                };
                wr32(this + RESPONSE, r);
                wr32(r + RESP_FLAG, rd32(r + RESP_FLAG) | RESP_BIT_BIG);
                let f: u32 = lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, subj, 1);
                return f;
            }
            if kind2 != 2 {
                return kind2;
            }
            if rd8(owner + FLAG2_BYTE) & FLAG2_BIT == 0 {
                return 2;
            }
            let w = rd32(owner + SUBJ_STATE);
            if rd32(w + 0x12c) == SUBJ_STATE_WANT {
                let b: u32 = lf_checker_rt::callee_thiscall!(
                    CHECK_B, u32, b30.wrapping_add(CHECK_WORD)
                );
                if b & 0xff == 0 {
                    return b;
                }
                let s: u32 = lf_checker_rt::callee_thiscall!(
                    CHECK_C, u32, inner.wrapping_add(SLOT_ADVANCE)
                );
                if s != 0 && rd32(s + KIND) & KIND_MASK == KIND_WANT {
                    let d: u32 = lf_checker_rt::callee_thiscall!(CHECK_D, u32, subj, s);
                    if d & 0xff != 0 {
                        return d;
                    }
                }
                let e: u32 = lf_checker_rt::callee_thiscall!(
                    CALL_E, u32, owner.wrapping_add(INNER_ADVANCE),
                    lf_checker_rt::relocated(TABLE_E), 0, 0, 0, 0xFFFF_FFFF, 0, 0,
                    0x3f80_0000, 0, 0
                );
                return e;
            }
            // Deep path.
            let v3: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(owner) + VT3_SLOT) as usize);
            let m3: u32 = v3(owner);
            if m3 & 0xff != 0 {
                return m3;
            }
            if rd8(owner + FLAG_BYTE) == 2 {
                return m3;
            }
            let fa: u32 = lf_checker_rt::callee_thiscall!(CHECK_FA, u32, b30);
            if fa & 0xff != 0 {
                if rd32(b30 + BIG_OFF) != BIG_WANT {
                    return deep_tail!();
                }
            }
            let fb: u32 = lf_checker_rt::callee_thiscall!(CHECK_FB, u32, subj);
            if fb & 0xff == 0 {
                return fb;
            }
            if rd32(subj + BIG_OFF) == BIG_WANT {
                return fb;
            }
            return deep_tail!();
        }
        // Default: parent slot.
        let parent: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
        return parent(this, code, subj);

    }
});
