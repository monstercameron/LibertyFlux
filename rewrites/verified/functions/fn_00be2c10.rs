// original: 0x00be2c10 CTaskComplexMoveSequence::vf19 (symbols)

/// Forward to the shared sequence-step helper with this task's cursors.
///
/// Passes the incoming argument through unchanged, together with pointers to
/// the two cursor words stored inline in this task (`this + CURSOR_A`, pushed
/// first, and `this + CURSOR_B`). The callee reads the second cursor as an
/// index into the task's child table and dispatches through it; this wrapper
/// contributes no logic of its own. Returns the callee's answer.
///
/// The callee keeps the caller's stack frame layout: it pops all three words
/// itself (thiscall, three stack arguments, object in ECX).
///
/// Original: 0x00be2c10 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be2c10(this: u32, arg: u32) -> u32 {
    unsafe {
        const CURSOR_A: u32 = 0x6c;
        const CURSOR_B: u32 = 0x64;
        const STEP: u32 = 1;
        lf_checker_rt::callee_thiscall!(
            STEP,
            u32,
            this,
            arg,
            this.wrapping_add(CURSOR_B),
            this.wrapping_add(CURSOR_A)
        )
    }
});
