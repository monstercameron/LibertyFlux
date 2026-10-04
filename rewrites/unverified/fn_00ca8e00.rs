// original: 0x00CA8E00 CEventHandler::vf29

/// Replace the pending task with a clone made by the event.
///
/// Calls slot `CLONE_SLOT` (0x54) of the event's (first stack argument)
/// virtual table with the event as `this`, and stores the returned task in
/// the handler's pending-task slot at `this+0x0C`. Returns the new task as
/// well (the original's eax). The other two stack arguments are not read.
///
/// Original: 0x00CA8E00 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca8e00(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const PENDING_TASK: u32 = 0x0C;
        const CLONE_SLOT: u32 = 0x54;
        let vtable = (event as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(CLONE_SLOT) as *const u32).read_unaligned();
        let clone: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let task = clone(event);
        (this.wrapping_add(PENDING_TASK) as *mut u32).write_unaligned(task);
        task
    }
});
