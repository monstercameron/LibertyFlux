// original: 0x00CA79C0 CEventHandler::vf31

/// Forward an event to the handler's own dispatch slot, or clear the task.
///
/// `event` (first stack argument) carries a kind word at `+0x10` and a
/// payload word at `+0x18`. When the kind is `CLEAR_KIND` (0xC8) the
/// handler's pending-task slot at `this+0x0C` is set to null. Otherwise the
/// handler calls its own virtual slot `DISPATCH_SLOT` (0x134) with
/// (kind, payload) as stack arguments. No meaningful return value; the other
/// two stack arguments are not read.
///
/// Original: 0x00CA79C0 (thiscall, three stack words, only the first read).
lf_checker_rt::export!(thiscall, rw_00ca79c0(this: u32, event: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const EV_KIND: u32 = 0x10;
        const EV_PAYLOAD: u32 = 0x18;
        const PENDING_TASK: u32 = 0x0C;
        const CLEAR_KIND: u32 = 0xC8;
        const DISPATCH_SLOT: u32 = 0x134;
        let kind = (event.wrapping_add(EV_KIND) as *const u32).read_unaligned();
        if kind == CLEAR_KIND {
            (this.wrapping_add(PENDING_TASK) as *mut u32).write_unaligned(0);
            return 0;
        }
        let payload = (event.wrapping_add(EV_PAYLOAD) as *const u32).read_unaligned();
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(DISPATCH_SLOT) as *const u32).read_unaligned();
        let dispatch: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        dispatch(this, kind, payload);
        0
    }
});
