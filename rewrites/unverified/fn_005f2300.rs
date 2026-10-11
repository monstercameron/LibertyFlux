// original: 0x005F2300 CEventHandlerNY::vf80

/// Handle the matching event type. A matching event whose marker byte is set
/// clears this handler's active object; otherwise it asks the manager for an
/// object, stores that result, and notifies the action object. A null event
/// pointer is accepted without either call. The third stack word is ignored.
///
/// The object layouts use the merged `CEventHandler` fields at `+0x04`,
/// `+0x0c`, and the event marker at `+0x219`. Calls are thiscall stand-ins:
/// manager lookup has no stack arguments; object creation and action dispatch
/// each have two. The boolean result is observed in AL.
lf_checker_rt::export!(thiscall, rw_005f2300(this: u32, event_type: u32, event: u32, _unused: u32) -> u32 {
    const EVENT_TYPE: u32 = 0x451;
    const EVENT_MARKER: u32 = 0x219;
    const ACTIVE_OBJECT: u32 = 0x0c;
    const ACTION_CONTEXT: u32 = 0x04;
    const ACTION_SLOT: u32 = 0x224;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;

    if event_type != EVENT_TYPE {
        return 0;
    }
    if event == 0 {
        return 1;
    }

    // SAFETY: the checker contract supplies a readable event object.
    let marker = unsafe { (event.wrapping_add(EVENT_MARKER) as *const u8).read() };
    if marker != 0 {
        // SAFETY: the contract supplies a writable handler object.
        unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(0) };
        return 1;
    }

    // SAFETY: the worker maps this global and patches the runtime base.
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned() };
    let manager_object = lf_checker_rt::callee_thiscall!(1, u32, manager);
    let new_object = if manager_object == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(2, u32, manager_object, event, 0)
    };

    // SAFETY: the contract supplies the owner context and action pointer.
    let action_owner = unsafe { (this.wrapping_add(ACTION_CONTEXT) as *const u32).read_unaligned() };
    let action = unsafe { (action_owner.wrapping_add(ACTION_SLOT) as *const u32).read_unaligned() };
    unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(new_object) };
    let _ = lf_checker_rt::callee_thiscall!(3, u32, action, event, 1);
    1
});
