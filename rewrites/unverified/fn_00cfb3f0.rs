// original: 0x00cfb3f0 climb_ladder_big_update (proposed) -- STAGE 1 (early exits only)

/// Climb-ladder big update, stage 1: table lookup, count gates and the
/// poll-reject loop. STAGE-1 PARTIAL rewrite: only the early-exit paths are
/// implemented (see plan in the lane folder for the rest).
///
/// `a0+0x2e` (signed 16-bit) indexes a global object table; the picked
/// object must have flag 0x10000 in `+0x40` or the function returns 1. A
/// count call must answer non-zero and then a positive count, else 1.
/// Each of `count` iterations fetches the next item and polls indirect
/// slot 1: any answer but 0xe ends the iteration (stage 1 steers every
/// trial here, so the 0xe matrix/loop body is not implemented yet).
/// Returns 1 on every stage-1 path.
///
/// Original: 0x00cfb3f0 (cdecl, two stack arguments, returns al).
lf_checker_rt::export!(cdecl, rw_00cfb3f0(a0: u32, _a1: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x2e;
        const FLAG_OFF: u32 = 0x40;
        const FLAG_BIT: u32 = 0x1_0000;
        const TABLE: u32 = 0x0129_5cd8;
        const POLL_SLOT: u32 = 4;
        const POLL_GO: u32 = 0xe;
        const COUNT: u32 = 1;
        const NEXT: u32 = 2;
        const POLL: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let idx = (a0 + INDEX_OFF) as *const u16;
        let idx = unsafe { idx.read_unaligned() } as i16 as i32 as u32;
        let edi = rd32(lf_checker_rt::relocated(TABLE) + idx.wrapping_mul(4));
        if rd32(edi + FLAG_OFF) & FLAG_BIT == 0 {
            return 1;
        }
        let c: u32 = lf_checker_rt::callee_thiscall!(COUNT, u32, edi);
        if c == 0 {
            return 1;
        }
        let count: u32 = lf_checker_rt::callee_thiscall!(COUNT, u32, edi);
        if (count as i32) <= 0 {
            return 1;
        }
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let it: u32 = lf_checker_rt::callee_thiscall!(NEXT, u32, edi, i);
            let target = rd32(rd32(it) + POLL_SLOT);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            // STAGE 1: the contract answers never-0xe, so only poll-reject
            // is reachable; the 0xe body is stage 2.
            if f(it) == POLL_GO {
                core::hint::black_box(());
            }
            i = i.wrapping_add(1);
            let _ = POLL;
        }
        1
    }
});
