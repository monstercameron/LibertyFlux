// original: 0x00CD2980 task_sys_jump_thunk (proposed)

/// Jump thunk into the shared task-system routine.
///
/// Loads the address of the task-system singleton object and tail-jumps to
/// the shared routine, which treats it as its `this` pointer. Takes no stack
/// arguments and ignores its incoming registers; the return value is whatever
/// the shared routine returns.
///
/// Original: 0x00CD2980 (thunk: `(an instruction of the original); jmp <routine>`).
/// The rewrite forwards through the checker's tail stub: the singleton
/// address is derived with `relocated` (never hard-coded), and the call
/// comparison normalises both sides to the same image RVA.
lf_checker_rt::export!(cdecl, rw_00CD2980() -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0171C968))
});
