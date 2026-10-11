// original: 0x0097E570 ped_task_dispatch_15_0

/// Dispatches fixed helper words 15 and 0 for the task context.
///
/// The first incoming cdecl word is unused. `task_owner`, the second
/// incoming word, is a pointer to an object whose embedded task
/// context begins at byte offset 0x3c0. The helper receives that
/// address in ECX and the two fixed words in stack order. Its EAX
/// value is returned unchanged; this wrapper performs no memory
/// writes and leaves incoming stack cleanup to its caller.
lf_checker_rt::export!(cdecl, rw_0097E570(_unused: u32, task_owner: u32) -> u32 {
    const TASK_CONTEXT_OFFSET: u32 = 0x3c0;
    const HELPER_ID: u32 = 1;
    const FIRST_HELPER_WORD: u32 = 15;
    const SECOND_HELPER_WORD: u32 = 0;
    let task_context = task_owner.wrapping_add(TASK_CONTEXT_OFFSET);
    lf_checker_rt::callee_thiscall!(HELPER_ID, u32, task_context, FIRST_HELPER_WORD, SECOND_HELPER_WORD)
});
