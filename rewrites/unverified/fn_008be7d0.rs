// original: 0x008BE7D0 menu_state_dispatch (proposed)

/// Poll the menu state machine and run one dispatch step.
///
/// With no arguments: ask the state poller for the current state. State 1
/// returns 0 at once; state 2 refreshes through the refresh callee and
/// returns its answer. Any other state asks the gate callee: when it agrees,
/// a follow-up check of 0 or 2 also refreshes, while any other answer is
/// returned directly. When the gate refuses, a selector callee picks one of
/// five cases (writing an out-word through its pointer argument) or the
/// default chain: refresh directly; run a three-callee chain; run a
/// two-callee chain; re-poll and refresh only on state 2; or walk the
/// default chain, which compares a detail callee's answer against three
/// sentinel values, consults a mode flag and a bound callee, and ends in one
/// of two two-argument update calls. Return values are the deciding callee's
/// answer, except states that return 0 after the decrement.
///
/// The second poll inside case 3 needs its own scripted answer (a per-call
/// sequence), because the shared per-trial answer can never be 2 there: a 2
/// on the first poll takes an earlier exit. One branch is unreachable while
/// callees are stubs: the mode flag cannot change between its two reads, so
/// the second read always agrees with the first.
///
/// Original: 0x008BE7D0 (cdecl, no arguments; fifteen direct callees, one
/// out-word through a stack-slot pointer, one five-way jump table).
lf_checker_rt::export!(cdecl, rw_008BE7D0() -> u32 {
    unsafe {
        const WORD_A: u32 = 0x01160C10;
        const MODE: u32 = 0x01160C32;
        const C_POLL: u32 = 0;
        const C_GATE: u32 = 1;
        const C_CHECK: u32 = 2;
        const C_REFRESH: u32 = 3;
        const C_SELECT: u32 = 4;
        const C_CHAIN1: u32 = 5;
        const C_CHAIN2: u32 = 6;
        const C_CHAIN3: u32 = 7;
        const C_APPLY: u32 = 8;
        const C_FINISH: u32 = 9;
        const C_PROBE: u32 = 10;
        const C_DETAIL: u32 = 11;
        const C_BOUND: u32 = 12;
        const C_TEST: u32 = 13;
        const C_UPDATE: u32 = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(a).read() }
        }

        let st: u32 = lf_checker_rt::callee_cdecl!(C_POLL, u32, 0u32, 0u32);
        if st.wrapping_sub(1) == 0 {
            return 0;
        }
        if st.wrapping_sub(2) == 0 {
            return lf_checker_rt::callee_cdecl!(C_REFRESH, u32,);
        }
        let g: u32 = lf_checker_rt::callee_cdecl!(C_GATE, u32,);
        if (g as u8) != 0 {
            let c: u32 = lf_checker_rt::callee_cdecl!(C_CHECK, u32, 1u32);
            if c == 0 || c == 2 {
                return lf_checker_rt::callee_cdecl!(C_REFRESH, u32,);
            }
            return c;
        }
        let mut slot: u32 = 0xFFFF_FFFF;
        let r: u32 = lf_checker_rt::callee_cdecl!(C_SELECT, u32, &mut slot as *mut u32 as u32);
        let case: u32 = r.wrapping_sub(1);
        if case > 4 {
            return {
                let a: u32 = rd32(WORD_A);
                let p: u32 = lf_checker_rt::callee_cdecl!(C_PROBE, u32, a);
                if (p as u8) == 0 {
                    p
                } else {
                    let d: u32 = lf_checker_rt::callee_cdecl!(C_DETAIL, u32, rd32(WORD_A));
                    let s: i32 = d as i32;
                    if s == -0x5C || s == 0x0F || s == -0x5B {
                        if s == -0x5C {
                            d
                        } else {
                            lf_checker_rt::callee_cdecl!(C_REFRESH, u32,)
                        }
                    } else if rd8(MODE) != 0 {
                        lf_checker_rt::callee_cdecl!(C_UPDATE, u32, d, 2u32)
                    } else if s < 0 {
                        d
                    } else {
                        let b: u32 = lf_checker_rt::callee_cdecl!(C_BOUND, u32, rd32(WORD_A));
                        if s >= b as i32 {
                            if rd8(MODE) != 0 {
                                lf_checker_rt::callee_cdecl!(C_UPDATE, u32, d, 2u32)
                            } else {
                                b
                            }
                        } else {
                            let t: u32 =
                                lf_checker_rt::callee_cdecl!(C_TEST, u32, rd32(WORD_A), d);
                            if (t as u8) == 0 {
                                if rd8(MODE) != 0 {
                                    lf_checker_rt::callee_cdecl!(C_UPDATE, u32, d, 2u32)
                                } else {
                                    t
                                }
                            } else {
                                lf_checker_rt::callee_cdecl!(C_UPDATE, u32, d, 1u32)
                            }
                        }
                    }
                }
            };
        }
        match case {
            0 => 0,
            1 => {
                lf_checker_rt::callee_cdecl!(C_CHAIN1, u32, 0x3Fu32);
                lf_checker_rt::callee_cdecl!(C_CHAIN2, u32, slot);
                lf_checker_rt::callee_cdecl!(C_CHAIN3, u32, slot)
            }
            2 => {
                lf_checker_rt::callee_cdecl!(C_APPLY, u32, slot);
                lf_checker_rt::callee_cdecl!(C_FINISH, u32, rd32(WORD_A), 0u32)
            }
            3 => {
                let st2: u32 = lf_checker_rt::callee_cdecl!(C_POLL, u32, 1u32, 0u32);
                if st2 == 2 {
                    lf_checker_rt::callee_cdecl!(C_REFRESH, u32,)
                } else {
                    st2
                }
            }
            _ => lf_checker_rt::callee_cdecl!(C_REFRESH, u32,),
        }
    }
});
