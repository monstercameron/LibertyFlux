// original: 0x005F2420 CEventHandlerNY::vf82

/// Dispatch the selected event subtype. The subtype is bits 6 through 9 of
/// the event word at `+0x28`: kind 3 checks the event marker at `+0x219`,
/// while kind 2 checks the nested event pointer at `+0xf50` and its marker.
/// A non-short-circuited kind 2 or 3 calls the manager, stores its result in
/// the handler's `+0x0c` field, and calls the action object at
/// `[this+4]+0x224`. Kind 3 also sets bit `0x4000` at result offset `+0x60`.
/// The direct helpers are scripted with their original thiscall arguments;
/// the boolean result is observed in AL.
lf_checker_rt::export!(thiscall, rw_005f2420(this: u32, event_type: u32, event: u32) -> u32 {
    const EVENT_TYPE: u32 = 0x451;
    const EVENT_KIND: u32 = 0x28;
    const EVENT_MARKER: u32 = 0x219;
    const NESTED_EVENT: u32 = 0xf50;
    const ACTIVE_OBJECT: u32 = 0x0c;
    const ACTION_CONTEXT: u32 = 0x04;
    const ACTION_SLOT: u32 = 0x224;
    const OBJECT_STATE: u32 = 0x60;
    const OBJECT_ENABLED: u32 = 0x4000;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;

    if event_type != EVENT_TYPE {
        return 0;
    }

    let subtype_word = unsafe { (event.wrapping_add(EVENT_KIND) as *const u32).read_unaligned() };
    let subtype = (subtype_word >> 6) & 0x0f;
    let event_for_action = if subtype == 3 {
        let marker = unsafe { (event.wrapping_add(EVENT_MARKER) as *const u8).read() };
        if marker != 0 {
            unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(0) };
            return 1;
        }
        event
    } else if subtype == 2 {
        let nested = unsafe { (event.wrapping_add(NESTED_EVENT) as *const u32).read_unaligned() };
        if nested != 0 {
            let nested_marker = unsafe { (nested.wrapping_add(EVENT_MARKER) as *const u8).read() };
            if nested_marker != 0 {
                unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(0) };
                return 1;
            }
        }
        nested
    } else {
        return 1;
    };

    // SAFETY: the contract seeds the manager global with a heap pointer.
    let manager_context = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let manager_object = lf_checker_rt::callee_thiscall!(1, u32, manager_context);
    let result = if manager_object == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(2, u32, manager_object, event_for_action, 0)
    };

    // SAFETY: the contract supplies a writable handler and scripted result.
    unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(result) };
    if subtype == 3 {
        let state = unsafe {
            (result.wrapping_add(OBJECT_STATE) as *const u32).read_unaligned()
        };
        unsafe {
            (result.wrapping_add(OBJECT_STATE) as *mut u32)
                .write_unaligned(state | OBJECT_ENABLED)
        };
    }

    let action_owner = unsafe { (this.wrapping_add(ACTION_CONTEXT) as *const u32).read_unaligned() };
    let action = unsafe { (action_owner.wrapping_add(ACTION_SLOT) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_thiscall!(3, u32, action, event_for_action, 1);
    1
});
