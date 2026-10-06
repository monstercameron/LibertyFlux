// original: 0x00ca6d80 CEventHandler::vf18
/// Vet two subject pointers from the event, check their separation, then
/// either build a proximity response or dispatch on the event code.
///
/// `this` is the handler (thiscall: `this` in ecx, `ev` plus two unread
/// stack words; the callee pops 0xc bytes): `+0x04` is the owner (whose `+0x224` is the
/// inner object), `+0x0c` takes the response, `+0x08` takes follow-up
/// state. `ev+0x18`/`ev+0x1c` are the two subjects, `ev+0x10` the code.
///
/// Head: id 1 vets the owner; three id 2 steps must pass (non-null, equal
/// to `ev+0x1c`, non-null) before id 3 checks a range; its low byte
/// becomes the proceed flag `al` (0 on any earlier failure). The early
/// exits (null subjects, a null owner, subjects unrelated to the owner
/// with `al == 0`) return eax as the original left it: the last callee
/// answer with its low byte replaced by `al`. A squared separation above
/// 8.0 (comparing `dx*dx+dy*dy+dz*dz` with `ja`, so NaN proceeds; the
/// arithmetic order is pinned to the original's) instead returns the
/// first subject's position pointer, which is what eax holds there.
///
/// With `al != 0` the response path allocates through id 4, builds through
/// id 5 (fed the first subject and `0`), stores at `this+0x0c`, forwards
/// through id 6 (fed the inner object, the subject and `1`) and returns
/// the forward answer.
///
/// With `al == 0` the code dispatches (the `0x2d6`/`0x3fe` range tests are
/// signed `jg`, matching the original; no tested code distinguishes the
/// signedness). `0x2d6` and `0x3fe` return the event pointer, which is
/// what eax holds there; `0xc8` stores zero and returns the jump-table
/// byte 0. A jump table serves `0xc8..=0x1ab`: `0x19c` gates on id
/// 7's low byte then builds through id 9, `0x19f` gates then builds
/// through id 8 (fed `0`, `0xbb8`, `-1`), `0x1ab` builds through id 8
/// (fed `0`, `0x98967f`, `-1`); anything else in range, below `0xc8`, or
/// between `0x1ab` and `0x2d6` calls the parent slot at `[this]+0x13c`
/// (id 16, fed the code, the subject and the event). Above `0x2d6`:
/// `0x398` builds through id 10 (fed the subject and the float bits
/// 1.0/`0.02`); `0x38f` tests mask `0x3c0 == 0xc0` at `subject+0x28` and
/// either builds through id 5 with the flag-or at `response+0x60`
/// (faulting exactly like the original on null) and forwards, or builds
/// through id 11 (fed the subject, `1`, 60.0 bits, 1000000, 1000, 1.0
/// bits, `0`) and sets byte `0x39` (faulting on null), then shares a
/// tail (`this+0x08` check returning the forward answer or the id 11
/// answer respectively, id 12 check on its low byte, second allocation,
/// id 14 stdcall, id 15 final build stored at `this+0x08`); `0x76c`
/// re-checks the mask (a bad mask returns the masked value) and runs an
/// optional deep block (id 17 gate, a
/// float above 0.0 where NaN skips, id 18, id 19 with a relocated
/// immediate `this`) before the same build/forward/tail shape with a `0`
/// first final-build argument; anything else calls the parent slot.
///
/// Two codegen quirks are reproduced: the second allocation's answer is
/// saved to the function's own incoming arg0 slot (dead store; the stack
/// check is off and this is disclosed), and the final build's `this` is
/// reloaded from one word above the incoming frame, which reads zero
/// under the checker's zero stack fill on every trial (verified in the
/// call log; the rewrite passes the observed zero, disclosed).
lf_checker_rt::export!(thiscall, rw_00ca6d80(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const SUBJ_A: u32 = 0x18;
        const SUBJ_B: u32 = 0x1c;
        const CODE: u32 = 0x10;
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const RESPONSE: u32 = 0x0c;
        const STATE: u32 = 0x08;
        const POS: u32 = 0x20;
        const FLAG_OFF: u32 = 0x60;
        const FLAG_BIT: u32 = 8;
        const MASK_OFF: u32 = 0x28;
        const MASK_BITS: u32 = 0x3c0;
        const MASK_WANT: u32 = 0xc0;
        const SET_BYTE: u32 = 0x39;
        const PARENT_SLOT: u32 = 0x13c;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const DIST_LIMIT: f32 = 8.0;
        const FIN_ECX: u32 = 0;
        const GATHER: u32 = 1;
        const STEP: u32 = 2;
        const RANGE: u32 = 3;
        const ALLOC_H: u32 = 4;
        const BUILD: u32 = 5;
        const FORWARD: u32 = 6;
        const GATE9: u32 = 7;
        const BUILD_D4: u32 = 8;
        const BUILD_E60: u32 = 9;
        const BUILD_CA: u32 = 10;
        const BUILD_DA: u32 = 11;
        const CHECK: u32 = 12;
        const ALLOC_T: u32 = 13;
        const MAKE: u32 = 14;
        const FINISH: u32 = 15;
        const PARENT: u32 = 16;
        const DEEP1: u32 = 17;
        const DEEP2: u32 = 18;
        const DEEP3: u32 = 19;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write_unaligned(v) }
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
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let sa = rd32(ev + SUBJ_A);
        let sb = rd32(ev + SUBJ_B);
        let owner = rd32(this + OWNER);
        let ga: u32 = lf_checker_rt::callee_thiscall!(GATHER, u32, owner);
        let (al, eaxv): (u32, u32);
        if ga == 0 {
            al = 0;
            eaxv = 0;
        } else {
            let s1: u32 = lf_checker_rt::callee_thiscall!(STEP, u32, ga.wrapping_add(8));
            if s1 == 0 {
                al = 0;
                eaxv = 0;
            } else {
                let s2: u32 = lf_checker_rt::callee_thiscall!(STEP, u32, ga.wrapping_add(8));
                if s2 != sb {
                    al = 0;
                    eaxv = s2 & 0xFFFF_FF00;
                } else {
                    let s3: u32 =
                        lf_checker_rt::callee_thiscall!(STEP, u32, ga.wrapping_add(8));
                    let rcx = rd32(s3.wrapping_add(0x224)) ;
                    let pos = rd32(owner.wrapping_add(POS)).wrapping_add(0x30);
                    let c: u32 = lf_checker_rt::callee_thiscall!(
                        RANGE, u32, rcx.wrapping_add(0x10), pos, 0u32
                    );
                    if (c & 0xFF) == 0 {
                        al = 0;
                        eaxv = c & 0xFFFF_FF00;
                    } else {
                        al = 1;
                        eaxv = (c & 0xFFFF_FF00) | 1;
                    }
                }
            }
        }
        if sa == 0 {
            return eaxv;
        }
        if sb == 0 {
            return eaxv;
        }
        if owner == 0 {
            return eaxv;
        }
        if sa != owner && sb != owner && al == 0 {
            return eaxv;
        }
        let pa = rd32(sa.wrapping_add(POS));
        let pb = rd32(sb.wrapping_add(POS));
        let dx = fsub(rdf(pa.wrapping_add(0x30)), rdf(pb.wrapping_add(0x30)));
        let dy = fsub(rdf(pa.wrapping_add(0x34)), rdf(pb.wrapping_add(0x34)));
        let dz = fsub(rdf(pa.wrapping_add(0x38)), rdf(pb.wrapping_add(0x38)));
        let d = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        if d > DIST_LIMIT {
            return pa;
        }
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));
        if al != 0 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
            let r: u32 = if obj == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(BUILD, u32, obj, sa, 0u32)
            };
            wr32(this + RESPONSE, r);
            let inner = rd32(owner + INNER);
            return lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, sa, 1u32);
        }
        let code = rd32(ev + CODE);
        if (code as i32) > 0x2d6 {
            if code == 0x3fe {
                return ev;
            }
            if (code as i32) > 0x3fe {
                if code == 0x76c {
                    let m76 = rd32(sa.wrapping_add(MASK_OFF)) & MASK_BITS;
                    if m76 != MASK_WANT {
                        return m76;
                    }
                    let w = rd32(owner.wrapping_add(0x21c));
                    if rd32(w.wrapping_add(0x12c)) == 2 && rd8(sa.wrapping_add(0x219)) != 0
                    {
                        let q = rd32(sa.wrapping_add(0x228));
                        let qc: u32 = if q == 0 { 0 } else { q.wrapping_add(0x70) };
                        let a1: u32 = lf_checker_rt::callee_thiscall!(DEEP1, u32, qc);
                        if a1 == 0 {
                            let q2 = rd32(sa.wrapping_add(0x228));
                            let qa: u32 = if q2 == 0 { 0 } else { q2.wrapping_add(0x70) };
                            if rdf(qa.wrapping_add(0x1c)) > 0.0 {
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    DEEP2, u32, sa, 1u32, 0x3E8u32
                                );
                                let p3 = rd32(sa.wrapping_add(POS)).wrapping_add(0x30);
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    DEEP3,
                                    u32,
                                    lf_checker_rt::relocated(0x0128_AA90),
                                    p3,
                                    0xCu32,
                                    0x3E8u32
                                );
                            }
                        }
                    }
                    let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
                    let r: u32 = if obj == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(BUILD, u32, obj, sa, 0u32)
                    };
                    wr32(this + RESPONSE, r);
                    let inner = rd32(owner + INNER);
                    let f: u32 =
                        lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, sa, 1u32);
                    if rd32(this + STATE) != 0 {
                        return f;
                    }
                    let k: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, owner, ev);
                    if (k & 0xFF) == 0 {
                        return k;
                    }
                    let obj2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_T, u32, global);
                    if obj2 == 0 {
                        wr32(this + STATE, 0);
                        return 0;
                    }
                    let t: u32 = lf_checker_rt::callee_stdcall!(MAKE, u32, owner, sa, 0u32);
                    let g: u32 =
                        lf_checker_rt::callee_thiscall!(FINISH, u32, FIN_ECX, 0u32, 1u32, t);
                    wr32(this + STATE, g);
                    return g;
                }
                let par: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
                return par(this, code, sa, ev);
            }
            let t = code.wrapping_sub(0x38f);
            if t == 0 {
                let tail_eax: u32;
                if rd32(sa.wrapping_add(MASK_OFF)) & MASK_BITS == MASK_WANT {
                    let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
                    let r: u32 = if obj == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(BUILD, u32, obj, sa, 0u32)
                    };
                    wr32(this + RESPONSE, r);
                    wr32(r + FLAG_OFF, rd32(r + FLAG_OFF) | FLAG_BIT);
                    let inner = rd32(owner + INNER);
                    tail_eax =
                        lf_checker_rt::callee_thiscall!(FORWARD, u32, inner, sa, 1u32);
                } else {
                    let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
                    let r: u32 = if obj == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            BUILD_DA, u32, obj, sa, 1u32, 0x4270_0000u32, 1_000_000u32,
                            1000u32, 0x3F80_0000u32, 0u32
                        )
                    };
                    wr32(this + RESPONSE, r);
                    wr8(r + SET_BYTE, 1);
                    tail_eax = r;
                }
                if rd32(this + STATE) != 0 {
                    return tail_eax;
                }
                let k: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, owner, ev);
                if (k & 0xFF) == 0 {
                    return k;
                }
                let obj2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_T, u32, global);
                if obj2 == 0 {
                    wr32(this + STATE, 0);
                    return 0;
                }
                let t: u32 = lf_checker_rt::callee_stdcall!(MAKE, u32, owner, sa, 0u32);
                let g: u32 =
                    lf_checker_rt::callee_thiscall!(FINISH, u32, FIN_ECX, 1u32, 1u32, t);
                wr32(this + STATE, g);
                return g;
            }
            if t.wrapping_sub(9) != 0 {
                let par: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
                return par(this, code, sa, ev);
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_CA, u32, obj, sa, 0x3F80_0000u32, 0x3CA3_D70Au32
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x2d6 {
            return ev;
        }
        if code.wrapping_sub(0xc8) > 0xe3 {
            let par: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
            return par(this, code, sa, ev);
        }
        if code == 0xc8 {
            wr32(this + RESPONSE, 0);
            return 0;
        }
        if code == 0x19c {
            let k: u32 = lf_checker_rt::callee_thiscall!(GATE9, u32, owner);
            if (k & 0xFF) != 0 {
                return k;
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(BUILD_E60, u32, obj);
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x19f {
            let k: u32 = lf_checker_rt::callee_thiscall!(GATE9, u32, owner);
            if (k & 0xFF) != 0 {
                return k;
            }
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_D4, u32, obj, 0u32, 0xBB8u32, 0xFFFF_FFFFu32
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        if code == 0x1ab {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC_H, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_D4, u32, obj, 0u32, 0x0989_67Fu32, 0xFFFF_FFFFu32
            );
            wr32(this + RESPONSE, r);
            return r;
        }
        let par: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + PARENT_SLOT) as usize);
        par(this, code, sa, ev)
    }
});
