// original: 0x00a8a820 pool_try_stage_transitions (proposed)

/// Drive the record's stage machine one step, returning 1 on progress.
///
/// `this` is the pool object, `arg` a token or-ed with 0xC6 for the stage
/// callees and `rec` the record (stage at +0). First the probe callee runs
/// on ([rec], token): a true answer jumps to the finish callee. Otherwise
/// stage 6 runs the gate-0 callee, stage 2 (when the flag callee agrees)
/// runs the gate-1 callee, and a true answer from either also jumps to the
/// finish callee. Stages below 5 (signed) succeed directly. The finish
/// callee runs on the token for stages 5 and above; a true answer, or a
/// stage below 7, succeeds by clearing [this+4] and incrementing the
/// stage, while stage 7 and above with a false answer fails. Returns 1 on
/// success, 0 on failure (low byte; the scripts only answer 0/1 so the
/// full word is deterministic).
///
/// Original: 0x00A8A820 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a8a820(this: u32, arg: u32, rec: u32) -> u32 {
    unsafe {
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_GATE: u32 = 2;
        const CALLEE_FLAG: u32 = 3;
        const CALLEE_FINISH: u32 = 4;
        const TOKEN_BITS: u32 = 0xc6;
        const REC_STAGE: u32 = 0;
        const MARK: u32 = 4;
        let token = arg | TOKEN_BITS;
        let finish = |this: u32, token: u32| -> u32 {
            let f = lf_checker_rt::callee_thiscall!(
                CALLEE_FINISH,
                u32,
                this,
                token
            );
            if f as u8 != 0 { 1 } else { 0 }
        };
        let stage0 = ((rec + REC_STAGE) as *const u32).read_unaligned();
        let p = lf_checker_rt::callee_thiscall!(
            CALLEE_PROBE,
            u32,
            this,
            stage0,
            token
        );
        if p as u8 != 0 {
            return finish(this, token);
        }
        let stage = ((rec + REC_STAGE) as *const u32).read_unaligned();
        if stage == 6 {
            let g = lf_checker_rt::callee_thiscall!(
                CALLEE_GATE,
                u32,
                this,
                0
            );
            if g as u8 != 0 {
                return finish(this, token);
            }
        }
        let stage = ((rec + REC_STAGE) as *const u32).read_unaligned();
        if stage == 2 {
            // The original calls the flag callee without setting ecx, so it
            // sees the previous stub's exit value (0); the real callee only
            // reads a global. Pass 0 to match what the stub observes.
            let f = lf_checker_rt::callee_thiscall!(CALLEE_FLAG, u32, 0);
            if f as u8 != 0 {
                let g = lf_checker_rt::callee_thiscall!(
                    CALLEE_GATE,
                    u32,
                    this,
                    1
                );
                if g as u8 != 0 {
                    return finish(this, token);
                }
            }
        }
        let stage = ((rec + REC_STAGE) as *const i32).read_unaligned();
        if stage >= 5 {
            let f = lf_checker_rt::callee_thiscall!(
                CALLEE_FINISH,
                u32,
                this,
                token
            );
            if f as u8 != 0 {
                return 1;
            }
            let stage2 =
                ((rec + REC_STAGE) as *const i32).read_unaligned();
            if stage2 >= 7 {
                return 0;
            }
        }
        ((this + MARK) as *mut u32).write_unaligned(0);
        let s = ((rec + REC_STAGE) as *const u32).read_unaligned();
        ((rec + REC_STAGE) as *mut u32).write_unaligned(s.wrapping_add(1));
        1
    }
});
