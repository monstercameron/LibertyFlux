// original: 0x00caa220 CEventHandler::vf45
/// Dispatch one event by its code through a three-way switch: a float-gated
/// build, a context-gated build, or an immediate exit.
///
/// `this` is the handler: word at `+0x04` points to the owner, whose word at
/// `+0xb30` is the context. `ev` is the event: word at `+0x18` points to
/// the subject and word at `+0x10` is the code. The second and third stack
/// arguments are not read. Returns the stored response, a build answer, the
/// event, the shifted code, or the table index per path. (thiscall: `this`
/// in ecx, three stack words.)
///
/// A null subject returns the event. Otherwise the code selects: `0x2d6`
/// runs the context path; `0x2c2`, `0x2c3`, `0x2e2`, `0x38d` and `0x38f`
/// run the float path; any other code in `0x2c2..=0x38f` returns `2`; codes
/// outside that range return the code minus `0x2c2`.
///
/// Float path: call the subject's measure slot at `[subject]+0xfc` (id 9,
/// returns a float). A zero measure (id 10's answer below `0x3fff`, signed,
/// then selects `-1.0` versus `1.0` for the third factor) builds through
/// id 12 (subject, `0`, a three-factor frame block, `0`, `2`); the block
/// address is skipped in the comparison and never read back. A nonzero
/// measure (NaN included) builds a helper through ids 2-4 (`0` on any
/// allocation failure): the helper is the id-3 allocation itself, initialised
/// in place (the id-14 answer is discarded), and the id-13 answer takes the
/// subject's register after the helper is pushed (the reload reads one slot
/// too low), so both id-15 links and the stored response use it while the
/// id-16 arguments reload the subject fresh. Both reach a tail gate: id 11's
/// answer at or
/// above `0x3fff` (signed) exits returning it, else the ten-argument id 20
/// (descriptor table, five zeros, `-1`, two zeros, `1.0`, two zeros) runs
/// on `owner+0x570` and its answer is returned.
///
/// Context path: with a nonzero context that passes the id 17 check,
/// allocate and build through id 18 (context, `0`), storing and returning
/// the answer (`0` when allocation fails). Otherwise build a fallback
/// through ids 6-8 (helper, four-argument id 19, id 16 again), link the
/// pieces via id 15, store the helper and return the last link answer.
lf_checker_rt::export!(thiscall, rw_00caa220(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const CONTEXT: u32 = 0xb30;
        const RESPONSE: u32 = 0x0c;
        const SUBJECT: u32 = 0x18;
        const CODE: u32 = 0x10;
        const CODE_BASE: u32 = 0x2c2;
        const CODE_SPAN: u32 = 0xcd;
        const CONTEXT_CODE: u32 = 0x2d6;
        const MEASURE_SLOT: u32 = 0xfc;
        const GATE_LIMIT: u32 = 0x3fff;
        const LOOKUP_OFF: u32 = 0x20;
        const LOOKUP_ADVANCE: u32 = 0x30;
        const NEG_ONE: u32 = 0xbf80_0000;
        const ONE: u32 = 0x3f80_0000;
        const SIX: u32 = 0x40c0_0000;
        const HELPER_VTABLE: u32 = 0x00e8_9044;
        const HELPER_WORD: u16 = 0x0100;
        const DESCRIPTOR_TABLE: u32 = 0x00ed_7920;
        const INNER_ADVANCE: u32 = 0x570;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const ALLOC_ZERO: u32 = 1;
        const ALLOC_NZ1: u32 = 2;
        const ALLOC_NZ2: u32 = 3;
        const ALLOC_NZ3: u32 = 4;
        const ALLOC_CTX: u32 = 5;
        const ALLOC_FB1: u32 = 6;
        const ALLOC_FB2: u32 = 7;
        const ALLOC_FB3: u32 = 8;
        const MEASURE: u32 = 9;
        const RNG_ZERO: u32 = 10;
        const RNG_TAIL: u32 = 11;
        const BUILD_ZERO: u32 = 12;
        const BUILD_HELP: u32 = 13;
        const MAKE_HELPER: u32 = 14;
        const LINK: u32 = 15;
        const BUILD_RESP: u32 = 16;
        const CTX_CHECK: u32 = 17;
        const BUILD_CTX: u32 = 18;
        const BUILD_FB: u32 = 19;
        const BUILD_TAIL: u32 = 20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let subj = rd32(ev + SUBJECT);
        if subj == 0 {
            return ev;
        }
        let code = rd32(ev + CODE);
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        let owner = rd32(this + OWNER);

        if code == CONTEXT_CODE {
            let ctx = rd32(owner + CONTEXT);
            let mut fallback = ctx == 0;
            if !fallback {
                let k: u32 = lf_checker_rt::callee_thiscall!(CTX_CHECK, u32, ctx, owner);
                fallback = k & 0xff == 0;
            }
            if !fallback {
                let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_CTX, u32, global);
                if obj == 0 {
                    wr32(this + RESPONSE, 0);
                    return 0;
                }
                let r: u32 =
                    lf_checker_rt::callee_thiscall!(BUILD_CTX, u32, obj, rd32(owner + CONTEXT), 0);
                wr32(this + RESPONSE, r);
                return r;
            }
            let h: u32 = {
                let o: u32 = lf_checker_rt::callee_thiscall!(ALLOC_FB1, u32, global);
                if o == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD_HELP, u32, o)
                }
            };
            let o2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_FB2, u32, global);
            let m: u32 = if o2 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(BUILD_FB, u32, o2, 0, 0, 1, 0)
            };
            lf_checker_rt::callee_thiscall!(LINK, u32, h, m);
            let o3: u32 = lf_checker_rt::callee_thiscall!(ALLOC_FB3, u32, global);
            let link_ans: u32 = if o3 == 0 {
                lf_checker_rt::callee_thiscall!(LINK, u32, h, 0)
            } else {
                let p = rd32(subj + LOOKUP_OFF).wrapping_add(LOOKUP_ADVANCE);
                let r: u32 =
                    lf_checker_rt::callee_thiscall!(BUILD_RESP, u32, o3, subj, p, SIX, 1, NEG_ONE);
                lf_checker_rt::callee_thiscall!(LINK, u32, h, r)
            };
            wr32(this + RESPONSE, h);
            return link_ans;
        }

        if code != 0x2c2 && code != 0x2c3 && code != 0x2e2 && code != 0x38d && code != 0x38f {
            if code.wrapping_sub(CODE_BASE) > CODE_SPAN {
                return code.wrapping_sub(CODE_BASE);
            }
            return 2;
        }

        // Float path.
        let measure: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(subj) + MEASURE_SLOT) as usize);
        let f = measure(subj);
        if f == 0.0 {
            let mut factors = [0u32; 3];
            let g: u32 = lf_checker_rt::callee_cdecl!(RNG_ZERO, u32,);
            factors[2] = if (g as i32) < GATE_LIMIT as i32 { ONE } else { NEG_ONE };  // a-S06: polarity fixed (original keeps +1.0 on less)
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ZERO, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
            } else {
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_ZERO, u32, obj, subj, 0, factors.as_mut_ptr() as u32, 0, 2
                );
                wr32(this + RESPONSE, r);
            }
        } else {
            // NOTE: the "helper" is the allocation itself (the id-14
            // answer is discarded); and the reload of the saved subject
            // after pushing the helper reads one slot too low, picking up
            // the id-13 answer instead. Both links and the stored response
            // use that answer, while the subject itself is reloaded fresh
            // for the id-16 arguments.
            let h0: u32 = {
                let o: u32 = lf_checker_rt::callee_thiscall!(ALLOC_NZ1, u32, global);
                if o == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(BUILD_HELP, u32, o)
                }
            };
            let o2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_NZ2, u32, global);
            let helper: u32 = if o2 == 0 {
                0
            } else {
                let _: u32 = lf_checker_rt::callee_thiscall!(MAKE_HELPER, u32, o2);
                wr32(o2, lf_checker_rt::relocated(HELPER_VTABLE));
                wr32(o2 + 0x14, 0);
                ((o2 + 0x18) as *mut u16).write_unaligned(HELPER_WORD);
                ((o2 + 0x1a) as *mut u8).write(0);
                o2
            };
            lf_checker_rt::callee_thiscall!(LINK, u32, h0, helper);
            let o3: u32 = lf_checker_rt::callee_thiscall!(ALLOC_NZ3, u32, global);
            if o3 == 0 {
                lf_checker_rt::callee_thiscall!(LINK, u32, h0, 0);
            } else {
                let p = rd32(subj + LOOKUP_OFF).wrapping_add(LOOKUP_ADVANCE);
                let r: u32 =
                    lf_checker_rt::callee_thiscall!(BUILD_RESP, u32, o3, subj, p, SIX, 1, NEG_ONE);
                lf_checker_rt::callee_thiscall!(LINK, u32, h0, r);
            }
            wr32(this + RESPONSE, h0);
        }

        let t: u32 = lf_checker_rt::callee_cdecl!(RNG_TAIL, u32,);
        if (t as i32) >= GATE_LIMIT as i32 {
            return t;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(
            BUILD_TAIL, u32, owner.wrapping_add(INNER_ADVANCE),
            lf_checker_rt::relocated(DESCRIPTOR_TABLE), 0, 0, 0, 0xFFFF_FFFF, 0, 0, ONE, 0, 0
        );
        r
    }
});
