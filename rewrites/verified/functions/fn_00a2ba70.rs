// original: 0x00a2ba70 ped_candidate_free

/// Report whether a task candidate counts as free.
/// The candidate arrives as a stack pointer (the function ignores its own
/// ECX). A null candidate is free; otherwise the state field at `+0x28`
/// masked with 0x3C0 must differ from 0xC0, or the hold flag 0x400 at
/// `+0x260` must be clear. Only a held busy candidate answers 0.
/// Original: 0x00a2ba70 (stdcall, one stack word, byte result).
lf_checker_rt::export!(stdcall, rw_00a2ba70(cand: u32) -> u8 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
        const STATE_OFF: u32 = 0x28;
        const STATE_MASK: u32 = 0x3c0;
        const STATE_BUSY: u32 = 0xc0;
        const HOLD_OFF: u32 = 0x260;
        const HOLD_BIT: u32 = 0x400;
        if cand == 0 {
            return 1;
        }
        if (rd32(cand.wrapping_add(STATE_OFF)) & STATE_MASK) != STATE_BUSY {
            return 1;
        }
        if (rd32(cand.wrapping_add(HOLD_OFF)) & HOLD_BIT) == 0 {
            1
        } else {
            0
        }
    }
});
