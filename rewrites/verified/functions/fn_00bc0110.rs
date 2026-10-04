// original: 0x00bc0110 ped_task_build5 (proposed)

/// Build a kind-0x11 task for a ped handle, forwarding five words.
///
/// When `handle` is non-zero the ped is looked up through the ped manager
/// (`G_PEDMGR`, thiscall with the handle) and its current task read from
/// `+0x6C` (a null lookup faults here, as in the original). A null task, or a
/// task whose flag byte at `+0x0E` is clear, falls through to the build below;
/// a task with the flag set is returned directly. The build looks the second
/// argument up through `G_LOOKUP` (thiscall with `arg1`) and tail-forwards
/// `(handle, looked_up, arg2, arg3, 0x11)` to the kind-0x11 task builder
/// (cdecl, five words), whose answer is returned. A null `handle` skips the
/// lookup and builds directly.
///
/// Original: 0x00BC0110 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00bc0110(handle: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    const G_PEDMGR: u32 = 0x018B6F1C;
    const G_LOOKUP: u32 = 0x012E22A4;
    const OFF_TASK: u32 = 0x6C;
    const OFF_FLAG: u32 = 0x0E;
    const KIND: u32 = 0x11;
    if handle != 0 {
        let mgr: u32 =
            unsafe { (lf_checker_rt::relocated(G_PEDMGR) as *const u32).read_unaligned() };
        let ped: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, handle);
        let task: u32 =
            unsafe { ((ped + OFF_TASK) as *const u32).read_unaligned() };
        if task != 0 {
            let flag: u8 = unsafe { ((task + OFF_FLAG) as *const u8).read() };
            if flag != 0 {
                return task;
            }
        }
    }
    let lookup: u32 =
        unsafe { (lf_checker_rt::relocated(G_LOOKUP) as *const u32).read_unaligned() };
    let found: u32 = lf_checker_rt::callee_thiscall!(1, u32, lookup, arg1);
    lf_checker_rt::callee_cdecl!(2, u32, handle, found, arg2, arg3, KIND)
});
