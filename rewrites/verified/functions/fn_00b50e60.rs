// original: 0x00B50E60 event_group_remove_by_type

/// Walk the event group's signed count and pointer array at byte offsets
/// `0x08` and `0x0c`. Query each event's type through vtable slot 1; when it
/// equals the one stack argument, call vtable slot 0 with deletion flag 1
/// and clear that array entry. The thiscall method itself returns no value.
lf_checker_rt::export!(thiscall, rw_remove_event(group: *mut u8, requested_type: u32) -> () {
    const COUNT_OFFSET: usize = 0x08;
    const EVENTS_OFFSET: usize = 0x0c;
    const EVENT_POINTER_BYTES: usize = 4;
    const TYPE_SLOT_OFFSET: usize = 4;
    const DESTROY_SLOT_OFFSET: usize = 0;
    unsafe {
        let count = (group.add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count <= 0 { return; }
        for index in 0..(count as usize) {
            let entry = (group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *mut u32);
            let event = entry.read_unaligned();
            let vtable = (event as *const u32).read_unaligned();
            let type_address = ((vtable as *const u8).add(TYPE_SLOT_OFFSET) as *const u32).read_unaligned();
            let get_type: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(type_address as usize);
            if get_type(event) != requested_type { continue; }
            let destroy_address = ((vtable as *const u8).add(DESTROY_SLOT_OFFSET) as *const u32).read_unaligned();
            let destroy: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(destroy_address as usize);
            let _ = destroy(event, 1);
            (group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *mut u32).write_unaligned(0);
        }
    }
});
