// original: 0x00B50A70 event_group_find_by_type

/// Scan the event pointers embedded at byte offset `0x0c`, using the signed
/// count at `0x08`. For each non-null-by-contract event, call vtable slot 1
/// as a zero-stack-argument thiscall to obtain its type. Return the first
/// event whose type equals the requested value, or null if none matches.
/// The method pops its one 32-bit stack argument.
lf_checker_rt::export!(thiscall, rw_find_event(group: *const u8, requested_type: u32) -> u32 {
    const COUNT_OFFSET: usize = 0x08;
    const EVENTS_OFFSET: usize = 0x0c;
    const EVENT_POINTER_BYTES: usize = 4;
    const GETTER_SLOT_OFFSET: usize = 4;
    unsafe {
        let count = (group.add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count <= 0 { return 0; }
        for index in 0..(count as usize) {
            let event = (group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *const u32).read_unaligned() as *const u8;
            let vtable = (event as *const u32).read_unaligned();
            let getter_address = ((vtable as *const u8).add(GETTER_SLOT_OFFSET) as *const u32).read_unaligned();
            let getter: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(getter_address as usize);
            let event_type = getter(event as u32);
            if event_type == requested_type { return event as u32; }
        }
        0
    }
});
