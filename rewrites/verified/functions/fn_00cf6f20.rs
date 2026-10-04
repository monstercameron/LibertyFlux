// original: 0x00cf6f20 climb_task_probe_commit (proposed)

/// Probes a climb task's aim solution and commits it: runs the solver with
/// the target (the solver also receives two stack-slot addresses that the
/// comparison does not log, and any writes it makes through them are
/// unmodelled), runs the validator, then commits the four inputs with the
/// committer; a zero commit result returns the committer's value with a zero
/// low byte. Otherwise the first two inputs are stored at `+0x78`/`+0x7c`,
/// and unless the state word at `+0x14` is set the allocator runs with
/// (0x4000, 0, 1.0f) and a nonzero result skips the final store of the
/// target's word at `+0x278` to `+0x54` of the state at `+0x10`. Returns the
/// stored word (or the allocator's value) with low byte forced to 1.
///
/// Original: 0x00cf6f20 (thiscall: ecx holds the object, four stack words).
lf_checker_rt::export!(thiscall, rw_00cf6f20(this: u32, target: u32, first: u32, second: u32, third: u32) -> u32 {
    unsafe {
        const SOLVE_CALLEE: u32 = 1;
        const VALID_CALLEE: u32 = 2;
        const COMMIT_CALLEE: u32 = 3;
        const ALLOC_CALLEE: u32 = 4;
        lf_checker_rt::callee_thiscall!(SOLVE_CALLEE, u32, this, target);
        lf_checker_rt::callee_thiscall!(VALID_CALLEE, u32, this);
        let commit =
            lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, this, target, first, second, third, 1);
        if (commit & 0xff) == 0 {
            return commit & 0xffff_ff00;
        }
        ((this + 0x78) as *mut u32).write_unaligned(first);
        ((this + 0x7c) as *mut u32).write_unaligned(second);
        let state_flag = ((this + 0x14) as *const u32).read_unaligned();
        let mut final_word: u32;
        if state_flag != 0 {
            let state = ((this + 0x10) as *const u32).read_unaligned();
            final_word = ((target + 0x278) as *const u32).read_unaligned();
            ((state + 0x54) as *mut u32).write_unaligned(final_word);
        } else {
            let state = ((this + 0x10) as *const u32).read_unaligned();
            let alloc = lf_checker_rt::callee_thiscall!(
                ALLOC_CALLEE, u32, state, 0x4000, 0, 0x3f80_0000);
            if alloc == 0 {
                final_word = ((target + 0x278) as *const u32).read_unaligned();
                ((state + 0x54) as *mut u32).write_unaligned(final_word);
            } else {
                final_word = alloc;
            }
        }
        (final_word & 0xffff_ff00) | 1
    }
});
