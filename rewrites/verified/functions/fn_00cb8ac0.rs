// original: 0x00cb8ac0 CTaskSimpleMoveGoToPointOnRoute::vf5
/// Refresh a go-to-point task, then check its mode argument (vf5).
///
/// Calls the route updater with (`this`, `a0`) (thiscall, three stack
/// arguments; `a2` is unread), then ignores its answer: the mode `a1` is
/// reloaded over it. When the mode is 1 or 2 the function returns it with
/// its low byte forced to 1; otherwise it sets bit `0x10` in the flag
/// word at `[this + 0xC4]` and returns the mode with its low byte cleared
/// (the original's `(an instruction of the original)` clears only the low byte). The callee is
/// intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00cb8ac0(this: u32, a0: u32, a1: u32, _a2: u32) -> u32 {
    unsafe {
        /// Flag word and bit set for an unrecognised mode.
        const FLAGS_OFF: u32 = 0xC4;
        const MODE_BIT: u32 = 0x10;
        /// Callee id of the route updater.
        const UPDATE: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(UPDATE, u32, this, a0);
        if a1 == 1 || a1 == 2 {
            (a1 & 0xFFFFFF00) | 1
        } else {
            let f = ((this + FLAGS_OFF) as *const u32).read_unaligned();
            ((this + FLAGS_OFF) as *mut u32).write_unaligned(f | MODE_BIT);
            a1 & 0xFFFFFF00
        }
    }
});
