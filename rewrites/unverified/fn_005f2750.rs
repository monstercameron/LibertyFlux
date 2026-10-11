// original: 0x005f2750 ProcessEventStreamPacket

/// Processes a packet when its event-data pointer is present. It calls the
/// event setup helpers with the payload and embedded context, asks the manager
/// for a current object, and stores the initializer result at handler `+0x0c`.
/// The final action helper receives the owner at `+0x04` adjusted by `0xbb0`,
/// the event data, fixed tuning words, and a relocated data pointer. A null
/// event-data pointer returns the packet pointer without calling helpers.
lf_checker_rt::export!(thiscall, rw_005f2750(this: u32, packet: u32, _unused1: u32, _unused2: u32) -> u32 {
    const PACKET_EVENT_DATA: u32 = 0x0c;
    const EVENT_CONTEXT: u32 = 0x20;
    const HANDLER_OWNER: u32 = 0x04;
    const ACTIVE_OBJECT: u32 = 0x0c;
    const ACTION_CONTEXT: u32 = 0x0bb0;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;
    const EVENT_SETUP_OBJECT: u32 = 0x016d_cc30;
    const ACTION_DATA: u32 = 0x00f9_37a8;
    const EVENT_SETUP_KIND: u32 = 8;
    const INIT_COUNT: u32 = 5000;
    const INIT_FLOAT_BITS: u32 = 0x4100_0000;
    const ACTION_LIMIT: u32 = 500;

    let event_data = unsafe { (packet.wrapping_add(PACKET_EVENT_DATA) as *const u32).read_unaligned() };
    if event_data == 0 {
        return packet;
    }

    let _ = lf_checker_rt::callee_thiscall!(
        1, u32, lf_checker_rt::relocated(EVENT_SETUP_OBJECT), EVENT_SETUP_KIND, event_data
    );
    let event_context = unsafe { (event_data.wrapping_add(EVENT_CONTEXT) as *const u32).read_unaligned() };
    let _ = lf_checker_rt::callee_cdecl!(
        2, u32, 1u32, event_context.wrapping_add(0x30), event_data, 0u32, 0u32, 0u32
    );

    let manager_context = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned() };
    let manager_object = lf_checker_rt::callee_thiscall!(3, u32, manager_context);
    let action_object = if manager_object == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            4, u32, manager_object, INIT_COUNT, 0u32, 0u32, INIT_FLOAT_BITS
        )
    };
    unsafe { (this.wrapping_add(ACTIVE_OBJECT) as *mut u32).write_unaligned(action_object) };

    let owner = unsafe { (this.wrapping_add(HANDLER_OWNER) as *const u32).read_unaligned() };
    let action_context = owner.wrapping_add(ACTION_CONTEXT);
    lf_checker_rt::callee_thiscall!(
        5, u32, action_context, ACTION_DATA, 0u32, event_data, INIT_COUNT,
        u32::MAX, 0u32, 0u32, ACTION_LIMIT, ACTION_LIMIT, 1u32
    )
});

