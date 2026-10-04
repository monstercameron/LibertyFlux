// original: 0x00bc10e0 ped_indexed_setter_d8 (proposed)

/// Run an indexed setter on a ped's task block, or allocate a kind-0x60 task.
///
/// When `handle` is non-zero the ped is looked up through the ped manager
/// (`G_PEDMGR`, thiscall with the handle) and its current task read from
/// `+0x6C` (a null lookup faults here, as in the original). A task with its
/// flag byte at `+0x0E` set is returned directly. Otherwise an index is
/// formed: `-1` when `arg1` is null, else the parameter getter's answer
/// (cdecl, `(arg1, 7)`). With a non-null handle the ped is looked up again
/// and the indexed setter runs (thiscall with the index) on the block at
/// `+0x224` of the lookup, storing the index at its `+0xD8`; the setter's
/// answer is returned. With a null handle a kind-0x60 task is allocated from
/// `G_ALLOC`, stamped with vtable `0xEB7584` and the index at `+0x14`, and
/// `(0, object, 0x60)` is reported to the result sink (a failed allocation
/// reports `(0, null, 0x60)`).
///
/// Original: 0x00BC10E0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc10e0(handle: u32, arg1: u32) -> u32 {
    const G_PEDMGR: u32 = 0x018B6F1C;
    const G_ALLOC: u32 = 0x0167E2A0;
    const OFF_TASK: u32 = 0x6C;
    const OFF_FLAG: u32 = 0x0E;
    const OFF_BLOCK: u32 = 0x224;
    const VTABLE: u32 = 0x00EB7584;
    const KIND: u32 = 0x60;
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
    let index: u32 = if arg1 == 0 {
        0xFFFF_FFFF
    } else {
        lf_checker_rt::callee_cdecl!(2, u32, arg1, 7)
    };
    if handle != 0 {
        let mgr: u32 =
            unsafe { (lf_checker_rt::relocated(G_PEDMGR) as *const u32).read_unaligned() };
        let ped: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, handle);
        let block: u32 =
            unsafe { ((ped + OFF_BLOCK) as *const u32).read_unaligned() };
        return lf_checker_rt::callee_thiscall!(3, u32, block, index);
    }
    let alloc: u32 =
        unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
    let obj: u32 = lf_checker_rt::callee_thiscall!(4, u32, alloc);
    if obj == 0 {
        return lf_checker_rt::callee_cdecl!(6, u32, 0, 0, KIND);
    }
    let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj);
    unsafe {
        (obj as *mut u32).write_unaligned(VTABLE);
        ((obj + 0x14) as *mut u32).write_unaligned(index);
    }
    lf_checker_rt::callee_cdecl!(6, u32, 0, obj, KIND)
});
