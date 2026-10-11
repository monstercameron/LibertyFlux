// original: 0x00CCC7A0 CTaskComplexOnFire::vf18

/// Passing stage for the on-fire task's event handler. It covers the event
/// type `0x00d9` path: query the event kind through the dispatch object's
/// virtual slot `+0x0c`, then ask the task to create the follow-up using the
/// event pointer and reason `0x0516`. The event's pre-dispatch flag word at
/// `+0x28` is pinned with bit 21 clear, so the optional pre-query call and the
/// handler's other event cases are outside this stage. Those cases remain to
/// be implemented before the function can be treated as complete.
///
/// ABI: thiscall, `ECX = task`, one stack word `event`, callee pops four bytes.
pub(crate) unsafe fn run(task: u32, _event: u32, reason: u32) -> u32 {
    const TASK_DISPATCH: u32 = 0x08;
    const EVENT_TYPE_SLOT: u32 = 0x0c;
    const EVENT_REASON_CREATE: u32 = 0x00d9;
    const CALLEE_EVENT_TYPE: u32 = 1;
    const CALLEE_CREATE_FOLLOWUP: u32 = 2;
    const EVENT_REASONS: u32 = 0x0516;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let dispatch = read_u32(task.wrapping_add(TASK_DISPATCH));
        let vtable = read_u32(dispatch);
        let function = read_u32(vtable.wrapping_add(EVENT_TYPE_SLOT));
        let get_event_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(function as usize);
        let event_type = get_event_type(dispatch);
        if event_type == EVENT_REASON_CREATE {
            return lf_checker_rt::callee_thiscall!(
                CALLEE_CREATE_FOLLOWUP,
                u32,
                task,
                _event,
                reason
            );
        }
        let _ = (CALLEE_EVENT_TYPE, EVENT_REASONS);
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00ccc7a0(task: u32, event: u32) -> u32 {
    unsafe { run(task, event, 0x0516) }
});
