// original: 0x00c81760 CTaskComplexDrivingScenario::vf5

/// Run one driving-scenario tick: type-gate the subject, clear a flag, run the subtask.
///
/// When `a2` is not 2 and `a3` is non-null, the subject's kind (virtual slot
/// `+4`) must differ from `0x83` or the tick fails with 0. When the marker at
/// `this+0x14` reads `0x5a` and `a3` is non-null, the subject's kind is asked
/// again: on `0x7f` with bit 2 of `a1+0x26c` clear and a non-null pointer at
/// `this+0x24`, bit 7 of that pointer's byte at `+0xf1d` is cleared. Then,
/// unless the subtask at `this+8` is already latched (bit 0 of `+0xc`), the
/// subtask entry (virtual slot `+0x14`) runs with `(a1, a2, a3)`; a zero
/// result fails with 0 and otherwise bit 1 of `+0xc` is set. Success returns
/// 1. Only AL carries the result.
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c81760(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 8;
        const LATCH_OFF: u32 = 0x0c;
        const MARK_OFF: u32 = 0x14;
        const MARK_WANT: u32 = 0x5a;
        const PTR_OFF: u32 = 0x24;
        const FLAG_OFF: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const CLEAR_OFF: u32 = 0xf1d;
        const VT_KIND: u32 = 4;
        const VT_SUB: u32 = 0x14;
        const GATE_KIND: u32 = 2;
        const REFUSE_KIND: u32 = 0x83;
        const CLEAR_KIND: u32 = 0x7f;
        type KindHook = extern "thiscall" fn(u32) -> u32;
        type SubHook = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        if a2 != GATE_KIND && a3 != 0 {
            let avt = (a3 as *const u32).read_unaligned();
            let kind_of: KindHook =
                core::mem::transmute(((avt + VT_KIND) as *const u32).read_unaligned() as usize);
            if kind_of(a3) == REFUSE_KIND {
                return 0;
            }
        }
        if ((this + MARK_OFF) as *const u32).read_unaligned() == MARK_WANT && a3 != 0 {
            let avt = (a3 as *const u32).read_unaligned();
            let kind_of: KindHook =
                core::mem::transmute(((avt + VT_KIND) as *const u32).read_unaligned() as usize);
            if kind_of(a3) == CLEAR_KIND
                && ((a1 + FLAG_OFF) as *const u8).read() & FLAG_BIT == 0
            {
                let p = ((this + PTR_OFF) as *const u32).read_unaligned();
                if p != 0 {
                    let b = ((p + CLEAR_OFF) as *mut u8).read();
                    ((p + CLEAR_OFF) as *mut u8).write(b & 0x7f);
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
