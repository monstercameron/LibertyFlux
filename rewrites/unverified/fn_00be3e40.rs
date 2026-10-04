// original: 0x00be3e40 CTaskComplexMoveSequence::vf18 (symbols)

/// Forward to the shared sequence-advance helper with this task's cursors.
///
/// Passes the incoming argument through unchanged, together with pointers to
/// the two cursor words stored inline in this task (`this + CURSOR_A`, pushed
/// first, and `this + CURSOR_B`). The callee bumps the cursors and dispatches
/// the next child; this wrapper contributes no logic of its own. Returns the
/// callee's answer.
///
/// Original: 0x00be3e40 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be3e40(this: u32, arg: u32) -> u32 {
    unsafe {
        const CURSOR_A: u32 = 0x6c;
        const CURSOR_B: u32 = 0x64;
        const ADVANCE: u32 = 1;
        lf_checker_rt::callee_thiscall!(
            ADVANCE,
            u32,
            this,
            arg,
            this.wrapping_add(CURSOR_B),
            this.wrapping_add(CURSOR_A)
        )
    }
});
