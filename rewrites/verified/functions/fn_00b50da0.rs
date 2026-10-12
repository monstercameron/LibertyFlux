// original: 0x00B50DA0 event_group_has_type20_with_id

/// Scan the event group using its signed count at byte offset `0x08` and
/// pointer array at `0x0c`. Skip null events; for each other event, call
/// vtable slot 1 and continue unless its type is `0x20`. A matching-type
/// event with a non-null nested object at `0x10` is queried through that
/// object's vtable slot 3; return byte one when the returned id equals the
/// one stack argument, otherwise return zero after the scan.
lf_checker_rt::export!(thiscall, rw_type20_id(group: *const u8, search_id: u32) -> u8 {
    const COUNT_OFFSET: usize = 0x08;
    const EVENTS_OFFSET: usize = 0x0c;
    const EVENT_POINTER_BYTES: usize = 4;
    const NESTED_OFFSET: usize = 0x10;
    const TYPE_SLOT_OFFSET: usize = 0x4;
    const ID_SLOT_OFFSET: usize = 0xc;
    const EVENT_TYPE: u32 = 0x20;
    unsafe {
        let count = (group.add(COUNT_OFFSET) as *const i32).read_unaligned();
        if count <= 0 { return 0; }
        for index in 0..(count as usize) {
            let event = (group.add(EVENTS_OFFSET + index * EVENT_POINTER_BYTES) as *const u32).read_unaligned() as *mut u8;
            if event.is_null() { continue; }
            let event_vtable = (event as *const u32).read_unaligned();
            let type_address = ((event_vtable as *const u8).add(TYPE_SLOT_OFFSET) as *const u32).read_unaligned();
            let get_type: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(type_address as usize);
            let event_type = get_type(event as u32);
            if event_type != EVENT_TYPE { continue; }
            let nested = (event.add(NESTED_OFFSET) as *const u32).read_unaligned();
            if nested == 0 { continue; }
            let nested_vtable = (nested as *const u32).read_unaligned();
            let id_address = ((nested_vtable as *const u8).add(ID_SLOT_OFFSET) as *const u32).read_unaligned();
            let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(id_address as usize);
            if get_id(nested) == search_id { return 1; }
        }
        0
    }
});
