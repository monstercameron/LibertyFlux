// original: 0x005f26d0 EventHandlerDispatchPacket

/// Examines a packet's event data and identifier against the handler's current
/// state. If the state matches and has flag bit two set, it asks the manager
/// for an object, optionally initializes it, stores that object in the handler,
/// then invokes the event action when the event marker is nonzero. The return
/// value follows the original's early exits: packet pointer before state
/// lookup, state pointer after lookup, then helper or action result.
lf_checker_rt::export!(thiscall, rw_005f26d0(this: u32, packet: u32, _unused1: u32, _unused2: u32) -> u32 {
    const CURRENT_STATE: u32 = 0x04;
    const ACTIVE_OBJECT: u32 = 0x0c;
    const EVENT_DATA: u32 = 0x0c;
    const EVENT_ID: u32 = 0x10;
    const STATE_EVENT_ID: u32 = 0xb30;
    const STATE_FLAGS: u32 = 0x26c;
    const EVENT_MARKER: u32 = 0x219;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;

    let event_data = unsafe { (packet.wrapping_add(EVENT_DATA) as *const u32).read_unaligned() };
    let event_id = unsafe { (packet.wrapping_add(EVENT_ID) as *const u32).read_unaligned() };
    if event_data == 0 || event_id == 0 {
        return packet;
    }

    let state = unsafe { (this.wrapping_add(CURRENT_STATE) as *const u32).read_unaligned() };
    let mut result = state;
    let state_event_id = unsafe { (state.wrapping_add(STATE_EVENT_ID) as *const u32).read_unaligned() };
    let state_flags = unsafe { (state.wrapping_add(STATE_FLAGS) as *const u8).read() };
    if state_event_id != event_id || state_flags & 4 == 0 || state == event_data {
        return result;
    }

    let manager_context = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned() };
    let manager_object = lf_checker_rt::callee_thiscall!(1, u32, manager_context);
    if manager_object == 0 {
        result = 0;
    } else {
        result = lf_checker_rt::callee_thiscall!(2, u32, manager_object, event_id, 0u32, 0u32, 0u32);
    }
    unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(result) };

    let marker = unsafe { (event_data.wrapping_add(EVENT_MARKER) as *const u8).read() };
    if marker != 0 {
        result = lf_checker_rt::callee_thiscall!(3, u32, event_data, 1u32, 1000u32);
    }
    result
});
