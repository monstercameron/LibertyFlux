// original: 0x00c817f0 CTaskSimpleScenario::vf5

/// Run one scenario tick: type-gate the subject, then run the subtask.
///
/// When `a2` is not 2 and `a3` is non-null, the subject's kind (virtual slot
/// `+4`) must differ from `0x83` or the tick fails with 0. Then, unless the
/// subtask at `this+8` is already latched (bit 0 of `+0xc`), the subtask entry
/// (virtual slot `+0x14`) runs with `(a1, a2, a3)`; a zero result fails with 0
/// and otherwise bit 1 of `+0xc` is set. Success returns 1. Only AL carries
/// the result.
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c817f0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 8;
        const LATCH_OFF: u32 = 0x0c;
        const VT_KIND: u32 = 4;
        const VT_SUB: u32 = 0x14;
        const GATE_KIND: u32 = 2;
        const REFUSE_KIND: u32 = 0x83;
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
