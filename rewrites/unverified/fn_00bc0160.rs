// original: 0x00bc0160 alloc_task_00_ped (proposed)

/// Allocate a kind-0 task for a ped handle after checking its current task.
///
/// When `handle` is non-zero the ped is looked up through the ped manager
/// (`G_PEDMGR`, thiscall with the handle) and its current task read from
/// `+0x6C` (a null lookup faults here, as in the original). A task with its
/// flag byte at `+0x0E` set is returned directly; a null task, a clear flag,
/// or a null handle falls through to the allocation. Allocation reads the
/// allocator from `G_ALLOC` (thiscall, no stack arguments): on failure
/// `(handle, null, null)` is reported to the result sink (cdecl, three words)
/// and its answer returned. On success the object is initialised (ecx-only
/// call), stamped with vtable `0xE9ED64`, zeroed at `+0x14`, `+0x18` and
/// `+0x1C` (word), given `arg1` at `+0x20`, and `(handle, object, 0)` is
/// reported to the sink, whose answer is returned.
///
/// Original: 0x00BC0160 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc0160(handle: u32, arg1: u32) -> u32 {
    const G_PEDMGR: u32 = 0x018B6F1C;
    const G_ALLOC: u32 = 0x0167E2A0;
    const OFF_TASK: u32 = 0x6C;
    const OFF_FLAG: u32 = 0x0E;
    const VTABLE: u32 = 0x00E9ED64;
    const OFF_VALUE: u32 = 0x20;
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
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(2, u32, alloc);
    if obj == 0 {
        return lf_checker_rt::callee_cdecl!(4, u32, handle, 0, 0);
    }
    let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj);
    unsafe {
        (obj as *mut u32).write_unaligned(VTABLE);
        ((obj + 0x14) as *mut u32).write_unaligned(0);
        ((obj + 0x18) as *mut u32).write_unaligned(0);
        ((obj + 0x1C) as *mut u16).write_unaligned(0);
        ((obj + OFF_VALUE) as *mut u32).write_unaligned(arg1);
    }
    lf_checker_rt::callee_cdecl!(4, u32, handle, obj, 0)
});
