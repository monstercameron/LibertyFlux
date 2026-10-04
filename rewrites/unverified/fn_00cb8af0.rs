// original: 0x00cb8af0 CTaskSimpleMoveSlideToCoord::vf5
/// Forward the target to the slide applier and report success (vf5).
///
/// Calls the slide applier with `a0` (stdcall, three stack arguments;
/// `a1` and `a2` are unread and ecx is unused) and returns the applier's
/// answer with its low byte forced to 1. The callee is intercepted and
/// answered by the checker. No reads or writes of its own.
lf_checker_rt::export!(stdcall, rw_00cb8af0(a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        /// Callee id of the slide applier.
        const APPLY: u32 = 1;
        let v: u32 = lf_checker_rt::callee_stdcall!(APPLY, u32, a0);
        (v & 0xFFFFFF00) | 1
    }
});
