// original: 0x00c819e0 CTaskSimpleScenario::vf5

/// Run one scenario tick, then release the finished marker when set.
///
/// Unless the subtask at `this+8` is already latched (bit 0 of `+0xc`), the
/// subtask entry (virtual slot `+0x14`) runs with `(a1, a2, a3)`; a zero
/// result fails with 0 and otherwise bit 1 of `+0xc` is set. Then, when the
/// marker at `this+0x28` reads 1, it is cleared and the payload at `this+0x2c`
/// is released through `c1` if non-null before that slot is cleared too.
/// Success returns 1. Only AL carries the result.
///
/// Original: thiscall, three stack words (the callee pops 0xc bytes).
lf_checker_rt::export!(thiscall, rw_00c819e0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 8;
        const LATCH_OFF: u32 = 0x0c;
        const VT_SUB: u32 = 0x14;
        const MARK_OFF: u32 = 0x28;
        const PAYLOAD_OFF: u32 = 0x2c;
        const C_RELEASE: u32 = 2;
        type SubHook = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
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
        if ((this + MARK_OFF) as *const u32).read_unaligned() == 1 {
            ((this + MARK_OFF) as *mut u32).write_unaligned(0);
            let payload = ((this + PAYLOAD_OFF) as *const u32).read_unaligned();
            if payload != 0 {
                lf_checker_rt::callee_cdecl!(C_RELEASE, u32, payload);
            }
            ((this + PAYLOAD_OFF) as *mut u32).write_unaligned(0);
        }
        1
    }
});
