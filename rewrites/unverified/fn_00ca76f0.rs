// original: 0x00CA76F0 CEventHandler::vf41

/// Refresh the pending task unless the owner is already settled.
///
/// Asks slot `READY_SLOT` (0x128) of the owner's (the `this+0x04` object's)
/// virtual table whether it is ready. When ready and the follow-up check
/// also passes, the handler keeps its task and returns the check's result.
/// Otherwise it asks the task source for a fresh task: a null source, or a
/// null answer from it, clears the pending-task slot at `this+0x0C`, and a
/// live answer is turned into task kind 5 and stored there. Returns the
/// stored task, or the check's result on the early path. The three stack
/// arguments are not read.
///
/// Original: 0x00CA76F0 (thiscall, three unread stack words).
lf_checker_rt::export!(thiscall, rw_00ca76f0(this: u32, _a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const CHECK: u32 = 2;
        const SOURCE: u32 = 3;
        const WORKER: u32 = 4;
        const OWNER: u32 = 0x04;
        const PENDING: u32 = 0x0C;
        const READY_SLOT: u32 = 0x128;
        const SOURCE_GLOBAL: u32 = 0x0167E2A0;
        const TASK_KIND: u32 = 5;
        let owner = (this.wrapping_add(OWNER) as *const u32).read_unaligned();
        let vtable = (owner as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(READY_SLOT) as *const u32).read_unaligned();
        let ready: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if (ready(owner) as u8) != 0 {
            let check = lf_checker_rt::callee_cdecl!(CHECK, u32,);
            if (check as u8) != 0 {
                return check;
            }
        }
        let source = lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned();
        let handle = lf_checker_rt::callee_thiscall!(SOURCE, u32, source);
        if handle == 0 {
            (this.wrapping_add(PENDING) as *mut u32).write_unaligned(0);
            return 0;
        }
        let task = lf_checker_rt::callee_thiscall!(WORKER, u32, handle, TASK_KIND);
        (this.wrapping_add(PENDING) as *mut u32).write_unaligned(task);
        task
    }
});
