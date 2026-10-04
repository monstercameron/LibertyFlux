// original: 0x00cb8a80 CTaskSimpleMoveAchieveHeading::vf5
/// Initialise an achieve-heading task through three callees (vf5).
///
/// Runs the notifier with (`this`, `a0`), the applier with (`a0`, the
/// heading at `[a0 + 0xAA0]`), and the finaliser with `a0` (thiscall,
/// three stack arguments; `a1` and `a2` are unread), then returns the
/// finaliser's answer with its low byte forced to 1. All callees are
/// intercepted and answered by the checker. No reads or writes of its own.
lf_checker_rt::export!(thiscall, rw_00cb8a80(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        /// Heading offset in the argument object.
        const HDG_OFF: u32 = 0xAA0;
        const NOTIFY: u32 = 1;
        const APPLY: u32 = 2;
        const FINISH: u32 = 3;
        let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, a0);
        let h = ((a0 + HDG_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(APPLY, u32, a0, h);
        let v: u32 = lf_checker_rt::callee_thiscall!(FINISH, u32, a0);
        (v & 0xFFFFFF00) | 1
    }
});
