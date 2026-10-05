// original: 0x00cd9270 task_event_dispatch_large (proposed)

/// Large event dispatcher for a ped task (thiscall, two stack words).
///
/// `this` is the task object (`+0x18` points at a record read at `+0xB30` on
/// one path and passed as a word on two others), `ev` the event code, `p1` a
/// record read at `+0xB30` on two paths. Every path first fetches a worker
/// through the manager getter; a null worker returns 0, except on the
/// `0x38B` path where a null first worker only zeroes an accumulator. The
/// return value is otherwise the request's answer.
///
/// Dispatch on `ev` (ordering comparisons signed): `0xCB` runs the four-word
/// request (8.0, 0, 0, 0x1F4); `0xCA` the one-word request (0x7D0); `0x119`
/// tail-calls the two-word filler with (0, 0) after overwriting both
/// incoming argument slots with zero (dead after the return, so the rewrite
/// forwards the zeroes as call arguments and the proof runs with the stack
/// check off); `0x2D4` the fifteen-word setup request (1, 0x14, 0x1E, -1.0,
/// 0x14, 4, 0, 0, 0, 0, 0x28, 2, 7, `[this+0x18]`, `[p1+0xB30]`);
/// `0x2DE` the five-word request (0, 0, 0x1B, -4, `[[this+0x18]+0xB30]`);
/// `0x2E2` the four-word request (0, 0, 0, `[p1+0xB30]`); `0x38B` runs the
/// seven-word setup request (`[this+0x18]`, 50000, 1000, 4.0, 2.0, 2.0, 1)
/// and then the four-word finish request (setup answer, 0, 0, 0) with a
/// second worker, returning 0 when that worker is null. Anything else,
/// including `0x516`, returns 0.
///
/// Original: 0x00cd9270 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00cd9270(this: u32, ev: u32, p1: u32) -> u32 {
    unsafe {
        const REC18_OFF: u32 = 0x18;
        const REC_B30: u32 = 0xb30;
        const EV_SMALL: u32 = 0xcb;
        const EV_ONE: u32 = 0xca;
        const EV_TAIL: u32 = 0x119;
        const EV_SETUP: u32 = 0x2d4;
        const EV_FIVE: u32 = 0x2de;
        const EV_FOUR: u32 = 0x2e2;
        const EV_TWO_STAGE: u32 = 0x38b;
        const MANAGER: u32 = 0x0167e2a0;
        const NEG_ONE: u32 = 0xbf80_0000;
        const EIGHT: u32 = 0x4100_0000;
        const TWO_BITS: u32 = 0x4000_0000;
        const FOUR_BITS: u32 = 0x4080_0000;
        const CALLEE_GET: u32 = 1;
        const CALLEE_SMALL: u32 = 2;
        const CALLEE_ONE: u32 = 3;
        const CALLEE_TAIL: u32 = 4;
        const CALLEE_SETUP: u32 = 5;
        const CALLEE_FIVE: u32 = 6;
        const CALLEE_GET_S1: u32 = 7;
        const CALLEE_GET_S2: u32 = 8;
        const CALLEE_STAGE1: u32 = 9;
        const CALLEE_STAGE2: u32 = 10;
        const CALLEE_FOUR: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn worker() -> u32 {
            unsafe {
                let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
                lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr)
            }
        }

        if ev == EV_FIVE {
            let g = worker();
            if g == 0 {
                return 0;
            }
            let rec = rd32(this + REC18_OFF);
            return lf_checker_rt::callee_thiscall!(
                CALLEE_FIVE, u32, g, rd32(rec + REC_B30), 0xffff_fffcu32, 0x1bu32, 0u32, 0u32
            );
        }
        if (ev as i32) > EV_FIVE as i32 {
            if ev == EV_FOUR {
                let g = worker();
                if g == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    CALLEE_FOUR, u32, g, rd32(p1 + REC_B30), 0u32, 0u32, 0u32
                );
            }
            if ev != EV_TWO_STAGE {
                return 0;
            }
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET_S1, u32, mgr);
            let mid = if g1 == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_STAGE1, u32, g1, rd32(this + REC18_OFF), 50000u32, 1000u32,
                    FOUR_BITS, TWO_BITS, TWO_BITS, 1u32
                )
            };
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET_S2, u32, mgr);
            if g2 == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CALLEE_STAGE2, u32, g2, mid, 0u32, 0u32, 0u32);
        }
        if (ev as i32) > EV_TAIL as i32 {
            if ev != EV_SETUP {
                return 0;
            }
            let g = worker();
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_SETUP, u32, g, rd32(p1 + REC_B30), rd32(this + REC18_OFF), 7u32, 2u32,
                0x28u32, 0u32, 0u32, 0u32, 0u32, 4u32, 0x14u32, NEG_ONE, 0x1eu32, 0x14u32, 1u32
            );
        }
        if ev == EV_TAIL {
            let g = worker();
            if g == 0 {
                return 0;
            }
            // Tail call in the original; a forwarding call here.
            return lf_checker_rt::callee_thiscall!(CALLEE_TAIL, u32, g, 0u32, 0u32);
        }
        if ev == EV_ONE {
            let g = worker();
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CALLEE_ONE, u32, g, 0x7d0u32);
        }
        if ev != EV_SMALL {
            return 0;
        }
        let g = worker();
        if g == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_SMALL, u32, g, 0x1f4u32, 0u32, 0u32, EIGHT)
    }
});
