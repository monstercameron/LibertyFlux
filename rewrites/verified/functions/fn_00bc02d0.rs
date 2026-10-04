// original: 0x00bc02d0 alloc_task_3e_bounded (proposed)

/// Allocate a kind-0x3E task selected by a bounded index, for a ped handle.
///
/// When `handle` is non-zero the ped is looked up through the ped manager
/// (`G_PEDMGR`, thiscall with the handle) and its current task read from
/// `+0x6C` (a null lookup faults here, as in the original). A task with its
/// flag byte at `+0x0E` set is returned directly; a null task, a clear flag,
/// or a null handle falls through. An index is read through the parameter
/// getter (cdecl, `(arg1, 4)`); above `0x7F` it is returned unchanged. Else
/// the allocator from `G_ALLOC` is used (thiscall, no stack arguments): a
/// failed allocation faults storing through null (`[0+0x18]`), as in the
/// original. On success the object is specialised with the index (thiscall,
/// one word), `arg2` and `arg3` are stored at `+0x18` and `+0x1C`,
/// `(handle, task, 0x3E)` is reported to the result sink (cdecl, three
/// words), and the sink's answer is returned.
///
/// Original: 0x00BC02D0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00bc02d0(handle: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    const G_PEDMGR: u32 = 0x018B6F1C;
    const G_ALLOC: u32 = 0x0167E2A0;
    const OFF_TASK: u32 = 0x6C;
    const OFF_FLAG: u32 = 0x0E;
    const MAX_INDEX: u32 = 0x7F;
    const KIND: u32 = 0x3E;
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
    let index: u32 = lf_checker_rt::callee_cdecl!(2, u32, arg1, 4);
    if index > MAX_INDEX {
        return index;
    }
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(3, u32, alloc);
    if obj == 0 {
        unsafe { ((0x18) as *mut u32).write_unaligned(arg2) };
        return lf_checker_rt::callee_cdecl!(5, u32, handle, 0, KIND);
    }
    let task: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj, index);
    unsafe {
        ((task + 0x18) as *mut u32).write_unaligned(arg2);
        ((task + 0x1C) as *mut u32).write_unaligned(arg3);
    }
    lf_checker_rt::callee_cdecl!(5, u32, handle, task, KIND)
});
