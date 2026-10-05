// original: 0x00c81a40 CTaskComplexPoliceSniperScenario::vf5

/// Run one sniper-scenario tick: check the spotter chain, then run the subtask.
///
/// When `a3` is non-null, the chain at `a1+0x224` is walked: the slot-`+0x20`
/// hook runs, and when its result's word at `+0xc` is non-zero the hook runs
/// again and the slot-`+0xc` hook on that word must report `0x76c`, else the
/// tick skips to the subtask. On `0x76c` the `+0x20` hook runs a third time:
/// a zero word at its result's `+8` counts one at `a3+4` and fails with 0,
/// otherwise the hook runs a fourth time and its result is finished through
/// the direct helper. Then, unless the subtask at `this+8` is already latched
/// (bit 0 of `+0xc`), the subtask entry (virtual slot `+0x14`) runs with
/// `(a1, a2, a3)`; a zero result fails with 0 and otherwise bit 1 of `+0xc`
/// is set. Success returns 1. Only AL carries the result.
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c81a40(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 8;
        const LATCH_OFF: u32 = 0x0c;
        const VT_SUB: u32 = 0x14;
        const CHAIN_OFF: u32 = 0x224;
        const VT_NEXT: u32 = 0x20;
        const VT_KIND: u32 = 0x0c;
        const STATE_OFF: u32 = 0x0c;
        const READY_OFF: u32 = 8;
        const COUNT_OFF: u32 = 4;
        const WANT_KIND: u32 = 0x76c;
        const C_FIN: u32 = 4;
        type NextHook = extern "thiscall" fn(u32) -> u32;
        type KindHook = extern "thiscall" fn(u32) -> u32;
        type SubHook = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        if a3 != 0 {
            let chain = ((a1 + CHAIN_OFF) as *const u32).read_unaligned();
            let cvt = (chain as *const u32).read_unaligned();
            let next: NextHook =
                core::mem::transmute(((cvt + VT_NEXT) as *const u32).read_unaligned() as usize);
            let st = next(chain);
            if ((st + STATE_OFF) as *const u32).read_unaligned() != 0 {
                let chain2 = ((a1 + CHAIN_OFF) as *const u32).read_unaligned();
                let cvt2 = (chain2 as *const u32).read_unaligned();
                let next2: NextHook = core::mem::transmute(
                    ((cvt2 + VT_NEXT) as *const u32).read_unaligned() as usize,
                );
                let q = ((next2(chain2) + STATE_OFF) as *const u32).read_unaligned();
                let qvt = (q as *const u32).read_unaligned();
                let kind_of: KindHook = core::mem::transmute(
                    ((qvt + VT_KIND) as *const u32).read_unaligned() as usize,
                );
                if kind_of(q) == WANT_KIND {
                    let chain3 = ((a1 + CHAIN_OFF) as *const u32).read_unaligned();
                    let cvt3 = (chain3 as *const u32).read_unaligned();
                    let next3: NextHook = core::mem::transmute(
                        ((cvt3 + VT_NEXT) as *const u32).read_unaligned() as usize,
                    );
                    if ((next3(chain3) + READY_OFF) as *const u32).read_unaligned() == 0 {
                        let n = ((a3 + COUNT_OFF) as *const u32).read_unaligned();
                        ((a3 + COUNT_OFF) as *mut u32).write_unaligned(n.wrapping_add(1));
                        return 0;
                    }
                    let chain4 = ((a1 + CHAIN_OFF) as *const u32).read_unaligned();
                    let cvt4 = (chain4 as *const u32).read_unaligned();
                    let next4: NextHook = core::mem::transmute(
                        ((cvt4 + VT_NEXT) as *const u32).read_unaligned() as usize,
                    );
                    let fin_arg = next4(chain4);
                    lf_checker_rt::callee_thiscall!(C_FIN, u32, fin_arg);
                }
            }
        }
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if ((sub + LATCH_OFF) as *const u8).read() & 1 == 0 {
            let svt = (sub as *const u32).read_unaligned();
            let run: SubHook =
                core::mem::transmute(((svt + VT_SUB) as *const u32).read_unaligned() as usize);
            if run(sub, a1, a2, a3) & 0xff == 0 {
                return 0;
            }
            let latch = ((sub + LATCH_OFF) as *mut u8).read();
            ((sub + LATCH_OFF) as *mut u8).write(latch | 2);
        }
        1
    }
});
