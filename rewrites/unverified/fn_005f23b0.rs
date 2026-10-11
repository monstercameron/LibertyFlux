// original: 0x005F23B0 CEventHandlerNY::vf83

/// Handle event type `0x44d`. The manager helper may return null, in which
/// case the handler's active-object field at `+0x0c` is cleared. Otherwise,
/// two helpers run, the returned object receives a relocated vtable pointer
/// and fields at `+0x18` and `+0x28`, and a non-null child is attached through
/// the address of the `+0x18` field. The manager object is stored at `+0x0c`.
/// The helpers are compared as thiscall or cdecl calls with their original
/// argument words; the boolean result is observed in AL.
lf_checker_rt::export!(thiscall, rw_005f23b0(this: u32, event_type: u32) -> u32 {
    const EVENT_TYPE: u32 = 0x44d;
    const ACTIVE_OBJECT: u32 = 0x0c;
    const CHILD_POINTER: u32 = 0x18;
    const CHILD_STATE: u32 = 0x28;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;
    const OBJECT_VTABLE: u32 = 0x00fe_142c;

    if event_type != EVENT_TYPE {
        return 0;
    }

    // SAFETY: the worker maps the global and the contract seeds its pointer.
    let manager_context = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let manager_object = lf_checker_rt::callee_thiscall!(1, u32, manager_context);
    if manager_object == 0 {
        // SAFETY: the contract supplies a writable handler object.
        unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(0) };
        return 1;
    }

    let child = lf_checker_rt::callee_cdecl!(2, u32, 0u32);
    let _ = lf_checker_rt::callee_thiscall!(3, u32, manager_object);

    // SAFETY: the scripted return points into writable heap storage.
    unsafe {
        (manager_object as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE));
        (manager_object.wrapping_add(CHILD_POINTER) as *mut u32).write_unaligned(child);
        (manager_object.wrapping_add(CHILD_STATE) as *mut u32).write_unaligned(0);
    }
    if child != 0 {
        let child_slot = manager_object.wrapping_add(CHILD_POINTER);
        let _ = lf_checker_rt::callee_thiscall!(4, u32, child, child_slot);
    }

    // SAFETY: the contract supplies a writable handler object.
    unsafe {
        (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(manager_object)
    };
    1
});
