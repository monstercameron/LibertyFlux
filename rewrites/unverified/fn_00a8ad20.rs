// original: 0x00a8ad20 pool_refill_loop (proposed)

/// Refill the pool head until the stage callee idles on a primed pool.
///
/// `this` is the pool, `base` the window base and `arg2` the settle
/// bound. Binds the slot, clears the state word, then runs the step
/// callee on the tick constant and resolves its answer: a non-null
/// resolution is registered. Then loops while the shared window still
/// covers `base` or the resolution sits below `arg2`: each pass runs the
/// stage callee on 0x20 and re-steps. The loop exits through the settle
/// test (returning the window bit in the low byte over the window
/// leftover) or, when the stage idles with the pool primed (word +0x48
/// at 7), through the final window test. The proof pins the window,
/// primes the pool word and scripts the stage answers to two passes.
///
/// Original: 0x00A8AD20 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a8ad20(this: u32, base: u32, arg2: u32) -> u32 {
    unsafe {
        const CALLEE_BIND: u32 = 1;
        const CALLEE_STEP: u32 = 2;
        const CALLEE_RESOLVE: u32 = 3;
        const CALLEE_REGISTER: u32 = 4;
        const CALLEE_STAGE: u32 = 5;
        const WINDOW_HI: u32 = 0x103e8f0;
        const WINDOW_LO: u32 = 0x103e8f4;
        const REGISTRY: u32 = 0x103e8d0;
        const STATE: u32 = 4;
        const PRIMED: u32 = 0x48;
        const PRIMED_VALUE: u32 = 7;
        const STAGE_ARG: u32 = 0x20;
        const TICK_BITS: u32 = 0x3f666666;
        lf_checker_rt::callee_thiscall!(CALLEE_BIND, u32, this);
        ((this + STATE) as *mut u32).write_unaligned(0);
        // The original calls the step callee without setting ecx, so it
        // sees the previous stub's exit value (0). Pass 0 to match.
        let step = lf_checker_rt::callee_thiscall!(
            CALLEE_STEP,
            u32,
            0,
            TICK_BITS
        );
        let mut resolved =
            lf_checker_rt::callee_thiscall!(CALLEE_RESOLVE, u32, step);
        if resolved != 0 {
            lf_checker_rt::callee_thiscall!(
                CALLEE_REGISTER,
                u32,
                lf_checker_rt::relocated(REGISTRY),
                resolved
            );
        }
        loop {
            let hi =
                lf_checker_rt::global::<u32>(WINDOW_HI).read_unaligned();
            let lo =
                lf_checker_rt::global::<u32>(WINDOW_LO).read_unaligned();
            let rel = hi.wrapping_sub(base);
            if !(lo > rel) {
                if (resolved as i32) >= (arg2 as i32) {
                    return (rel & 0xffffff00) | 1;
                }
            }
            let busy = lf_checker_rt::callee_thiscall!(
                CALLEE_STAGE,
                u32,
                this,
                STAGE_ARG
            );
            let step = lf_checker_rt::callee_thiscall!(
                CALLEE_STEP,
                u32,
                0,
                TICK_BITS
            );
            resolved =
                lf_checker_rt::callee_thiscall!(CALLEE_RESOLVE, u32, step);
            if busy as u8 != 0 {
                continue;
            }
            let primed =
                ((this + PRIMED) as *const u32).read_unaligned();
            if primed != PRIMED_VALUE {
                continue;
            }
            let hi =
                lf_checker_rt::global::<u32>(WINDOW_HI).read_unaligned();
            let lo =
                lf_checker_rt::global::<u32>(WINDOW_LO).read_unaligned();
            let rel = hi.wrapping_sub(base);
            return (rel & 0xffffff00) | ((lo < rel) as u32);
        }
    }
});
