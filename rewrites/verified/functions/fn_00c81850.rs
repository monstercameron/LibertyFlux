// original: 0x00c81850 CTaskComplexMobileChatScenario::vf5

/// Run one mobile-chat tick: gate the subject, run the subtask, service the phone.
///
/// When `a3` is non-null and `a2` is not 2, the subject's kind (virtual slot
/// `+4`) is asked: on `0x84` a second ask below 12 (signed) fails with 0,
/// then any ask of `0x83` fails with 0. Unless the subtask at `this+8` is
/// already latched (bit 0 of `+0xc`), the subtask entry (virtual slot `+0x14`)
/// runs with `(a1, a2, a3)`; a zero result fails with 0 and otherwise bit 1 of
/// `+0xc` is set. The phone helper then runs on `a1+0x2b0` when the marker at
/// `this+0x14` reads `0x2b`, or when the pointer at `a1+0x2c4` is non-null and
/// the slot-`+0x12c` hook on `a1` returns the sign-extended word at that
/// pointer's `+0x2e`. Two guarded helper pairs follow (`a1+0x570`, then
/// `a1+0x3c0`): the first of each pair runs, and a non-zero result runs the
/// second. Success returns 1. Only AL carries the result.
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c81850(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 8;
        const LATCH_OFF: u32 = 0x0c;
        const MARK_OFF: u32 = 0x14;
        const MARK_WANT: u32 = 0x2b;
        const VT_KIND: u32 = 4;
        const VT_SUB: u32 = 0x14;
        const VT_SLOT: u32 = 0x12c;
        const GATE_KIND: u32 = 2;
        const REASK_KIND: u32 = 0x84;
        const REASK_BOUND: i32 = 0x0c;
        const REFUSE_KIND: u32 = 0x83;
        const REC_OFF: u32 = 0x2c4;
        const WORD_OFF: u32 = 0x2e;
        const PHONE_OFF: u32 = 0x2b0;
        const AUX_OFF: u32 = 0x570;
        const PAIR_OFF: u32 = 0x3c0;
        const C_PHONE: u32 = 4;
        const C_AUX1: u32 = 5;
        const C_AUX2: u32 = 6;
        const C_P1A: u32 = 7;
        const C_P1B: u32 = 8;
        const C_P2A: u32 = 9;
        const C_P2B: u32 = 10;
        type KindHook = extern "thiscall" fn(u32) -> u32;
        type SubHook = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        if a3 != 0 && a2 != GATE_KIND {
            let avt = (a3 as *const u32).read_unaligned();
            let kind_of: KindHook =
                core::mem::transmute(((avt + VT_KIND) as *const u32).read_unaligned() as usize);
            if kind_of(a3) == REASK_KIND && (kind_of(a3) as i32) < REASK_BOUND {
                return 0;
            }
            let avt2 = (a3 as *const u32).read_unaligned();
            let kind_of2: KindHook =
                core::mem::transmute(((avt2 + VT_KIND) as *const u32).read_unaligned() as usize);
            if kind_of2(a3) == REFUSE_KIND {
                return 0;
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
        let mut fire_phone = ((this + MARK_OFF) as *const u32).read_unaligned() == MARK_WANT;
        if !fire_phone {
            let rec = ((a1 + REC_OFF) as *const u32).read_unaligned();
            if rec != 0 {
                let want = ((rec + WORD_OFF) as *const i16).read_unaligned() as i32 as u32;
                let avt = (a1 as *const u32).read_unaligned();
                let slot: KindHook = core::mem::transmute(
                    ((avt + VT_SLOT) as *const u32).read_unaligned() as usize,
                );
                fire_phone = slot(a1) == want;
            }
        }
        if fire_phone {
            lf_checker_rt::callee_thiscall!(C_PHONE, u32, a1 + PHONE_OFF);
        }
        let r1: u32 = lf_checker_rt::callee_thiscall!(C_AUX1, u32, a1 + AUX_OFF);
        if r1 & 0xff != 0 {
            lf_checker_rt::callee_thiscall!(C_AUX2, u32, a1 + AUX_OFF);
        }
        let e = a1 + PAIR_OFF;
        let r2: u32 = lf_checker_rt::callee_thiscall!(C_P1A, u32, e);
        if r2 & 0xff != 0 {
            lf_checker_rt::callee_thiscall!(C_P1B, u32, e);
        }
        let r3: u32 = lf_checker_rt::callee_thiscall!(C_P2A, u32, e);
        if r3 & 0xff != 0 {
            lf_checker_rt::callee_thiscall!(C_P2B, u32, e);
        }
        1
    }
});
