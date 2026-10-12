// original: 0x00B50FB0 event_group_remove_type62_if_accepted

/// Scan the event group's signed count and pointer array at byte offsets
/// `0x08` and `0x0c`. Skip null events, query each other event's type through
/// vtable slot 1, and for type `0x62` call slot 5 with the stack acceptance
/// key. Accepted events are destroyed through slot 0 with argument 1 and
/// removed from the array. Finally call the group's compact routine and
/// return its EAX value. This thiscall method pops one stack argument.
lf_checker_rt::export!(thiscall, rw_remove62(group: *mut u8, acceptance_key: u32) -> u32 {
    const COUNT_OFFSET: usize = 0x08;
    const EVENTS_OFFSET: usize = 0x0c;
    const EVENT_POINTER_BYTES: usize = 4;
    const TYPE_SLOT_OFFSET: usize = 4;
    const ACCEPT_SLOT_OFFSET: usize = 0x14;
    const DESTROY_SLOT_OFFSET: usize = 0;
    const TARGET_TYPE: u32 = 0x62;
    unsafe {
        let count = (group.add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count > 0 {
            for index in 0..(count as usize) {
                let entry = group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *mut u32;
                let event = entry.read_unaligned();
                if event == 0 { continue; }
                let vtable = (event as *const u32).read_unaligned();
                let type_address = ((vtable as *const u8).add(TYPE_SLOT_OFFSET) as *const u32).read_unaligned();
                let get_type: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(type_address as usize);
                if get_type(event) != TARGET_TYPE { continue; }
                let accept_address = ((vtable as *const u8).add(ACCEPT_SLOT_OFFSET) as *const u32).read_unaligned();
                let accept: extern "thiscall" fn(u32, u32) -> u8 = core::mem::transmute(accept_address as usize);
                if accept(event, acceptance_key) == 0 { continue; }
                let destroy_address = ((vtable as *const u8).add(DESTROY_SLOT_OFFSET) as *const u32).read_unaligned();
                let destroy: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(destroy_address as usize);
                let _ = destroy(event, 1);
                entry.write_unaligned(0);
            }
        }
        lf_checker_rt::callee_thiscall!(4, u32, group as u32)
    }
});
