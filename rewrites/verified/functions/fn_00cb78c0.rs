// original: 0x00cb78c0 subtask_select_then_copy
/// Select a shelter subtask, then copy through it when found (2 calls).
///
/// Calls the subtask selector with (`this`, `a0`) (thiscall, two stack
/// arguments). A null answer returns 0; otherwise the entry-copy callee
/// runs with the subtask and `a1`, and the function returns the copy's
/// answer with its low byte replaced by whether that low byte was
/// non-zero (the original's `(an instruction of the original)` / `setne al`). Both callees are
/// intercepted and answered by the checker. No reads or writes of its own.
lf_checker_rt::export!(thiscall, rw_00cb78c0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        /// Callee ids: subtask selector, then entry copy.
        const SELECT: u32 = 1;
        const COPY: u32 = 2;
        let sub: u32 = lf_checker_rt::callee_thiscall!(SELECT, u32, this, a0);
        if sub == 0 {
            return 0;
        }
        let v: u32 = lf_checker_rt::callee_thiscall!(COPY, u32, sub, a1);
        (v & 0xFFFFFF00) | u32::from((v & 0xFF) != 0)
    }
});
